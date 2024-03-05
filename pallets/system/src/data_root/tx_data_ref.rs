use crate::data_root::{BridgedDataRef, SubmittedDataRef};

use derive_more::Constructor;

use sp_std::{vec, vec::Vec};

/// It contains references (no copies) of the data used to calculate roots.
/// Data referenced is from the following extrinsics:
/// - `da::submit_data`
/// - `bridge::send_message`
#[derive(Default, Constructor)]
pub struct TxDataRef<'a> {
	pub submitted: Vec<SubmittedDataRef<'a>>,
	pub bridged: Vec<BridgedDataRef<'a>>,
	pub failed_send_msg_txs: Vec<u32>,
}

impl<'a> TxDataRef<'a> {
	pub fn failed_send_msg_txs<I>(failed: I) -> Self
	where
		I: IntoIterator<Item = u32>,
	{
		let failed_send_msg_txs = failed.into_iter().collect::<Vec<_>>();
		Self {
			failed_send_msg_txs,
			..Default::default()
		}
	}

	pub fn is_empty(&self) -> bool {
		self.submitted.is_empty() && self.bridged.is_empty()
	}
}

impl<'a> From<SubmittedDataRef<'a>> for TxDataRef<'a> {
	fn from(s: SubmittedDataRef<'a>) -> Self {
		Self {
			submitted: vec![s],
			..Default::default()
		}
	}
}

impl<'a> From<BridgedDataRef<'a>> for TxDataRef<'a> {
	fn from(b: BridgedDataRef<'a>) -> Self {
		Self {
			bridged: vec![b],
			..Default::default()
		}
	}
}
