use std::ops::Range;

use serde::{Deserialize, Serialize};

use crate::config;

const EXTENSION_FACTOR_U32: u32 = config::EXTENSION_FACTOR as u32;

/// Position of a cell in the the matrix.
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub struct Position {
	pub row: u32,
	pub col: u16,
}

impl Position {
	/// Refrence in format `block_number:column_number:row_number`
	pub fn reference(&self, block_number: u32) -> String {
		format!("{}:{}:{}", block_number, self.col, self.row)
	}
}

/// Matrix partition (column-wise)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Partition {
	pub number: u8,
	pub fraction: u8,
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
	pub rows: u16,
	pub cols: u16,
}

impl Dimensions {
	/// Creates new matrix dimensions.
	/// Data layout is assumed to be row-wise.
	pub const fn new(rows: u16, cols: u16) -> Self { Dimensions { rows, cols } }

	/// Matrix size.
	pub fn size(&self) -> u32 { self.rows as u32 * self.cols as u32 }

	/// Extended matrix size.
	pub fn extended_size(&self) -> u32 { self.extended_rows() * self.cols as u32 }

	/// Extended matrix rows count.
	pub fn extended_rows(&self) -> u32 { (self.rows as u32) * EXTENSION_FACTOR_U32 }

	/// List of data row indexes in the extended matrix.
	pub fn extended_data_rows(&self, cells: Range<u32>) -> Vec<u32> {
		assert!(cells.end <= self.size());

		let first_row = self.extended_data_row(cells.start);
		let last_row = self.extended_data_row(cells.end - 1);

		(first_row..=last_row)
			.step_by(config::EXTENSION_FACTOR)
			.collect::<Vec<u32>>()
	}

	/// Column index of a cell in the matrix.
	fn col(&self, cell: u32) -> u16 { (cell % self.cols as u32) as u16 }

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
		(position.row as u32) < self.extended_rows() && position.col < self.cols
	}

	/// Creates iterator over rows in extended matrix.
	pub fn iter_extended_rows(&self) -> impl Iterator<Item = u32> { 0..self.extended_rows() }

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
		let extended_rows = self.extended_rows();

		(start..end).map(move |cell| Position {
			row: cell % extended_rows,
			col: (cell / extended_rows) as u16,
		})
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	use crate::matrix::Dimensions;

	#[test_case(2, 4, vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 0), (1, 1), (1, 2), (1, 3)] ; "2*4 matrix data iteration")]
	fn iter_data(rows: u16, cols: u16, expected: Vec<(usize, usize)>) {
		let dimensions = Dimensions::new(rows, cols);
		let cells = dimensions.iter_data().collect::<Vec<_>>();
		assert_eq!(cells, expected);
	}

	#[test]
	fn iter_cells() {
		let dimensions = Dimensions::new(2, 4);
		let cells = dimensions.iter_cells().collect::<Vec<_>>();
		assert_eq!(cells, vec![0, 4, 1, 5, 2, 6, 3, 7]);
	}
}
