use crate::{AccountId, Runtime, RuntimeCall as Call};
use avail_core::{data_proof_v2::AddressedMessage, AppId};

use da_control::Call as DACall;
use frame_system::data_root::{Metrics, TxDataFilter, TxData, SubmittedData};
use pallet_utility::Call as UtilCall;
use pallet_vector::Call as VectorCall;
use sp_core::H256;
use sp_std::vec;

/// Filters and extracts `data` from `call` if it is a `DataAvailability::submit_data` type.
impl TxDataFilter<AccountId, Call> for Runtime {
	fn filter(
		caller: &AccountId,
		call: &Call,
		app_id: AppId,
		block: u32,
		tx_index: usize,
		metrics: &mut Metrics,
	) -> Option<TxData> {
		metrics.total_extrinsics += 1;

		match call {
			Call::Vector(call) => filter_vector_call(caller, call, app_id, block, tx_index, metrics),
			Call::DataAvailability(call) => filter_da_call(call, app_id, metrics),
			Call::Utility(util_call) => match util_call {
				UtilCall::batch { calls }
				| UtilCall::batch_all { calls }
				| UtilCall::force_batch { calls } => {
					Self::process_calls(caller, calls, app_id, block, tx_index, metrics)
				},
				_ => None,
			},
			_ => None,
		}
	}
}

/// Filters and extracts `data` from `calls` if internal data is not empty.
fn filter_da_call(
	call: &DACall<Runtime>,
	app_id: AppId,
	metrics: &mut Metrics,
) -> Option<TxData> {
	metrics.data_submit_extrinsics += 1;

	match call {
		DACall::submit_data { data } if !data.is_empty() => {
			metrics.data_submit_leaves += 1;
			let submitted = SubmittedData::new(app_id, data.as_slice().to_vec());
			Some(submitted.into())
		},
		_ => None,
	}
}

/// Filters and extracts message references from `call`
fn filter_vector_call(
	caller: &AccountId,
	call: &VectorCall<Runtime>,
	app_id: AppId,
	block: u32,
	tx_index: usize,
	metrics: &mut Metrics,
) -> Option<TxData> {
	match call {
		VectorCall::send_message {
			message,
			to,
			domain,
		} => {
			metrics.bridge_leaves += 1;
			let tx_index = u32::try_from(tx_index).ok()?;
			let from: [u8; 32] = *caller.as_ref();
			let addr_msg =
				AddressedMessage::new(message.clone(), H256(from), *to, *domain, block, tx_index);

			(!addr_msg.is_empty()).then_some(addr_msg.into())
		},
		VectorCall::failed_send_message_txs { failed_txs } => {
			TxData::failed_send_message_txs(failed_txs)
		},

		_ => None,
	}
}
