use super::*;
use crate::gridgen::*;
use crate::testnet;
use crate::Seed;
use da_types::AppExtrinsic;
use da_types::AppId;
use da_types::BlockLengthColumns;
use da_types::BlockLengthRows;
use dusk_bytes::Serializable;
use hex_literal::hex;
use kate_grid::Dimensions;
use kate_recovery::matrix::Position;
use test_case::test_case;

#[test]
fn test_build_commitments_simple_commitment_check() {
	let original_data = br#"test"#;
	let block_height = 256usize;
	let block_width = 256usize;
	let hash: Seed = [
		76, 41, 174, 145, 187, 12, 97, 32, 75, 111, 149, 209, 243, 195, 165, 10, 166, 172, 47, 41,
		218, 24, 212, 66, 62, 5, 187, 191, 129, 5, 105, 3,
	];

	let evals = EvaluationGrid::from_extrinsics(
		vec![AppExtrinsic::from(original_data.to_vec())],
		4,
		block_width,
		block_height,
		hash,
	)
	.unwrap();
	let evals = evals.extend_columns(2).unwrap();
	let polys = evals.make_polynomial_grid().unwrap();
	let commits = polys
		.commitments(pp().commit_key())
		.unwrap()
		.into_iter()
		.flat_map(|p| p.to_bytes())
		.collect::<Vec<_>>();

	assert_eq!(evals.dims, Dimensions::new_unchecked(4, 2));
	let expected_commitments = hex!("960F08F97D3A8BD21C3F5682366130132E18E375A587A1E5900937D7AA5F33C4E20A1C0ACAE664DCE1FD99EDC2693B8D960F08F97D3A8BD21C3F5682366130132E18E375A587A1E5900937D7AA5F33C4E20A1C0ACAE664DCE1FD99EDC2693B8D");
	assert_eq!(commits, expected_commitments);
}

#[test]
fn par_build_commitments_row_wise_constant_row() {
	// Due to scale encoding, first line is not constant.
	// We will use second line to ensure constant row.
	let hash = Seed::default();
	let xts = vec![AppExtrinsic {
		app_id: AppId(0),
		data: vec![0; 31 * 8],
	}];

	let evals = EvaluationGrid::from_extrinsics(xts, 4, 4, 4, hash).unwrap();
	let evals = evals.extend_columns(2).unwrap();
	let polys = evals.make_polynomial_grid().unwrap();
	polys.commitments(pp().commit_key()).unwrap();
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(20))]
	#[test]
	fn commitments_verify(ref exts in app_extrinsics_strategy())  {
		//let (layout, commitments, dims, matrix) = par_build_commitments(BlockLengthRows(64), BlockLengthColumns(16), 32, xts, Seed::default()).unwrap();
		let grid = EvaluationGrid::from_extrinsics(exts.clone(), 4, 16, 64, Seed::default()).unwrap().extend_columns(2).unwrap();
		let orig_dims = Dimensions::new(grid.dims.width_nz(), (grid.dims.height() / 2).try_into().unwrap());
		let polys = grid.make_polynomial_grid().unwrap();
		let commits = polys.commitments(pp().commit_key())
			.unwrap()
			.iter()
			.map(|c| c.to_bytes())
			.collect::<Vec<_>>();

		let index = app_data_index_from_lookup(&grid.lookup);
		let public_params = testnet::public_params((grid.dims.width() as u32).into());

		for xt in exts {
			let rows = grid.app_rows(&xt.app_id, Some(&orig_dims)).unwrap();
			// Have to put the rows we find in this funky data structure
			let mut app_rows = vec![None; grid.dims.height()];
			for (row_i, row) in rows {
				app_rows[row_i] = Some(row.iter().flat_map(|s| s.to_bytes()).collect());
			}
			// Need to provide the original dimensions here too
			let extended_dims = kate_recovery::matrix::Dimensions::new(orig_dims.height() as u16, orig_dims.width() as u16).unwrap();
			let (_, missing) = kate_recovery::commitments::verify_equality(&public_params, &commits, &app_rows, &index, &extended_dims, xt.app_id.0).unwrap();
			prop_assert!(missing.is_empty());
		}
	}

	fn verify_commitments_missing_row(ref xts in app_extrinsics_strategy())  {
		let grid = EvaluationGrid::from_extrinsics(xts.clone(), 4, 16, 64, Seed::default()).unwrap().extend_columns(2).unwrap();
		let orig_dims = Dimensions::new(grid.dims.width_nz(), (grid.dims.height() / 2).try_into().unwrap());
		let polys = grid.make_polynomial_grid().unwrap();
		let commits = polys.commitments(pp().commit_key())
			.unwrap()
			.iter()
			.map(|c| c.to_bytes())
			.collect::<Vec<_>>();

		let index = app_data_index_from_lookup(&grid.lookup);
		let public_params = testnet::public_params((grid.dims.width() as u32).into());

		//let (layout, commitments, dims, matrix) = par_build_commitments(BlockLengthRows(64), BlockLengthColumns(16), 32, xts, Seed::default(), &IgnoreMetrics{}).unwrap();

		//let index = app_data_index_try_from_layout(layout).unwrap();
		//let public_params = testnet::public_params(dims.cols.as_usize());
		//let extended_dims =  dims.try_into().unwrap();
		//let commitments = commitments::from_slice(&commitments).unwrap();
		for xt in xts {
			let rows = grid.app_rows(&xt.app_id, Some(&orig_dims)).unwrap();
			let mut row_elems = vec![None; grid.dims.height()];
			for (i, data) in &rows {
				row_elems[*i] = Some(data.iter().flat_map(|s| s.to_bytes()).collect());
			}
			let first_index = rows.iter().map(|(i, _)| *i).min().unwrap();
			row_elems.remove(first_index);

			let extended_dims = kate_recovery::matrix::Dimensions::new(orig_dims.height() as u16, orig_dims.width() as u16).unwrap();
			let (_, missing) = kate_recovery::commitments::verify_equality(&public_params, &commits, &row_elems,&index,&extended_dims,xt.app_id.0).unwrap();
			prop_assert!(!missing.is_empty());
		}
	}
}

