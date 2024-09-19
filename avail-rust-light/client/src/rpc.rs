use crate::error::ClientError;
use jsonrpsee_core::{client::ClientT, traits::ToRpcParams, JsonRawValue as RawValue};
use jsonrpsee_http_client::HttpClient as JRPSHttpClient;
use sdk_core::{
	crypto::{AccountId, Ss58Codec},
	types::{
		avail::{BlockHeader, RuntimeVersion},
		OpaqueTransaction, H256,
	},
};
use serde::Serialize;

#[derive(Debug, Clone, Default)]
pub struct RpcParams(Vec<u8>);
impl ToRpcParams for RpcParams {
	fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, serde_json::Error> {
		let params = self.build();
		Ok(params)
	}
}

impl RpcParams {
	/// Create a new empty set of [`RpcParams`].
	pub fn new() -> Self {
		Self(Vec::new())
	}
	/// Push a parameter into our [`RpcParams`]. This serializes it to JSON
	/// in the process, and so will return an error if this is not possible.
	pub fn push<P: Serialize>(&mut self, param: P) -> Result<(), ClientError> {
		match self.0.len() {
			0 => self.0.push(b'['),
			_ => self.0.push(b','),
		};
		serde_json::to_writer(&mut self.0, &param).map_err(|e| ClientError::SerdeJson(e))?;

		Ok(())
	}
	/// Build a [`RawValue`] from our params, returning `None` if no parameters
	/// were provided.
	pub fn build(mut self) -> Option<Box<RawValue>> {
		if self.0.is_empty() {
			None
		} else {
			self.0.push(b']');
			let s = unsafe { String::from_utf8_unchecked(self.0) };
			Some(RawValue::from_string(s).expect("Should be valid JSON"))
		}
	}
}

pub async fn system_account_next_index(
	client: &JRPSHttpClient,
	account_id: &AccountId,
) -> Result<u32, ClientError> {
	let mut params = RpcParams::new();
	params.push(account_id.to_ss58check())?;

	let value: Result<u32, _> = client
		.request::<u32, _>("system_accountNextIndex", params)
		.await;

	value.map_err(|e| ClientError::Jsonrpsee(e))
}

pub async fn account_nonce_api_account_nonce(
	client: &JRPSHttpClient,
	account_id: &AccountId,
	block_hash: H256,
) -> Result<u32, ClientError> {
	use parity_scale_codec::Decode;

	let mut params = RpcParams::new();
	params.push("AccountNonceApi_account_nonce")?;
	params.push(account_id.to_hex_string())?;
	params.push(Some(block_hash.to_hex_string()))?;

	let encoded_value: Result<String, _> = client.request::<String, _>("state_call", params).await;
	let encoded_value: String = encoded_value.map_err(|e| ClientError::Jsonrpsee(e))?;
	let encoded_value =
		hex::decode(&encoded_value[2..]).map_err(|e| ClientError::FromHexError(e))?;

	u32::decode(&mut encoded_value.as_ref()).map_err(|e| ClientError::CodecError(e))
}

pub async fn fetch_best_block_hash(client: &JRPSHttpClient) -> Result<H256, ClientError> {
	let value: Result<String, _> = client
		.request::<String, _>("chain_getBlockHash", RpcParams::new())
		.await;
	let value: String = value.map_err(|e| ClientError::Jsonrpsee(e))?;

	H256::from_hex_string(&value).map_err(|e| ClientError::Core(e))
}

pub async fn fetch_finalized_block_hash(client: &JRPSHttpClient) -> Result<H256, ClientError> {
	let value: Result<String, _> = client
		.request::<String, _>("chain_getFinalizedHead", RpcParams::new())
		.await;
	let value: String = value.map_err(|e| ClientError::Jsonrpsee(e))?;

	H256::from_hex_string(&value).map_err(|e| ClientError::Core(e))
}

// Needs caching
pub async fn chain_spec_v1_genesis_hash(client: &JRPSHttpClient) -> Result<H256, ClientError> {
	let value: Result<String, _> = client
		.request::<String, _>("chainSpec_v1_genesisHash", RpcParams::new())
		.await;
	let value: String = value.map_err(|e| ClientError::Jsonrpsee(e))?;

	H256::from_hex_string(&value).map_err(|e| ClientError::Core(e))
}

// Needs caching
pub async fn state_get_runtime_version(
	client: &JRPSHttpClient,
) -> Result<RuntimeVersion, ClientError> {
	let value: Result<RuntimeVersion, _> = client
		.request::<RuntimeVersion, _>("state_getRuntimeVersion", RpcParams::new())
		.await;

	value.map_err(|e| ClientError::Jsonrpsee(e))
}

pub async fn fetch_block_header(
	client: &JRPSHttpClient,
	hash: Option<H256>,
) -> Result<BlockHeader, ClientError> {
	let mut params: RpcParams = RpcParams::new();
	if let Some(hash) = hash {
		params.push(hash.to_hex_string())?;
	}

	let value: Result<BlockHeader, _> = client
		.request::<BlockHeader, _>("chain_getHeader", params)
		.await;

	value.map_err(|e| ClientError::Jsonrpsee(e))
}

pub async fn author_submit_extrinsic(
	client: &JRPSHttpClient,
	extrinsic: OpaqueTransaction,
) -> Result<H256, ClientError> {
	let mut params = RpcParams::new();
	params.push(extrinsic.data.to_hex_string())?;

	let value: Result<String, _> = client
		.request::<String, _>("author_submitExtrinsic", params)
		.await;
	let value: String = value.map_err(|e| ClientError::Jsonrpsee(e))?;

	H256::from_hex_string(&value).map_err(|e| ClientError::Core(e))
}
