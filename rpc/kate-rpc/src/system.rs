use avail_core::OpaqueExtrinsic;
use codec::Encode;
use frame_system_rpc_runtime_api::SystemEventsApi;
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::ErrorObject,
};
use sc_client_api::BlockBackend;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::{Blake2Hasher, Hasher, H256};
use sp_runtime::traits::{Block as BlockT, Header as HeaderT};
use std::{
	marker::PhantomData,
	sync::{Arc, Mutex},
};

#[rpc(client, server)]
pub trait Api {
	#[method(name = "system_fetchEvents")]
	async fn fetch_events(
		&self,
		at: H256,
		options: Option<fetch_events::Options>,
	) -> RpcResult<fetch_events::ApiResult>;

	#[method(name = "system_fetchExtrinsics")]
	async fn fetch_extrinsics(
		&self,
		params: fetch_extrinsics::Params,
	) -> RpcResult<fetch_extrinsics::ApiResult>;

	#[method(name = "system_latestChainInfo")]
	async fn latest_chain_info(&self) -> RpcResult<types::ChainInfo>;

	#[method(name = "system_getBlockNumber")]
	async fn block_get_block_number(&self, hash: H256) -> RpcResult<Option<u32>>;
}

pub struct Rpc<C, Block>
where
	C: ProvideRuntimeApi<Block> + Send + Sync + 'static,
	C::Api: frame_system_rpc_runtime_api::SystemEventsApi<Block>,
	Block: BlockT,
{
	pub client: Arc<C>,
	pub block_cache: Arc<Mutex<fetch_extrinsics::Cache>>,
	_phantom: PhantomData<Block>,
}
impl<C, Block> Rpc<C, Block>
where
	C: ProvideRuntimeApi<Block> + Send + Sync + 'static,
	C: HeaderBackend<Block>,
	C::Api: frame_system_rpc_runtime_api::SystemEventsApi<Block>,
	Block: BlockT,
	<Block as BlockT>::Hash: From<H256>,
{
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			block_cache: Arc::new(Mutex::new(fetch_extrinsics::Cache::new(5))),
			_phantom: PhantomData,
		}
	}
}

/// Error type for this RPC API.
pub enum Error {
	/// Generic runtime error.
	RuntimeApi,
	// Invalid inputs
	InvalidInput,
	// No block found
	NoBlockFound,
	// Other
	Other,
}

impl Error {
	pub fn into_error_object<'a>(self, msg: String) -> ErrorObject<'a> {
		ErrorObject::owned(i32::from(self), msg, None::<()>)
	}
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::RuntimeApi => 1,
			Error::InvalidInput => 2,
			Error::NoBlockFound => 3,
			Error::Other => 4,
		}
	}
}

