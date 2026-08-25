use crate::{opaque_to_unchecked, Runtime, RuntimeCall as Call};
use avail_base::header_extension::{
	BridgedData, ExtractedTxData, HeaderExtensionDataFilter, PostInherentInfo, SubmittedData,
};
use sp_runtime::OpaqueExtrinsic;

use da_control::Call as DACall;
use pallet_vector::Call as VectorCall;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec::Vec;

/// Filters and extracts data from `DataAvailability::submit_blob_metadata` or `Vector::send_message` calls.
/// Bridge messages are only extracted from direct `Vector::send_message` calls.
impl HeaderExtensionDataFilter for Runtime {
	fn filter(
		post_inherent_info: &PostInherentInfo,
		opaque: &OpaqueExtrinsic,
		_block: u32,
		tx_index: usize,
	) -> Option<ExtractedTxData> {
		let res = opaque_to_unchecked(&opaque);
		match res {
			Ok(unchecked_extrinsic) => {
				let tx_index = u32::try_from(tx_index).ok()?;
				if let Some(message) = post_inherent_info.successful_bridge_messages.get(&tx_index)
				{
					return Some(ExtractedTxData {
						bridge_data: Some(BridgedData::new(tx_index, message.clone())),
						..Default::default()
					});
				}

				match &unchecked_extrinsic.function {
					Call::DataAvailability(call) => {
						filter_da_call(call, tx_index as usize, post_inherent_info)
					},
					_ => None,
				}
			},
			Err(e) => {
				// ideally we should not reach heer
				// TODO: add logs
				log::error!("failed to convert opaque to uxt: {:?}", e);
				None
			},
		}
	}

	fn get_data_from_post_inherents(opaques: &[OpaqueExtrinsic]) -> PostInherentInfo {
		let mut failed = Vec::new();
		let mut eval_proofs = BTreeMap::new();
		let mut successful_bridge_messages = BTreeMap::new();
		let len = opaques.len();
		if len == 0 {
			return PostInherentInfo::default();
		}

		// Canonical bridge messages recorded only after successful runtime execution.
		if let Ok(unchecked_extrinsic) = opaque_to_unchecked(&opaques[len - 1]) {
			if let Call::Vector(VectorCall::successful_send_messages { messages }) =
				&unchecked_extrinsic.function
			{
				for (tx_index, message) in messages {
					successful_bridge_messages.insert(tx_index.0, message.clone());
				}
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
			successful_bridge_messages,
		}
	}
}

/// Filters and extracts `data` from `calls` if internal data is not empty.
fn filter_da_call(
	call: &DACall<Runtime>,
	tx_index: usize,
	post_inherent_info: &PostInherentInfo,
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
