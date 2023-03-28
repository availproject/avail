use da_types::{AppExtrinsic, DataLookup, DataLookupIndexItem};
use dusk_bytes::Serializable;
use dusk_plonk::prelude::BlsScalar;
use hex_literal::hex;
use kate_grid::{Dimensions, Grid, IntoColumnMajor, IntoRowMajor};
use kate_recovery::{
	com::{app_specific_cells, decode_app_extrinsics, reconstruct_extrinsics},
	data::DataCell,
};

use crate::{
	config::DATA_CHUNK_SIZE,
	gridgen::{
		tests::{app_data_index_from_lookup, sample_cells},
		EvaluationGrid,
	},
	Seed,
};

#[test]
fn newapi_test_flatten_block() {
	let extrinsics: Vec<AppExtrinsic> = vec![
		AppExtrinsic {
			app_id: 0.into(),
			data: (1..=29).collect(),
		},
		AppExtrinsic {
			app_id: 1.into(),
			data: (1..=30).collect(),
		},
		AppExtrinsic {
			app_id: 2.into(),
			data: (1..=31).collect(),
		},
		AppExtrinsic {
			app_id: 3.into(),
			data: (1..=60).collect(),
		},
	];

	let expected_dims = Dimensions::new_unchecked(16, 1);
	let evals = EvaluationGrid::from_extrinsics(extrinsics, 4, 256, 256, Seed::default()).unwrap();

	let expected_index = [(0.into(), 0), (1.into(), 2), (2.into(), 4), (3.into(), 6)]
		.into_iter()
		.map(|(app_id, start)| DataLookupIndexItem { app_id, start })
		.collect::<Vec<_>>();

	let expected_lookup = DataLookup {
		size: 9,
		index: expected_index,
	};

	assert_eq!(evals.lookup, expected_lookup, "The layouts don't match");
	assert_eq!(
		evals.dims, expected_dims,
		"Dimensions don't match the expected"
	);

	let expected_data = hex!("04740102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d00800000000000000000000000000000000000000000000000000000000000000004780102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d001e80000000000000000000000000000000000000000000000000000000000000047c0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d001e1f80000000000000000000000000000000000000000000000000000000000004f00102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d001e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c00800000000000000000000000000000000000000000000000000000000000000076a04053bda0a88bda5177b86a15c3b29f559873cb481232299cd5743151ac004b2d63ae198e7bb0a9011f28e473c95f4013d7d53ec5fbc3b42df8ed101f6d00e831e52bfb76e51cca8b4e9016838657edfae09cb9a71eb219025c4c87a67c004aaa86f20ac0aa792bc121ee42e2c326127061eda15599cb5db3db870bea5a00ecf353161c3cb528b0c5d98050c4570bfc942d8b19ed7b0cbba5725e03e5f000b7e30db36b6df82ac151f668f5f80a5e2a9cac7c64991dd6a6ce21c060175800edb9260d2a86c836efc05f17e5c59525e404c6a93d051651fe2e4eefae281300");

	let data = evals
		.evals
		.inner()
		.iter()
		.flat_map(|s| s.to_bytes())
		.collect::<Vec<_>>();
	assert_eq!(data, expected_data, "Data doesn't match the expected data");
}