#[async_trait]
impl<'a, C, Block> ApiServer for Rpc<C, Block>
where
	C: ProvideRuntimeApi<Block> + Send + Sync + 'static,
	C: BlockBackend<Block>,
	C: HeaderBackend<Block>,
	C::Api: frame_system_rpc_runtime_api::SystemEventsApi<Block>,
	Block: BlockT<Extrinsic = OpaqueExtrinsic>,
	<Block as BlockT>::Hash: From<H256> + Into<H256>,
	<<Block as BlockT>::Header as HeaderT>::Number: From<u32>,
	<<Block as BlockT>::Header as HeaderT>::Number: Into<u32>,
{
	async fn fetch_events(
		&self,
		at: H256,
		options: Option<fetch_events::Options>,
	) -> RpcResult<fetch_events::ApiResult> {
		use fetch_events::GroupedRuntimeEvents;

		let runtime_api = self.client.runtime_api();
		let result = runtime_api
			.fetch_events(at.into(), options.unwrap_or_default())
			.map_err(|x| Error::RuntimeApi.into_error_object(x.to_string()))?;

		match result {
			Ok(res) => Ok(res.into_iter().map(GroupedRuntimeEvents::from).collect()),
			Err(code) => Err(Error::InvalidInput
				.into_error_object(std::format!("Runtime Api Error Code: {code}"))),
		}
	}

	async fn fetch_extrinsics(
		&self,
		params: fetch_extrinsics::Params,
	) -> RpcResult<fetch_extrinsics::ApiResult> {
		use fetch_extrinsics::{
			BlockId, EncodeSelector, ExtrinsicFilterOptions, ExtrinsicInformation,
		};

		if !params.is_valid() {
			return Err(Error::InvalidInput.into_error_object(String::from("Invalid input")));
		}

		let block_hash = match params.block_id {
			BlockId::Hash(h) => h,
			BlockId::Number(n) => {
				let hash = match self.client.block_hash(n.into()) {
					Ok(ok) => ok,
					Err(err) => return Err(Error::NoBlockFound.into_error_object(err.to_string())),
				};
				let Some(hash) = hash else {
					return Err(Error::NoBlockFound
						.into_error_object(String::from("Failed to find block hash")));
				};
				hash.into()
			},
		};
		let Ok(mut cache) = self.block_cache.lock() else {
			return Err(Error::Other.into_error_object(String::from("failed to lock mutex")));
		};

		let cached_block = match cache.block(block_hash) {
			Some(block) => block,
			None => {
				let block = fetch_extrinsics::cache_block::<C, Block>(&self.client, block_hash)?;
				cache.insert(block_hash, block)
			},
		};

		let extrinsics = cached_block.transactions();
		let mut found_extrinsics = match &params.extrinsic {
			ExtrinsicFilterOptions::All => Vec::with_capacity(extrinsics.len()),
			ExtrinsicFilterOptions::TxHash(list) => Vec::with_capacity(list.len()),
			ExtrinsicFilterOptions::TxIndex(list) => Vec::with_capacity(list.len()),
			_ => Vec::new(),
		};
		for tx in extrinsics.iter() {
			if !params.filter_in_tx_index(tx.index) || !params.filter_in_tx_hash(tx.tx_hash) {
				continue;
			}

			if !params.filter_in_pallet(tx.dispatch_index.0)
				|| !params.filter_in_pallet_call(tx.dispatch_index)
			{
				continue;
			}

			if !params.filter_in(&tx.signature) {
				continue;
			}

			let encoded = match params.encode_selector {
				EncodeSelector::None => None,
				EncodeSelector::Call => Some(tx.tx_encoded[tx.call_start_pos..].to_string()),
				EncodeSelector::Extrinsic => Some(tx.tx_encoded.clone()),
			};

			let ext_info = ExtrinsicInformation {
				encoded,
				tx_hash: tx.tx_hash,
				tx_index: tx.index,
				pallet_id: tx.dispatch_index.0,
				call_id: tx.dispatch_index.1,
				signature: tx.signature.clone(),
			};
			found_extrinsics.push(ext_info);

			if let ExtrinsicFilterOptions::TxIndex(list) = &params.extrinsic {
				if found_extrinsics.len() >= list.len() {
					break;
				}
			}

			if let ExtrinsicFilterOptions::TxHash(list) = &params.extrinsic {
				if found_extrinsics.len() >= list.len() {
					break;
				}
			}
		}

		cache.promote_block(block_hash);

		Ok(found_extrinsics)
	}

	async fn latest_chain_info(&self) -> RpcResult<types::ChainInfo> {
		let info = self.client.info();
		return Ok(types::ChainInfo {
			best_hash: info.best_hash.into(),
			best_height: info.best_number.into(),
			finalized_hash: info.finalized_hash.into(),
			finalized_height: info.finalized_number.into(),
			genesis_hash: info.genesis_hash.into(),
		});
	}

	async fn block_get_block_number(&self, hash: H256) -> RpcResult<Option<u32>> {
		let result = self
			.client
			.block_number_from_id(&sp_runtime::generic::BlockId::Hash(hash.into()))
			.map_err(|err| Error::Other.into_error_object(err.to_string()))?;
		Ok(result.map(|x| x.into()))
	}
}

pub mod types {
	use super::*;

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub struct ChainInfo {
		pub best_hash: H256,
		pub best_height: u32,
		pub finalized_hash: H256,
		pub finalized_height: u32,
		pub genesis_hash: H256,
	}
}

pub mod fetch_events {
	pub use frame_system_rpc_runtime_api::system_events_api::fetch_events::{
		GroupedRuntimeEvents as RuntimeGroupedRuntimeEvents, Options,
		RuntimeEvent as RuntimeRuntimeEvent,
	};
	pub type ApiResult = Vec<GroupedRuntimeEvents>;

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub struct GroupedRuntimeEvents {
		pub phase: frame_system::Phase,
		pub events: Vec<RuntimeEvent>,
	}

	impl GroupedRuntimeEvents {
		pub fn new(phase: frame_system::Phase) -> Self {
			Self {
				phase,
				events: Vec::new(),
			}
		}
	}

