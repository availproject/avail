use crate::{
	api::runtime_types::da_control::pallet::Call as DaCall,
	avail::{Extrinsics, RuntimeCall},
	primitives::AppUncheckedExtrinsic,
};

use avail_base::data_root::{SubmittedData, TxData};
use avail_core::AppExtrinsic;
use codec::{Decode, Encode};

pub mod democracy {
	use derive_more::Constructor;
	use num_enum::TryFromPrimitive;

	/// A number of lock periods, plus a vote, one way or the other.
	#[derive(Copy, Clone, Eq, PartialEq, Default, Constructor)]
	pub struct Vote {
		pub aye: bool,
		pub conviction: Conviction,
	}

	#[repr(u8)]
	#[derive(Copy, Clone, Eq, PartialEq, Default, TryFromPrimitive)]
	pub enum Conviction {
		/// 0.1x votes, unlocked.
		#[default]
		None = 0,
		/// 1x votes, locked for an enactment period following a successful vote.
		Locked1x,
		/// 2x votes, locked for 2x enactment periods following a successful vote.
		Locked2x,
		/// 3x votes, locked for 4x...
		Locked3x,
		/// 4x votes, locked for 8x...
		Locked4x,
		/// 5x votes, locked for 16x...
		Locked5x,
		/// 6x votes, locked for 32x...
		Locked6x,
	}
}

pub fn to_app_extrinsics(extrinsics: &Extrinsics) -> Vec<AppExtrinsic> {
	extrinsics
		.iter()
		.filter_map(|ext_details| {
			let raw = ext_details.ok()?.bytes().encode();
			AppExtrinsic::decode(&mut raw.as_slice()).ok()
		})
		.collect()
}

pub fn submitted_data_from(extrinsics: &Extrinsics) -> Vec<SubmittedData> {
	let tx_data = extrinsics
		.iter()
		.enumerate()
		.filter_map(|(tx_idx, details)| {
			let extrinsic = AppUncheckedExtrinsic::try_from(details.ok()?).ok()?;
			let app_id = extrinsic.app_id();
			if let RuntimeCall::DataAvailability(DaCall::submit_data { data }) = extrinsic.function
			{
				if !data.0.is_empty() {
					let tx_idx = u32::try_from(tx_idx).ok()?;
					let submitted = SubmittedData::new(app_id, tx_idx, data.0);
					return Some(submitted.into());
				}
			}
			None
		})
		.collect::<TxData>();

	tx_data.submitted
}
