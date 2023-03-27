use crate::com::Cell;
use crate::gridgen::EvaluationGrid;
use super::{app_data_index_from_lookup, pp};
use crate::Seed;
use dusk_bytes::Serializable;
use kate_grid::Grid;
use kate_recovery::com::reconstruct_extrinsics;
use kate_recovery::data::Cell as DCell;
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

proptest! {
#![proptest_config(ProptestConfig::with_cases(5))]
#[test]
fn newapi_test_build_and_reconstruct(exts in super::app_extrinsics_strategy())  {
	let grid = EvaluationGrid::from_extrinsics(exts.clone(), 4, 256, 256, Seed::default()).unwrap().extend_columns(2).unwrap();
	let gref = &grid;
	let dims = &grid.dims;
	//let (layout, commitments, dims, matrix) = par_build_commitments(
	//	BlockLengthRows(64), BlockLengthColumns(16), 32, xts, Seed::default()).unwrap();
	const RNG_SEED: Seed = [42u8; 32];
	let mut rng = ChaChaRng::from_seed(RNG_SEED);
	let cells = (0..dims.width())
		.flat_map(move |x| {
			sample_unique(&mut rng, dims.height()/2, dims.height())
				.into_iter()
				.map(move |y| {
					kate_recovery::data::DataCell {
						position: kate_recovery::matrix::Position { row: y as u32, col: x as u16 },
						data: gref.evals.get(x, y).unwrap().to_bytes()
					}
				}).collect::<Vec<_>>()
		}).collect::<Vec<_>>();
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
