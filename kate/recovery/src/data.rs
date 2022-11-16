use std::convert::TryInto;

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
	pub fn reference(&self, block: u32) -> String { self.position.reference(block) }

	pub fn data(&self) -> [u8; 32] { self.content[48..].try_into().expect("content is 80 bytes") }

	pub fn proof(&self) -> [u8; 48] { self.content[..48].try_into().expect("content is 80 bytes") }
}

impl From<Cell> for DataCell {
	fn from(cell: Cell) -> Self {
		DataCell {
			position: cell.position.clone(),
			data: cell.data(),
		}
	}
}
