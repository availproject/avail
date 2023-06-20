use crate::pmp::{
	ark_poly::{EvaluationDomain, GeneralEvaluationDomain},
	m1_blst::{Bls12_381, M1NoPrecomp},
	merlin::Transcript,
	traits::Committer,
};
use codec::Encode;
use core::{cmp::max, num::NonZeroU16};
use da_types::{AppExtrinsic, AppId, DataLookup, DataLookupIndexItem};
use kate_recovery::{config::PADDING_TAIL_VALUE, ensure, matrix::Dimensions};
use nalgebra::base::DMatrix;
use poly_multiproof::{
	m1_blst::Proof,
	traits::{KZGProof, PolyMultiProofNoPrecomp},
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use std::{cmp::min, collections::BTreeMap};

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
	pub evals: DMatrix<ArkScalar>,
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
		let dim_size: usize = dims.size();
		let (rows, cols): (usize, usize) = dims.into();
		let mut rng = ChaChaRng::from_seed(rng_seed);
		while grid.len() != dim_size {
			let rnd_values: [u8; SCALAR_SIZE - 1] = rng.gen();
			// TODO: can we just use zeros instead?
			grid.push(pad_to_bls_scalar(rnd_values)?);
		}

		Ok(EvaluationGrid {
			lookup,
			evals: DMatrix::from_row_iterator(rows, cols, grid.into_iter()),
		})
	}

	pub fn row(&self, y: usize) -> Option<Vec<ArkScalar>> {
		let (rows, _) = self.evals.shape();
		(y < rows).then(|| self.evals.row(y).into_iter().cloned().collect::<Vec<_>>())
	}

	pub fn dims(&self) -> Dimensions {
		let (rows, cols) = self.evals.shape();
		// SAFETY: We cannot construct an `EvaluationGrid` with any dimension `< 1` or `> u16::MAX`
		unsafe { Dimensions::new_unchecked(rows as u16, cols as u16) }
	}

	/// Returns a list of `(index, row)` pairs for the underlying rows of an application.
	/// Returns `None` if the `app_id` cannot be found, or if the provided `orig_dims` are invalid.
	pub fn app_rows(
		&self,
		app_id: AppId,
		orig_dims: Option<Dimensions>,
	) -> Option<Vec<(usize, Vec<ArkScalar>)>> {
		let (rows, _cols) = self.evals.shape();
		let dims = self.dims();
		let orig_dims = match orig_dims {
			Some(d) => {
				if !d.divides(&dims) {
					return None;
				}
				d
			},
			None => dims,
		};

		// SAFETY: `origin_dims.rows is NonZeroU16`
		// Compiler checks that `Dimensions::rows()` returns a `NonZeroU16` using the expression
		// `NonZeroU16::get(x)` instead of `x.get()`.
		#[allow(clippy::integer_arithmetic)]
		let h_mul: usize = rows / usize::from(NonZeroU16::get(orig_dims.rows()));
		#[allow(clippy::integer_arithmetic)]
		let index_to_y_coord = |dims: &Dimensions, index: u32| -> u32 {
			index / u32::from(NonZeroU16::get(dims.rows()))
		};

		let (start_ind, end_ind) = self.lookup.range_of(app_id)?;
		let start_y: usize = index_to_y_coord(&orig_dims, start_ind).try_into().ok()?;
		let end_y: usize = index_to_y_coord(&orig_dims, end_ind.saturating_sub(1))
			.try_into()
			.ok()?; // Find y of last cell elt
		let (new_start_y, new_end_y) = (start_y.checked_mul(h_mul)?, end_y.checked_mul(h_mul)?);

		(new_start_y..=new_end_y)
			.step_by(h_mul)
			.map(|y| self.row(y).map(|a| (y, a)))
			.collect()
	}

	pub fn extend_columns(&self, row_factor: NonZeroU16) -> Result<Self, Error> {
		let dims = self.dims();
		let (new_rows, new_cols): (usize, usize) = dims
			.extend(row_factor, unsafe { NonZeroU16::new_unchecked(1) })
			.ok_or(Error::CellLengthExceeded)?
			.into();
		let (rows, cols): (usize, usize) = dims.into();

		let domain =
			GeneralEvaluationDomain::<ArkScalar>::new(rows).ok_or(Error::DomainSizeInvalid)?;
		let domain_new =
			GeneralEvaluationDomain::<ArkScalar>::new(new_rows).ok_or(Error::DomainSizeInvalid)?;
		ensure!(domain_new.size() == new_rows, Error::DomainSizeInvalid);

		let cols = (0..cols)
			.into_iter()
			.map(|c| self.evals.column(c).iter().cloned().collect::<Vec<_>>());

		let new_evals = cfg_into_iter!(cols)
			.flat_map(|mut col| {
				// ifft, resize, fft
				domain.ifft_in_place(&mut col);
				domain_new.fft_in_place(&mut col);
				col
			})
			.collect::<Vec<_>>();

		let new_evals = DMatrix::from_column_slice(new_rows, new_cols, &new_evals);
		debug_assert!(new_evals.shape() == (new_rows, new_cols));

		Ok(Self {
			lookup: self.lookup.clone(),
			evals: new_evals,
		})
	}

	pub fn make_polynomial_grid(&self) -> Result<PolynomialGrid, Error> {
		let (_rows, cols) = self.evals.shape();
		let domain =
			GeneralEvaluationDomain::<ArkScalar>::new(cols).ok_or(Error::DomainSizeInvalid)?;

		let inner = self
			.evals
			.row_iter()
			.map(|row_iter| {
				let row = row_iter.iter().cloned().collect::<Vec<_>>();
				domain.ifft(row.as_slice())
			})
			.collect::<Vec<_>>();

		Ok(PolynomialGrid {
			dims: self.dims(),
			points: domain.elements().collect(),
			inner,
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
			.map(|coeffs| srs.commit(coeffs).map_err(Error::MultiproofError))
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
			self.dims,
			target_dims,
		)
		.ok_or(Error::CellLengthExceeded)?;
		let polys = &self.inner[block.start_y..block.end_y];
		let evals: Vec<Vec<ArkScalar>> = (block.start_y..block.end_y)
			.map(|y| {
				eval_grid.row(y).expect("Already bounds checked .qed")[block.start_x..block.end_x]
					.to_vec()
			})
			.collect::<Vec<_>>();
		let evals_view = evals.iter().map(|row| row.as_slice()).collect::<Vec<_>>();

		let points = &self.points[block.start_x..block.end_x];
		let mut ts = Transcript::new(b"avail-mp");
		let proof = PolyMultiProofNoPrecomp::open(srs, &mut ts, &evals_view, polys, points)
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
	grid: Dimensions,
	target: &Dimensions,
) -> Option<CellBlock> {
	let (mp_rows, mp_cols): (usize, usize) = multiproof_dims(grid, target)?.into();
	let (g_rows, g_cols): (usize, usize) = grid.into();
	if x >= mp_cols || y >= mp_rows {
		return None;
	}

	let block_width = g_cols
		.checked_div(mp_cols)
		.expect("`mp_cols` created from a `NonZeroU16` .qed");
	let block_height = g_rows
		.checked_div(mp_rows)
		.expect("`mp_rows` created from a `NonZeroU16` .qed");
	Some(CellBlock {
		start_x: x.checked_mul(block_width)?,
		start_y: y.checked_mul(block_height)?,
		end_x: x.checked_add(1)?.checked_mul(block_width)?,
		end_y: y.checked_add(1)?.checked_mul(block_height)?,
	})
}

/// Dimensions of the multiproof grid. These are guarenteed to cleanly divide `grid_dims`.
/// `target_dims` must cleanly divide `grid_dims`.
pub fn multiproof_dims(grid: Dimensions, target: &Dimensions) -> Option<Dimensions> {
	let cols = min(grid.cols(), target.cols());
	let rows = min(grid.rows(), target.rows());
	if grid.cols().get() % cols != 0 || grid.rows().get() % rows != 0 {
		return None;
	}

	Dimensions::new(rows, cols)
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
		let width = max(
			current_width
				.checked_next_power_of_two()
				.ok_or(Error::BlockTooBig)?,
			min_width,
		);
		let height = unsafe { NonZeroU16::new_unchecked(1) };

		Dimensions::new_from(height, width).ok_or(Error::ZeroDimension)
	} else {
		let width = NonZeroU16::try_from(u16::try_from(max_width)?)?;
		let current_height = round_up_to_multiple(n_scalars, width)
			.checked_div(max_width)
			.expect("`max_width` is non zero, checked one line before");
		// Round the height up to a power of 2 for ffts
		let height = current_height
			.checked_next_power_of_two()
			.ok_or(Error::BlockTooBig)?;
		// Error if height too big
		ensure!(height <= max_height, Error::BlockTooBig);

		Dimensions::new_from(height, width).ok_or(Error::ZeroDimension)
	}
}

