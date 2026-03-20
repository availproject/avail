use codec::{Compact, Decode, Encode};
use da_runtime::{AccountId, Preamble};
use fetch_events::AllowedEvents;
use fetch_extrinsics::{AllowedExtrinsic, DataFormat, Extrinsic, Extrinsics, Query};
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
use sp_runtime::OpaqueExtrinsic;
use sp_runtime::{
	traits::{Block as BlockT, Header as HeaderT},
	MultiAddress,
};
use std::{marker::PhantomData, sync::Arc};

#[rpc(client, server)]
pub trait Api {
	#[method(name = "custom_events")]
	async fn events(
		&self,
		at: types::BlockId,
		allow_list: AllowedEvents,
		fetch_data: bool,
	) -> RpcResult<fetch_events::Events>;

	#[method(name = "custom_extrinsics")]
	async fn extrinsics(
		&self,
		at: types::BlockId,
		allow_list: Option<Vec<AllowedExtrinsic>>,
		query: Query,
		data_format: DataFormat,
	) -> RpcResult<Extrinsics>;

	#[method(name = "custom_chainInfo")]
	async fn chain_info(&self) -> RpcResult<types::ChainInfo>;

	#[method(name = "custom_blockNumber")]
	async fn block_number(&self, hash: H256) -> RpcResult<Option<u32>>;

	#[method(name = "custom_blockTimestamp")]
	async fn block_timestamp(&self, block_id: types::BlockId) -> RpcResult<u64>;
}

