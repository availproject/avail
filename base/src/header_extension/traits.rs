use super::ExtractedTxData;
use avail_core::OpaqueExtrinsic;

pub trait HeaderExtensionDataFilter {
	fn filter(
		failed_transactions: &[u32],
		opaque: OpaqueExtrinsic,
		block: u32,
		tx_idx: usize,
	) -> Option<ExtractedTxData>;

	fn get_failed_transaction_ids(opaque: &OpaqueExtrinsic) -> Option<Vec<u32>>;
}

#[cfg(feature = "std")]
impl HeaderExtensionDataFilter for () {
	fn filter(_: &[u32], _: OpaqueExtrinsic, _: u32, _: usize) -> Option<ExtractedTxData> {
		None
	}

	fn get_failed_transaction_ids(_: &OpaqueExtrinsic) -> Option<Vec<u32>> {
		None
	}
}
#[cfg(not(feature = "std"))]
impl HeaderExtensionDataFilter for () {
	fn filter(_: &[u32], _: OpaqueExtrinsic, _: u32, _: usize) -> Option<ExtractedTxData> {
		None
	}

	fn get_failed_transaction_ids(_: &OpaqueExtrinsic) -> Option<Vec<u32>> {
		None
	}
}
