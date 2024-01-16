use super::PMP;
use crate::{
	com::Cell,
	gridgen::{tests::sample_cells, EvaluationGrid},
	Seed,
};
use avail_core::{AppExtrinsic, AppId, BlockLengthColumns, BlockLengthRows, HeaderVersion};
use core::num::NonZeroU16;
use kate_recovery::{
	com::{reconstruct_app_extrinsics, reconstruct_extrinsics},
	data::Cell as DCell,
	matrix::{Dimensions, Position},
};
use poly_multiproof::traits::AsBytes;
use proptest::prelude::*;
use rand::{distributions::Uniform, prelude::Distribution, SeedableRng};
use rand_chacha::ChaChaRng;

#[test]
fn test_multiple_extrinsics_for_same_app_id() {
	let xt1 = vec![5, 5];
	let xt2 = vec![6, 6];
	let xts = [
		AppExtrinsic::new(AppId(1), xt1.clone()),
		AppExtrinsic::new(AppId(1), xt2.clone()),
	];
	// The hash is used for seed for padding the block to next power of two value
	let hash = Seed::default();
	let ev = EvaluationGrid::from_extrinsics(xts.into(), 4, 128, 2, hash, HeaderVersion::V3)
		.unwrap()
		.extend_columns(unsafe { NonZeroU16::new_unchecked(2) })
		.unwrap();

	let cells = sample_cells(&ev, None);
	let (rows, cols): (u16, u16) = ev.dims().into();
	let bdims = Dimensions::new_from(rows, cols).unwrap();
	let res = reconstruct_extrinsics(&ev.lookup, bdims, cells).unwrap();

	assert_eq!(res[0].1[0], xt1);
	assert_eq!(res[0].1[1], xt2);
}

proptest! {
#![proptest_config(ProptestConfig::with_cases(5))]
#[test]
fn test_build_and_reconstruct(exts in super::app_extrinsics_strategy())  {
	let grid = EvaluationGrid::from_extrinsics(exts.clone(), 4, 256, 256, Seed::default(), HeaderVersion::V3).unwrap().extend_columns(unsafe { NonZeroU16::new_unchecked(2)}).unwrap();
	let (rows, cols) :(usize,usize)= grid.dims().into();
	//let (layout, commitments, dims, matrix) = par_build_commitments(
	//	BlockLengthRows(64), BlockLengthColumns(16), 32, xts, Seed::default()).unwrap();
	const RNG_SEED: Seed = [42u8; 32];

	let cells = sample_cells(&grid, None);
	let bdims = Dimensions::new_from(rows, cols).unwrap();
	let reconstructed = reconstruct_extrinsics(&grid.lookup, bdims, cells).unwrap();
	for ((id,data), xt) in reconstructed.iter().zip(exts) {
		prop_assert_eq!(id.0, *xt.app_id);
		prop_assert_eq!(data[0].as_slice(), &xt.data);
	}

	let pp = &*PMP;
	let polys = grid.make_polynomial_grid().unwrap();
	let commitments = polys.commitments(pp).unwrap();
	let indices = (0..cols).flat_map(|x| (0..rows).map(move |y| (x, y))).collect::<Vec<_>>();

	// Sample some number 10 of the indices, all is too slow for tests...
	let mut rng = ChaChaRng::from_seed(RNG_SEED);
	let sampled = Uniform::from(0..indices.len()).sample_iter(&mut rng).take(10).map(|i| indices[i]);
	for (x, y) in sampled {
		let row = BlockLengthRows(u32::try_from(y).unwrap());
		let col = BlockLengthColumns(u32::try_from(x).unwrap());
		let cell = Cell::new( row, col);
		let proof = polys.proof(pp, &cell).unwrap();
		let mut content = [0u8; 80];
		content[..48].copy_from_slice(&proof.to_bytes().unwrap()[..]);
		content[48..].copy_from_slice(&grid.get(y, x).unwrap().to_bytes().unwrap()[..]);

		let dcell = DCell{position: Position { row: y as u32, col: x as u16 }, content };
		let verification =  kate_recovery::proof::verify(&kate_recovery::testnet::public_params(256), bdims, &commitments[y].to_bytes().unwrap(),  &dcell);
		prop_assert!(verification.is_ok());
		prop_assert!(verification.unwrap());
	}
}
}

#[test]
fn test_reconstruct_app_extrinsics_with_app_id() {
	let app_id_1_data = br#""This is mocked test data. It will be formatted as a matrix of BLS scalar cells and then individual columns 
get erasure coded to ensure redundancy."#;

	let app_id_2_data =
		br#""Let's see how this gets encoded and then reconstructed by sampling only some data."#;

	let xts = vec![
		AppExtrinsic::new(AppId(0), vec![0]),
		AppExtrinsic::new(AppId(1), app_id_1_data.to_vec()),
		AppExtrinsic::new(AppId(2), app_id_2_data.to_vec()),
	];

	let grid =
		EvaluationGrid::from_extrinsics(xts.clone(), 4, 4, 32, Seed::default(), HeaderVersion::V3)
			.unwrap()
			.extend_columns(unsafe { NonZeroU16::new_unchecked(2) })
			.unwrap();

	let cols_1 = sample_cells(&grid, Some(vec![0, 1, 2, 3]));

	let bdims = grid.dims();
	let res_1 = reconstruct_app_extrinsics(&grid.lookup, bdims, cols_1, AppId(1)).unwrap();
	assert_eq!(res_1[0], app_id_1_data);

	let cols_2 = sample_cells(&grid, Some(vec![0, 2, 3]));

	let res_2 = reconstruct_app_extrinsics(&grid.lookup, bdims, cols_2, AppId(2)).unwrap();
	assert_eq!(res_2[0], app_id_2_data);
}
