use core::marker::PhantomData;

use codec::Encode;
use da_primitives::asdr::{AppExtrinsic, AppId};
use dusk_bytes::Serializable;
use dusk_plonk::{
	commitment_scheme::kzg10::commitment::Commitment,
	fft::{EvaluationDomain, Polynomial},
	prelude::{BlsScalar, CommitKey},
};
use kate_grid::{AsColumnMajor, AsRowMajor, Dimensions, Extension, RowMajor};
use kate_recovery::config::PADDING_TAIL_VALUE;
use merlin::Transcript;
use poly_multiproof::m1_blst::M1NoPrecomp;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

use crate::{
	com::{Cell, Error, XtsLayout},
	config::DATA_CHUNK_SIZE,
	Seed,
};

pub struct EvaluationGrid {
	pub layout: XtsLayout,
	pub evals: RowMajor<BlsScalar>,
	pub dims: Dimensions,
}

impl EvaluationGrid {
	/// From the app extrinsics, create a data grid of Scalars
	pub fn from_extrinsics(
		mut extrinsics: Vec<AppExtrinsic>,
		min_width: usize,
		max_width: usize,
		max_height: usize,
		rng_seed: Seed,
	) -> Result<Self, Error> {
		// Group extrinsics by app id, also sorted by app id.
		extrinsics.sort_by(|a, b| a.app_id.cmp(&b.app_id));
		let grouped =
			extrinsics
				.iter()
				.fold::<Vec<(AppId, Vec<Vec<_>>)>, _>(vec![], |mut acc, e| {
					match acc.last_mut() {
						Some((app_id, data)) if e.app_id == *app_id => data.push(e.data.clone()),
						None | Some(_) => acc.push((e.app_id, vec![e.data.clone()])),
					}
					acc
				});

		// Convert each grup of extrinsics into scalars
		let encoded = grouped
			.into_iter()
			.map(|(id, datas)| {
				let mut enc = datas.encode();
				enc.push(PADDING_TAIL_VALUE); // TODO: remove 9797 padding stuff
				enc.chunks(DATA_CHUNK_SIZE)
					.map(|c| pad_to_bls_scalar(c))
					.collect::<Result<Vec<_>, _>>()
					.map(|scalars| (id, scalars))
			})
			.collect::<Result<Vec<_>, _>>()?;

		// Get the layout of each app id's start
		let layout = encoded
			.iter()
			.map(|(id, data)| (*id, data.len() as u32))
			.collect::<Vec<_>>();

		// Flatten the grid
		let mut grid = encoded
			.into_iter()
			.flat_map(|(_, scalars)| scalars)
			.collect::<Vec<_>>();

		// Fit the grid to the desired grid size
		let dims = get_block_dims(grid.len(), min_width, max_width, max_height)?;
		let mut rng = ChaChaRng::from_seed(rng_seed);
		while grid.len() != dims.n_cells() {
			let rnd_values: [u8; BlsScalar::SIZE - 1] = rng.gen();
			// TODO: can we just use zeros instead?
			grid.push(pad_to_bls_scalar(&rnd_values)?);
		}

		Ok(EvaluationGrid {
			layout,
			evals: grid
				.as_row_major(dims.width(), dims.height())
				.ok_or(Error::DimensionsMismatch)?,
			dims,
		})
	}

	pub fn extend_columns(&self, extension_factor: usize) -> Result<Self, Error> {
		let new_dims = self.dims.extend(Extension::height(extension_factor));

		let domain = EvaluationDomain::new(self.dims.height())?;
		let domain_new = EvaluationDomain::new(new_dims.height())?;
		if domain_new.size() != new_dims.height() {
			// TODO: throw a reasonable error
			return Err(Error::CellLenghtExceeded);
		}

		let new_evals = self
			.evals
			.columns()
			.flat_map(|(_x, col)| {
				// put elts into a new column
				let mut ext_col = Vec::with_capacity(domain_new.size());
				col.for_each(|s| ext_col.push(s.clone()));
				// ifft, resize, fft
				domain.ifft_slice(&mut ext_col);
				ext_col.resize(domain_new.size(), BlsScalar::zero());
				domain_new.fft_slice(&mut ext_col);
				ext_col
			})
			.collect::<Vec<_>>()
			.as_column_major(new_dims.width(), new_dims.height())
			.expect("Each column should be expanded to news dims")
			.to_row_major();

		Ok(Self {
			layout: self.layout.clone(),
			evals: new_evals,
			dims: new_dims,
		})
	}

