use alloc::vec::Vec;

use crate::Dimensions;

pub trait Grid<A> {
	fn width(&self) -> usize;
	fn height(&self) -> usize;
	fn dims(&self) -> &Dimensions;
	// x indexes within a row, y indexes within a column
	// 0 <= x < width, 0 <= y < height
	fn get(&self, x: usize, y: usize) -> Option<&A> {
		let i = Self::coord_to_ind(self.dims(), x, y);
		self.get_ind(i)
	}
	fn get_ind(&self, i: usize) -> Option<&A>;
	fn ind_to_coord(dims: &Dimensions, i: usize) -> (usize, usize);
	fn coord_to_ind(dims: &Dimensions, x: usize, y: usize) -> usize;
}

pub struct RowMajor<A> {
	dims: Dimensions,
	pub inner: Vec<A>,
}

pub struct ColumnMajor<A> {
	dims: Dimensions,
	pub inner: Vec<A>,
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
		(i % dims.width(), i / dims.width())
	}

	fn coord_to_ind(dims: &Dimensions, x: usize, y: usize) -> usize {
		x + y * dims.width()
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
		(i / dims.height(), i % dims.height())
	}

	fn coord_to_ind(dims: &Dimensions, x: usize, y: usize) -> usize {
		y + x * dims.height()
	}
}

impl<A: Clone> RowMajor<A> {
	pub fn row(&self, y: usize) -> Option<&[A]> {
		if y >= self.height() {
			return None;
		}
		Some(&self.inner[(y * self.width())..((y + 1) * self.width())])
	}

	pub fn iter_col(&self, x: usize) -> Option<impl Iterator<Item = &A> + '_> {
		if x >= self.width() {
			return None;
		}
		Some((0..self.height()).map(move |y| self.get(x, y).expect("Size checked at instantiation")))
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

	pub fn to_column_major(&self) -> ColumnMajor<A> {
		self.iter_column_wise()
			.map(Clone::clone)
			.collect::<Vec<_>>()
			.as_column_major(self.width(), self.height())
			.expect("Bounds already checked")
	}
}

impl<A: Clone> ColumnMajor<A> {
	pub fn col(&self, x: usize) -> Option<&[A]> {
		if x >= self.width() {
			return None;
		}
		Some(&self.inner[(x * self.height())..((x + 1) * self.height())])
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

	pub fn to_row_major(&self) -> RowMajor<A> {
		self.iter_row_wise()
			.map(Clone::clone)
			.collect::<Vec<_>>()
			.as_row_major(self.width(), self.height())
			.expect("Bounds already checked")
	}
}

pub trait AsRowMajor<A> {
	fn as_row_major(self, width: usize, height: usize) -> Option<RowMajor<A>>;
}

pub trait AsColumnMajor<A> {
	fn as_column_major(self, width: usize, height: usize) -> Option<ColumnMajor<A>>;
}

impl<A> AsRowMajor<A> for Vec<A> {
	fn as_row_major(self, width: usize, height: usize) -> Option<RowMajor<A>> {
		if self.len() == width * height {
			Some(RowMajor {
                dims: Dimensions::new(width, height),
				inner: self,
			})
		} else {
			None
		}
	}
}

impl<A> AsColumnMajor<A> for Vec<A> {
	fn as_column_major(self, width: usize, height: usize) -> Option<ColumnMajor<A>> {
		if self.len() == width * height {
			Some(ColumnMajor {
                dims: Dimensions::new(width, height),
				inner: self,
			})
		} else {
			None
		}
	}
}

impl<A, const LEN: usize> AsColumnMajor<A> for [A; LEN] {
	fn as_column_major(self, width: usize, height: usize) -> Option<ColumnMajor<A>> {
		if self.len() == width * height {
			Some(ColumnMajor {
                dims: Dimensions::new(width, height),
				inner: self.into(),
			})
		} else {
			None
		}
	}
}

impl<A, const LEN: usize> AsRowMajor<A> for [A; LEN] {
	fn as_row_major(self, width: usize, height: usize) -> Option<RowMajor<A>> {
		if self.len() == width * height {
			Some(RowMajor {
                dims: Dimensions::new(width, height),
				inner: self.into(),
			})
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloc::vec::Vec;

	#[test]
	fn test_row_major() {
		let data = [1, 2, 3, 4, 5, 6];
		let rm = data.as_row_major(3, 2).unwrap();

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
		let cm = data.as_column_major(3, 2).unwrap();

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
