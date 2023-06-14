use crate::pmp::{
	ark_poly::{EvaluationDomain, GeneralEvaluationDomain},
	m1_blst::{Bls12_381, M1NoPrecomp},
	merlin::Transcript,
	traits::Committer,
};
use codec::Encode;
use core::num::NonZeroUsize;
use da_types::{AppExtrinsic, AppId, DataLookup, DataLookupIndexItem};
use kate_grid::{Dimensions, Extension, Grid, IntoColumnMajor, IntoRowMajor, RowMajor};
use kate_recovery::config::PADDING_TAIL_VALUE;
use poly_multiproof::{
	m1_blst::Proof,
	traits::{KZGProof, PolyMultiProofNoPrecomp},
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use std::collections::BTreeMap;

use crate::{
	com::{Cell, Error},
	config::DATA_CHUNK_SIZE,
	Seed,
};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

macro_rules! cfg_iter {
	($e: expr) => {{
		#[cfg(feature = "parallel")]
		let result = $e.par_iter();
		#[cfg(not(feature = "parallel"))]
		let result = $e.iter();
		result
	}};
}

macro_rules! cfg_into_iter {
	($e: expr) => {{
		#[cfg(feature = "parallel")]
		let result = $e.into_par_iter();
		#[cfg(not(feature = "parallel"))]
		let result = $e.into_iter();
		result
	}};
}

pub const SCALAR_SIZE: usize = 32;
pub type ArkScalar = crate::pmp::m1_blst::Fr;
pub type Commitment = crate::pmp::Commitment<Bls12_381>;
pub use poly_multiproof::traits::AsBytes;

#[cfg(test)]
mod tests;

pub struct EvaluationGrid {
	pub lookup: DataLookup,
	pub evals: RowMajor<ArkScalar>,
	pub dims: Dimensions,
}

impl EvaluationGrid {
	/// From the app extrinsics, create a data grid of Scalars
	pub fn from_extrinsics(
		extrinsics: Vec<AppExtrinsic>,
		min_width: usize,
		max_width: usize,
		max_height: usize,
		rng_seed: Seed,
	) -> Result<Self, Error> {
		// Group extrinsics by app id, also sorted by app id.
		// Using a BTreeMap here will still iter in sorted order. Sweet!
		let grouped = extrinsics.into_iter().fold::<BTreeMap<AppId, Vec<_>>, _>(
			BTreeMap::default(),
			|mut acc, e| {
				acc.entry(e.app_id).or_default().push(e.data);
				acc
			},
		);

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
			start = start
				.checked_add(scalars.len() as u32)
				.ok_or(Error::CellLengthExceeded)?; // next item should start after current one
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
			let rnd_values: [u8; SCALAR_SIZE - 1] = rng.gen();
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

	pub fn row(&self, y: usize) -> Option<&[ArkScalar]> {
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
	) -> Option<Vec<(usize, Vec<ArkScalar>)>> {
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

		let domain = GeneralEvaluationDomain::<ArkScalar>::new(self.dims.height())
			.ok_or(Error::BaseGridDomainSizeInvalid(self.dims.width()))?;
		let domain_new = GeneralEvaluationDomain::<ArkScalar>::new(new_dims.height())
			.ok_or(Error::ExtendedGridDomianSizeInvalid(new_dims.width()))?;
		if domain_new.size() != new_dims.height() {
			return Err(Error::DomainSizeInvalid);
		}

		let cols: Vec<Vec<ArkScalar>> = self
			.evals
			.columns()
			.map(|(_i, col)| col.map(|s| *s).collect::<Vec<_>>())
			.collect::<Vec<_>>();

		let new_evals = cfg_into_iter!(cols)
			.flat_map(|mut col| {
				// ifft, resize, fft
				domain.ifft_in_place(&mut col);
				domain_new.fft_in_place(&mut col);
				col
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
		let domain = GeneralEvaluationDomain::<ArkScalar>::new(self.dims.width())
			.ok_or(Error::DomainSizeInvalid)?;
		#[cfg(not(feature = "parallel"))]
		let rows = self.evals.rows();
		#[cfg(feature = "parallel")]
		let rows = self.evals.rows_par_iter();
		Ok(PolynomialGrid {
			dims: self.dims.clone(),
			points: domain.elements().collect(),
			inner: rows.map(|(_, row)| domain.ifft(row)).collect::<Vec<_>>(),
		})
	}
}

pub struct PolynomialGrid {
	inner: Vec<Vec<ArkScalar>>,
	points: Vec<ArkScalar>,
	dims: Dimensions,
}

impl PolynomialGrid {
	pub fn commitments(
		&self,
		srs: &(impl Committer<Bls12_381> + Sync),
	) -> Result<Vec<Commitment>, Error> {
		cfg_iter!(self.inner)
			.map(|poly| srs.commit(poly).map_err(Error::MultiproofError))
			.collect()
	}

	/// Computes the commitments of the grid for the given extension by committing, then fft-ing
	/// the commitments.
	// TODO: fix this all up without the gross conversions after moving to arkworks
	pub fn extended_commitments(
		&self,
		srs: &(impl Committer<Bls12_381> + Sync),
		extension_factor: usize,
	) -> Result<Vec<Commitment>, Error> {
		let res = cfg_iter!(self.inner)
			.map(|coeffs| srs.commit(&coeffs).map_err(Error::MultiproofError))
			.collect::<Result<Vec<_>, _>>()?;
		poly_multiproof::Commitment::<Bls12_381>::extend_commitments(
			&res,
			res.len().saturating_mul(extension_factor),
		)
		.map_err(Error::MultiproofError)
	}

	pub fn commitment(
		&self,
		srs: &impl Committer<Bls12_381>,
		row: usize,
	) -> Result<Commitment, Error> {
		self.inner
			.get(row)
			.ok_or(Error::CellLengthExceeded)
			.and_then(|poly| srs.commit(poly).map_err(Error::MultiproofError))
	}

	pub fn proof(&self, srs: &M1NoPrecomp, cell: &Cell) -> Result<Proof, Error> {
		let x = cell.col.0 as usize;
		let y = cell.row.0 as usize;
		let poly = self.inner.get(y).ok_or(Error::CellLengthExceeded)?;
		let witness = KZGProof::compute_witness_polynomial(srs, poly.clone(), self.points[x])?;
		Ok(KZGProof::open(srs, witness)?)
	}

	pub fn multiproof(
		&self,
		srs: &M1NoPrecomp,
		cell: &Cell,
		eval_grid: &EvaluationGrid,
		target_dims: &Dimensions,
	) -> Result<Multiproof, Error> {
		let block = multiproof_block(
			cell.col.0 as usize,
			cell.row.0 as usize,
			&self.dims,
			target_dims,
		)
		.ok_or(Error::CellLengthExceeded)?;
		let polys = &self.inner[block.start_y..block.end_y];
		let evals = (block.start_y..block.end_y)
			.map(|y| {
				eval_grid.evals.row(y).expect("Already bounds checked")[block.start_x..block.end_x]
					.to_vec()
			})
			.collect::<Vec<_>>();
		let points = &self.points[block.start_x..block.end_x];
		let mut ts = Transcript::new(b"avail-mp");
		let proof = PolyMultiProofNoPrecomp::open(srs, &mut ts, &evals, &polys, points)
			.map_err(Error::MultiproofError)?;

		Ok(Multiproof {
			proof,
			evals,
			block,
		})
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
	// SAFETY: values never overflow since x,y are always less than grid_dims.{width,height}().
	// This is because x,y < mp_grid_dims.{width, height} and block width is the quotient of
	// grid_dims and mp_grid_dims.
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
		let width = core::cmp::max(round_up_power_of_2(current_width), min_width).try_into()?;
		let height = 1.try_into()?;
		Ok(Dimensions::new(width, height))
	} else {
		let width = NonZeroUsize::try_from(max_width)?;
		let current_height = round_up_to_multiple(n_scalars, width) / width;
		// Round the height up to a power of 2 for ffts
		let height = round_up_power_of_2(current_height);
		// Error if height too big
		if height > max_height {
			return Err(Error::BlockTooBig);
		}
		Ok(Dimensions::new(width, height.try_into()?))
	}
}

pub fn domain_points(n: usize) -> Result<Vec<ArkScalar>, Error> {
	let domain = GeneralEvaluationDomain::<ArkScalar>::new(n).ok_or(Error::DomainSizeInvalid)?;
	Ok(domain.elements().collect())
}

fn round_up_to_multiple(input: usize, multiple: NonZeroUsize) -> usize {
	let n_multiples = input.saturating_add(multiple.get()).saturating_sub(1) / multiple;
	n_multiples.saturating_mul(multiple.get())
}

pub(crate) fn pad_to_bls_scalar(a: impl AsRef<[u8]>) -> Result<ArkScalar, Error> {
	if a.as_ref().len() > DATA_CHUNK_SIZE {
		return Err(Error::InvalidChunkLength);
	}
	let mut buf = [0u8; SCALAR_SIZE];
	buf[0..a.as_ref().len()].copy_from_slice(a.as_ref());
	ArkScalar::from_bytes(&buf).map_err(Error::MultiproofError)
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
