#![deny(unused_crate_dependencies)]
use core::num::NonZeroU16;
use std::{marker::Sync, sync::Arc};

use avail_base::metrics::RPCMetricAdapter;
use avail_core::{
	header::HeaderExtension, traits::ExtendedHeader, AppExtrinsic, AppId, DataProof,
	OpaqueExtrinsic,
};
use da_runtime::{apis::DataAvailApi, Runtime, UncheckedExtrinsic};
use frame_system::{limits::BlockLength, submitted_data};
use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
};
use kate::{
	com::Cell,
	config::{COL_EXTENSION, ROW_EXTENSION},
	gridgen::{AsBytes, EvaluationGrid, PolynomialGrid},
	pmp::m1_blst,
	Seed,
};
use kate_recovery::matrix::Dimensions;
use moka::future::Cache;
use rayon::prelude::*;
use sc_client_api::BlockBackend;
use serde::{Deserialize, Serialize};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::{Digest, SignedBlock},
	traits::{Block as BlockT, Header},
};

pub type HashOf<Block> = <Block as BlockT>::Hash;
#[derive(Serialize, Deserialize)]
pub struct MultiproofSer {
	pub proof: Vec<u8>,
	pub evals: Vec<u8>,
}

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
	#[method(name = "kate_queryMultiproof")]
	async fn query_multiproof(
		&self,
		cells: Vec<Cell>,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<MultiproofSer>>;

	#[method(name = "kate_queryDataProof")]
	async fn query_data_proof(
		&self,
		transaction_index: u32,
		at: Option<HashOf<Block>>,
	) -> RpcResult<DataProof>;
}

#[allow(clippy::type_complexity)]
pub struct Kate<Client, Block: BlockT> {
	client: Arc<Client>,
	eval_grid_cache: Cache<Block::Hash, Arc<EvaluationGrid>>,
	// Have to put dimensions here b/c it's not public in polynomialgrid
	poly_grid_cache: Cache<Block::Hash, Arc<(Dimensions, PolynomialGrid)>>,
	multiproof_srs: m1_blst::M1NoPrecomp,
}

