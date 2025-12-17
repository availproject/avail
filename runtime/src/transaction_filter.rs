use crate::{opaque_to_unchecked, unchecked_get_caller, AccountId, Runtime, RuntimeCall as Call};
use avail_base::header_extension::{
	BridgedData, ExtractedTxData, HeaderExtensionDataFilter, PostInherentInfo, SubmittedData,
};
use avail_core::data_proof::{tx_uid, AddressedMessage};
use sp_runtime::OpaqueExtrinsic;

use da_control::Call as DACall;
use pallet_multisig::Call as MultisigCall;
use pallet_proxy::Call as ProxyCall;
use pallet_vector::Call as VectorCall;
use sp_core::H256;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec::Vec;

const MAX_FILTER_ITERATIONS: usize = 3;

/// Filters and extracts `data` from `call` if it is a `DataAvailability::submit_data` or `Vector::send_message` type.
/// Handles N levels of nesting in case those calls are wrapped in proxy / multisig calls.
impl HeaderExtensionDataFilter for Runtime {
	fn filter(
		post_inherent_info: PostInherentInfo,
		opaque: OpaqueExtrinsic,
		block: u32,
		tx_index: usize,
	) -> Option<ExtractedTxData> {
		let res = opaque_to_unchecked(&opaque);
		match res {
			Ok(unchecked_extrinsic) => {
				let maybe_caller = unchecked_get_caller(&unchecked_extrinsic);

				let (final_call, nb_iterations) = extract_final_call(&unchecked_extrinsic.function);

				if nb_iterations > 0 {
					match final_call {
						Call::Vector(call) => filter_vector_call(
							&post_inherent_info.failed,
							maybe_caller,
							call,
							block,
							tx_index,
						),
						_ => None,
					}
				} else {
					match final_call {
						Call::Vector(call) => filter_vector_call(
							&post_inherent_info.failed,
							maybe_caller,
							call,
							block,
							tx_index,
						),
						Call::DataAvailability(call) => {
							filter_da_call(call, tx_index, post_inherent_info)
						},
						_ => None,
					}
				}
			},
			Err(_e) => {
				// ideally we should not reach heer
				// TODO: add logs
				None
			},
		}
	}

	fn get_data_from_post_inherents(opaques: &[OpaqueExtrinsic]) -> PostInherentInfo {
		let mut failed = Vec::new();
		let mut eval_proofs = BTreeMap::new();
		let len = opaques.len();
		if len == 0 {
			return PostInherentInfo::default();
		}

		// Vector failed transactions
		if let Ok(unchecked_extrinsic) = opaque_to_unchecked(&opaques[len - 1]) {
			if let Call::Vector(VectorCall::failed_send_message_txs { failed_txs }) =
				&unchecked_extrinsic.function
			{
				let failed_vector_tx = failed_txs.iter().map(|c| c.0).collect::<Vec<_>>();
				failed.extend(failed_vector_tx);
			};
		};

		if len > 1 {
			// DA submit blob failed transactions
			if let Ok(unchecked_extrinsic) = opaque_to_unchecked(&opaques[len - 2]) {
				if let Call::DataAvailability(DACall::submit_blob_txs_summary {
					total_blob_size: _,
					nb_blobs: _,
					blob_txs_summary,
				}) = &unchecked_extrinsic.function
				{
					for summary in blob_txs_summary {
						if let Some(proof) = &summary.eval_proof {
							eval_proofs.insert(summary.tx_index, proof.clone());
						}
						if !summary.success {
							failed.push(summary.tx_index);
						}
					}
				};
			}
		}

		PostInherentInfo {
			failed,
			eval_proofs,
		}
	}
}

/// Filters and extracts `data` from `calls` if internal data is not empty.
fn filter_da_call(
	call: &DACall<Runtime>,
	tx_index: usize,
	post_inherent_info: PostInherentInfo,
) -> Option<ExtractedTxData> {
	let tx_index = u32::try_from(tx_index).ok()?;
	if post_inherent_info.failed.contains(&tx_index) {
		return None;
	}

	let (app_id, blob_hash, size_bytes, commitment, eval_point_seed, eval_claim) = match call {
		DACall::submit_blob_metadata {
			app_id,
			blob_hash,
			commitment,
			size,
			eval_point_seed,
			eval_claim,
		} => {
			if commitment.is_empty() {
				return None;
			}
			(
				*app_id,
				*blob_hash,
				*size,
				commitment.clone(),
				*eval_point_seed,
				*eval_claim,
			)
		},
		_ => return None,
	};

	let tx_index = u32::try_from(tx_index).ok()?;
	let submitted_data = Some(SubmittedData::new(
		app_id,
		tx_index,
		blob_hash,
		size_bytes,
		commitment,
		eval_point_seed,
		eval_claim,
		post_inherent_info.eval_proofs.get(&tx_index).cloned(),
	));

	Some(ExtractedTxData {
		submitted_data,
		..Default::default()
	})
}

/// Filters and extracts message references from `call`
fn filter_vector_call(
	failed_transactions: &[u32],
	caller: Option<&AccountId>,
	call: &VectorCall<Runtime>,
	block: u32,
	tx_index: usize,
) -> Option<ExtractedTxData> {
	let tx_index = u32::try_from(tx_index).ok()?;
	if failed_transactions.contains(&tx_index) {
		return None;
	}

	let VectorCall::send_message {
		message,
		to,
		domain,
	} = call
	else {
		return None;
	};

	if message.is_empty() {
		return None;
	}

	let from: [u8; 32] = *caller?.as_ref();
	let id = tx_uid(block, tx_index);
	let msg = AddressedMessage::new(message.clone(), H256(from), *to, 1, *domain, id);
	let bridge_data = Some(BridgedData::new(tx_index, msg));
	Some(ExtractedTxData {
		bridge_data,
		..Default::default()
	})
}

/// Recursively unwrap Proxy/Multisig calls up to `MAX_ITERATIONS` to find `DataAvailability::submit_data` or `Vector::send_message` calls.
/// If we exceed `MAX_ITERATIONS`, we stop and return the current call.
fn extract_final_call(mut call: &Call) -> (&Call, usize) {
	let mut nb_iterations = 0;
	for i in 0..MAX_FILTER_ITERATIONS {
		nb_iterations = i;
		match call {
			Call::Proxy(proxy_call) => match proxy_call {
				ProxyCall::proxy { call: inner, .. }
				| ProxyCall::proxy_announced { call: inner, .. } => {
					call = inner;
				},
				_ => break,
			},
			Call::Multisig(multisig_call) => match multisig_call {
				MultisigCall::as_multi_threshold_1 { call: inner, .. }
				| MultisigCall::as_multi { call: inner, .. } => {
					call = inner;
				},
				_ => break,
			},
			_ => break,
		}
	}
	(call, nb_iterations)
}
