use crate::rpcs::Rpc;
use crate::sdk::WaitFor;
use crate::{avail, AccountId, AvailBlocksClient, AvailConfig, BlockHash, TxApi};

use std::str::FromStr;
use subxt::blocks::ExtrinsicEvents;
use subxt_signer::sr25519::Keypair;

use avail::balances::events as BalancesEvents;
use avail::system::events as SystemEvents;

use super::options::{from_options_to_params, Options};
use super::progress_transaction_ex;

#[derive(Debug)]
pub struct TransferAllTxSuccess {
	pub event: BalancesEvents::Transfer,
	pub event2: Option<SystemEvents::KilledAccount>,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct TransferAllowDeathTxSuccess {
	pub event: BalancesEvents::Transfer,
	pub event2: Option<SystemEvents::KilledAccount>,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct TransferKeepAliveTxSuccess {
	pub event: BalancesEvents::Transfer,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Clone)]
pub struct Balances {
	api: TxApi,
	rpc_client: Rpc,
	blocks: AvailBlocksClient,
}

impl Balances {
	pub fn new(api: TxApi, rpc_client: Rpc, blocks: AvailBlocksClient) -> Self {
		Self {
			api,
			rpc_client,
			blocks,
		}
	}

	pub async fn transfer_all(
		&self,
		dest: &str,
		keep_alive: bool,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<TransferAllTxSuccess, String> {
		let dest = match AccountId::from_str(dest) {
			Ok(dest) => dest,
			Err(error) => return Err(std::format!("{:?}", error)),
		};

		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().balances().transfer_all(dest.into(), keep_alive);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<BalancesEvents::Transfer>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Transfer event"));
		};

		let event2 = events.find_first::<SystemEvents::KilledAccount>();
		let event2 = event2.ok().flatten();

		Ok(TransferAllTxSuccess {
			event,
			event2,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn transfer_allow_death(
		&self,
		dest: &str,
		amount: u128,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<TransferAllowDeathTxSuccess, String> {
		let dest = match AccountId::from_str(dest) {
			Ok(dest) => dest,
			Err(error) => return Err(std::format!("{:?}", error)),
		};

		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx()
			.balances()
			.transfer_allow_death(dest.into(), amount);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<BalancesEvents::Transfer>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Transfer event"));
		};

		let event2 = events.find_first::<SystemEvents::KilledAccount>();
		let event2 = event2.ok().flatten();

		Ok(TransferAllowDeathTxSuccess {
			event,
			event2,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn transfer_keep_alive(
		&self,
		dest: &str,
		value: u128,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<TransferKeepAliveTxSuccess, String> {
		let dest = match AccountId::from_str(dest) {
			Ok(dest) => dest,
			Err(error) => return Err(std::format!("{:?}", error)),
		};

		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx()
			.balances()
			.transfer_keep_alive(dest.into(), value);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<BalancesEvents::Transfer>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Transfer event"));
		};

		Ok(TransferKeepAliveTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}
}
