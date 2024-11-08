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
use crate::utils_raw::fetch_transaction;
use crate::{ABlocksClient, ATxClient, AccountId, H256};

use std::str::FromStr;
use subxt::backend::rpc::RpcClient;
use subxt_signer::sr25519::Keypair;

use super::{options::Options, sign_and_submit_and_progress_transaction, TxResultDetails};

#[derive(Debug)]
pub struct CreateTx {
	pub event: NominationPoolsEvents::Created,
	pub event2: NominationPoolsEvents::Bonded,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct CreateWithPoolIdTx {
	pub event: NominationPoolsEvents::Created,
	pub event2: NominationPoolsEvents::Bonded,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct JoinTx {
	pub event: NominationPoolsEvents::Bonded,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct NominateTx {
	pub data: NominationPoolsCalls::Nominate,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct BondExtraTx {
	pub event: NominationPoolsEvents::Bonded,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct SetCommissionTx {
	pub event: NominationPoolsEvents::PoolCommissionUpdated,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct SetStateTx {
	pub event: Option<NominationPoolsEvents::StateChanged>,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct ClaimPayoutTx {
	pub event: Option<NominationPoolsEvents::PaidOut>,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct ChillTx {
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct SetClaimPermissionTx {
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct ClaimCommissionTx {
	pub event: NominationPoolsEvents::PoolCommissionClaimed,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct ClaimPayoutOtherTx {
	pub event: Option<NominationPoolsEvents::PaidOut>,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct UnbondTx {
	pub event: Option<NominationPoolsEvents::Unbonded>,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct SetMetadataTx {
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct WithdrawUnbondedTx {
	pub event: Option<NominationPoolsEvents::Withdrawn>,
	pub details: TxResultDetails,
}

#[derive(Debug, Clone)]
pub struct NewCommission {
	pub amount: Perbill,
	pub payee: String,
}

#[derive(Clone)]
pub struct NominationPools {
	tx_client: ATxClient,
	blocks_client: ABlocksClient,
	rpc_client: RpcClient,
}

impl NominationPools {
	pub fn new(tx_client: ATxClient, rpc_client: RpcClient, blocks_client: ABlocksClient) -> Self {
		Self {
			tx_client,
			blocks_client,
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
	) -> Result<NominateTx, String> {
		let validators: Result<Vec<AccountId>, _> = validators
			.into_iter()
			.map(|v| AccountId::from_str(&v))
			.collect();
		let validators = validators.map_err(|e| e.to_string())?;

		let call = avail::tx().nomination_pools().nominate(pool_id, validators);
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
			tx_data_pool_nominate(&self.blocks_client, details.block_hash, details.tx_hash).await?;

		Ok(NominateTx { data, details })
	}

	pub async fn join(
		&self,
		amount: u128,
		pool_id: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<JoinTx, String> {
		let call = avail::tx().nomination_pools().join(amount, pool_id);
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

		let event = details.events.find_first::<NominationPoolsEvents::Bonded>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

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
	) -> Result<CreateWithPoolIdTx, String> {
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
			.find_first::<NominationPoolsEvents::Created>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Created event"));
		};

		let event2 = details.events.find_first::<NominationPoolsEvents::Bonded>();
		let Some(event2) = event2.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

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
	) -> Result<CreateTx, String> {
		let root = AccountId::from_str(root).map_err(|e| e.to_string())?;
		let nominator = AccountId::from_str(nominator).map_err(|e| e.to_string())?;
		let bouncer = AccountId::from_str(bouncer).map_err(|e| e.to_string())?;

		let call = avail::tx().nomination_pools().create(
			amount,
			root.into(),
			nominator.into(),
			bouncer.into(),
		);
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
			.find_first::<NominationPoolsEvents::Created>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Created event"));
		};

		let event2 = details.events.find_first::<NominationPoolsEvents::Bonded>();
		let Some(event2) = event2.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

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
	) -> Result<BondExtraTx, String> {
		let call = avail::tx().nomination_pools().bond_extra(extra);
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

		let event = details.events.find_first::<NominationPoolsEvents::Bonded>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

		Ok(BondExtraTx { event, details })
	}

	pub async fn set_commission(
		&self,
		pool_id: u32,
		new_commission: Option<NewCommission>,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SetCommissionTx, String> {
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
			.find_first::<NominationPoolsEvents::PoolCommissionUpdated>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find PoolCommissionUpdated event"));
		};

		Ok(SetCommissionTx { event, details })
	}

	pub async fn set_state(
		&self,
		pool_id: u32,
		state: State,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SetStateTx, String> {
		let call = avail::tx().nomination_pools().set_state(pool_id, state);
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
			.find_first::<NominationPoolsEvents::StateChanged>();
		let event = event.ok().flatten();

		Ok(SetStateTx { event, details })
	}

	pub async fn claim_payout(
		&self,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ClaimPayoutTx, String> {
		let call = avail::tx().nomination_pools().claim_payout();
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
			.find_first::<NominationPoolsEvents::PaidOut>();
		let event = event.ok().flatten();

		Ok(ClaimPayoutTx { event, details })
	}

	pub async fn chill(
		&self,
		pool_id: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ChillTx, String> {
		let call = avail::tx().nomination_pools().chill(pool_id);
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

		Ok(ChillTx { details })
	}

	pub async fn set_claim_permission(
		&self,
		permission: Permission,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SetClaimPermissionTx, String> {
		let call = avail::tx()
			.nomination_pools()
			.set_claim_permission(permission);
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

		Ok(SetClaimPermissionTx { details })
	}

	pub async fn claim_commission(
		&self,
		pool_id: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ClaimCommissionTx, String> {
		let call = avail::tx().nomination_pools().claim_commission(pool_id);
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
			.find_first::<NominationPoolsEvents::PoolCommissionClaimed>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find PoolCommissionClaimed event"));
		};

		Ok(ClaimCommissionTx { event, details })
	}

	pub async fn claim_payout_other(
		&self,
		other: &str,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<ClaimPayoutOtherTx, String> {
		let other = AccountId::from_str(other).map_err(|e| e.to_string())?;
		let call = avail::tx()
			.nomination_pools()
			.claim_payout_other(other.into());
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
			.find_first::<NominationPoolsEvents::PaidOut>();
		let event = event.ok().flatten();

		Ok(ClaimPayoutOtherTx { event, details })
	}

	pub async fn unbond(
		&self,
		member_account: &str,
		unbonding_points: u128,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<UnbondTx, String> {
		let member_account = AccountId::from_str(member_account).map_err(|e| e.to_string())?;
		let call = avail::tx()
			.nomination_pools()
			.unbond(member_account.into(), unbonding_points);
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
			.find_first::<NominationPoolsEvents::Unbonded>();
		let event = event.ok().flatten();

		Ok(UnbondTx { event, details })
	}

	pub async fn set_metadata(
		&self,
		pool_id: u32,
		metadata: Vec<u8>,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SetMetadataTx, String> {
		let call = avail::tx()
			.nomination_pools()
			.set_metadata(pool_id, metadata);
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

		Ok(SetMetadataTx { details })
	}

	pub async fn withdraw_unbonded(
		&self,
		member_account: &str,
		num_slashing_spans: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<WithdrawUnbondedTx, String> {
		let member_account = AccountId::from_str(member_account).map_err(|e| e.to_string())?;
		let call = avail::tx()
			.nomination_pools()
			.withdraw_unbonded(member_account.into(), num_slashing_spans);
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
			.find_first::<NominationPoolsEvents::Withdrawn>();
		let event = event.ok().flatten();

		Ok(WithdrawUnbondedTx { event, details })
	}
}

pub async fn tx_data_pool_nominate(
	client: &ABlocksClient,
	block_hash: H256,
	tx_hash: H256,
) -> Result<NominationPoolsCalls::Nominate, String> {
	let transaction =
		fetch_transaction::<NominationPoolsCalls::Nominate>(client, block_hash, tx_hash).await;
	let transaction = transaction.map_err(|err| err.to_string())?;
	Ok(transaction.value)
}
