use avail_base::metrics::avail::{MetricObserver, ObserveKind};
use avail_base::HeaderExtensionBuilderData;
use avail_core::{
	data_proof::ProofResponse, header::HeaderExtension, traits::ExtendedHeader, OpaqueExtrinsic,
};
use da_runtime::{
	apis::{DataAvailApi, KateApi as RTKateApi},
	kate::{GDataProof, GProof, GRawScalar, GRow},
	Runtime,
};
use frame_system::native::hosted_header_builder::MIN_WIDTH;
use kate::{
	com::Cell,
	couscous::multiproof_params,
	gridgen::{AsBytes, EvaluationGrid, PolynomialGrid},
	pmp::m1_blst::M1NoPrecomp,
	Seed,
};
use kate_recovery::matrix::Dimensions;
use moka::future::Cache;

use frame_support::BoundedVec;
use frame_system::limits::BlockLength;
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::{ErrorCode, ErrorObject},
};
use rayon::prelude::*;
use sc_client_api::BlockBackend;
use sp_api::{ApiRef, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::SignedBlock,
	traits::{Block as BlockT, ConstU32, Header},
};
use std::num::NonZeroU16;
use std::{marker::PhantomData, marker::Sync, sync::Arc};

pub type HashOf<Block> = <Block as BlockT>::Hash;
pub type MaxRows = ConstU32<64>;
pub type Rows = BoundedVec<u32, MaxRows>;
pub type MaxCells = ConstU32<10_000>;
pub type Cells = BoundedVec<Cell, MaxCells>;
type RTExtractor = <Runtime as frame_system::Config>::HeaderExtensionDataFilter;

static SRS: std::sync::OnceLock<M1NoPrecomp> = std::sync::OnceLock::new();

pub mod metrics;

#[derive(Clone, Default)]
pub struct Deps {
	/// The maximum number of cells that can be requested in one go.
	pub max_cells_size: usize,
	/// Enable Kate RPCs
	pub rpc_enabled: bool,
	/// Enable Kate RPCs Metrics
	///
	/// Should not be used unless unless you know what you're doing.
	pub rpc_metrics_enabled: bool,
	/// Max size of evaluation grid cache in MiB.
	pub eval_grid_size: u64,
	/// Max size of polynomial grid cache in MiB.
	pub poly_grid_size: u64,
}

/// # TODO
/// - [ ] Update type definitions for RPCs in our subxt & explorer.
#[rpc(client, server)]
pub trait KateApi<Block>
where
	Block: BlockT,
{
	#[method(name = "kate_queryRows")]
	async fn query_rows(&self, rows: Rows, at: Option<HashOf<Block>>) -> RpcResult<Vec<GRow>>;

	#[method(name = "kate_queryProof")]
	async fn query_proof(
		&self,
		cells: Cells,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<GDataProof>>;

	#[method(name = "kate_blockLength")]
	async fn query_block_length(&self, at: Option<HashOf<Block>>) -> RpcResult<BlockLength>;

	#[method(name = "kate_queryDataProof")]
	async fn query_data_proof(
		&self,
		transaction_index: u32,
		at: Option<HashOf<Block>>,
	) -> RpcResult<ProofResponse>;
}

#[allow(clippy::type_complexity)]
pub struct Kate<Client, Block: BlockT> {
	client: Arc<Client>,
	eval_grid_cache: Cache<Block::Hash, Arc<EvaluationGrid>>,
	poly_grid_cache: Cache<Block::Hash, Arc<(Dimensions, PolynomialGrid)>>,
	max_cells_size: usize,
	_block: PhantomData<Block>,
}

impl<Client, Block: BlockT> Kate<Client, Block> {
	pub fn new(
		client: Arc<Client>,
		max_cells_size: usize,
		eval_grid_cache_size: u64,
		poly_grid_cach_size: u64,
	) -> Self {
		// cache sizes are in MiB
		let eval_grid_cache_size = eval_grid_cache_size * 1024 * 1024;
		let poly_grid_cach_size = poly_grid_cach_size * 1024 * 1024;

		Self {
			client,
			eval_grid_cache: Cache::<_, Arc<EvaluationGrid>>::builder()
				.weigher(|_, v| {
					let n_cells: u32 = v.dims().size();
					n_cells * 32 + 8
				})
				.max_capacity(eval_grid_cache_size)
				.build(),
			poly_grid_cache: Cache::<_, Arc<(Dimensions, PolynomialGrid)>>::builder()
				.weigher(|_, v| {
					let n_cells: u32 = v.0.size();
					// currently we support only 2^10 points, and can never have more than 2^32 points
					let n_points: u32 =
						v.0.width().try_into().expect("Never more than 2^32 points");
					n_cells * 32 + n_points * 32
				})
				.max_capacity(poly_grid_cach_size)
				.build(),
			max_cells_size,
			_block: PhantomData,
		}
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	KateRPCError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::KateRPCError => 1,
		}
	}
}

