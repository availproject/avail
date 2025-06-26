use avail_core::OpaqueExtrinsic;
use codec::Encode;
use da_runtime::UncheckedExtrinsic;
use frame_system_rpc_runtime_api::SystemEventsApi;
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::ErrorObject,
};
use sc_client_api::BlockBackend;
use sp_api::ProvideRuntimeApi;
use sp_core::{Blake2Hasher, Hasher, H256};
use sp_runtime::traits::{Block as BlockT, BlockIdTo, Header as HeaderT};
use std::{
	marker::PhantomData,
	sync::{Arc, Mutex},
};

#[rpc(client, server)]
pub trait Api {
	#[method(name = "system_fetchEventsV1")]
	async fn fetch_events_v1(
		&self,
		params: fetch_events_v1::Params,
		at: H256,
	) -> RpcResult<fetch_events_v1::ApiResult>;

	#[method(name = "system_fetchExtrinsicsV1")]
	async fn fetch_extrinsics_v1(
		&self,
		params: fetch_extrinsics_v1::Params,
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
			block_cache: Arc::new(Mutex::new(fetch_extrinsics_v1::Cache::new(3))),
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
	C: BlockIdTo<Block>,
	C::Api: frame_system_rpc_runtime_api::SystemEventsApi<Block>,
	Block: BlockT<Extrinsic = OpaqueExtrinsic>,
	<Block as BlockT>::Hash: From<H256> + Into<H256>,
	<<Block as BlockT>::Header as HeaderT>::Number: From<u32>,
{
	async fn fetch_events_v1(
		&self,
		params: fetch_events_v1::Params,
		at: H256,
	) -> RpcResult<fetch_events_v1::ApiResult> {
		use fetch_events_v1::GroupedRuntimeEvents;

		let runtime_api = self.client.runtime_api();
		let result = runtime_api
			.fetch_events_v1(at.into(), params)
			.map_err(|x| Error::RuntimeApi.into_error_object(x.to_string()))?;

		match result {
			Ok(res) => Ok(res.into_iter().map(GroupedRuntimeEvents::from).collect()),
			Err(code) => Err(Error::InvalidInput
				.into_error_object(std::format!("Runtime Api Error Code: {code}"))),
		}
	}

	async fn fetch_extrinsics_v1(
		&self,
		params: fetch_extrinsics_v1::Params,
	) -> RpcResult<fetch_extrinsics_v1::ApiResult> {
		use fetch_extrinsics_v1::{
			BlockId, EncodeSelector, ExtCached, ExtrinsicInformation, TransactionFilterOptions,
			TransactionSignature, UncheckedCached,
		};
		let filter = params.filter.unwrap_or_default();
		let tx_filter = filter.transaction.unwrap_or_default();
		let sig_filter = filter.signature.unwrap_or_default();
		let encode_selector = params.encode_selector.unwrap_or_default();

		if !tx_filter.is_valid() {
			return Err(Error::InvalidInput
				.into_error_object(String::from("Transaction filter: Invalid input")));
		}

		if !sig_filter.is_valid() {
			return Err(Error::InvalidInput
				.into_error_object(String::from("Signature filter: Invalid input")));
		}

		let block_hash = match params.block_id {
			BlockId::Hash(h) => h,
			BlockId::Number(n) => {
				let n = &sp_runtime::generic::BlockId::Number(n.into());
				let hash = match self.client.to_hash(n) {
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

		let mut found_extrinsics = Vec::new();
		let block = match self.client.block(block_hash.into()) {
			Ok(x) => x,
			Err(err) => return Err(Error::NoBlockFound.into_error_object(err.to_string())),
		};
		let Some(block) = block else {
			return Err(Error::NoBlockFound.into_error_object(String::from("No block found")));
		};

		let Ok(mut cache) = self.block_cache.lock() else {
			return Err(Error::Other.into_error_object(String::from("failed to lock mutex")));
		};
		let cached_block = cache.block_mut(block_hash);
		let extrinsics = block.block.extrinsics();
		for (i, ext) in extrinsics.iter().enumerate() {
			if !tx_filter.filter_in_tx_index(i as u32) {
				continue;
			}

			let cached_extrinsic = cached_block.extrinsic_mut(i as u32);
			let ext_cache: &ExtCached = if let Some(x) = cached_extrinsic.ext_cached.as_ref() {
				x
			} else {
				let ext_encoded = {
					let mut encoded: Vec<u8> = Vec::with_capacity(ext.0.len() + 4);
					codec::Compact::<u32>(ext.0.len() as u32).encode_to(&mut encoded);
					encoded.extend_from_slice(&ext.0);
					encoded
				};

				let tx_hash = Blake2Hasher::hash(&ext_encoded);
				let ext_encoded = hex_encode(ext_encoded);
				cached_extrinsic.ext_cached = Some(ExtCached::new(ext_encoded, tx_hash));
				cached_extrinsic.ext_cached.as_ref().expect("Just added it")
			};

			let tx_hash = ext_cache.tx_hash.clone();
			if !tx_filter.filter_in_tx_hash(tx_hash) {
				continue;
			}

			let unchecked_cache: &UncheckedCached = {
				if let Some(cached) = cached_extrinsic.unchecked_cached.as_ref() {
					cached
				} else {
					let Ok(uxt) = UncheckedExtrinsic::decode_no_vec_prefix(&mut ext.0.as_slice())
					else {
						continue;
					};
					let call_encoded = uxt.function.encode();
					if call_encoded.len() < 2 {
						continue;
					}
					let dispatch_index = (call_encoded[0], call_encoded[1]);
					let signature = TransactionSignature::from_unchecked(&uxt);
					let call_encoded = hex_encode(call_encoded);

					cached_extrinsic.unchecked_cached = Some(UncheckedCached::new(
						signature.clone(),
						dispatch_index,
						call_encoded,
					));
					cached_extrinsic
						.unchecked_cached
						.as_ref()
						.expect("Just added it")
				}
			};

			let signature = &unchecked_cache.signature;
			let dispatch_index = unchecked_cache.dispatch_index;

			if !sig_filter.filter_in(signature) {
				continue;
			}

			if !tx_filter.filter_in_pallet(dispatch_index.0) {
				continue;
			}

			if !tx_filter.filter_in_pallet_call(dispatch_index) {
				continue;
			}

			let encoded = match encode_selector {
				EncodeSelector::None => None,
				EncodeSelector::Call => Some(unchecked_cache.call_encoded.clone()),
				EncodeSelector::Extrinsic => Some(ext_cache.ext_encoded.clone()),
			};

			let ext_info = ExtrinsicInformation {
				encoded,
				tx_hash,
				tx_index: i as u32,
				pallet_id: dispatch_index.0,
				call_id: dispatch_index.1,
				signature: signature.clone(),
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

		Ok(found_extrinsics)
	}
}

pub mod fetch_events_v1 {
	use super::hex_encode;
	pub use frame_system_rpc_runtime_api::system_events_api::fetch_events_v1::{
		GroupedRuntimeEvents as RuntimeGroupedRuntimeEvents, Params,
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
		pub encoded: Option<String>,
		pub decoded: Option<String>,
	}

	impl From<RuntimeRuntimeEvent> for RuntimeEvent {
		fn from(value: RuntimeRuntimeEvent) -> Self {
			Self {
				index: value.index,
				emitted_index: value.emitted_index,
				encoded: value.encoded.map(hex_encode),
				decoded: value.decoded.map(hex_encode),
			}
		}
	}
}

// Efficient way to concatenate two strings.
// `hex::encode_to_slice` returns a valid utf8 string so `from_utf8_unchecked` will always work
fn hex_encode(input: Vec<u8>) -> String {
	let mut encoded: Vec<u8> = vec![0u8; input.len() * 2 + 2];
	encoded[0] = b'0';
	encoded[1] = b'x';

	if encoded[2..].len() >= input.len() * 2 {
		hex::encode_to_slice(input, &mut encoded[2..])
			.expect("Made sure that encoded has enough space");
	}

	unsafe { String::from_utf8_unchecked(encoded) }
}

pub mod fetch_extrinsics_v1 {
	use super::*;
	use serde::{Deserialize, Serialize};
	use sp_runtime::MultiAddress;

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
		pub filter: Option<Filter>,
		pub encode_selector: Option<EncodeSelector>,
	}

	#[derive(Clone, Serialize, Deserialize)]
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
	pub struct Filter {
		pub transaction: Option<TransactionFilterOptions>,
		pub signature: Option<SignatureFilterOptions>,
	}

	#[derive(Clone, Serialize, Deserialize)]
	pub enum TransactionFilterOptions {
		All,
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
	pub struct TransactionSignature {
		pub ss58_address: Option<String>,
		pub nonce: u32,
		pub app_id: u32,
		pub mortality: Option<(u64, u64)>,
	}

	impl TransactionSignature {
		pub fn from_unchecked(ext: &UncheckedExtrinsic) -> Option<Self> {
			let Some(sig) = &ext.signature else {
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

	pub struct UncheckedCached {
		pub signature: Option<TransactionSignature>,
		pub dispatch_index: (u8, u8),
		// Hex string encoded with 0x
		pub call_encoded: String,
	}

	impl UncheckedCached {
		pub fn new(
			signature: Option<TransactionSignature>,
			dispatch_index: (u8, u8),
			call_encoded: String,
		) -> Self {
			Self {
				signature,
				dispatch_index,
				call_encoded,
			}
		}
	}

	pub struct ExtCached {
		// Hex string encoded with 0x
		pub ext_encoded: String,
		pub tx_hash: H256,
	}

	impl ExtCached {
		pub fn new(ext_encoded: String, tx_hash: H256) -> Self {
			Self {
				ext_encoded,
				tx_hash,
			}
		}
	}

	#[derive(Default)]
	pub struct CachedExtrinsic {
		pub unchecked_cached: Option<UncheckedCached>,
		pub ext_cached: Option<ExtCached>,
	}

	pub struct CachedBlock {
		extrinsics: Vec<(u32, CachedExtrinsic)>,
	}

	impl CachedBlock {
		pub fn new() -> Self {
			Self {
				extrinsics: Vec::new(),
			}
		}

		pub fn extrinsic_mut(&mut self, index: u32) -> &mut CachedExtrinsic {
			let pos = self.extrinsics.iter().position(|x| x.0 == index);
			if let Some(pos) = pos {
				return &mut self.extrinsics[pos].1;
			}

			self.extrinsics.push((index, CachedExtrinsic::default()));
			&mut self.extrinsics.last_mut().expect("Just added it").1
		}
	}

	pub struct Cache {
		blocks: Vec<(H256, CachedBlock)>,
		max_size: u32,
	}

	impl Cache {
		pub fn new(max_size: u32) -> Self {
			Self {
				blocks: Vec::new(),
				max_size,
			}
		}

		pub fn block_mut(&mut self, block_hash: H256) -> &mut CachedBlock {
			let pos = self.blocks.iter().position(|x| x.0 == block_hash);
			if let Some(pos) = pos {
				return &mut self.blocks[pos].1;
			}

			if self.blocks.len() >= self.max_size as usize && self.blocks.len() > 0 {
				self.blocks.remove(0);
			}
			self.blocks.push((block_hash, CachedBlock::new()));
			&mut self.blocks.last_mut().expect("Just added it").1
		}
	}
}
