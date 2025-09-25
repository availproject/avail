use crate::{AccountId, Runtime, RuntimeCall as Call, UncheckedExtrinsic};
use avail_base::header_extension::{
	BridgedData, ExtractedTxData, HeaderExtensionDataFilter, SubmittedData,
};
use avail_core::{
	data_proof::{tx_uid, AddressedMessage},
	traits::{GetAppId, MaybeCaller},
	AppExtrinsic, AppId, OpaqueExtrinsic,
};

use da_control::Call as DACall;
use kate::Seed;
use pallet_multisig::Call as MultisigCall;
use pallet_proxy::Call as ProxyCall;
use pallet_vector::Call as VectorCall;
use sp_core::H256;
use sp_io::hashing::keccak_256;
use sp_std::vec::Vec;

const MAX_FILTER_ITERATIONS: usize = 3;

/// Filters and extracts `data` from `call` if it is a `DataAvailability::submit_data` or `Vector::send_message` type.
/// Handles N levels of nesting in case those calls are wrapped in proxy / multisig calls.
impl HeaderExtensionDataFilter for Runtime {
	fn filter(
		failed_transactions: &[u32],
		opaque: OpaqueExtrinsic,
		block: u32,
		tx_index: usize,
		cols: u32,
		rows: u32,
	) -> Option<ExtractedTxData> {
		let Ok(unchecked_extrinsic) = UncheckedExtrinsic::try_from(opaque) else {
			return None;
		};

		let app_id = unchecked_extrinsic.app_id();
		let maybe_caller = unchecked_extrinsic.caller();

		let (final_call, nb_iterations) = extract_final_call(&unchecked_extrinsic.function);

		if nb_iterations > 0 {
			match final_call {
				Call::Vector(call) => {
					filter_vector_call(failed_transactions, maybe_caller, call, block, tx_index)
				},
				_ => None,
			}
		} else {
			match final_call {
				Call::Vector(call) => {
					filter_vector_call(failed_transactions, maybe_caller, call, block, tx_index)
				},
				Call::DataAvailability(call) => {
					let app_extrinsic = AppExtrinsic::from(unchecked_extrinsic.clone());
					filter_da_call(
						app_extrinsic,
						call,
						app_id,
						tx_index,
						failed_transactions,
						cols,
						rows,
					)
				},
				_ => None,
			}
		}
	}

	fn get_failed_transaction_ids(opaques: &[OpaqueExtrinsic]) -> Vec<u32> {
		let mut failed_tx = Vec::new();
		let len = opaques.len();
		if len == 0 {
			return failed_tx;
		}

		// Vector failed transactions
		if let Ok(unchecked_extrinsic) = UncheckedExtrinsic::try_from(opaques[len - 1].clone()) {
			if let Call::Vector(VectorCall::failed_send_message_txs { failed_txs }) =
				&unchecked_extrinsic.function
			{
				let failed_vector_tx = failed_txs.iter().map(|c| c.0).collect::<Vec<_>>();
				failed_tx.extend(failed_vector_tx);
			};
		};

		if len > 1 {
			// DA submit blob failed transactions
			if let Ok(unchecked_extrinsic) = UncheckedExtrinsic::try_from(opaques[len - 2].clone())
			{
				if let Call::DataAvailability(DACall::submit_blob_txs_summary {
					total_blob_size: _,
					nb_blobs: _,
					blob_txs_summary,
				}) = &unchecked_extrinsic.function
				{
					let failed_tx_da: Vec<u32> = blob_txs_summary
						.iter()
						.filter(|summary| !summary.success)
						.map(|summary| summary.tx_index)
						.collect();

					failed_tx.extend(failed_tx_da);
				};
			}
		}

		failed_tx
	}
}

/// Filters and extracts `data` from `calls` if internal data is not empty.
fn filter_da_call(
	app_extrinsic: AppExtrinsic,
	call: &DACall<Runtime>,
	app_id: AppId,
	tx_index: usize,
	failed_transactions: &[u32],
	cols: u32,
	rows: u32,
) -> Option<ExtractedTxData> {
	let tx_index = u32::try_from(tx_index).ok()?;
	if failed_transactions.contains(&tx_index) {
		return None;
	}

	let (blob_hash, commitment) = match call {
		DACall::submit_blob_metadata {
			blob_hash,
			commitment,
			size: _,
		} => {
			if commitment.is_empty() {
				return None;
			}
			(*blob_hash, commitment.clone())
		},
		DACall::submit_data { data } => {
			if data.is_empty() {
				return None;
			}
			let blob_hash = H256(keccak_256(&data));
			let commitment =
				da_control::extensions::native::hosted_commitment_builder::build_da_commitments(
					data,
					cols,
					rows,
					Seed::default(),
				);
			(blob_hash, commitment)
		},
		_ => return None,
	};

	let tx_index = u32::try_from(tx_index).ok()?;
	let submitted_data = Some(SubmittedData::new(app_id, tx_index, blob_hash, commitment));

	Some(ExtractedTxData {
		submitted_data,
		app_extrinsic: Some(app_extrinsic),
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