	pub fn make_polynomial_grid(&self) -> Result<PolynomialGrid, Error> {
		let domain = EvaluationDomain::new(self.dims.width())?;
		Ok(PolynomialGrid {
			dims: self.dims.clone(),
			points: domain.elements().collect(),
			inner: self
				.evals
				.rows()
				.map(|(_, row)| Polynomial {
					coeffs: domain.ifft(row),
				})
				.collect::<Vec<_>>(),
		})
	}
}

pub struct PolynomialGrid {
	inner: Vec<Polynomial>,
	points: Vec<BlsScalar>,
	dims: Dimensions,
}

impl PolynomialGrid {
	pub fn commitments(&self, srs: &CommitKey) -> Result<Vec<Commitment>, Error> {
		self.inner
			.iter()
			.map(|poly| srs.commit(&poly).map_err(|e| Error::PlonkError(e)))
			.collect()
	}

	pub fn proof(&self, srs: &CommitKey, cell: &Cell) -> Result<Commitment, Error> {
		let x = cell.col.0 as usize;
		let y = cell.row.0 as usize;
		// TODO: better error msg
		let poly = self.inner.get(y).ok_or(Error::CellLenghtExceeded)?;
		let witness = srs.compute_single_witness(poly, &self.points[x]);
		Ok(srs.commit(&witness)?)
	}

