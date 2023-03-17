use core::marker::PhantomData;

use codec::Encode;
use da_primitives::asdr::{AppExtrinsic, AppId, DataLookup, DataLookupIndexItem};
use dusk_bytes::Serializable;
use dusk_plonk::{
	commitment_scheme::kzg10::commitment::Commitment,
	fft::{EvaluationDomain, Polynomial},
	prelude::{BlsScalar, CommitKey},
};
use kate_grid::{AsColumnMajor, AsRowMajor, Dimensions, Extension, Grid, RowMajor};
use kate_recovery::config::PADDING_TAIL_VALUE;
use poly_multiproof::{m1_blst::M1NoPrecomp, merlin::Transcript};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

use crate::{
	com::{Cell, Error},
	config::DATA_CHUNK_SIZE,
	Seed,
};

pub struct EvaluationGrid {
	pub lookup: DataLookup,
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

		// make the index of app info
		let mut start = 0u32;
		let mut index = vec![];
		for (app_id, scalars) in &encoded {
			index.push(DataLookupIndexItem {
				app_id: *app_id,
				start,
			});
			start += scalars.len() as u32; // next item should start after current one
		}

		// Flatten the grid
		let mut grid = encoded
			.into_iter()
			.flat_map(|(_, scalars)| scalars)
			.collect::<Vec<_>>();

		let lookup = DataLookup {
			size: grid.len() as u32,
			index,
		};

		// Fit the grid to the desired grid size
		let dims = get_block_dims(grid.len(), min_width, max_width, max_height)?;
		let mut rng = ChaChaRng::from_seed(rng_seed);
		while grid.len() != dims.n_cells() {
			let rnd_values: [u8; BlsScalar::SIZE - 1] = rng.gen();
			// TODO: can we just use zeros instead?
			grid.push(pad_to_bls_scalar(&rnd_values)?);
		}

