use super::{Error, GCellBlock, GDataProof, GMultiProof, GProof, GRawScalar, GRow};
// use avail_core::{AppId, BlockLengthColumns, BlockLengthRows};
use core::num::NonZeroU16;
use frame_system::{limits::BlockLength, native::hosted_header_builder::MIN_WIDTH};
use kate::{ArkScalar, Seed};
use kate_recovery::matrix::Dimensions;
use sp_runtime::SaturatedConversion as _;
use sp_runtime_interface::{
	pass_by::{AllocateAndReturnByCodec, PassFatPointerAndDecode},
	runtime_interface,
};
use sp_std::vec::Vec;

// === std-only / host-only imports ==========================================

#[cfg(not(substrate_runtime))]
use kate::{
	com::Cell,
	couscous::multiproof_params,
	gridgen::core::{AsBytes as _, EvaluationGrid as EGrid},
	M1NoPrecomp,
};

#[cfg(not(substrate_runtime))]
static SRS: std::sync::OnceLock<M1NoPrecomp> = std::sync::OnceLock::new();

#[cfg(not(substrate_runtime))]
use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// Hosted functions to work with KZG / kate commitments from the node.
///
/// NOTE:
/// - Arguments use `PassFatPointerAndDecode<...>` so they implement `RIType`.
/// - Return values use `AllocateAndReturnByCodec<...>` so they implement `IntoFFIValue`.
/// - Heavy kate/rayon logic is `#[cfg(not(substrate_runtime))]` so wasm side compiles.
#[runtime_interface]
pub trait HostedKate {
	// fn grid(
	// 	submitted: PassFatPointerAndDecode<Vec<AppExtrinsic>>,
	// 	block_length: PassFatPointerAndDecode<BlockLength>,
	// 	seed: PassFatPointerAndDecode<Seed>,
	// 	selected_rows: PassFatPointerAndDecode<Vec<u32>>,
	// ) -> AllocateAndReturnByCodec<Result<Vec<GRow>, Error>> {
	// 	#[cfg(not(substrate_runtime))]
	// 	{
	// 		let submitted: Vec<AppExtrinsic> = submitted.to_vec();
	// 		let block_length: BlockLength = *block_length;
	// 		let seed: Seed = *seed;
	// 		let selected_rows: Vec<u32> = selected_rows.to_vec();

	// 		let (max_width, max_height) = to_width_height(&block_length);
	// 		let selected_rows = selected_rows
	// 			.into_par_iter()
	// 			.map(usize::try_from)
	// 			.collect::<Result<Vec<_>, _>>()?;

	// 		let grid = EGrid::from_extrinsics(submitted, MIN_WIDTH, max_width, max_height, seed)?
	// 			.extend_columns(NonZeroU16::new(2).expect("2>0"))
	// 			.map_err(|_| Error::ColumnExtension)?;

	// 		let rows = selected_rows
	// 			.into_par_iter()
	// 			.map(|row_idx| {
	// 				let row = grid.row(row_idx).ok_or(Error::MissingRow(row_idx as u32))?;
	// 				row.iter()
	// 					.map(|scalar| scalar.to_bytes().map(GRawScalar::from))
	// 					.collect::<Result<Vec<_>, _>>()
	// 					.map_err(|_| Error::InvalidScalarAtRow(row_idx as u32))
	// 			})
	// 			.collect::<Result<Vec<_>, _>>()?;

	// 		Ok(rows)
	// 	}

	// 	#[cfg(substrate_runtime)]
	// 	{
	// 		// Should never be called from wasm; this is only for host.
	// 		let _ = (submitted, block_length, seed, selected_rows);
	// 		panic!("HostedKate::grid is only available on the host (not in substrate_runtime)");
	// 	}
	// }

	// fn proof(
	// 	extrinsics: PassFatPointerAndDecode<Vec<AppExtrinsic>>,
	// 	block_len: PassFatPointerAndDecode<BlockLength>,
	// 	seed: PassFatPointerAndDecode<Seed>,
	// 	cells: PassFatPointerAndDecode<Vec<(u32, u32)>>,
	// ) -> AllocateAndReturnByCodec<Result<Vec<GDataProof>, Error>> {
	// 	#[cfg(not(substrate_runtime))]
	// 	{
	// 		let extrinsics: Vec<AppExtrinsic> = extrinsics.to_vec();
	// 		let block_len: BlockLength = *block_len;
	// 		let seed: Seed = *seed;
	// 		let cells: Vec<(u32, u32)> = cells.to_vec();

	// 		let srs = SRS.get_or_init(multiproof_params);
	// 		let (max_width, max_height) = to_width_height(&block_len);
	// 		let grid = EGrid::from_extrinsics(extrinsics, MIN_WIDTH, max_width, max_height, seed)?
	// 			.extend_columns(NonZeroU16::new(2).expect("2>0"))
	// 			.map_err(|_| Error::ColumnExtension)?;

	// 		let poly = grid.make_polynomial_grid()?;

	// 		let proofs = cells
	// 			.into_par_iter()
	// 			.map(|(row, col)| -> Result<GDataProof, Error> {
	// 				let data: GRawScalar = grid
	// 					.get(row as usize, col as usize)
	// 					.ok_or(Error::MissingCell { row, col })?
	// 					.to_bytes()
	// 					.map(GRawScalar::from)
	// 					.map_err(|_| Error::InvalidScalarAtRow(row))?;