pub fn domain_points(n: usize) -> Result<Vec<ArkScalar>, Error> {
	let domain = GeneralEvaluationDomain::<ArkScalar>::new(n).ok_or(Error::DomainSizeInvalid)?;
	Ok(domain.elements().collect())
}

/// SAFETY: As `multiple` is a `NonZeroU16` we can safetly make the following ops.
#[allow(clippy::integer_arithmetic)]
fn round_up_to_multiple(input: usize, multiple: NonZeroU16) -> usize {
	let multiple: usize = multiple.get().into();
	let n_multiples = input.saturating_add(multiple - 1) / multiple;
	n_multiples.saturating_mul(multiple)
}

pub(crate) fn pad_to_bls_scalar(a: impl AsRef<[u8]>) -> Result<ArkScalar, Error> {
	if a.as_ref().len() > DATA_CHUNK_SIZE {
		return Err(Error::InvalidChunkLength);
	}
	let mut buf = [0u8; SCALAR_SIZE];
	buf[0..a.as_ref().len()].copy_from_slice(a.as_ref());
	ArkScalar::from_bytes(&buf).map_err(Error::MultiproofError)
}

#[cfg(test)]
#[allow(clippy::integer_arithmetic)]
mod unit_tests {
	use super::*;
	use proptest::{prop_assert_eq, proptest};
	use test_case::test_case;

