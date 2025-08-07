use super::{Error, GCellBlock, GDataProof, GMultiProof, GProof, GRawScalar, GRow, LOG_TARGET};
use avail_base::header_extension::SubmittedData;
use avail_core::{
	header::{extension as he, HeaderExtension},
	kate::DATA_CHUNK_SIZE,
	kate_commitment as kc, AppExtrinsic, AppId, BlockLengthColumns, BlockLengthRows, DataLookup,
};
use core::num::NonZeroU16;
use frame_system::{limits::BlockLength, native::hosted_header_builder::MIN_WIDTH};
#[cfg(feature = "std")]
use kate::{
	com::Cell,
	couscous::multiproof_params,
	gridgen::core::{AsBytes as _, EvaluationGrid as EGrid, PolynomialGrid},
	M1NoPrecomp,
};
use kate::{ArkScalar, Seed};
use kate_recovery::matrix::Dimensions;
use sp_runtime::SaturatedConversion as _;
use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;

use lru::LruCache;
#[cfg(feature = "std")]
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sp_core::H256;
use sp_std::sync::{Arc, Mutex};

const MAX_CACHED_GRIDS: usize = 5;
#[cfg(feature = "std")]
static CACHE: std::sync::OnceLock<BlockGridCache> = std::sync::OnceLock::new();
#[cfg(feature = "std")]
static SRS: std::sync::OnceLock<M1NoPrecomp> = std::sync::OnceLock::new();

#[cfg(feature = "std")]
struct GridCache {
	eval_grid: Arc<EGrid>,
	polynomial_grid: Arc<PolynomialGrid>,
}

#[cfg(feature = "std")]
struct BlockGridCache {
	cache: Mutex<LruCache<H256, Arc<GridCache>>>,
}

#[cfg(feature = "std")]
impl BlockGridCache {
	fn new() -> Self {
		Self {
			cache: Mutex::new(LruCache::new(MAX_CACHED_GRIDS)),
		}
	}

	fn get_or_insert(
		&self,
		block_hash: H256,
		create_fn: impl FnOnce() -> Result<(Arc<EGrid>, Arc<PolynomialGrid>), Error>,
	) -> Result<Arc<GridCache>, Error> {
		// Try to get existing cache entry
		if let Some(cached) = self
			.cache
			.lock()
			.map_err(|_| Error::CacheLockPoisoned)?
			.get(&block_hash)
		{
			log::debug!(target: LOG_TARGET, "Using cached grids for block hash: {:?}", block_hash);
			return Ok(cached.clone());
		}
		// Create new grids if not in cache
		let (eval_grid, polynomial_grid) = create_fn()?;
		let grid_cache = Arc::new(GridCache {
			eval_grid,
			polynomial_grid,
		});

		// Insert into cache
		self.cache
			.lock()
			.map_err(|_| Error::CacheLockPoisoned)?
			.put(block_hash, grid_cache.clone());

		Ok(grid_cache)
	}
}

#[cfg(feature = "std")]
pub fn proof_with_cache(
	block_hash: H256,
	extrinsics: Vec<Vec<u8>>,
	block_len: BlockLength,
	cells: Vec<(u32, u32)>,
) -> Result<Vec<GDataProof>, Error> {
	let cache = CACHE.get_or_init(|| BlockGridCache::new());

	let srs = SRS.get_or_init(multiproof_params);
	let (max_width, max_height) = to_width_height(&block_len);

	let seed = Seed::default();
	// Get or create grids
	let cache_entry = cache.get_or_insert(block_hash, || {
		// Compute fresh grids

		let grids: Vec<EGrid> = extrinsics
			.iter()
			.map(|data| EGrid::from_data(data.clone(), max_width, max_width, max_height, seed))
			.collect::<Result<_, _>>()?;

		let uni_grid = EGrid::merge_with_padding(grids)?
			.extend_columns(NonZeroU16::new(2).ok_or(Error::ColumnExtension)?)
			.map_err(|_| Error::ColumnExtension)?;

		let polynomial_grid = Arc::new(uni_grid.make_polynomial_grid()?);

		let eval_grid = Arc::new(uni_grid);

		Ok((eval_grid, polynomial_grid))
	})?;

	// Use cached or newly created grids
	let uni_grid = cache_entry.eval_grid.as_ref();
	let poly = cache_entry.polynomial_grid.as_ref();

	generate_proofs(uni_grid, poly, srs, cells)
}

