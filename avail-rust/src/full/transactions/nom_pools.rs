use super::super::rpcs::Rpc;
use crate::avail::nomination_pools::calls::types::set_claim_permission::Permission;
use crate::avail::nomination_pools::calls::types::set_commission::NewCommission as NewCommissionOriginal;
use crate::avail::nomination_pools::calls::types::set_state::State;
use crate::avail::runtime_types::pallet_nomination_pools::BondExtra;
use crate::avail::runtime_types::sp_arithmetic::per_things::Perbill;
use crate::utils_raw::fetch_transaction;
use crate::WaitFor;
use crate::{avail, AccountId, AvailBlocksClient, AvailConfig, BlockHash, TxApi};

use std::str::FromStr;
use subxt::blocks::ExtrinsicEvents;
use subxt_signer::sr25519::Keypair;

use avail::nomination_pools::calls::types as NominationPoolsCalls;
use avail::nomination_pools::events as NominationPoolsEvents;

use super::options::{from_options_to_params, Options};
use super::progress_transaction_ex;

#[derive(Debug)]
pub struct PoolCreateTxSuccess {
	pub event: NominationPoolsEvents::Created,
	pub event2: NominationPoolsEvents::Bonded,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct PoolCreateWithPoolIdTxSuccess {
	pub event: NominationPoolsEvents::Created,
	pub event2: NominationPoolsEvents::Bonded,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct PoolJoinTxSuccess {
	pub event: NominationPoolsEvents::Bonded,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct PoolNominateTxSuccess {
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_data: NominationPoolsCalls::Nominate,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct PoolBondExtraTxSuccess {
	pub event: NominationPoolsEvents::Bonded,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct PoolSetCommissionTxSuccess {
	pub event: NominationPoolsEvents::PoolCommissionUpdated,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct PoolSetStateTxSuccess {
	pub event: Option<NominationPoolsEvents::StateChanged>,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct PoolClaimPayoutTxSuccess {
	pub event: Option<NominationPoolsEvents::PaidOut>,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct PoolChillTxSuccess {
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct PoolSetClaimPermissionTxSuccess {
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct PoolClaimCommissionTxSuccess {
	pub event: NominationPoolsEvents::PoolCommissionClaimed,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct PoolClaimPayoutOtherTxSuccess {
	pub event: Option<NominationPoolsEvents::PaidOut>,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct PoolUnbondTxSuccess {
	pub event: Option<NominationPoolsEvents::Unbonded>,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct PoolSetMetadataTxSuccess {
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct PoolWithdrawUnbondedTxSuccess {
	pub event: Option<NominationPoolsEvents::Withdrawn>,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug, Clone)]
pub struct NewCommission {
	pub amount: Perbill,
	pub payee: String,
}

#[derive(Clone)]
pub struct NominationPools {
	api: TxApi,
	rpc_client: Rpc,
	blocks: AvailBlocksClient,
}

impl NominationPools {
	pub fn new(api: TxApi, rpc_client: Rpc, blocks: AvailBlocksClient) -> Self {
		Self {
			api,
			rpc_client,
			blocks,
		}
	}

	pub async fn nominate(
		&self,
		pool_id: u32,
		validators: Vec<String>,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<PoolNominateTxSuccess, String> {
		let validators: Result<Vec<AccountId>, _> = validators
			.into_iter()
			.map(|v| AccountId::from_str(&v))
			.collect();
		let validators = validators.map_err(|e| e.to_string())?;

		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().nomination_pools().nominate(pool_id, validators);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let tx_data = tx_data_pool_nominate(block_hash, tx_hash, &self.blocks).await?;

		Ok(PoolNominateTxSuccess {
			events,
			tx_data,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn join(
		&self,
		amount: u128,
		pool_id: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<PoolJoinTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().nomination_pools().join(amount, pool_id);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<NominationPoolsEvents::Bonded>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

		Ok(PoolJoinTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
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
	) -> Result<PoolCreateWithPoolIdTxSuccess, String> {
		let root = AccountId::from_str(root).map_err(|e| e.to_string())?;
		let nominator = AccountId::from_str(nominator).map_err(|e| e.to_string())?;
		let bouncer = AccountId::from_str(bouncer).map_err(|e| e.to_string())?;

		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().nomination_pools().create_with_pool_id(
			amount,
			root.into(),
			nominator.into(),
			bouncer.into(),
			pool_id,
		);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<NominationPoolsEvents::Created>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Created event"));
		};

		let event2 = events.find_first::<NominationPoolsEvents::Bonded>();
		let Some(event2) = event2.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

		Ok(PoolCreateWithPoolIdTxSuccess {
			event,
			event2,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
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
	) -> Result<PoolCreateTxSuccess, String> {
		let root = AccountId::from_str(root).map_err(|e| e.to_string())?;
		let nominator = AccountId::from_str(nominator).map_err(|e| e.to_string())?;
		let bouncer = AccountId::from_str(bouncer).map_err(|e| e.to_string())?;

		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().nomination_pools().create(
			amount,
			root.into(),
			nominator.into(),
			bouncer.into(),
		);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<NominationPoolsEvents::Created>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Created event"));
		};

		let event2 = events.find_first::<NominationPoolsEvents::Bonded>();
		let Some(event2) = event2.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

		Ok(PoolCreateTxSuccess {
			event,
			event2,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn bond_extra(
		&self,
		extra: BondExtra<u128>,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<PoolBondExtraTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().nomination_pools().bond_extra(extra);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<NominationPoolsEvents::Bonded>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Bonded event"));
		};

		Ok(PoolBondExtraTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn set_commission(
		&self,
		pool_id: u32,
		new_commission: Option<NewCommission>,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<PoolSetCommissionTxSuccess, String> {
		let new_commission: NewCommissionOriginal = match new_commission {
			Some(x) => {
				let account = AccountId::from_str(&x.payee).map_err(|e| e.to_string())?;
				Some((x.amount, account))
			},
			None => None,
		};

		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx()
			.nomination_pools()
			.set_commission(pool_id, new_commission);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<NominationPoolsEvents::PoolCommissionUpdated>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find PoolCommissionUpdated event"));
		};

		Ok(PoolSetCommissionTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn set_state(
		&self,
		pool_id: u32,
		state: State,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<PoolSetStateTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().nomination_pools().set_state(pool_id, state);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<NominationPoolsEvents::StateChanged>();
		let event = event.ok().flatten();

		Ok(PoolSetStateTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn claim_payout(
		&self,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<PoolClaimPayoutTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().nomination_pools().claim_payout();

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<NominationPoolsEvents::PaidOut>();
		let event = event.ok().flatten();

		Ok(PoolClaimPayoutTxSuccess {
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
		pool_id: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<PoolChillTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().nomination_pools().chill(pool_id);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		Ok(PoolChillTxSuccess {
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn set_claim_permission(
		&self,
		permission: Permission,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<PoolSetClaimPermissionTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx()
			.nomination_pools()
			.set_claim_permission(permission);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		Ok(PoolSetClaimPermissionTxSuccess {
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn claim_commission(
		&self,
		pool_id: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<PoolClaimCommissionTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().nomination_pools().claim_commission(pool_id);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<NominationPoolsEvents::PoolCommissionClaimed>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find PoolCommissionClaimed event"));
		};

		Ok(PoolClaimCommissionTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn claim_payout_other(
		&self,
		other: &str,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<PoolClaimPayoutOtherTxSuccess, String> {
		let other = AccountId::from_str(other).map_err(|e| e.to_string())?;
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx()
			.nomination_pools()
			.claim_payout_other(other.into());

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<NominationPoolsEvents::PaidOut>();
		let event = event.ok().flatten();

		Ok(PoolClaimPayoutOtherTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn unbond(
		&self,
		member_account: &str,
		unbonding_points: u128,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<PoolUnbondTxSuccess, String> {
		let member_account = AccountId::from_str(member_account).map_err(|e| e.to_string())?;
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx()
			.nomination_pools()
			.unbond(member_account.into(), unbonding_points);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<NominationPoolsEvents::Unbonded>();
		let event = event.ok().flatten();

		Ok(PoolUnbondTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn set_metadata(
		&self,
		pool_id: u32,
		metadata: Vec<u8>,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<PoolSetMetadataTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx()
			.nomination_pools()
			.set_metadata(pool_id, metadata);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		Ok(PoolSetMetadataTxSuccess {
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn withdraw_unbonded(
		&self,
		member_account: &str,
		num_slashing_spans: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<PoolWithdrawUnbondedTxSuccess, String> {
		let member_account = AccountId::from_str(member_account).map_err(|e| e.to_string())?;
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx()
			.nomination_pools()
			.withdraw_unbonded(member_account.into(), num_slashing_spans);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<NominationPoolsEvents::Withdrawn>();
		let event = event.ok().flatten();

		Ok(PoolWithdrawUnbondedTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}
}

pub async fn tx_data_pool_nominate(
	block_hash: BlockHash,
	tx_hash: BlockHash,
	blocks: &AvailBlocksClient,
) -> Result<NominationPoolsCalls::Nominate, String> {
	let transaction =
		fetch_transaction::<NominationPoolsCalls::Nominate>(block_hash, tx_hash, blocks).await;
	let transaction = transaction.map_err(|err| err.to_string())?;
	Ok(transaction.value)
}
