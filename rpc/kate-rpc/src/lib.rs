use std::{
	result::Result as AvailResult,
	sync::{Arc, RwLock},
};

use codec::{Compact, Decode, Encode, Error as DecodeError, Input};
use da_primitives::{
	asdr::{AppExtrinsic, GetAppId},
	traits::ExtendedHeader,
	DataProof,
};
use frame_support::traits::ExtrinsicCall;
use frame_system::{limits::BlockLength, submitted_data};
use jsonrpc_core::{Error as RpcError, Result};
use jsonrpc_derive::rpc;
use kate::{com::Cell, BlockDimensions, BlsScalar, PublicParameters};
use kate_rpc_runtime_api::KateParamsGetter;
use lru::LruCache;
use sc_client_api::{BlockBackend, StorageProvider};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, Extrinsic, Header},
	AccountId32, MultiAddress, MultiSignature,
};

pub type HashOf<Block> = <Block as BlockT>::Hash;
pub type CallOf<Block> = <<Block as BlockT>::Extrinsic as Extrinsic>::Call;

#[rpc]
pub trait KateApi<Block, SDFilter>
where
	Block: BlockT,
	SDFilter: submitted_data::Filter<CallOf<Block>>,
{
	#[rpc(name = "kate_queryProof")]
	fn query_proof(&self, cells: Vec<Cell>, at: Option<HashOf<Block>>) -> Result<Vec<u8>>;

	#[rpc(name = "kate_blockLength")]
	fn query_block_length(&self, at: Option<HashOf<Block>>) -> Result<BlockLength>;

	#[rpc(name = "kate_queryDataProof")]
	fn query_data_proof(&self, data_index: u32, at: Option<HashOf<Block>>) -> Result<DataProof>;
}

pub struct Kate<Client, Block: BlockT> {
	client: Arc<Client>,
	block_ext_cache: RwLock<LruCache<Block::Hash, (Vec<BlsScalar>, BlockDimensions)>>,
}

