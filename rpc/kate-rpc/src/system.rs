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
use sp_core::{Blake2Hasher, Hasher, H256};
use sp_runtime::traits::{Block as BlockT, Header as HeaderT};
use std::{
	marker::PhantomData,
	sync::{Arc, Mutex},
};

#[rpc(client, server)]
pub trait Api {
	#[method(name = "system_fetchEventsV1")]
	async fn fetch_events_v1(
		&self,
		at: H256,
		options: Option<fetch_events_v1::Options>,
	) -> RpcResult<fetch_events_v1::ApiResult>;

	#[method(name = "system_fetchExtrinsicsV1")]
	async fn fetch_extrinsics_v1(
		&self,
		block_id: fetch_extrinsics_v1::BlockId,
		options: Option<fetch_extrinsics_v1::Options>,
	) -> RpcResult<fetch_extrinsics_v1::ApiResult>;
}

pub struct Rpc<C, Block>
where
	C: ProvideRuntimeApi<Block> + Send + Sync + 'static,
	C::Api: frame_system_rpc_runtime_api::SystemEventsApi<Block>,
	Block: BlockT,
{
	pub client: Arc<C>,
	pub block_cache: Arc<Mutex<fetch_extrinsics_v1::Cache>>,
	_phantom: PhantomData<Block>,
}
impl<C, Block> Rpc<C, Block>
where
	C: ProvideRuntimeApi<Block> + Send + Sync + 'static,
	C::Api: frame_system_rpc_runtime_api::SystemEventsApi<Block>,
	Block: BlockT,
	<Block as BlockT>::Hash: From<H256>,
{
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			block_cache: Arc::new(Mutex::new(fetch_extrinsics_v1::Cache::new(5))),
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
	C::Api: frame_system_rpc_runtime_api::SystemEventsApi<Block>,
	Block: BlockT<Extrinsic = OpaqueExtrinsic>,
	<Block as BlockT>::Hash: From<H256> + Into<H256>,
	<<Block as BlockT>::Header as HeaderT>::Number: From<u32>,
{
	async fn fetch_events_v1(
		&self,
		at: H256,
		options: Option<fetch_events_v1::Options>,
	) -> RpcResult<fetch_events_v1::ApiResult> {
		use fetch_events_v1::GroupedRuntimeEvents;

		let runtime_api = self.client.runtime_api();
		let result = runtime_api
			.fetch_events_v1(at.into(), options.unwrap_or_default())
			.map_err(|x| Error::RuntimeApi.into_error_object(x.to_string()))?;

		match result {
			Ok(res) => Ok(res.into_iter().map(GroupedRuntimeEvents::from).collect()),
			Err(code) => Err(Error::InvalidInput
				.into_error_object(std::format!("Runtime Api Error Code: {code}"))),
		}
	}

	async fn fetch_extrinsics_v1(
		&self,
		block_id: fetch_extrinsics_v1::BlockId,
		options: Option<fetch_extrinsics_v1::Options>,
	) -> RpcResult<fetch_extrinsics_v1::ApiResult> {
		use fetch_extrinsics_v1::{
			BlockId, EncodeSelector, ExtrinsicInformation, TransactionFilterOptions,
		};
		let options = options.unwrap_or_default();
		let filter = options.filter.unwrap_or_default();
		let tx_filter = filter.transaction.unwrap_or_default();
		let sig_filter = filter.signature.unwrap_or_default();
		let encode_selector = options.encode_selector.unwrap_or_default();

		if !tx_filter.is_valid() {
			return Err(Error::InvalidInput
				.into_error_object(String::from("Transaction filter: Invalid input")));
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
		let Ok(mut cache) = self.block_cache.lock() else {
			return Err(Error::Other.into_error_object(String::from("failed to lock mutex")));
		};

		let cached_block = match cache.block(block_hash) {
			Some(block) => block,
			None => {
				let block = fetch_extrinsics_v1::cache_block::<C, Block>(&self.client, block_hash)?;
				cache.insert(block_hash, block)
			},
		};

		let transactions = cached_block.transactions();
		let mut found_extrinsics = match &tx_filter {
			TransactionFilterOptions::All => Vec::with_capacity(transactions.len()),
			TransactionFilterOptions::TxHash(list) => Vec::with_capacity(list.len()),
			TransactionFilterOptions::TxIndex(list) => Vec::with_capacity(list.len()),
			_ => Vec::new(),
		};
		for tx in transactions.iter() {
			if !tx_filter.filter_in_tx_index(tx.index) || !tx_filter.filter_in_tx_hash(tx.tx_hash) {
				continue;
			}

			if !tx_filter.filter_in_pallet(tx.dispatch_index.0)
				|| !tx_filter.filter_in_pallet_call(tx.dispatch_index)
			{
				continue;
			}

			if !sig_filter.filter_in(&tx.signature) {
				continue;
			}

			let encoded = match encode_selector {
				EncodeSelector::None => None,
				EncodeSelector::Call => Some((&tx.tx_encoded[tx.call_start_pos..]).to_string()),
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

			if let TransactionFilterOptions::TxIndex(list) = &tx_filter {
				if found_extrinsics.len() >= list.len() {
					break;
				}
			}

			if let TransactionFilterOptions::TxHash(list) = &tx_filter {
				if found_extrinsics.len() >= list.len() {
					break;
				}
			}
		}

		cache.promote_block(block_hash);

		Ok(found_extrinsics)
	}
}

pub mod fetch_events_v1 {
	pub use frame_system_rpc_runtime_api::system_events_api::fetch_events_v1::{
		GroupedRuntimeEvents as RuntimeGroupedRuntimeEvents, Options,
		RuntimeEvent as RuntimeRuntimeEvent,
	};
	pub type ApiResult = Vec<GroupedRuntimeEvents>;

	#[allow(dead_code)]
	#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
	#[cfg_attr(feature = "ts", ts(export, export_to = "FetchEvents.ts"))]
	struct ApiResultTS(pub Vec<GroupedRuntimeEvents>);

	#[allow(dead_code)]
	#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
	#[cfg_attr(feature = "ts", ts(export, export_to = "FetchEvents.ts"))]
	struct ApiParamsTS((String, Option<Options>));

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
	#[cfg_attr(feature = "ts", ts(export, export_to = "FetchEvents.ts"))]
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
	#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
	#[cfg_attr(feature = "ts", ts(export, export_to = "FetchEvents.ts"))]
	pub struct RuntimeEvent {
		pub index: u32,
		// (Pallet Id, Event Id)
		pub emitted_index: (u8, u8),
		pub encoded: Option<String>,
		pub decoded: Option<String>,
	}

	impl From<RuntimeRuntimeEvent> for RuntimeEvent {
		fn from(value: RuntimeRuntimeEvent) -> Self {
			Self {
				index: value.index,
				emitted_index: value.emitted_index,
				encoded: value.encoded.map(const_hex::encode),
				decoded: value.decoded.map(const_hex::encode),
			}
		}
	}
}

pub mod fetch_extrinsics_v1 {
	use super::*;
	use avail_core::asdr::EXTRINSIC_FORMAT_VERSION;
	use codec::{Decode, Input};
	use da_runtime::{Address, Signature, SignedExtra};
	use serde::{Deserialize, Serialize};
	use sp_runtime::MultiAddress;
	type SignaturePayload = (Address, Signature, SignedExtra);

	pub type ApiResult = Vec<ExtrinsicInformation>;

	#[allow(dead_code)]
	#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
	#[cfg_attr(feature = "ts", ts(export, export_to = "FetchExtrinsics.ts"))]
	struct ApiResultTS(pub Vec<ExtrinsicInformation>);

	#[allow(dead_code)]
	#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
	#[cfg_attr(feature = "ts", ts(export, export_to = "FetchExtrinsics.ts"))]
	struct ApiParamsTS(pub (BlockId, Option<Options>));

	#[derive(Clone, Serialize, Deserialize)]
	#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
	#[cfg_attr(feature = "ts", ts(export, export_to = "FetchExtrinsics.ts"))]
	pub struct ExtrinsicInformation {
		pub encoded: Option<String>,
		#[cfg_attr(feature = "ts", ts(as = "String"))]
		pub tx_hash: H256,
		pub tx_index: u32,
		pub pallet_id: u8,
		pub call_id: u8,
		pub signature: Option<TransactionSignature>,
	}

	#[derive(Clone, Copy, Serialize, Deserialize)]
	#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
	#[cfg_attr(feature = "ts", ts(export, export_to = "FetchExtrinsics.ts"))]
	pub enum BlockId {
		/// Identify by block header hash.
		#[cfg_attr(feature = "ts", ts(as = "String"))]
		Hash(H256),
		/// Identify by block number.
		Number(u32),
	}

	#[derive(Default, Clone, Serialize, Deserialize)]
	#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
	#[cfg_attr(feature = "ts", ts(export, export_to = "FetchExtrinsics.ts"))]
	pub struct Options {
		pub filter: Option<Filter>,
		pub encode_selector: Option<EncodeSelector>,
	}

	#[derive(Clone, Serialize, Deserialize)]
	#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
	#[cfg_attr(feature = "ts", ts(export, export_to = "FetchExtrinsics.ts"))]
	#[repr(u8)]
	pub enum EncodeSelector {
		None = 0,
		Call = 1,
		Extrinsic = 2,
	}

	impl EncodeSelector {
		pub fn is_call(&self) -> bool {
			match self {
				EncodeSelector::Call => true,
				_ => false,
			}
		}

		pub fn is_extrinsic(&self) -> bool {
			match self {
				EncodeSelector::Extrinsic => true,
				_ => false,
			}
		}
	}

	impl Default for EncodeSelector {
		fn default() -> Self {
			Self::Extrinsic
		}
	}

	#[derive(Default, Clone, Serialize, Deserialize)]
	#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
	#[cfg_attr(feature = "ts", ts(export, export_to = "FetchExtrinsics.ts"))]
	pub struct Filter {
		pub transaction: Option<TransactionFilterOptions>,
		pub signature: Option<SignatureFilterOptions>,
	}

	#[derive(Clone, Serialize, Deserialize)]
	#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
	#[cfg_attr(feature = "ts", ts(export, export_to = "FetchExtrinsics.ts"))]
	pub enum TransactionFilterOptions {
		All,
		#[cfg_attr(feature = "ts", ts(as = "Vec<String>"))]
		TxHash(Vec<H256>),
		TxIndex(Vec<u32>),
		Pallet(Vec<u8>),
		PalletCall(Vec<(u8, u8)>),
	}

	impl TransactionFilterOptions {
		pub fn is_valid(&self) -> bool {
			match self {
				TransactionFilterOptions::All => true,
				TransactionFilterOptions::TxHash(items) => items.len() < 30,
				TransactionFilterOptions::TxIndex(items) => items.len() < 30,
				TransactionFilterOptions::Pallet(items) => items.len() < 30,
				TransactionFilterOptions::PalletCall(items) => items.len() < 30,
			}
		}

		pub fn is_tx_hash(&self) -> bool {
			match self {
				Self::TxHash(_) => true,
				_ => false,
			}
		}

		pub fn filter_in_pallet(&self, value: u8) -> bool {
			let TransactionFilterOptions::Pallet(list) = self else {
				return true;
			};
			list.contains(&value)
		}

		pub fn filter_in_pallet_call(&self, value: (u8, u8)) -> bool {
			let TransactionFilterOptions::PalletCall(list) = self else {
				return true;
			};
			list.contains(&value)
		}

		pub fn filter_in_tx_hash(&self, value: H256) -> bool {
			let TransactionFilterOptions::TxHash(list) = self else {
				return true;
			};
			list.contains(&value)
		}

		pub fn filter_in_tx_index(&self, value: u32) -> bool {
			let TransactionFilterOptions::TxIndex(list) = self else {
				return true;
			};
			list.contains(&value)
		}
	}

	impl Default for TransactionFilterOptions {
		fn default() -> Self {
			Self::All
		}
	}

	#[derive(Default, Clone, Serialize, Deserialize)]
	#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
	#[cfg_attr(feature = "ts", ts(export, export_to = "FetchExtrinsics.ts"))]
	pub struct SignatureFilterOptions {
		pub ss58_address: Option<String>,
		pub app_id: Option<u32>,
		pub nonce: Option<u32>,
	}

	impl SignatureFilterOptions {
		pub fn is_valid(&self) -> bool {
			if self.ss58_address.as_ref().is_some_and(|x| x.len() > 100) {
				return false;
			}

			true
		}
		pub fn filter_in(&self, signature: &Option<TransactionSignature>) -> bool {
			if !self.filter_in_app_id(signature.as_ref().map(|x| x.app_id)) {
				return false;
			}

			if !self.filter_in_ss58_address(
				signature.as_ref().map(|x| x.ss58_address.clone()).flatten(),
			) {
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

		pub fn filter_in_app_id(&self, value: Option<u32>) -> bool {
			if self.app_id.is_none() {
				return true;
			}
			self.app_id == value
		}

		pub fn filter_in_nonce(&self, value: Option<u32>) -> bool {
			if self.nonce.is_none() {
				return true;
			}
			self.nonce == value
		}
	}

	#[derive(Clone, Serialize, Deserialize)]
	#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
	#[cfg_attr(feature = "ts", ts(export, export_to = "FetchExtrinsics.ts"))]
	pub struct TransactionSignature {
		pub ss58_address: Option<String>,
		pub nonce: u32,
		pub app_id: u32,
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
			let app_id = sig.2 .8 .0 .0;
			let mortality = match sig.2 .4 .0 {
				sp_runtime::generic::Era::Immortal => None,
				sp_runtime::generic::Era::Mortal(x, y) => Some((x, y)),
			};

			let value = Self {
				ss58_address,
				nonce,
				app_id,
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
			if self.blocks.len() >= self.max_size as usize && self.blocks.len() > 0 {
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
	use super::fetch_extrinsics_v1::*;
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
