use core::num::NonZeroUsize;

/// The dimensions of a grid
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Dimensions {
	width: NonZeroUsize,
	height: NonZeroUsize,
}

impl Dimensions {
	pub const fn new(width: NonZeroUsize, height: NonZeroUsize) -> Self {
		Dimensions { width, height }
	}

    /// Make a new `Dimensions` panicking if either width or height are zero.
    /// Again, **this will panic if a zero width or zero height are given**.
	pub const fn new_unchecked(width: usize, height: usize) -> Self {
		Self {
			width: nonzero_unchecked(width),
			height: nonzero_unchecked(height),
		}
	}

	pub fn width(&self) -> usize {
		self.width.get()
	}

	pub fn width_nz(&self) -> NonZeroUsize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.height.get()
	}

	pub fn height_nz(&self) -> NonZeroUsize {
		self.height
	}

	pub fn n_cells(&self) -> usize {
		self.width.saturating_mul(self.height).get()
	}

	pub fn divides(&self, other: &Self) -> bool {
		other.width.get() % self.width == 0 && other.height.get() % self.height == 0
	}

	pub fn extend(&self, e: Extension) -> Self {
		Self {
			width: e.width_factor.saturating_mul(self.width),
			height: e.height_factor.saturating_mul(self.height),
		}
	}
}

/// The ways a set of dimensions can be extended
#[derive(Debug, Clone)]
pub struct Extension {
	/// This means extending the height of the grid by some factor.
	/// `2` would mean doubling the grid upwards, increasing the height by a factor of
	/// 2 and multiplying the number of rows by 2
	pub height_factor: NonZeroUsize,
	/// This means extending the width of the grid by some factor.
	/// `2` would mean doubling the grid sideways, increasing the width by a factor of
	/// 2 and multiplying the number of columns by 2
	pub width_factor: NonZeroUsize,
}

impl Extension {
	pub const fn height(factor: NonZeroUsize) -> Self {
		Self {
			height_factor: factor,
			width_factor: nonzero_unchecked(1),
		}
	}

    /// Make a new height extension without checking if `factor` is nonzero.
    /// Again, **this will panic if a zero `factor` is given**.
	pub const fn height_unchecked(factor: usize) -> Self {
		Self {
			height_factor: nonzero_unchecked(factor),
			width_factor: nonzero_unchecked(1),
		}
	}

	pub const fn width(factor: NonZeroUsize) -> Self {
		Self {
			height_factor: nonzero_unchecked(1),
			width_factor: factor,
		}
	}

    /// Make a new width extension without checking if `factor` is nonzero.
    /// Again, **this will panic if a zero `factor` is given**.
	pub const fn width_unchecked(factor: usize) -> Self {
		Self {
			height_factor: nonzero_unchecked(1),
			width_factor: nonzero_unchecked(factor),
		}
	}
}

#[allow(unconditional_panic)]
const fn nonzero_unchecked(a: usize) -> NonZeroUsize {
	// Hack to get around not being able to unwrap in a const context
	match NonZeroUsize::new(a) {
		Some(a) => a,
		None => [][0],
	}
}
