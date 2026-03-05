use codec::{Compact, Decode, Encode};
use frame_system_rpc_runtime_api::SystemEventsApi;
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::ErrorObject,
};
use parking_lot::Mutex;
use sc_client_api::BlockBackend;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::{Blake2Hasher, Hasher, H256};
use sp_runtime::traits::{Block as BlockT, Header as HeaderT};
use sp_runtime::OpaqueExtrinsic;
use std::{marker::PhantomData, sync::Arc};

#[rpc(client, server)]
pub trait Api {
	#[method(name = "custom_events")]
	async fn fetch_events(
		&self,
		block_id: types::BlockId,
		filter: Option<fetch_events::Filter>,
		fetch_data: Option<bool>,
	) -> RpcResult<fetch_events::Events>;

	#[method(name = "custom_extrinsics")]
	async fn fetch_extrinsics(
		&self,
		block_id: types::BlockId,
		allow_list: Option<Vec<fetch_extrinsics::AllowedExtrinsic>>,
		sig_filter: Option<fetch_extrinsics::SignatureFilter>,
		data_format: Option<fetch_extrinsics::DataFormat>,
	) -> RpcResult<fetch_extrinsics::Extrinsics>;

	#[method(name = "custom_chainInfo")]
	async fn fetch_chain_info(&self) -> RpcResult<types::ChainInfo>;

	#[method(name = "custom_blockNumber")]
	async fn fetch_block_number(&self, hash: H256) -> RpcResult<Option<u32>>;

