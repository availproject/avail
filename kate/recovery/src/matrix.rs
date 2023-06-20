#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use std::{
	convert::TryInto,
	fmt::{Display, Formatter, Result},
	num::NonZeroU16,
	ops::{Mul, Range},
};

use crate::config::{self, CHUNK_SIZE};

const EXTENSION_FACTOR_U32: u32 = config::EXTENSION_FACTOR as u32;

/// Position of a cell in the the matrix.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Position {
	pub row: u32,
	pub col: u16,
}

impl<R, C> From<(R, C)> for Position
where
	R: Into<u32>,
	C: Into<u16>,
{
	fn from(row_col: (R, C)) -> Self {
		Self {
			row: row_col.0.into(),
			col: row_col.1.into(),
		}
	}
}

impl<R, C> From<Position> for (R, C)
where
	R: From<u32>,
	C: From<u16>,
{
	fn from(p: Position) -> (R, C) {
		(p.row.into(), p.col.into())
	}
}

impl Display for Position {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		f.write_fmt(format_args!("{}:{}", self.col, self.row))
	}
}

impl Position {
	/// Refrence in format `block_number:column_number:row_number`
	pub fn reference(&self, block_number: u32) -> String {
		format!("{}:{}", block_number, self)
	}

	/// Checks if position is from extended row
	pub fn is_extended(&self) -> bool {
		self.row % 2 == 1
	}
}

/// Matrix partition (column-wise)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Partition {
	pub number: u8,
	pub fraction: u8,
}

/// Matrix row index
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RowIndex(pub u32);

impl RowIndex {
	/// Refrence in format `block_number:row_number`
	pub fn reference(&self, block_number: u32) -> String {
		format!("{}:{}", block_number, self.0)
	}
}

/// Dimensions of a non-extended matrix.
/// Extended matrix (with factor of 2) is a matrix where data is in odd rows and even rows contains erasure codes.
/// Matrix is represented as list of cells, concatenated column by column, which is optimized for erasure coding.
/// Data is stored in matrix row-wise, which means that accessing data is not optimal.
/// Dimensions struct provides functions to index specific data cells and rows in matrix representation.
///
/// # Example of 2x4 matrix
///
/// Data: [1,2,3,4,5,6,7,8]  
/// Data rows: [1,2,3,4], [5,6,7,8]  
/// Columns: [1,5], [2,6], [3,7], [4,8]  
/// Extended columns (EC is erasure code): [1,EC,5,EC], [2,EC,6,EC], [3,EC,7,EC], [4,EC,8,EC]  
/// Matrix representation: [1,5,2,6,3,7,4,8]  
/// Extended matrix representation: [1,EC,5,EC,2,EC,6,EC,3,EC,7,EC,4,EC,8,EC]
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct Dimensions {
	rows: NonZeroU16,
	cols: NonZeroU16,
}

impl<R, C> From<(R, C)> for Dimensions
where
	R: Into<NonZeroU16>,
	C: Into<NonZeroU16>,
{
	fn from(rows_cols: (R, C)) -> Self {
		let (rows, cols) = rows_cols;
		Self {
			rows: rows.into(),
			cols: cols.into(),
		}
	}
}

impl<R, C> From<Dimensions> for (R, C)
where
	R: From<u16>,
	C: From<u16>,
{
	fn from(d: Dimensions) -> Self {
		(d.rows.get().into(), d.cols.get().into())
	}
}

impl Dimensions {
	pub fn new<R: TryInto<NonZeroU16>, C: TryInto<NonZeroU16>>(rows: R, cols: C) -> Option<Self> {
		let rows = rows.try_into().ok()?;
		let cols = cols.try_into().ok()?;

		Some(Self { rows, cols })
	}

	pub fn new_from<R: TryInto<u16>, C: TryInto<u16>>(rows: R, cols: C) -> Option<Self> {
		let rows: u16 = rows.try_into().ok()?;
		let cols: u16 = cols.try_into().ok()?;

		Self::new(rows, cols)
	}

	/// Creates a `Dimension` without checking whether parameters are non-zero. This results in
	/// undefined behaviour if any parameter is zero.
	///
	/// # Safety
	/// Parameters `rows` and `cols` must not be zero.
	pub const unsafe fn new_unchecked(rows: u16, cols: u16) -> Self {
		Self {
			rows: NonZeroU16::new_unchecked(rows),
			cols: NonZeroU16::new_unchecked(cols),
		}
	}

