use crate::api_dev::api::data_availability::calls::types::create_application_key::Key;
use crate::api_dev::api::data_availability::calls::types::submit_data::Data;
use crate::api_dev::api::runtime_types::frame_support::dispatch::DispatchFeeModifier;
use crate::api_dev::api::runtime_types::pallet_staking::ValidatorPrefs;
use crate::api_dev::api::runtime_types::sp_arithmetic::per_things::Perbill;
use crate::api_dev::api::Call;
use crate::sdk::WaitFor;
use crate::{
	avail, transaction_data, AccountId, Api, AvailBlocksClient, AvailConfig, RewardDestination,
	TransactionInBlock, TxApi, H256,
};

use std::str::FromStr;
use subxt::tx::{TxProgress, TxStatus};
use subxt::utils::MultiAddress;
use subxt_signer::sr25519::Keypair;

use avail::balances::events as BalancesEvents;
use avail::data_availability::events as DataAvailabilityEvents;
use avail::staking::events as StakingEvents;
use avail::sudo::events as SudoEvents;
use avail::system::events as SystemEvents;

pub struct TransferAllTxSuccess {
	pub event: BalancesEvents::Transfer,
	pub event2: Option<SystemEvents::KilledAccount>,
	pub tx_hash: H256,
	pub block_hash: H256,
}

pub struct TransferAllowDeathTxSuccess {
	pub event: BalancesEvents::Transfer,
	pub event2: Option<SystemEvents::KilledAccount>,
	pub tx_hash: H256,
	pub block_hash: H256,
}

pub struct TransferKeepAliveTxSuccess {
	pub event: BalancesEvents::Transfer,
	pub tx_hash: H256,
	pub block_hash: H256,
}

pub struct BondTxSuccess {
	pub event: StakingEvents::Bonded,
	pub tx_hash: H256,
	pub block_hash: H256,
}

pub struct BondExtraTxSuccess {
	pub event: StakingEvents::Bonded,
	pub tx_hash: H256,
	pub block_hash: H256,
}

pub struct ChillTxSuccess {
	pub event: Option<StakingEvents::Chilled>,
	pub tx_hash: H256,
	pub block_hash: H256,
}

pub struct ChillOtherTxSuccess {
	pub event: StakingEvents::Chilled,
	pub tx_hash: H256,
	pub block_hash: H256,
}

pub struct NominateTxSuccess {
	pub tx_data: transaction_data::staking::Nominate,
	pub tx_hash: H256,
	pub block_hash: H256,
}

pub struct UnbondTxSuccess {
	pub event: StakingEvents::Unbonded,
	pub tx_hash: H256,
	pub block_hash: H256,
}

pub struct ValidateTxSuccess {
	pub event: StakingEvents::ValidatorPrefsSet,
	pub tx_hash: H256,
	pub block_hash: H256,
}

pub struct SubmitDataTxSuccess {
	pub event: DataAvailabilityEvents::DataSubmitted,
	pub tx_data: transaction_data::data_availability::SubmitData,
	pub tx_hash: H256,
	pub block_hash: H256,
}

pub struct CreateApplicationKeyTxSuccess {
	pub event: DataAvailabilityEvents::ApplicationKeyCreated,
	pub tx_hash: H256,
	pub block_hash: H256,
}

pub struct SetApplicationKeyTxSuccess {
	pub event: DataAvailabilityEvents::ApplicationKeySet,
	pub tx_hash: H256,
	pub block_hash: H256,
}

pub struct SubmitBlockLengthProposalTxSuccess {
	pub event: DataAvailabilityEvents::BlockLengthProposalSubmitted,
	pub tx_hash: H256,
	pub block_hash: H256,
}

pub struct SetSubmitDataFeeModifierTxSuccess {
	pub event: DataAvailabilityEvents::SubmitDataFeeModifierSet,
	pub tx_hash: H256,
	pub block_hash: H256,
}

pub struct Transactions {
	pub balances: Balances,
	pub staking: Staking,
	pub data_availability: DataAvailability,
}

impl Transactions {
	pub fn new(api: Api) -> Self {
		let tx = api.tx();
		let blocks = api.blocks();

		Self {
			balances: Balances::new(tx.clone()),
			staking: Staking::new(tx.clone(), blocks.clone()),
			data_availability: DataAvailability::new(tx.clone(), blocks),
		}
	}
}

pub struct Balances {
	api: TxApi,
}

impl Balances {
	pub fn new(api: TxApi) -> Self {
		Self { api }
	}

	pub async fn transfer_all(
		&self,
		dest: &str,
		keep_alive: bool,
		wait_for: WaitFor,
		account: &Keypair,
	) -> Result<TransferAllTxSuccess, String> {
		let dest = match AccountId::from_str(dest) {
			Ok(dest) => dest,
			Err(error) => return Err(std::format!("{:?}", error)),
		};

		let call = avail::tx().balances().transfer_all(dest.into(), keep_alive);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch_default(&call, account)
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

		Ok(TransferAllTxSuccess {
			event,
			event2,
			tx_hash: tx_in_block.extrinsic_hash(),
			block_hash: tx_in_block.block_hash(),
		})
	}

