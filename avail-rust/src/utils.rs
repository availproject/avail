use log::{debug, info, log_enabled, warn};
use primitive_types::H256;
use subxt::{
	backend::{legacy::rpc_methods::Bytes, rpc::RpcClient},
	blocks::StaticExtrinsic,
	error::DispatchError,
	ext::scale_encode::EncodeAsFields,
	tx::DefaultPayload,
};
use subxt_signer::sr25519::Keypair;

use crate::{
	avail::{self, runtime_types::da_runtime::primitives::SessionKeys},
	block::Block,
	error::ClientError,
	rpcs::{account_next_index, get_block_hash},
	transactions::{options::parse_options, TransactionDetails, TransactionFailed},
	AExtrinsicEvents, AOnlineClient, AccountId, AppUncheckedExtrinsic, Options, WaitFor,
};

use core::str::FromStr;

/// Creates and signs an extrinsic and submits to the chain for block inclusion.
///
/// Returns `Ok` with the extrinsic hash if it is valid extrinsic.
///
/// # Note
///
/// Success does not mean the extrinsic has been included in the block, just that it is valid
/// and has been included in the transaction pool.
pub async fn sign_send_and_forget<T>(
	online_client: &AOnlineClient,
	rpc_client: &RpcClient,
	account: &Keypair,
	call: &DefaultPayload<T>,
	options: Option<Options>,
) -> Result<H256, TransactionFailed>
where
	T: StaticExtrinsic + EncodeAsFields,
{
	let account_id = account.public_key().to_account_id();
	let params = parse_options(online_client, rpc_client, &account_id, options).await?;

	let tx_client = online_client.tx();
	let tx_hash = tx_client
		.sign_and_submit(call, account, params)
		.await
		.map_err(|e| e.to_string())?;

	Ok(tx_hash)
}

pub async fn execute_and_watch_transaction<T>(
	online_client: &AOnlineClient,
	rpc_client: &RpcClient,
	account: &Keypair,
	call: &DefaultPayload<T>,
	wait_for: WaitFor,
	options: Option<Options>,
	block_timeout: Option<u32>,
	retry_count: Option<u32>,
) -> Result<TransactionDetails, ClientError>
where
	T: StaticExtrinsic + EncodeAsFields,
{
	let mut retry_count = retry_count.unwrap_or(0);
	loop {
		let tx_hash =
			execute_transaction(online_client, rpc_client, account, call, options).await?;
		let result = watch_transaction(online_client, tx_hash, wait_for, block_timeout).await;
		let error = match result {
			Ok(details) => return Ok(details),
			Err(err) => err,
		};

		match error {
			TransactionExecutionError::TransactionNotFound => (),
			TransactionExecutionError::BlockStreamFailure => {
				return Err(ClientError::TransactionExecution(error))
			},
			TransactionExecutionError::SubxtError(_) => {
				return Err(ClientError::TransactionExecution(error))
			},
		};

		if retry_count == 0 {
			warn!(target: "watcher", "Failed to find transaction. Tx Hash: {:?}. Aborting", tx_hash);
			return Err(ClientError::TransactionExecution(error));
		}

		info!(target: "watcher", "Failed to find transaction. Tx Hash: {:?}. Trying again.", tx_hash);
		retry_count -= 1;
	}
}

pub async fn execute_transaction<T>(
	online_client: &AOnlineClient,
	rpc_client: &RpcClient,
	account: &Keypair,
	call: &DefaultPayload<T>,
	options: Option<Options>,
) -> Result<H256, ClientError>
where
	T: StaticExtrinsic + EncodeAsFields,
{
	let account_id = account.public_key().to_account_id();
	let params = parse_options(online_client, rpc_client, &account_id, options).await?;

	let tx_client = online_client.tx();
	if log_enabled!(log::Level::Debug) {
		let address = account.public_key().to_account_id().to_string();
		let call_name = call.call_name();
		let pallet_name = call.pallet_name();
		let nonce = &params.4 .0;
		debug!(
			target: "transaction",
			"Signing and submitting new transaction. Account: {}, Nonce: {:?}, Pallet Name: {}, Call Name: {}",
			address, nonce, pallet_name, call_name
		);
	}
	let tx_hash = tx_client.sign_and_submit(call, account, params).await?;

	debug!(
		target: "transaction",
		"Transaction was submitted. Tx Hash: {:?}",
		tx_hash
	);
	Ok(tx_hash)
}

