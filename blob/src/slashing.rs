use crate::build_signature_payload;
use crate::store::StorageApiT;
use crate::types::{BlobSignatureData, FullClient};
use crate::utils::{
	get_active_validators, get_current_session_key, get_my_validator_public_account, sign_data,
	verify_signed_blob_data,
};
use anyhow::{anyhow, Result};
use codec::{Decode, Encode};
use da_control::{BlobOffenceKind, BlobTxSummaryRuntime, Call};
use da_control::{OffenceKey, ValidatorVoucher};
use da_runtime::{apis::BlobApi, RuntimeCall, UncheckedExtrinsic};
use futures::future::join_all;
use futures::StreamExt;
use sc_client_api::{BlockBackend, BlockchainEvents, StorageKey, StorageProvider};
use sc_keystore::LocalKeystore;
use sc_transaction_pool_api::{TransactionPool, TransactionSource};
use scale_info::prelude::collections::HashSet;
use sp_api::ProvideRuntimeApi;
use sp_authority_discovery::AuthorityDiscoveryApi;
use sp_consensus_babe::{digests::PreDigest, BABE_ENGINE_ID};
use sp_core::H256;
use sp_runtime::{
	traits::{Block as BlockT, Header as HeaderT},
	AccountId32,
	DigestItem::PreRuntime,
};
use std::sync::Arc;

fn get_blob_summary_runtime(
	client: &Arc<FullClient>,
	block_hash: H256,
) -> Result<(Vec<BlobTxSummaryRuntime>, u32, u64)> {
	let blob_txs_summary_data = match client.block_body(block_hash) {
		Ok(Some(exts)) => {
			if exts.len() < 2 {
				return Err(anyhow!("Invalid extrinsic count at {:?}", block_hash));
			}
			let summary_extrinsic_encoded = &exts[exts.len() - 2].encode();
			let summary_extrinsic: UncheckedExtrinsic =
				match Decode::decode(&mut &summary_extrinsic_encoded[..]) {
					Ok(s) => s,
					Err(e) => {
						return Err(anyhow!(
							"Failed to decode summary extrinsic at {:?}: {:?}",
							block_hash,
							e
						));
					},
				};
			let blob_txs_summary_data = match summary_extrinsic.function {
				RuntimeCall::DataAvailability(Call::submit_blob_txs_summary {
					blob_txs_summary,
					nb_blobs,
					total_blob_size,
				}) => (blob_txs_summary, nb_blobs, total_blob_size),
				_ => {
					return Err(anyhow!("Invalid summary extrinsic at {:?}", block_hash));
				},
			};

			blob_txs_summary_data
		},
		Ok(None) => {
			return Err(anyhow!("Blob txs summary not found at {:?}", block_hash,));
		},
		Err(e) => {
			return Err(anyhow!(
				"Could not get block body at {:?}: {:?}",
				block_hash,
				e
			));
		},
	};

	Ok(blob_txs_summary_data)
}

/// Checks if the current node is an active validator of the chain
fn check_is_active_validator<Client, Block>(
	client: &Arc<Client>,
	keystore: &Arc<LocalKeystore>,
	imported_block_hash: <Block as BlockT>::Hash,
) -> (bool, Option<AccountId32>)
where
	Block: BlockT,
	Client: ProvideRuntimeApi<Block>,
	Client::Api: BlobApi<Block>,
{
	let Some((authority_id, key_type_id)) = get_my_validator_public_account(keystore) else {
		return (false, None);
	};

	let Ok(owner_opt) = client.runtime_api().get_validator_from_key(
		imported_block_hash,
		key_type_id,
		authority_id.encode(),
	) else {
		return (false, None);
	};

	let Some(my_validator_id) = owner_opt else {
		return (false, None);
	};

	let active_validators = get_active_validators(client, &imported_block_hash.encode());

	(
		active_validators.contains(&my_validator_id),
		Some(my_validator_id),
	)
}

