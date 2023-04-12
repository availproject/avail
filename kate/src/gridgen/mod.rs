use core::{marker::PhantomData, num::NonZeroUsize};

use codec::Encode;
use da_types::{AppExtrinsic, AppId, DataLookup, DataLookupIndexItem};
use dusk_bytes::Serializable;
use dusk_plonk::{
	fft::{EvaluationDomain, Evaluations, Polynomial},
	prelude::{BlsScalar, CommitKey},
};
use kate_grid::{Dimensions, Extension, Grid, IntoColumnMajor, IntoRowMajor, RowMajor};
use kate_recovery::config::PADDING_TAIL_VALUE;
use poly_multiproof::{m1_blst::M1NoPrecomp, merlin::Transcript, traits::AsBytes};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

use crate::{
	com::{Cell, Error},
	config::DATA_CHUNK_SIZE,
	Seed,
};

pub use dusk_plonk::commitment_scheme::kzg10::commitment::Commitment;

pub type ArkScalar = crate::pmp::m1_blst::Fr;
pub type MpCommitment = crate::pmp::Commitment<poly_multiproof::m1_blst::Bls12_381>;

#[cfg(test)]
mod tests;

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
					.map(pad_to_bls_scalar)
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
			start = start.saturating_add(scalars.len() as u32); // next item should start after current one
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
			grid.push(pad_to_bls_scalar(rnd_values)?);
		}

		Ok(EvaluationGrid {
			lookup,
			evals: grid
				.into_row_major(dims.width(), dims.height())
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
			.get(i.saturating_add(1))
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
		let h_mul = self.dims.height() / orig_dims.height_nz();

		let (start_ind, end_ind) = self.app_data_indices(app_id)?;
		let (_, start_y) = RowMajor::<()>::ind_to_coord(orig_dims, start_ind);
		let (_, end_y) = RowMajor::<()>::ind_to_coord(orig_dims, end_ind.saturating_sub(1)); // Find y of last cell elt
		let (new_start_y, new_end_y) = (start_y.saturating_mul(h_mul), end_y.saturating_mul(h_mul));

		(new_start_y..=new_end_y)
			.step_by(h_mul)
			.map(|y| self.evals.row(y).map(|a| (y, a.to_vec())))
			.collect()
	}

	pub fn extend_columns(&self, extension_factor: usize) -> Result<Self, Error> {
		let new_dims = self.dims.extend(Extension::height(
			extension_factor
				.try_into()
				.map_err(|_| Error::CellLengthExceeded)?,
		));

		let domain = EvaluationDomain::new(self.dims.height())?;
		let domain_new = EvaluationDomain::new(new_dims.height())?;
		if domain_new.size() != new_dims.height() {
			return Err(Error::DomainSizeInalid);
		}

		let new_evals = self
			.evals
			.columns()
			.flat_map(|(_x, col)| {
				// put elts into a new column
				let mut ext_col = Vec::with_capacity(domain_new.size());
				col.for_each(|s| ext_col.push(*s));
				// ifft, resize, fft
				domain.ifft_slice(&mut ext_col);
				ext_col.resize(domain_new.size(), BlsScalar::zero());
				domain_new.fft_slice(&mut ext_col);
				ext_col
			})
			.collect::<Vec<_>>()
			.into_column_major(new_dims.width(), new_dims.height())
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
				.map(|(_, row)| {
					Evaluations::from_vec_and_domain(row.to_vec(), domain).interpolate()
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
			.map(|poly| srs.commit(poly).map_err(Error::PlonkError))
			.collect()
	}

	pub fn commitment(&self, srs: &CommitKey, row: usize) -> Result<Commitment, Error> {
		self.inner
			.get(row)
			.ok_or(Error::CellLengthExceeded)
			.and_then(|poly| srs.commit(poly).map_err(Error::PlonkError))
	}

	pub fn proof(&self, srs: &CommitKey, cell: &Cell) -> Result<Commitment, Error> {
		let x = cell.col.0 as usize;
		let y = cell.row.0 as usize;
		let poly = self.inner.get(y).ok_or(Error::CellLengthExceeded)?;
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
		let block = multiproof_block(
			cell.col.0 as usize,
			cell.row.0 as usize,
			&self.dims,
			target_dims,
		)
		.ok_or(Error::CellLengthExceeded)?;
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

		let mut ts = Transcript::new(b"avail-mp");
		let proof = srs
			.open(&mut ts, &evals, &polys, points)
			.map_err(Error::MultiproofError)?;

		Ok(Multiproof {
			proof,
			evals,
			block,
		})
	}
}

fn convert_bls(dusk: &dusk_plonk::bls12_381::BlsScalar) -> poly_multiproof::m1_blst::Fr {
	poly_multiproof::m1_blst::Fr {
		0: poly_multiproof::ark_ff::BigInt(dusk.0),
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
	pub start_x: usize,
	pub start_y: usize,
	pub end_x: usize,
	pub end_y: usize,
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

	let block_width = grid_dims.width() / mp_grid_dims.width_nz();
	let block_height = grid_dims.height() / mp_grid_dims.height_nz();
	Some(CellBlock {
		start_x: x.checked_mul(block_width)?,
		start_y: y.checked_mul(block_height)?,
		end_x: x.checked_add(1)?.checked_mul(block_width)?,
		end_y: y.checked_add(1)?.checked_mul(block_height)?,
	})
}

/// Dimensions of the multiproof grid. These are guarenteed to cleanly divide `grid_dims`.
/// `target_dims` must cleanly divide `grid_dims`.
pub fn multiproof_dims(grid_dims: &Dimensions, target_dims: &Dimensions) -> Option<Dimensions> {
	let target_width = grid_dims.width_nz().min(target_dims.width_nz());
	let target_height = grid_dims.height_nz().min(target_dims.height_nz());
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
		Ok(Dimensions::new(
			width.try_into().map_err(|_| Error::ZeroDimension)?,
			1.try_into().expect("1 is nonzero"),
		))
	} else {
		let width = NonZeroUsize::new(max_width).ok_or(Error::ZeroDimension)?;
		let current_height = round_up_to_multiple(n_scalars, width) / width;
		// Round the height up to a power of 2 for ffts
		let height = round_up_power_of_2(current_height);
		// Error if height too big
		if height > max_height {
			return Err(Error::BlockTooBig);
		}
		Ok(Dimensions::new(
			width,
			height.try_into().map_err(|_| Error::ZeroDimension)?,
		))
	}
}

pub fn domain_points(n: usize) -> Result<Vec<BlsScalar>, Error> {
	let domain = EvaluationDomain::new(n)?;
	Ok(domain.elements().collect())
}

pub fn to_ark_scalar(s: &BlsScalar) -> ArkScalar {
	ArkScalar {
		0: poly_multiproof::ark_ff::BigInt(s.0),
		1: PhantomData,
	}
}

pub fn to_mp_commitment(c: Commitment) -> MpCommitment {
    MpCommitment::from_bytes(&c.to_bytes()).expect("commitment is valid")
}

fn round_up_to_multiple(input: usize, multiple: NonZeroUsize) -> usize {
	let n_multiples = input.saturating_add(multiple.get()).saturating_sub(1) / multiple;
	n_multiples.saturating_mul(multiple.get())
}

pub(crate) fn pad_to_bls_scalar(a: impl AsRef<[u8]>) -> Result<BlsScalar, Error> {
	if a.as_ref().len() > DATA_CHUNK_SIZE {
		return Err(Error::InvalidChunkLength);
	}
	let mut buf = [0u8; BlsScalar::SIZE];
	buf[0..a.as_ref().len()].copy_from_slice(a.as_ref());
	BlsScalar::from_bytes(&buf).map_err(Error::DuskBytesError)
}

// Round up. only valid for positive integers
#[allow(clippy::integer_arithmetic)]
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
	v
}

#[cfg(test)]
#[allow(clippy::integer_arithmetic)]
mod unit_tests {
	use super::*;
	use proptest::{prop_assert_eq, proptest};
	use test_case::test_case;

	// parameters that will split a 256x256 grid into pieces of size 4x16
	const TARGET: Dimensions = Dimensions::new_unchecked(64, 16);
	const GRID: Dimensions = Dimensions::new_unchecked(256, 256);

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
			&Dimensions::new_unchecked(grid_w, grid_h),
			&Dimensions::new_unchecked(target_w, target_h),
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
				prop_assert_eq!(round_up_to_multiple(a, m.try_into().unwrap()), i * m)
			}
		}

		#[test]
		fn test_convert_bls_scalar(input: [u8; 31]) {
			use poly_multiproof::ark_serialize::CanonicalSerialize;
			let dusk = pad_to_bls_scalar(input).unwrap();
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

	#[test_case(0 => Dimensions::new_unchecked(4,  1) ; "block size zero")]
	#[test_case(1 => Dimensions::new_unchecked(4,  1) ; "below minimum block size")]
	#[test_case(10 => Dimensions::new_unchecked(16, 1) ; "regular case")]
	#[test_case(17 => Dimensions::new_unchecked(32, 1) ; "minimum overhead after 512")]
	#[test_case(256 => Dimensions::new_unchecked(256, 1) ; "maximum cols")]
	#[test_case(257 => Dimensions::new_unchecked(256, 2) ; "two rows")]
	#[test_case(256 * 256 => Dimensions::new_unchecked(256, 256) ; "max block size")]
	#[test_case(256 * 256 + 1 => panics "BlockTooBig" ; "too much data")]
	fn test_get_block_dims(size: usize) -> Dimensions
where {
		get_block_dims(size, 4, 256, 256).unwrap()
	}
}