	impl From<RuntimeGroupedRuntimeEvents> for GroupedRuntimeEvents {
		fn from(value: RuntimeGroupedRuntimeEvents) -> Self {
			Self {
				phase: value.phase,
				events: value.events.into_iter().map(RuntimeEvent::from).collect(),
			}
		}
	}

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub struct RuntimeEvent {
		pub index: u32,
		// (Pallet Id, Event Id)
		pub emitted_index: (u8, u8),
		pub data: Option<String>,
	}

	impl From<RuntimeRuntimeEvent> for RuntimeEvent {
		fn from(value: RuntimeRuntimeEvent) -> Self {
			Self {
				index: value.index,
				emitted_index: value.emitted_index,
				data: value.data.map(const_hex::encode),
			}
		}
	}
}

pub mod fetch_extrinsics {
	use super::*;
	use avail_core::asdr::EXTRINSIC_FORMAT_VERSION;
	use codec::{Decode, Input};
	use da_runtime::{Address, Signature, SignedExtra};
	use serde::{Deserialize, Serialize};
	use sp_runtime::MultiAddress;
	type SignaturePayload = (Address, Signature, SignedExtra);

	pub type ApiResult = Vec<ExtrinsicInformation>;

	#[derive(Clone, Serialize, Deserialize)]
	pub struct ExtrinsicInformation {
		pub encoded: Option<String>,
		pub tx_hash: H256,
		pub tx_index: u32,
		pub pallet_id: u8,
		pub call_id: u8,
		pub signature: Option<TransactionSignature>,
	}

	#[derive(Clone, Copy, Serialize, Deserialize)]
	pub enum BlockId {
		/// Identify by block header hash.
		Hash(H256),
		/// Identify by block number.
		Number(u32),
	}

	#[derive(Clone, Serialize, Deserialize)]
	pub struct Params {
		pub block_id: BlockId,
		pub ss58_address: Option<String>,
		pub app_id: Option<u32>,
		pub nonce: Option<u32>,
		#[serde(default)]
		pub extrinsic: ExtrinsicFilterOptions,
		#[serde(default)]
		pub encode_selector: EncodeSelector,
	}

	impl Params {
		pub fn is_valid(&self) -> bool {
			if self.ss58_address.as_ref().is_some_and(|x| x.len() > 100) {
				return false;
			}

			let valid = match &self.extrinsic {
				ExtrinsicFilterOptions::All => true,
				ExtrinsicFilterOptions::TxHash(items) => items.len() < 30,
				ExtrinsicFilterOptions::TxIndex(items) => items.len() < 30,
				ExtrinsicFilterOptions::Pallet(items) => items.len() < 30,
				ExtrinsicFilterOptions::PalletCall(items) => items.len() < 30,
			};

			if !valid {
				return false;
			}

			true
		}

		pub fn filter_in(&self, signature: &Option<TransactionSignature>) -> bool {
			if !self.filter_in_ss58_address(signature.as_ref().and_then(|x| x.ss58_address.clone()))
			{
				return false;
			}

			if !self.filter_in_nonce(signature.as_ref().map(|x| x.nonce)) {
				return false;
			}

			true
		}

		pub fn filter_in_ss58_address(&self, value: Option<String>) -> bool {
			if self.ss58_address.is_none() {
				return true;
			}
			self.ss58_address == value
		}

		pub fn filter_in_nonce(&self, value: Option<u32>) -> bool {
			if self.nonce.is_none() {
				return true;
			}
			self.nonce == value
		}

		pub fn is_tx_hash(&self) -> bool {
			match &self.extrinsic {
				ExtrinsicFilterOptions::TxHash(_) => true,
				_ => false,
			}
		}

		pub fn filter_in_pallet(&self, value: u8) -> bool {
			let ExtrinsicFilterOptions::Pallet(list) = &self.extrinsic else {
				return true;
			};
			list.contains(&value)
		}

		pub fn filter_in_pallet_call(&self, value: (u8, u8)) -> bool {
			let ExtrinsicFilterOptions::PalletCall(list) = &self.extrinsic else {
				return true;
			};
			list.contains(&value)
		}

		pub fn filter_in_tx_hash(&self, value: H256) -> bool {
			let ExtrinsicFilterOptions::TxHash(list) = &self.extrinsic else {
				return true;
			};
			list.contains(&value)
		}

		pub fn filter_in_tx_index(&self, value: u32) -> bool {
			let ExtrinsicFilterOptions::TxIndex(list) = &self.extrinsic else {
				return true;
			};
			list.contains(&value)
		}

