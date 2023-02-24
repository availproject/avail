use core::marker::PhantomData;

pub trait Grid<A> {
	fn width(&self) -> usize;
	fn height(&self) -> usize;
	// x indexes within a row, y indexes within a column
	// 0 <= x < width, 0 <= y < height
	fn get(&self, x: usize, y: usize) -> Option<&A>;
}

pub struct RowMajor<A, T> {
	width: usize,
	height: usize,
	pub inner: T,
	_phantom: PhantomData<A>,
}

pub struct ColumnMajor<A, T> {
	width: usize,
	height: usize,
	pub inner: T,
	_phantom: PhantomData<A>,
}

impl<A, T: AsRef<[A]>> Grid<A> for RowMajor<A, T> {
	fn width(&self) -> usize {
		self.width
	}

	fn height(&self) -> usize {
		self.height
	}

	fn get(&self, x: usize, y: usize) -> Option<&A> {
		self.inner.as_ref().get(x + y * self.width)
	}
}

impl<A, T: AsRef<[A]>> Grid<A> for ColumnMajor<A, T> {
	fn width(&self) -> usize {
		self.width
	}

	fn height(&self) -> usize {
		self.height
	}

	fn get(&self, x: usize, y: usize) -> Option<&A> {
		self.inner.as_ref().get(y + x * self.height)
	}
}

impl<A: Sized, T: AsRef<[A]>> RowMajor<A, T> {
	pub fn row(&self, y: usize) -> Option<&[A]> {
		if y >= self.height {
			return None;
		}
		Some(&self.inner.as_ref()[(y * self.width)..((y + 1) * self.width)])
	}

	pub fn iter_col(&self, x: usize) -> Option<impl Iterator<Item = &A> + '_> {
		if x >= self.width {
			return None;
		}
		Some((0..self.height).map(move |y| self.get(x, y).expect("Size checked at instantiation")))
	}

	pub fn iter_row_wise(&self) -> impl Iterator<Item = &A> + '_ {
		(0..self.height).flat_map(move |y| {
			(0..self.width).map(move |x| self.get(x, y).expect("Bounds already checked"))
		})
	}

	pub fn iter_column_wise(&self) -> impl Iterator<Item = &A> + '_ {
		(0..self.width).flat_map(move |x| {
			(0..self.height).map(move |y| self.get(x, y).expect("Bounds already checked"))
		})
	}
}

impl<A: Sized, T: AsRef<[A]>> ColumnMajor<A, T> {
	pub fn col(&self, x: usize) -> Option<&[A]> {
		if x >= self.width {
			return None;
		}
		Some(&self.inner.as_ref()[(x * self.height)..((x + 1) * self.height)])
	}

	pub fn iter_row(&self, y: usize) -> Option<impl Iterator<Item = &A> + '_> {
		if y >= self.height {
			return None;
		}
		Some((0..self.width).map(move |x| self.get(x, y).expect("Size checked at instantiation")))
	}

	pub fn iter_row_wise(&self) -> impl Iterator<Item = &A> + '_ {
		(0..self.height).flat_map(move |y| {
			(0..self.width).map(move |x| self.get(x, y).expect("Bounds already checked"))
		})
	}

	pub fn iter_column_wise(&self) -> impl Iterator<Item = &A> + '_ {
		(0..self.width).flat_map(move |x| {
			(0..self.height).map(move |y| self.get(x, y).expect("Bounds already checked"))
		})
	}
}

pub trait AsRowMajor<A: Sized> {
	type Output: Sized;
	fn as_row_major(self, width: usize, height: usize) -> Option<RowMajor<A, Self::Output>>;
}

pub trait AsColumnMajor<A: Sized> {
	type Output: Sized;
	fn as_column_major(self, width: usize, height: usize) -> Option<ColumnMajor<A, Self::Output>>;
}

impl<A: Sized, T: AsRef<[A]>> AsRowMajor<A> for T {
	type Output = Self;
	fn as_row_major(self, width: usize, height: usize) -> Option<RowMajor<A, Self>> {
		if self.as_ref().len() == width * height {
			Some(RowMajor {
				width,
				height,
				inner: self,
				_phantom: PhantomData,
			})
		} else {
			None
		}
	}
}

impl<A: Sized, T: AsRef<[A]>> AsColumnMajor<A> for T {
	type Output = Self;
	fn as_column_major(self, width: usize, height: usize) -> Option<ColumnMajor<A, Self::Output>> {
		if self.as_ref().len() == width * height {
			Some(ColumnMajor {
				width,
				height,
				inner: self,
				_phantom: PhantomData,
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