#[test]
fn newapi_test_extend_data_matrix() {
	// This test expects this result in column major
	let expected_result = vec![
		hex!("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e00"),
		hex!("bc1c6b8b4b02ca677b825ec9dace9aa706813f3ec47abdf9f03c680f4468555e"),
		hex!("7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a00"),
		hex!("c16115f73784be22106830c9bc6bbb469bf5026ee80325e403efe5ccc3f55016"),
		hex!("1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d00"),
		hex!("db3b8aaa6a21e9869aa17de8f9edb9c625a05e5de399dc18105c872e6387745e"),
		hex!("9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b900"),
		hex!("e080341657a3dd412f874fe8db8ada65ba14228d07234403230e05ece2147016"),
		hex!("3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c00"),
		hex!("fa5aa9c9894008a6b9c09c07190dd9e544bf7d7c02b9fb372f7ba64d82a6935e"),
		hex!("babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d800"),
		hex!("ff9f533576c2fc604ea66e07fba9f984d93341ac26426322422d240b02348f16"),
		hex!("5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b00"),
		hex!("197ac8e8a85f27c5d8dfbb26382cf80464de9c9b21d81a574e9ac56ca1c5b25e"),
		hex!("d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f700"),
		hex!("1ebf725495e11b806dc58d261ac918a4f85260cb45618241614c432a2153ae16"),
	]
	.into_iter()
	.map(|e| BlsScalar::from_bytes(e.as_slice().try_into().unwrap()).unwrap())
	.collect::<Vec<_>>()
	.into_column_major(4, 4)
	.unwrap()
	.to_row_major();

	let block_dims = Dimensions::new_unchecked(4, 2);
	let scalars = (0..=247)
		.collect::<Vec<u8>>()
		.chunks_exact(DATA_CHUNK_SIZE)
		.flat_map(crate::gridgen::pad_to_bls_scalar)
		.collect::<Vec<_>>();

	let grid = EvaluationGrid {
		lookup: DataLookup::default(),
		evals: scalars
			.into_row_major(block_dims.width(), block_dims.height())
			.unwrap(),
		dims: block_dims,
	};
	let extend = grid.extend_columns(2).unwrap();

	assert_eq!(extend.evals.inner(), expected_result.inner());
}

#[test]
fn test_decode_app_extrinsics() {
	let app_id_1_data = br#""This is mocked test data. It will be formatted as a matrix of BLS scalar cells and then individual columns 
get erasure coded to ensure redundancy."#;

	let app_id_2_data =
		br#""Let's see how this gets encoded and then reconstructed by sampling only some data."#;

	let data = [vec![0], app_id_1_data.to_vec(), app_id_2_data.to_vec()];

	let hash = Seed::default();
	let xts = (0..=2)
		.zip(data)
		.map(|(app_id, data)| AppExtrinsic {
			app_id: app_id.into(),
			data,
		})
		.collect::<Vec<_>>();

	let grid = EvaluationGrid::from_extrinsics(xts.clone(), 4, 32, 4, hash)
		.unwrap()
		.extend_columns(2)
		.unwrap();

	let index = app_data_index_from_lookup(&grid.lookup);
	let bdims =
		kate_recovery::matrix::Dimensions::new(grid.dims.height() as u16, grid.dims.width() as u16)
			.unwrap();
	for xt in &xts {
		let positions = app_specific_cells(&index, &bdims, xt.app_id.0).unwrap();
		let cells = positions
			.iter()
			.map(|pos| DataCell {
				position: pos.clone(),
				data: grid
					.evals
					.get(pos.col as usize, pos.row as usize)
					.unwrap()
					.to_bytes(),
			})
			.collect::<Vec<_>>();
		let data = &decode_app_extrinsics(&index, &bdims, cells, xt.app_id.0).unwrap()[0];
		assert_eq!(data, &xt.data);
	}

	assert!(matches!(
		decode_app_extrinsics(&index, &bdims, vec![], 0),
		Err(kate_recovery::com::ReconstructionError::MissingCell { .. })
	));
}

#[test]
fn test_extend_mock_data() {
	let orig_data = br#"This is mocked test data. It will be formatted as a matrix of BLS scalar cells and then individual columns 
get erasure coded to ensure redundancy.
Let's see how this gets encoded and then reconstructed by sampling only some data."#;
	let exts = vec![AppExtrinsic::from(orig_data.to_vec())];

	// The hash is used for seed for padding the block to next power of two value
	let hash = Seed::default();
	let grid = EvaluationGrid::from_extrinsics(exts.clone(), 4, 128, 2, hash)
		.unwrap()
		.extend_columns(2)
		.unwrap();

	let cols = sample_cells(&grid, None);
	let bdims =
		kate_recovery::matrix::Dimensions::new(grid.dims.height() as u16, grid.dims.width() as u16)
			.unwrap();

	let index = app_data_index_from_lookup(&grid.lookup);
	let res = reconstruct_extrinsics(&index, &bdims, cols).unwrap();
	let s = String::from_utf8_lossy(res[0].1[0].as_slice());

	assert_eq!(res[0].1[0], orig_data);

	eprintln!("Decoded: {}", s);
}
