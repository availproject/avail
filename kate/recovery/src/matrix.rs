use std::ops::Range;

use crate::config;

enum Layout {
	Row,
	// Column,
}

/// Position in a data matrix
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub struct Position {
	pub row: u32,
	pub col: u16,
}

impl Position {
	/// Refrence in format `block_number:column_number:row_number`
	pub fn reference(&self, block_number: u64) -> String {
		format!("{}:{}:{}", block_number, self.col, self.row)
	}
}

pub struct Dimensions {
	pub rows: u16,
	pub cols: u16,
	layout: Layout,
}

const EXTENSION_FACTOR_U32: u32 = config::EXTENSION_FACTOR as u32;

impl Dimensions {
	pub fn row_wise(rows: u16, cols: u16) -> Self {
		Dimensions {
			rows,
			cols,
			layout: Layout::Row,
		}
	}

	pub fn extended_rows(&self) -> u32 { (self.rows as u32) * EXTENSION_FACTOR_U32 }

	fn size(&self) -> u32 { self.rows as u32 * self.cols as u32 }

	// fn extended_size(&self) -> u64 { self.extended_rows() as u64 * self.cols as u64 }

	pub fn contains(&self, position: &Position) -> bool {
		(position.row as u32) < self.extended_rows() && position.col < self.cols
	}

	pub fn data_rows(&self, cells_range: Range<u32>) -> Vec<u32> {
		let first_row = self.row(cells_range.start);
		let last_row = self.row(cells_range.end - 1);

		(first_row..=last_row)
			.map(|row| row as u32 * EXTENSION_FACTOR_U32)
			.collect::<Vec<u32>>()
	}

	pub fn row(&self, cell: u32) -> u16 {
		match &self.layout {
			Layout::Row => (cell / self.cols as u32) as u16,
			// Layout::Column => (cell / self.rows as u32) as u16,
		}
	}

	pub fn col(&self, cell: u32) -> u16 {
		match &self.layout {
			Layout::Row => (cell % self.cols as u32) as u16,
			// Layout::Column => (cell % self.rows as u32) as u16,
		}
	}

	// TODO: Doc position in extended matrix
	fn position(&self, cell: u32) -> Position {
		Position {
			col: self.col(cell),
			row: self.row(cell) as u32 * EXTENSION_FACTOR_U32,
		}
	}

	pub fn positions(&self, cells: Range<u32>) -> Vec<Position> {
		assert!(cells.end < self.size());
		cells.map(|cell| self.position(cell)).collect::<Vec<_>>()
	}

	pub fn iter_cells_by_row(&self) -> impl Iterator<Item = u32> {
		let rows = self.rows as u32;
		let cols = self.cols;
		(0..rows).flat_map(move |row| (0..cols).map(move |col| col as u32 * rows + row))
	}

	pub fn iter_data_cells_by_row(&self) -> impl Iterator<Item = (u32, u16)> {
		let rows = self.rows as u32;
		let cols = self.cols;
		(0..rows).flat_map(move |row| (0..cols).map(move |col| (row * EXTENSION_FACTOR_U32, col)))
	}
}

#[cfg(test)]
mod tests {}
