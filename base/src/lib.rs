#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use hex_literal::hex;
use scale_info::TypeInfo;
use sp_core::H256;

#[cfg(feature = "std")]
pub mod metrics;

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct MRoots {
	pub data_root: H256,
	pub bridge_root: H256,
}

/// Kecckak_256 of `H256::zero`
pub const KECCAK_OF_ZERO: H256 = H256(hex!(
	"290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563"
));
impl Default for MRoots {
	fn default() -> Self {
		Self {
			data_root: KECCAK_OF_ZERO,
			bridge_root: KECCAK_OF_ZERO,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use sp_io::hashing::keccak_256;

	// Default roots are just keccak_256(H256::zero())
	#[test]
	fn check_default_mroots() {
		let mroots = MRoots::default();
		let kecckak_of_zero: H256 = keccak_256(H256::zero().as_bytes()).into();

		assert_eq!(mroots.data_root, kecckak_of_zero);
		assert_eq!(mroots.bridge_root, kecckak_of_zero)
	}
}
