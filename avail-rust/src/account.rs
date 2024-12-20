use std::str::FromStr;
use subxt::{
	backend::rpc::reconnecting_rpc_client::RpcClient, blocks::StaticExtrinsic,
	ext::scale_encode::EncodeAsFields,
};

use crate::{
	avail,
	error::ClientError,
	rpcs::get_block_hash,
	transactions::{Transaction, TransactionDetails},
	utils, AOnlineClient, AccountId, Keypair, Options, SecretUri, WaitFor, SDK,
};

pub struct Account {
	pub keyring: Keypair,
}

impl From<Keypair> for Account {
	fn from(value: Keypair) -> Self {
		Self { keyring: value }
	}
}

impl TryFrom<SecretUri> for Account {
	type Error = ClientError;

	fn try_from(value: SecretUri) -> Result<Self, Self::Error> {
		let keyring = Keypair::from_uri(&value)?;
		Ok(Self { keyring })
	}
}

impl TryFrom<&str> for Account {
	type Error = ClientError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let secret_uri = SecretUri::from_str(value)?;
		Account::try_from(secret_uri)
	}
}

impl TryFrom<String> for Account {
	type Error = ClientError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Account::try_from(value.as_str())
	}
}

impl Account {
	pub fn new(keyring: Keypair) -> Self {
		Self { keyring }
	}

	pub fn alice() -> Result<Self, ClientError> {
		Account::try_from("//Alice")
	}

	pub fn bob() -> Result<Self, ClientError> {
		Account::try_from("//Bob")
	}

	pub fn eve() -> Result<Self, ClientError> {
		Account::try_from("//Eve")
	}

	pub fn address(&self) -> String {
		self.account_id().to_string()
	}

	pub fn account_id(&self) -> AccountId {
		self.keyring.public_key().to_account_id()
	}

	pub fn one_avail() -> u128 {
		SDK::one_avail()
	}

	pub async fn execute<T>(
		&self,
		tx: Transaction<T>,
		wait_for: WaitFor,
		options: Option<Options>,
	) -> Result<TransactionDetails, ClientError>
	where
		T: StaticExtrinsic + EncodeAsFields,
	{
		tx.execute(wait_for, &self.keyring, options, Some(6)).await
	}

	pub async fn get_nonce_state(
		&self,
		online_client: &AOnlineClient,
		rpc_client: &RpcClient,
	) -> Result<u32, ClientError> {
		utils::get_nonce_state(online_client, rpc_client, &self.address()).await
	}

	pub async fn get_nonce_node(&self, rpc_client: &RpcClient) -> Result<u32, ClientError> {
		utils::get_nonce_node(rpc_client, &self.address()).await
	}

	pub async fn get_app_keys(
		&self,
		online_client: &AOnlineClient,
		rpc_client: &RpcClient,
	) -> Result<Vec<(String, u32)>, String> {
		let block_hash = get_block_hash(rpc_client, None).await;
		let block_hash = block_hash.map_err(|e| e.to_string())?;

		let storage = online_client.storage().at(block_hash);
		let address = avail::storage().data_availability().app_keys_iter();

		let mut app_keys = storage.iter(address).await.map_err(|e| e.to_string())?;

		let account_id = self.keyring.public_key().to_account_id();
		let mut result = Vec::new();
		while let Some(Ok(kv)) = app_keys.next().await {
			let key = (&kv.key_bytes[49..]).to_vec();
			let key = String::from_utf8(key).unwrap();

			if kv.value.owner == account_id {
				result.push((key.clone(), kv.value.id.0.clone()));
			}
		}

		result.sort_by(|a, b| a.1.cmp(&b.1));

		Ok(result)
	}

	pub async fn get_app_ids(
		&self,
		online_client: &AOnlineClient,
		rpc_client: &RpcClient,
	) -> Result<Vec<u32>, String> {
		let keys = self.get_app_keys(online_client, rpc_client).await?;
		Ok(keys.into_iter().map(|v| v.1).collect())
	}
}
