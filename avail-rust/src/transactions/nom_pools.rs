use crate::avail::{
	self, nomination_pools::calls::types as NominationPoolsCalls,
	nomination_pools::calls::types::set_claim_permission::Permission,
	nomination_pools::calls::types::set_commission::NewCommission as NewCommissionOriginal,
	nomination_pools::calls::types::set_state::State,
	nomination_pools::events as NominationPoolsEvents,
	runtime_types::pallet_nomination_pools::BondExtra,
	runtime_types::sp_arithmetic::per_things::Perbill,
};
use crate::sdk::WaitFor;
use crate::{AOnlineClient, AccountId};

use std::str::FromStr;
use subxt::backend::rpc::RpcClient;
use subxt_signer::sr25519::Keypair;

use super::{
	find_data_or_return_error, find_event_or_nothing, find_event_or_return_error, TransactionFailed,
};
use super::{options::Options, progress_and_parse_transaction, TransactionDetails};

#[derive(Debug)]
pub struct CreateTx {
	pub event: NominationPoolsEvents::Created,
	pub event2: NominationPoolsEvents::Bonded,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct CreateWithPoolIdTx {
	pub event: NominationPoolsEvents::Created,
	pub event2: NominationPoolsEvents::Bonded,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct JoinTx {
	pub event: NominationPoolsEvents::Bonded,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct NominateTx {
	pub data: NominationPoolsCalls::Nominate,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct BondExtraTx {
	pub event: NominationPoolsEvents::Bonded,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct SetCommissionTx {
	pub event: NominationPoolsEvents::PoolCommissionUpdated,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct SetStateTx {
	pub event: Option<NominationPoolsEvents::StateChanged>,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct ClaimPayoutTx {
	pub event: Option<NominationPoolsEvents::PaidOut>,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct ChillTx {
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct SetClaimPermissionTx {
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct ClaimCommissionTx {
	pub event: NominationPoolsEvents::PoolCommissionClaimed,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct ClaimPayoutOtherTx {
	pub event: Option<NominationPoolsEvents::PaidOut>,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct UnbondTx {
	pub event: Option<NominationPoolsEvents::Unbonded>,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct SetMetadataTx {
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct WithdrawUnbondedTx {
	pub event: Option<NominationPoolsEvents::Withdrawn>,
	pub details: TransactionDetails,
}

#[derive(Debug, Clone)]
pub struct NewCommission {
	pub amount: Perbill,
	pub payee: String,
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

	pub async fn nominate(
		&self,
		pool_id: u32,
		validators: Vec<String>,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<NominateTx, TransactionFailed> {
		let validators: Result<Vec<AccountId>, _> = validators
			.into_iter()
			.map(|v| AccountId::from_str(&v))
			.collect();
		let validators = validators.map_err(|e| e.to_string())?;

		let call = avail::tx().nomination_pools().nominate(pool_id, validators);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let data = find_data_or_return_error::<NominationPoolsCalls::Nominate>(
			&self.online_client,
			"Failed to find NominationPools::Nominate data",
			&details,
		)
		.await?;

		Ok(NominateTx { data, details })
	}

	pub async fn join(
		&self,
		amount: u128,
		pool_id: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<JoinTx, TransactionFailed> {
		let call = avail::tx().nomination_pools().join(amount, pool_id);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = find_event_or_return_error::<NominationPoolsEvents::Bonded>(
			"Failed to find NominationPools::Bonded event",
			&details,
		)?;

		Ok(JoinTx { event, details })
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
	) -> Result<CreateWithPoolIdTx, TransactionFailed> {
		let root = AccountId::from_str(root).map_err(|e| e.to_string())?;
		let nominator = AccountId::from_str(nominator).map_err(|e| e.to_string())?;
		let bouncer = AccountId::from_str(bouncer).map_err(|e| e.to_string())?;

		let call = avail::tx().nomination_pools().create_with_pool_id(
			amount,
			root.into(),
			nominator.into(),
			bouncer.into(),
			pool_id,
		);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = find_event_or_return_error::<NominationPoolsEvents::Created>(
			"Failed to find NominationPools::Created event",
			&details,
		)?;

		let event2 = find_event_or_return_error::<NominationPoolsEvents::Bonded>(
			"Failed to find NominationPools::Bonded event",
			&details,
		)?;

		Ok(CreateWithPoolIdTx {
			event,
			event2,
			details,
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
	) -> Result<CreateTx, TransactionFailed> {
		let root = AccountId::from_str(root).map_err(|e| e.to_string())?;
		let nominator = AccountId::from_str(nominator).map_err(|e| e.to_string())?;
		let bouncer = AccountId::from_str(bouncer).map_err(|e| e.to_string())?;

		let call = avail::tx().nomination_pools().create(
			amount,
			root.into(),
			nominator.into(),
			bouncer.into(),
		);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = find_event_or_return_error::<NominationPoolsEvents::Created>(
			"Failed to find NominationPools::Created event",
			&details,
		)?;

		let event2 = find_event_or_return_error::<NominationPoolsEvents::Bonded>(
			"Failed to find NominationPools::Bonded event",
			&details,
		)?;

		Ok(CreateTx {
			event,
			event2,
			details,
		})
	}

	pub async fn bond_extra(
		&self,
		extra: BondExtra<u128>,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<BondExtraTx, TransactionFailed> {
		let call = avail::tx().nomination_pools().bond_extra(extra);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = find_event_or_return_error::<NominationPoolsEvents::Bonded>(
			"Failed to find NominationPools::Bonded event",
			&details,
		)?;

		Ok(BondExtraTx { event, details })
	}

	pub async fn set_commission(
		&self,
		pool_id: u32,
		new_commission: Option<NewCommission>,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SetCommissionTx, TransactionFailed> {
		let new_commission: NewCommissionOriginal = match new_commission {
			Some(x) => {
				let account = AccountId::from_str(&x.payee).map_err(|e| e.to_string())?;
				Some((x.amount, account))
			},
			None => None,
		};

		let call = avail::tx()
			.nomination_pools()
			.set_commission(pool_id, new_commission);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = find_event_or_return_error::<NominationPoolsEvents::PoolCommissionUpdated>(
			"Failed to find NominationPools::PoolCommissionUpdated event",
			&details,
		)?;

		Ok(SetCommissionTx { event, details })
	}

	pub async fn set_state(
		&self,
		pool_id: u32,
		state: State,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SetStateTx, TransactionFailed> {
		let call = avail::tx().nomination_pools().set_state(pool_id, state);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = find_event_or_nothing::<NominationPoolsEvents::StateChanged>(&details);

		Ok(SetStateTx { event, details })
	}

	pub async fn claim_payout(
		&self,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ClaimPayoutTx, TransactionFailed> {
		let call = avail::tx().nomination_pools().claim_payout();
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = find_event_or_nothing::<NominationPoolsEvents::PaidOut>(&details);

		Ok(ClaimPayoutTx { event, details })
	}

	pub async fn chill(
		&self,
		pool_id: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ChillTx, TransactionFailed> {
		let call = avail::tx().nomination_pools().chill(pool_id);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		Ok(ChillTx { details })
	}

	pub async fn set_claim_permission(
		&self,
		permission: Permission,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SetClaimPermissionTx, TransactionFailed> {
		let call = avail::tx()
			.nomination_pools()
			.set_claim_permission(permission);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		Ok(SetClaimPermissionTx { details })
	}

	pub async fn claim_commission(
		&self,
		pool_id: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ClaimCommissionTx, TransactionFailed> {
		let call = avail::tx().nomination_pools().claim_commission(pool_id);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = find_event_or_return_error::<NominationPoolsEvents::PoolCommissionClaimed>(
			"Failed to find NominationPools::PoolCommissionClaimed event",
			&details,
		)?;

		Ok(ClaimCommissionTx { event, details })
	}

	pub async fn claim_payout_other(
		&self,
		other: &str,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ClaimPayoutOtherTx, TransactionFailed> {
		let other = AccountId::from_str(other).map_err(|e| e.to_string())?;
		let call = avail::tx()
			.nomination_pools()
			.claim_payout_other(other.into());
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = find_event_or_nothing::<NominationPoolsEvents::PaidOut>(&details);

		Ok(ClaimPayoutOtherTx { event, details })
	}

	pub async fn unbond(
		&self,
		member_account: &str,
		unbonding_points: u128,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<UnbondTx, TransactionFailed> {
		let member_account = AccountId::from_str(member_account).map_err(|e| e.to_string())?;
		let call = avail::tx()
			.nomination_pools()
			.unbond(member_account.into(), unbonding_points);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = find_event_or_nothing::<NominationPoolsEvents::Unbonded>(&details);

		Ok(UnbondTx { event, details })
	}

	pub async fn set_metadata(
		&self,
		pool_id: u32,
		metadata: Vec<u8>,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SetMetadataTx, TransactionFailed> {
		let call = avail::tx()
			.nomination_pools()
			.set_metadata(pool_id, metadata);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		Ok(SetMetadataTx { details })
	}

	pub async fn withdraw_unbonded(
		&self,
		member_account: &str,
		num_slashing_spans: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<WithdrawUnbondedTx, TransactionFailed> {
		let member_account = AccountId::from_str(member_account).map_err(|e| e.to_string())?;
		let call = avail::tx()
			.nomination_pools()
			.withdraw_unbonded(member_account.into(), num_slashing_spans);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = find_event_or_nothing::<NominationPoolsEvents::Withdrawn>(&details);

		Ok(WithdrawUnbondedTx { event, details })
	}
}
