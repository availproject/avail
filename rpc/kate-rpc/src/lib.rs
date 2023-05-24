#![deny(unused_crate_dependencies)]
use std::{num::NonZeroUsize, sync::Arc, vec};

use avail_base::metrics::RPCMetricAdapter;
use da_primitives::{
	asdr::{AppExtrinsic, AppId},
	traits::ExtendedHeader,
	DataProof,
};
use da_runtime::{Runtime, UncheckedExtrinsic};
use frame_system::{limits::BlockLength, submitted_data};
use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
};
use kate::{
	com::Cell,
	grid::{Dimensions, Grid as GridTrait},
	gridgen::{AsBytes, EvaluationGrid, PolynomialGrid},
	pmp::m1_blst,
};
use kate_rpc_runtime_api::KateParamsGetter;
use moka::sync::Cache;
use sc_client_api::BlockBackend;
use serde::{Deserialize, Serialize};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::{BlockId, SignedBlock},
	traits::{Block as BlockT, Extrinsic, Header},
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
	) -> RpcResult<Vec<Option<Vec<u8>>>>;

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

	/// Query the multiproof for a given cell. The cells must be within the multiproof grid given
	/// by `kate::gridgen::multiproof_dims`. This returns a JSON of the following format:
	/// ```json
	/// [{
	///    "proof": "0x...",
	///    "evals": "0x...",
	/// }..]
	/// ```
	///
	/// The `proof` key contains the serialized multiproof, and the `evals` key contains the
	/// scalars in the chunk of the base grid for the given cell of the multiproof grid, stored in
	/// row-wise order.
	///
	/// The size of `evals` will correspond to the result of `kate::gridgen::multiproof_block`.
	#[method(name = "kate_queryMultiProof")]
	async fn query_multiproof(
		&self,
		cells: Vec<Cell>,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<MultiproofSer>>;

	#[method(name = "kate_queryDataProof")]
	async fn query_data_proof(
		&self,
		data_index: u32,
		at: Option<HashOf<Block>>,
	) -> RpcResult<DataProof>;
}

struct Grid {
	evals: EvaluationGrid,
	polys: PolynomialGrid,
}
pub struct Kate<Client, Block: BlockT> {
	client: Arc<Client>,
	block_ext_cache: Cache<Block::Hash, Arc<Grid>>,
	multiproof_srs: m1_blst::M1NoPrecomp,
}