	pub fn multiproof(
		&self,
		srs: &M1NoPrecomp,
		cell: &Cell,
		eval_grid: &EvaluationGrid,
		target_dims: &Dimensions,
	) -> Result<Multiproof, Error> {
		use poly_multiproof::traits::PolyMultiProofNoPrecomp;
		// TODO: useful error
		let block = multiproof_block(
			cell.col.0 as usize,
			cell.row.0 as usize,
			&self.dims,
			target_dims,
		)
		.ok_or(Error::CellLenghtExceeded)?;
		let polys = self.inner[block.start_y..block.end_y]
			.iter()
			.map(|s| s.coeffs.iter().map(convert_bls).collect::<Vec<_>>())
			.collect::<Vec<_>>();
		let evals = (block.start_y..block.end_y)
			.map(|y| {
				eval_grid.evals.row(y).expect("Already bounds checked")[block.start_x..block.end_x]
					.iter()
					.map(convert_bls)
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		let points = &self.points[block.start_x..block.end_x]
			.iter()
			.map(convert_bls)
			.collect::<Vec<_>>();
		//let eval_slices = eval_grid.evals.rows().map(|(_, row)| &row[]).collect::<Vec<_>>();

		let mut ts = Transcript::new(b"avail-mp");
		let proof = srs
			.open(&mut ts, &evals, &polys, &points)
			.expect("TODO: real error msg");
		Ok(Multiproof {
			proof,
			evals,
			block,
		})
	}
}

fn convert_bls(dusk: &dusk_plonk::bls12_381::BlsScalar) -> ark_bls12_381::Fr {
	ark_bls12_381::Fr {
		0: ark_ff::BigInt(dusk.0.clone()),
		1: PhantomData,
	}
}

#[derive(Debug, Clone)]
pub struct Multiproof {
	pub proof: poly_multiproof::m1_blst::Proof,
	pub evals: Vec<Vec<ark_bls12_381::Fr>>,
	pub block: CellBlock,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellBlock {
	start_x: usize,
	start_y: usize,
	end_x: usize,
	end_y: usize,
}
fn multiproof_block(
	x: usize,
	y: usize,
	grid_dims: &Dimensions,
	target_dims: &Dimensions,
) -> Option<CellBlock> {
	let target_width = core::cmp::min(grid_dims.width(), target_dims.width());
	let target_height = core::cmp::min(grid_dims.height(), target_dims.height());
	dbg!(&target_width, target_height);
	dbg!(&x, &y);
	if x >= target_width || y >= target_height {
		return None;
	}

	if grid_dims.width() % target_width != 0 || grid_dims.height() % target_height != 0 {
		return None;
	}

	let block_width = grid_dims.width() / target_width;
	let block_height = grid_dims.height() / target_height;
	Some(CellBlock {
		start_x: x * block_width,
		start_y: y * block_height,
		end_x: (x + 1) * block_width,
		end_y: (y + 1) * block_height,
	})
}

fn get_block_dims(
	n_scalars: usize,
	min_width: usize,
	max_width: usize,
	max_height: usize,
) -> Result<Dimensions, Error> {
	// Less than max_width wide block
	if n_scalars < max_width {
		let current_width = n_scalars;
		// Don't let the width get lower than the minimum provided
		let width = core::cmp::max(round_up_power_of_2(current_width), min_width);
		Ok(Dimensions::new(width, 1))
	} else {
		let width = max_width;
		let current_height = round_up_to_multiple(n_scalars, width) / width;
		// Round the height up to a power of 2 for ffts
		let height = round_up_power_of_2(current_height);
		// Error if height too big
		if height > max_height {
			return Err(Error::BlockTooBig);
		}
		Ok(Dimensions::new(width, height))
	}
}

fn round_up_to_multiple(input: usize, multiple: usize) -> usize {
	let n_multiples = (input + multiple - 1) / multiple;
	n_multiples * multiple
}

fn pad_to_bls_scalar(a: impl AsRef<[u8]>) -> Result<BlsScalar, Error> {
	if a.as_ref().len() > DATA_CHUNK_SIZE {
		todo!()
	}
	let mut buf = [0u8; BlsScalar::SIZE];
	buf[0..a.as_ref().len()].copy_from_slice(a.as_ref());
	//TODO: better error type
	BlsScalar::from_bytes(&buf).map_err(|_| Error::CellLenghtExceeded)
}

// Round up. only valid for positive integers
fn round_up_power_of_2(mut v: usize) -> usize {
	if v == 0 {
		return 1;
	}
	v -= 1;
	v |= v >> 1;
	v |= v >> 2;
	v |= v >> 4;
	v |= v >> 8;
	v |= v >> 16;
	v += 1;
	return v;
}

#[cfg(test)]
mod tests {
	use super::*;
	use proptest::{prop_assert_eq, proptest};
	use test_case::test_case;

	// parameters that will split a 256x256 grid into pieces of size 4x16
	const TARGET: Dimensions = Dimensions::new(64, 16);
	const GRID: Dimensions = Dimensions::new(256, 256);
	fn cb(start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> CellBlock {
		CellBlock {
			start_x,
			start_y,
			end_x,
			end_y,
		}
	}
	#[test_case(0, 0 => Some(cb(0, 0, 4, 16)))]
	#[test_case(1, 0 => Some(cb(4, 0, 8, 16)))]
	#[test_case(0, 1 => Some(cb(0, 16, 4, 32)))]
	#[test_case(1, 1 => Some(cb(4, 16, 8, 32)))]
	#[test_case(64, 0 => None)]
	#[test_case(0, 16 => None)]
	fn multiproof_max_grid_size(x: usize, y: usize) -> Option<CellBlock> {
		multiproof_block(x, y, &GRID, &TARGET)
	}

	use proptest::prelude::*;
	proptest! {
		#![proptest_config(ProptestConfig {
			cases: 200, .. ProptestConfig::default()
		  })]
		#[test]
		fn test_round_up_to_multiple(i in 1..1000usize, m in 1..32usize) {
			for k in 0..m {
				let a = i * m - k;
				prop_assert_eq!(round_up_to_multiple(a, m), i * m)
			}
		}

		#[test]
		fn test_convert_bls_scalar(input: [u8; 31]) {
			use ark_serialize::CanonicalSerialize;
			let dusk = pad_to_bls_scalar(&input).unwrap();
			let ark = convert_bls(&dusk);
			let dusk_out = dusk.to_bytes();
			let mut ark_out = [0u8; 32];
			ark.serialize_compressed(&mut ark_out[..]).unwrap();
			assert_eq!(dusk_out, ark_out);
		}

	}
	#[test_case(0 => 1)]
	#[test_case(1 => 1)]
	#[test_case(2 => 2)]
	#[test_case(3 => 4)]
	#[test_case(6 => 8)]
	#[test_case(972 => 1024)]
	fn test_round_up_to_2(i: usize) -> usize {
		round_up_power_of_2(i)
	}

	#[test_case(0 => Dimensions::new(4,  1) ; "block size zero")]
	#[test_case(1 => Dimensions::new(4,  1) ; "below minimum block size")]
	#[test_case(10 => Dimensions::new(16, 1) ; "regular case")]
	#[test_case(17 => Dimensions::new(32, 1) ; "minimum overhead after 512")]
	#[test_case(256 => Dimensions::new(256, 1) ; "maximum cols")]
	#[test_case(257 => Dimensions::new(256, 2) ; "two rows")]
	#[test_case(256 * 256 => Dimensions::new(256, 256) ; "max block size")]
	#[test_case(256 * 256 + 1 => panics "BlockTooBig" ; "too much data")]
	fn test_get_block_dims(size: usize) -> Dimensions
where {
		get_block_dims(size, 4, 256, 256).unwrap()
	}
}

#[cfg(test)]
mod consistency_tests {

