use codec::{Compact, Decode, Encode};
use da_runtime::Preamble;
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

use crate::system::fetch_extrinsics::{AllowedExtrinsic, TransactionSignature};

#[rpc(client, server)]
pub trait Api {
	#[method(name = "custom_events")]
	async fn events(
		&self,
		at: types::BlockId,
		allow_list: fetch_events::AllowedEvents,
		fetch_data: bool,
	) -> RpcResult<fetch_events::Events>;

	#[method(name = "custom_extrinsics")]
	async fn extrinsics(
		&self,
		at: types::BlockId,
		allow_list: Option<Vec<fetch_extrinsics::AllowedExtrinsic>>,
		sig_filter: fetch_extrinsics::SignatureFilter,
		data_format: fetch_extrinsics::DataFormat,
	) -> RpcResult<fetch_extrinsics::Extrinsics>;

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
		block_id: types::BlockId,
		filter: fetch_events::AllowedEvents,
		fetch_data: bool,
	) -> RpcResult<fetch_events::Events> {
		use fetch_events::PhaseEvents;

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

	async fn extrinsics(
		&self,
		at: types::BlockId,
		allow_list: Option<Vec<fetch_extrinsics::AllowedExtrinsic>>,
		sig_filter: fetch_extrinsics::SignatureFilter,
		data_format: fetch_extrinsics::DataFormat,
	) -> RpcResult<fetch_extrinsics::Extrinsics> {
		use fetch_extrinsics::{DataFormat, Extrinsic};
		use types::BlockId;
		const MAX_INDICES_COUNT: usize = 30;

		if let Some(allow_list) = &allow_list {
			if allow_list.len() > MAX_INDICES_COUNT {
				return Err(Error::InvalidInput.into_error_object(String::from(
					"Allow list: Invalid input. Cannot have more than 30 items",
				)));
			}
		}

		let block_hash = match at {
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

		let block_body = match self.client.block_body(block_hash.into()) {
			Ok(x) => x,
			Err(e) => {
				return Err(Error::Other.into_error_object(e.to_string()));
			},
		};

		let Some(block_body) = block_body else {
			return Ok(Vec::new());
		};

		let (allowed_indices, allowed_hashes, allowed_pallets, allowed_calls) =
			allowed_extrinsics_to_parts(allow_list);

		let mut returned_extrinsics = Vec::new();
		for (ext_index, opaque) in block_body.into_iter().enumerate() {
			let ext_index = ext_index as u32;
			// Filter Indices
			if let Some(allowed) = &allowed_indices {
				if !allowed.contains(&ext_index) {
					continue;
				}
			}

			let transparent = TransparentOpaque::from_opaque(&opaque)?;

			let signature = if let Some((address, _, extended)) = transparent.preamble.to_signed() {
				let nonce = extended.5 .0;
				let account_id = match address {
					MultiAddress::Id(id) => Some(id),
					_ => None,
				};
				Some(TransactionSignature { account_id, nonce })
			} else {
				None
			};

			if let Some(allowed_address) = &sig_filter.ss58_address {
				if let Some(account) = signature.as_ref().map(|x| x.account_id.clone()).flatten() {
					let address = std::format!("{}", account);
					if allowed_address.as_str() != address {
						continue;
					}
				} else {
					continue;
				}
			}

			if let Some(allowed_nonce) = &sig_filter.nonce {
				if let Some(nonce) = signature.as_ref().map(|x| x.nonce) {
					if *allowed_nonce != nonce {
						continue;
					}
				} else {
					continue;
				}
			}

			// Filter Pallets
			if let Some(allowed) = &allowed_pallets {
				if !allowed.contains(&transparent.pallet_id) {
					continue;
				}
			}

			// Filter Calls
			if let Some(allowed) = &allowed_calls {
				if !allowed.contains(&(transparent.pallet_id, transparent.variant_id)) {
					continue;
				}
			}

			let ext_hash = Blake2Hasher::hash(&transparent.bytes);

			// Filter Hashes
			if let Some(allowed) = &allowed_hashes {
				if !allowed.contains(&ext_hash) {
					continue;
				}
			}

			let data = match data_format {
				DataFormat::None => String::new(),
				DataFormat::Call => {
					const_hex::encode(&transparent.bytes[transparent.call_start_pos..])
				},
				DataFormat::Extrinsic => const_hex::encode(transparent.bytes),
			};

			let ext = Extrinsic {
				data,
				ext_hash,
				ext_index,
				pallet_id: transparent.pallet_id,
				variant_id: transparent.variant_id,
				signature,
			};

			returned_extrinsics.push(ext);
		}

		Ok(returned_extrinsics)
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

		let block_hash = match at {
			types::BlockId::Hash(h) => h,
			types::BlockId::Number(n) => {
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

		let block_body = match self.client.block_body(block_hash.into()) {
			Ok(x) => x,
			Err(e) => {
				return Err(Error::Other.into_error_object(e.to_string()));
			},
		};

		let Some(block_body) = block_body else {
			return Ok(0);
		};

		let Some(opaque) = block_body.get(0) else {
			return Ok(0);
		};

		let transparent = TransparentOpaque::from_opaque(opaque)?;

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

struct TransparentOpaque {
	pub bytes: Vec<u8>,
	pub call_start_pos: usize,
	pub pallet_id: u8,
	pub variant_id: u8,
	pub preamble: Preamble,
}

impl TransparentOpaque {
	pub fn from_opaque<'a>(opaque: &OpaqueExtrinsic) -> Result<TransparentOpaque, ErrorObject<'a>> {
		let bytes = opaque.encode();
		let mut iter = bytes.as_slice();
		let _ = match Compact::<u32>::decode(&mut iter) {
			Ok(x) => x,
			Err(e) => {
				return Err(Error::Other.into_error_object(e.to_string()));
			},
		};
		let preamble = match Preamble::decode(&mut iter) {
			Ok(p) => p,
			Err(e) => {
				return Err(Error::Other.into_error_object(e.to_string()));
			},
		};

		let call_start_pos = bytes.len().saturating_sub(iter.len());
		let pallet_id = *bytes
			.get(call_start_pos)
			.ok_or(Error::Other.into_error_object(String::from("Invalid extrinsic found.")))?;
		let variant_id = *bytes
			.get(call_start_pos + 1)
			.ok_or(Error::Other.into_error_object(String::from("Invalid extrinsic found.")))?;

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

fn allowed_extrinsics_to_parts(
	list: Option<Vec<AllowedExtrinsic>>,
) -> (
	Option<Vec<u32>>,
	Option<Vec<H256>>,
	Option<Vec<u8>>,
	Option<Vec<(u8, u8)>>,
) {
	let Some(list) = list else {
		return (None, None, None, None);
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

	(
		allowed_indices,
		allowed_hashes,
		allowed_pallets,
		allowed_calls,
	)
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
		pub nonce: Option<u32>,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	pub struct TransactionSignature {
		pub account_id: Option<AccountId>,
		pub nonce: u32,
	}
}
