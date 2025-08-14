use crate::{
	p2p::BlobHandle,
	send_blob_query_request,
	store::BlobStore,
	types::{Blob, BlobMetadata, BlobNotification, BlobReceived, Deps, OwnershipEntry},
	utils::{
		build_signature_payload, check_store_blob, generate_base_index, get_my_validator_id,
		get_validator_per_blob, sign_blob_data_inner,
	},
	BLOB_TTL, MAX_BLOB_SIZE, MAX_RPC_RETRIES, MAX_TRANSACTION_VALIDITY, MIN_TRANSACTION_VALIDITY,
	TEMP_BLOB_TTL,
};
use anyhow::{anyhow, Result};
use codec::{Decode, Encode};
use da_commitment::build_da_commitments::build_da_commitments;
use da_control::{pallet::BlobTxSummaryRuntime, Call};
use da_runtime::{RuntimeCall, UncheckedExtrinsic};
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::ErrorObject,
};
use kate::Seed;
use sc_authority_discovery::AuthorityDiscovery;
use sc_client_api::{BlockBackend, HeaderBackend};
use sc_network::{NetworkStateInfo, PeerId};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_authority_discovery::AuthorityId;
use sp_core::{keccak_256, Bytes, H256};
use sp_runtime::transaction_validity::TransactionSource;
use sp_runtime::{traits::Block as BlockT, SaturatedConversion};
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use std::{
	marker::{PhantomData, Sync},
	str::FromStr,
	sync::Arc,
};
use tokio::{task, try_join};
pub enum Error {
	BlobError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::BlobError => 1,
		}
	}
}

macro_rules! internal_err {
    ($($arg:tt)*) => {{
        ErrorObject::owned(
            Error::BlobError.into(),
            format!($($arg)*),
            None::<()>
        )
    }}
}

#[rpc(client, server)]
pub trait BlobApi<Block>
where
	Block: BlockT,
{
	#[method(name = "blob_submitBlob")]
	async fn submit_blob(&self, metadata_signed_transaction: Bytes, blob: Bytes) -> RpcResult<()>;

	#[method(name = "blob_getBlob")]
	async fn get_blob(
		&self,
		block_hash: Block::Hash,
		blob_index: u32,
		blob_hash: H256,
	) -> RpcResult<Blob>;

	#[method(name = "blob_logStuff")]
	async fn log_stuff(&self) -> RpcResult<()>;
}

pub struct BlobRpc<Client, Pool, Block: BlockT> {
	client: Arc<Client>,
	pool: Arc<Pool>,
	blob_handle: Arc<BlobHandle<Block>>,
	_block: PhantomData<Block>,
}

impl<Client, Pool, Block: BlockT> BlobRpc<Client, Pool, Block> {
	pub fn new(client: Arc<Client>, pool: Arc<Pool>, deps: Deps<Block>) -> Self {
		Self {
			client,
			pool,
			blob_handle: deps.blob_handle,
			_block: PhantomData,
		}
	}
}

