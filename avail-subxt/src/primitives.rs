use codec::{Decode, Encode};
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

pub mod extrinsic_params;
pub use extrinsic_params::AvailExtrinsicParams;

pub mod header;
pub use header::Header;

pub mod app_unchecked_extrinsic;
pub use app_unchecked_extrinsic::AppUncheckedExtrinsic;

pub mod babe;
pub mod grandpa;

/// Compatible with `kate::com::Cell`
#[derive(Clone, Constructor, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Cell {
	#[codec(compact)]
	pub row: u32,
	#[codec(compact)]
	pub col: u32,
}

impl<R, C> From<(R, C)> for Cell
where
	R: Into<u32>,
	C: Into<u32>,
{
	fn from((row, col): (R, C)) -> Self {
		Self {
			row: row.into(),
			col: col.into(),
		}
	}
}
