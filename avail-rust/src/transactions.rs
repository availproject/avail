use crate::api_dev::api::data_availability::calls::types::create_application_key::Key;
use crate::api_dev::api::data_availability::calls::types::submit_data::Data;
use crate::api_dev::api::runtime_types::frame_support::dispatch::DispatchFeeModifier;
use crate::api_dev::api::runtime_types::pallet_staking::ValidatorPrefs;
use crate::api_dev::api::runtime_types::sp_arithmetic::per_things::Perbill;
use crate::api_dev::api::Call;
use crate::avail::runtime_types::da_runtime::primitives::SessionKeys;
use crate::sdk::WaitFor;
use crate::utils_raw::progress_transaction;
use crate::{
	avail, transaction_data, AccountId, Api, AvailBlocksClient, AvailConfig, BlockHash,
	RewardDestination, TxApi,
};

use std::str::FromStr;
use subxt::blocks::ExtrinsicEvents;
use subxt_core::utils::MultiAddress;
use subxt_signer::sr25519::Keypair;

use avail::balances::events as BalancesEvents;
use avail::data_availability::events as DataAvailabilityEvents;
use avail::staking::events as StakingEvents;
use avail::sudo::events as SudoEvents;
use avail::system::events as SystemEvents;

pub type Params =
	<<AvailConfig as subxt::Config>::ExtrinsicParams as subxt::config::ExtrinsicParams<
		AvailConfig,
	>>::Params;

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

