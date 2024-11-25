use primitive_types::H256;
use subxt::{
	backend::{legacy::rpc_methods::Bytes, rpc::RpcClient},
	blocks::StaticExtrinsic,
	error::DispatchError,
	ext::scale_encode::EncodeAsFields,
	tx::{DefaultPayload, TxStatus},
};
use subxt_signer::sr25519::Keypair;

use crate::{
	avail::{self, runtime_types::da_runtime::primitives::SessionKeys},
	error::ClientError,
	rpcs::{account_next_index, get_best_block_hash, get_block_hash},
	transactions::{options::parse_options, TransactionDetails, TransactionFailed},
	AExtrinsicEvents, AOnlineClient, ATxInBlock, ATxProgress, AccountId, AppUncheckedExtrinsic,
	Options, WaitFor,
};

use core::str::FromStr;

pub async fn progress_transaction(
	maybe_tx_progress: Result<ATxProgress, subxt::Error>,
	wait_for: WaitFor,
) -> Result<ATxInBlock, ClientError> {
	let mut tx_progress = maybe_tx_progress?;

	while let Some(tx_status) = tx_progress.next().await {
		let tx_status = tx_status?;

		match tx_status {
			TxStatus::InBestBlock(tx_in_block) => {
				if wait_for == WaitFor::BlockInclusion {
					return Ok(tx_in_block);
				}
			},
			TxStatus::InFinalizedBlock(tx_in_block) => {
				if wait_for == WaitFor::BlockFinalization {
					return Ok(tx_in_block);
				}
			},
			TxStatus::Error { message } => return Err(ClientError::from(message)),
			TxStatus::Invalid { message } => return Err(ClientError::from(message)),
			TxStatus::Dropped { message } => return Err(ClientError::from(message)),
			_ => {},
		};
	}

	Err(ClientError::from("Something went wrong."))
}

pub async fn parse_transaction_in_block(
	client: &AOnlineClient,
	tx_in_block: ATxInBlock,
) -> Result<TransactionDetails, TransactionFailed> {
	// Fetch transaction details
	let details = match fetch_transaction_details(client, tx_in_block).await {
		Ok(d) => d,
		Err(error) => return Err(TransactionFailed::from(ClientError::from(error))),
	};

	// Check if the transaction was successful
	if let Err(error) = details.check_if_transaction_was_successful(client) {
		return Err(TransactionFailed::from((ClientError::from(error), details)));
	}

	Ok(details)
}

pub async fn progress_and_parse_transaction<T>(
	online_client: &AOnlineClient,
	rpc_client: &RpcClient,
	account: &Keypair,
	call: &DefaultPayload<T>,
	wait_for: WaitFor,
	options: Option<Options>,
) -> Result<TransactionDetails, TransactionFailed>
where
	T: StaticExtrinsic + EncodeAsFields,
{
	let account_id = account.public_key().to_account_id();
	let params = parse_options(online_client, rpc_client, &account_id, options).await?;

	let tx_client = online_client.tx();
	let maybe_tx_progress = tx_client
		.sign_and_submit_then_watch(call, account, params)
		.await;
	let tx_in_block = progress_transaction(maybe_tx_progress, wait_for).await?;
	let tx_details = parse_transaction_in_block(online_client, tx_in_block).await?;

	Ok(tx_details)
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

async fn fetch_transaction_details(
	client: &AOnlineClient,
	tx_in_block: ATxInBlock,
) -> Result<TransactionDetails, subxt::Error> {
	let events = tx_in_block.fetch_events().await?;
	let tx_hash = tx_in_block.extrinsic_hash();
	let tx_index = events.extrinsic_index();
	let block_hash = tx_in_block.block_hash();
	let block_number = get_block_number(client, block_hash).await?;

	let details = TransactionDetails {
		tx_in_block: tx_in_block.into(),
		events: events.into(),
		tx_hash,
		tx_index,
		block_hash,
		block_number,
	};

	Ok(details)
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
	let hash = get_best_block_hash(rpc_client).await?;
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

async fn get_block_number(client: &AOnlineClient, block_hash: H256) -> Result<u32, subxt::Error> {
	let block_number = client.blocks().at(block_hash).await?.number();

	Ok(block_number)
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