		pub fn is_call(&self) -> bool {
			match self.encode_selector {
				EncodeSelector::Call => true,
				_ => false,
			}
		}

		pub fn is_extrinsic(&self) -> bool {
			match self.encode_selector {
				EncodeSelector::Extrinsic => true,
				_ => false,
			}
		}
	}

	#[derive(Clone, Copy, Serialize, Deserialize)]
	#[repr(u8)]
	pub enum EncodeSelector {
		None = 0,
		Call = 1,
		Extrinsic = 2,
	}

	impl Default for EncodeSelector {
		fn default() -> Self {
			Self::Extrinsic
		}
	}

	#[derive(Clone, Serialize, Deserialize)]
	pub enum ExtrinsicFilterOptions {
		All,
		TxHash(Vec<H256>),
		TxIndex(Vec<u32>),
		Pallet(Vec<u8>),
		PalletCall(Vec<(u8, u8)>),
	}

	impl Default for ExtrinsicFilterOptions {
		fn default() -> Self {
			Self::All
		}
	}

	#[derive(Clone, Serialize, Deserialize)]
	pub struct TransactionSignature {
		pub ss58_address: Option<String>,
		pub nonce: u32,
		// pub app_id: u32,
		pub mortality: Option<(u64, u64)>,
	}

	impl TransactionSignature {
		pub fn from_signature_payload(sig: &Option<SignaturePayload>) -> Option<Self> {
			let Some(sig) = sig else {
				return None;
			};

			let ss58_address = if let MultiAddress::Id(id) = &sig.0 {
				Some(std::format!("{}", id))
			} else {
				None
			};
			let nonce = sig.2 .5 .0;
			// let app_id = sig.2 .8 .0 .0;
			let mortality = match sig.2 .4 .0 {
				sp_runtime::generic::Era::Immortal => None,
				sp_runtime::generic::Era::Mortal(x, y) => Some((x, y)),
			};

			let value = Self {
				ss58_address,
				nonce,
				// app_id: *app_id,
				mortality,
			};
			Some(value)
		}
	}

	pub struct CachedTransaction {
		pub index: u32,
		pub signature: Option<TransactionSignature>,
		pub dispatch_index: (u8, u8),
		pub tx_hash: H256,
		// This is the whole tx encoded together with signature and call
		pub tx_encoded: String,
		// position from where the call starts in the encoded transaction
		pub call_start_pos: usize,
	}

	#[derive(Default)]
	pub struct CachedBlock {
		transactions: Vec<CachedTransaction>,
	}

	impl CachedBlock {
		pub fn new(transactions: Vec<CachedTransaction>) -> Self {
			Self { transactions }
		}

		pub fn transactions(&self) -> &Vec<CachedTransaction> {
			&self.transactions
		}

		pub fn insert(&mut self, value: CachedTransaction) {
			self.transactions.push(value);
		}
	}

	pub struct Cache {
		pub(crate) blocks: Vec<(H256, CachedBlock)>,
		max_size: u32,
	}

	impl Cache {
		pub fn new(max_size: u32) -> Self {
			Self {
				blocks: Vec::new(),
				max_size,
			}
		}

		pub fn promote_block(&mut self, block_hash: H256) {
			if self.blocks.is_empty() {
				return;
			}

			if let Some(first) = self.blocks.last() {
				if first.0 == block_hash {
					return;
				}
			}

			let stop = self.blocks.len() - 1;
			let mut i = 0;
			while i < stop {
				if self.blocks[i].0 == block_hash {
					self.blocks.swap(i, i + 1);
				}

				i += 1;
			}
		}

		pub fn block(&self, block_hash: H256) -> Option<&CachedBlock> {
			self.blocks.iter().find(|x| x.0 == block_hash).map(|x| &x.1)
		}

		pub fn insert(&mut self, hash: H256, value: CachedBlock) -> &CachedBlock {
			if self.blocks.len() >= self.max_size as usize && !self.blocks.is_empty() {
				self.blocks.remove(0);
			}
			self.blocks.push((hash, value));
			&self.blocks.last().expect("Just added it").1
		}
	}

