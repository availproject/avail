/// The dimensions of a grid
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dimensions {
	width: usize,
	height: usize,
}

/// The ways a set of dimensions can be extended
#[derive(Debug, Clone)]
pub struct Extension {
	/// This means extending the height of the grid by some factor.
	/// `2` would mean doubling the grid upwards, increasing the height by a factor of
	/// 2 and multiplying the number of rows by 2
	pub height_factor: usize,
	/// This means extending the width of the grid by some factor.
	/// `2` would mean doubling the grid sideways, increasing the width by a factor of
	/// 2 and multiplying the number of columns by 2
	pub width_factor: usize,
}

impl Extension {
	pub fn height(factor: usize) -> Self {
		Self {
			height_factor: factor,
			width_factor: 1,
		}
	}

	pub fn width(factor: usize) -> Self {
		Self {
			height_factor: 1,
			width_factor: factor,
		}
	}
}

impl Dimensions {
	pub const fn new(width: usize, height: usize) -> Self {
		Dimensions { width, height }
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.height
	}

	pub fn n_cells(&self) -> usize {
		self.width * self.height
	}

	pub fn divides(&self, other: &Self) -> bool {
		other.width() % self.width() == 0 && other.height() % self.height() == 0
	}

	pub fn extend(&self, e: Extension) -> Self {
		Self {
			width: e.width_factor as usize * self.width,
			height: e.height_factor as usize * self.height,
		}
	}
}
