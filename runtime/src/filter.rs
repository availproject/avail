use crate::{AccountId, Runtime, RuntimeCall as Call};
use avail_core::data_proof_v2::AddressedMessageRef;

use da_control::Call as DACall;
use frame_system::submitted_data::{Metrics, TxDataFilter, TxDataRef};
use pallet_utility::Call as UtilCall;
use pallet_vector::Call as VectorCall;
use sp_core::H256;
use sp_std::vec;

/// Filters and extracts `data` from `call` if it is a `DataAvailability::submit_data` type.
impl TxDataFilter<AccountId, Call> for Runtime {
	fn filter<'a, 'b>(
		caller: &'a AccountId,
		call: &'a Call,
		block: u32,
		tx_index: usize,
		metrics: &'b mut Metrics,
	) -> Option<TxDataRef<'a>> {
		metrics.total_extrinsics += 1;

		match call {
			Call::Vector(call) => filter_vector_call(caller, call, block, tx_index, metrics),
			Call::DataAvailability(call) => filter_da_call(call, metrics),
			Call::Utility(util_call) => match util_call {
				UtilCall::batch { calls }
				| UtilCall::batch_all { calls }
				| UtilCall::force_batch { calls } => {
					Self::process_calls(caller, calls, block, tx_index, metrics)
				},
				_ => None,
			},
			_ => None,
		}
	}
}

/// Filters and extracts `data` from `calls` if internal data is not empty.
fn filter_da_call<'a, 'b>(
	call: &'a DACall<Runtime>,
	metrics: &'b mut Metrics,
) -> Option<TxDataRef<'a>> {
	metrics.data_submit_extrinsics += 1;

	match call {
		DACall::submit_data { data } if !data.is_empty() => {
			metrics.data_submit_leaves += 1;
			Some(TxDataRef::new(vec![data.as_slice()], vec![]))
		},
		_ => None,
	}
}

/// Filters and extracts message references from `call`
fn filter_vector_call<'a, 'b>(
	caller: &'a AccountId,
	call: &'a VectorCall<Runtime>,
	block: u32,
	tx_index: usize,
	metrics: &'b mut Metrics,
) -> Option<TxDataRef<'a>> {
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
				AddressedMessageRef::new(&message, H256(from), *to, *domain, block, tx_index);

			(!addr_msg.is_empty()).then_some(TxDataRef::new(vec![], vec![addr_msg]))
		},
		_ => None,
	}
}
