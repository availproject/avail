use crate::sdk::WaitFor;
use crate::utils_raw::progress_transaction;
use crate::{avail, AccountId, AvailBlocksClient, AvailConfig, BlockHash, TxApi};

use std::str::FromStr;
use subxt::blocks::ExtrinsicEvents;
use subxt_signer::sr25519::Keypair;

use avail::nomination_pools::events as NominationPoolsEvents;

use super::{block_and_tx_hash, Params};

#[derive(Debug)]
pub struct PoolCreateTxSuccess {
	pub event: NominationPoolsEvents::Created,
	pub event2: NominationPoolsEvents::Bonded,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct PoolCreateWithPoolIdTxSuccess {
	pub event: NominationPoolsEvents::Created,
	pub event2: NominationPoolsEvents::Bonded,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Clone)]
pub struct NominationPools {
	api: TxApi,
	blocks: AvailBlocksClient,
}

impl NominationPools {
	pub fn new(api: TxApi, blocks: AvailBlocksClient) -> Self {
		Self { api, blocks }
	}

	pub async fn create_with_pool_id(
		&self,
		amount: u128,
		root: &str,
		nominator: &str,
		bouncer: &str,
		pool_id: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Params>,
	) -> Result<PoolCreateWithPoolIdTxSuccess, String> {
		let root = AccountId::from_str(root).map_err(|e| e.to_string())?;
		let nominator = AccountId::from_str(nominator).map_err(|e| e.to_string())?;
		let bouncer = AccountId::from_str(bouncer).map_err(|e| e.to_string())?;

		let params = options.unwrap_or_default();
		let call = avail::tx().nomination_pools().create_with_pool_id(
			amount,
			root.into(),
			nominator.into(),
			bouncer.into(),
			pool_id,
		);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let transaction = progress_transaction(maybe_tx_progress, wait_for).await;
		let tx_in_block = match transaction {
			Ok(tx_in_block) => tx_in_block,
			Err(message) => return Err(message),
		};

		let events = match tx_in_block.wait_for_success().await {
			Ok(e) => e,
			Err(error) => return Err(error.to_string()),
		};

		let event = events.find_first::<NominationPoolsEvents::Created>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Created event"));
		};

		let event2 = events.find_first::<NominationPoolsEvents::Bonded>();
		let Some(event2) = event2.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

		let (block_hash, block_number, tx_hash, tx_index) =
			block_and_tx_hash(&tx_in_block, &events, &self.blocks).await?;

		Ok(PoolCreateWithPoolIdTxSuccess {
			event,
			event2,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn create(
		&self,
		amount: u128,
		root: &str,
		nominator: &str,
		bouncer: &str,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Params>,
	) -> Result<PoolCreateTxSuccess, String> {
		let root = AccountId::from_str(root).map_err(|e| e.to_string())?;
		let nominator = AccountId::from_str(nominator).map_err(|e| e.to_string())?;
		let bouncer = AccountId::from_str(bouncer).map_err(|e| e.to_string())?;

		let params = options.unwrap_or_default();
		let call = avail::tx().nomination_pools().create(
			amount,
			root.into(),
			nominator.into(),
			bouncer.into(),
		);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let transaction = progress_transaction(maybe_tx_progress, wait_for).await;
		let tx_in_block = match transaction {
			Ok(tx_in_block) => tx_in_block,
			Err(message) => return Err(message),
		};

		let events = match tx_in_block.wait_for_success().await {
			Ok(e) => e,
			Err(error) => return Err(error.to_string()),
		};

		let event = events.find_first::<NominationPoolsEvents::Created>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Created event"));
		};

		let event2 = events.find_first::<NominationPoolsEvents::Bonded>();
		let Some(event2) = event2.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

		let (block_hash, block_number, tx_hash, tx_index) =
			block_and_tx_hash(&tx_in_block, &events, &self.blocks).await?;

		Ok(PoolCreateTxSuccess {
			event,
			event2,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}
}
