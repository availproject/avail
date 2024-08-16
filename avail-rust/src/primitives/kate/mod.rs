use crate::U256;
use codec::{Decode, Encode};
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

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

pub type GRawScalar = U256;
pub type GRow = Vec<GRawScalar>;
pub type GDataProof = (GRawScalar, GProof);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(try_from = "Vec<u8>", into = "Vec<u8>")]
pub struct GProof(pub [u8; 48]);

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
