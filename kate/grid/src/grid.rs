use alloc::vec::Vec;

use crate::Dimensions;

pub trait Grid<A> {
	fn width(&self) -> usize;
	fn height(&self) -> usize;
	fn dims(&self) -> &Dimensions;
	fn inner(&self) -> &Vec<A>;
	// x indexes within a row, y indexes within a column
	// 0 <= x < width, 0 <= y < height
	fn get(&self, x: usize, y: usize) -> Option<&A> {
		let i = Self::coord_to_ind(self.dims(), x, y)?;
		self.get_ind(i)
	}
	fn get_ind(&self, i: usize) -> Option<&A>;
	fn ind_to_coord(dims: &Dimensions, i: usize) -> (usize, usize);
	fn coord_to_ind(dims: &Dimensions, x: usize, y: usize) -> Option<usize>;
}

pub struct RowMajor<A> {
	dims: Dimensions,
	inner: Vec<A>,
}

pub struct ColumnMajor<A> {
	dims: Dimensions,
	inner: Vec<A>,
}

impl<A> Grid<A> for RowMajor<A> {
	fn width(&self) -> usize {
		self.dims.width()
	}

	fn height(&self) -> usize {
		self.dims.height()
	}

	fn dims(&self) -> &Dimensions {
		&self.dims
	}

	fn get_ind(&self, i: usize) -> Option<&A> {
		self.inner.get(i)
	}

	fn ind_to_coord(dims: &Dimensions, i: usize) -> (usize, usize) {
		(i % dims.width_nz(), i / dims.width_nz())
	}

	fn coord_to_ind(dims: &Dimensions, x: usize, y: usize) -> Option<usize> {
		x.checked_add(y.checked_mul(dims.width())?)
	}

	fn inner(&self) -> &Vec<A> {
		&self.inner
	}
}

impl<A> Grid<A> for ColumnMajor<A> {
	fn width(&self) -> usize {
		self.dims.width()
	}

	fn height(&self) -> usize {
		self.dims.height()
	}

	fn dims(&self) -> &Dimensions {
		&self.dims
	}

	fn get_ind(&self, i: usize) -> Option<&A> {
		self.inner.get(i)
	}

	fn ind_to_coord(dims: &Dimensions, i: usize) -> (usize, usize) {
		(i / dims.height_nz(), i % dims.height_nz())
	}

	fn coord_to_ind(dims: &Dimensions, x: usize, y: usize) -> Option<usize> {
		y.checked_add(x.checked_mul(dims.height())?)
	}

	fn inner(&self) -> &Vec<A> {
		&self.inner
	}
}

#[cfg(feature = "parallel")]
use rayon::prelude::*;

impl<A> RowMajor<A> {
	pub fn new(width: usize, height: usize, data: Vec<A>) -> Option<Self> {
		if data.len() == usize::checked_mul(width, height)? {
			Some(Self {
				dims: Dimensions::new(width.try_into().ok()?, height.try_into().ok()?),
				inner: data,
			})
		} else {
			None
		}
	}
	pub fn row(&self, y: usize) -> Option<&[A]> {
		if y >= self.height() {
			return None;
		}
		// SAFETY: `y < height` (just one line up)  and `height * width` **is already checked** at `new / into_column_mayor` fns
		// as invariant of this type, then we can omit `checked_` operations.
		let start = y * self.width();
		let end = (y + 1) * self.width();
		Some(&self.inner[start..end])
	}

	pub fn iter_col(&self, x: usize) -> Option<impl Iterator<Item = &A> + '_> {
		if x >= self.width() {
			return None;
		}
		Some((0..self.height()).map(move |y| self.get(x, y).expect("Bounds already checked")))
	}

	pub fn rows(&self) -> impl Iterator<Item = (usize, &[A])> + '_ {
		(0..self.height()).map(|y| (y, self.row(y).expect("Bounds already checked")))
	}
	// TODO: this return type is kinda gross, should it just iterate over vecs?
	pub fn columns(&self) -> impl Iterator<Item = (usize, impl Iterator<Item = &A>)> + '_ {
		(0..self.width()).map(|x| (x, self.iter_col(x).expect("Bounds already checked")))
	}

	pub fn iter_row_wise(&self) -> impl Iterator<Item = &A> + '_ {
		(0..self.height()).flat_map(move |y| {
			(0..self.width()).map(move |x| self.get(x, y).expect("Bounds already checked"))
		})
	}

	pub fn iter_column_wise(&self) -> impl Iterator<Item = &A> + '_ {
		(0..self.width()).flat_map(move |x| {
			(0..self.height()).map(move |y| self.get(x, y).expect("Bounds already checked"))
		})
	}
}

impl<A: Clone> RowMajor<A> {
	pub fn to_column_major(&self) -> ColumnMajor<A> {
		self.iter_column_wise()
			.map(Clone::clone)
			.collect::<Vec<_>>()
			.into_column_major(self.width(), self.height())
			.expect("Bounds already checked")
	}
}