/// Checks whether the validator has enough free balance to cover the required blob offence deposit.
/// Returns `true` if the validator can afford the deposit, otherwise `false`.
fn check_has_sufficient_balance(
	client: &Arc<FullClient>,
	imported_block_hash: H256,
	my_address: &AccountId32,
) -> u128 {
	let required_deposit = client
		.runtime_api()
		.get_blob_vouch_fee_reserve(imported_block_hash)
		.unwrap_or(0);

	let account_storage_key =
		StorageKey(frame_system::Account::<da_runtime::Runtime>::hashed_key_for(&my_address));

	let free_balance: u128 = client
		.storage(imported_block_hash, &account_storage_key)
		.ok()
		.flatten()
		.and_then(|raw| {
			use pallet_balances::AccountData;
			AccountData::<u128>::decode(&mut &raw.0[..])
				.ok()
				.map(|data| data.free)
		})
		.unwrap_or(0);

	free_balance % required_deposit
}

fn create_blob_offence(
	keystore: &Arc<LocalKeystore>,
	offence_key: OffenceKey,
	validator: AccountId32,
	block_author: AccountId32,
	current_session_index: u32,
) -> Result<RuntimeCall> {
	let (public, signature) = sign_data(
		keystore,
		(offence_key.clone(), current_session_index).encode(),
	)?;

	let voucher = ValidatorVoucher {
		validator,
		key: public,
		session_index: current_session_index,
		signature,
		block_author,
	};

	let call = RuntimeCall::DataAvailability(da_control::Call::register_blob_offence {
		offence_key,
		voucher,
	});

	Ok(call)
}

fn create_unsigned_extrinsics(calls: Vec<RuntimeCall>) -> Vec<UncheckedExtrinsic> {
	let txs = calls
		.iter()
		.map(|x| UncheckedExtrinsic::new_unsigned(x.clone()))
		.collect();

	txs
}

fn get_finalized_block_author(client: &Arc<FullClient>, hash: H256) -> Result<Option<AccountId32>> {
	let header = client
		.header(hash)?
		.ok_or_else(|| anyhow::anyhow!("No header"))?;

	let mut authority_index = None;

	for log in header.digest().logs.clone().into_iter() {
		match log {
			PreRuntime(consensus_engine_id, data) => {
				if consensus_engine_id == BABE_ENGINE_ID {
					let Ok(pre_digest) = PreDigest::decode(&mut &data[..]) else {
						continue;
					};
					authority_index = Some(pre_digest.authority_index());
					break;
				}
			},
			_ => {},
		}
	}

	let Some(authority_index) = authority_index else {
		return Ok(None);
	};

	let authorities = client.runtime_api().authorities(hash)?;
	if authority_index as usize >= authorities.len() {
		return Err(anyhow!("Invalid authority index"));
	}

	let babe_id = &authorities[authority_index as usize];
	let key_type = sp_core::crypto::key_types::BABE;
	let encoded_key = babe_id.encode();
	let validator_account =
		client
			.runtime_api()
			.get_validator_from_key(hash, key_type, encoded_key)?;

	Ok(validator_account)
}

