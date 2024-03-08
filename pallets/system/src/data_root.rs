use avail_core::traits::{GetAppId, MaybeCaller};

use codec::Decode;
use frame_support::traits::ExtrinsicCall;
use sp_std::vec::Vec;

mod traits;
pub use traits::TxDataFilter;
mod submitted_data;
pub use submitted_data::SubmittedData;
mod tx_data;
pub use tx_data::TxData;
mod bridge_data;
pub use bridge_data::BridgedData;
mod metrics;
pub use metrics::{Metrics, RcMetrics};

#[cfg(test)]
mod tests;

pub fn build_tx_data<'a, F, E, A, I>(block: u32, extrinsics: I) -> TxData
where
	F: TxDataFilter<A, E::Call>,
	E: ExtrinsicCall + MaybeCaller<A> + GetAppId + Decode,
	I: Iterator<Item = &'a Vec<u8>> + 'a,
{
	let mut metrics = Metrics::default();

	extrinsics
		.enumerate()
		.filter_map(|(idx, raw_extrinsic)| {
			let ext = E::decode(&mut raw_extrinsic.as_slice()).ok()?;
			let caller = ext.caller()?;
			let app_id = ext.app_id();
			let call = ext.call();
			F::filter(caller, call, app_id, block, idx, &mut metrics)
		})
		.collect::<TxData>()
}
