use crate::api_dev::api::runtime_types::pallet_staking::ValidatorPrefs;
use crate::api_dev::api::runtime_types::sp_arithmetic::per_things::Perbill;
use crate::sdk::WaitFor;
use crate::{avail, AOnlineClient, AccountId, RewardDestination};

use std::str::FromStr;
use subxt::backend::rpc::RpcClient;
use subxt_core::utils::MultiAddress;
use subxt_signer::sr25519::Keypair;

use avail::staking::calls::types as StakingCalls;
use avail::staking::events as StakingEvents;

use super::{options::Options, progress_and_parse_transaction, TransactionDetails};

#[derive(Debug)]
pub struct BondTx {
	pub event: StakingEvents::Bonded,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct BondExtraTx {
	pub event: StakingEvents::Bonded,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct ChillTx {
	pub event: Option<StakingEvents::Chilled>,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct ChillOtherTx {
	pub event: StakingEvents::Chilled,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct NominateTx {
	pub data: StakingCalls::Nominate,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct UnbondTx {
	pub event: StakingEvents::Unbonded,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct ValidateTx {
	pub event: StakingEvents::ValidatorPrefsSet,
	pub details: TransactionDetails,
}

#[derive(Clone)]
pub struct Staking {
	online_client: AOnlineClient,
	rpc_client: RpcClient,
}

impl Staking {
	pub fn new(online_client: AOnlineClient, rpc_client: RpcClient) -> Self {
		Self {
			online_client,
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
	) -> Result<BondTx, String> {
		let call = avail::tx().staking().bond(value, payee);
		let details = progress_and_parse_transaction(
			&self.online_client,
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

		Ok(BondTx { event, details })
	}

	pub async fn bond_extra(
		&self,
		max_additional: u128,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<BondExtraTx, String> {
		let call = avail::tx().staking().bond_extra(max_additional);
		let details = progress_and_parse_transaction(
			&self.online_client,
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

		Ok(BondExtraTx { event, details })
	}

	pub async fn chill(
		&self,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ChillTx, String> {
		let call = avail::tx().staking().chill();
		let details = progress_and_parse_transaction(
			&self.online_client,
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

		Ok(ChillTx { event, details })
	}

	pub async fn chill_other(
		&self,
		stash: &str,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ChillOtherTx, String> {
		let stash = match AccountId::from_str(stash) {
			Ok(stash) => stash,
			Err(error) => return Err(std::format!("{:?}", error)),
		};

		let call = avail::tx().staking().chill_other(stash);
		let details = progress_and_parse_transaction(
			&self.online_client,
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

		Ok(ChillOtherTx { event, details })
	}

	pub async fn nominate(
		&self,
		targets: &[String],
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<NominateTx, String> {
		let targets: Result<Vec<AccountId>, _> = targets
			.iter()
			.map(|address| AccountId::from_str(address))
			.collect();
		let targets = targets.map_err(|e| std::format!("{:?}", e))?;
		let targets = targets.into_iter().map(|a| MultiAddress::Id(a)).collect();

		let call = avail::tx().staking().nominate(targets);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let block = details.fetch_block(&self.online_client).await;
		let block = block.map_err(|e| e.to_string())?;
		let data = block.transaction_by_index_static::<StakingCalls::Nominate>(details.tx_index);
		let data = data
			.ok_or(String::from("Failed to find transaction data"))?
			.value;

		Ok(NominateTx { data, details })
	}

	pub async fn unbond(
		&self,
		value: u128,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<UnbondTx, String> {
		let call = avail::tx().staking().unbond(value);
		let details = progress_and_parse_transaction(
			&self.online_client,
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

		Ok(UnbondTx { event, details })
	}

	pub async fn validate(
		&self,
		commission: u8,
		blocked: bool,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ValidateTx, String> {
		if commission > 100 {
			return Err(String::from("Commission cannot be more than 100"));
		}

		let commission = Perbill(commission as u32);
		let perfs = ValidatorPrefs {
			commission,
			blocked,
		};

		let call = avail::tx().staking().validate(perfs);
		let details = progress_and_parse_transaction(
			&self.online_client,
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

		Ok(ValidateTx { event, details })
	}
}
