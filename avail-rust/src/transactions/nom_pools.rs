use crate::rpcs::Rpc;
use crate::sdk::WaitFor;
use crate::utils_raw::fetch_transaction;
use crate::{avail, AccountId, AvailBlocksClient, AvailConfig, BlockHash, TxApi};

use std::str::FromStr;
use subxt::blocks::ExtrinsicEvents;
use subxt_signer::sr25519::Keypair;

use avail::nomination_pools::calls::types as NominationPoolsCalls;
use avail::nomination_pools::events as NominationPoolsEvents;

use super::options::{from_options_to_params, Options};
use super::progress_transaction_ex;

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

#[derive(Debug)]
pub struct PoolJoinTxSuccess {
	pub event: NominationPoolsEvents::Bonded,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct PoolNominateTxSuccess {
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_data: NominationPoolsCalls::Nominate,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Clone)]
pub struct NominationPools {
	api: TxApi,
	rpc_client: Rpc,
	blocks: AvailBlocksClient,
}

impl NominationPools {
	pub fn new(api: TxApi, rpc_client: Rpc, blocks: AvailBlocksClient) -> Self {
		Self {
			api,
			rpc_client,
			blocks,
		}
	}

	pub async fn nominate(
		&self,
		pool_id: u32,
		validators: Vec<String>,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<PoolNominateTxSuccess, String> {
		let validators: Result<Vec<AccountId>, _> = validators
			.into_iter()
			.map(|v| AccountId::from_str(&v))
			.collect();
		let validators = validators.map_err(|e| e.to_string())?;

		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().nomination_pools().nominate(pool_id, validators);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let tx_data = tx_data_pool_nominate(block_hash, tx_hash, &self.blocks).await?;

		Ok(PoolNominateTxSuccess {
			events,
			tx_data,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn join(
		&self,
		amount: u128,
		pool_id: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<PoolJoinTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().nomination_pools().join(amount, pool_id);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<NominationPoolsEvents::Bonded>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Created event"));
		};

		Ok(PoolJoinTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
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
		options: Option<Options>,
	) -> Result<PoolCreateWithPoolIdTxSuccess, String> {
		let root = AccountId::from_str(root).map_err(|e| e.to_string())?;
		let nominator = AccountId::from_str(nominator).map_err(|e| e.to_string())?;
		let bouncer = AccountId::from_str(bouncer).map_err(|e| e.to_string())?;

		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
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

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<NominationPoolsEvents::Created>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Created event"));
		};

		let event2 = events.find_first::<NominationPoolsEvents::Bonded>();
		let Some(event2) = event2.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

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
		options: Option<Options>,
	) -> Result<PoolCreateTxSuccess, String> {
		let root = AccountId::from_str(root).map_err(|e| e.to_string())?;
		let nominator = AccountId::from_str(nominator).map_err(|e| e.to_string())?;
		let bouncer = AccountId::from_str(bouncer).map_err(|e| e.to_string())?;

		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
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

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<NominationPoolsEvents::Created>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Created event"));
		};

		let event2 = events.find_first::<NominationPoolsEvents::Bonded>();
		let Some(event2) = event2.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

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

pub async fn tx_data_pool_nominate(
	block_hash: BlockHash,
	tx_hash: BlockHash,
	blocks: &AvailBlocksClient,
) -> Result<NominationPoolsCalls::Nominate, String> {
	let transaction =
		fetch_transaction::<NominationPoolsCalls::Nominate>(block_hash, tx_hash, blocks).await;
	let transaction = transaction.map_err(|err| err.to_string())?;
	Ok(transaction.value)
}