pub struct Rpc<C, Block>
where
	C: ProvideRuntimeApi<Block> + Send + Sync + 'static,
	C::Api: frame_system_rpc_runtime_api::SystemEventsApi<Block>,
	Block: BlockT,
{
	pub client: Arc<C>,
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

	pub fn other_error<'a>(msg: String) -> ErrorObject<'a> {
		ErrorObject::owned(i32::from(Self::Other), msg, None::<()>)
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
	async fn events(
		&self,
		at: types::BlockId,
		filter: fetch_events::AllowedEvents,
		fetch_data: bool,
	) -> RpcResult<fetch_events::Events> {
		use fetch_events::PhaseEvents;

		let block_hash = block_hash_from_block_id(&self.client, at)
			.map_err(|e| Error::NoBlockFound.into_error_object(e))?;

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

	async fn extrinsics(
		&self,
		at: types::BlockId,
		allow_list: Option<Vec<AllowedExtrinsic>>,
		query: Query,
		data_format: DataFormat,
	) -> RpcResult<Extrinsics> {
		const MAX_INDICES_COUNT: usize = 30;

		if let Some(allow_list) = &allow_list {
			if allow_list.len() > MAX_INDICES_COUNT {
				return Err(Error::InvalidInput.into_error_object(String::from(
					"Allow list: Invalid input. Cannot have more than 30 items",
				)));
			}
		}

		let block_hash = block_hash_from_block_id(&self.client, at)
			.map_err(|e| Error::NoBlockFound.into_error_object(e))?;

		let block_body = self
			.client
			.block_body(block_hash.into())
			.map_err(|e| Error::other_error(e.to_string()))?;

		let Some(block_body) = block_body else {
			return Ok(Vec::new());
		};

		let limit = query.max_items.unwrap_or(u32::MAX) as usize;
		if limit == 0 {
			return Ok(Vec::new());
		}

		let mut extrinsics_to_return = Vec::new();
		let body_iter: Box<dyn Iterator<Item = (usize, OpaqueExtrinsic)>> = if query.reverse {
			Box::new(block_body.into_iter().enumerate().rev())
		} else {
			Box::new(block_body.into_iter().enumerate())
		};

		let allow_list = AllowedExtrinsicsWrapper::new(allow_list);
		for (ext_index, opaque) in body_iter {
			let mut wrapper = ExtrinsicWrapper::new(opaque, ext_index as u32);

			if !allow_list.index_allowed(wrapper.ext_index) {
				continue;
			}

			let ext_hash = wrapper.hash().map_err(|e| Error::other_error(e))?;
			if !allow_list.hash_allowed(ext_hash) {
				continue;
			}

			let ext_call_id = wrapper.call_id().map_err(|e| Error::other_error(e))?;
			if !allow_list.pallet_allowed(ext_call_id.0) {
				continue;
			}
			if !allow_list.call_allowed(&ext_call_id) {
				continue;
			}

			let account_id = wrapper.account_id().map_err(|e| Error::other_error(e))?;
			let nonce = wrapper.nonce().map_err(|e| Error::other_error(e))?;
			if let Some(allowed_address) = &query.address {
				let Some(account) = account_id.as_ref() else {
					continue;
				};
				let address = std::format!("{}", account);
				if allowed_address.as_str() != address.as_str() {
					continue;
				}
			}

			if let Some(allowed_nonce) = query.nonce {
				if Some(allowed_nonce) != nonce {
					continue;
				}
			}

			let transparent = wrapper.transparent().map_err(|e| Error::other_error(e))?;
			let data = match data_format {
				DataFormat::None => String::new(),
				DataFormat::Call => {
					const_hex::encode(&transparent.bytes[transparent.call_start_pos..])
				},
				DataFormat::Extrinsic => const_hex::encode(&transparent.bytes),
			};

			let ext = Extrinsic {
				data,
				ext_hash,
				ext_index: ext_index as u32,
				pallet_id: transparent.pallet_id,
				variant_id: transparent.variant_id,
				account_id,
				nonce,
			};

			extrinsics_to_return.push(ext);
			if extrinsics_to_return.len() >= limit {
				break;
			}
		}

		Ok(extrinsics_to_return)
	}

	async fn chain_info(&self) -> RpcResult<types::ChainInfo> {
		let info = self.client.info();
		return Ok(types::ChainInfo {
			best_hash: info.best_hash.into(),
			best_height: info.best_number.into(),
			finalized_hash: info.finalized_hash.into(),
			finalized_height: info.finalized_number.into(),
			genesis_hash: info.genesis_hash.into(),
		});
	}

	async fn block_number(&self, hash: H256) -> RpcResult<Option<u32>> {
		let result = self
			.client
			.block_number_from_id(&sp_runtime::generic::BlockId::Hash(hash.into()))
			.map_err(|err| Error::Other.into_error_object(err.to_string()))?;
		Ok(result.map(|x| x.into()))
	}

	async fn block_timestamp(&self, at: types::BlockId) -> RpcResult<u64> {
		const TIMESTAMP_SET_PALLET_ID: u8 = 3;
		const TIMESTAMP_SET_VARIANT_ID: u8 = 0;

		let block_hash = block_hash_from_block_id(&self.client, at)
			.map_err(|e| Error::NoBlockFound.into_error_object(e))?;

		let block_body = self
			.client
			.block_body(block_hash.into())
			.map_err(|e| Error::other_error(e.to_string()))?;

		let Some(block_body) = block_body else {
			return Ok(0);
		};

		let Some(opaque) = block_body.get(0) else {
			return Ok(0);
		};

		let transparent =
			TransparentOpaque::from_opaque(opaque).map_err(|e| Error::other_error(e))?;

		if (transparent.pallet_id != TIMESTAMP_SET_PALLET_ID)
			|| (transparent.variant_id != TIMESTAMP_SET_VARIANT_ID)
		{
			return Ok(0);
		}

		let Ok(timestamp) =
			Compact::<u64>::decode(&mut &transparent.bytes[transparent.call_start_pos + 2..])
		else {
			return Ok(0);
		};

		Ok(timestamp.0)
	}
}

fn block_hash_from_block_id<C, Block>(client: &Arc<C>, at: types::BlockId) -> Result<H256, String>
where
	C: ProvideRuntimeApi<Block> + Send + Sync + 'static,
	C: BlockBackend<Block>,
	C::Api: frame_system_rpc_runtime_api::SystemEventsApi<Block>,
	Block: BlockT,
	<Block as BlockT>::Hash: From<H256> + Into<H256>,
{
	let block_hash = match at {
		types::BlockId::Hash(h) => h,
		types::BlockId::Number(n) => {
			let hash = match client.block_hash(n.into()) {
				Ok(ok) => ok,
				Err(err) => return Err(err.to_string()),
			};
			let Some(hash) = hash else {
				return Err(String::from("Failed to find block hash"));
			};
			hash.into()
		},
	};

	Ok(block_hash)
}

#[derive(Default)]
struct AllowedExtrinsicsWrapper {
	allowed_indices: Option<Vec<u32>>,
	allowed_hashes: Option<Vec<H256>>,
	allowed_pallets: Option<Vec<u8>>,
	allowed_calls: Option<Vec<(u8, u8)>>,
}

impl AllowedExtrinsicsWrapper {
	pub fn new(list: Option<Vec<AllowedExtrinsic>>) -> Self {
		let Some(list) = list else {
			return Self::default();
		};

		let mut allowed_indices: Option<Vec<u32>> = None;
		let mut allowed_hashes: Option<Vec<H256>> = None;
		let mut allowed_pallets: Option<Vec<u8>> = None;
		let mut allowed_calls: Option<Vec<(u8, u8)>> = None;

		for allowed in list {
			match allowed {
				AllowedExtrinsic::TxHash(x) => {
					if let Some(hashes) = allowed_hashes.as_mut() {
						hashes.push(x);
					} else {
						allowed_hashes = Some(vec![x])
					}
				},
				AllowedExtrinsic::TxIndex(x) => {
					if let Some(items) = allowed_indices.as_mut() {
						items.push(x);
					} else {
						allowed_indices = Some(vec![x])
					}
				},
				AllowedExtrinsic::Pallet(x) => {
					if let Some(items) = allowed_pallets.as_mut() {
						items.push(x);
					} else {
						allowed_pallets = Some(vec![x])
					}
				},
				AllowedExtrinsic::PalletCall(x) => {
					if let Some(items) = allowed_calls.as_mut() {
						items.push(x);
					} else {
						allowed_calls = Some(vec![x])
					}
				},
			}
		}

		Self {
			allowed_indices,
			allowed_hashes,
			allowed_pallets,
			allowed_calls,
		}
	}

	pub fn index_allowed(&self, index: u32) -> bool {
		let Some(list) = self.allowed_indices.as_ref() else {
			return true;
		};
		return list.contains(&index);
	}

	pub fn hash_allowed(&self, hash: H256) -> bool {
		let Some(list) = self.allowed_hashes.as_ref() else {
			return true;
		};
		return list.contains(&hash);
	}

	pub fn pallet_allowed(&self, pallet: u8) -> bool {
		let Some(list) = self.allowed_pallets.as_ref() else {
			return true;
		};
		return list.contains(&pallet);
	}

	pub fn call_allowed(&self, call: &(u8, u8)) -> bool {
		let Some(list) = self.allowed_calls.as_ref() else {
			return true;
		};
		return list.contains(call);
	}
}

struct ExtrinsicWrapper {
	pub opaque: OpaqueExtrinsic,
	pub transparent: Option<TransparentOpaque>,
	pub ext_index: u32,
}

impl ExtrinsicWrapper {
	pub fn new(opaque: OpaqueExtrinsic, ext_index: u32) -> Self {
		Self {
			opaque,
			transparent: None,
			ext_index,
		}
	}

	pub fn hash(&mut self) -> Result<H256, String> {
		let transparent = self.transparent()?;
		let hash = Blake2Hasher::hash(&transparent.bytes);
		return Ok(hash);
	}

	pub fn call_id(&mut self) -> Result<(u8, u8), String> {
		let transparent = self.transparent()?;
		return Ok((transparent.pallet_id, transparent.variant_id));
	}

	pub fn nonce(&mut self) -> Result<Option<u32>, String> {
		let transparent = self.transparent()?;
		if let Preamble::Signed(_, _, extended) = &transparent.preamble {
			return Ok(Some(extended.5 .0));
		}

		Ok(None)
	}

	pub fn account_id(&mut self) -> Result<Option<AccountId>, String> {
		let transparent = self.transparent()?;
		if let Preamble::Signed(address, _, _) = &transparent.preamble {
			if let MultiAddress::Id(id) = address {
				return Ok(Some(id.clone()));
			}

			if let MultiAddress::Address32(id) = address {
				return Ok(Some(AccountId::from(id.clone())));
			}
		}

		Ok(None)
	}

	fn transparent(&mut self) -> Result<&TransparentOpaque, String> {
		if self.transparent.is_none() {
			let transparent = TransparentOpaque::from_opaque(&self.opaque)?;
			self.transparent = Some(transparent);
		}

		// Will never fail
		self.transparent
			.as_ref()
			.ok_or(String::from("Failed to find transparent extrinsic."))
	}
}

impl ExtrinsicWrapper {}

struct TransparentOpaque {
	pub bytes: Vec<u8>,
	pub call_start_pos: usize,
	pub pallet_id: u8,
	pub variant_id: u8,
	pub preamble: Preamble,
}

impl TransparentOpaque {
	pub fn from_opaque(opaque: &OpaqueExtrinsic) -> Result<TransparentOpaque, String> {
		let bytes = opaque.encode();
		let mut iter = bytes.as_slice();
		let _ = match Compact::<u32>::decode(&mut iter) {
			Ok(x) => x,
			Err(e) => {
				return Err(e.to_string());
			},
		};
		let preamble = match Preamble::decode(&mut iter) {
			Ok(p) => p,
			Err(e) => {
				return Err(e.to_string());
			},
		};

		let call_start_pos = bytes.len().saturating_sub(iter.len());
		let pallet_id = *bytes
			.get(call_start_pos)
			.ok_or(String::from("Invalid extrinsic found."))?;
		let variant_id = *bytes
			.get(call_start_pos + 1)
			.ok_or(String::from("Invalid extrinsic found."))?;

		let res = TransparentOpaque {
			bytes,
			call_start_pos,
			pallet_id,
			variant_id,
			preamble,
		};
		Ok(res)
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
		AllowedEvents, PhaseEvents as RuntimeGroupedRuntimeEvents,
		RuntimeEvent as RuntimeRuntimeEvent,
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
		pub pallet_id: u8,
		pub variant_id: u8,
		pub data: String,
	}

	impl From<RuntimeRuntimeEvent> for RuntimeEvent {
		fn from(value: RuntimeRuntimeEvent) -> Self {
			Self {
				index: value.index,
				pallet_id: value.pallet_id,
				variant_id: value.variant_id,
				data: const_hex::encode(value.data),
			}
		}
	}
}

pub mod fetch_extrinsics {
	use super::*;

	use da_runtime::AccountId;
	use serde::{Deserialize, Serialize};

	pub type Extrinsics = Vec<Extrinsic>;

	#[derive(Clone, Serialize, Deserialize)]
	pub struct Extrinsic {
		pub data: String,
		pub ext_hash: H256,
		pub ext_index: u32,
		pub pallet_id: u8,
		pub variant_id: u8,
		pub account_id: Option<AccountId>,
		pub nonce: Option<u32>,
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
	pub struct Query {
		/// SS58 address
		pub address: Option<String>,
		/// Nonce
		pub nonce: Option<u32>,
		/// If set to Some(1) it will return only the first occurrence of a extrinsic that match.
		pub max_items: Option<u32>,
		/// If true, it will traverse the extrinsic list from end to start.
		pub reverse: bool,
	}
}
