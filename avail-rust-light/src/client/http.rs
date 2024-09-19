use crate::{
	core::types::{
		self, avail::BlockHeader, avail::RuntimeVersion, Additional, AlreadyEncoded, Era,
		OpaqueTransaction, UnsignedEncodedPayload, H256,
	},
	core::{AccountId, PublicKey, Signature, Ss58Codec},
};
use jsonrpsee_core::{client::ClientT, traits::ToRpcParams, JsonRawValue as RawValue};
use jsonrpsee_http_client::HttpClient as JRPSHttpClient;
use parity_scale_codec::Compact;
use serde::Serialize;
use std::sync::Arc;

use super::params::{Extra, Mortality, Nonce};

#[derive(Clone)]
pub struct Client(Arc<JRPSHttpClient>);

impl Client {
	pub fn new(endpoint: &str) -> Self {
		let client = JRPSHttpClient::builder().build(endpoint).unwrap();
		Self(Arc::new(client))
	}

	pub async fn build_payload(
		&self,
		call: types::Call,
		account_id: AccountId,
		extra: Extra,
	) -> UnsignedEncodedPayload {
		// We are not going to check if the call is properly setup. We will blindly trust it. :)
		let call = call;

		// Account ID cannot be in a invalid state so no checking needs to be done
		let account_id = account_id;

		// Extrinsic Extras deconstructed
		let (nonce, mortality, tip, app_id) = extra.deconstruct();

		// Checking Nonce
		let nonce = match nonce {
			Some(Nonce::BestBlockAndTxPool) | None => {
				self.rpc_system_account_next_index(account_id.clone()).await
			},
			Some(Nonce::BestBlock) => {
				let block_hash = self.get_best_block_hash().await;
				self.account_nonce_api_account_nonce(account_id.clone(), block_hash)
					.await
			},
			Some(Nonce::FinalizedBlock) => {
				let block_hash = self.get_finalized_block_hash().await;
				self.account_nonce_api_account_nonce(account_id.clone(), block_hash)
					.await
			},
			Some(Nonce::Custom(n)) => n,
		};
		let nonce = Compact(nonce);

		// We could do some checking for App ID but not now
		let app_id = Compact(app_id.unwrap_or(0u32));

		// No check for tip
		let tip = Compact(tip.unwrap_or(0u128));

		let genesis_hash = self.rpc_chainSpec_v1_genesis_hash().await;
		let best_block_hash = self.get_best_block_hash().await;
		let best_block_header = self.get_header(Some(best_block_hash)).await;
		let best_block_number = best_block_header.number;

		// Mortality Nonce
		let (mortality, fork_hash) = match mortality {
			Some(x) => match x {
				Mortality::Period(period) => (
					Era::mortal(period, best_block_number as u64),
					best_block_hash,
				),
				Mortality::Custom((period, best_number, block_hash)) => {
					(Era::mortal(period, best_number as u64), block_hash)
				},
			},
			None => (Era::mortal(32, best_block_number as u64), best_block_hash),
		};

		let extra = types::Extra {
			mortality,
			nonce,
			tip,
			app_id,
		};

		let rtv = self.rpc_state_get_runtime_version().await;
		let additional = Additional::new(
			rtv.spec_version,
			rtv.transaction_version,
			genesis_hash,
			fork_hash,
		);

		types::UnsignedPayload::new(call, extra, additional).encode()
	}

	pub fn sign(
		&self,
		payload: &UnsignedEncodedPayload,
		address: PublicKey,
		signature: Signature,
	) -> OpaqueTransaction {
		OpaqueTransaction::new(&payload.extra, &payload.call, address, signature)
	}

	pub async fn get_header(&self, hash: Option<H256>) -> BlockHeader {
		let mut params: Params = Params(None);
		if let Some(hash) = hash {
			let mut p = RpcParams::new();
			p.push(hash.to_hex_string()).unwrap();
			params = Params(p.build());
		}

		let header: BlockHeader = self
			.0
			.request::<BlockHeader, _>("chain_getHeader", params)
			.await
			.unwrap();

		header
	}

	pub async fn get_best_block_hash(&self) -> H256 {
		let block_hash: String = self
			.0
			.request::<String, _>("chain_getBlockHash", Params(None))
			.await
			.unwrap();

		H256::from_hex_string(&block_hash).unwrap()
	}

	pub async fn get_finalized_block_hash(&self) -> H256 {
		let block_hash: String = self
			.0
			.request::<String, _>("chain_getFinalizedHead", Params(None))
			.await
			.unwrap();

		H256::from_hex_string(&block_hash).unwrap()
	}

	pub async fn rpc_author_submit_extrinsic(&self, extrinsic: AlreadyEncoded) -> H256 {
		let mut params = RpcParams::new();
		params.push(extrinsic.to_hex_string()).unwrap();

		let block_hash: String = self
			.0
			.request::<String, _>("author_submitExtrinsic", Params(params.build()))
			.await
			.unwrap();

		H256::from_hex_string(&block_hash).unwrap()
	}

	pub async fn rpc_system_account_next_index(&self, account_id: AccountId) -> u32 {
		let mut params = RpcParams::new();
		params.push(account_id.to_ss58check()).unwrap();

		let nonce: u32 = self
			.0
			.request::<u32, _>("system_accountNextIndex", Params(params.build()))
			.await
			.unwrap();

		nonce
	}

	pub async fn rpc_chainSpec_v1_genesis_hash(&self) -> H256 {
		let genesis_hash: String = self
			.0
			.request::<String, _>("chainSpec_v1_genesisHash", Params(None))
			.await
			.unwrap();

		H256::from_hex_string(&genesis_hash).unwrap()
	}

	pub async fn rpc_state_get_runtime_version(&self) -> RuntimeVersion {
		let runtime_version: RuntimeVersion = self
			.0
			.request::<RuntimeVersion, _>("state_getRuntimeVersion", Params(None))
			.await
			.unwrap();

		runtime_version
	}

	async fn account_nonce_api_account_nonce(
		&self,
		account_id: AccountId,
		block_hash: H256,
	) -> u32 {
		use parity_scale_codec::Decode;

		let mut params = RpcParams::new();
		params.push("AccountNonceApi_account_nonce").unwrap();
		params.push(account_id.to_hex_string()).unwrap();
		params.push(Some(block_hash.to_hex_string())).unwrap();

		let encoded_nonce: String = self
			.0
			.request::<String, _>("state_call", Params(params.build()))
			.await
			.unwrap();
		let encoded_nonce = hex::decode(&encoded_nonce[2..]).unwrap();

		u32::decode(&mut encoded_nonce.as_ref()).unwrap()
	}
}

#[derive(Debug, Clone, Default)]
pub struct RpcParams(Vec<u8>);

struct Params(pub Option<Box<RawValue>>);

impl ToRpcParams for Params {
	fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, serde_json::Error> {
		Ok(self.0)
	}
}

impl RpcParams {
	/// Create a new empty set of [`RpcParams`].
	pub fn new() -> Self {
		Self(Vec::new())
	}
	/// Push a parameter into our [`RpcParams`]. This serializes it to JSON
	/// in the process, and so will return an error if this is not possible.
	pub fn push<P: Serialize>(&mut self, param: P) -> Result<(), ()> {
		if self.0.is_empty() {
			self.0.push(b'[');
		} else {
			self.0.push(b',')
		}
		serde_json::to_writer(&mut self.0, &param).unwrap();
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
