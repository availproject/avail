pub mod balances;
pub mod da;
pub mod nom_pools;
pub mod options;
pub mod session;
pub mod staking;

use crate::{
	error::ClientError,
	from_substrate::FeeDetails,
	rpcs::query_fee_details,
	utils::{self, *},
	AExtrinsicEvents, AOnlineClient, ATxInBlock, AvailConfig, WaitFor, H256,
};

use options::parse_options;
pub use options::{Mortality, Nonce, Options};
use subxt_signer::sr25519::Keypair;

use std::sync::Arc;
use subxt::{
	backend::rpc::RpcClient, blocks::StaticExtrinsic, events::StaticEvent,
	ext::scale_encode::EncodeAsFields, tx::DefaultPayload,
};

pub type Params =
	<<AvailConfig as subxt::Config>::ExtrinsicParams as subxt::config::ExtrinsicParams<
		AvailConfig,
	>>::Params;

pub use crate::avail::balances::events as BalancesEvents;
pub use crate::avail::data_availability::events as DataAvailabilityEvents;
pub use crate::avail::nomination_pools::events as NominationPoolsEvents;
pub use crate::avail::session::events as SessionEvents;
pub use crate::avail::staking::events as StakingEvents;
pub use crate::avail::system::events as SystemEvents;

pub use crate::avail::balances::calls::types as BalancesCalls;
pub use crate::avail::data_availability::calls::types as DataAvailabilityCalls;
pub use crate::avail::nomination_pools::calls::types as NominationPoolsCalls;
pub use crate::avail::session::calls::types as SessionCalls;
pub use crate::avail::staking::calls::types as StakingCalls;
pub use crate::avail::system::calls::types as SystemCalls;

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

	pub fn print_debug(&self) {
		let formatted_string = format!(
			r#"
TransactionDetails {{
    tx_in_block: TxInBlock {{...}},
    events: ExtrinsicEvents {{
        ext_hash: {:?},
        idx: {},
        events: Events {{
            num_events: {},
            ...
        }},
    }},
    tx_hash: {:?},
    tx_index: {},
    block_hash: {:?},
    block_number: {},
}}
		"#,
			self.events.extrinsic_hash(),
			self.events.extrinsic_index(),
			self.events.all_events_in_block().len(),
			self.tx_hash,
			self.tx_index,
			self.block_hash,
			self.block_number
		);

		println!("{}", formatted_string);
	}

	pub async fn fetch_block(
		&self,
		client: &AOnlineClient,
	) -> Result<crate::block::Block, subxt::Error> {
		crate::block::Block::new(client, self.block_hash).await
	}

	pub fn find_first_event<T>(&self) -> Option<T>
	where
		T: StaticEvent,
	{
		self.events.find_first::<T>().ok().flatten()
	}

	pub fn find_last_event<T>(&self) -> Option<T>
	where
		T: StaticEvent,
	{
		self.events.find_last::<T>().ok().flatten()
	}

	pub fn find_all_events<T>(&self) -> Vec<T>
	where
		T: StaticEvent,
	{
		self.events.find::<T>().flatten().collect()
	}

	pub async fn get_data<T>(&self, client: &AOnlineClient) -> Option<T>
	where
		T: StaticExtrinsic,
	{
		let block = self.fetch_block(client).await.ok()?;
		let tx = block.transaction_by_index_static::<T>(self.tx_index)?;
		Some(tx.value)
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

#[derive(Debug, Clone)]
pub struct Transaction<T>
where
	T: StaticExtrinsic + EncodeAsFields,
{
	online_client: AOnlineClient,
	rpc_client: RpcClient,
	payload: DefaultPayload<T>,
}
impl<T> Transaction<T>
where
	T: StaticExtrinsic + EncodeAsFields,
{
	pub fn new(
		online_client: AOnlineClient,
		rpc_client: RpcClient,
		payload: DefaultPayload<T>,
	) -> Self {
		Self {
			online_client,
			rpc_client,
			payload,
		}
	}

	pub async fn execute_wait_for_inclusion(
		&self,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<TransactionDetails, TransactionFailed> {
		self.execute(WaitFor::BlockInclusion, account, options)
			.await
	}

	pub async fn execute_wait_for_finalization(
		&self,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<TransactionDetails, TransactionFailed> {
		self.execute(WaitFor::BlockFinalization, account, options)
			.await
	}

	pub async fn execute(
		&self,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<TransactionDetails, TransactionFailed> {
		progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			&self.payload,
			wait_for,
			options,
		)
		.await
	}

	pub async fn execute_and_forget(
		&self,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<H256, TransactionFailed> {
		sign_send_and_forget(
			&self.online_client,
			&self.rpc_client,
			account,
			&self.payload,
			options,
		)
		.await
	}

	pub async fn payment_query_info(
		&self,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<u128, ClientError> {
		let account_id = account.public_key().to_account_id();
		let params =
			parse_options(&self.online_client, &self.rpc_client, &account_id, options).await?;
		let tx = self
			.online_client
			.tx()
			.create_signed(&self.payload, account, params)
			.await?;

		Ok(tx.partial_fee_estimate().await?)
	}

	pub async fn payment_query_fee_details(
		&self,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<FeeDetails, ClientError> {
		let account_id = account.public_key().to_account_id();
		let params =
			parse_options(&self.online_client, &self.rpc_client, &account_id, options).await?;
		let tx = self
			.online_client
			.tx()
			.create_signed(&self.payload, account, params)
			.await?;

		let len_bytes: [u8; 4] = (tx.encoded().len() as u32).to_le_bytes();
		let encoded_with_len = [tx.encoded(), &len_bytes[..]].concat();

		Ok(query_fee_details(&self.rpc_client, encoded_with_len.into(), None).await?)
	}
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
