use crate::avail;
use crate::sdk::WaitFor;
use crate::{utils, AOnlineClient};

use avail::balances::events as BalancesEvents;
use avail::system::events as SystemEvents;
use subxt::backend::rpc::RpcClient;
use subxt_signer::sr25519::Keypair;

use super::{
	find_event_or_nothing, find_event_or_return_error, progress_and_parse_transaction,
	TransactionFailed,
};
use super::{options::Options, TransactionDetails};

#[derive(Debug)]
pub struct TransferAllTx {
	pub event: BalancesEvents::Transfer,
	pub event2: Option<SystemEvents::KilledAccount>,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct TransferAllowDeathTx {
	pub event: BalancesEvents::Transfer,
	pub event2: Option<SystemEvents::KilledAccount>,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct TransferKeepAliveTx {
	pub event: BalancesEvents::Transfer,
	pub details: TransactionDetails,
}

#[derive(Clone)]
pub struct Balances {
	online_client: AOnlineClient,
	rpc_client: RpcClient,
}

impl Balances {
	pub fn new(online_client: AOnlineClient, rpc_client: RpcClient) -> Self {
		Self {
			online_client,
			rpc_client,
		}
	}

	pub async fn transfer_all(
		&self,
		dest: &str,
		keep_alive: bool,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<TransferAllTx, TransactionFailed> {
		let dest = match utils::account_id_from_str(dest) {
			Ok(dest) => dest,
			Err(error) => return Err(TransactionFailed::from(error)),
		};

		let call = avail::tx().balances().transfer_all(dest.into(), keep_alive);
		let details = utils::progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = find_event_or_return_error::<BalancesEvents::Transfer>(
			"Failed to find Transfer event",
			&details,
		)?;
		let event2 = find_event_or_nothing::<SystemEvents::KilledAccount>(&details);

		Ok(TransferAllTx {
			event,
			event2,
			details,
		})
	}

	pub async fn transfer_allow_death(
		&self,
		dest: &str,
		amount: u128,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<TransferAllowDeathTx, TransactionFailed> {
		let dest = match utils::account_id_from_str(dest) {
			Ok(dest) => dest,
			Err(error) => return Err(TransactionFailed::from(error)),
		};

		let call = avail::tx()
			.balances()
			.transfer_allow_death(dest.into(), amount);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = find_event_or_return_error::<BalancesEvents::Transfer>(
			"Failed to find Transfer event",
			&details,
		)?;
		let event2 = find_event_or_nothing::<SystemEvents::KilledAccount>(&details);

		Ok(TransferAllowDeathTx {
			event,
			event2,
			details,
		})
	}

	pub async fn transfer_keep_alive(
		&self,
		dest: &str,
		value: u128,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<TransferKeepAliveTx, TransactionFailed> {
		let dest = match utils::account_id_from_str(dest) {
			Ok(dest) => dest,
			Err(error) => return Err(TransactionFailed::from(error).into()),
		};

		let call = avail::tx()
			.balances()
			.transfer_keep_alive(dest.into(), value);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = find_event_or_return_error::<BalancesEvents::Transfer>(
			"Failed to find Transfer event",
			&details,
		)?;

		Ok(TransferKeepAliveTx { event, details })
	}
}
