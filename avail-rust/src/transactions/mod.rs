pub mod balances;
pub mod da;
pub mod nom_pools;
pub mod options;
pub mod session;
pub mod staking;

use crate::{
	error::ClientError,
	utils::{self, *},
	AExtrinsicEvents, AOnlineClient, ATxInBlock, AvailConfig, H256,
};

pub use options::{Mortality, Nonce, Options};

use std::sync::Arc;
use subxt::{backend::rpc::RpcClient, blocks::StaticExtrinsic, events::StaticEvent};

pub type Params =
	<<AvailConfig as subxt::Config>::ExtrinsicParams as subxt::config::ExtrinsicParams<
		AvailConfig,
	>>::Params;

#[derive(Clone)]
pub struct Transactions {
	pub balances: balances::Balances,
	pub staking: staking::Staking,
	pub data_availability: da::DataAvailability,
	pub session: session::Session,
	pub nomination_pools: nom_pools::NominationPools,
}

impl Transactions {
	pub fn new(online_client: AOnlineClient, rpc_client: RpcClient) -> Self {
		Self {
			balances: balances::Balances::new(online_client.clone(), rpc_client.clone()),
			staking: staking::Staking::new(online_client.clone(), rpc_client.clone()),
			data_availability: da::DataAvailability::new(online_client.clone(), rpc_client.clone()),
			session: session::Session::new(online_client.clone(), rpc_client.clone()),
			nomination_pools: nom_pools::NominationPools::new(
				online_client.clone(),
				rpc_client.clone(),
			),
		}
	}
}

#[derive(Debug, Clone)]
pub struct TransactionDetails {
	pub tx_in_block: Arc<ATxInBlock>,
	pub events: Arc<AExtrinsicEvents>,
	pub tx_hash: H256,
	pub tx_index: u32,
	pub block_hash: H256,
	pub block_number: u32,
}

impl TransactionDetails {
	pub fn new(
		tx_in_block: ATxInBlock,
		events: AExtrinsicEvents,
		tx_hash: H256,
		tx_index: u32,
		block_hash: H256,
		block_number: u32,
	) -> Self {
		Self {
			tx_in_block: tx_in_block.into(),
			events: events.into(),
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		}
	}

	pub async fn fetch_block(
		&self,
		client: &AOnlineClient,
	) -> Result<crate::block::Block, subxt::Error> {
		crate::block::Block::new(client, self.block_hash).await
	}

	pub fn check_if_transaction_was_successful(
		&self,
		client: &AOnlineClient,
	) -> Result<(), subxt::Error> {
		utils::check_if_transaction_was_successful(client, &self.events)
	}
}

#[derive(Debug)]
pub struct TransactionFailed {
	pub reason: ClientError,
	pub details: Option<TransactionDetails>,
}

impl TransactionFailed {
	pub fn new(reason: String, details: TransactionDetails) -> Self {
		Self {
			reason: ClientError::from(reason),
			details: Some(details),
		}
	}
}

impl From<&str> for TransactionFailed {
	fn from(value: &str) -> Self {
		Self {
			reason: ClientError::from(value),
			details: None,
		}
	}
}

impl From<(&str, TransactionDetails)> for TransactionFailed {
	fn from(value: (&str, TransactionDetails)) -> Self {
		Self {
			reason: ClientError::from(value.0),
			details: Some(value.1),
		}
	}
}

impl From<(String, TransactionDetails)> for TransactionFailed {
	fn from(value: (String, TransactionDetails)) -> Self {
		Self {
			reason: ClientError::from(value.0),
			details: Some(value.1),
		}
	}
}

impl From<String> for TransactionFailed {
	fn from(value: String) -> Self {
		Self {
			reason: ClientError::from(value),
			details: None,
		}
	}
}

impl From<ClientError> for TransactionFailed {
	fn from(value: ClientError) -> Self {
		Self {
			reason: value,
			details: None,
		}
	}
}

impl From<(ClientError, TransactionDetails)> for TransactionFailed {
	fn from(value: (ClientError, TransactionDetails)) -> Self {
		Self {
			reason: value.0,
			details: Some(value.1),
		}
	}
}

async fn find_data_or_return_error<T: StaticExtrinsic>(
	client: &AOnlineClient,
	error: &str,
	details: &TransactionDetails,
) -> Result<T, TransactionFailed> {
	let block = details.fetch_block(client).await;
	let block = block.map_err(|e| TransactionFailed {
		reason: e.into(),
		details: Some(details.clone()),
	})?;

	let data = block.transaction_by_index_static::<T>(details.tx_index);
	let data = match data {
		Some(d) => d.value,
		None => {
			return Err(TransactionFailed {
				reason: error.into(),
				details: Some(details.clone()),
			})
		},
	};

	Ok(data)
}

fn find_event_or_return_error<T: StaticEvent>(
	error: &str,
	details: &TransactionDetails,
) -> Result<T, TransactionFailed> {
	let event = details.events.find_first::<T>().ok().flatten();
	event.ok_or(TransactionFailed::from((error, details.clone())))
}

fn find_event_or_nothing<T: StaticEvent>(details: &TransactionDetails) -> Option<T> {
	details.events.find_first::<T>().ok().flatten()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::account::Account;

	#[tokio::test]
	async fn testing_function() {
		let sdk = crate::sdk::SDK::new("ws://127.0.0.1:9944").await.unwrap();
		let alice = Account::alice(sdk).unwrap();
		let a = alice.get_app_keys().await.unwrap();
		dbg!(a);
	}
}
