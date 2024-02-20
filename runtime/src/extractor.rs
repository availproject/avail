use crate::{Address, Runtime, RuntimeCall, UncheckedExtrinsic};

use avail_core::OpaqueExtrinsic;
use derive_more::From;
use frame_system::submitted_data::{Extractor, Filter, Metrics, TxData, TxDataRef};

#[derive(From, Debug)]
pub enum RTEErr {
	CodecError(codec::Error),
	MissingSignature,
	InvalidAddress,
	None,
}

/// Decodes and extracts the `data` of `DataAvailability::submit_data` extrinsics.
impl Extractor for Runtime {
	type Error = RTEErr;

	fn extract<'a, 'b>(
		opaque: &'a OpaqueExtrinsic,
		block: u32,
		tx_idx: usize,
		metrics: &'b mut Metrics,
	) -> Result<TxData, Self::Error> {
		let extrinsic = UncheckedExtrinsic::try_from(opaque)?;
		let caller = extrinsic
			.signature
			.as_ref()
			.map(|s| &s.0)
			.ok_or(RTEErr::MissingSignature)?;
		let Address::Id(caller) = caller else { return Err(RTEErr::InvalidAddress); };

		<Runtime as Filter<RuntimeCall>>::filter(
			caller.clone(),
			&extrinsic.function,
			block,
			tx_idx,
			metrics,
		)
		.map(TxData::from)
		.ok_or(RTEErr::None)
	}
}