#[async_trait]
impl<Client, Pool, Block> BlobApiServer<Block> for BlobRpc<Client, Pool, Block>
where
	Block: BlockT,
	Client: HeaderBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ AuthorityDiscovery<Block>
		+ Send
		+ Sync
		+ 'static,
	Client::Api: TaggedTransactionQueue<Block>,
	Pool: TransactionPool<Block = Block> + 'static,
{
	async fn submit_blob(&self, metadata_signed_transaction: Bytes, blob: Bytes) -> RpcResult<()> {
		// --- 0. Quick checks -------------------------------------------------
		if blob.0.is_empty() {
			return Err(internal_err!("blob cannot be empty"));
		}
		if blob.0.len() as u64 > MAX_BLOB_SIZE {
			return Err(internal_err!("blob is too big"));
		}
		if metadata_signed_transaction.0.is_empty() {
			return Err(internal_err!("metadata tx cannot be empty"));
		}

		// Decode to concrete call to read the metadata, Check hash, commitment, ... of the blob compared to the submitted metadata ----------------
		let encoded_metadata_signed_transaction: UncheckedExtrinsic =
			Decode::decode(&mut &metadata_signed_transaction[..])
				.map_err(|_| internal_err!("failed to decode concrete metadata call"))?;
		let (provided_size, provided_blob_hash, provided_commitment) =
			match encoded_metadata_signed_transaction.function {
				RuntimeCall::DataAvailability(Call::submit_blob_metadata {
					size,
					blob_hash,
					commitment,
				}) => (size as usize, blob_hash, commitment),
				_ => {
					return Err(internal_err!(
						"metadata extrinsic must be dataAvailability.submitBlobMetadata"
					))
				},
			};

		// Check size
		if provided_size != blob.len() {
			return Err(internal_err!(
				"submit data length ({}) != blob length ({})",
				provided_size,
				blob.len()
			));
		}

		// Check blob_hash
		let blob_hash = H256::from(keccak_256(&blob));
		if provided_blob_hash != blob_hash {
			return Err(internal_err!("submitted blob: {provided_blob_hash:?} does not correspond to generated blob {blob_hash:?}"));
		}

		// Prepare generated commitment
		let blob_vec: Vec<u8> = blob.to_vec();
		let commitment_fut = async {
			task::spawn_blocking({
				let blob_vec = blob_vec.clone();
				// TODO Blob take values from the runtime
				move || build_da_commitments(blob_vec, 1024, 4096, Seed::default())
			})
			.await
			.map_err(|e| internal_err!("Join error: {e}"))
		};

		// Check tx validity
		let client_info = self.client.info();
		let best_hash = client_info.best_hash;
		let pre_validate_fut = async {
			// --- a. Decode the opaque extrinsic ---------------------------------
			let opaque_tx: Block::Extrinsic =
				codec::Decode::decode(&mut &metadata_signed_transaction[..])
					.map_err(|_| internal_err!("failed to decode metadata extrinsic"))?;
			// --- b. Let the runtime validate it (signature, nonce, weight) ------
			let validity = self
				.client
				.runtime_api()
				.validate_transaction(
					best_hash,
					TransactionSource::External,
					opaque_tx.clone(),
					best_hash,
				)
				.map_err(|e| internal_err!("runtime validate_transaction error: {e}"))?;
			let Ok(validity) = validity else {
				return Err(internal_err!("metadata extrinsic rejected by runtime"));
			};
			// --- c. Check also that transaction lifetime is above minimum tx lifetime so it does not expire. If validity is not correct, we reject the tx
			if validity.longevity < MIN_TRANSACTION_VALIDITY {
				return Err(internal_err!(
					"signed transaction does not live for enough time"
				));
			}
			if validity.longevity > MAX_TRANSACTION_VALIDITY {
				return Err(internal_err!("signed transaction lifetime is too long"));
			}
			Ok(opaque_tx)
		};

		let (opaque_tx, commitment) = try_join!(pre_validate_fut, commitment_fut)?;

		// Check comitment
		if provided_commitment != commitment {
			return Err(internal_err!("submitted blob commitment mismatch"));
		}

		// From this point, the transaction should not fail as the user has done everything correctly
		// We will spawn a task to finish the work and instantly return to the user.
		let blob_handle = self.blob_handle.clone();
		let client = self.client.clone();
		let pool = self.pool.clone();
		let network = self.blob_handle.network.get().cloned();
		task::spawn(async move {
			// Get my own peer id data
			let Some(net) = network else {
				log::error!("submit_blob(bg): network not initialized");
				return;
			};
			let my_peer_id_base58 = net.local_peer_id().to_base58();

			// Get client info
			let finalized_block_number = client_info.finalized_number.saturated_into::<u64>();
			let finalized_block_hash = client_info.finalized_hash;

			// Setup blob metadata and blob and check first in case we already received this exact blob before
			let maybe_blob_metadata = match blob_handle.blob_store.get_blob_metadata(&blob_hash) {
				Ok(m) => m,
				Err(e) => {
					log::error!("Failed to get data from blob storage: {e}");
					return;
				},
			};

			let mut blob_metadata = maybe_blob_metadata.unwrap_or_else(|| {
				let blob_len = blob.len();

				BlobMetadata {
					hash: blob_hash,
					size: blob_len.saturated_into(),
					commitment,
					is_notified: true,
					expires_at: 0,
					finalized_block_hash: Block::Hash::default(),
					finalized_block_number: 0,
					nb_validators_per_blob: 0,
				}
			});

			// It might be a new blob or an old one being resubmitted, we still update most of the values
			let validators = match client.authorities(finalized_block_hash).await {
				Ok(v) => v,
				Err(e) => {
					log::error!("failed to get validators from runtime: {e}");
					return;
				},
			};
			let nb_validators_per_blob = get_validator_per_blob(validators.len() as u32);
			blob_metadata.is_notified = true;
			blob_metadata.expires_at = finalized_block_number.saturating_add(TEMP_BLOB_TTL);
			blob_metadata.finalized_block_hash = finalized_block_hash;
			blob_metadata.finalized_block_number = finalized_block_number;
			blob_metadata.nb_validators_per_blob = nb_validators_per_blob;
			let maybe_ownership: Option<OwnershipEntry> =
				match check_rpc_store_blob::<Client, Block>(
					&blob_handle,
					&blob_metadata,
					my_peer_id_base58.clone(),
					nb_validators_per_blob,
					&validators,
				)
				.await
				{
					Ok(o) => o,
					Err(e) => {
						log::error!("could not check if rpc should store blob: {e}");
						return;
					},
				};

			let blob = Blob {
				blob_hash,
				size: blob.len().saturated_into(),
				data: blob_vec,
			};

			if let Some(o) = &maybe_ownership {
				if let Err(e) = blob_handle.blob_store.insert_blob_ownership(&blob_hash, o) {
					log::error!("failed to insert blob ownership into store: {e}");
				}
				blob_metadata.expires_at = finalized_block_number.saturating_add(BLOB_TTL);
			}

			// Store the blob in the store -------------------
			if let Err(e) = blob_handle.blob_store.insert_blob_metadata(&blob_metadata) {
				log::error!("failed to insert blob metadata into store: {e}");
			}

			if let Err(e) = blob_handle.blob_data_store.insert_blob(&blob) {
				log::error!("failed to insert blob into store: {e}");
			}

			// Announce the blob to the network -------------------
			let blob_received_notification: BlobNotification<Block> =
				BlobNotification::BlobReceived(BlobReceived {
					hash: blob_metadata.hash,
					size: blob_metadata.size,
					commitment: blob_metadata.commitment.clone(),
					ownership: maybe_ownership,
					original_peer_id: my_peer_id_base58.clone(),
					finalized_block_hash,
					finalized_block_number,
				});

			let Some(gossip_cmd_sender) = blob_handle.gossip_cmd_sender.get() else {
				log::error!("gossip_cmd_sender was not initialized");
				return;
			};

			if let Err(e) = gossip_cmd_sender.send(blob_received_notification).await {
				log::error!("internal channel closed: {e}");
				return;
			}

			// Push the clean extrinsic to the tx pool ---------------------
			// Get the best hash once more, to submit the tx
			let best_hash = client.info().best_hash;
			if let Err(e) = pool
				.submit_one(best_hash, TransactionSource::External, opaque_tx)
				.await
			{
				log::error!("tx-pool error: {e}")
			}
		});

		Ok(())
	}

	async fn get_blob(
		&self,
		block_hash: Block::Hash,
		blob_index: u32,
		blob_hash: H256,
	) -> RpcResult<Blob> {
		let network = self
			.blob_handle
			.network
			.get()
			.ok_or_else(|| internal_err!("Could not get network to get my peer id"))
			.map_err(|e| e)?;

		let Ok(Some(summaries)) = get_blob_tx_summaries_from_block(&self.client, block_hash).await
		else {
			return Err(internal_err!(
				"Blob transactions summaries not found in the given block"
			));
		};

		let Some(blob_summary) = summaries.get(blob_index as usize) else {
			return Err(internal_err!(
				"Blob transaction summary not found in the given block and blob_index"
			));
		};

		if blob_summary.hash != blob_hash {
			return Err(internal_err!(
				"Blob hash mismatch - Expected: {:?} - Found: {:?}",
				blob_hash,
				blob_summary.hash
			));
		}

		if !blob_summary.success {
			return Err(internal_err!(
				"Blob update was not successful: {:?}",
				blob_summary.reason
			));
		}

		let ownership_len = blob_summary.ownership.len();
		if ownership_len == 0 {
			return Err(internal_err!("Blob ownership is empty"));
		}

		let finalized_hash = self.client.info().finalized_hash;

		// Take a random owner index and try to get the blob from him, retry with next ones
		let base_index =
			match generate_base_index(blob_hash, &finalized_hash.encode(), ownership_len, None) {
				Ok(i) => i,
				Err(e) => {
					return Err(internal_err!(
						"An error has occured while generating a base index: {e:?}"
					));
				},
			};

		for attempt in 0..(MAX_RPC_RETRIES as usize) {
			let index = (base_index + attempt) % ownership_len;
			let Some((_, encoded_peer_id, _)) = blob_summary.ownership.get(index) else {
				log::warn!(
					"Attempt {}/{}: invalid array index",
					attempt + 1,
					MAX_RPC_RETRIES
				);
				continue;
			};

			match PeerId::from_str(&encoded_peer_id) {
				Ok(peer_id) => match send_blob_query_request(blob_hash, peer_id, &network).await {
					Ok(Some(blob)) => {
						return Ok(blob);
					},
					Ok(None) => {
						log::warn!(
							"attempt {}/{}: no blob returned",
							attempt + 1,
							MAX_RPC_RETRIES
						);
					},
					Err(e) => {
						log::warn!(
							"attempt {}/{} RPC error: {:?}",
							attempt + 1,
							MAX_RPC_RETRIES,
							e
						);
					},
				},
				Err(e) => {
					log::warn!("Attempt {}: invalid peer_id {:?}", attempt + 1, e);
				},
			}
		}

		Err(internal_err!(
			"All attempts to get the blob from validators failed."
		))
	}

	async fn log_stuff(&self) -> RpcResult<()> {
		let _ = self.blob_handle.blob_store.log_all_entries();
		Ok(())
	}
}