	#[method(name = "custom_blockTimestamp")]
	async fn fetch_block_timestamp(&self, block_id: types::BlockId) -> RpcResult<u64>;
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
		block_id: types::BlockId,
		filter: Option<fetch_events::Filter>,
		fetch_data: Option<bool>,
	) -> RpcResult<fetch_events::Events> {
		use fetch_events::PhaseEvents;

		let filter = filter.unwrap_or_default();
		let fetch_data = fetch_data.unwrap_or(true);

		let block_hash = match block_id {
			types::BlockId::Hash(hash) => hash,
			types::BlockId::Number(number) => {
				let hash = match self.client.block_hash(number.into()) {
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

		let runtime_api = self.client.runtime_api();
		let result = runtime_api
			.fetch_events(block_hash.into(), filter, fetch_data)
			.map_err(|x| Error::RuntimeApi.into_error_object(x.to_string()))?;

		match result {
			Ok(res) => Ok(res.into_iter().map(PhaseEvents::from).collect()),
			Err(code) => Err(Error::InvalidInput
				.into_error_object(std::format!("Runtime Api Error Code: {code}"))),
		}
	}

	async fn fetch_extrinsics(
		&self,
		block_id: types::BlockId,
		allow_list: Option<Vec<fetch_extrinsics::AllowedExtrinsic>>,
		sig_filter: Option<fetch_extrinsics::SignatureFilter>,
		data_format: Option<fetch_extrinsics::DataFormat>,
	) -> RpcResult<fetch_extrinsics::Extrinsics> {
		use fetch_extrinsics::{AllowedExtrinsic, DataFormat, Extrinsic};
		use types::BlockId;
		const MAX_INDICES_COUNT: usize = 30;

		let sig_filter = sig_filter.unwrap_or_default();
		let data_format = data_format.unwrap_or_default();

		if let Some(allow_list) = &allow_list {
			if allow_list.len() > MAX_INDICES_COUNT {
				return Err(Error::InvalidInput.into_error_object(String::from(
					"Allow list: Invalid input. Cannot have more than 30 items",
				)));
			}
		}

		if !sig_filter.is_valid() {
			return Err(Error::InvalidInput
				.into_error_object(String::from("Signature filter: Invalid input")));
		}

		let block_hash = match block_id {
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

		let block = {
			let mut cache = self.block_cache.lock();
			match cache.block(block_hash) {
				Some(block) => block.clone(),
				None => {
					let block =
						fetch_extrinsics::cache_block::<C, Block>(&self.client, block_hash)?;
					cache.insert(block_hash, block).clone()
				},
			}
		};

		let transactions = &block.0;
		let mut allowed_extrinsics = Vec::new();

		for tx in transactions.iter() {
			// Check if it is allowed
			if let Some(allow_list) = &allow_list {
				let mut allowed = false;

				for rule in allow_list.iter() {
					match rule {
						AllowedExtrinsic::TxIndex(index) if *index == tx.index => {
							allowed = true;
							break;
						},
						AllowedExtrinsic::TxHash(hash) if *hash == tx.tx_hash => {
							allowed = true;
							break;
						},
						AllowedExtrinsic::Pallet(pallet) if *pallet == tx.pallet_id => {
							allowed = true;
							break;
						},
						AllowedExtrinsic::PalletCall(index)
							if *index == (tx.pallet_id, tx.variant_id) =>
						{
							allowed = true;
							break;
						},
						_ => (),
					};
				}

				if !allowed {
					continue;
				}
			}

			if !sig_filter.filter_in(&tx.signature) {
				continue;
			}

			let data = match data_format {
				DataFormat::None => String::new(),
				DataFormat::Call => tx.data[tx.call_start_pos..].to_string(),
				DataFormat::Extrinsic => tx.data.clone(),
			};

			let ext = Extrinsic {
				data,
				tx_hash: tx.tx_hash,
				tx_index: tx.index,
				pallet_id: tx.pallet_id,
				variant_id: tx.variant_id,
				signature: tx.signature.clone(),
			};

			allowed_extrinsics.push(ext);
		}

		{
			let mut cache = self.block_cache.lock();
			cache.promote_block(block_hash);
		}

		Ok(allowed_extrinsics)
	}

	async fn fetch_chain_info(&self) -> RpcResult<types::ChainInfo> {
		let info = self.client.info();
		return Ok(types::ChainInfo {
			best_hash: info.best_hash.into(),
			best_height: info.best_number.into(),
			finalized_hash: info.finalized_hash.into(),
			finalized_height: info.finalized_number.into(),
			genesis_hash: info.genesis_hash.into(),
		});
	}

	async fn fetch_block_number(&self, hash: H256) -> RpcResult<Option<u32>> {
		let result = self
			.client
			.block_number_from_id(&sp_runtime::generic::BlockId::Hash(hash.into()))
			.map_err(|err| Error::Other.into_error_object(err.to_string()))?;
		Ok(result.map(|x| x.into()))
	}

	async fn fetch_block_timestamp(&self, block_id: types::BlockId) -> RpcResult<u64> {
		use types::BlockId;

		const TIMESTAMP_SET_PALLET_ID: u8 = 3;
		const TIMESTAMP_SET_VARIANT_ID: u8 = 3;

		let block_hash = match block_id {
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

		let block = {
			let mut cache = self.block_cache.lock();
			match cache.block(block_hash) {
				Some(block) => block.clone(),
				None => {
					let block =
						fetch_extrinsics::cache_block::<C, Block>(&self.client, block_hash)?;
					cache.insert(block_hash, block).clone()
				},
			}
		};

		let Some(ext) = block.0.get(0) else {
			return Ok(0);
		};

		if ext.pallet_id != TIMESTAMP_SET_PALLET_ID || ext.variant_id != TIMESTAMP_SET_VARIANT_ID {
			return Ok(0);
		}

		let Ok(input) = const_hex::decode(&ext.data[ext.call_start_pos..]) else {
			return Ok(0);
		};

		let mut inp = &input[2..];

		let Ok(timestamp) = Compact::<u64>::decode(&mut inp) else {
			return Ok(0);
		};

		Ok(timestamp.0)
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

	#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
	pub enum BlockId {
		/// Identify by block header hash.
		Hash(H256),
		/// Identify by block number.
		Number(u32),
	}
}

pub mod fetch_events {
	pub use frame_system_rpc_runtime_api::system_events_api::fetch_events::{
		Filter, PhaseEvents as RuntimeGroupedRuntimeEvents, RuntimeEvent as RuntimeRuntimeEvent,
	};
	pub type Events = Vec<PhaseEvents>;

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub struct PhaseEvents {
		pub phase: frame_system::Phase,
		pub events: Vec<RuntimeEvent>,
	}

	impl PhaseEvents {
		pub fn new(phase: frame_system::Phase) -> Self {
			Self {
				phase,
				events: Vec::new(),
			}
		}
	}

	impl From<RuntimeGroupedRuntimeEvents> for PhaseEvents {
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
		pub data: String,
	}

	impl From<RuntimeRuntimeEvent> for RuntimeEvent {
		fn from(value: RuntimeRuntimeEvent) -> Self {
			Self {
				index: value.index,
				emitted_index: value.emitted_index,
				data: const_hex::encode(value.data),
			}
		}
	}
}

pub mod fetch_extrinsics {
	use super::*;
	// TODO: move this to appropriate place
	const EXTRINSIC_FORMAT_VERSION: u8 = 4;

	use codec::{Decode, Input};
	use da_runtime::{Address, Signature, SignedExtra};
	use serde::{Deserialize, Serialize};
	use sp_runtime::MultiAddress;
	type SignaturePayload = (Address, Signature, SignedExtra);

	pub type Extrinsics = Vec<Extrinsic>;

	#[derive(Clone, Serialize, Deserialize)]
	pub struct Extrinsic {
		pub data: String,
		pub tx_hash: H256,
		pub tx_index: u32,
		pub pallet_id: u8,
		pub variant_id: u8,
		pub signature: Option<TransactionSignature>,
	}

	#[derive(Clone, Default, Copy, Serialize, Deserialize)]
	#[repr(u8)]
	pub enum DataFormat {
		None = 0,
		Call = 1,
		#[default]
		Extrinsic = 2,
	}

	impl DataFormat {
		pub fn is_call(&self) -> bool {
			match self {
				DataFormat::Call => true,
				_ => false,
			}
		}

		pub fn is_extrinsic(&self) -> bool {
			match self {
				DataFormat::Extrinsic => true,
				_ => false,
			}
		}
	}

	#[derive(Clone, Serialize, Deserialize)]
	pub enum AllowedExtrinsic {
		TxHash(H256),
		TxIndex(u32),
		Pallet(u8),
		PalletCall((u8, u8)),
	}

	#[derive(Default, Clone, Serialize, Deserialize)]
	pub struct SignatureFilter {
		pub ss58_address: Option<String>,
		pub app_id: Option<u32>,
		pub nonce: Option<u32>,
	}

	impl SignatureFilter {
		pub fn is_valid(&self) -> bool {
			if self.ss58_address.as_ref().is_some_and(|x| x.len() > 100) {
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
	}

	#[derive(Clone, Serialize, Deserialize)]
	pub struct TransactionSignature {
		pub ss58_address: Option<String>,
		pub nonce: u32,
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

	#[derive(Clone)]
	pub struct CachedTransaction {
		pub index: u32,
		pub signature: Option<TransactionSignature>,
		pub pallet_id: u8,
		pub variant_id: u8,
		pub tx_hash: H256,
		// This is the whole tx encoded together with signature and call
		pub data: String,
		// position from where the call starts in the encoded transaction
		pub call_start_pos: usize,
	}

	#[derive(Default, Clone)]
	pub struct CachedBlock(pub Vec<CachedTransaction>);

	impl CachedBlock {
		pub fn new(transactions: Vec<CachedTransaction>) -> Self {
			Self(transactions)
		}

		pub fn insert(&mut self, value: CachedTransaction) {
			self.0.push(value);
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

	/// Recover the inner `Vec<u8>` from a `sp_runtime::OpaqueExtrinsic`.
	/// Returns `None` if something goes wrong (decode error) — the caller can decide to continue.
	fn opaque_into_inner_bytes(ext: &OpaqueExtrinsic) -> Option<Vec<u8>> {
		// Encode the wrapper (SCALE), then decode into Vec<u8> to recover the original inner Vec<u8>.
		// This avoids accessing the private field `0`.
		let encoded = ext.encode();
		Vec::<u8>::decode(&mut &encoded[..]).ok()
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
			let ext_inner = match opaque_into_inner_bytes(ext) {
				Some(b) => b,
				None => continue,
			};

			let ext_slice = &mut ext_inner.as_slice();

			// read version byte
			let Ok(version_byte) = ext_slice.read_byte() else {
				continue;
			};

			let is_signed = version_byte & 0b1000_0000 != 0;
			let version = version_byte & 0b0111_1111;
			if version != EXTRINSIC_FORMAT_VERSION {
				continue;
			}

			// parse signature payload if signed
			let signature = if is_signed {
				let Ok(signature) = SignaturePayload::decode(ext_slice) else {
					continue;
				};
				Some(signature)
			} else {
				None
			};
			let call_start_pos = ext_inner.len() - ext_slice.len();
			let call_length = ext_slice.len();
			let Some(pallet_id) = ext_inner.get(call_start_pos) else {
				continue;
			};
			let Some(variant_id) = ext_inner.get(call_start_pos + 1) else {
				continue;
			};

			// build tx_encoded and tx_hash using ext_inner
			let (tx_encoded, tx_hash) = {
				let mut encoded: Vec<u8> = Vec::with_capacity(ext_inner.len() + 4);
				codec::Compact::<u32>(ext_inner.len() as u32).encode_to(&mut encoded);
				encoded.extend_from_slice(&ext_inner);

				let tx_hash = Blake2Hasher::hash(&encoded);
				(const_hex::encode(encoded), tx_hash)
			};

			let signature = TransactionSignature::from_signature_payload(&signature);
			let encoded_call_start_pos = tx_encoded.len().saturating_sub(call_length * 2);

			let tx = CachedTransaction {
				index: index as u32,
				signature,
				pallet_id: *pallet_id,
				variant_id: *variant_id,
				tx_hash,
				data: tx_encoded,
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
