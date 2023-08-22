use core::num::NonZeroU16;
use std::{
	marker::Sync,
	sync::{Arc, RwLock},
};

use avail_base::metrics::RPCMetricAdapter;
use avail_core::{
	header::HeaderExtension, traits::ExtendedHeader, AppExtrinsic, AppId, BlockLengthColumns,
	BlockLengthRows, DataProof, OpaqueExtrinsic, BLOCK_CHUNK_SIZE,
};
use codec::{Decode, Encode, Input};
use da_runtime::{apis::DataAvailApi, Address, Runtime, Signature, SignedExtra};
use frame_system::{
	limits::BlockLength,
	submitted_data::{self},
};
use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
};
use kate::{
	com::Cell,
	config::{COL_EXTENSION, ROW_EXTENSION},
	BlockDimensions, BlsScalar, PublicParameters, Seed,
};
use kate_recovery::matrix::Dimensions;
use lru::LruCache;
use nalgebra::DMatrix;
use sc_client_api::BlockBackend;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::{BlockId, Digest},
	traits::{Block as BlockT, Header},
};

pub type HashOf<Block> = <Block as BlockT>::Hash;

#[rpc(client, server)]
pub trait KateApi<Block>
where
	Block: BlockT,
{
	#[method(name = "kate_queryRows")]
	async fn query_rows(
		&self,
		rows: Vec<u32>,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<Vec<u8>>>;

	#[method(name = "kate_queryAppData")]
	async fn query_app_data(
		&self,
		app_id: AppId,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<Option<Vec<u8>>>>;

	#[method(name = "kate_queryProof")]
	async fn query_proof(&self, cells: Vec<Cell>, at: Option<HashOf<Block>>) -> RpcResult<Vec<u8>>;

	#[method(name = "kate_blockLength")]
	async fn query_block_length(&self, at: Option<HashOf<Block>>) -> RpcResult<BlockLength>;

	#[method(name = "kate_queryDataProof")]
	async fn query_data_proof(
		&self,
		data_index: u32,
		at: Option<HashOf<Block>>,
	) -> RpcResult<DataProof>;
}

#[allow(clippy::type_complexity)]
pub struct Kate<Client, Block: BlockT> {
	client: Arc<Client>,
	block_ext_cache: RwLock<LruCache<Block::Hash, DMatrix<BlsScalar>>>,
}

impl<Client, Block: BlockT> Kate<Client, Block> {
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
		JsonRpseeError::Custom(format!($($arg)*))
	}}
}

/// If feature `secure_padding_fill` is enabled then the returned seed is generated using Babe VRF.
/// Otherwise, it will use the default `Seed` value.
fn get_seed<B, C>(client: &C, block_id: &BlockId<B>) -> Option<Seed>
where
	B: BlockT,
	<B as BlockT>::Header: ExtendedHeader<
		<<B as BlockT>::Header as Header>::Number,
		<B as BlockT>::Hash,
		Digest,
		HeaderExtension,
	>,
	C: ProvideRuntimeApi<B>,
	C::Api: DataAvailApi<B>,
{
	if cfg!(feature = "secure_padding_fill") {
		client.runtime_api().babe_vrf(block_id).ok()
	} else {
		Some(Seed::default())
	}
}

impl<Client, Block> Kate<Client, Block>
where
	Block: BlockT,
	Client: Send + Sync + 'static,
	Client: HeaderBackend<Block> + ProvideRuntimeApi<Block> + BlockBackend<Block>,
	Client::Api: DataAvailApi<Block>,
{
	fn at_or_best(&self, at: Option<<Block as BlockT>::Hash>) -> <Block as BlockT>::Hash {
		at.unwrap_or_else(|| self.client.info().best_hash)
	}

	#[inline]
	fn block_id(&self, at: Option<<Block as BlockT>::Hash>) -> BlockId<Block> {
		BlockId::Hash(self.at_or_best(at))
	}
}

