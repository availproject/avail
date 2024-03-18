use codec::{Decode, Encode};
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

pub mod extrinsic_params;
pub use extrinsic_params::{new_params_from_app_id, CheckAppId, ExtrinsicParams};

pub mod header;
pub use header::Header;
use sp_core::U256;

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

// TODO: Move all types used in rpc in avail-core so we don't have to create them here
pub type GRawScalar = U256;
pub type GDataProof = (GRawScalar, GProof);

#[derive(Encode, Decode, Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(try_from = "Vec<u8>", into = "Vec<u8>"))]
pub struct GProof([u8; 48]);
impl From<GProof> for Vec<u8> {
	fn from(proof: GProof) -> Self {
		proof.0.to_vec()
	}
}
impl TryFrom<Vec<u8>> for GProof {
	type Error = u32;
	fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
		if data.len() != 48 {
			return Err(data.len() as u32);
		};

		let mut proof = [0u8; 48];
		proof.copy_from_slice(&data);
		Ok(GProof(proof))
	}
}