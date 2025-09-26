pub use impl_serde::serialize as bytes;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub struct Bytes(#[serde(with = "bytes")] pub Vec<u8>);

impl From<Vec<u8>> for Bytes {
	fn from(s: Vec<u8>) -> Self {
		Bytes(s)
	}
}

impl Deref for Bytes {
	type Target = [u8];
	fn deref(&self) -> &[u8] {
		&self.0[..]
	}
}

impl std::str::FromStr for Bytes {
	type Err = bytes::FromHexError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		bytes::from_hex(s).map(Bytes)
	}
}