	// 				let cell = Cell::new(BlockLengthRows(row), BlockLengthColumns(col));
	// 				let proof = poly
	// 					.proof(srs, &cell)?
	// 					.to_bytes()
	// 					.map(GProof)
	// 					.map_err(|_| Error::Proof)?;

	// 				Ok((data, proof))
	// 			})
	// 			.collect::<Result<Vec<_>, _>>()?;

	// 		Ok(proofs)
	// 	}

	// 	#[cfg(substrate_runtime)]
	// 	{
	// 		let _ = (extrinsics, block_len, seed, cells);
	// 		panic!("HostedKate::proof is only available on the host (not in substrate_runtime)");
	// 	}
	// }

	// fn multiproof(
	// 	extrinsics: PassFatPointerAndDecode<Vec<AppExtrinsic>>,
	// 	block_len: PassFatPointerAndDecode<BlockLength>,
	// 	seed: PassFatPointerAndDecode<Seed>,
	// 	cells: PassFatPointerAndDecode<Vec<(u32, u32)>>,
	// ) -> AllocateAndReturnByCodec<Result<Vec<(GMultiProof, GCellBlock)>, Error>> {
	// 	#[cfg(not(substrate_runtime))]
	// 	{
	// 		let extrinsics: Vec<AppExtrinsic> = extrinsics.to_vec();
	// 		let block_len: BlockLength = *block_len;
	// 		let seed: Seed = *seed;
	// 		let cells: Vec<(u32, u32)> = cells.to_vec();

	// 		let srs = SRS.get_or_init(multiproof_params);
	// 		let (max_width, max_height) = to_width_height(&block_len);
	// 		let grid = EGrid::from_extrinsics(extrinsics, MIN_WIDTH, max_width, max_height, seed)?
	// 			.extend_columns(NonZeroU16::new(2).expect("2>0"))
	// 			.map_err(|_| Error::ColumnExtension)?;

	// 		let poly = grid.make_polynomial_grid()?;

	// 		let proofs = cells
	// 			.into_par_iter()
	// 			.map(|(row, col)| -> Result<(GMultiProof, GCellBlock), Error> {
	// 				let cell = Cell::new(BlockLengthRows(row), BlockLengthColumns(col));
	// 				let target_dims = Dimensions::new(16, 64).expect("16,64>0");

	// 				if cell.row.0 >= grid.dims().height() as u32
	// 					|| cell.col.0 >= grid.dims().width() as u32
	// 				{
	// 					return Err(Error::MissingCell { row, col });
	// 				}

	// 				let mp = poly.multiproof(srs, &cell, &grid, target_dims)?;
	// 				let data = mp
	// 					.evals
	// 					.into_iter()
	// 					.flatten()
	// 					.map(|e: ArkScalar| {
	// 						e.to_bytes()
	// 							.map(GRawScalar::from)
	// 							.map_err(|_| Error::InvalidScalarAtRow(row))
	// 					})
	// 					.collect::<Result<Vec<GRawScalar>, _>>()?;

	// 				let proof = mp.proof.to_bytes().map(GProof).map_err(|_| Error::Proof)?;

	// 				Ok(((data, proof), GCellBlock::from(mp.block)))
	// 			})
	// 			.collect::<Result<Vec<_>, _>>()?;

	// 		Ok(proofs)
	// 	}

	// 	#[cfg(substrate_runtime)]
	// 	{
	// 		let _ = (extrinsics, block_len, seed, cells);
	// 		panic!(
	// 			"HostedKate::multiproof is only available on the host (not in substrate_runtime)"
	// 		);
	// 	}
	// }

	// fn app_data(
	// 	submitted: PassFatPointerAndDecode<Vec<AppExtrinsic>>,
	// 	block_length: PassFatPointerAndDecode<BlockLength>,
	// 	seed: PassFatPointerAndDecode<Seed>,
	// 	app_id: u32,
	// ) -> AllocateAndReturnByCodec<Result<Vec<Option<GRow>>, Error>> {
	// 	#[cfg(not(substrate_runtime))]
	// 	{
	// 		let submitted: Vec<AppExtrinsic> = submitted.to_vec();
	// 		let block_length: BlockLength = *block_length;
	// 		let seed: Seed = *seed;

	// 		let (max_width, max_height) = to_width_height(&block_length);
	// 		let grid = EGrid::from_extrinsics(submitted, MIN_WIDTH, max_width, max_height, seed)?;

	// 		let dims = grid.dims();
	// 		let Some(rows) = grid.app_rows(AppId(app_id), Some(dims))? else {
	// 			return Err(Error::AppRow);
	// 		};

	// 		let mut all_rows: Vec<Option<GRow>> = vec![None; dims.height()];
	// 		for (row_y, row) in rows {
	// 			let g_row = row
	// 				.into_par_iter()
	// 				.map(|s| s.to_bytes().map(GRawScalar::from))
	// 				.collect::<Result<Vec<_>, _>>()
	// 				.map_err(|_| Error::InvalidScalarAtRow(row_y as u32))?;
	// 			all_rows[row_y] = Some(g_row);
	// 		}

	// 		Ok(all_rows)
	// 	}

	// 	#[cfg(substrate_runtime)]
	// 	{
	// 		let _ = (submitted, block_length, seed, app_id);
	// 		panic!("HostedKate::app_data is only available on the host (not in substrate_runtime)");
	// 	}
	// }
}

fn to_width_height(block_len: &BlockLength) -> (usize, usize) {
	// even if we run on a u16 target this is fine
	let width = block_len.cols.0.saturated_into();
	let height = block_len.rows.0.saturated_into();
	(width, height)
}