	pub async fn transfer_allow_death(
		&self,
		dest: &str,
		amount: u128,
		wait_for: WaitFor,
		account: &Keypair,
	) -> Result<TransferAllowDeathTxSuccess, String> {
		let dest = match AccountId::from_str(dest) {
			Ok(dest) => dest,
			Err(error) => return Err(std::format!("{:?}", error)),
		};

		let call = avail::tx()
			.balances()
			.transfer_allow_death(dest.into(), amount);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch_default(&call, account)
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

		Ok(TransferAllowDeathTxSuccess {
			event,
			event2,
			tx_hash: tx_in_block.extrinsic_hash(),
			block_hash: tx_in_block.block_hash(),
		})
	}

	pub async fn transfer_keep_alive(
		&self,
		dest: &str,
		value: u128,
		wait_for: WaitFor,
		account: &Keypair,
	) -> Result<TransferKeepAliveTxSuccess, String> {
		let dest = match AccountId::from_str(dest) {
			Ok(dest) => dest,
			Err(error) => return Err(std::format!("{:?}", error)),
		};

		let call = avail::tx()
			.balances()
			.transfer_keep_alive(dest.into(), value);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch_default(&call, account)
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

		Ok(TransferKeepAliveTxSuccess {
			event,
			tx_hash: tx_in_block.extrinsic_hash(),
			block_hash: tx_in_block.block_hash(),
		})
	}
}

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
	) -> Result<BondTxSuccess, String> {
		let call = avail::tx().staking().bond(value, payee);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch_default(&call, account)
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

		Ok(BondTxSuccess {
			event,
			tx_hash: tx_in_block.extrinsic_hash(),
			block_hash: tx_in_block.block_hash(),
		})
	}

	pub async fn bond_extra(
		&self,
		max_additional: u128,
		wait_for: WaitFor,
		account: &Keypair,
	) -> Result<BondExtraTxSuccess, String> {
		let call = avail::tx().staking().bond_extra(max_additional);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch_default(&call, account)
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

		Ok(BondExtraTxSuccess {
			event,
			tx_hash: tx_in_block.extrinsic_hash(),
			block_hash: tx_in_block.block_hash(),
		})
	}

	pub async fn chill(
		&self,
		wait_for: WaitFor,
		account: &Keypair,
	) -> Result<ChillTxSuccess, String> {
		let call = avail::tx().staking().chill();

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch_default(&call, account)
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

		Ok(ChillTxSuccess {
			event,
			tx_hash: tx_in_block.extrinsic_hash(),
			block_hash: tx_in_block.block_hash(),
		})
	}

	pub async fn chill_other(
		&self,
		stash: &str,
		wait_for: WaitFor,
		account: &Keypair,
	) -> Result<ChillOtherTxSuccess, String> {
		let stash = match AccountId::from_str(stash) {
			Ok(stash) => stash,
			Err(error) => return Err(std::format!("{:?}", error)),
		};

		let call = avail::tx().staking().chill_other(stash);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch_default(&call, account)
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

		Ok(ChillOtherTxSuccess {
			event,
			tx_hash: tx_in_block.extrinsic_hash(),
			block_hash: tx_in_block.block_hash(),
		})
	}

	pub async fn nominate(
		&self,
		targets: &[String],
		wait_for: WaitFor,
		account: &Keypair,
	) -> Result<NominateTxSuccess, String> {
		let targets: Result<Vec<AccountId>, _> = targets
			.iter()
			.map(|address| AccountId::from_str(address))
			.collect();
		let targets = targets.map_err(|e| std::format!("{:?}", e))?;
		let targets = targets.into_iter().map(|a| MultiAddress::Id(a)).collect();

		let call = avail::tx().staking().nominate(targets);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch_default(&call, account)
			.await;

		let transaction = progress_transaction(maybe_tx_progress, wait_for).await;
		let tx_in_block = match transaction {
			Ok(tx_in_block) => tx_in_block,
			Err(message) => return Err(message),
		};

		let _events = match tx_in_block.wait_for_success().await {
			Ok(e) => e,
			Err(error) => return Err(error.to_string()),
		};

		let (tx_hash, block_hash) = (tx_in_block.extrinsic_hash(), tx_in_block.block_hash());

		let tx_data =
			transaction_data::staking::Nominate::new(block_hash, tx_hash, &self.blocks).await?;

		Ok(NominateTxSuccess {
			tx_data,
			tx_hash,
			block_hash,
		})
	}

	pub async fn unbond(
		&self,
		value: u128,
		wait_for: WaitFor,
		account: &Keypair,
	) -> Result<UnbondTxSuccess, String> {
		let call = avail::tx().staking().unbond(value);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch_default(&call, account)
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

		Ok(UnbondTxSuccess {
			event,
			tx_hash: tx_in_block.extrinsic_hash(),
			block_hash: tx_in_block.block_hash(),
		})
	}

	pub async fn validate(
		&self,
		commission: u8,
		blocked: bool,
		wait_for: WaitFor,
		account: &Keypair,
	) -> Result<ValidateTxSuccess, String> {
		if commission > 100 {
			return Err(String::from("Commission cannot be more than 100"));
		}

		let commission = Perbill(commission as u32);
		let perfs = ValidatorPrefs {
			commission,
			blocked,
		};

		let call = avail::tx().staking().validate(perfs);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch_default(&call, account)
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

		Ok(ValidateTxSuccess {
			event,
			tx_hash: tx_in_block.extrinsic_hash(),
			block_hash: tx_in_block.block_hash(),
		})
	}
}

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
	) -> Result<SubmitDataTxSuccess, String> {
		let call = avail::tx().data_availability().submit_data(data);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch_default(&call, account)
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

		Ok(SubmitDataTxSuccess {
			event,
			tx_data,
			tx_hash,
			block_hash,
		})
	}

	pub async fn create_application_key(
		&self,
		key: Key,
		wait_for: WaitFor,
		account: &Keypair,
	) -> Result<CreateApplicationKeyTxSuccess, String> {
		let call = avail::tx().data_availability().create_application_key(key);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch_default(&call, account)
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

		Ok(CreateApplicationKeyTxSuccess {
			event,
			tx_hash: tx_in_block.extrinsic_hash(),
			block_hash: tx_in_block.block_hash(),
		})
	}

	pub async fn set_application_key(
		&self,
		old_key: Key,
		new_key: Key,
		wait_for: WaitFor,
		account: &Keypair,
	) -> Result<SetApplicationKeyTxSuccess, String> {
		let call = Call::DataAvailability(
			avail::runtime_types::da_control::pallet::Call::set_application_key {
				old_key,
				new_key,
			},
		);
		let sudo = avail::tx().sudo().sudo(call);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch_default(&sudo, account)
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

		Ok(SetApplicationKeyTxSuccess {
			event,
			tx_hash: tx_in_block.extrinsic_hash(),
			block_hash: tx_in_block.block_hash(),
		})
	}

	pub async fn submit_block_length_proposal(
		&self,
		rows: u32,
		cols: u32,
		wait_for: WaitFor,
		account: &Keypair,
	) -> Result<SubmitBlockLengthProposalTxSuccess, String> {
		let call = Call::DataAvailability(
			avail::runtime_types::da_control::pallet::Call::submit_block_length_proposal {
				rows,
				cols,
			},
		);
		let sudo = avail::tx().sudo().sudo(call);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch_default(&sudo, account)
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

		Ok(SubmitBlockLengthProposalTxSuccess {
			event,
			tx_hash: tx_in_block.extrinsic_hash(),
			block_hash: tx_in_block.block_hash(),
		})
	}

	pub async fn set_submit_data_fee_modifier(
		&self,
		modifier: DispatchFeeModifier,
		wait_for: WaitFor,
		account: &Keypair,
	) -> Result<SetSubmitDataFeeModifierTxSuccess, String> {
		let call = Call::DataAvailability(
			avail::runtime_types::da_control::pallet::Call::set_submit_data_fee_modifier {
				modifier,
			},
		);
		let sudo = avail::tx().sudo().sudo(call);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch_default(&sudo, account)
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

		Ok(SetSubmitDataFeeModifierTxSuccess {
			event,
			tx_hash: tx_in_block.extrinsic_hash(),
			block_hash: tx_in_block.block_hash(),
		})
	}
}