impl<Client, Block> Kate<Client, Block>
where
	Block: BlockT,
{
	pub fn new(client: Arc<Client>) -> Self {
		const GB: u64 = 2u64.pow(30);
		Self {
			client,
			block_ext_cache: Cache::<_, Arc<Grid>>::builder()
				.weigher(|_, v| (v.evals.dims.n_cells() * 2 * 32) as u32)
				.thread_pool_enabled(false) // TODO: decide if this should be true
				.max_capacity(GB)
				.build(),
			multiproof_srs: kate::testnet::multiproof_params(256, 256),
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

impl<Client, Block> Kate<Client, Block>
where
	Block: BlockT,
	Block::Header: ExtendedHeader,
	Client: Send + Sync + 'static,
	Client: HeaderBackend<Block> + ProvideRuntimeApi<Block> + BlockBackend<Block>,
	Client::Api: KateParamsGetter<Block>,
	UncheckedExtrinsic: TryFrom<<Block as BlockT>::Extrinsic>,
{
	fn at_or_best(&self, at: Option<<Block as BlockT>::Hash>) -> <Block as BlockT>::Hash {
		at.unwrap_or_else(|| self.client.info().best_hash)
	}

	#[inline]
	fn block_id(&self, at: Option<<Block as BlockT>::Hash>) -> BlockId<Block> {
		BlockId::Hash(self.at_or_best(at))
	}

	fn get_grid(&self, signed_block: &SignedBlock<Block>) -> RpcResult<Arc<Grid>> {
		let block_hash = signed_block.block.hash();
		let block_id = BlockId::Hash(block_hash.clone());

		let block_length: BlockLength = self
			.client
			.runtime_api()
			.get_block_length(&block_id)
			.map_err(|e| internal_err!("Block Length cannot be fetched: {:?}", e))?;

		self.block_ext_cache
			.try_get_with(block_hash, || {
				use kate::metrics::Metrics; // TODO: Just do this in the RPC
				let metrics = RPCMetricAdapter {};
				// build block data extension and cache it
				let t1 = std::time::Instant::now();
				let xts_by_id: Vec<AppExtrinsic> = signed_block
					.block
					.extrinsics()
					.iter()
					.cloned()
					.filter_map(|opaque| UncheckedExtrinsic::try_from(opaque).ok())
					.map(AppExtrinsic::from)
					.collect();

				// Use Babe's VRF
				let seed: [u8; 32] =
					self.client
						.runtime_api()
						.get_babe_vrf(&block_id)
						.map_err(|e| {
							internal_err!("Babe VRF not found for block {}: {:?}", block_id, e)
						})?;

				let mut evals = kate::gridgen::EvaluationGrid::from_extrinsics(
					xts_by_id.clone(),
					4,
					block_length.cols.as_usize(), // 'cols' is the # of cols, so width
					block_length.rows.as_usize(), // 'rows' is the # of rows, so height
					seed.clone(),
				)
				.map_err(|e| internal_err!("Building evals grid failed: {:?}", e))?;

				let t2 = std::time::Instant::now();
				metrics.preparation_block_time(t2 - t1);

				evals = evals
					.extend_columns(2)
					.map_err(|e| internal_err!("Error extending grid {:?}", e))?;

				let t3 = std::time::Instant::now();
				metrics.extended_block_time(t3 - t2);

				let polys = evals
					.make_polynomial_grid()
					.map_err(|e| internal_err!("Error getting polynomial grid {:?}", e))?;

				Ok::<_, JsonRpseeError>(Arc::new(Grid { evals, polys }))
			})
			.map_err(|e: Arc<_>| internal_err!("failed to construct block: {}", e)) // Deref the arc into a reference, clone the ref
	}

	fn get_signed_block(&self, at: Option<Block::Hash>) -> RpcResult<SignedBlock<Block>> {
		let at = self.at_or_best(at);
		self.client
			.block(at)
			.map_err(|e| internal_err!("Invalid block number: {:?}", e))?
			.ok_or_else(|| internal_err!("Missing block {}", at))
	}
}

// TODO: figure out what to actually do here
use once_cell::sync::Lazy;
static PMP: Lazy<kate::pmp::m1_blst::M1NoPrecomp> =
	once_cell::sync::Lazy::new(|| kate::testnet::multiproof_params(256, 256));

#[async_trait]
impl<Client, Block> KateApiServer<Block> for Kate<Client, Block>
where
	Block: BlockT,
	Block::Header: ExtendedHeader,
	Client: Send + Sync + 'static,
	Client: HeaderBackend<Block> + ProvideRuntimeApi<Block> + BlockBackend<Block>,
	Client::Api: KateParamsGetter<Block>,
	<<Block as BlockT>::Extrinsic as Extrinsic>::Call: Clone,
	UncheckedExtrinsic: TryFrom<<Block as BlockT>::Extrinsic>,
{
	async fn query_rows(
		&self,
		rows: Vec<u32>,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<Option<Vec<u8>>>> {
		let signed_block = self.get_signed_block(at)?;
		let block_hash = signed_block.block.header().hash();

		if self.client.info().finalized_number < *signed_block.block.header().number() {
			return Err(internal_err!(
				"Requested block {block_hash} is not finalized"
			));
		}

		let grid = self.get_grid(&signed_block)?;

		let mut all_rows = vec![None; grid.evals.dims.height()];
		rows.iter()
			.map(|y| (*y as usize, grid.evals.row(*y as usize)))
			.for_each(|(y, row)| match row {
				Some(row) => {
					let row_bytes = row
						.iter()
						.flat_map(|s| s.to_bytes().expect("Ser cannot fail"))
						.collect();
					all_rows[y as usize] = Some(row_bytes)
				},
				_ => (),
			});
		Ok(all_rows)
	}

	async fn query_app_data(
		&self,
		app_id: AppId,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<Option<Vec<u8>>>> {
		let signed_block = self.get_signed_block(at)?;
		let block_hash = signed_block.block.header().hash();
		let grid = self.get_grid(&signed_block)?;

		if self.client.info().finalized_number < *signed_block.block.header().number() {
			return Err(internal_err!(
				"Requested block {block_hash} is not finalized"
			));
		}

		let orig_height = NonZeroUsize::new(grid.evals.dims.height() / 2).ok_or(internal_err!(
			"Extended grid has height 1? This should never happen"
		))?;
		let orig_dims = Dimensions::new(grid.evals.dims.width_nz(), orig_height);

		let rows = grid
			.evals
			.app_rows(&app_id, Some(&orig_dims))
			.unwrap_or(vec![]);
		let mut all_rows = vec![None; orig_dims.height()];
		for (row_y, row) in rows {
			all_rows[row_y] = Some(
				row.into_iter()
					.flat_map(|s| s.to_bytes().expect("Ser cannot fail"))
					.collect::<Vec<u8>>(),
			);
		}

		Ok(all_rows)
	}

	//TODO allocate static thread pool, just for RPC related work, to free up resources, for the block producing processes.
	async fn query_proof(&self, cells: Vec<Cell>, at: Option<HashOf<Block>>) -> RpcResult<Vec<u8>> {
		let signed_block = self.get_signed_block(at)?;
		let block_hash = signed_block.block.header().hash();

		if self.client.info().finalized_number < *signed_block.block.header().number() {
			return Err(internal_err!(
				"Requested block {block_hash} is not finalized"
			));
		}

		let grid = self.get_grid(&signed_block)?;

		let proof = cells
			.iter()
			.map(|cell| {
				grid.evals
					.evals
					.get(cell.col.as_usize(), cell.row.as_usize())
					.ok_or(internal_err!(
						"Invalid cell {:?} for dims {:?}",
						cell,
						grid.evals.dims
					))
					.and_then(|data| {
						grid.polys
							.proof(&*PMP, cell)
							.map_err(|e| internal_err!("Unable to make proof: {:?}", e))
							.map(|proof| {
								(
									data.to_bytes().expect("Ser cannot fail"),
									proof.to_bytes().expect("Ser cannot fail"),
								)
							})
					})
			})
			.collect::<Result<Vec<_>, _>>()?
			.into_iter()
			.flat_map(|(data, proof)| [proof.to_vec(), data.to_vec()])
			.collect::<Vec<_>>()
			.concat();

		Ok(proof)
	}

	async fn query_block_length(&self, at: Option<HashOf<Block>>) -> RpcResult<BlockLength> {
		let block_id = self.block_id(at);
		let block_length = self
			.client
			.runtime_api()
			.get_block_length(&block_id)
			.map_err(|e| internal_err!("Length of best block({:?}): {:?}", block_id, e))?;

		Ok(block_length)
	}

	async fn query_data_proof(
		&self,
		data_index: u32,
		at: Option<HashOf<Block>>,
	) -> RpcResult<DataProof> {
		// Fetch block
		let block = self.get_signed_block(at)?.block;

		// Get Opaque Extrinsics and transform into AppUncheckedExt.
		let calls = block
			.extrinsics()
			.iter()
			.cloned()
			.filter_map(|opaque| UncheckedExtrinsic::try_from(opaque).ok())
			.map(|app_ext| app_ext.function);

		// Build the proof.
		let merkle_proof = submitted_data::calls_proof::<Runtime, _, _>(calls, data_index)
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

	async fn query_multiproof(
		&self,
		cells: Vec<Cell>,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<MultiproofSer>> {
		let block = self.get_signed_block(at)?;
		let grid = self.get_grid(&block)?;

		let target_dims = Dimensions::new(
			NonZeroUsize::new(16).expect("16>0"),
			NonZeroUsize::new(64).expect("64>0"),
		);
		let multiproofs = cells
			.iter()
			.map(|cell| {
				grid.polys
					.multiproof(&self.multiproof_srs, cell, &grid.evals, &target_dims)
					.map_err(|e| internal_err!("Error building multiproof {:?}", e))
			})
			.collect::<Result<Vec<_>, _>>()?;

		use kate::pmp::traits::AsBytes;
		Ok(multiproofs
			.iter()
			.map(|mp| {
				let evals = mp
					.evals
					.iter()
					.flat_map(|s| s)
					.flat_map(|c| c.to_bytes().unwrap())
					.collect::<Vec<u8>>();
				let proof: Vec<u8> = mp.proof.to_bytes().unwrap().into();
				MultiproofSer { proof, evals }
			})
			.collect::<Vec<_>>())
	}
}

#[derive(Serialize, Deserialize)]
pub struct MultiproofSer {
	pub proof: Vec<u8>,
	pub evals: Vec<u8>,
}