/// builds header extension and stores the grids in cache to be used later
#[cfg(feature = "std")]
pub fn build_extension(
	block_hash: H256,
	submitted_datas: Vec<SubmittedData>,
	data_root: H256,
	block_length: BlockLength,
) -> Result<HeaderExtension, Error> {
	let cache = CACHE.get_or_init(|| BlockGridCache::new());
	let srs = SRS.get_or_init(multiproof_params);

	let (max_width, max_height) = to_width_height(&block_length);
	let data_per_row = max_width * DATA_CHUNK_SIZE;
	let seed = Seed::default();

	// Compute app_rows: ceil(len / grid_width)
	let app_rows: Vec<(AppId, usize)> = submitted_datas
		.iter()
		.map(|sd| {
			let len = sd.data.len();
			let rows = (len + data_per_row - 1) / data_per_row;
			(sd.id, rows)
		})
		.collect();

	// Construct individual EGrids
	let grids: Vec<EGrid> = submitted_datas
		.into_iter()
		.map(|sd| EGrid::from_data(sd.data, max_width, max_width, max_height, seed))
		.collect::<Result<_, _>>()?;

	// Merge into unified grid and create polynomial grid
	let uni_grid = EGrid::merge_with_padding(grids)?;
	let poly_grid = uni_grid.make_polynomial_grid()?;


	// Generate commitments and convert to flat Vec<u8>
	let commitment_bytes: Vec<u8> = poly_grid
		.commitments(srs)?
		.into_iter()
		.map(|c| {
			c.to_bytes().map_err(|e| {
				log::error!(target: LOG_TARGET, "Invalid commitment bytes: {e}");
				Error::InvalidCommitments
			})
		})
		.collect::<Result<Vec<[u8; 48]>, _>>()?
		.into_iter()
		.flatten()
		.collect();

	// Construct DataLookup from app_rows
	let app_lookup = DataLookup::from_id_and_len_iter(app_rows.into_iter())
		.map_err(|_| Error::DataLookupFailed)?;
	let original_rows = app_lookup.len();
	let padded_rows = original_rows.next_power_of_two();

	let commitment = kc::v3::KateCommitment::new(
		padded_rows.try_into().unwrap_or_default(),
		max_width.try_into().unwrap_or_default(),
		data_root,
		commitment_bytes,
	);

	log::debug!(target: LOG_TARGET, "HeaderExtension building finished, caching the grids for future use...");

	// Cache the (eval_grid, poly_grid) for this block_hash
	let _cache_entry = cache.get_or_insert(block_hash, || {
		let extended_grid = uni_grid
			.extend_columns_par(NonZeroU16::new(2).ok_or(Error::ColumnExtension)?)
			.map_err(|_| Error::ColumnExtension)?;

		let polynomial_grid = Arc::new(extended_grid.make_polynomial_grid()?);
		let eval_grid = Arc::new(extended_grid);

		Ok((eval_grid, polynomial_grid))
	})?;

	Ok(he::v4::HeaderExtension {
		app_lookup,
		commitment,
	}
	.into())
}

#[cfg(feature = "std")]
fn generate_proofs(
	uni_grid: &EGrid,
	poly: &PolynomialGrid,
	srs: &M1NoPrecomp,
	cells: Vec<(u32, u32)>,
) -> Result<Vec<GDataProof>, Error> {
	cells
		.into_par_iter()
		.map(|(row, col)| {
			let data = uni_grid
				.get(row as usize, col as usize)
				.ok_or(Error::MissingCell { row, col })?
				.to_bytes()
				.map(GRawScalar::from)
				.map_err(|_| Error::InvalidScalarAtRow(row))?;
			let proof = poly
				.proof(
					srs,
					&Cell::new(BlockLengthRows(row), BlockLengthColumns(col)),
				)?
				.to_bytes()
				.map(GProof)
				.map_err(|_| Error::Proof)?;

			Ok((data, proof))
		})
		.collect::<Result<Vec<_>, _>>()
}