	use super::*;
	use crate::testnet;
	use dusk_plonk::prelude::PublicParameters;
	use hex_literal::hex;

	fn pp() -> PublicParameters {
		testnet::public_params(da_primitives::BlockLengthColumns(256))
	}

	#[test]
	fn newapi_test_build_commitments_simple_commitment_check() {
		let original_data = br#"test"#;
		let block_height = 256usize;
		let block_width = 256usize;
		let hash: Seed = [
			76, 41, 174, 145, 187, 12, 97, 32, 75, 111, 149, 209, 243, 195, 165, 10, 166, 172, 47,
			41, 218, 24, 212, 66, 62, 5, 187, 191, 129, 5, 105, 3,
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

		assert_eq!(evals.dims, Dimensions::new(4, 2));
		let expected_commitments = hex!("960F08F97D3A8BD21C3F5682366130132E18E375A587A1E5900937D7AA5F33C4E20A1C0ACAE664DCE1FD99EDC2693B8D960F08F97D3A8BD21C3F5682366130132E18E375A587A1E5900937D7AA5F33C4E20A1C0ACAE664DCE1FD99EDC2693B8D");
		assert_eq!(commits, expected_commitments);
	}

	#[test]
	fn newapi_par_build_commitments_row_wise_constant_row() {
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

		let expected_dims = Dimensions::new(16, 1);
		let evals =
			EvaluationGrid::from_extrinsics(extrinsics, 4, 256, 256, Seed::default()).unwrap();

		let expected_layout = vec![(0.into(), 2), (1.into(), 2), (2.into(), 2), (3.into(), 3)];
		assert_eq!(evals.layout, expected_layout, "The layouts don't match");
		assert_eq!(
			evals.dims, expected_dims,
			"Dimensions don't match the expected"
		);

		let expected_data = hex!("04740102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d00800000000000000000000000000000000000000000000000000000000000000004780102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d001e80000000000000000000000000000000000000000000000000000000000000047c0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d001e1f80000000000000000000000000000000000000000000000000000000000004f00102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d001e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c00800000000000000000000000000000000000000000000000000000000000000076a04053bda0a88bda5177b86a15c3b29f559873cb481232299cd5743151ac004b2d63ae198e7bb0a9011f28e473c95f4013d7d53ec5fbc3b42df8ed101f6d00e831e52bfb76e51cca8b4e9016838657edfae09cb9a71eb219025c4c87a67c004aaa86f20ac0aa792bc121ee42e2c326127061eda15599cb5db3db870bea5a00ecf353161c3cb528b0c5d98050c4570bfc942d8b19ed7b0cbba5725e03e5f000b7e30db36b6df82ac151f668f5f80a5e2a9cac7c64991dd6a6ce21c060175800edb9260d2a86c836efc05f17e5c59525e404c6a93d051651fe2e4eefae281300");

		let data = evals
			.evals
			.inner
			.into_iter()
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
		.as_column_major(4, 4)
		.unwrap()
		.to_row_major()
		.inner;

		let block_dims = Dimensions::new(4, 2);
		let scalars = (0..=247)
			.collect::<Vec<u8>>()
			.chunks_exact(DATA_CHUNK_SIZE)
			.flat_map(|chunk| pad_to_bls_scalar(chunk))
			.collect::<Vec<_>>();
		dbg!(scalars.len());

		let grid = EvaluationGrid {
			layout: vec![],
			evals: scalars
				.as_row_major(block_dims.width(), block_dims.height())
				.unwrap(),
			dims: block_dims,
		};
		let extend = grid.extend_columns(2).unwrap();

		for i in 0..expected_result.len() {
			let e = expected_result[i];
			for j in 0..expected_result.len() {
				let r = extend.evals.inner[j];
				if e == r {
					eprintln!("Eq: {} {}", i, j);
				}
			}
		}

		assert_eq!(extend.evals.inner, expected_result);
	}
}
