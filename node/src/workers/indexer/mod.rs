mod block_worker;
mod cache;
mod database;

use avail_core::OpaqueExtrinsic;
use block_rpc::{
	common::{DispatchIndex, TransactionLocation},
	transaction_overview,
};
use codec::Encode;
use da_runtime::UncheckedExtrinsic;
pub use database::DatabaseWorker;
use jsonrpsee::tokio::sync::{mpsc, Notify};
use sp_core::{Blake2Hasher, Hasher};
use sp_runtime::MultiAddress;
use std::sync::Arc;

pub use block_worker::BlockWorker;

use super::common::read_pallet_call_index;

pub type Channel = BlockDetails;
pub type Receiver = mpsc::Receiver<BlockDetails>;
pub type Sender = mpsc::Sender<BlockDetails>;

pub const BLOCK_CHANNEL_LIMIT: usize = 20_000;
pub const RPC_CHANNEL_LIMIT: usize = 20_000;
pub const DATABASE_SIZE_BUFFER: usize = 180; // in blocks. cca every one hour

#[derive(Clone, Default)]
pub struct CliDeps {
	pub enabled: bool,
	pub logging_interval: u64,
	pub block_pruning: usize,
	pub result_length: usize,
	// in kB
	pub event_cache_size: u64,
}

pub struct Deps {
	pub block_receiver: Receiver,
	pub block_sender: Sender,
	pub transaction_receiver: transaction_overview::Receiver,
	pub notifier: Arc<Notify>,
	pub cli: CliDeps,
}

#[derive(Clone)]
struct BlockTransactionSignature {
	nonce: u32,
	app_id: u32,
	tip: u128,
	ss58_address: Option<String>,
}

#[derive(Clone)]
struct BlockTransaction {
	tx_location: TransactionLocation,
	dispatch_index: DispatchIndex,
	signature: Option<BlockTransactionSignature>,
}

#[derive(Clone)]
pub struct BlockDetails {
	pub block_height: u32,
	pub finalized: bool,
	pub transactions: Vec<BlockTransaction>,
	pub new_block: bool,
}

impl BlockDetails {
	pub fn from_opaques(
		opaques: Vec<OpaqueExtrinsic>,
		block_height: u32,
		finalized: bool,
		new_block: bool,
	) -> Self {
		let mut transactions: Vec<BlockTransaction> = Vec::with_capacity(opaques.len());
		for (i, opaq) in opaques.iter().enumerate() {
			let unchecked_ext =
				match UncheckedExtrinsic::decode_no_vec_prefix(&mut opaq.0.as_slice()) {
					Ok(x) => x,
					Err(_) => continue,
				};

			let Some(ext_sig) = unchecked_ext.signature.clone() else {
				continue;
			};

			let Some(dispatch_index) = read_pallet_call_index(&unchecked_ext) else {
				continue;
			};

			let mut signature: Option<BlockTransactionSignature> = None;
			if new_block {
				let mut ss58_address = None;
				if let MultiAddress::Id(x) = ext_sig.0 {
					ss58_address = Some(std::format!("{}", x));
				}

				signature = Some(BlockTransactionSignature {
					nonce: ext_sig.2 .5 .0,
					app_id: ext_sig.2 .8 .0 .0,
					tip: ext_sig.2 .7.tip(),
					ss58_address,
				})
			}
			let tx_location = TransactionLocation {
				hash: Blake2Hasher::hash(&unchecked_ext.encode()),
				index: i as u32,
			};
			let transaction = BlockTransaction {
				tx_location,
				dispatch_index,
				signature,
			};

			transactions.push(transaction);
		}

		BlockDetails {
			block_height,
			finalized,
			transactions,
			new_block,
		}
	}
}
