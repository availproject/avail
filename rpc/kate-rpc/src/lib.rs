use std::{marker::PhantomData, sync::Arc, vec};

use codec::{Compact, Decode, Encode, Error as DecodeError, Input};
use da_primitives::{
	asdr::{AppExtrinsic, AppId, GetAppId},
	traits::ExtendedHeader,
	DataProof,
};
use dusk_bytes::Serializable;
use frame_support::traits::ExtrinsicCall;
use frame_system::{limits::BlockLength, submitted_data};
use jsonrpsee::{
	core::{Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
};
use kate::{
	com::Cell,
	gridgen::{EvaluationGrid, PolynomialGrid},
	PublicParameters,
};
use kate_rpc_runtime_api::KateParamsGetter;
use moka::sync::Cache;
use sc_client_api::{BlockBackend, StorageProvider};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::{BlockId, SignedBlock},
	traits::{Block as BlockT, Extrinsic, Header},
	AccountId32, MultiAddress, MultiSignature,
};
use submitted_data::Filter;

pub type HashOf<Block> = <Block as BlockT>::Hash;
pub type CallOf<Block> = <<Block as BlockT>::Extrinsic as Extrinsic>::Call;

#[rpc(client, server)]
pub trait KateApi<Block, SDFilter>
where
	Block: BlockT,
	SDFilter: Filter<CallOf<Block>>,
{
	#[method(name = "kate_queryRows")]
	fn query_rows(
		&self,
		rows: Vec<u32>,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<Option<Vec<u8>>>>;

	#[method(name = "kate_queryAppData")]
	fn query_app_data(
		&self,
		app_id: AppId,
		at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<Option<Vec<u8>>>>;

	#[method(name = "kate_queryProof")]
	fn query_proof(&self, cells: Vec<Cell>, at: Option<HashOf<Block>>) -> RpcResult<Vec<u8>>;

	#[method(name = "kate_blockLength")]
	fn query_block_length(&self, at: Option<HashOf<Block>>) -> RpcResult<BlockLength>;

	#[method(name = "kate_queryMultiProof")]
	fn query_multiproof(&self, cells: Vec<Cell>, at: Option<HashOf<Block>>) -> RpcResult<Vec<u8>>;

	#[method(name = "kate_queryDataProof")]
	fn query_data_proof(&self, data_index: u32, at: Option<HashOf<Block>>) -> RpcResult<DataProof>;
}

struct Grid {
	evals: EvaluationGrid,
	polys: PolynomialGrid,
}
pub struct Kate<Client, Block: BlockT, SDFilter: Filter<CallOf<Block>>> {
	client: Arc<Client>,
	block_ext_cache: Cache<Block::Hash, Arc<Grid>>,
	filter: PhantomData<SDFilter>,
}

impl<Client, Block, SDFilter> Kate<Client, Block, SDFilter>
where
	Block: BlockT,
	SDFilter: Filter<CallOf<Block>>,
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
			filter: Default::default(),
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

impl<Client, Block, SDFilter> Kate<Client, Block, SDFilter>
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
	SDFilter: Filter<CallOf<Block>>,
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
				let seed: [u8; 32] =
					self.client
						.runtime_api()
						.get_babe_vrf(&block_id)
						.map_err(|e| {
							internal_err!("Babe VRF not found for block {}: {:?}", block_id, e)
						})?;

				let evals = kate::gridgen::EvaluationGrid::from_extrinsics(
					xts_by_id.clone(),
					4,
					block_length.cols.as_usize(), // 'cols' is the # of cols, so width
					block_length.rows.as_usize(), // 'rows' is the # of rows, so height
					seed.clone(),
				)
				.map_err(|e| internal_err!("Building evals grid failed: {:?}", e))?
				.extend_columns(2)
				.map_err(|e| internal_err!("Error extending grid {:?}", e))?;
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

impl<Client, Block, SDFilter> KateApiServer<Block, SDFilter> for Kate<Client, Block, SDFilter>
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
	SDFilter: Filter<CallOf<Block>> + 'static + Send + Sync,
	<<Block as BlockT>::Extrinsic as Extrinsic>::Call: Clone,
{
	fn query_rows(
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
					let row_bytes = row.iter().flat_map(|s| s.to_bytes()).collect();
					all_rows[y as usize] = Some(row_bytes)
				},
				_ => (),
			});
		Ok(all_rows)
	}

	fn query_app_data(
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

		let orig_dims =
			kate_grid::Dimensions::new(grid.evals.dims.width(), grid.evals.dims.height() / 2);

		let rows = grid
			.evals
			.app_rows(&app_id, Some(&orig_dims))
			.unwrap_or(vec![]);
		let mut all_rows = vec![None; orig_dims.height()];
		for (row_y, row) in rows {
			all_rows[row_y] = Some(
				row.into_iter()
					.flat_map(|s| s.to_bytes())
					.collect::<Vec<_>>(),
			);
		}

		Ok(all_rows)
	}

	//TODO allocate static thread pool, just for RPC related work, to free up resources, for the block producing processes.
	fn query_proof(&self, cells: Vec<Cell>, at: Option<HashOf<Block>>) -> RpcResult<Vec<u8>> {
		let signed_block = self.get_signed_block(at)?;
		let block_hash = signed_block.block.header().hash();

		if self.client.info().finalized_number < *signed_block.block.header().number() {
			return Err(internal_err!(
				"Requested block {block_hash} is not finalized"
			));
		}

		let kc_public_params_raw = self
			.client
			.runtime_api()
			.get_public_params(&BlockId::Hash(block_hash))
			.map_err(|e| {
				internal_err!(
					"Public params cannot be fetched on block {}: {:?}",
					signed_block.block.header().hash(),
					e
				)
			})?;
		let kc_public_params =
			unsafe { PublicParameters::from_slice_unchecked(&kc_public_params_raw) };

		let grid = self.get_grid(&signed_block)?;

		// TODO: cleaner serialization code
		let proof = cells
			.iter()
			.map(|cell| {
				use kate_grid::Grid;
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
							.proof(kc_public_params.commit_key(), cell)
							.map_err(|e| internal_err!("Unable to make proof: {:?}", e))
							.map(|proof| (data.to_bytes(), proof.to_bytes()))
					})
			})
			.collect::<Result<Vec<_>, _>>()?
			.into_iter()
			.flat_map(|(data, proof)| [proof.to_vec(), data.to_vec()])
			.collect::<Vec<_>>()
			.concat();

		Ok(proof)
	}

	fn query_block_length(&self, at: Option<HashOf<Block>>) -> RpcResult<BlockLength> {
		let block_id = self.block_id(at);
		let block_length = self
			.client
			.runtime_api()
			.get_block_length(&block_id)
			.map_err(|e| internal_err!("Length of best block({:?}): {:?}", block_id, e))?;

		Ok(block_length)
	}

	fn query_data_proof(&self, data_index: u32, at: Option<HashOf<Block>>) -> RpcResult<DataProof> {
		// Fetch block
		let block = self.get_signed_block(at)?.block;

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
					at
				)
			})?;
		DataProof::try_from(&merkle_proof)
			.map_err(|e| internal_err!("Data proof cannot be loaded from merkle root: {:?}", e))
	}

	fn query_multiproof(
		&self,
		_cells: Vec<Cell>,
		_at: Option<HashOf<Block>>,
	) -> RpcResult<Vec<u8>> {
		todo!()
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
	fn decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
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
	fn decode<I: Input>(input: &mut I) -> Result<AvailExtrinsic, DecodeError> {
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
