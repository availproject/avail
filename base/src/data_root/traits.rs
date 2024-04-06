use super::{ExtractedTxData, Metrics};
use avail_core::OpaqueExtrinsic;

/// It is similar to `Extractor` but it uses `C` type for calls, instead of `AppExtrinsic`.
pub trait TxDataFilter {
	fn filter(
		failed_transactions: &[u32],
		opaque: OpaqueExtrinsic,
		block: u32,
		tx_idx: usize,
		metrics: &mut Metrics,
	) -> Option<ExtractedTxData>;

	fn get_failed_transaction_ids(opaque: &OpaqueExtrinsic) -> Option<Vec<u32>>;
}

#[cfg(feature = "std")]
impl TxDataFilter for () {
	fn filter(
		_: &[u32],
		_: OpaqueExtrinsic,
		_: u32,
		_: usize,
		_: &mut Metrics,
	) -> Option<ExtractedTxData> {
		None
	}

	fn get_failed_transaction_ids(_: &OpaqueExtrinsic) -> Option<Vec<u32>> {
		None
	}
}
#[cfg(not(feature = "std"))]
impl TxDataFilter for () {
	fn filter(
		_: &[u32],
		_: OpaqueExtrinsic,
		_: u32,
		_: usize,
		_: &mut Metrics,
	) -> Option<ExtractedTxData> {
		None
	}

	fn get_failed_transaction_ids(_: &OpaqueExtrinsic) -> Option<Vec<u32>> {
		None
	}
}