#[derive(Debug)]
pub enum TransactionExecutionError {
	TransactionNotFound,
	BlockStreamFailure,
	SubxtError(subxt::Error),
}

impl TransactionExecutionError {
	pub fn to_string(&self) -> String {
		match self {
			TransactionExecutionError::TransactionNotFound => {
				String::from("Transaction not found").to_string()
			},
			TransactionExecutionError::BlockStreamFailure => {
				String::from("Block Stream Failure").to_string()
			},
			TransactionExecutionError::SubxtError(error) => error.to_string(),
		}
	}
}

impl From<subxt::Error> for TransactionExecutionError {
	fn from(value: subxt::Error) -> Self {
		Self::SubxtError(value)
	}
}

pub async fn watch_transaction(
	online_client: &AOnlineClient,
	tx_hash: H256,
	wait_for: WaitFor,
	block_timeout: Option<u32>,
) -> Result<TransactionDetails, TransactionExecutionError> {
	let mut block_hash;
	let mut block_number;
	let tx_details;

	let mut stream = match wait_for == WaitFor::BlockInclusion {
		true => online_client.blocks().subscribe_all().await,
		false => online_client.blocks().subscribe_finalized().await,
	}?;

	let mut current_block_number: Option<u32> = None;
	let mut timeout_block_number: Option<u32> = None;

	debug!(target: "watcher", "Watching for Tx Hash: {:?}. Waiting for: {}, Block timeout: {:?}", tx_hash, wait_for.to_str(), block_timeout);
	loop {
		let Some(block) = stream.next().await else {
			return Err(TransactionExecutionError::BlockStreamFailure);
		};

		let block = block?;
		block_hash = block.hash();
		block_number = block.number();

		debug!(target: "watcher", "New block fetched. Hash: {:?}, Number: {}", block_hash, block_number);

		let transactions = block.extrinsics().await?;
		let tx_found = transactions.iter().find(|e| e.hash() == tx_hash);
		if let Some(tx) = tx_found {
			tx_details = tx;
			break;
		}

		// Block timeout logic
		let Some(block_timeout) = block_timeout else {
			continue;
		};

		if current_block_number.is_none() {
			current_block_number = Some(block_number);
			timeout_block_number = Some(block_number + block_timeout);
			debug!(target: "watcher", "Current Block Number: {}, Timeout Block Number: {}", block_number, block_number + block_timeout + 1);
		}
		if timeout_block_number.is_some_and(|timeout| block_number > timeout) {
			return Err(TransactionExecutionError::TransactionNotFound);
		}
	}

	let events = tx_details.events().await?;
	let tx_index = tx_details.index();

	debug!(target: "watcher", "Transaction was found. Tx Hash: {:?}, Tx Index: {}, Block Hash: {:?}, Block Number: {}", tx_hash, tx_index, block_hash, block_number);

	return Ok(TransactionDetails::new(
		events,
		tx_hash,
		tx_index,
		block_hash,
		block_number,
	));
}

pub fn check_if_transaction_was_successful(
	client: &AOnlineClient,
	events: &AExtrinsicEvents,
) -> Result<(), subxt::Error> {
	// Try to find any errors; return the first one we encounter.
	for ev in events.iter() {
		let ev = ev?;
		if ev.pallet_name() == "System" && ev.variant_name() == "ExtrinsicFailed" {
			let dispatch_error = DispatchError::decode_from(ev.field_bytes(), client.metadata())?;
			return Err(dispatch_error.into());
		}
	}

	Ok(())
}

pub fn decode_raw_block_rpc_extrinsics(
	extrinsics: Vec<Bytes>,
) -> Result<Vec<AppUncheckedExtrinsic>, String> {
	let extrinsics: Result<Vec<AppUncheckedExtrinsic>, String> = extrinsics
		.into_iter()
		.map(|e| AppUncheckedExtrinsic::try_from(e))
		.collect();

	extrinsics
}

