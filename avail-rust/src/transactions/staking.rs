use crate::api_dev::api::runtime_types::pallet_staking::ValidatorPrefs;
use crate::api_dev::api::runtime_types::sp_arithmetic::per_things::Perbill;
use crate::sdk::WaitFor;
use crate::utils_raw::fetch_transaction;
use crate::{avail, ABlocksClient, ATxClient, AccountId, RewardDestination, H256};

use std::str::FromStr;
use subxt::backend::rpc::RpcClient;
use subxt_core::utils::MultiAddress;
use subxt_signer::sr25519::Keypair;

use avail::staking::calls::types as StakingCalls;
use avail::staking::events as StakingEvents;

use super::{options::Options, sign_and_submit_and_progress_transaction, TxResultDetails};

#[derive(Debug)]
pub struct BondTxSuccess {
	pub event: StakingEvents::Bonded,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct BondExtraTxSuccess {
	pub event: StakingEvents::Bonded,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct ChillTxSuccess {
	pub event: Option<StakingEvents::Chilled>,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct ChillOtherTxSuccess {
	pub event: StakingEvents::Chilled,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct NominateTxSuccess {
	pub data: StakingCalls::Nominate,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct UnbondTxSuccess {
	pub event: StakingEvents::Unbonded,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct ValidateTxSuccess {
	pub event: StakingEvents::ValidatorPrefsSet,
	pub details: TxResultDetails,
}

#[derive(Clone)]
pub struct Staking {
	tx_client: ATxClient,
	blocks_client: ABlocksClient,
	rpc_client: RpcClient,
}

impl Staking {
	pub fn new(tx_client: ATxClient, rpc_client: RpcClient, blocks_client: ABlocksClient) -> Self {
		Self {
			tx_client,
			blocks_client,
			rpc_client,
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
		let call = avail::tx().staking().bond(value, payee);
		let details = sign_and_submit_and_progress_transaction(
			&self.tx_client,
			&self.blocks_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = details.events.find_first::<StakingEvents::Bonded>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

		Ok(BondTxSuccess { event, details })
	}

	pub async fn bond_extra(
		&self,
		max_additional: u128,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<BondExtraTxSuccess, String> {
		let call = avail::tx().staking().bond_extra(max_additional);
		let details = sign_and_submit_and_progress_transaction(
			&self.tx_client,
			&self.blocks_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = details.events.find_first::<StakingEvents::Bonded>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

		Ok(BondExtraTxSuccess { event, details })
	}

	pub async fn chill(
		&self,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ChillTxSuccess, String> {
		let call = avail::tx().staking().chill();
		let details = sign_and_submit_and_progress_transaction(
			&self.tx_client,
			&self.blocks_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = details
			.events
			.find_first::<StakingEvents::Chilled>()
			.ok()
			.flatten();

		Ok(ChillTxSuccess { event, details })
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

		let call = avail::tx().staking().chill_other(stash);
		let details = sign_and_submit_and_progress_transaction(
			&self.tx_client,
			&self.blocks_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;
		let event = details.events.find_first::<StakingEvents::Chilled>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Chilled event"));
		};

		Ok(ChillOtherTxSuccess { event, details })
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

		let call = avail::tx().staking().nominate(targets);
		let details = sign_and_submit_and_progress_transaction(
			&self.tx_client,
			&self.blocks_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let data =
			tx_data_staking_nominate(&self.blocks_client, details.block_hash, details.tx_hash)
				.await?;

		Ok(NominateTxSuccess { data, details })
	}

	pub async fn unbond(
		&self,
		value: u128,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<UnbondTxSuccess, String> {
		let call = avail::tx().staking().unbond(value);
		let details = sign_and_submit_and_progress_transaction(
			&self.tx_client,
			&self.blocks_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = details.events.find_first::<StakingEvents::Unbonded>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Unbonded event"));
		};

		Ok(UnbondTxSuccess { event, details })
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

		let call = avail::tx().staking().validate(perfs);
		let details = sign_and_submit_and_progress_transaction(
			&self.tx_client,
			&self.blocks_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = details
			.events
			.find_first::<StakingEvents::ValidatorPrefsSet>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find ValidatorPrefsSet event"));
		};

		Ok(ValidateTxSuccess { event, details })
	}
}

pub async fn tx_data_staking_nominate(
	client: &ABlocksClient,
	block_hash: H256,
	tx_hash: H256,
) -> Result<StakingCalls::Nominate, String> {
	let transaction =
		fetch_transaction::<StakingCalls::Nominate>(client, block_hash, tx_hash).await;
	let transaction = transaction.map_err(|err| err.to_string())?;
	Ok(transaction.value)
}
