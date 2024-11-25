use crate::api_dev::api::runtime_types::pallet_staking::ValidatorPrefs;
use crate::api_dev::api::runtime_types::sp_arithmetic::per_things::Perbill;
use crate::avail::staking::events::era_paid::EraIndex;
use crate::sdk::WaitFor;
use crate::{avail, AOnlineClient, AccountId, RewardDestination};

use std::str::FromStr;
use subxt::backend::rpc::RpcClient;
use subxt_core::utils::MultiAddress;
use subxt_signer::sr25519::Keypair;

use avail::staking::calls::types as StakingCalls;
use avail::staking::events as StakingEvents;

use super::{
	find_data_or_return_error, find_event_or_nothing, find_event_or_return_error, TransactionFailed,
};
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
	pub event: Option<StakingEvents::Chilled>,
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

#[derive(Debug)]
pub struct PayoutStakersTx {
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
	) -> Result<BondTx, TransactionFailed> {
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

		let event = find_event_or_return_error::<StakingEvents::Bonded>(
			"Failed to find Staking::Bonded event",
			&details,
		)?;
		Ok(BondTx { event, details })
	}

	pub async fn bond_extra(
		&self,
		max_additional: u128,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<BondExtraTx, TransactionFailed> {
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

		let event = find_event_or_return_error::<StakingEvents::Bonded>(
			"Failed to find Staking::Bonded event",
			&details,
		)?;

		Ok(BondExtraTx { event, details })
	}

	pub async fn chill(
		&self,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ChillTx, TransactionFailed> {
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

		let event = find_event_or_nothing::<StakingEvents::Chilled>(&details);

		Ok(ChillTx { event, details })
	}

	pub async fn chill_other(
		&self,
		stash: AccountId,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ChillOtherTx, TransactionFailed> {
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

		let event = find_event_or_nothing::<StakingEvents::Chilled>(&details);

		Ok(ChillOtherTx { event, details })
	}

	pub async fn nominate(
		&self,
		targets: &[AccountId],
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<NominateTx, TransactionFailed> {
		let targets = targets
			.into_iter()
			.map(|a| MultiAddress::Id(a.clone()))
			.collect();

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

		let data = find_data_or_return_error::<StakingCalls::Nominate>(
			&self.online_client,
			"Failed to find Nominate data",
			&details,
		)
		.await?;

		Ok(NominateTx { data, details })
	}

	pub async fn unbond(
		&self,
		value: u128,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<UnbondTx, TransactionFailed> {
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

		let event = find_event_or_return_error::<StakingEvents::Unbonded>(
			"Failed to find Staking::Unbonded event",
			&details,
		)?;

		Ok(UnbondTx { event, details })
	}

	pub async fn validate(
		&self,
		commission: u8,
		blocked: bool,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ValidateTx, TransactionFailed> {
		if commission > 100 {
			return Err(TransactionFailed::from(
				"Commission cannot be more than 100",
			));
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

		let event = find_event_or_return_error::<StakingEvents::ValidatorPrefsSet>(
			"Failed to find Staking::ValidatorPrefsSet event",
			&details,
		)?;

		Ok(ValidateTx { event, details })
	}

	pub async fn payout_stakers(
		&self,
		validator_stash: AccountId,
		era: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<PayoutStakersTx, TransactionFailed> {
		let call = avail::tx().staking().payout_stakers(validator_stash, era);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		Ok(PayoutStakersTx { details })
	}
}
