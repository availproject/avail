use super::{Metrics, TxDataRef};

/// It is similar to `Extractor` but it uses `C` type for calls, instead of `AppExtrinsic`.
pub trait TxDataFilter<A, C> {
	fn filter<'a>(
		caller: &'a A,
		call: &'a C,
		block: u32,
		tx_idx: usize,
		metrics: &'_ mut Metrics,
	) -> Option<TxDataRef<'a>>;

	fn process_calls<'a>(
		caller: &'a A,
		calls: &'a [C],
		block: u32,
		tx_idx: usize,
		metrics: &'_ mut Metrics,
	) -> Option<TxDataRef<'a>> {
		let tx_data = calls
			.iter()
			.filter_map(|call| Self::filter(caller, call, block, tx_idx, metrics))
			.collect::<TxDataRef>();
		(tx_data.is_empty()).then_some(tx_data)
	}
}

#[cfg(feature = "std")]
impl<A, C> TxDataFilter<A, C> for () {
	fn filter<'a>(
		_: &'a A,
		_: &'a C,
		_: u32,
		_: usize,
		_: &'_ mut Metrics,
	) -> Option<TxDataRef<'a>> {
		None
	}
}
