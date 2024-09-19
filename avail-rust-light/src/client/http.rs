use super::params::{Extra, Mortality, Nonce};
use crate::core::{
	crypto::{AccountId, Signature, Ss58Codec},
	types::{
		self,
		avail::{BlockHeader, RuntimeVersion},
		Additional, AlreadyEncoded, Call, Era, OpaqueTransaction, UnsignedEncodedPayload,
		UnsignedPayload, H256,
	},
};
use jsonrpsee_core::{client::ClientT, traits::ToRpcParams, JsonRawValue as RawValue};
use jsonrpsee_http_client::HttpClient as JRPSHttpClient;
use parity_scale_codec::Compact;
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct Client(Arc<JRPSHttpClient>);

impl Client {
	pub fn new(endpoint: &str) -> Result<Self, ()> {
		let client = JRPSHttpClient::builder().build(endpoint).map_err(|_| ())?;
		Ok(Self(Arc::new(client)))
	}

	pub async fn build_payload(
		&self,
		call: Call,
		account_id: AccountId,
		extra: Extra,
	) -> Result<UnsignedEncodedPayload, ()> {
		let (nonce, mortality, tip, app_id) = extra.deconstruct();

		let app_id = Compact(app_id.unwrap_or(0u32));
		let tip = Compact(tip.unwrap_or(0u128));
		let nonce = self.check_nonce(nonce, &account_id).await?;
		let (mortality, fork_hash) = self.check_mortality(mortality).await?;

		let extra = types::Extra {
			mortality,
			nonce,
			tip,
			app_id,
		};

		let RuntimeVersion {
			spec_version,
			transaction_version,
			..
		} = self.rpc_state_get_runtime_version().await?;
		let genesis_hash = self.rpc_chain_spec_v1_genesis_hash().await?;

		let additional =
			Additional::new(spec_version, transaction_version, genesis_hash, fork_hash);

		Ok(UnsignedPayload::new(call, extra, additional).encode())
	}

	async fn check_nonce(
		&self,
		nonce: Option<Nonce>,
		account_id: &AccountId,
	) -> Result<Compact<u32>, ()> {
		let nonce = match nonce {
			Some(Nonce::BestBlockAndTxPool) | None => {
				self.rpc_system_account_next_index(&account_id).await?
			},
			Some(Nonce::BestBlock) => {
				let block_hash = self.get_best_block_hash().await?;
				self.account_nonce_api_account_nonce(&account_id, block_hash)
					.await?
			},
			Some(Nonce::FinalizedBlock) => {
				let block_hash = self.get_finalized_block_hash().await?;
				self.account_nonce_api_account_nonce(&account_id, block_hash)
					.await?
			},
			Some(Nonce::Custom(n)) => n,
		};

		Ok(Compact(nonce))
	}

	async fn check_mortality(&self, mortality: Option<Mortality>) -> Result<(Era, H256), ()> {
		let (era, fork_hash) = match mortality {
			Some(x) => match x {
				Mortality::Period(period) => {
					let hash = self.get_best_block_hash().await?;
					let header = self.get_header(Some(hash)).await?;
					let number = header.number;
					(Era::mortal(period, number as u64), hash)
				},
				Mortality::Custom((period, best_number, block_hash)) => {
					(Era::mortal(period, best_number as u64), block_hash)
				},
			},
			None => {
				let hash = self.get_best_block_hash().await?;
				let header = self.get_header(Some(hash)).await?;
				let number = header.number;
				(Era::mortal(32, number as u64), hash)
			},
		};

		Ok((era, fork_hash))
	}

	pub fn sign(
		&self,
		payload: &UnsignedEncodedPayload,
		account_id: AccountId,
		signature: Signature,
	) -> OpaqueTransaction {
		OpaqueTransaction::new(&payload.extra, &payload.call, account_id, signature)
	}

	pub async fn get_header(&self, hash: Option<H256>) -> Result<BlockHeader, ()> {
		let mut params: Params = Params(None);
		if let Some(hash) = hash {
			let mut p = RpcParams::new();
			p.push(hash.to_hex_string()).map_err(|_| ())?;
			params = Params(p.build());
		}

		let header: BlockHeader = self
			.0
			.request::<BlockHeader, _>("chain_getHeader", params)
			.await
			.map_err(|_| ())?;

		Ok(header)
	}

	pub async fn get_best_block_hash(&self) -> Result<H256, ()> {
		let block_hash: String = self
			.0
			.request::<String, _>("chain_getBlockHash", Params(None))
			.await
			.map_err(|_| ())?;

		Ok(H256::from_hex_string(&block_hash)?)
	}

	pub async fn get_finalized_block_hash(&self) -> Result<H256, ()> {
		let block_hash: String = self
			.0
			.request::<String, _>("chain_getFinalizedHead", Params(None))
			.await
			.map_err(|_| ())?;

		Ok(H256::from_hex_string(&block_hash)?)
	}

	pub async fn rpc_author_submit_extrinsic(&self, extrinsic: AlreadyEncoded) -> Result<H256, ()> {
		let mut params = RpcParams::new();
		params.push(extrinsic.to_hex_string()).map_err(|_| ())?;

		let block_hash: String = self
			.0
			.request::<String, _>("author_submitExtrinsic", Params(params.build()))
			.await
			.map_err(|_| ())?;

		Ok(H256::from_hex_string(&block_hash)?)
	}

	pub async fn rpc_system_account_next_index(&self, account_id: &AccountId) -> Result<u32, ()> {
		let mut params = RpcParams::new();
		params.push(account_id.to_ss58check()).map_err(|_| ())?;

		let nonce: u32 = self
			.0
			.request::<u32, _>("system_accountNextIndex", Params(params.build()))
			.await
			.map_err(|_| ())?;

		Ok(nonce)
	}

	// Needs caching
	pub async fn rpc_chain_spec_v1_genesis_hash(&self) -> Result<H256, ()> {
		let genesis_hash: String = self
			.0
			.request::<String, _>("chainSpec_v1_genesisHash", Params(None))
			.await
			.map_err(|_| ())?;

		Ok(H256::from_hex_string(&genesis_hash)?)
	}

	// Needs caching
	pub async fn rpc_state_get_runtime_version(&self) -> Result<RuntimeVersion, ()> {
		let runtime_version: RuntimeVersion = self
			.0
			.request::<RuntimeVersion, _>("state_getRuntimeVersion", Params(None))
			.await
			.map_err(|_| ())?;

		Ok(runtime_version)
	}

	async fn account_nonce_api_account_nonce(
		&self,
		account_id: &AccountId,
		block_hash: H256,
	) -> Result<u32, ()> {
		use parity_scale_codec::Decode;

		let mut params = RpcParams::new();
		params
			.push("AccountNonceApi_account_nonce")
			.map_err(|_| ())?;
		params.push(account_id.to_hex_string()).map_err(|_| ())?;
		params
			.push(Some(block_hash.to_hex_string()))
			.map_err(|_| ())?;

		let encoded_nonce: String = self
			.0
			.request::<String, _>("state_call", Params(params.build()))
			.await
			.map_err(|_| ())?;
		let encoded_nonce = hex::decode(&encoded_nonce[2..]).map_err(|_| ())?;

		Ok(u32::decode(&mut encoded_nonce.as_ref()).map_err(|_| ())?)
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
