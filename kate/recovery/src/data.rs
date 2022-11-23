use std::{collections::HashMap, convert::TryInto};

use crate::matrix::Position;

/// Position and data of a cell in extended matrix
#[derive(Default, Debug, Clone)]
pub struct DataCell {
	/// Cell's position
	pub position: Position,
	/// Cell's data
	pub data: [u8; 32],
}

/// Position and content of a cell in extended matrix
#[derive(Debug, Clone)]
pub struct Cell {
	/// Cell's position
	pub position: Position,
	/// Cell's data
	pub content: [u8; 80],
}

impl Cell {
	pub fn reference(&self, block: u32) -> String {
		self.position.reference(block)
	}

	pub fn data(&self) -> [u8; 32] {
		self.content[48..].try_into().expect("content is 80 bytes")
	}

	pub fn proof(&self) -> [u8; 48] {
		self.content[..48].try_into().expect("content is 80 bytes")
	}
}

/// Merges cells data per row.
/// Cells are sorted before merge.
pub fn rows(cells: &[Cell]) -> Vec<(u32, Vec<u8>)> {
	let mut sorted_cells = cells.to_vec();
	sorted_cells.sort_by(|a, b| {
		let by_row = a.position.row.cmp(&b.position.row);
		let by_col = a.position.col.cmp(&b.position.col);
		by_row.then(by_col)
	});

	let mut rows = HashMap::new();
	for cell in sorted_cells {
		rows.entry(cell.position.row)
			.or_insert_with(Vec::default)
			.extend(cell.data());
	}

	rows.into_iter().collect::<Vec<(_, _)>>()
}

impl From<Cell> for DataCell {
	fn from(cell: Cell) -> Self {
		DataCell {
			position: cell.position.clone(),
			data: cell.data(),
		}
	}
}

#[cfg(test)]
mod tests {
	use std::convert::TryInto;

	use crate::{data::rows, data::Cell, matrix::Position};

	fn cell(position: Position, content: [u8; 80]) -> Cell {
		Cell { position, content }
	}

	fn position(row: u32, col: u16) -> Position {
		Position { row, col }
	}

	#[test]
	fn rows_ok() {
		fn content(data: [u8; 32]) -> [u8; 80] {
			[&[0u8; 48], &data[..]].concat().try_into().unwrap()
		}

		let cells = [
			cell(position(1, 1), content([3; 32])),
			cell(position(1, 0), content([2; 32])),
			cell(position(0, 0), content([0; 32])),
			cell(position(0, 1), content([1; 32])),
		];

		let mut rows = rows(&cells);
		rows.sort_by_key(|&(key, _)| key);

		let expected = [
			[[0u8; 32], [1u8; 32]].concat(),
			[[2u8; 32], [3u8; 32]].concat(),
		];

		for i in 0..1 {
			let (row_index, row) = &rows[i];
			assert_eq!(*row_index, 0);
			assert_eq!(*row, expected[i]);
		}
	}
}
