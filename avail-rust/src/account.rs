use std::str::FromStr;

use crate::{
	avail::{
		self,
		runtime_types::{frame_system::AccountInfo, pallet_balances::types::AccountData},
	},
	error::ClientError,
	transactions::{TransactionDetails, TransactionFailed},
	utils, AccountId, Keypair, Nonce, Options, SecretUri, WaitFor, SDK,
};

pub struct Account {
	sdk: SDK,
	keyring: Keypair,
	nonce: Option<Nonce>,
	app_id: Option<u32>,
	tip: Option<u128>,
	wait_for: WaitFor,
}

impl Account {
	pub fn new(sdk: SDK, keyring: Keypair) -> Self {
		Self {
			sdk,
			keyring,
			nonce: None,
			app_id: None,
			tip: None,
			wait_for: WaitFor::BlockInclusion,
		}
	}

	pub fn alice(sdk: SDK) -> Result<Self, String> {
		let secret_uri = SecretUri::from_str("//Alice").map_err(|e| e.to_string())?;
		let keyring = Keypair::from_uri(&secret_uri).map_err(|e| e.to_string())?;

		Ok(Account::new(sdk, keyring))
	}

	pub fn set_nonce(&mut self, value: Option<Nonce>) {
		self.nonce = value;
	}

	pub fn set_app_id(&mut self, value: Option<u32>) {
		self.app_id = value;
	}

	pub fn set_wait_for(&mut self, value: WaitFor) {
		self.wait_for = value;
	}

	pub fn set_tip(&mut self, value: Option<u128>) {
		self.tip = value;
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

	pub async fn balance_transfer(
		&self,
		dest: AccountId,
		value: u128,
	) -> Result<TransactionDetails, TransactionFailed> {
		let options = Some(self.build_options());
		let tx = self.sdk.tx.balances.transfer_keep_alive(dest, value);
		tx.execute(self.wait_for, &self.keyring, options).await
	}

	pub async fn submit_data(
		&self,
		data: Vec<u8>,
	) -> Result<TransactionDetails, TransactionFailed> {
		let options = Some(self.build_options());
		let tx = self.sdk.tx.data_availability.submit_data(data);
		tx.execute(self.wait_for, &self.keyring, options).await
	}

	pub async fn create_application_key(
		&self,
		key: Vec<u8>,
	) -> Result<TransactionDetails, TransactionFailed> {
		let options = Some(self.build_options());
		let tx = self.sdk.tx.data_availability.create_application_key(key);
		tx.execute(self.wait_for, &self.keyring, options).await
	}

	pub async fn get_balance(&self) -> Result<AccountInfo<u32, AccountData<u128>>, String> {
		let block_hash = self.sdk.rpc.chain.get_block_hash(None).await;
		let block_hash = block_hash.map_err(|e| e.to_string())?;

		let account_id = self.keyring.public_key().to_account_id();
		let storage = self.sdk.online_client.storage().at(block_hash);
		let address = avail::storage().system().account(account_id);

		let result = storage
			.fetch_or_default(&address)
			.await
			.map_err(|e| e.to_string())?;
		Ok(result)
	}

	pub async fn get_nonce_state(&self) -> Result<u32, ClientError> {
		utils::get_nonce_state(
			&self.sdk.online_client,
			&self.sdk.rpc_client,
			&self.address(),
		)
		.await
	}

	pub async fn get_nonce_node(&self) -> Result<u32, ClientError> {
		utils::get_nonce_node(&self.sdk.rpc_client, &self.address()).await
	}

	pub async fn get_app_keys(&self) -> Result<Vec<(String, u32)>, String> {
		let block_hash = self.sdk.rpc.chain.get_block_hash(None).await;
		let block_hash = block_hash.map_err(|e| e.to_string())?;

		let storage = self.sdk.online_client.storage().at(block_hash);
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

	pub async fn get_app_ids(&self) -> Result<Vec<u32>, String> {
		let keys = match self.get_app_keys().await {
			Ok(k) => k,
			Err(e) => return Err(e),
		};

		Ok(keys.into_iter().map(|v| v.1).collect())
	}

	fn build_options(&self) -> Options {
		let mut options = Options::new();
		if let Some(nonce) = &self.nonce {
			options = options.nonce(nonce.clone());
		}

		if let Some(app_id) = &self.app_id {
			options = options.app_id(app_id.clone());
		}

		if let Some(tip) = &self.tip {
			options = options.tip(tip.clone());
		}

		options
	}
}
