use avail_base::metrics::avail::KateRpcMetrics;
use avail_core::{data_proof::ProofResponse, traits::ExtendedHeader, AppId, OpaqueExtrinsic};
use da_control::kate::{GDataProof, GRow};
use da_runtime::apis::{DataAvailApi, KateApi as RTKateApi};
use kate::com::Cell;

use frame_support::BoundedVec;
use frame_system::limits::BlockLength;
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::{ErrorCode, ErrorObject},
};
use sc_client_api::BlockBackend;
use sp_api::{ApiRef, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::SignedBlock,
	traits::{Block as BlockT, ConstU32, Header},
};
use std::{marker::PhantomData, marker::Sync, sync::Arc, time::Instant};

pub type HashOf<Block> = <Block as BlockT>::Hash;
pub type MaxRows = ConstU32<64>;
pub type Rows = BoundedVec<u32, MaxRows>;
pub type MaxCells = ConstU32<10_000>;
pub type Cells = BoundedVec<Cell, MaxCells>;

pub mod metrics;

/// # TODO
/// - [ ] Update type definitions for RPCs in our subxt & explorer.
#[rpc(client, server)]
pub trait KateApi<Block>
where
	Block: BlockT,
{
	#[method(name = "kate_queryRows")]
	async fn query_rows(&self, rows: Rows, at: Option<HashOf<Block>>) -> RpcResult<Vec<GRow>>;

	#[method(name = "kate_queryAppData")]
	async fn query_app_data(
		&self,
		app_id: AppId,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<Option<GRow>>>;

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

/// TODO
/// - Re-enable caches on RPC.
/// - Use rayon again.
#[allow(clippy::type_complexity)]
pub struct Kate<Client, Block: BlockT> {
	client: Arc<Client>,
	// eval_grid_cache: Cache<Block::Hash, Arc<EvaluationGrid>>,
	// Have to put dimensions here b/c it's not public in polynomialgrid
	// poly_grid_cache: Cache<Block::Hash, Arc<(Dimensions, PolynomialGrid)>>,
	// multiproof_srs: m1_blst::M1NoPrecomp,
	max_cells_size: usize,
	_block: PhantomData<Block>,
}

impl<Client, Block: BlockT> Kate<Client, Block> {
	pub fn new(
		client: Arc<Client>,
		max_cells_size: usize,
		_eval_grid_cache_size: u64,
		_poly_grid_cach_size: u64,
	) -> Self {
		/*
		// eval_grid_cache_size and poly_grid_cach_size are in MiB. We need Bytes.
		let eval_grid_cache_size = eval_grid_cache_size * 1024 * 1024;
		let poly_grid_cach_size = poly_grid_cach_size * 1024 * 1024;
		*/

		Self {
			client,
			/*
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
					let n_points: u32 =
						v.0.width().try_into().expect("Never more than 2^32 points");
					n_cells * 32 + n_points * 32
				})
				.max_capacity(poly_grid_cach_size)
				.build(),
			multiproof_srs: kate::couscous::multiproof_params(),
			*/
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
	Block: BlockT,
	<Block as BlockT>::Header: ExtendedHeader,
	Client: Send + Sync + 'static,
	Client: HeaderBackend<Block> + ProvideRuntimeApi<Block> + BlockBackend<Block>,
	Client::Api: DataAvailApi<Block>,
	// Extrinsic: TryFrom<<Block as BlockT>::Extrinsic>,
{
	fn scope(
		&self,
		at: Option<Block::Hash>,
	) -> RpcResult<(
		Api<'_, Client, Block>,
		<Block as BlockT>::Hash,
		u32,
		BlockLength,
		Opaques<Block>,
	)> {
		let at = self.at_or_best(at);
		let block = self.get_finalized_block(Some(at))?.block;
		let number: u32 = (*block.header().number())
			.try_into()
			.map_err(|_| ErrorCode::InvalidParams)?;
		let (_, extrinsics) = block.deconstruct();

		let api = self.client.runtime_api();
		let block_len = api
			.block_length(at)
			.map_err(|e| internal_err!("Length of best block({at:?}): {e:?}"))?;

		Ok((api, at, number, block_len, extrinsics))
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
}

#[async_trait]
impl<Client, Block> KateApiServer<Block> for Kate<Client, Block>
where
	Block: BlockT<Extrinsic = OpaqueExtrinsic>,
	<Block as BlockT>::Header: ExtendedHeader,
	Client: Send + Sync + 'static,
	Client: HeaderBackend<Block> + ProvideRuntimeApi<Block> + BlockBackend<Block>,
	Client::Api: DataAvailApi<Block> + RTKateApi<Block>,
{
	async fn query_rows(&self, rows: Rows, at: Option<HashOf<Block>>) -> RpcResult<Vec<GRow>> {
		let (api, at, number, block_len, extrinsics) = self.scope(at)?;

		let execution_start = Instant::now();
		let grid_rows = api
			.rows(at, number, extrinsics, block_len, rows.into())
			.map_err(|kate_err| internal_err!("Failed Kate rows: {kate_err:?}"))?
			.map_err(|api_err| internal_err!("Failed API: {api_err:?}"))?;
		KateRpcMetrics::observe_query_rows_execution_time(execution_start.elapsed());

		Ok(grid_rows)
	}

	async fn query_app_data(
		&self,
		app_id: AppId,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<Option<GRow>>> {
		let (api, at, number, block_len, extrinsics) = self.scope(at)?;

		let execution_start = Instant::now();
		let app_data = api
			.app_data(at, number, extrinsics, block_len, app_id)
			.map_err(|kate_err| internal_err!("Failed Kate app data: {kate_err:?}"))?
			.map_err(|api_err| internal_err!("Failed API: {api_err:?}"))?;
		KateRpcMetrics::observe_query_app_data_execution_time(execution_start.elapsed());

		Ok(app_data)
	}

	async fn query_proof(
		&self,
		cells: Cells,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<GDataProof>> {
		if cells.len() > self.max_cells_size {
			return Err(
				internal_err!(
					"Cannot query ({}) more than {} amount of cells per request. Either increase the max cells size (--kate-max-cells-size) or query less amount of cells per request.",
					cells.len(),
					self.max_cells_size
				)
			);
		}

		let (api, at, number, block_len, extrinsics) = self.scope(at)?;

		let execution_start = Instant::now();
		let cells = cells
			.into_iter()
			.map(|cell| (cell.row.0, cell.col.0))
			.collect::<Vec<_>>();
		let proof = api
			.proof(at, number, extrinsics, block_len, cells)
			.map_err(|kate_err| internal_err!("KateApi::proof failed: {kate_err:?}"))?
			.map_err(|api_err| internal_err!("Failed API: {api_err:?}"))?;

		// Execution Time Metric
		KateRpcMetrics::observe_query_proof_execution_time(execution_start.elapsed());

		Ok(proof)
	}

	async fn query_block_length(&self, at: Option<HashOf<Block>>) -> RpcResult<BlockLength> {
		let execution_start = Instant::now();

		let at = self.at_or_best(at);
		let api = self.client.runtime_api();
		let block_length = api
			.block_length(at)
			.map_err(|e| internal_err!("Length of best block({at:?}): {e:?}"))?;

		// Execution Time Metric
		KateRpcMetrics::observe_query_block_length_execution_time(execution_start.elapsed());

		Ok(block_length)
	}

	async fn query_data_proof(
		&self,
		tx_idx: u32,
		at: Option<HashOf<Block>>,
	) -> RpcResult<ProofResponse> {
		// Calculate proof for block and tx index
		let (api, at, number, _, extrinsics) = self.scope(at)?;

		let execution_start = Instant::now();
		let proof = api
			.data_proof(at, number, extrinsics, tx_idx)
			.map_err(|e| internal_err!("KateApi::data_proof failed: {e:?}"))?
			.ok_or_else(|| {
				internal_err!("Cannot to fetch tx data at tx index {tx_idx:?} at block {at:?}")
			})?;
		KateRpcMetrics::observe_query_data_proof_execution_time(execution_start.elapsed());

		Ok(proof)
	}
}
