use crate::avail::{
	self, nomination_pools::calls::types::set_commission::NewCommission as NewCommissionOriginal,
	runtime_types::sp_arithmetic::per_things::Perbill,
};
use crate::{AOnlineClient, AccountId};

use super::Transaction;
use subxt::backend::rpc::RpcClient;

pub use crate::avail::nomination_pools::calls::types::set_claim_permission::Permission;
pub use crate::avail::nomination_pools::calls::types::set_state::State;
pub use crate::avail::runtime_types::pallet_nomination_pools::BondExtra;

pub type NominateCall = avail::nomination_pools::calls::types::Nominate;
pub type JoinCall = avail::nomination_pools::calls::types::Join;
pub type CreateCall = avail::nomination_pools::calls::types::Create;
pub type CreateWithPoolIdCall = avail::nomination_pools::calls::types::CreateWithPoolId;
pub type BondExtraCall = avail::nomination_pools::calls::types::BondExtra;
pub type BondExtraOtherCall = avail::nomination_pools::calls::types::BondExtraOther;
pub type SetCommissionCall = avail::nomination_pools::calls::types::SetCommission;
pub type SetClaimPermissionCall = avail::nomination_pools::calls::types::SetClaimPermission;
pub type SetStateCall = avail::nomination_pools::calls::types::SetState;
pub type ClaimPayoutCall = avail::nomination_pools::calls::types::ClaimPayout;
pub type ClaimPayoutOtherCall = avail::nomination_pools::calls::types::ClaimPayoutOther;
pub type ChillCall = avail::nomination_pools::calls::types::Chill;
pub type ClaimCommissionCall = avail::nomination_pools::calls::types::ClaimCommission;
pub type UnbondCall = avail::nomination_pools::calls::types::Unbond;
pub type SetMetadataCall = avail::nomination_pools::calls::types::SetMetadata;
pub type WithdrawUnbondedCall = avail::nomination_pools::calls::types::WithdrawUnbonded;

#[derive(Debug, Clone)]
pub struct NewCommission {
	pub amount: Perbill,
	pub payee: AccountId,
}

#[derive(Clone)]
pub struct NominationPools {
	online_client: AOnlineClient,
	rpc_client: RpcClient,
}

impl NominationPools {
	pub fn new(online_client: AOnlineClient, rpc_client: RpcClient) -> Self {
		Self {
			online_client,
			rpc_client,
		}
	}

	pub fn nominate(&self, pool_id: u32, validators: Vec<AccountId>) -> Transaction<NominateCall> {
		let payload = avail::tx().nomination_pools().nominate(pool_id, validators);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn join(&self, amount: u128, pool_id: u32) -> Transaction<JoinCall> {
		let payload = avail::tx().nomination_pools().join(amount, pool_id);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn create_with_pool_id(
		&self,
		amount: u128,
		root: AccountId,
		nominator: AccountId,
		bouncer: AccountId,
		pool_id: u32,
	) -> Transaction<CreateWithPoolIdCall> {
		let payload = avail::tx().nomination_pools().create_with_pool_id(
			amount,
			root.into(),
			nominator.into(),
			bouncer.into(),
			pool_id,
		);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn create(
		&self,
		amount: u128,
		root: AccountId,
		nominator: AccountId,
		bouncer: AccountId,
	) -> Transaction<CreateCall> {
		let payload = avail::tx().nomination_pools().create(
			amount,
			root.into(),
			nominator.into(),
			bouncer.into(),
		);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn bond_extra(&self, extra: BondExtra<u128>) -> Transaction<BondExtraCall> {
		let payload = avail::tx().nomination_pools().bond_extra(extra);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn set_commission(
		&self,
		pool_id: u32,
		new_commission: Option<NewCommission>,
	) -> Transaction<SetCommissionCall> {
		let new_commission: NewCommissionOriginal = match new_commission {
			Some(x) => Some((x.amount, x.payee)),
			None => None,
		};

		let payload = avail::tx()
			.nomination_pools()
			.set_commission(pool_id, new_commission);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn set_state(&self, pool_id: u32, state: State) -> Transaction<SetStateCall> {
		let payload = avail::tx().nomination_pools().set_state(pool_id, state);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn claim_payout(&self) -> Transaction<ClaimPayoutCall> {
		let payload = avail::tx().nomination_pools().claim_payout();
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn chill(&self, pool_id: u32) -> Transaction<ChillCall> {
		let payload = avail::tx().nomination_pools().chill(pool_id);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn set_claim_permission(
		&self,
		permission: Permission,
	) -> Transaction<SetClaimPermissionCall> {
		let payload = avail::tx()
			.nomination_pools()
			.set_claim_permission(permission);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn claim_commission(&self, pool_id: u32) -> Transaction<ClaimCommissionCall> {
		let payload = avail::tx().nomination_pools().claim_commission(pool_id);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn claim_payout_other(&self, other: AccountId) -> Transaction<ClaimPayoutOtherCall> {
		let payload = avail::tx()
			.nomination_pools()
			.claim_payout_other(other.into());
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn unbond(
		&self,
		member_account: AccountId,
		unbonding_points: u128,
	) -> Transaction<UnbondCall> {
		let payload = avail::tx()
			.nomination_pools()
			.unbond(member_account.into(), unbonding_points);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn set_metadata(&self, pool_id: u32, metadata: Vec<u8>) -> Transaction<SetMetadataCall> {
		let payload = avail::tx()
			.nomination_pools()
			.set_metadata(pool_id, metadata);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn withdraw_unbonded(
		&self,
		member_account: AccountId,
		num_slashing_spans: u32,
	) -> Transaction<WithdrawUnbondedCall> {
		let payload = avail::tx()
			.nomination_pools()
			.withdraw_unbonded(member_account.into(), num_slashing_spans);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}
}