#[test_case( ([1,1,1,1]).to_vec(); "All values are non-zero but same")]
#[test_case( ([0,0,0,0]).to_vec(); "All values are zero")]
#[test_case( ([0,5,2,1]).to_vec(); "All values are different")]
fn test_zero_deg_poly_commit(row_values: Vec<u8>) {
	// There are two main cases that generate a zero degree polynomial. One is for data that is non-zero, but the same.
	// The other is for all-zero data. They differ, as the former yields a polynomial with one coefficient, and latter generates zero coefficients.
	let len = row_values.len();
	let public_params = pp();

	let row = row_values
		.iter()
		.map(|val| pad_to_bls_scalar(&[*val]).unwrap())
		.collect::<Vec<_>>();

	//let ae = AppExtrinsic { 0.into(), vec![}
	let ev = EvaluationGrid {
		lookup: Default::default(), // Shouldn't need to care about this
		dims: Dimensions::new_unchecked(row_values.len(), 1),
		evals: row.into_row_major(row_values.len(), 1).unwrap(),
	};

	println!("Row: {:?}", ev.evals.inner());

	let pg = ev.make_polynomial_grid().unwrap();
	println!("Poly: {:?}", pg.inner[0]);
	let commitment = pg.commitment(pp().commit_key(), 0).unwrap().to_bytes();

	for x in 0..len {
		// Randomly chosen cell to prove, probably should test all of them
		let cell = Cell {
			col: BlockLengthColumns(x.try_into().unwrap()),
			row: BlockLengthRows(0),
		};

		let proof = pg.proof(pp().commit_key(), &cell).unwrap();

		let proof_bytes = proof.to_bytes();
		let cell_bytes = ev.evals.get(x, 0).unwrap().to_bytes();
		let content = [&proof_bytes[..], &cell_bytes[..]].concat();
		let dims = kate_recovery::matrix::Dimensions::new(1, 4).unwrap();
		let cell = kate_recovery::data::Cell {
			position: Position {
				row: 0,
				col: x as u16,
			},
			content: content.try_into().unwrap(),
		};
		let verification = kate_recovery::proof::verify(&public_params, &dims, &commitment, &cell);
		assert!(verification.is_ok());
		assert!(verification.unwrap())
	}
}