	/// Returns number of rows
	pub fn rows(&self) -> NonZeroU16 {
		self.rows
	}

	/// Returns number of columns
	pub fn cols(&self) -> NonZeroU16 {
		self.cols
	}

	/// Matrix size.
	pub fn size<T: From<u16> + Mul<Output = T>>(&self) -> T {
		T::from(self.rows.get()) * T::from(self.cols.get())
	}

	pub fn divides(&self, other: &Self) -> bool {
		other.cols.get() % self.cols == 0u16 && other.rows.get() % self.rows == 0u16
	}

	/// Extends rows by `row_factor` and cols by `col_factor`.
	pub fn extend(&self, row_factor: NonZeroU16, col_factor: NonZeroU16) -> Option<Self> {
		let rows = self.rows.checked_mul(row_factor)?;
		let cols = self.cols.checked_mul(col_factor)?;

		Some(Self { rows, cols })
	}

	/// Extended matrix size.
	pub fn extended_size(&self) -> u32 {
		self.extended_rows() * u32::from(self.cols.get())
	}

	/// Row size in bytes
	pub fn row_byte_size(&self) -> usize {
		CHUNK_SIZE * usize::from(self.cols.get())
	}

	/// Extended matrix rows count.
	pub fn extended_rows(&self) -> u32 {
		u32::from(self.rows.get()) * EXTENSION_FACTOR_U32
	}

	/// List of data row indexes in the extended matrix.
	pub fn extended_data_rows(&self, cells: Range<u32>) -> Option<Vec<u32>> {
		// Invalid range returns `None`
		if cells.end > self.size() || cells.end == 0 {
			return None;
		}

		let first_row = self.extended_data_row(cells.start);
		let last_row = self.extended_data_row(cells.end - 1);

		let data = (first_row..=last_row)
			.step_by(config::EXTENSION_FACTOR)
			.collect::<Vec<u32>>();
		Some(data)
	}

	/// Cell positions for given column in extended matrix.
	/// Empty if column index is not valid.
	pub fn col_positions(&self, col: u16) -> Vec<Position> {
		if self.cols().get() <= col {
			return vec![];
		}
		(0..self.extended_rows())
			.map(|row| Position { col, row })
			.collect::<Vec<_>>()
	}

	/// Cell positions for given rows in extended matrix.
	/// Empty if row index is not valid.
	fn extended_row_positions(&self, row: u32) -> Vec<Position> {
		if self.extended_rows() <= row {
			return vec![];
		}
		(0..self.cols().get())
			.map(|col| Position { col, row })
			.collect::<Vec<_>>()
	}

	/// Cell positions for given rows in extended matrix.
	/// Row indexes that are out of bounds are ignored.
	pub fn extended_rows_positions(&self, rows: &[u32]) -> Vec<Position> {
		rows.iter()
			.flat_map(|&row| self.extended_row_positions(row))
			.collect::<Vec<_>>()
	}

	/// Column index of a cell in the matrix.
	fn col(&self, cell: u32) -> u16 {
		(cell % u32::from(self.cols.get())) as u16
	}

	/// Extended matrix data row index of cell in the data matrix.
	fn extended_data_row(&self, cell: u32) -> u32 {
		(cell / u32::from(self.cols.get())) * EXTENSION_FACTOR_U32
	}

	/// Extended matrix data position of a cell in the data matrix.
	fn extended_data_position(&self, cell: u32) -> Position {
		Position {
			col: self.col(cell),
			row: self.extended_data_row(cell),
		}
	}

	/// Extended matrix data positions for given data matrix cells range.
	pub fn extended_data_positions(&self, cells: Range<u32>) -> Option<Vec<Position>> {
		(cells.end <= self.size()).then(|| {
			cells
				.map(|cell| self.extended_data_position(cell))
				.collect::<Vec<_>>()
		})
	}

	/// Checks if extended matrix contains given position.
	pub fn extended_contains(&self, position: &Position) -> bool {
		position.row < self.extended_rows() && position.col < self.cols.get()
	}

