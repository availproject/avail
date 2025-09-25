use super::ExtractedTxData;
use avail_core::OpaqueExtrinsic;
use sp_std::vec::Vec;

pub trait HeaderExtensionDataFilter {
	fn filter(
		failed_transactions: &[u32],
		opaque: OpaqueExtrinsic,
		block: u32,
		tx_idx: usize,
		cols: u32,
		rows: u32,
	) -> Option<ExtractedTxData>;

	fn get_failed_transaction_ids(opaques: &[OpaqueExtrinsic]) -> Vec<u32>;
}

#[cfg(feature = "std")]
impl HeaderExtensionDataFilter for () {
	fn filter(
		_: &[u32],
		_: OpaqueExtrinsic,
		_: u32,
		_: usize,
		_: u32,
		_: u32,
	) -> Option<ExtractedTxData> {
		None
	}

	fn get_failed_transaction_ids(_: &[OpaqueExtrinsic]) -> Vec<u32> {
		Vec::new()
	}
}
#[cfg(not(feature = "std"))]
impl HeaderExtensionDataFilter for () {
	fn filter(
		_: &[u32],
		_: OpaqueExtrinsic,
		_: u32,
		_: usize,
		_: u32,
		_: u32,
	) -> Option<ExtractedTxData> {
		None
	}

	fn get_failed_transaction_ids(_: &[OpaqueExtrinsic]) -> Vec<u32> {
		Vec::new()
	}
}
