use avail_core::{
	traits::{GetAppId, MaybeCaller},
	OpaqueExtrinsic,
};

use codec::Decode;
use frame_support::traits::ExtrinsicCall;
use sp_std::vec::Vec;

mod traits;
pub use traits::TxDataFilter;
mod submitted_data;
pub use submitted_data::SubmittedData;
mod tx_data;
pub use tx_data::{ExtractedTxData, TxData};
mod bridge_data;
pub use bridge_data::BridgedData;
mod metrics;
pub use metrics::{Metrics, RcMetrics};

#[cfg(test)]
mod tests;

pub fn build_tx_data<'a, F>(block: u32, extrinsics: &[Vec<u8>]) -> TxData
where
	F: TxDataFilter,
{
	let opaques: Vec<OpaqueExtrinsic> = extrinsics
		.iter()
		.filter_map(|e| OpaqueExtrinsic::from_bytes(e).ok())
		.collect();

	build_tx_data_from_opaque::<F>(block, &opaques)
}

pub fn build_tx_data_from_opaque<F>(block: u32, opaques: &[OpaqueExtrinsic]) -> TxData
where
	F: TxDataFilter,
{
	let mut metrics = Metrics::default();
	let failed_transactions = opaques
		.iter()
		.rev()
		.find_map(|o| F::get_failed_transaction_ids(o));
	let failed_transactions = failed_transactions.unwrap_or_else(|| Vec::new());

	let extracted_tx_datas: Vec<ExtractedTxData> = opaques
		.into_iter()
		.enumerate()
		.filter_map(|(idx, opaque)| {
			F::filter(
				&failed_transactions,
				opaque.clone(),
				block,
				idx,
				&mut metrics,
			)
		})
		.collect();

	TxData::from(extracted_tx_datas)
}