/// Hosted function to build the header using `kate` commitments.
#[runtime_interface]
pub trait HostedKate {
	fn grid(
		submitted: Vec<AppExtrinsic>,
		block_length: BlockLength,
		seed: Seed,
		selected_rows: Vec<u32>,
	) -> Result<Vec<GRow>, Error> {
		let (max_width, max_height) = to_width_height(&block_length);
		let selected_rows = selected_rows
			.into_par_iter()
			.map(usize::try_from)
			.collect::<Result<Vec<_>, _>>()?;

		let grid = EGrid::from_extrinsics(submitted, MIN_WIDTH, max_width, max_height, seed)?
			.extend_columns(NonZeroU16::new(2).expect("2>0"))
			.map_err(|_| Error::ColumnExtension)?;
		let rows = selected_rows
			.into_par_iter()
			.map(|row_idx| {
				let row = grid.row(row_idx).ok_or(Error::MissingRow(row_idx as u32))?;
				row.iter()
					.map(|scalar| scalar.to_bytes().map(GRawScalar::from))
					.collect::<Result<Vec<_>, _>>()
					.map_err(|_| Error::InvalidScalarAtRow(row_idx as u32))
			})
			.collect::<Result<Vec<_>, _>>()?;

		Ok(rows)
	}

	fn proof(
		extrinsics: Vec<AppExtrinsic>,
		block_len: BlockLength,
		seed: Seed,
		cells: Vec<(u32, u32)>,
	) -> Result<Vec<GDataProof>, Error> {
		let srs = SRS.get_or_init(multiproof_params);
		let (max_width, max_height) = to_width_height(&block_len);
		let grid = EGrid::from_extrinsics(extrinsics, MIN_WIDTH, max_width, max_height, seed)?
			.extend_columns(NonZeroU16::new(2).expect("2>0"))
			.map_err(|_| Error::ColumnExtension)?;

		let poly = grid.make_polynomial_grid()?;

		let proofs = cells
			.into_par_iter()
			.map(|(row, col)| -> Result<GDataProof, Error> {
				let data: GRawScalar = grid
					.get(row as usize, col as usize)
					.ok_or(Error::MissingCell { row, col })?
					.to_bytes()
					.map(GRawScalar::from)
					.map_err(|_| Error::InvalidScalarAtRow(row))?;

				let cell = Cell::new(BlockLengthRows(row), BlockLengthColumns(col));
				let proof = poly
					.proof(srs, &cell)?
					.to_bytes()
					.map(GProof)
					.map_err(|_| Error::Proof)?;

				Ok((data, proof))
			})
			.collect::<Result<Vec<_>, _>>()?;

		Ok(proofs)
	}

	fn grid_v4(
		submitted: Vec<SubmittedData>,
		block_length: BlockLength,
		seed: Seed,
		selected_rows: Vec<u32>,
	) -> Result<Vec<GRow>, Error> {
		let (max_width, max_height) = to_width_height(&block_length);
		let selected_rows = selected_rows
			.into_par_iter()
			.map(usize::try_from)
			.collect::<Result<Vec<_>, _>>()?;

		let grids: Vec<EGrid> = submitted
			.into_iter()
			.map(|ext| EGrid::from_data(ext.data, max_width, max_width, max_height, seed))
			.collect::<Result<_, _>>()?;
		// Create a universal grid by merging individual tx grids
		let uni_grid = EGrid::merge_with_padding(grids)?
			.extend_columns(NonZeroU16::new(2).expect("2 > 0"))
			.map_err(|_| Error::ColumnExtension)?;
		let rows = selected_rows
			.into_par_iter()
			.map(|row_idx| {
				let row = uni_grid
					.row(row_idx)
					.ok_or(Error::MissingRow(row_idx as u32))?;
				row.iter()
					.map(|scalar| scalar.to_bytes().map(GRawScalar::from))
					.collect::<Result<Vec<_>, _>>()
					.map_err(|_| Error::InvalidScalarAtRow(row_idx as u32))
			})
			.collect::<Result<Vec<_>, _>>()?;

		Ok(rows)
	}

