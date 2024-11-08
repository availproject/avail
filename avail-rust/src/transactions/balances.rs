use crate::sdk::WaitFor;
use crate::{avail, ABlocksClient, ATxClient, AccountId};

use avail::balances::events as BalancesEvents;
use avail::system::events as SystemEvents;
use std::str::FromStr;
use subxt::backend::rpc::RpcClient;
use subxt_signer::sr25519::Keypair;

use super::{options::Options, sign_and_submit_and_progress_transaction, TxResultDetails};

#[derive(Debug)]
pub struct TransferAllTx {
	pub event: BalancesEvents::Transfer,
	pub event2: Option<SystemEvents::KilledAccount>,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct TransferAllowDeathTx {
	pub event: BalancesEvents::Transfer,
	pub event2: Option<SystemEvents::KilledAccount>,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct TransferKeepAliveTx {
	pub event: BalancesEvents::Transfer,
	pub details: TxResultDetails,
}

#[derive(Clone)]
pub struct Balances {
	tx_client: ATxClient,
	blocks_client: ABlocksClient,
	rpc_client: RpcClient,
}

impl Balances {
	pub fn new(tx_client: ATxClient, rpc_client: RpcClient, blocks_client: ABlocksClient) -> Self {
		Self {
			tx_client,
			blocks_client,
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
	) -> Result<TransferAllTx, String> {
		let dest = match AccountId::from_str(dest) {
			Ok(dest) => dest,
			Err(error) => return Err(std::format!("{:?}", error)),
		};

		let call = avail::tx().balances().transfer_all(dest.into(), keep_alive);
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

		let event = details.events.find_first::<BalancesEvents::Transfer>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Transfer event"));
		};

		let event2 = details.events.find_first::<SystemEvents::KilledAccount>();
		let event2 = event2.ok().flatten();

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
	) -> Result<TransferAllowDeathTx, String> {
		let dest = match AccountId::from_str(dest) {
			Ok(dest) => dest,
			Err(error) => return Err(std::format!("{:?}", error)),
		};

		let call = avail::tx()
			.balances()
			.transfer_allow_death(dest.into(), amount);
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

		let event = details.events.find_first::<BalancesEvents::Transfer>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Transfer event"));
		};

		let event2 = details.events.find_first::<SystemEvents::KilledAccount>();
		let event2 = event2.ok().flatten();

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
	) -> Result<TransferKeepAliveTx, String> {
		let dest = match AccountId::from_str(dest) {
			Ok(dest) => dest,
			Err(error) => return Err(std::format!("{:?}", error)),
		};

		let call = avail::tx()
			.balances()
			.transfer_keep_alive(dest.into(), value);
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

		let event = details.events.find_first::<BalancesEvents::Transfer>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find Transfer event"));
		};

		Ok(TransferKeepAliveTx { event, details })
	}
}
