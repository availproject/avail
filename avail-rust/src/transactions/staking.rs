use crate::api_dev::api::runtime_types::pallet_staking::ValidatorPrefs;
use crate::api_dev::api::runtime_types::sp_arithmetic::per_things::Perbill;
use crate::{avail, AOnlineClient, AccountId, RewardDestination};

use super::Transaction;
use subxt::backend::rpc::RpcClient;
use subxt_core::utils::MultiAddress;

pub type BondCall = avail::staking::calls::types::Bond;
pub type BondExtraCall = avail::staking::calls::types::BondExtra;
pub type ChillCall = avail::staking::calls::types::Chill;
pub type ChillOtherCall = avail::staking::calls::types::ChillOther;
pub type NominateCall = avail::staking::calls::types::Nominate;
pub type UnbondCall = avail::staking::calls::types::Unbond;
pub type ValidateCall = avail::staking::calls::types::Validate;
pub type PayoutStakersCall = avail::staking::calls::types::PayoutStakers;

#[derive(Clone)]
pub struct Staking {
	online_client: AOnlineClient,
	rpc_client: RpcClient,
}

pub struct Commission(u8);
impl Commission {
	pub fn new(value: u8) -> Result<Self, String> {
		if value > 100 {
			return Err(String::from("Commission cannot be more than 100"));
		}

		Ok(Self(value))
	}
}

impl Staking {
	pub fn new(online_client: AOnlineClient, rpc_client: RpcClient) -> Self {
		Self {
			online_client,
			rpc_client,
		}
	}

	pub fn bond(&self, value: u128, payee: RewardDestination) -> Transaction<BondCall> {
		let payload = avail::tx().staking().bond(value, payee);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn bond_extra(&self, max_additional: u128) -> Transaction<BondExtraCall> {
		let payload = avail::tx().staking().bond_extra(max_additional);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn chill(&self) -> Transaction<ChillCall> {
		let payload = avail::tx().staking().chill();
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn chill_other(&self, stash: AccountId) -> Transaction<ChillOtherCall> {
		let payload = avail::tx().staking().chill_other(stash);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn nominate(&self, targets: &[AccountId]) -> Transaction<NominateCall> {
		let targets = targets
			.into_iter()
			.map(|a| MultiAddress::Id(a.clone()))
			.collect();

		let payload = avail::tx().staking().nominate(targets);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn unbond(&self, value: u128) -> Transaction<UnbondCall> {
		let payload = avail::tx().staking().unbond(value);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn validate(&self, commission: Commission, blocked: bool) -> Transaction<ValidateCall> {
		let commission = Perbill(commission.0 as u32);
		let perfs = ValidatorPrefs {
			commission,
			blocked,
		};

		let payload = avail::tx().staking().validate(perfs);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn payout_stakers(
		&self,
		validator_stash: AccountId,
		era: u32,
	) -> Transaction<PayoutStakersCall> {
		let payload = avail::tx().staking().payout_stakers(validator_stash, era);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}
}
