use serde::{Deserialize, Serialize};
use sp_core::H256;

pub use events::{DecodedEventData, Event, Events};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct EnabledServices {
	pub transaction_overview: bool,
	pub block_overview: bool,
	pub block_data: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HashIndex {
	Hash(H256),
	Index(u32),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BlockState {
	Included,
	Finalized,
	Discarded,
}

pub mod events {
	use super::*;

	pub type Events = Vec<Event>;

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub enum DecodedEventData {
		Unknown,
		SystemExtrinsicSuccess,
		SystemExtrinsicFailed,
		SudoSudid(bool),
		SudoSudoAsDone(bool),
		MultisigMultisigExecuted(MultisigExecuted),
		ProxyProxyExecuted(bool),
		SchedulerDispatched(bool),
		DataAvailabilityDataSubmitted(DataSubmitted),
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct MultisigExecuted {
		pub multisig: String,
		pub call_hash: String,
		pub result: bool,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct DataSubmitted {
		pub who: String,
		pub data_hash: String,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct Event {
		pub index: u32,
		pub emitted_index: (u8, u8),
		pub decoded: Option<DecodedEventData>,
	}

	impl Event {
		pub fn new(index: u32, emitted_index: (u8, u8), decoded: Option<DecodedEventData>) -> Self {
			Self {
				index,
				emitted_index,
				decoded,
			}
		}
	}
}