	pub fn cache_block<'a, C, Block>(client: &C, block_hash: H256) -> RpcResult<CachedBlock>
	where
		C: BlockBackend<Block>,
		Block: BlockT<Extrinsic = OpaqueExtrinsic>,
		<Block as BlockT>::Hash: From<H256> + Into<H256>,
		<<Block as BlockT>::Header as HeaderT>::Number: From<u32>,
	{
		let opaque_extrinsics = match client.block_body(block_hash.into()) {
			Ok(x) => x,
			Err(err) => return Err(Error::NoBlockFound.into_error_object(err.to_string())),
		};
		let Some(opaque_extrinsics) = opaque_extrinsics else {
			return Err(Error::NoBlockFound.into_error_object(String::from("No block found")));
		};

		let mut cached_transactions: Vec<CachedTransaction> =
			Vec::with_capacity(opaque_extrinsics.len());

		for (index, ext) in opaque_extrinsics.iter().enumerate() {
			let ext_slice = &mut ext.0.as_slice();
			let Ok(version) = ext_slice.read_byte() else {
				continue;
			};

			let is_signed = version & 0b1000_0000 != 0;
			let version = version & 0b0111_1111;
			if version != EXTRINSIC_FORMAT_VERSION {
				continue;
			}

			let signature = if is_signed {
				let Ok(signature) = SignaturePayload::decode(ext_slice) else {
					continue;
				};
				Some(signature)
			} else {
				None
			};
			let call_start_pos = ext.0.len() - ext_slice.len();
			let call_length = ext_slice.len();
			let Some(pallet_id) = ext.0.get(call_start_pos) else {
				continue;
			};
			let Some(call_id) = ext.0.get(call_start_pos + 1) else {
				continue;
			};
			let dispatch_index = (*pallet_id, *call_id);

			let (tx_encoded, tx_hash) = {
				let mut encoded: Vec<u8> = Vec::with_capacity(ext.0.len() + 4);
				codec::Compact::<u32>(ext.0.len() as u32).encode_to(&mut encoded);
				encoded.extend_from_slice(&ext.0);

				let tx_hash = Blake2Hasher::hash(&encoded);
				(const_hex::encode(encoded), tx_hash)
			};

			let signature = TransactionSignature::from_signature_payload(&signature);
			let encoded_call_start_pos = tx_encoded.len().saturating_sub(call_length * 2);

			let tx = CachedTransaction {
				index: index as u32,
				signature,
				dispatch_index,
				tx_hash,
				tx_encoded,
				call_start_pos: encoded_call_start_pos,
			};
			cached_transactions.push(tx)
		}

		Ok(CachedBlock::new(cached_transactions))
	}
}

#[cfg(test)]
mod test {
	use super::fetch_extrinsics::*;
	use sp_core::H256;

	#[test]
	fn cache_test() {
		let mut cache = Cache::new(3);
		assert_eq!(cache.blocks.len(), 0);

		let hash_01 = H256::random();
		let hash_02 = H256::random();
		let hash_03 = H256::random();
		cache.insert(hash_01, CachedBlock::default());
		cache.insert(hash_02, CachedBlock::default());
		cache.insert(hash_03, CachedBlock::default());

		assert!(cache.block(hash_01).is_some());
		assert!(cache.block(hash_02).is_some());
		assert!(cache.block(hash_03).is_some());

		assert_eq!(cache.blocks.len(), 3);
		assert_eq!(cache.blocks[0].0, hash_01);
		assert_eq!(cache.blocks[1].0, hash_02);
		assert_eq!(cache.blocks[2].0, hash_03);

		// Adding one more should remove the first hash
		let hash_04 = H256::random();
		cache.insert(hash_04, CachedBlock::default());

		assert_eq!(cache.blocks.len(), 3);
		assert_eq!(cache.blocks[0].0, hash_02);
		assert_eq!(cache.blocks[1].0, hash_03);
		assert_eq!(cache.blocks[2].0, hash_04);

		// The order should change if a block is promoted
		cache.promote_block(hash_02);

		assert_eq!(cache.blocks.len(), 3);
		assert_eq!(cache.blocks[0].0, hash_03);
		assert_eq!(cache.blocks[1].0, hash_04);
		assert_eq!(cache.blocks[2].0, hash_02);

		// Adding back hash_01 should remove hash_03
		cache.insert(hash_01, CachedBlock::default());

		assert_eq!(cache.blocks.len(), 3);
		assert_eq!(cache.blocks[0].0, hash_04);
		assert_eq!(cache.blocks[1].0, hash_02);
		assert_eq!(cache.blocks[2].0, hash_01);

		assert!(cache.block(hash_04).is_some());
		assert!(cache.block(hash_02).is_some());
		assert!(cache.block(hash_01).is_some());
	}
}
