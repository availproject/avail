use super::{common, BlockId, BlockState, HashIndex};
use crate::common::EmittedIndex;
use crate::common::{DispatchIndex, TransactionLocation};
use jsonrpsee::tokio::sync::{mpsc, oneshot};
use serde::{Deserialize, Serialize};
use sp_core::H256;

pub use events::*;
pub use filter::*;

pub type ChannelResponse = oneshot::Sender<Result<Response, String>>;
pub type Channel = (Params, ChannelResponse);
pub type Receiver = mpsc::Receiver<Channel>;
pub type Sender = mpsc::Sender<Channel>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
	pub block_index: HashIndex,
	#[serde(default)]
	pub extension: ParamsExtension,
	#[serde(default)]
	pub filter: Filter,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct ParamsExtension {
	#[serde(default)]
	pub enable_call_decoding: bool,
	#[serde(default)]
	pub fetch_events: bool,
	#[serde(default)]
	pub enable_event_decoding: bool,
	#[serde(default)]
	pub enable_consensus_event: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
	pub block_id: BlockId,
	pub block_state: BlockState,
	pub transactions: Vec<TransactionData>,
	pub consensus_events: Option<ConsensusEvents>,
}

impl Response {
	pub fn new(
		block_id: BlockId,
		block_state: BlockState,
		transactions: Vec<TransactionData>,
		consensus_events: Option<ConsensusEvents>,
	) -> Self {
		Self {
			block_id,
			block_state,
			transactions,
			consensus_events,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseDebug {
	pub value: Response,
	pub debug_execution_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
	pub location: TransactionLocation,
	pub dispatch_index: DispatchIndex,
	pub signature: Option<TransactionSignature>,
	pub decoded: Option<String>,
	pub events: Option<common::Events>,
}

pub mod events {
	pub use super::*;
	pub type ConsensusEvents = Vec<ConsensusEvent>;

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct ConsensusEvent {
		pub phase: ConsensusEventPhase,
		pub emitted_index: EmittedIndex,
		pub decoded: Option<String>,
	}

	#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
	pub enum ConsensusEventPhase {
		// Finalizing the block.
		Finalization,
		/// Initializing the block.
		Initialization,
	}
}

pub mod filter {
	use crate::common::EmittedIndex;

	pub use super::*;

	#[derive(Debug, Default, Clone, Serialize, Deserialize)]
	pub struct Filter {
		#[serde(default)]
		pub transaction: TransactionFilterOptions,
		#[serde(default)]
		pub signature: SignatureFilterOptions,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub enum TransactionFilterOptions {
		All,
		TxHash(Vec<H256>),
		TxIndex(Vec<u32>),
		Pallet(Vec<u8>),
		PalletCall(Vec<DispatchIndex>),
		HasEvent(Vec<EmittedIndex>),
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

		pub fn is_has_events(&self) -> bool {
			let TransactionFilterOptions::HasEvent(..) = self else {
				return false;
			};
			true
		}
	}

	impl Default for TransactionFilterOptions {
		fn default() -> Self {
			Self::All
		}
	}

	#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionSignature {
	pub ss58_address: Option<String>,
	pub nonce: u32,
	pub app_id: u32,
	pub mortality: Option<(u64, u64)>, // None means the tx is Immortal
}