pub async fn check_missing_validators<Pool, Block>(
	client: Arc<FullClient>,
	keystore: Arc<LocalKeystore>,
	blob_database: Arc<dyn StorageApiT>,
	pool: Arc<Pool>,
) where
	Block: BlockT,
	Pool: TransactionPool<Block = Block> + 'static,
{
	let mut block_sub = client.finality_notification_stream();
	let mut last_checked_session_index: Option<u32> = None;
	let mut is_active_validator = false;
	let mut my_address: Option<AccountId32> = None;
	while let Some(imported_block) = block_sub.next().await {
		let mut offence_to_create = Vec::new();
		// Get the current session index
		let current_session_index: u32 = client
			.storage(imported_block.hash, &get_current_session_key())
			.ok()
			.flatten()
			.and_then(|raw| codec::Decode::decode(&mut &raw.0[..]).ok())
			.unwrap_or(0);

		// If we're in a new era and the node is authority, we can check if it has been included in the active set.
		let should_check = match last_checked_session_index {
			None => true,
			Some(last) => current_session_index > last,
		};
		if should_check {
			last_checked_session_index = Some(current_session_index);

			let (is_active, maybe_address) =
				check_is_active_validator(&client, &keystore, imported_block.hash);
			is_active_validator = is_active;
			my_address = maybe_address;
		}

		// Return if I'm not active
		if !is_active_validator {
			continue;
		}

		// Return if for some reason the address is not found
		let Some(my_address) = my_address.clone() else {
			log::warn!("Could not get validator address for missing-validators-listener");
			continue;
		};

		// Check the balance of the validator, it makes no sense to continue if
		// the validator has not enough funds to make the deposit for a blob offence.
		let nb_offence_deposits =
			check_has_sufficient_balance(&client, imported_block.hash, &my_address);
		if nb_offence_deposits == 0 {
			log::warn!("Validator has not enough balance to register an offence if needed for missing-validators-listener.");
			continue;
		}

		// Get blobs summary from finalized block
		let (blob_txs_summary, nb_blobs, _) =
			match get_blob_summary_runtime(&client, imported_block.hash) {
				Ok(s) => s,
				Err(e) => {
					log::warn!(
						"Could not get blob_txs_summary at hash {:?}: {:?}",
						imported_block.hash,
						e
					);
					continue;
				},
			};

		// SummaryNbBlobMismatch: Check that the number of blobs annouced is the same as in the array
		let valid_nb_blobs = nb_blobs == blob_txs_summary.len() as u32;
		if !valid_nb_blobs {
			offence_to_create.push(OffenceKey {
				block_hash: imported_block.hash,
				kind: BlobOffenceKind::SummaryNbBlobMismatch,
				blob_hash: None,
				missing_validator: None,
			});
		}

		for summary in blob_txs_summary.iter() {
			// InvalidSignatureForBlob && DuplicateSignatureForBlob: Check the provided ownership signatures and check for duplicates
			let mut has_invalid_sig = false;
			let mut has_duplicate_validator = false;
			let mut ownership_validators: HashSet<AccountId32> = HashSet::new();
			for (addr, babe_key, _, sig) in summary.ownership.iter() {
				if !ownership_validators.insert(addr.clone()) {
					has_duplicate_validator = true;
				}

				if !has_invalid_sig {
					let payload = build_signature_payload(
						summary.hash,
						[addr.encode(), b"stored".to_vec()].concat(),
					);
					let ok = verify_signed_blob_data(
						BlobSignatureData {
							signer: babe_key.encode(),
							signature: sig.clone(),
						},
						payload,
					)
					.unwrap_or(false);
					if !ok {
						has_invalid_sig = true
					}
				}
			}

			if has_invalid_sig {
				offence_to_create.push(OffenceKey {
					block_hash: imported_block.hash,
					kind: BlobOffenceKind::InvalidSignatureForBlob,
					blob_hash: Some(summary.hash),
					missing_validator: None,
				});
			}

			if has_duplicate_validator {
				offence_to_create.push(OffenceKey {
					block_hash: imported_block.hash,
					kind: BlobOffenceKind::DuplicateSignatureForBlob,
					blob_hash: Some(summary.hash),
					missing_validator: None,
				});
			}

			// We can only act on this error "Not enough validators..."
			if summary.reason.is_some()
				&& summary.reason != Some("Not enough validators vouched for this block".into())
			{
				continue;
			}

			// Get blob metadata
			let Some(meta) = blob_database
				.get_blob_metadata(&summary.hash)
				.ok()
				.flatten()
			else {
				continue;
			};

			// InvalidNbOfOwnershipForBlob: Check if threshold are correctly reached
			let threshold = meta.nb_validators_per_blob_threshold;
			if summary.success && summary.ownership.len() < threshold as usize {
				offence_to_create.push(OffenceKey {
					block_hash: imported_block.hash,
					kind: BlobOffenceKind::InvalidNbOfOwnershipForBlob,
					blob_hash: Some(summary.hash),
					missing_validator: None,
				});
			}

			// Check if it's successfull
			if summary.success {
				continue;
			}

			// Get the missing validators and invalid validator account in ownership
			let expected_storing_validators: HashSet<AccountId32> =
				meta.storing_validator_list.iter().cloned().collect();

			// InvalidValidatorForBlob: Invalid validator were put in the ownership
			let invalid_validators: Vec<AccountId32> = ownership_validators
				.difference(&expected_storing_validators)
				.cloned()
				.collect();
			if invalid_validators.len() > 0 {
				offence_to_create.push(OffenceKey {
					block_hash: imported_block.hash,
					kind: BlobOffenceKind::InvalidValidatorForBlob,
					blob_hash: Some(summary.hash),
					missing_validator: None,
				});
			}

			// MissingValidatorForBlob && OmittedValidatorForBlob: Missing validator (either really missing or omitted)
			let reported_missing_validators: Vec<AccountId32> = expected_storing_validators
				.difference(&ownership_validators)
				.cloned()
				.collect();
			if reported_missing_validators.len() > 0 {
				let Ok(my_ownerships) = blob_database.get_blob_ownerships(&summary.hash) else {
					continue;
				};
				let my_ownerships: HashSet<AccountId32> =
					my_ownerships.iter().cloned().map(|x| x.address).collect();
				for reported_missing_validator in reported_missing_validators.iter() {
					let mut offence_key = OffenceKey {
						block_hash: imported_block.hash,
						kind: BlobOffenceKind::MissingValidatorForBlob,
						blob_hash: Some(summary.hash),
						missing_validator: Some(reported_missing_validator.clone()),
					};
					let ownership_exist = my_ownerships.contains(reported_missing_validator);
					if ownership_exist {
						offence_key.kind = BlobOffenceKind::OmittedValidatorForBlob;
					}
					offence_to_create.push(offence_key);
				}
			}
		}

		// Create all offence
		let offence_len = offence_to_create.len() as u128;
		if offence_len > 0 {
			if nb_offence_deposits < offence_len {
				log::warn!("Validator has not enough balance to register {:?} offences needed for missing-validators-listener.", offence_len);
				continue;
			}

			let author = match get_finalized_block_author(&client, imported_block.hash) {
				Ok(a) => a,
				Err(e) => {
					log::warn!(
						"Could not get block author at hash: {:?} - {:?}",
						imported_block.hash,
						e
					);
					continue;
				},
			};
			let Some(author) = author else {
				log::warn!("No block author at hash: {:?}", imported_block.hash);
				continue;
			};

			let chain_info = client.chain_info();
			let best_hash = chain_info.best_hash;

			let offences: Vec<RuntimeCall> = offence_to_create
				.iter()
				.filter_map(|x| {
					create_blob_offence(
						&keystore,
						x.clone(),
						my_address.clone(),
						author.clone(),
						current_session_index,
					)
					.ok()
				})
				.collect();

			let txs: Vec<<Pool::Block as BlockT>::Extrinsic> = create_unsigned_extrinsics(offences)
				.iter()
				.filter_map(|x| {
					let pool_tx: Option<<Pool::Block as BlockT>::Extrinsic> =
						Decode::decode(&mut x.encode().as_slice()).ok();

					pool_tx
				})
				.collect();

			// Find a better trick...
			let best_hash_pool: <Pool::Block as BlockT>::Hash =
				match Decode::decode(&mut best_hash.encode().as_slice()) {
					Ok(h) => h,
					Err(e) => {
						log::error!("Hash decode into pool Block::Hash failed: {:?}", e);
						continue;
					},
				};

			let results = join_all(
				txs.into_iter()
					.map(|tx| pool.submit_one(best_hash_pool, TransactionSource::Local, tx)),
			)
			.await;

			for res in results {
				if let Err(e) = res {
					log::error!("Failed to submit: {e:?}");
				}
			}
		}
	}
}
