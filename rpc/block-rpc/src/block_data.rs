use super::{BlockIdentifier, BlockState, HashIndex};
use crate::common::{DispatchIndex, EmittedIndex, TransactionLocation};
use frame_system_rpc_runtime_api::RuntimePhase;
use jsonrpsee::tokio::sync::{mpsc, oneshot};
use serde::{Deserialize, Serialize};
use sp_core::H256;

pub use filter::*;

pub type ChannelResponse = oneshot::Sender<Result<Response, String>>;
pub type Channel = (Params, ChannelResponse);
pub type Receiver = mpsc::Receiver<Channel>;
pub type Sender = mpsc::Sender<Channel>;

#[derive(Clone, Serialize, Deserialize)]
pub struct Params {
	pub block_index: HashIndex,
	#[serde(default)]
	pub fetch_calls: bool,
	#[serde(default)]
	pub fetch_events: bool,
	#[serde(default)]
	pub call_filter: CallFilter,
	#[serde(default)]
	pub event_filter: EventFilter,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Response {
	pub block_id: BlockIdentifier,
	pub block_state: BlockState,
	pub calls: Option<Vec<CallData>>,
	pub events: Option<Vec<EventData>>,
}

impl Response {
	pub fn new(
		block_id: BlockIdentifier,
		block_state: BlockState,
		calls: Option<Vec<CallData>>,
		events: Option<Vec<EventData>>,
	) -> Self {
		Self {
			block_id,
			block_state,
			calls,
			events,
		}
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ResponseDebug {
	pub value: Response,
	pub debug_execution_time: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CallData {
	pub tx_location: TransactionLocation,
	pub dispatch_index: DispatchIndex,
	pub call: String,
}

impl CallData {
	pub fn new(
		tx_location: TransactionLocation,
		dispatch_index: DispatchIndex,
		call: String,
	) -> Self {
		Self {
			tx_location,
			dispatch_index,
			call,
		}
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EventData {
	// (pallet id, event id)
	pub emitted_index: EmittedIndex,
	pub phase: RuntimePhase,
	pub event: String,
}

impl EventData {
	pub fn new(emitted_index: EmittedIndex, phase: RuntimePhase, event: String) -> Self {
		Self {
			emitted_index,
			phase,
			event,
		}
	}
}

pub mod filter {
	pub use super::*;

	#[derive(Default, Clone, Serialize, Deserialize)]
	pub struct CallFilter {
		#[serde(default)]
		pub transaction: TransactionFilterOptions,
		#[serde(default)]
		pub signature: SignatureFilterOptions,
	}

	#[derive(Clone, Serialize, Deserialize)]
	pub enum TransactionFilterOptions {
		All,
		TxHash(Vec<H256>),
		TxIndex(Vec<u32>),
		Pallet(Vec<u8>),
		PalletCall(Vec<DispatchIndex>),
	}

	impl TransactionFilterOptions {
		pub fn filter_in_pallet(&self, value: u8) -> Option<()> {
			let TransactionFilterOptions::Pallet(list) = self else {
				return Some(());
			};
			list.contains(&value).then_some(())
		}

		pub fn filter_in_pallet_call(&self, value: DispatchIndex) -> Option<()> {
			let TransactionFilterOptions::PalletCall(list) = self else {
				return Some(());
			};
			list.contains(&value).then_some(())
		}

		pub fn filter_in_tx_hash(&self, value: H256) -> Option<()> {
			let TransactionFilterOptions::TxHash(list) = self else {
				return Some(());
			};
			list.contains(&value).then_some(())
		}

		pub fn filter_in_tx_index(&self, value: u32) -> Option<()> {
			let TransactionFilterOptions::TxIndex(list) = self else {
				return Some(());
			};
			list.contains(&value).then_some(())
		}
	}

	impl Default for TransactionFilterOptions {
		fn default() -> Self {
			Self::All
		}
	}

	#[derive(Default, Clone, Serialize, Deserialize)]
	pub struct SignatureFilterOptions {
		pub ss58_address: Option<String>,
		pub app_id: Option<u32>,
		pub nonce: Option<u32>,
	}

	impl SignatureFilterOptions {
		pub fn filter_in_ss58_address(&self, value: Option<String>) -> Option<()> {
			if self.ss58_address.is_none() {
				return Some(());
			}
			(self.ss58_address == value).then_some(())
		}

		pub fn filter_in_app_id(&self, value: Option<u32>) -> Option<()> {
			if self.app_id.is_none() {
				return Some(());
			}
			(self.app_id == value).then_some(())
		}

		pub fn filter_in_nonce(&self, value: Option<u32>) -> Option<()> {
			if self.nonce.is_none() {
				return Some(());
			}
			(self.nonce == value).then_some(())
		}
	}

	#[derive(Debug, Default, Clone, Serialize, Deserialize)]
	pub struct EventFilter {
		#[serde(default)]
		pub phase: PhaseFilterOptions,
		#[serde(default)]
		pub pallet: PalletFilterOptions,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub enum PhaseFilterOptions {
		All,
		TxIndex(Vec<u32>),
		OnlyConsensus,
	}

	impl Default for PhaseFilterOptions {
		fn default() -> Self {
			Self::All
		}
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub enum PalletFilterOptions {
		All,
		Pallet(u8),
		Combination(Vec<DispatchIndex>),
	}

	impl Default for PalletFilterOptions {
		fn default() -> Self {
			Self::All
		}
	}
}
