use crate::api_dev::api::runtime_types::pallet_staking::ValidatorPrefs;
use crate::api_dev::api::runtime_types::sp_arithmetic::per_things::Perbill;
use crate::rpcs::Rpc;
use crate::sdk::WaitFor;
use crate::utils_raw::fetch_transaction;
use crate::{avail, AccountId, AvailBlocksClient, AvailConfig, RewardDestination, TxApi, H256};

use std::str::FromStr;
use subxt::blocks::ExtrinsicEvents;
use subxt_core::utils::MultiAddress;
use subxt_signer::sr25519::Keypair;

use avail::staking::calls::types as StakingCalls;
use avail::staking::events as StakingEvents;

use super::options::{from_options_to_params, Options};
use super::progress_transaction_ex;

#[derive(Debug)]
pub struct BondTxSuccess {
	pub event: StakingEvents::Bonded,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: H256,
	pub tx_index: u32,
	pub block_hash: H256,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct BondExtraTxSuccess {
	pub event: StakingEvents::Bonded,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: H256,
	pub tx_index: u32,
	pub block_hash: H256,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct ChillTxSuccess {
	pub event: Option<StakingEvents::Chilled>,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: H256,
	pub tx_index: u32,
	pub block_hash: H256,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct ChillOtherTxSuccess {
	pub event: StakingEvents::Chilled,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: H256,
	pub tx_index: u32,
	pub block_hash: H256,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct NominateTxSuccess {
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_data: StakingCalls::Nominate,
	pub tx_hash: H256,
	pub tx_index: u32,
	pub block_hash: H256,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct UnbondTxSuccess {
	pub event: StakingEvents::Unbonded,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: H256,
	pub tx_index: u32,
	pub block_hash: H256,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct ValidateTxSuccess {
	pub event: StakingEvents::ValidatorPrefsSet,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: H256,
	pub tx_index: u32,
	pub block_hash: H256,
	pub block_number: u32,
}

#[derive(Clone)]
pub struct Staking {
	api: TxApi,
	rpc_client: Rpc,
	blocks: AvailBlocksClient,
}

impl Staking {
	pub fn new(api: TxApi, rpc_client: Rpc, blocks: AvailBlocksClient) -> Self {
		Self {
			api,
			rpc_client,
			blocks,
		}
	}

	pub async fn bond(
		&self,
		value: u128,
		payee: RewardDestination,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<BondTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().staking().bond(value, payee);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<StakingEvents::Bonded>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

		Ok(BondTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn bond_extra(
		&self,
		max_additional: u128,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<BondExtraTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().staking().bond_extra(max_additional);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<StakingEvents::Bonded>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

		Ok(BondExtraTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn chill(
		&self,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ChillTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().staking().chill();

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<StakingEvents::Chilled>().ok().flatten();

		Ok(ChillTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn chill_other(
		&self,
		stash: &str,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ChillOtherTxSuccess, String> {
		let stash = match AccountId::from_str(stash) {
			Ok(stash) => stash,
			Err(error) => return Err(std::format!("{:?}", error)),
		};

		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().staking().chill_other(stash);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<StakingEvents::Chilled>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Chilled event"));
		};

		Ok(ChillOtherTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn nominate(
		&self,
		targets: &[String],
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<NominateTxSuccess, String> {
		let targets: Result<Vec<AccountId>, _> = targets
			.iter()
			.map(|address| AccountId::from_str(address))
			.collect();
		let targets = targets.map_err(|e| std::format!("{:?}", e))?;
		let targets = targets.into_iter().map(|a| MultiAddress::Id(a)).collect();

		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().staking().nominate(targets);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let tx_data = tx_data_staking_nominate(block_hash, tx_hash, &self.blocks).await?;

		Ok(NominateTxSuccess {
			events,
			tx_data,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn unbond(
		&self,
		value: u128,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<UnbondTxSuccess, String> {
		let call = avail::tx().staking().unbond(value);

		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<StakingEvents::Unbonded>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Unbonded event"));
		};

		Ok(UnbondTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn validate(
		&self,
		commission: u8,
		blocked: bool,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ValidateTxSuccess, String> {
		if commission > 100 {
			return Err(String::from("Commission cannot be more than 100"));
		}

		let commission = Perbill(commission as u32);
		let perfs = ValidatorPrefs {
			commission,
			blocked,
		};

		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().staking().validate(perfs);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<StakingEvents::ValidatorPrefsSet>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find ValidatorPrefsSet event"));
		};

		Ok(ValidateTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}
}

pub async fn tx_data_staking_nominate(
	block_hash: H256,
	tx_hash: H256,
	blocks: &AvailBlocksClient,
) -> Result<StakingCalls::Nominate, String> {
	let transaction =
		fetch_transaction::<StakingCalls::Nominate>(block_hash, tx_hash, blocks).await;
	let transaction = transaction.map_err(|err| err.to_string())?;
	Ok(transaction.value)
}
