use super::{Metrics, TxData};
use avail_core::AppId;

/// It is similar to `Extractor` but it uses `C` type for calls, instead of `AppExtrinsic`.
pub trait TxDataFilter<A, C> {
	fn filter(
		caller: Option<&A>,
		call: &C,
		app_id: AppId,
		block: u32,
		tx_idx: usize,
		metrics: &mut Metrics,
	) -> Option<TxData>;

	fn process_calls(
		caller: &A,
		calls: &[C],
		app_id: AppId,
		block: u32,
		tx_idx: usize,
		metrics: &mut Metrics,
	) -> Option<TxData> {
		let tx_data = calls
			.iter()
			.filter_map(|call| Self::filter(Some(caller), call, app_id, block, tx_idx, metrics))
			.collect::<TxData>();
		(tx_data.is_empty()).then_some(tx_data)
	}
}

#[cfg(feature = "std")]
impl<A, C> TxDataFilter<A, C> for () {
	fn filter(_: Option<&A>, _: &C, _: AppId, _: u32, _: usize, _: &mut Metrics) -> Option<TxData> {
		None
	}
}
#[cfg(not(feature = "std"))]
impl<A, C> TxDataFilter<A, C> for () {
	fn filter(_: Option<&A>, _: &C, _: AppId, _: u32, _: usize, _: &mut Metrics) -> Option<TxData> {
		None
	}
}
