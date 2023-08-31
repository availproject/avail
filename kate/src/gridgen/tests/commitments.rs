use super::*;
use crate::{gridgen::*, testnet, Seed};
use avail_core::{AppExtrinsic, AppId, BlockLengthColumns, BlockLengthRows};
use hex_literal::hex;
use kate_recovery::{
	commitments::verify_equality,
	matrix::{Dimensions, Position},
};
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
	let pmp_pp = crate::testnet::multiproof_params(256, 256);

	let evals = EvaluationGrid::from_extrinsics(
		vec![AppExtrinsic::from(original_data.to_vec())],
		4,
		block_width,
		block_height,
		hash,
	)
	.unwrap();
	let ext_evals = evals
		.extend_columns(unsafe { NonZeroU16::new_unchecked(2) })
		.unwrap();
	let polys = ext_evals.make_polynomial_grid().unwrap();
	let commits = polys
		.commitments(&*PMP)
		.unwrap()
		.into_iter()
		.flat_map(|p| p.to_bytes().unwrap())
		.collect::<Vec<_>>();
	let commits_fft_extended = evals
		.make_polynomial_grid()
		.unwrap()
		.extended_commitments(&pmp_pp, 2)
		.unwrap()
		.into_iter()
		.flat_map(|p| p.to_bytes().unwrap())
		.collect::<Vec<_>>();

	assert_eq!(ext_evals.dims(), Dimensions::new_from(2, 4).unwrap());
	let expected_commitments = hex!("960F08F97D3A8BD21C3F5682366130132E18E375A587A1E5900937D7AA5F33C4E20A1C0ACAE664DCE1FD99EDC2693B8D960F08F97D3A8BD21C3F5682366130132E18E375A587A1E5900937D7AA5F33C4E20A1C0ACAE664DCE1FD99EDC2693B8D");
	assert_eq!(commits, expected_commitments);
	assert_eq!(commits_fft_extended, expected_commitments);
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
	let evals = evals
		.extend_columns(unsafe { NonZeroU16::new_unchecked(2) })
		.unwrap();
	let polys = evals.make_polynomial_grid().unwrap();
	polys.commitments(&*PMP).unwrap();
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(20))]
	#[test]
	fn commitments_verify(ref exts in app_extrinsics_strategy())  {
		//let (layout, commitments, dims, matrix) = par_build_commitments(BlockLengthRows(64), BlockLengthColumns(16), 32, xts, Seed::default()).unwrap();
		let grid = EvaluationGrid::from_extrinsics(exts.clone(), 4, 16, 64, Seed::default()).unwrap();
		let grid = grid.extend_columns( unsafe { NonZeroU16::new_unchecked(2)}).unwrap();
		let (g_rows, g_cols) :(u16,u16) = grid.dims().into();
		let orig_dims = Dimensions::new(g_rows / 2, g_cols).unwrap();
		let polys = grid.make_polynomial_grid().unwrap();
		let commits = polys.commitments(&*PMP)
			.unwrap()
			.iter()
			.map(|c| c.to_bytes().unwrap())
			.collect::<Vec<_>>();

		let public_params = testnet::public_params(BlockLengthColumns(g_cols as u32));

		for xt in exts.iter() {
			let rows = grid.app_rows(xt.app_id, Some(orig_dims)).unwrap().unwrap();
			// Have to put the rows we find in this funky data structure
			let mut app_rows = vec![None; g_rows.into()];
			for (row_i, row) in rows {
				app_rows[row_i] = Some(row.iter().flat_map(|s| s.to_bytes().unwrap()).collect());
			}
			// Need to provide the original dimensions here too
			let extended_dims = orig_dims.clone();
			let (_, missing) = verify_equality(&public_params, &commits, &app_rows, &grid.lookup, extended_dims, xt.app_id).unwrap();
			prop_assert!(missing.is_empty());
		}
	}

	fn verify_commitments_missing_row(ref xts in app_extrinsics_strategy())  {
		let grid = EvaluationGrid::from_extrinsics(xts.clone(), 4, 16, 64, Seed::default()).unwrap().extend_columns( unsafe { NonZeroU16::new_unchecked(2) }).unwrap();
		let (g_rows, g_cols):(u16,u16) = grid.dims().into();
		let orig_dims = Dimensions::new_from(g_rows / 2, g_cols).unwrap();
		let polys = grid.make_polynomial_grid().unwrap();
		let commits = polys.commitments(&*PMP)
			.unwrap()
			.iter()
			.map(|c| c.to_bytes().unwrap())
			.collect::<Vec<_>>();

		let public_params = testnet::public_params( BlockLengthColumns(g_cols.into()));

		for xt in xts {
			let rows = grid.app_rows(xt.app_id, Some(orig_dims)).unwrap().unwrap();
			let mut row_elems = vec![None; g_rows.into()];
			for (i, data) in &rows {
				row_elems[*i] = Some(data.iter().flat_map(|s| s.to_bytes().unwrap()).collect());
			}
			let first_index = rows.iter().map(|(i, _)| *i).min().unwrap();
			row_elems.remove(first_index);

			let extended_dims = orig_dims.transpose();
			let (_, missing) = verify_equality(&public_params, &commits, &row_elems,&grid.lookup,extended_dims,xt.app_id).unwrap();
			prop_assert!(!missing.is_empty());
		}
	}
}

#[test_case( vec![1;4]; "All values are non-zero but same")]
#[test_case( vec![0;4]; "All values are zero")]
#[test_case( vec![0,5,2,1]; "All values are different")]
fn test_zero_deg_poly_commit(row_values: Vec<u8>) {
	// There are two main cases that generate a zero degree polynomial. One is for data that is non-zero, but the same.
	// The other is for all-zero data. They differ, as the former yields a polynomial with one coefficient, and latter generates zero coefficients.
	let len = row_values.len();
	let row = row_values
		.iter()
		.map(|val| pad_to_bls_scalar(&[*val]).unwrap())
		.collect::<Vec<_>>();

	//let ae = AppExtrinsic { 0.into(), vec![}
	let ev = EvaluationGrid {
		lookup: Default::default(), // Shouldn't need to care about this
		evals: DMatrix::from_row_iterator(len, 1, row.into_iter()).transpose(),
	};

	println!("Row: {:?}", ev.evals);

	let pg = ev.make_polynomial_grid().unwrap();
	println!("Poly: {:?}", pg.inner[0]);
	let commitment = pg.commitment(&*PMP, 0).unwrap().to_bytes().unwrap();

	for x in 0..len {
		// Randomly chosen cell to prove, probably should test all of them
		let cell = Cell {
			col: BlockLengthColumns(x.try_into().unwrap()),
			row: BlockLengthRows(0),
		};

		let proof = pg.proof(&*PMP, &cell).unwrap();

		let proof_bytes = proof.to_bytes().unwrap();
		let cell_bytes = ev.get(0usize, x).unwrap().to_bytes().unwrap();
		let content = [&proof_bytes[..], &cell_bytes[..]].concat();
		let dims = Dimensions::new(1, 4).unwrap();
		let cell = kate_recovery::data::Cell {
			position: Position {
				row: 0,
				col: x as u16,
			},
			content: content.try_into().unwrap(),
		};
		let verification = kate_recovery::proof::verify(
			&kate_recovery::testnet::public_params(256),
			dims,
			&commitment,
			&cell,
		);
		assert!(verification.is_ok());
		assert!(verification.unwrap())
	}
}