#[cfg(feature = "parallel")]
impl<A: Clone + Send + Sync> RowMajor<A> {
	pub fn rows_par_iter(&self) -> impl ParallelIterator<Item = (usize, &[A])> + '_ {
		(0..self.height())
			.into_par_iter()
			.map(|y| (y, self.row(y).expect("Bounds already checked")))
	}
}

impl<A> ColumnMajor<A> {
	pub fn new(width: usize, height: usize, data: Vec<A>) -> Option<Self> {
		if data.len() == usize::checked_mul(width, height)? {
			Some(Self {
				dims: Dimensions::new(width.try_into().ok()?, height.try_into().ok()?),
				inner: data,
			})
		} else {
			None
		}
	}
	pub fn col(&self, x: usize) -> Option<&[A]> {
		if x >= self.width() {
			return None;
		}
		let start = x.checked_mul(self.height())?;
		let end = x.checked_add(1)?.checked_mul(self.height())?;
		Some(&self.inner[start..end])
	}

	pub fn iter_row(&self, y: usize) -> Option<impl Iterator<Item = &A> + '_> {
		if y >= self.height() {
			return None;
		}
		Some((0..self.width()).map(move |x| self.get(x, y).expect("Size checked at instantiation")))
	}

	pub fn iter_row_wise(&self) -> impl Iterator<Item = &A> + '_ {
		(0..self.height()).flat_map(move |y| {
			(0..self.width()).map(move |x| self.get(x, y).expect("Bounds already checked"))
		})
	}

	pub fn iter_column_wise(&self) -> impl Iterator<Item = &A> + '_ {
		(0..self.width()).flat_map(move |x| {
			(0..self.height()).map(move |y| self.get(x, y).expect("Bounds already checked"))
		})
	}
}

impl<A: Clone> ColumnMajor<A> {
	pub fn to_row_major(&self) -> RowMajor<A> {
		self.iter_row_wise()
			.map(Clone::clone)
			.collect::<Vec<_>>()
			.into_row_major(self.width(), self.height())
			.expect("Bounds already checked")
	}
}

pub trait IntoRowMajor<A> {
	/// Convert the underlying data structure to be row-major. This likely involves
	/// re-allocating the array or re-arranging its elements.
	fn into_row_major(self, width: usize, height: usize) -> Option<RowMajor<A>>;
}

pub trait IntoColumnMajor<A> {
	/// Convert the underlying data structure to be column-major. This likely involves
	/// re-allocating the array or re-arranging its elements.
	fn into_column_major(self, width: usize, height: usize) -> Option<ColumnMajor<A>>;
}

impl<A, B: Into<Vec<A>>> IntoRowMajor<A> for B {
	fn into_row_major(self, width: usize, height: usize) -> Option<RowMajor<A>> {
		RowMajor::new(width, height, self.into())
	}
}

impl<A, B: Into<Vec<A>>> IntoColumnMajor<A> for B {
	fn into_column_major(self, width: usize, height: usize) -> Option<ColumnMajor<A>> {
		ColumnMajor::new(width, height, self.into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloc::vec::Vec;

	#[test]
	fn test_row_major() {
		let data = [1, 2, 3, 4, 5, 6];
		let rm = data.into_row_major(3, 2).unwrap();

		assert_eq!(rm.get(0, 0), Some(&1));
		assert_eq!(rm.get(1, 0), Some(&2));
		assert_eq!(rm.get(2, 0), Some(&3));
		assert_eq!(rm.get(0, 1), Some(&4));
		assert_eq!(rm.get(1, 1), Some(&5));
		assert_eq!(rm.get(2, 1), Some(&6));

		assert_eq!([1, 2, 3].as_slice(), rm.row(0).unwrap());
		assert_eq!([4, 5, 6].as_slice(), rm.row(1).unwrap());
		assert_eq!(vec![&1, &4], rm.iter_col(0).unwrap().collect::<Vec<_>>());
		assert_eq!(vec![&2, &5], rm.iter_col(1).unwrap().collect::<Vec<_>>());
		assert_eq!(vec![&3, &6], rm.iter_col(2).unwrap().collect::<Vec<_>>());
	}

	#[test]
	fn test_column_major() {
		let data = [1, 4, 2, 5, 3, 6];
		let cm = data.into_column_major(3, 2).unwrap();

		assert_eq!(cm.get(0, 0), Some(&1));
		assert_eq!(cm.get(1, 0), Some(&2));
		assert_eq!(cm.get(2, 0), Some(&3));
		assert_eq!(cm.get(0, 1), Some(&4));
		assert_eq!(cm.get(1, 1), Some(&5));
		assert_eq!(cm.get(2, 1), Some(&6));

		assert_eq!([1, 4].as_slice(), cm.col(0).unwrap());
		assert_eq!([2, 5].as_slice(), cm.col(1).unwrap());
		assert_eq!([3, 6].as_slice(), cm.col(2).unwrap());
		assert_eq!(
			vec![&1, &2, &3],
			cm.iter_row(0).unwrap().collect::<Vec<_>>()
		);
		assert_eq!(
			vec![&4, &5, &6],
			cm.iter_row(1).unwrap().collect::<Vec<_>>()
		);
	}
}