pub fn deconstruct_session_keys(session_keys: Vec<u8>) -> Result<SessionKeys, String> {
	use crate::avail::runtime_types::sp_core::ed25519::Public as EDPublic;
	use crate::avail::runtime_types::sp_core::sr25519::Public as SRPublic;
	use crate::avail::runtime_types::{
		pallet_im_online, sp_authority_discovery, sp_consensus_babe, sp_consensus_grandpa,
	};
	use core::array::TryFromSliceError;

	if session_keys.len() != 128 {
		return Err(String::from(
			"Session keys len cannot have length be more or less than 128",
		));
	}

	let err = |e: TryFromSliceError| e.to_string();

	let babe: [u8; 32] = session_keys[0..32].try_into().map_err(err)?;
	let grandpa: [u8; 32] = session_keys[32..64].try_into().map_err(err)?;
	let im_online: [u8; 32] = session_keys[64..96].try_into().map_err(err)?;
	let authority_discovery: [u8; 32] = session_keys[96..128].try_into().map_err(err)?;

	Ok(SessionKeys {
		babe: sp_consensus_babe::app::Public(SRPublic(babe)),
		grandpa: sp_consensus_grandpa::app::Public(EDPublic(grandpa)),
		im_online: pallet_im_online::sr25519::app_sr25519::Public(SRPublic(im_online)),
		authority_discovery: sp_authority_discovery::app::Public(SRPublic(authority_discovery)),
	})
}

pub fn deconstruct_session_keys_string(session_keys: String) -> Result<SessionKeys, String> {
	if session_keys.len() != 256 {
		return Err(String::from(
			"Session keys len cannot have length be more or less than 256",
		));
	}

	let err = || String::from("Internal Math Error");
	let len = session_keys.len();
	let mut session_keys_u8: Vec<u8> = Vec::with_capacity(128);
	let mut iter = session_keys.chars();
	for _ in (0..len).step_by(2) {
		let value_1: u8 = iter
			.next()
			.and_then(|v| v.to_digit(16))
			.and_then(|v| Some((v * 16) as u8))
			.ok_or_else(err)?;
		let value_2: u8 = iter
			.next()
			.and_then(|v| v.to_digit(16))
			.and_then(|v| Some(v as u8))
			.ok_or_else(err)?;
		session_keys_u8.push(value_1 + value_2);
	}

	if session_keys_u8.len() != 128 {
		return Err(String::from(
			"Something went wrong and the length of the calculated session keys is wrong",
		));
	}

	deconstruct_session_keys(session_keys_u8)
}

pub async fn get_nonce_state(
	online_client: &AOnlineClient,
	rpc_client: &RpcClient,
	address: &str,
) -> Result<u32, ClientError> {
	let account = account_id_from_str(address)?;
	let hash = Block::fetch_best_block_hash(rpc_client).await?;
	let block = online_client.blocks().at(hash).await?;

	Ok(block.account_nonce(&account).await? as u32)
}

pub async fn get_nonce_node(client: &RpcClient, address: &str) -> Result<u32, ClientError> {
	let account = account_id_from_str(address)?;
	Ok(account_next_index(client, account.to_string()).await?)
}

pub fn account_id_from_str(value: &str) -> Result<AccountId, String> {
	AccountId::from_str(value).map_err(|e| std::format!("{:?}", e))
}

pub async fn get_app_keys(
	online_client: &AOnlineClient,
	rpc_client: &RpcClient,
	address: &str,
) -> Result<Vec<(String, u32)>, String> {
	let block_hash = get_block_hash(rpc_client, None).await;
	let block_hash = block_hash.map_err(|e| e.to_string())?;

	let storage = online_client.storage().at(block_hash);
	let storage_address = avail::storage().data_availability().app_keys_iter();

	let mut app_keys = storage
		.iter(storage_address)
		.await
		.map_err(|e| e.to_string())?;

	let mut result = Vec::new();
	while let Some(Ok(kv)) = app_keys.next().await {
		let key = (&kv.key_bytes[49..]).to_vec();
		let key = String::from_utf8(key).unwrap();

		if kv.value.owner.to_string() == address {
			result.push((key.clone(), kv.value.id.0.clone()));
		}
	}

	result.sort_by(|a, b| a.1.cmp(&b.1));

	Ok(result)
}

pub async fn get_app_ids(
	api: &AOnlineClient,
	rpc_client: &RpcClient,
	address: &str,
) -> Result<Vec<u32>, String> {
	let keys = match get_app_keys(api, rpc_client, address).await {
		Ok(k) => k,
		Err(e) => return Err(e),
	};

	Ok(keys.into_iter().map(|v| v.1).collect())
}