macro_rules! internal_err {
	($($arg:tt)*) => {{
		ErrorObject::owned(
			Error::KateRPCError.into(),
			format!($($arg)*),
			None::<()>
		)
	}}
}

// ApiRef<'_, dyn ApiExt<Block>>,

type Opaques<B> = Vec<<B as BlockT>::Extrinsic>;
type Api<'a, C, B> = ApiRef<'a, <C as ProvideRuntimeApi<B>>::Api>;

impl<Client, Block> Kate<Client, Block>
where
	Block: BlockT<Extrinsic = OpaqueExtrinsic>,
	<Block as BlockT>::Header: ExtendedHeader<Extension = HeaderExtension>,
	Client: Send + Sync + 'static,
	Client: HeaderBackend<Block> + ProvideRuntimeApi<Block> + BlockBackend<Block>,
	Client::Api: DataAvailApi<Block>,
{
	#[allow(clippy::type_complexity)]
	fn scope(
		&self,
		at: Option<Block::Hash>,
	) -> RpcResult<(
		Api<'_, Client, Block>,
		<Block as BlockT>::Hash,
		u32,
		BlockLength,
		Opaques<Block>,
		<Block as BlockT>::Header,
	)> {
		let at = self.at_or_best(at);
		let block = self.get_finalized_block(Some(at))?.block;
		let number: u32 = (*block.header().number())
			.try_into()
			.map_err(|_| ErrorCode::InvalidParams)?;
		let (header, extrinsics) = block.deconstruct();

		let api = self.client.runtime_api();
		let block_len = api
			.block_length(at)
			.map_err(|e| internal_err!("Length of best block({at:?}): {e:?}"))?;

		Ok((api, at, number, block_len, extrinsics, header))
	}

	fn at_or_best(&self, at: Option<<Block as BlockT>::Hash>) -> <Block as BlockT>::Hash {
		at.unwrap_or_else(|| self.client.info().best_hash)
	}

	fn ensure_block_finalized(&self, block: &SignedBlock<Block>) -> RpcResult<()> {
		let block_header = block.block.header();
		let (block_hash, block_number) = (block_header.hash(), *block_header.number());

		if self.client.info().finalized_number < block_number {
			return Err(internal_err!(
				"Requested block {block_hash} is not finalized"
			));
		}

		Ok(())
	}

	fn get_block(&self, at: Option<Block::Hash>) -> RpcResult<SignedBlock<Block>> {
		let at = self.at_or_best(at);
		self.client
			.block(at)
			.map_err(|e| internal_err!("Invalid block number: {:?}", e))?
			.ok_or_else(|| internal_err!("Missing block {}", at))
	}

	fn get_finalized_block(&self, at: Option<Block::Hash>) -> RpcResult<SignedBlock<Block>> {
		let signed_block = self.get_block(at)?;
		self.ensure_block_finalized(&signed_block)?;
		Ok(signed_block)
	}

	/// Get the evaluation grid for the given block from cache if available, otherwise construct it.
	async fn get_eval_grid(&self, at: Option<Block::Hash>) -> RpcResult<Arc<EvaluationGrid>> {
		let (_api, at, block_number, block_len, extrinsics, header) = self.scope(at)?;
		self.eval_grid_cache
			.try_get_with(at, async move {
				match header.extension() {
					HeaderExtension::V3(ext) => {
						if ext.commitment.commitment.is_empty() {
							return Err(internal_err!(
								"Requested block {at} has empty commitments"
							));
						}
					},
				};

				let app_extrinsics = HeaderExtensionBuilderData::from_opaque_extrinsics::<
					RTExtractor,
				>(block_number, &extrinsics)
				.to_app_extrinsics();

				let grid = EvaluationGrid::from_extrinsics(
					app_extrinsics,
					MIN_WIDTH,
					block_len.cols.0 as usize,
					block_len.rows.0 as usize,
					Seed::default(),
				)
				.map_err(|e| internal_err!("Building evals grid failed: {:?}", e))?
				.extend_columns(NonZeroU16::new(2).expect("2>0"))
				.map_err(|_| {
					internal_err!("Failed to extend the columns of the evaluation grid")
				})?;
				Ok(Arc::new(grid))
			})
			.await
			.map_err(|e| internal_err!("Failed to get evaluation grid: {e:?}"))
	}

	/// Get the polynomial grid for the given block from cache if available, otherwise construct it.
	async fn get_poly_grid(
		&self,
		at: Option<Block::Hash>,
	) -> RpcResult<Arc<(Dimensions, PolynomialGrid)>> {
		let block_hash = self.at_or_best(at);
		self.poly_grid_cache
			.try_get_with(block_hash, async move {
				let evals = self.get_eval_grid(Some(block_hash)).await?;
				let polys = evals
					.make_polynomial_grid()
					.map_err(|e| internal_err!("Error getting polynomial grid {:?}", e))?;
				Ok::<Arc<(Dimensions, PolynomialGrid)>, ErrorObject<'static>>(Arc::new((
					evals.dims(),
					polys,
				)))
			})
			.await
			.map_err(|e| internal_err!("Failed to construct polynomial grid: {e:?}"))
	}
}