		Ok(EvaluationGrid {
			lookup,
			evals: grid
				.as_row_major(dims.width(), dims.height())
				.ok_or(Error::DimensionsMismatch)?,
			dims,
		})
	}

	pub fn row(&self, y: usize) -> Option<&[BlsScalar]> {
		self.evals.row(y)
	}

	/// Returns the start/end indices of the given app id *for the non-extended grid*
	fn app_data_indices(&self, app_id: &AppId) -> Option<(usize, usize)> {
		if self.lookup.size == 0 {
			// Empty block, short circuit.
			return None;
		}
		let (i, start_index) = self
			.lookup
			.index
			.iter()
			.enumerate()
			.find(|(_i, item)| &item.app_id == app_id)
			.map(|(i, item)| (i, item.start as usize))?;
		let end_index = self
			.lookup
			.index
			.get(i + 1)
			.map(|elem| elem.start)
			.unwrap_or(self.lookup.size) as usize;
		Some((start_index, end_index))
	}

	/// Returns a list of `(index, row)` pairs for the underlying rows of an application.
	/// Returns `None` if the `app_id` cannot be found, or if the provided `orig_dims` are invalid.
	pub fn app_rows(
		&self,
		app_id: &AppId,
		orig_dims: Option<&Dimensions>,
	) -> Option<Vec<(usize, Vec<BlsScalar>)>> {
		let orig_dims = orig_dims.unwrap_or(&self.dims);
		if !orig_dims.divides(&self.dims) {
			return None;
		}
		let h_mul = self.dims.height() / orig_dims.height();

		let (start_ind, end_ind) = self.app_data_indices(app_id)?;
		let (_, start_y) = RowMajor::<()>::ind_to_coord(&orig_dims, start_ind);
		let (_, end_y) = RowMajor::<()>::ind_to_coord(&orig_dims, end_ind - 1); // Find y of last cell elt
		let (new_start_y, new_end_y) = (start_y * h_mul, end_y * h_mul);

		(new_start_y..=new_end_y)
			.step_by(h_mul)
			.map(|y| self.evals.row(y).map(|a| (y, a.to_vec())))
			.collect()
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
			lookup: self.lookup.clone(),
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

	pub fn commitment(&self, srs: &CommitKey, row: usize) -> Result<Commitment, Error> {
		self.inner
			.get(row)
			.ok_or(Error::CellLenghtExceeded)
			.and_then(|poly| srs.commit(&poly).map_err(|e| Error::PlonkError(e)))
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

fn convert_bls(dusk: &dusk_plonk::bls12_381::BlsScalar) -> poly_multiproof::m1_blst::Fr {
	poly_multiproof::m1_blst::Fr {
		0: poly_multiproof::ark_ff::BigInt(dusk.0.clone()),
		1: PhantomData,
	}
}

#[derive(Debug, Clone)]
pub struct Multiproof {
	pub proof: poly_multiproof::m1_blst::Proof,
	pub evals: Vec<Vec<poly_multiproof::m1_blst::Fr>>,
	pub block: CellBlock,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellBlock {
	start_x: usize,
	start_y: usize,
	end_x: usize,
	end_y: usize,
}

/// Computes the `x, y`-th multiproof block of a grid of size `grid_dims`.
/// `mp_grid_dims` is the size of the multiproof grid, which `x,y` lies in.
/// For example, a 256x256 grid could be converted to a 4x4 target size multiproof grid, by making 16 multiproofs
/// of size 64x64.
pub fn multiproof_block(
	x: usize,
	y: usize,
	grid_dims: &Dimensions,
	target_dims: &Dimensions,
) -> Option<CellBlock> {
	let mp_grid_dims = multiproof_dims(grid_dims, target_dims)?;
	if x >= mp_grid_dims.width() || y >= mp_grid_dims.height() {
		return None;
	}

	let block_width = grid_dims.width() / mp_grid_dims.width();
	let block_height = grid_dims.height() / mp_grid_dims.height();
	Some(CellBlock {
		start_x: x * block_width,
		start_y: y * block_height,
		end_x: (x + 1) * block_width,
		end_y: (y + 1) * block_height,
	})
}

/// Dimensions of the multiproof grid. These are guarenteed to cleanly divide `grid_dims`.
/// `target_dims` must cleanly divide `grid_dims`.
pub fn multiproof_dims(grid_dims: &Dimensions, target_dims: &Dimensions) -> Option<Dimensions> {
	let target_width = core::cmp::min(grid_dims.width(), target_dims.width());
	let target_height = core::cmp::min(grid_dims.height(), target_dims.height());
	if grid_dims.width() % target_width != 0 || grid_dims.height() % target_height != 0 {
		return None;
	}
	Some(Dimensions::new(target_width, target_height))
}

pub fn get_block_dims(
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

	#[test_case(256, 256,  64,  16 => Some((64, 16)))]
	#[test_case(256, 256,  32,  32 => Some((32, 32)))]
	#[test_case(256, 256,   7,  32 => None)]
	#[test_case(32 ,  32,  32,  32 => Some((32, 32)))]
	#[test_case(256,   8,  32,  32 => Some((32, 8)))]
	#[test_case(4  ,   1,  32,  32 => Some((4, 1)))]
	fn test_multiproof_dims(
		grid_w: usize,
		grid_h: usize,
		target_w: usize,
		target_h: usize,
	) -> Option<(usize, usize)> {
		multiproof_dims(
			&Dimensions::new(grid_w, grid_h),
			&Dimensions::new(target_w, target_h),
		)
		.map(|i| (i.width(), i.height()))
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
			use poly_multiproof::ark_serialize::CanonicalSerialize;
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
	use kate_grid::Grid;
	use kate_recovery::com::reconstruct_extrinsics;
	use kate_recovery::data::Cell as DCell;
	use kate_recovery::index::AppDataIndex;
	use kate_recovery::matrix::Position as DPosition;
	use proptest::prelude::*;
	use proptest::{collection, sample::size_range, strategy::Strategy};
	use rand::distributions::Uniform;
	use rand::prelude::Distribution;

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

		let grid = EvaluationGrid {
			lookup: DataLookup::default(),
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

	fn app_extrinsic_strategy() -> impl Strategy<Value = AppExtrinsic> {
		(
			any::<u32>(),
			any_with::<Vec<u8>>(size_range(1..2048).lift()),
		)
			.prop_map(|(app_id, data)| AppExtrinsic {
				app_id: app_id.into(),
				data,
			})
	}

	fn app_extrinsics_strategy() -> impl Strategy<Value = Vec<AppExtrinsic>> {
		collection::vec(app_extrinsic_strategy(), size_range(1..16)).prop_map(|xts| {
			let mut new_xts = xts;
			new_xts.sort_by(|a1, a2| a1.app_id.cmp(&a2.app_id));
			new_xts
		})
	}

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

	// This copied method is still confusing to me... it just accumulates the size but skips over
	// the app_id 0 size? not sure what's going on...
	fn app_data_index_from_lookup(lookup: &DataLookup) -> AppDataIndex {
		AppDataIndex {
			size: lookup.size,
			index: lookup.index.iter().map(|e| (e.app_id.0, e.start)).collect(),
		}
	}

	proptest! {
	#![proptest_config(ProptestConfig::with_cases(5))]
	#[test]
	fn newapi_test_build_and_reconstruct(exts in app_extrinsics_strategy())  {
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
		let commitments = polys.commitments(&pp.commit_key()).unwrap();
		let indices = (0..dims.width()).flat_map(|x| (0..dims.height()).map(move |y| (x, y))).collect::<Vec<_>>();

		// Sample some number 10 of the indices, all is too slow for tests...
		let mut rng = ChaChaRng::from_seed(RNG_SEED);
		let sampled = Uniform::from(0..indices.len()).sample_iter(&mut rng).take(10).map(|i| indices[i].clone());
		for (x, y) in sampled {
			let cell = Cell { row: (y as u32).into(), col: (x as u32).into() };
			let proof = polys.proof(&pp.commit_key(), &cell).unwrap();
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

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(1))]
		#[test]
		fn newapi_commitments_verify(ref exts in app_extrinsics_strategy())  {
			//let (layout, commitments, dims, matrix) = par_build_commitments(BlockLengthRows(64), BlockLengthColumns(16), 32, xts, Seed::default()).unwrap();
			let grid = EvaluationGrid::from_extrinsics(exts.clone(), 4, 16, 64, Seed::default()).unwrap().extend_columns(2).unwrap();
			let orig_dims = Dimensions::new(grid.dims.width(), grid.dims.height() / 2);
			let polys = grid.make_polynomial_grid().unwrap();
			let commits = polys.commitments(&pp().commit_key())
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
	}
}