impl<Client, Block> Kate<Client, Block>
where
	Block: BlockT,
{
	pub fn new(client: Arc<Client>) -> Self {
		Self {
			client,
			block_ext_cache: RwLock::new(LruCache::new(2048)), // 524288 bytes per block, ~1Gb max size
		}
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

macro_rules! internal_err {
	($($arg:tt)*) => {{
		let mut error = RpcError::internal_error();
		error.message = format!($($arg)*);
		error
	}}
}

impl<Client, Block> Kate<Client, Block>
where
	Block: BlockT,
	Block::Extrinsic: GetAppId,
	Block::Header: ExtendedHeader,
	Client: Send + Sync + 'static,
	Client: HeaderBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ StorageProvider<Block, sc_client_db::Backend<Block>>,
	Client::Api: KateParamsGetter<Block>,
{
	fn block_id(&self, at: Option<<Block as BlockT>::Hash>) -> BlockId<Block> {
		let hash = at.unwrap_or_else(|| self.client.info().best_hash);
		BlockId::Hash(hash)
	}
}

impl<Client, Block, SDFilter> KateApi<Block, SDFilter> for Kate<Client, Block>
where
	Block: BlockT,
	Block::Extrinsic: GetAppId + ExtrinsicCall,
	Block::Header: ExtendedHeader,
	Client: Send + Sync + 'static,
	Client: HeaderBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ StorageProvider<Block, sc_client_db::Backend<Block>>,
	Client::Api: KateParamsGetter<Block>,
	SDFilter: submitted_data::Filter<CallOf<Block>>,
	<<Block as BlockT>::Extrinsic as Extrinsic>::Call: Clone,
{
	//TODO allocate static thread pool, just for RPC related work, to free up resources, for the block producing processes.
	fn query_proof(&self, cells: Vec<Cell>, at: Option<HashOf<Block>>) -> Result<Vec<u8>> {
		let block_id = self.block_id(at);

		let signed_block = self
			.client
			.block(&block_id)
			.map_err(|e| internal_err!("Invalid block number: {:?}", e))?
			.ok_or_else(|| internal_err!("Missing block {}", block_id))?;
		let block_hash = signed_block.block.header().hash();

		let mut block_ext_cache = self
			.block_ext_cache
			.write()
			.map_err(|_| internal_err!("Block cache lock is poisoned .qed"))?;

		let block_length: BlockLength = self
			.client
			.runtime_api()
			.get_block_length(&block_id)
			.map_err(|e| internal_err!("Block Length cannot be fetched: {:?}", e))?;

		if !block_ext_cache.contains(&block_hash) {
			// build block data extension and cache it
			let xts_by_id: Vec<AppExtrinsic> = signed_block
				.block
				.extrinsics()
				.iter()
				.map(|e| AppExtrinsic {
					app_id: e.app_id(),
					data: e.encode(),
				})
				.collect();

			// Use Babe's VRF
			let seed: [u8; 32] = self
				.client
				.runtime_api()
				.get_babe_vrf(&block_id)
				.map_err(|e| internal_err!("Babe VRF not found for block {}: {:?}", block_id, e))?;

			let (_, block, block_dims) = kate::com::flatten_and_pad_block(
				block_length.rows as usize,
				block_length.cols as usize,
				block_length.chunk_size() as usize,
				&xts_by_id,
				seed,
			)
			.map_err(|e| internal_err!("Flatten and pad block failed: {:?}", e))?;

			let data = kate::com::par_extend_data_matrix(block_dims, &block)
				.map_err(|e| internal_err!("Matrix cannot be extended: {:?}", e))?;
			block_ext_cache.put(block_hash, (data, block_dims));
		}

		let (ext_data, block_dims) = block_ext_cache
			.get(&block_hash)
			.ok_or_else(|| internal_err!("Block hash {} cannot be fetched", block_hash))?;
		let kc_public_params_raw = self
			.client
			.runtime_api()
			.get_public_params(&block_id)
			.map_err(|e| {
				internal_err!(
					"Public params cannot be fetched on block {}: {:?}",
					block_hash,
					e
				)
			})?;
		let kc_public_params =
			unsafe { PublicParameters::from_slice_unchecked(&kc_public_params_raw) };

		let proof = kate::com::build_proof(&kc_public_params, *block_dims, ext_data, &cells)
			.map_err(|e| internal_err!("Proof cannot be generated: {:?}", e))?;

		Ok(proof)
	}

	fn query_block_length(&self, at: Option<HashOf<Block>>) -> Result<BlockLength> {
		let block_id = self.block_id(at);
		let block_length = self
			.client
			.runtime_api()
			.get_block_length(&block_id)
			.map_err(|e| internal_err!("Length of best block({:?}): {:?}", block_id, e))?;

		Ok(block_length)
	}

	fn query_data_proof(&self, data_index: u32, at: Option<HashOf<Block>>) -> Result<DataProof> {
		// Fetch block
		let block_id = self.block_id(at);
		let block = self
			.client
			.block(&block_id)
			.map_err(|e| internal_err!("Invalid block number: {:?}", e))?
			.ok_or_else(|| internal_err!("Missing block number {:?}", block_id))?
			.block;

		// Get App Extrinsics from the block.
		let calls = block
			.extrinsics()
			.iter()
			.map(|extrinsic| extrinsic.call().clone());

		// Build the proof.
		let merkle_proof = submitted_data::calls_proof::<SDFilter, _, _>(calls, data_index)
			.ok_or_else(|| {
				internal_err!(
					"Data proof cannot be generated for index={} at block {:?}",
					data_index,
					block_id
				)
			})?;
		DataProof::try_from(&merkle_proof)
			.map_err(|e| internal_err!("Data proof cannot be loaded from merkle root: {:?}", e))
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AvailExtrinsic {
	pub app_id: u32,
	pub signature: Option<MultiSignature>,
	pub data: Vec<u8>,
}

pub type AvailSignedExtra = ((), (), (), AvailMortality, Nonce, (), Balance, u32);

#[derive(Decode)]
pub struct Balance(#[codec(compact)] u128);

#[derive(Decode)]
pub struct Nonce(#[codec(compact)] u32);

pub enum AvailMortality {
	Immortal,
	Mortal(u64, u64),
}

impl Decode for AvailMortality {
	fn decode<I: Input>(input: &mut I) -> AvailResult<Self, DecodeError> {
		let first = input.read_byte()?;
		if first == 0 {
			Ok(Self::Immortal)
		} else {
			let encoded = first as u64 + ((input.read_byte()? as u64) << 8);
			let period = 2 << (encoded % (1 << 4));
			let quantize_factor = (period >> 12).max(1);
			let phase = (encoded >> 4) * quantize_factor;
			if period >= 4 && phase < period {
				Ok(Self::Mortal(period, phase))
			} else {
				Err("Invalid period and phase".into())
			}
		}
	}
}

const EXTRINSIC_VERSION: u8 = 4;
impl Decode for AvailExtrinsic {
	fn decode<I: Input>(input: &mut I) -> AvailResult<AvailExtrinsic, DecodeError> {
		// This is a little more complicated than usual since the binary format must be compatible
		// with substrate's generic `Vec<u8>` type. Basically this just means accepting that there
		// will be a prefix of vector length (we don't need
		// to use this).
		let _length_do_not_remove_me_see_above: Compact<u32> = Decode::decode(input)?;

		let version = input.read_byte()?;

		let is_signed = version & 0b1000_0000 != 0;
		let version = version & 0b0111_1111;
		if version != EXTRINSIC_VERSION {
			return Err("Invalid transaction version".into());
		}
		let (app_id, signature) = if is_signed {
			let _address = <MultiAddress<AccountId32, u32>>::decode(input)?;
			let sig = MultiSignature::decode(input)?;
			let extra = <AvailSignedExtra>::decode(input)?;
			let app_id = extra.7;

			(app_id, Some(sig))
		} else {
			return Err("Not signed".into());
		};

		let section: u8 = Decode::decode(input)?;
		let method: u8 = Decode::decode(input)?;

		let data: Vec<u8> = match (section, method) {
			// TODO: Define these pairs as enums or better yet - make a dependency on substrate enums if possible
			(29, 1) => Decode::decode(input)?,
			_ => return Err("Not Avail Extrinsic".into()),
		};

		Ok(Self {
			app_id,
			signature,
			data,
		})
	}
}