	fn proof_v4(
		extrinsics: Vec<SubmittedData>,
		block_len: BlockLength,
		seed: Seed,
		cells: Vec<(u32, u32)>,
	) -> Result<Vec<GDataProof>, Error> {
		let srs = SRS.get_or_init(multiproof_params);
		let (max_width, max_height) = to_width_height(&block_len);
		let grids: Vec<EGrid> = extrinsics
			.into_iter()
			.map(|ext| EGrid::from_data(ext.data, max_width, max_width, max_height, seed))
			.collect::<Result<_, _>>()?;
		// Create a universal grid by merging individual tx grids
		let uni_grid = EGrid::merge_with_padding(grids)?
			.extend_columns(NonZeroU16::new(2).expect("2 > 0"))
			.map_err(|_| Error::ColumnExtension)?;

		let poly = uni_grid.make_polynomial_grid()?;

		let proofs: Vec<GDataProof> = cells
			.into_par_iter()
			.map(|(row, col)| -> Result<GDataProof, Error> {
				let data = uni_grid
					.get(row as usize, col as usize)
					.ok_or(Error::MissingCell { row, col })?
					.to_bytes()
					.map(GRawScalar::from)
					.map_err(|_| Error::InvalidScalarAtRow(row))?;

				let proof = poly
					.proof(
						srs,
						&Cell::new(BlockLengthRows(row), BlockLengthColumns(col)),
					)?
					.to_bytes()
					.map(GProof)
					.map_err(|_| Error::Proof)?;

				Ok((data, proof))
			})
			.collect::<Result<_, _>>()?;
		Ok(proofs)
	}

	fn multiproof(
		extrinsics: Vec<AppExtrinsic>,
		block_len: BlockLength,
		seed: Seed,
		cells: Vec<(u32, u32)>,
	) -> Result<Vec<(GMultiProof, GCellBlock)>, Error> {
		let srs = SRS.get_or_init(multiproof_params);
		let (max_width, max_height) = to_width_height(&block_len);
		let grid = EGrid::from_extrinsics(extrinsics, MIN_WIDTH, max_width, max_height, seed)?
			.extend_columns(NonZeroU16::new(2).expect("2>0"))
			.map_err(|_| Error::ColumnExtension)?;

		let poly = grid.make_polynomial_grid()?;

		let proofs = cells
			.into_par_iter()
			.map(|(row, col)| -> Result<(GMultiProof, GCellBlock), Error> {
				let cell = Cell::new(BlockLengthRows(row), BlockLengthColumns(col));
				let target_dims = Dimensions::new(16, 64).expect("16,64>0");
				// TODO: This isn't correct, need to put in the correct mp grid dim
				// TODO: safety
				if cell.row.0 >= grid.dims().height() as u32
					|| cell.col.0 >= grid.dims().width() as u32
				{
					return Err(Error::MissingCell { row, col });
				}
				let mp = poly.multiproof(srs, &cell, &grid, target_dims)?;
				let data = mp
					.evals
					.into_iter()
					.flatten()
					.map(|e: ArkScalar| {
						e.to_bytes()
							.map(GRawScalar::from)
							.map_err(|_| Error::InvalidScalarAtRow(row))
					})
					.collect::<Result<Vec<GRawScalar>, _>>()?;

				let proof = mp.proof.to_bytes().map(GProof).map_err(|_| Error::Proof)?;

				Ok(((data, proof), GCellBlock::from(mp.block)))
			})
			.collect::<Result<Vec<_>, _>>()?;

		Ok(proofs)
	}

	fn app_data(
		submitted: Vec<AppExtrinsic>,
		block_length: BlockLength,
		seed: Seed,
		app_id: u32,
	) -> Result<Vec<Option<GRow>>, Error> {
		let (max_width, max_height) = to_width_height(&block_length);
		let grid = EGrid::from_extrinsics(submitted, MIN_WIDTH, max_width, max_height, seed)?;

		// let orig_dims = non_extended_dims(grid.dims()).ok_or(Error::InvalidDimension)?;
		let dims = grid.dims();
		let Some(rows) = grid.app_rows(AppId(app_id), Some(dims))? else {
			return Err(Error::AppRow);
		};

		let mut all_rows = vec![None; dims.height()];
		for (row_y, row) in rows {
			let g_row = row
				.into_par_iter()
				.map(|s| s.to_bytes().map(GRawScalar::from))
				.collect::<Result<Vec<_>, _>>()
				.map_err(|_| Error::InvalidScalarAtRow(row_y as u32))?;
			all_rows[row_y] = Some(g_row);
		}

		Ok(all_rows)
	}
}

fn to_width_height(block_len: &BlockLength) -> (usize, usize) {
	// even if we run on a u16 target this is fine
	let width = block_len.cols.0.saturated_into();
	let height = block_len.rows.0.saturated_into();
	(width, height)
}