#[derive(Debug)]
pub struct BondTxSuccess {
	pub event: StakingEvents::Bonded,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct BondExtraTxSuccess {
	pub event: StakingEvents::Bonded,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct ChillTxSuccess {
	pub event: Option<StakingEvents::Chilled>,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct ChillOtherTxSuccess {
	pub event: StakingEvents::Chilled,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct NominateTxSuccess {
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_data: transaction_data::staking::Nominate,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct UnbondTxSuccess {
	pub event: StakingEvents::Unbonded,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct ValidateTxSuccess {
	pub event: StakingEvents::ValidatorPrefsSet,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct SubmitDataTxSuccess {
	pub event: DataAvailabilityEvents::DataSubmitted,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_data: transaction_data::data_availability::SubmitData,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct CreateApplicationKeyTxSuccess {
	pub event: DataAvailabilityEvents::ApplicationKeyCreated,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct SetApplicationKeyTxSuccess {
	pub event: DataAvailabilityEvents::ApplicationKeySet,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct SubmitBlockLengthProposalTxSuccess {
	pub event: DataAvailabilityEvents::BlockLengthProposalSubmitted,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct SetSubmitDataFeeModifierTxSuccess {
	pub event: DataAvailabilityEvents::SubmitDataFeeModifierSet,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct SetKeysTxSuccess {
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_data: transaction_data::session::SetKeys,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Clone)]
pub struct Transactions {
	pub balances: Balances,
	pub staking: Staking,
	pub data_availability: DataAvailability,
	pub session: Session,
}

impl Transactions {
	pub fn new(api: Api) -> Self {
		let tx = api.tx();
		let blocks = api.blocks();

		Self {
			balances: Balances::new(tx.clone(), blocks.clone()),
			staking: Staking::new(tx.clone(), blocks.clone()),
			data_availability: DataAvailability::new(tx.clone(), blocks.clone()),
			session: Session::new(tx.clone(), blocks),
		}
	}
}

async fn get_block_number(
	blocks: &AvailBlocksClient,
	block_hash: BlockHash,
) -> Result<u32, String> {
	let block_number = blocks
		.at(block_hash)
		.await
		.map_err(|e| e.to_string())?
		.number();

	Ok(block_number)
}

#[derive(Clone)]
pub struct Session {
	api: TxApi,
	blocks: AvailBlocksClient,
}

impl Session {
	pub fn new(api: TxApi, blocks: AvailBlocksClient) -> Self {
		Self { api, blocks }
	}

	pub async fn set_keys(
		&self,
		keys: SessionKeys,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Params>,
	) -> Result<SetKeysTxSuccess, String> {
		let params = options.unwrap_or_default();
		let call = avail::tx().session().set_keys(keys, vec![]);

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

		let tx_hash = tx_in_block.extrinsic_hash();
		let tx_index = events.extrinsic_index();
		let block_hash = tx_in_block.block_hash();
		let block_number = get_block_number(&self.blocks, block_hash).await?;

		let tx_data =
			transaction_data::session::SetKeys::new(block_hash, tx_hash, &self.blocks).await?;

		Ok(SetKeysTxSuccess {
			events,
			tx_data,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}
}

#[derive(Clone)]
pub struct Balances {
	api: TxApi,
	blocks: AvailBlocksClient,
}

impl Balances {
	pub fn new(api: TxApi, blocks: AvailBlocksClient) -> Self {
		Self { api, blocks }
	}

	pub async fn transfer_all(
		&self,
		dest: &str,
		keep_alive: bool,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Params>,
	) -> Result<TransferAllTxSuccess, String> {
		let dest = match AccountId::from_str(dest) {
			Ok(dest) => dest,
			Err(error) => return Err(std::format!("{:?}", error)),
		};

		let params = options.unwrap_or_default();
		let call = avail::tx().balances().transfer_all(dest.into(), keep_alive);

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

		let event = events.find_first::<BalancesEvents::Transfer>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Transfer event"));
		};

		let event2 = events.find_first::<SystemEvents::KilledAccount>();
		let event2 = event2.ok().flatten();

		let tx_hash = tx_in_block.extrinsic_hash();
		let tx_index = events.extrinsic_index();
		let block_hash = tx_in_block.block_hash();
		let block_number = get_block_number(&self.blocks, block_hash).await?;

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
		options: Option<Params>,
	) -> Result<TransferAllowDeathTxSuccess, String> {
		let dest = match AccountId::from_str(dest) {
			Ok(dest) => dest,
			Err(error) => return Err(std::format!("{:?}", error)),
		};

		let params = options.unwrap_or_default();
		let call = avail::tx()
			.balances()
			.transfer_allow_death(dest.into(), amount);

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

		let event = events.find_first::<BalancesEvents::Transfer>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Transfer event"));
		};

		let event2 = events.find_first::<SystemEvents::KilledAccount>();
		let event2 = event2.ok().flatten();

		let tx_hash = tx_in_block.extrinsic_hash();
		let tx_index = events.extrinsic_index();
		let block_hash = tx_in_block.block_hash();
		let block_number = get_block_number(&self.blocks, block_hash).await?;

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
		options: Option<Params>,
	) -> Result<TransferKeepAliveTxSuccess, String> {
		let dest = match AccountId::from_str(dest) {
			Ok(dest) => dest,
			Err(error) => return Err(std::format!("{:?}", error)),
		};

		let params = options.unwrap_or_default();
		let call = avail::tx()
			.balances()
			.transfer_keep_alive(dest.into(), value);

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

		let event = events.find_first::<BalancesEvents::Transfer>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Transfer event"));
		};

		let tx_hash = tx_in_block.extrinsic_hash();
		let tx_index = events.extrinsic_index();
		let block_hash = tx_in_block.block_hash();
		let block_number = get_block_number(&self.blocks, block_hash).await?;

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

#[derive(Clone)]
pub struct Staking {
	api: TxApi,
	blocks: AvailBlocksClient,
}

impl Staking {
	pub fn new(api: TxApi, blocks: AvailBlocksClient) -> Self {
		Self { api, blocks }
	}

	pub async fn bond(
		&self,
		value: u128,
		payee: RewardDestination,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Params>,
	) -> Result<BondTxSuccess, String> {
		let params = options.unwrap_or_default();
		let call = avail::tx().staking().bond(value, payee);

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

		let event = events.find_first::<StakingEvents::Bonded>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

		let tx_hash = tx_in_block.extrinsic_hash();
		let tx_index = events.extrinsic_index();
		let block_hash = tx_in_block.block_hash();
		let block_number = get_block_number(&self.blocks, block_hash).await?;

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
		options: Option<Params>,
	) -> Result<BondExtraTxSuccess, String> {
		let params = options.unwrap_or_default();
		let call = avail::tx().staking().bond_extra(max_additional);

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

		let event = events.find_first::<StakingEvents::Bonded>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

		let tx_hash = tx_in_block.extrinsic_hash();
		let tx_index = events.extrinsic_index();
		let block_hash = tx_in_block.block_hash();
		let block_number = get_block_number(&self.blocks, block_hash).await?;

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
		options: Option<Params>,
	) -> Result<ChillTxSuccess, String> {
		let params = options.unwrap_or_default();
		let call = avail::tx().staking().chill();

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

		let event = events.find_first::<StakingEvents::Chilled>().ok().flatten();

		let tx_hash = tx_in_block.extrinsic_hash();
		let tx_index = events.extrinsic_index();
		let block_hash = tx_in_block.block_hash();
		let block_number = get_block_number(&self.blocks, block_hash).await?;

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
		options: Option<Params>,
	) -> Result<ChillOtherTxSuccess, String> {
		let stash = match AccountId::from_str(stash) {
			Ok(stash) => stash,
			Err(error) => return Err(std::format!("{:?}", error)),
		};

		let params = options.unwrap_or_default();
		let call = avail::tx().staking().chill_other(stash);

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

		let event = events.find_first::<StakingEvents::Chilled>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Chilled event"));
		};

		let tx_hash = tx_in_block.extrinsic_hash();
		let tx_index = events.extrinsic_index();
		let block_hash = tx_in_block.block_hash();
		let block_number = get_block_number(&self.blocks, block_hash).await?;

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
		options: Option<Params>,
	) -> Result<NominateTxSuccess, String> {
		let targets: Result<Vec<AccountId>, _> = targets
			.iter()
			.map(|address| AccountId::from_str(address))
			.collect();
		let targets = targets.map_err(|e| std::format!("{:?}", e))?;
		let targets = targets.into_iter().map(|a| MultiAddress::Id(a)).collect();

		let params = options.unwrap_or_default();
		let call = avail::tx().staking().nominate(targets);

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

		let (tx_hash, block_hash) = (tx_in_block.extrinsic_hash(), tx_in_block.block_hash());

		let tx_data =
			transaction_data::staking::Nominate::new(block_hash, tx_hash, &self.blocks).await?;

		let tx_hash = tx_in_block.extrinsic_hash();
		let tx_index = events.extrinsic_index();
		let block_hash = tx_in_block.block_hash();
		let block_number = get_block_number(&self.blocks, block_hash).await?;

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
		options: Option<Params>,
	) -> Result<UnbondTxSuccess, String> {
		let call = avail::tx().staking().unbond(value);

		let params = options.unwrap_or_default();
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

		let event = events.find_first::<StakingEvents::Unbonded>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Unbonded event"));
		};

		let tx_hash = tx_in_block.extrinsic_hash();
		let tx_index = events.extrinsic_index();
		let block_hash = tx_in_block.block_hash();
		let block_number = get_block_number(&self.blocks, block_hash).await?;

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
		options: Option<Params>,
	) -> Result<ValidateTxSuccess, String> {
		if commission > 100 {
			return Err(String::from("Commission cannot be more than 100"));
		}

		let commission = Perbill(commission as u32);
		let perfs = ValidatorPrefs {
			commission,
			blocked,
		};

		let params = options.unwrap_or_default();
		let call = avail::tx().staking().validate(perfs);

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

		let event = events.find_first::<StakingEvents::ValidatorPrefsSet>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find ValidatorPrefsSet event"));
		};

		let tx_hash = tx_in_block.extrinsic_hash();
		let tx_index = events.extrinsic_index();
		let block_hash = tx_in_block.block_hash();
		let block_number = get_block_number(&self.blocks, block_hash).await?;

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

#[derive(Clone)]
pub struct DataAvailability {
	api: TxApi,
	blocks: AvailBlocksClient,
}

impl DataAvailability {
	pub fn new(api: TxApi, blocks: AvailBlocksClient) -> Self {
		Self { api, blocks }
	}

	pub async fn submit_data(
		&self,
		data: Data,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Params>,
	) -> Result<SubmitDataTxSuccess, String> {
		let params = options.unwrap_or_default();
		let call = avail::tx().data_availability().submit_data(data);

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

		let event = events.find_first::<DataAvailabilityEvents::DataSubmitted>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find DataSubmitted event"));
		};

		let (tx_hash, block_hash) = (tx_in_block.extrinsic_hash(), tx_in_block.block_hash());

		let tx_data =
			transaction_data::data_availability::SubmitData::new(block_hash, tx_hash, &self.blocks)
				.await?;

		let tx_hash = tx_in_block.extrinsic_hash();
		let tx_index = events.extrinsic_index();
		let block_hash = tx_in_block.block_hash();
		let block_number = get_block_number(&self.blocks, block_hash).await?;

		Ok(SubmitDataTxSuccess {
			event,
			events,
			tx_data,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn create_application_key(
		&self,
		key: Key,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Params>,
	) -> Result<CreateApplicationKeyTxSuccess, String> {
		let params = options.unwrap_or_default();
		let call = avail::tx().data_availability().create_application_key(key);

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

		let event = events.find_first::<DataAvailabilityEvents::ApplicationKeyCreated>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find ApplicationKeyCreated event"));
		};

		let tx_hash = tx_in_block.extrinsic_hash();
		let tx_index = events.extrinsic_index();
		let block_hash = tx_in_block.block_hash();
		let block_number = get_block_number(&self.blocks, block_hash).await?;

		Ok(CreateApplicationKeyTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn set_application_key(
		&self,
		old_key: Key,
		new_key: Key,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Params>,
	) -> Result<SetApplicationKeyTxSuccess, String> {
		let params = options.unwrap_or_default();
		let call = Call::DataAvailability(
			avail::runtime_types::da_control::pallet::Call::set_application_key {
				old_key,
				new_key,
			},
		);
		let sudo = avail::tx().sudo().sudo(call);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&sudo, account, params)
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

		let sudo_event = events.find_first::<SudoEvents::Sudid>();
		let Some(sudo_event) = sudo_event.ok().flatten() else {
			return Err(String::from("Failed to find Sudid event"));
		};

		if let Err(error) = sudo_event.sudo_result {
			return Err(std::format!("{:?}", error));
		}

		let event = events.find_first::<DataAvailabilityEvents::ApplicationKeySet>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find ApplicationKeySet event"));
		};

		let tx_hash = tx_in_block.extrinsic_hash();
		let tx_index = events.extrinsic_index();
		let block_hash = tx_in_block.block_hash();
		let block_number = get_block_number(&self.blocks, block_hash).await?;

		Ok(SetApplicationKeyTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn submit_block_length_proposal(
		&self,
		rows: u32,
		cols: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Params>,
	) -> Result<SubmitBlockLengthProposalTxSuccess, String> {
		let params = options.unwrap_or_default();
		let call = Call::DataAvailability(
			avail::runtime_types::da_control::pallet::Call::submit_block_length_proposal {
				rows,
				cols,
			},
		);
		let sudo = avail::tx().sudo().sudo(call);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&sudo, account, params)
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

		let sudo_event = events.find_first::<SudoEvents::Sudid>();
		let Some(sudo_event) = sudo_event.ok().flatten() else {
			return Err(String::from("Failed to find Sudid event"));
		};

		if let Err(error) = sudo_event.sudo_result {
			return Err(std::format!("{:?}", error));
		}

		let event = events.find_first::<DataAvailabilityEvents::BlockLengthProposalSubmitted>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from(
				"Failed to find BlockLengthProposalSubmitted event",
			));
		};

		let tx_hash = tx_in_block.extrinsic_hash();
		let tx_index = events.extrinsic_index();
		let block_hash = tx_in_block.block_hash();
		let block_number = get_block_number(&self.blocks, block_hash).await?;

		Ok(SubmitBlockLengthProposalTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn set_submit_data_fee_modifier(
		&self,
		modifier: DispatchFeeModifier,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Params>,
	) -> Result<SetSubmitDataFeeModifierTxSuccess, String> {
		let params = options.unwrap_or_default();
		let call = Call::DataAvailability(
			avail::runtime_types::da_control::pallet::Call::set_submit_data_fee_modifier {
				modifier,
			},
		);
		let sudo = avail::tx().sudo().sudo(call);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&sudo, account, params)
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

		let sudo_event = events.find_first::<SudoEvents::Sudid>();
		let Some(sudo_event) = sudo_event.ok().flatten() else {
			return Err(String::from("Failed to find Sudid event"));
		};

		if let Err(error) = sudo_event.sudo_result {
			return Err(std::format!("{:?}", error));
		}

		let event = events.find_first::<DataAvailabilityEvents::SubmitDataFeeModifierSet>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from(
				"Failed to find SubmitDataFeeModifierSet event",
			));
		};

		let tx_hash = tx_in_block.extrinsic_hash();
		let tx_index = events.extrinsic_index();
		let block_hash = tx_in_block.block_hash();
		let block_number = get_block_number(&self.blocks, block_hash).await?;

		Ok(SetSubmitDataFeeModifierTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn testing_function() {
		let sdk = crate::sdk::SDK::new("ws://127.0.0.1:9944").await.unwrap();
		let h = BlockHash::from_str(
			"0x6c5ebed687ed008b76028072fe1ad0a06ecf3c00dd9067aa049ea14e180702f8",
		)
		.unwrap();
		match sdk.rpc.kate.query_rows(vec![0], Some(h)).await {
			Ok(a) => {
				dbg!(a);
			},
			Err(a) => {
				dbg!(a);
			},
		};
	}
}
