use std::ops::Range;

use serde::{Deserialize, Serialize};

use crate::config::{self, CHUNK_SIZE};

const EXTENSION_FACTOR_U32: u32 = config::EXTENSION_FACTOR as u32;

/// Position of a cell in the the matrix.
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Position {
	pub row: u32,
	pub col: u16,
}

impl Position {
	/// Refrence in format `block_number:column_number:row_number`
	pub fn reference(&self, block_number: u32) -> String {
		format!("{}:{}:{}", block_number, self.col, self.row)
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
#[derive(Debug, Clone)]
pub struct Dimensions {
	rows: u16,
	cols: u16,
}

impl Dimensions {
	/// Creates new matrix dimensions.
	/// Data layout is assumed to be row-wise.
	/// Returns `None` if rows or cols is 0.
	pub const fn new(rows: u16, cols: u16) -> Option<Self> {
		if rows == 0 || cols == 0 {
			return None;
		}
		Some(Dimensions { rows, cols })
	}

	/// Returns number of rows
	pub fn rows(&self) -> u16 {
		self.rows
	}

	/// Returns number of columns
	pub fn cols(&self) -> u16 {
		self.cols
	}

	/// Matrix size.
	pub fn size(&self) -> u32 {
		self.rows as u32 * self.cols as u32
	}

	/// Extended matrix size.
	pub fn extended_size(&self) -> u64 {
		self.extended_rows() as u64 * self.cols as u64
	}

	/// Row size in bytes
	pub fn row_byte_size(&self) -> usize {
		CHUNK_SIZE * self.cols as usize
	}

	/// Extended matrix rows count.
	pub fn extended_rows(&self) -> u32 {
		(self.rows as u32) * EXTENSION_FACTOR_U32
	}

	/// List of data row indexes in the extended matrix.
	pub fn extended_data_rows(&self, cells: Range<u32>) -> Vec<u32> {
		assert!(cells.end <= self.size());
		if cells.end == 0 {
			return vec![];
		}

		let first_row = self.extended_data_row(cells.start);
		let last_row = self.extended_data_row(cells.end - 1);

		(first_row..=last_row)
			.step_by(config::EXTENSION_FACTOR)
			.collect::<Vec<u32>>()
	}

	/// Cell positions for given column in extended matrix.
	/// Empty if column index is not valid.
	pub fn col_positions(&self, col: u16) -> Vec<Position> {
		if self.cols() <= col {
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
		(0..self.cols())
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
		(cell % self.cols as u32) as u16
	}

	/// Extended matrix data row index of cell in the data matrix.
	fn extended_data_row(&self, cell: u32) -> u32 {
		(cell / self.cols as u32) * EXTENSION_FACTOR_U32
	}

	/// Extended matrix data position of a cell in the data matrix.
	fn extended_data_position(&self, cell: u32) -> Position {
		Position {
			col: self.col(cell),
			row: self.extended_data_row(cell),
		}
	}

	/// Extended matrix data positions for given data matrix cells range.
	pub fn extended_data_positions(&self, cells: Range<u32>) -> Vec<Position> {
		assert!(cells.end <= self.size());
		cells
			.map(|cell| self.extended_data_position(cell))
			.collect::<Vec<_>>()
	}

	/// Checks if extended matrix contains given position.
	pub fn extended_contains(&self, position: &Position) -> bool {
		position.row < self.extended_rows() && position.col < self.cols
	}

	/// Creates iterator over rows in extended matrix.
	pub fn iter_extended_rows(&self) -> impl Iterator<Item = u32> {
		0..self.extended_rows()
	}

	/// Creates iterator over data cells in data matrix (used to retrieve data from the matrix).
	pub fn iter_data(&self) -> impl Iterator<Item = (usize, usize)> {
		let rows = self.rows as usize;
		let cols = self.cols as usize;
		(0..rows).flat_map(move |row| (0..cols).map(move |col| (row, col)))
	}

	/// Creates iterator over cell indexes in data matrix (used to store data in the matrix).
	pub fn iter_cells(&self) -> impl Iterator<Item = u32> {
		let rows = self.rows as u32;
		let cols = self.cols;
		(0..cols).flat_map(move |col| (0..rows).map(move |row| row * cols as u32 + col as u32))
	}

	/// Creates iterator over data positions by row in extended matrix.
	pub fn iter_extended_data_positions(&self) -> impl Iterator<Item = (u32, u16)> {
		let rows = self.rows as u32;
		let cols = self.cols;
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
		let cols: u32 = self.cols().into();

		(start..end).map(move |cell| Position {
			row: cell / cols,
			col: (cell % cols) as u16,
		})
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
			.for_each(|(p1, p2)| assert!(p1 == p2));
	}
}