#[async_trait]
impl<Client, Block> KateApiServer<Block> for Kate<Client, Block>
where
	Block: BlockT<Extrinsic = OpaqueExtrinsic>,
	<Block as BlockT>::Header: ExtendedHeader<Extension = HeaderExtension>,
	Client: Send + Sync + 'static,
	Client: HeaderBackend<Block> + ProvideRuntimeApi<Block> + BlockBackend<Block>,
	Client::Api: DataAvailApi<Block> + RTKateApi<Block>,
{
	async fn query_rows(&self, rows: Rows, at: Option<HashOf<Block>>) -> RpcResult<Vec<GRow>> {
		let _metric_observer = MetricObserver::new(ObserveKind::KateQueryRows);

		let grid = self.get_eval_grid(at).await?;

		let selected_rows = rows
			.iter()
			.map(|&row| usize::try_from(row))
			.collect::<Result<Vec<_>, _>>()
			.map_err(|_| internal_err!("Failed to convert row indexes"))?;

		let rows_data = selected_rows
			.into_par_iter()
			.map(|row_idx| {
				grid.row(row_idx)
					.ok_or_else(|| internal_err!("Row does not exist: {row_idx}"))
					.and_then(|row| {
						row.iter()
							.map(|scalar| scalar.to_bytes().map(GRawScalar::from))
							.collect::<Result<Vec<_>, _>>()
							.map_err(|_| {
								internal_err!("Failed to convert scalar for row {row_idx}")
							})
					})
			})
			.collect::<Result<Vec<_>, _>>()?;

		Ok(rows_data)
	}

	async fn query_proof(
		&self,
		cells: Cells,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<GDataProof>> {
		if cells.len() > self.max_cells_size {
			return Err(internal_err!(
				"Cannot query ({}) more than {} cells per request. Either increase the max cells size (--kate-max-cells-size) or query fewer cells.",
				cells.len(),
				self.max_cells_size
			));
		}

		let _metric_observer = MetricObserver::new(ObserveKind::KateQueryProof);
		let start = std::time::Instant::now();
		let grid = self.get_eval_grid(at).await?;
		let poly = self.get_poly_grid(at).await?;
		let srs = SRS.get_or_init(multiproof_params);

		let proofs: Result<Vec<GDataProof>, ErrorObject> = cells
			.par_iter()
			.map(|cell| {
				let (row, col) = (cell.row.0 as usize, cell.col.0 as usize);

				let data = grid.get(row, col).ok_or_else(|| {
					internal_err!(
						"Invalid cell {:?} for grid dimensions {:?}",
						cell,
						grid.dims()
					)
				})?;

				let proof = poly
					.1
					.proof(srs, cell)
					.map_err(|e| internal_err!("Unable to generate proof: {:?}", e))?;

				Ok((
					GRawScalar::from(
						data.to_bytes()
							.map_err(|_| internal_err!("Failed to serialize data"))?,
					),
					GProof::try_from(
						proof
							.to_bytes()
							.map_err(|_| internal_err!("Failed to serialize proof"))?
							.to_vec(),
					)
					.map_err(|_| internal_err!("Failed to convert proof"))?,
				))
			})
			.collect();
		println!("Proofs generation time: {:?}", start.elapsed());
		proofs.map_err(|e| internal_err!("Failed to generate proof: {:?}", e))
	}

	async fn query_block_length(&self, at: Option<HashOf<Block>>) -> RpcResult<BlockLength> {
		let _metric_observer = MetricObserver::new(ObserveKind::KateQueryBlockLength);

		let at = self.at_or_best(at);
		let api = self.client.runtime_api();
		let block_length = api
			.block_length(at)
			.map_err(|e| internal_err!("Length of best block({at:?}): {e:?}"))?;

		Ok(block_length)
	}

	async fn query_data_proof(
		&self,
		tx_idx: u32,
		at: Option<HashOf<Block>>,
	) -> RpcResult<ProofResponse> {
		let _metric_observer = MetricObserver::new(ObserveKind::KateQueryDataProof);

		// Calculate proof for block and tx index
		let (api, at, number, _, extrinsics, _) = self.scope(at)?;
		let proof = api
			.data_proof(at, number, extrinsics, tx_idx)
			.map_err(|e| internal_err!("KateApi::data_proof failed: {e:?}"))?
			.ok_or_else(|| {
				internal_err!("Cannot fetch tx data at tx index {tx_idx:?} at block {at:?}")
			})?;

		Ok(proof)
	}
}