/// Unused function to demonstrate how to get only extract the app_id from an opaque extrinsic
#[allow(dead_code)]
fn app_id_from_opaque(opaque: &OpaqueExtrinsic) -> Result<AppId, String> {
	let input = &mut opaque.0.as_slice();
	let version = input.read_byte().unwrap();

	let is_signed = version & 0b1000_0000 != 0;
	let version = version & 0b0111_1111;
	if version != 4 {
		return Err("Invalid transaction version".to_string());
	}

	let signature: Option<(Address, Signature, SignedExtra)> = is_signed
		.then(|| Decode::decode(input))
		.transpose()
		.map_err(|e| format!("{e:?}"))?;

	signature
		.map(|(_address, _signature, extra)| extra.8 .0)
		.ok_or("Not signed".to_string())
}

#[async_trait]
impl<Client, Block> KateApiServer<Block> for Kate<Client, Block>
where
	Block: BlockT<Extrinsic = OpaqueExtrinsic>,
	<Block as BlockT>::Header: ExtendedHeader<
		<<Block as BlockT>::Header as Header>::Number,
		<Block as BlockT>::Hash,
		Digest,
		HeaderExtension,
	>,
	Client: Send + Sync + 'static,
	Client: HeaderBackend<Block> + ProvideRuntimeApi<Block> + BlockBackend<Block>,
	Client::Api: DataAvailApi<Block>,
{
	async fn query_rows(
		&self,
		rows: Vec<u32>,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<Vec<u8>>> {
		let at = self.at_or_best(at);
		let block_id = BlockId::Hash(at);

		let signed_block = self
			.client
			.block(at)
			.map_err(|e| internal_err!("Invalid block number: {:?}", e))?
			.ok_or_else(|| internal_err!("Missing block {}", block_id))?;

		let block_hash = signed_block.block.header().hash();

		if self.client.info().finalized_number < *signed_block.block.header().number() {
			return Err(internal_err!(
				"Requested block {block_hash} is not finalized"
			));
		}

		let mut block_ext_cache = self
			.block_ext_cache
			.write()
			.map_err(|_| internal_err!("Block cache lock is poisoned .qed"))?;

		if !block_ext_cache.contains(&block_hash) {
			// build block data extension and cache it
			let xts_by_id: Vec<AppExtrinsic> = signed_block
				.block
				.extrinsics()
				.iter()
				.map(|e| {
					let appid = submitted_data::extract_app_id::<Runtime>(e).unwrap_or(AppId(0));
					AppExtrinsic {
						app_id: appid,
						data: Encode::encode(&e.0),
					}
				})
				.collect();

			let seed = get_seed::<Block, Client>(&self.client, &block_id)
				.ok_or_else(|| internal_err!("Babe VRF not found for block {}", block_id))?;

			let block_length: BlockLength = self
				.client
				.runtime_api()
				.block_length(&block_id)
				.map_err(|e| internal_err!("Block Length cannot be fetched: {:?}", e))?;

			let (_, block, block_dims) = kate::com::flatten_and_pad_block(
				block_length.rows,
				block_length.cols,
				block_length.chunk_size(),
				&xts_by_id,
				seed,
			)
			.map_err(|e| internal_err!("Flatten and pad block failed: {:?}", e))?;

			let metrics = RPCMetricAdapter {};
			let data = kate::com::par_extend_data_matrix(block_dims, &block, &metrics)
				.map_err(|e| internal_err!("Matrix cannot be extended: {:?}", e))?;

			block_ext_cache.put(block_hash, data);
		}

		let ext_data = block_ext_cache
			.get(&block_hash)
			.ok_or_else(|| internal_err!("Block hash {} cannot be fetched", block_hash))?;

		Ok(kate::com::scalars_to_rows(&rows, ext_data))
	}

	async fn query_app_data(
		&self,
		app_id: AppId,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<Option<Vec<u8>>>> {
		let at = self.at_or_best(at);
		let block_id = BlockId::Hash(at);

		let signed_block = self
			.client
			.block(at)
			.map_err(|e| internal_err!("Invalid block number: {:?}", e))?
			.ok_or_else(|| internal_err!("Missing block {}", block_id))?;

		let block_hash = signed_block.block.header().hash();

		if self.client.info().finalized_number < *signed_block.block.header().number() {
			return Err(internal_err!(
				"Requested block {block_hash} is not finalized"
			));
		}

		let mut block_ext_cache = self
			.block_ext_cache
			.write()
			.map_err(|_| internal_err!("Block cache lock is poisoned .qed"))?;

		if !block_ext_cache.contains(&block_hash) {
			// build block data extension and cache it
			let xts_by_id: Vec<AppExtrinsic> = signed_block
				.block
				.extrinsics()
				.iter()
				.map(|e| {
					let appid = submitted_data::extract_app_id::<Runtime>(e).unwrap_or(AppId(0));
					AppExtrinsic {
						app_id: appid,
						data: Encode::encode(&e.0),
					}
				})
				.collect();

			let block_length: BlockLength = self
				.client
				.runtime_api()
				.block_length(&block_id)
				.map_err(|e| internal_err!("Block Length cannot be fetched: {:?}", e))?;

			let seed = get_seed::<Block, Client>(&self.client, &block_id)
				.ok_or_else(|| internal_err!("Babe VRF not found for block {block_id}"))?;

			let (_, block, block_dims) = kate::com::flatten_and_pad_block(
				block_length.rows,
				block_length.cols,
				block_length.chunk_size(),
				&xts_by_id,
				seed,
			)
			.map_err(|e| internal_err!("Flatten and pad block failed: {:?}", e))?;

			let metrics = RPCMetricAdapter {};
			let data = kate::com::par_extend_data_matrix(block_dims, &block, &metrics)
				.map_err(|e| internal_err!("Matrix cannot be extended: {:?}", e))?;

			block_ext_cache.put(block_hash, data);
		}

		let ext_data = block_ext_cache
			.get(&block_hash)
			.ok_or_else(|| internal_err!("Block hash {} cannot be fetched", block_hash))?;

		let app_data_index = signed_block.block.header().extension().app_lookup();
		let dimensions = non_extended_dimensions(ext_data)?;

		Ok(kate::com::scalars_to_app_rows(
			app_id,
			app_data_index,
			dimensions,
			ext_data,
		))
	}

	//TODO allocate static thread pool, just for RPC related work, to free up resources, for the block producing processes.
	async fn query_proof(&self, cells: Vec<Cell>, at: Option<HashOf<Block>>) -> RpcResult<Vec<u8>> {
		let at = self.at_or_best(at);
		let block_id = BlockId::Hash(at);

		let signed_block = self
			.client
			.block(at)
			.map_err(|e| internal_err!("Invalid block number: {:?}", e))?
			.ok_or_else(|| internal_err!("Missing block {}", block_id))?;

		let block_hash = signed_block.block.header().hash();

		if self.client.info().finalized_number < *signed_block.block.header().number() {
			return Err(internal_err!(
				"Requested block {block_hash} is not finalized"
			));
		}

		let mut block_ext_cache = self
			.block_ext_cache
			.write()
			.map_err(|_| internal_err!("Block cache lock is poisoned .qed"))?;
		let metrics = RPCMetricAdapter {};

		if !block_ext_cache.contains(&block_hash) {
			// build block data extension and cache it
			let xts_by_id: Vec<AppExtrinsic> = signed_block
				.block
				.extrinsics()
				.iter()
				.map(|e| {
					let appid = submitted_data::extract_app_id::<Runtime>(e).unwrap_or(AppId(0));
					AppExtrinsic {
						app_id: appid,
						data: Encode::encode(&e.0),
					}
				})
				.collect();

			let block_length: BlockLength = self
				.client
				.runtime_api()
				.block_length(&block_id)
				.map_err(|e| internal_err!("Block Length cannot be fetched: {:?}", e))?;

			let seed = get_seed::<Block, Client>(&self.client, &block_id)
				.ok_or_else(|| internal_err!("Babe VRF not found for block {block_id}"))?;

			let (_, block, block_dims) = kate::com::flatten_and_pad_block(
				block_length.rows,
				block_length.cols,
				block_length.chunk_size(),
				&xts_by_id,
				seed,
			)
			.map_err(|e| internal_err!("Flatten and pad block failed: {:?}", e))?;

			let data = kate::com::par_extend_data_matrix(block_dims, &block, &metrics)
				.map_err(|e| internal_err!("Matrix cannot be extended: {:?}", e))?;
			block_ext_cache.put(block_hash, data);
		}

		let ext_data = block_ext_cache
			.get(&block_hash)
			.ok_or_else(|| internal_err!("Block hash {} cannot be fetched", block_hash))?;
		let dimensions = non_extended_dimensions(ext_data)?;
		let block_dims = BlockDimensions::new(
			BlockLengthRows(dimensions.rows().get().into()),
			BlockLengthColumns(dimensions.cols().get().into()),
			BLOCK_CHUNK_SIZE,
		)
		.ok_or_else(|| internal_err!("Block dimensions are invalid"))?;

		let kc_public_params_raw =
			self.client
				.runtime_api()
				.public_params(&block_id)
				.map_err(|e| {
					internal_err!(
						"Public params cannot be fetched on block {}: {:?}",
						block_hash,
						e
					)
				})?;
		let kc_public_params =
			unsafe { PublicParameters::from_slice_unchecked(&kc_public_params_raw) };

		let proof =
			kate::com::build_proof(&kc_public_params, block_dims, ext_data, &cells, &metrics)
				.map_err(|e| internal_err!("Proof cannot be generated: {:?}", e))?;

		Ok(proof)
	}

	async fn query_block_length(&self, at: Option<HashOf<Block>>) -> RpcResult<BlockLength> {
		let block_id = self.block_id(at);
		let block_length = self
			.client
			.runtime_api()
			.block_length(&block_id)
			.map_err(|e| internal_err!("Length of best block({:?}): {:?}", block_id, e))?;

		Ok(block_length)
	}

	async fn query_data_proof(
		&self,
		data_index: u32,
		at: Option<HashOf<Block>>,
	) -> RpcResult<DataProof> {
		// Fetch block
		let at = self.at_or_best(at);

		let block = self
			.client
			.block(at)
			.map_err(|e| internal_err!("Invalid block hash: {:?}", e))?
			.ok_or_else(|| internal_err!("Missing block hash {:?}", at))?
			.block;

		// Build the proof.
		let merkle_proof =
			submitted_data::extrinsics_proof::<Runtime, _>(block.extrinsics().iter(), data_index)
				.ok_or_else(|| {
				internal_err!(
					"Data proof cannot be generated for index={} at block {:?}",
					data_index,
					at
				)
			})?;

		DataProof::try_from(&merkle_proof)
			.map_err(|e| internal_err!("Data proof cannot be loaded from merkle root: {:?}", e))
	}
}

fn non_extended_dimensions(ext_data: &DMatrix<BlsScalar>) -> RpcResult<Dimensions> {
	// Dimension of no extended matrix.
	let (ext_rows, ext_cols) = ext_data.shape();
	let rows = ext_rows
		.checked_div(NonZeroU16::get(ROW_EXTENSION).into())
		.ok_or_else(|| internal_err!("Invalid row extension"))?;
	let cols = ext_cols
		.checked_div(NonZeroU16::get(COL_EXTENSION).into())
		.ok_or_else(|| internal_err!("Invalid col extension"))?;
	let dimensions =
		Dimensions::new_from(rows, cols).ok_or_else(|| internal_err!("Invalid dimensions"))?;

	Ok(dimensions)
}
