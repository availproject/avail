use super::{ExtractedTxData, PostInherentInfo};
use sp_runtime::OpaqueExtrinsic;

pub trait HeaderExtensionDataFilter {
	fn filter(
		post_inherent_info: &PostInherentInfo,
		opaque: &OpaqueExtrinsic,
		block: u32,
		tx_idx: usize,
	) -> Option<ExtractedTxData>;

	fn get_data_from_post_inherents(opaques: &[OpaqueExtrinsic]) -> PostInherentInfo;
}

#[cfg(feature = "std")]
impl HeaderExtensionDataFilter for () {
	fn filter(
		_: &PostInherentInfo,
		_: &OpaqueExtrinsic,
		_: u32,
		_: usize,
	) -> Option<ExtractedTxData> {
		None
	}

	fn get_data_from_post_inherents(_: &[OpaqueExtrinsic]) -> PostInherentInfo {
		PostInherentInfo::default()
	}
}
#[cfg(not(feature = "std"))]
impl HeaderExtensionDataFilter for () {
	fn filter(
		_: &PostInherentInfo,
		_: &OpaqueExtrinsic,
		_: u32,
		_: usize,
	) -> Option<ExtractedTxData> {
		None
	}

	fn get_data_from_post_inherents(_: &[OpaqueExtrinsic]) -> PostInherentInfo {
		PostInherentInfo::default()
	}
}