	/// Creates iterator over rows in extended matrix.
	pub fn iter_extended_rows(&self) -> impl Iterator<Item = u32> {
		0..self.extended_rows()
	}

	/// Creates iterator over data cells in data matrix (used to retrieve data from the matrix).
	pub fn iter_data(&self) -> impl Iterator<Item = (usize, usize)> {
		let rows = self.rows.get().into();
		let cols = self.cols.get().into();
		(0..rows).flat_map(move |row| (0..cols).map(move |col| (row, col)))
	}

	/// Creates iterator over cell indexes in data matrix (used to store data in the matrix).
	pub fn iter_cells(&self) -> impl Iterator<Item = u32> {
		let rows: u32 = self.rows.get().into();
		let cols: u32 = self.cols.get().into();
		(0..cols).flat_map(move |col| (0..rows).map(move |row| row * cols + col))
	}

	/// Creates iterator over data positions by row in extended matrix.
	pub fn iter_extended_data_positions(&self) -> impl Iterator<Item = (u32, u16)> {
		let rows: u32 = self.rows.get().into();
		let cols = self.cols.get();
		(0..rows).flat_map(move |row| (0..cols).map(move |col| (row * EXTENSION_FACTOR_U32, col)))
	}

	/// Generates cell positions for given block partition
	pub fn iter_extended_partition_positions(
		&self,
		partition: &Partition,
	) -> impl Iterator<Item = Position> {
		let size = (self.extended_size() as f64 / partition.fraction as f64).ceil() as u32;
		let start = size * (partition.number - 1) as u32;
		let end = size * (partition.number as u32);
		let cols: u32 = self.cols.get().into();

		(start..end).map(move |cell| Position {
			row: cell / cols,
			col: (cell % cols) as u16,
		})
	}

	pub fn transpose(self) -> Self {
		Self {
			rows: self.cols,
			cols: self.rows,
		}
	}
}

#[cfg(test)]
mod tests {

	use test_case::test_case;

	use crate::matrix::{Dimensions, Position};

	use super::Partition;

	#[test]
	fn zero_dimensions() {
		assert!(Dimensions::new(0, 0).is_none());
		assert!(Dimensions::new(0, 1).is_none());
		assert!(Dimensions::new(1, 0).is_none());
	}

	#[test_case(2, 4, vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 0), (1, 1), (1, 2), (1, 3)] ; "2*4 matrix data iteration")]
	fn iter_data(rows: u16, cols: u16, expected: Vec<(usize, usize)>) {
		let dimensions = Dimensions::new(rows, cols).unwrap();
		let cells = dimensions.iter_data().collect::<Vec<_>>();
		assert_eq!(cells, expected);
	}

	#[test]
	fn iter_cells() {
		let dimensions = Dimensions::new(2, 4).unwrap();
		let cells = dimensions.iter_cells().collect::<Vec<_>>();
		assert_eq!(cells, vec![0, 4, 1, 5, 2, 6, 3, 7]);
	}

	#[test_case(&[0], &[(0, 0), (0, 1)] ; "first row positions")]
	#[test_case(&[1], &[(1, 0), (1, 1)] ; "second row positions")]
	#[test_case(&[0,1], &[(0, 0), (0, 1), (1,0), (1,1)] ; "all positions")]
	#[test_case(&[2], &[] ; "no positions")]
	fn extended_rows_positions(rows: &[u32], expected: &[(usize, usize)]) {
		let dimensions = Dimensions::new(1, 2).unwrap();
		let cells = dimensions
			.extended_rows_positions(rows)
			.iter()
			.map(|&Position { row, col }| (row as usize, col as usize))
			.collect::<Vec<_>>();
		assert_eq!(cells, expected);
	}

	#[test_case(1, 2, &[(0, 0), (0, 1), (0, 2), (0, 3)] ; "First partition")]
	#[test_case(2, 2, &[(1, 0), (1, 1), (1, 2), (1, 3)] ; "Second partition")]
	fn iter_extended_partition_positions(number: u8, fraction: u8, expected: &[(u32, u16)]) {
		Dimensions::new(1, 4)
			.unwrap()
			.iter_extended_partition_positions(&Partition { number, fraction })
			.zip(expected.iter().map(|&(row, col)| Position { row, col }))
			.for_each(|(p1, p2)| assert_eq!(p1, p2));
	}
}