impl<Client, Block: BlockT> Kate<Client, Block> {
	pub fn new(client: Arc<Client>) -> Self {
		const GB: u64 = 2u64.pow(30);
		Self {
			client,
			eval_grid_cache: Cache::<_, Arc<EvaluationGrid>>::builder()
				.weigher(|_, v| {
					let n_cells: u32 = v.dims().size();
					n_cells * 32 + 8
				})
				.max_capacity(GB)
				.build(),
			poly_grid_cache: Cache::<_, Arc<(Dimensions, PolynomialGrid)>>::builder()
				.weigher(|_, v| {
					let n_cells: u32 = v.0.size();
					let n_points: u32 =
						v.0.width().try_into().expect("Never more than 2^32 points");
					n_cells * 32 + n_points * 32
				})
				.max_capacity(GB)
				.build(),
			multiproof_srs: kate::testnet::multiproof_params(256, 256), // TODO: pull this from the
			                                                            // system
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
	Client: Send + Sync + 'static,
	Client: HeaderBackend<Block> + ProvideRuntimeApi<Block> + BlockBackend<Block>,
	Client::Api: DataAvailApi<Block>,
	UncheckedExtrinsic: TryFrom<<Block as BlockT>::Extrinsic>,
{
	fn at_or_best(&self, at: Option<<Block as BlockT>::Hash>) -> <Block as BlockT>::Hash {
		at.unwrap_or_else(|| self.client.info().best_hash)
	}

	fn is_block_finalized(&self, block: &SignedBlock<Block>) -> Result<(), JsonRpseeError> {
		let block_header = block.block.header();
		let (block_hash, block_number) = (block_header.hash(), *block_header.number());

		if self.client.info().finalized_number < block_number {
			return Err(internal_err!(
				"Requested block {block_hash} is not finalized"
			));
		}

		Ok(())
	}

	fn get_signed_block(&self, at: Option<Block::Hash>) -> RpcResult<SignedBlock<Block>> {
		let at = self.at_or_best(at);
		self.client
			.block(at)
			.map_err(|e| internal_err!("Invalid block number: {:?}", e))?
			.ok_or_else(|| internal_err!("Missing block {}", at))
	}

	fn get_signed_and_finalized_block(
		&self,
		at: Option<Block::Hash>,
	) -> RpcResult<SignedBlock<Block>> {
		let signed_block = self.get_signed_block(at)?;
		self.is_block_finalized(&signed_block)?;
		Ok(signed_block)
	}

	/// If feature `secure_padding_fill` is enabled then the returned seed is generated using Babe VRF.
	/// Otherwise, it will use the default `Seed` value.
	fn get_seed(&self, at: Block::Hash) -> RpcResult<Seed> {
		if cfg!(feature = "secure_padding_fill") {
			self.client
				.runtime_api()
				.babe_vrf(at)
				.map_err(|e| internal_err!("Babe VRF not found for block {}: {:?}", at, e))
		} else {
			Ok(Seed::default())
		}
	}

	/// The signed_block needs to be finalized.
	async fn get_eval_grid(
		&self,
		signed_block: &SignedBlock<Block>,
	) -> RpcResult<Arc<EvaluationGrid>> {
		let block_hash = signed_block.block.header().hash();

		self.eval_grid_cache
			.try_get_with(block_hash, async move {
				use kate::metrics::Metrics; // TODO: Rework this for the correct metrics
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
				let seed = self.get_seed(block_hash)?;
				let block_length: BlockLength = self
					.client
					.runtime_api()
					.block_length(block_hash)
					.map_err(|e| internal_err!("Block Length cannot be fetched: {:?}", e))?;

				let mut evals = kate::gridgen::EvaluationGrid::from_extrinsics(
					xts_by_id.clone(),
					4,
					block_length.cols.0.try_into().expect("TODO"), // 'cols' is the # of cols, so width
					block_length.rows.0.try_into().expect("TODO"), // 'rows' is the # of rows, so height
					seed,
				)
				.map_err(|e| internal_err!("Building evals grid failed: {:?}", e))?;

				let t2 = std::time::Instant::now();
				metrics.preparation_block_time(t2 - t1);

				evals = evals
					.extend_columns(NonZeroU16::new(2).expect("2>0"))
					.map_err(|e| internal_err!("Error extending grid {:?}", e))?;

				let t3 = std::time::Instant::now();
				metrics.extended_block_time(t3 - t2);

				Ok::<_, JsonRpseeError>(Arc::new(evals))
			})
			.await
			.map_err(|e: Arc<_>| internal_err!("failed to construct block: {}", e)) // Deref the arc into a reference, clone the ref
	}

	// TODO: We should probably have a metrics item for this
	async fn get_poly_grid(
		&self,
		signed_block: &SignedBlock<Block>,
	) -> RpcResult<Arc<(Dimensions, PolynomialGrid)>> {
		let block_hash = signed_block.block.header().hash();
		self.poly_grid_cache
			.try_get_with(block_hash, async move {
				let evals = self.get_eval_grid(signed_block).await?;
				let polys = evals
					.make_polynomial_grid()
					.map_err(|e| internal_err!("Error getting polynomial grid {:?}", e))?;
				Ok::<_, JsonRpseeError>(Arc::new((evals.dims(), polys)))
			})
			.await
			.map_err(|e: Arc<_>| internal_err!("failed to construct block: {}", e)) // Deref the arc into a reference, clone the ref
	}
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
	) -> RpcResult<Vec<Option<Vec<u8>>>> {
		let signed_block = self.get_signed_and_finalized_block(at)?;
		let evals = self.get_eval_grid(&signed_block).await?;

		let rows: Vec<Option<Vec<u8>>> = rows
			.iter()
			.map(|i| {
				evals.row(*i as usize).map(|row| {
					row.iter()
						.flat_map(|a| a.to_bytes().expect("Ser cannot fail"))
						.collect::<Vec<u8>>()
				})
			})
			.collect();

		Ok(rows)
	}

	async fn query_app_data(
		&self,
		app_id: AppId,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<Option<Vec<u8>>>> {
		let signed_block = self.get_signed_and_finalized_block(at)?;
		let evals = self.get_eval_grid(&signed_block).await?;

		let extended_dims = evals.dims();
		let orig_dims = non_extended_dimensions(extended_dims)?;

		let rows = evals
			.app_rows(app_id, Some(orig_dims))
			.map_err(|e| internal_err!("Failed to get app rows: {:?}", e))?;
		let Some(rows) = rows else {
			return Err(internal_err!("No rows found"));
		};

		let mut div = 1;
		if extended_dims.height() == 2 * orig_dims.height() {
			div = 2;
		}

		let mut all_rows = vec![None; orig_dims.height()];
		for (mut row_y, row) in rows {
			row_y /= div;
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
		let signed_block = self.get_signed_and_finalized_block(at)?;
		let evals = self.get_eval_grid(&signed_block).await?;
		let polys = self.get_poly_grid(&signed_block).await?;

		let proof = cells
			.par_iter()
			.map(|cell| {
				let Ok(row) = usize::try_from(cell.row.0) else {
					return Err(internal_err!("cell row did not fit in usize"));
				};
				let Ok(col) = usize::try_from(cell.col.0) else {
					return Err(internal_err!("cell row did not fit in usize"));
				};
				let Some(data) = evals.get::<usize, usize>(row, col) else {
					let e = internal_err!("Invalid cell {:?} for dims {:?}", cell, evals.dims());
					return Err(e);
				};
				let proof = match polys.1.proof(&self.multiproof_srs, cell) {
					Ok(x) => x,
					Err(e) => return Err(internal_err!("Unable to make proof: {:?}", e)),
				};

				let data = data.to_bytes().expect("Ser cannot fail").to_vec();
				let proof = proof.to_bytes().expect("Ser cannot fail").to_vec();

				Ok([proof, data].into_iter().flatten().collect::<Vec<_>>())
			})
			.collect::<Result<Vec<_>, _>>()?;
		let proof: Vec<u8> = proof.into_iter().flatten().collect();
		Ok(proof)
	}

	async fn query_block_length(&self, at: Option<HashOf<Block>>) -> RpcResult<BlockLength> {
		let at = self.at_or_best(at);
		let api = self.client.runtime_api();
		let block_length = api
			.block_length(at)
			.map_err(|e| internal_err!("Length of best block({:?}): {:?}", at, e))?;

		Ok(block_length)
	}

	async fn query_data_proof(
		&self,
		transaction_index: u32,
		at: Option<HashOf<Block>>,
	) -> RpcResult<DataProof> {
		let block = self.get_signed_block(at)?.block;

		let calls = block
			.extrinsics()
			.iter()
			.flat_map(|extrinsic| UncheckedExtrinsic::try_from(extrinsic).ok())
			.map(|extrinsic| extrinsic.function);

		// Build the proof.
		let merkle_proof = submitted_data::calls_proof::<Runtime, _, _>(calls, transaction_index)
			.ok_or_else(|| {
			internal_err!(
				"Data proof cannot be generated for transaction index={} at block {:?}",
				transaction_index,
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
		let block = self.get_signed_and_finalized_block(at)?;
		let evals = self.get_eval_grid(&block).await?;
		let polys = self.get_poly_grid(&block).await?;

		let target_dims = Dimensions::new(16, 64).expect("16,64>0"); // TODO: make configurable
		let multiproofs = cells
			.iter()
			.map(|cell| {
				polys
					.1
					.multiproof(&self.multiproof_srs, cell, &evals, target_dims)
					.map_err(|e| internal_err!("Error building multiproof {:?}", e))
			})
			.collect::<Result<Vec<_>, _>>()?;

		let multiproof = multiproofs
			.iter()
			.map(|mp| {
				let evals_flatten = mp.evals.iter().flatten();
				let evals: Vec<u8> = evals_flatten.flat_map(|c| c.to_bytes().unwrap()).collect();
				let proof: Vec<u8> = mp.proof.to_bytes().unwrap().into();
				MultiproofSer { proof, evals }
			})
			.collect::<Vec<_>>();

		Ok(multiproof)
	}
}

fn non_extended_dimensions(ext_dims: Dimensions) -> RpcResult<Dimensions> {
	// Dimension of no extended matrix.
	let rows = ext_dims
		.rows()
		.get()
		.checked_div(NonZeroU16::get(ROW_EXTENSION))
		.ok_or_else(|| internal_err!("Invalid row extension"))?;
	let cols = ext_dims
		.cols()
		.get()
		.checked_div(NonZeroU16::get(COL_EXTENSION))
		.ok_or_else(|| internal_err!("Invalid col extension"))?;
	let dimensions =
		Dimensions::new_from(rows, cols).ok_or_else(|| internal_err!("Invalid dimensions"))?;

	Ok(dimensions)
}
