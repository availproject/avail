use avail_core::{AppExtrinsic, AppId, DataLookup};
use hex_literal::hex;
use kate_recovery::{
	com::{app_specific_cells, decode_app_extrinsics, reconstruct_extrinsics},
	data::DataCell,
	matrix::Dimensions,
};
use nalgebra::base::DMatrix;
use poly_multiproof::traits::AsBytes;

use crate::{
	config::DATA_CHUNK_SIZE,
	gridgen::{
		tests::{app_data_index_from_lookup, sample_cells},
		ArkScalar, EvaluationGrid,
	},
	Seed,
};
use core::num::NonZeroU16;

#[test]
fn newapi_test_flatten_block() {
	let extrinsics: Vec<AppExtrinsic> = vec![
		AppExtrinsic::new(AppId(0), (1..=29).collect()),
		AppExtrinsic::new(AppId(1), (1..=30).collect()),
		AppExtrinsic::new(AppId(2), (1..=31).collect()),
		AppExtrinsic::new(AppId(3), (1..=60).collect()),
	];

	let expected_dims = Dimensions::new_from(1, 16).unwrap();
	let evals = EvaluationGrid::from_extrinsics(extrinsics, 4, 256, 256, Seed::default()).unwrap();

	let expected_lookup = DataLookup::new_from_id_lenght(
		[(AppId(0), 2), (AppId(1), 2), (AppId(2), 2), (AppId(3), 3)].into_iter(),
	)
	.unwrap();

	assert_eq!(evals.lookup, expected_lookup, "The layouts don't match");
	assert_eq!(
		evals.dims(),
		expected_dims,
		"Dimensions don't match the expected"
	);

	let expected_data = hex!("04740102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d00800000000000000000000000000000000000000000000000000000000000000004780102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d001e80000000000000000000000000000000000000000000000000000000000000047c0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d001e1f80000000000000000000000000000000000000000000000000000000000004f00102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d001e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c00800000000000000000000000000000000000000000000000000000000000000076b8e0ada0f13d90405d6ae55386bd28bdd219b8a08ded1aa836efcc8b770d00da41597c5157488d7724e03fb8d84a376a43b8f41518a11cc387b669b2ee65009f07e7be5551387a98ba977c732d080dcb0f29a048e3656912c6533e32ee7a0029b721769ce64e43d57133b074d839d531ed1f28510afb45ace10a1f4b794d002d09a0e663266ce1ae7ed1081968a0758e718e997bd362c6b0c34634a9a0b300012737681f7b5d0f281e3afde458bc1e73d2d313c9cf94c05ff3716240a248001320a058d7b3566bd520daaa3ed2bf0ac5b8b120fb852773c3639734b45c9100");

	let data = evals
		.evals
		.row_iter()
		.flat_map(|row| {
			row.iter()
				.flat_map(|s| s.to_bytes().unwrap())
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	assert_eq!(data, expected_data, "Data doesn't match the expected data");
}

#[test]
fn newapi_test_extend_data_matrix() {
	// This test expects this result in column major
	let expected_data = vec![
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
	.iter()
	.map(ArkScalar::from_bytes)
	.collect::<Result<Vec<_>, _>>()
	.expect("Invalid Expected result");

	let expected_result = DMatrix::from_column_slice(4, 4, &expected_data);

	let scalars = (0..=247)
		.collect::<Vec<u8>>()
		.chunks_exact(DATA_CHUNK_SIZE)
		.flat_map(crate::gridgen::pad_to_bls_scalar)
		.collect::<Vec<_>>();

	let grid = EvaluationGrid {
		lookup: DataLookup::default(),
		evals: DMatrix::from_row_iterator(2, 4, scalars.into_iter()),
	};
	let extend = grid
		.extend_columns(unsafe { NonZeroU16::new_unchecked(2) })
		.unwrap();

	assert_eq!(extend.evals, expected_result);
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
		.map(|(id, data)| AppExtrinsic::new(AppId(id), data))
		.collect::<Vec<_>>();

	let grid = EvaluationGrid::from_extrinsics(xts.clone(), 4, 32, 4, hash)
		.unwrap()
		.extend_columns(unsafe { NonZeroU16::new_unchecked(2) })
		.unwrap();

	let index = app_data_index_from_lookup(&grid.lookup);
	let bdims = grid.dims();
	for xt in &xts {
		let positions = app_specific_cells(&index, bdims, xt.app_id.0).unwrap();
		let cells = positions
			.iter()
			.map(|pos| DataCell {
				position: pos.clone(),
				data: grid
					.evals
					.get((pos.row as usize, pos.col as usize))
					.unwrap()
					.to_bytes()
					.unwrap(),
			})
			.collect::<Vec<_>>();
		let data = &decode_app_extrinsics(&index, bdims, cells, xt.app_id.0).unwrap()[0];
		assert_eq!(data, &xt.data);
	}

	assert!(matches!(
		decode_app_extrinsics(&index, bdims, vec![], 0),
		Err(kate_recovery::com::ReconstructionError::MissingCell { .. })
	));
}

#[test]
fn test_extend_mock_data() {
	let orig_data = r#"This is mocked test data. It will be formatted as a matrix of BLS scalar cells and then individual columns 
get erasure coded to ensure redundancy.
Let's see how this gets encoded and then reconstructed by sampling only some data."#;
	let exts = vec![AppExtrinsic::from(orig_data.as_bytes().to_vec())];

	// The hash is used for seed for padding the block to next power of two value
	let hash = Seed::default();
	let grid = EvaluationGrid::from_extrinsics(exts.clone(), 4, 128, 2, hash)
		.unwrap()
		.extend_columns(unsafe { NonZeroU16::new_unchecked(2) })
		.unwrap();

	let cols = sample_cells(&grid, None);
	let bdims = grid.dims();

	let index = app_data_index_from_lookup(&grid.lookup);
	let res = reconstruct_extrinsics(&index, bdims, cols).unwrap();
	let s = String::from_utf8_lossy(res[0].1[0].as_slice());

	assert_eq!(s, orig_data);
	assert_eq!(res[0].1[0], orig_data.as_bytes());

	eprintln!("Decoded: {}", s);
}
