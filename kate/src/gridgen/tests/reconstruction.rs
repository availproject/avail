use super::{app_data_index_from_lookup, pp};
use crate::com::Cell;
use crate::gridgen::EvaluationGrid;
use crate::Seed;
use da_types::AppExtrinsic;
use dusk_bytes::Serializable;
use kate_grid::Grid;
use kate_recovery::com::reconstruct_extrinsics;
use kate_recovery::data::{Cell as DCell, DataCell};
use kate_recovery::matrix::Position as DPosition;
use proptest::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

fn sample_unique(rng: &mut impl Rng, n_samples: usize, n: usize) -> Vec<usize> {
	let mut sampled = vec![];
	let u = Uniform::from(0..n);
	while sampled.len() < n_samples || sampled.len() < n {
		let t = u.sample(rng);
		if !sampled.contains(&t) {
			sampled.push(t)
		}
	}
	sampled
}

fn sample_cells(grid: &EvaluationGrid, columns: Option<&[usize]>) -> Vec<DataCell> {
	let mut rng = ChaChaRng::from_seed([42u8; 32]);
	let cols: Vec<usize> = match columns {
		Some(cols) => cols.to_vec(),
		None => (0..grid.dims.width()).into_iter().collect(),
	};
	cols.iter()
		.flat_map(|x| {
			sample_unique(&mut rng, grid.dims.height() / 2, grid.dims.height())
				.into_iter()
				.map(move |y| kate_recovery::data::DataCell {
					position: kate_recovery::matrix::Position {
						row: y as u32,
						col: *x as u16,
					},
					data: grid.evals.get(*x, y).unwrap().to_bytes(),
				})
		})
		.collect::<Vec<_>>()
}

#[test]
fn test_multiple_extrinsics_for_same_app_id() {
	let xt1 = vec![5, 5];
	let xt2 = vec![6, 6];
	let xts = [
		AppExtrinsic {
			app_id: 1.into(),
			data: xt1.clone(),
		},
		AppExtrinsic {
			app_id: 1.into(),
			data: xt2.clone(),
		},
	];
	// The hash is used for seed for padding the block to next power of two value
	let hash = Seed::default();
	let ev = EvaluationGrid::from_extrinsics(xts.into(), 4, 128, 2, hash)
		.unwrap()
		.extend_columns(2)
		.unwrap();

	let cells = sample_cells(&ev, None);
	let index = app_data_index_from_lookup(&ev.lookup);
	let bdims =
		kate_recovery::matrix::Dimensions::new(ev.dims.height() as u16, ev.dims.width() as u16)
			.unwrap();
	let res = reconstruct_extrinsics(&index, &bdims, cells).unwrap();

	assert_eq!(res[0].1[0], xt1);
	assert_eq!(res[0].1[1], xt2);
}

proptest! {
#![proptest_config(ProptestConfig::with_cases(5))]
#[test]
fn test_build_and_reconstruct(exts in super::app_extrinsics_strategy())  {
	let grid = EvaluationGrid::from_extrinsics(exts.clone(), 4, 256, 256, Seed::default()).unwrap().extend_columns(2).unwrap();
	let dims = &grid.dims;
	//let (layout, commitments, dims, matrix) = par_build_commitments(
	//	BlockLengthRows(64), BlockLengthColumns(16), 32, xts, Seed::default()).unwrap();
	const RNG_SEED: Seed = [42u8; 32];

	let cells = sample_cells(&grid, None);
	let index = app_data_index_from_lookup(&grid.lookup);
	let bdims = kate_recovery::matrix::Dimensions::new(dims.height() as u16, dims.width() as u16).unwrap();
	let reconstructed = reconstruct_extrinsics(&index, &bdims, cells).unwrap();
	for (result, xt) in reconstructed.iter().zip(exts) {
		prop_assert_eq!(result.0, *xt.app_id);
		prop_assert_eq!(result.1[0].as_slice(), &xt.data);
	}

	let pp = pp();
	let polys = grid.make_polynomial_grid().unwrap();
	let commitments = polys.commitments(pp.commit_key()).unwrap();
	let indices = (0..dims.width()).flat_map(|x| (0..dims.height()).map(move |y| (x, y))).collect::<Vec<_>>();

	// Sample some number 10 of the indices, all is too slow for tests...
	let mut rng = ChaChaRng::from_seed(RNG_SEED);
	let sampled = Uniform::from(0..indices.len()).sample_iter(&mut rng).take(10).map(|i| indices[i]);
	for (x, y) in sampled {
		let cell = Cell { row: (y as u32).into(), col: (x as u32).into() };
		let proof = polys.proof(pp.commit_key(), &cell).unwrap();
		let mut content = [0u8; 80];
		content[..48].copy_from_slice(&proof.to_bytes()[..]);
		content[48..].copy_from_slice(&grid.evals.get(x, y).unwrap().to_bytes()[..]);

		let dcell = DCell{position: DPosition { row: y as u32, col: x as u16 }, content };
		let verification =  kate_recovery::proof::verify(&pp, &bdims, &commitments[y].to_bytes(),  &dcell);
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
		AppExtrinsic {
			app_id: 0.into(),
			data: vec![0],
		},
		AppExtrinsic {
			app_id: 1.into(),
			data: app_id_1_data.to_vec(),
		},
		AppExtrinsic {
			app_id: 2.into(),
			data: app_id_2_data.to_vec(),
		},
	];

	let grid = EvaluationGrid::from_extrinsics(xts.clone(), 4, 32, 4, Seed::default())
		.unwrap()
		.extend_columns(2)
		.unwrap();

    dbg!(&grid.evals.inner());

	let cols_1 = sample_cells(&grid, Some(&[0, 1, 2, 3]));

	let index = app_data_index_from_lookup(&grid.lookup);

	let bdims =
		kate_recovery::matrix::Dimensions::new(grid.dims.height() as u16, grid.dims.width() as u16)
			.unwrap();
	let res_1 = kate_recovery::com::reconstruct_app_extrinsics(&index, &bdims, cols_1, 1).unwrap();
	assert_eq!(res_1[0], app_id_1_data);

	let cols_2 = sample_cells(&grid, Some(&[0, 2, 3]));

	let res_2 = kate_recovery::com::reconstruct_app_extrinsics(&index, &bdims, cols_2, 2).unwrap();
	assert_eq!(res_2[0], app_id_2_data);
}