	// parameters that will split a 256x256 grid into pieces of size 4x16
	const TARGET: Dimensions = unsafe { Dimensions::new_unchecked(16, 64) };
	const GRID: Dimensions = unsafe { Dimensions::new_unchecked(256, 256) };

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
		multiproof_block(x, y, GRID.clone(), &TARGET)
	}

	#[test_case(256, 256,  64,  16 => Some((64, 16)))]
	#[test_case(256, 256,  32,  32 => Some((32, 32)))]
	#[test_case(256, 256,   7,  32 => None)]
	#[test_case(32 ,  32,  32,  32 => Some((32, 32)))]
	#[test_case(256,   8,  32,  32 => Some((32, 8)))]
	#[test_case(4  ,   1,  32,  32 => Some((4, 1)))]
	fn test_multiproof_dims(
		grid_w: u16,
		grid_h: u16,
		target_w: u16,
		target_h: u16,
	) -> Option<(usize, usize)> {
		let grid = unsafe { Dimensions::new_unchecked(grid_w, grid_h) };
		let target = unsafe { Dimensions::new_unchecked(target_w, target_h) };

		multiproof_dims(grid, &target).map(Into::into)
	}

	use proptest::prelude::*;
	proptest! {
		#![proptest_config(ProptestConfig {
			cases: 200, .. ProptestConfig::default()
		  })]
		#[test]
		fn test_round_up_to_multiple(i in 1..1000usize, m in 1..32u16) {
			for k in 0..usize::from(m) {
				let a :usize = i * usize::from(m) - k;
				let output = round_up_to_multiple(a, m.try_into().unwrap());
				let expected :usize = i * usize::from(m);
				prop_assert_eq!( output, expected)
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
		i.next_power_of_two()
	}

	fn new_dim(rows: u16, cols: u16) -> Result<Dimensions, Error> {
		Dimensions::new(rows, cols).ok_or(Error::BlockTooBig)
	}

	#[test_case(0 => new_dim(1,4) ; "block size zero")]
	#[test_case(1 => new_dim(1,4) ; "below minimum block size")]
	#[test_case(10 => new_dim(1, 16) ; "regular case")]
	#[test_case(17 => new_dim(1, 32) ; "minimum overhead after 512")]
	#[test_case(256 => new_dim(1, 256) ; "maximum cols")]
	#[test_case(257 => new_dim(2, 256) ; "two rows")]
	#[test_case(256 * 256 => new_dim(256, 256) ; "max block size")]
	#[test_case(256 * 256 + 1 => Err(Error::BlockTooBig) ; "too much data")]
	fn test_get_block_dims(size: usize) -> Result<Dimensions, Error>
where {
		get_block_dims(size, 4, 256, 256)
	}
}