async fn check_rpc_store_blob<Client, Block>(
	blob_handle: &BlobHandle<Block>,
	blob_metadata: &BlobMetadata<Block>,
	my_encoded_peer_id: String,
	nb_validators_per_blob: u32,
	validators: &Vec<AuthorityId>,
) -> Result<Option<OwnershipEntry>>
where
	Block: BlockT,
	Client: HeaderBackend<Block> + ProvideRuntimeApi<Block> + AuthorityDiscovery<Block>,
{
	let role = &blob_handle.role;
	if !role.is_authority() {
		// RPC node (me) is not an authority, so I don't have to store blobs
		return Ok(None);
	}

	let keystore = blob_handle.keystore.get();
	let Some(keystore) = keystore else {
		return Err(anyhow!("failed to get keystore"));
	};

	let Some(my_validator_id) = get_my_validator_id(&keystore) else {
		return Ok(None);
	};

	let should_store_blob = match check_store_blob(
		blob_metadata.hash,
		validators,
		&my_validator_id,
		&blob_metadata.finalized_block_hash.encode(),
		nb_validators_per_blob,
	) {
		Ok(s) => s,
		Err(e) => {
			return Err(anyhow!("failed to check if I should hold a blob: {e}"));
		},
	};

	if !should_store_blob {
		return Ok(None);
	}

	let signature_payload = build_signature_payload(blob_metadata.hash, b"stored".to_vec());
	let signature = match sign_blob_data_inner(keystore, signature_payload) {
		Ok(s) => s.signature,
		Err(e) => {
			return Err(anyhow!(
				"An error has occured while trying to sign data, exiting the function: {e}"
			));
		},
	};

	Ok(Some(OwnershipEntry {
		address: my_validator_id,
		encoded_peer_id: my_encoded_peer_id,
		signature,
	}))
}

async fn get_blob_tx_summaries_from_block<Client, Block>(
	client: &Arc<Client>,
	block_hash: Block::Hash,
) -> RpcResult<Option<Vec<BlobTxSummaryRuntime>>>
where
	Block: BlockT,
	<Block as BlockT>::Extrinsic: Encode,
	Client: HeaderBackend<Block> + BlockBackend<Block> + Send + Sync + 'static,
{
	let block = client
		.block(block_hash)
		.map_err(|e| internal_err!("Failed to get block: {e}"))?
		.ok_or_else(|| internal_err!("Block not found"))?;

	let extrinsics = block.block.extrinsics();
	if extrinsics.len() < 2 {
		return Ok(None);
	}

	let summary_extrinsic_encoded = extrinsics[extrinsics.len() - 2].encode();
	let summary_extrinsic: UncheckedExtrinsic = Decode::decode(&mut &summary_extrinsic_encoded[..])
		.map_err(|_| internal_err!("Failed to decode summary extrinsic"))?;

	if let RuntimeCall::DataAvailability(Call::submit_blob_txs_summary {
		blob_txs_summary, ..
	}) = summary_extrinsic.function
	{
		Ok(Some(blob_txs_summary))
	} else {
		Ok(None)
	}
}