async fn progress_transaction(
	maybe_tx_progress: Result<TxProgress<AvailConfig, Api>, subxt::Error>,
	wait_for: WaitFor,
) -> Result<TransactionInBlock, String> {
	if let Err(error) = maybe_tx_progress {
		return Err(error.to_string());
	}
	let mut tx_progress = maybe_tx_progress.unwrap();

	while let Some(tx_status) = tx_progress.next().await {
		if let Err(error) = tx_status {
			return Err(error.to_string());
		}
		let tx_status = tx_status.unwrap();

		match tx_status {
			TxStatus::InBestBlock(tx_in_block) => {
				if wait_for == WaitFor::BlockInclusion {
					return Ok(tx_in_block);
				}
			},
			TxStatus::InFinalizedBlock(tx_in_block) => {
				if wait_for == WaitFor::BlockFinalization {
					return Ok(tx_in_block);
				}
			},
			TxStatus::Error { message } => return Err(message),
			TxStatus::Invalid { message } => return Err(message),
			TxStatus::Dropped { message } => return Err(message),
			_ => {},
		};
	}

	Err(String::from("Something went wrong."))
}

#[cfg(test)]
mod tests {
	use super::*;
	use subxt_signer::sr25519::dev;

	#[tokio::test]
	async fn testing_function() {
		let sdk = crate::sdk::SDK::new("ws://127.0.0.1:9944").await.unwrap();
		let account = dev::alice();
		let res = sdk
			.tx
			.data_availability
			.submit_block_length_proposal(0, 0, WaitFor::BlockInclusion, &account)
			.await;
		if let Err(e) = res {
			dbg!(e);
		}
	}
}
