use frame_support::RuntimeDebug;
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{storage::StateVersion, Hasher};
use sp_runtime::traits::Hash;
use sp_std::vec::Vec;
use sp_trie::{LayoutV0, LayoutV1, TrieConfiguration as _};

/// Keccak 256 wrapper which supports `beefy-merkle-tree::Hasher`.
#[derive(PartialEq, Eq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Keccak256 {}

impl Hasher for Keccak256 {
	type Out = sp_core::H256;
	type StdHasher = hash256_std_hasher::Hash256StdHasher;
	const LENGTH: usize = 32;

	fn hash(s: &[u8]) -> Self::Out {
		sp_io::hashing::keccak_256(s).into()
	}
}

impl Hash for Keccak256 {
	type Output = sp_core::H256;

	fn trie_root(input: Vec<(Vec<u8>, Vec<u8>)>, version: StateVersion) -> Self::Output {
		match version {
			StateVersion::V0 => LayoutV0::<Keccak256>::trie_root(input),
			StateVersion::V1 => LayoutV1::<Keccak256>::trie_root(input),
		}
	}

	fn ordered_trie_root(input: Vec<Vec<u8>>, version: StateVersion) -> Self::Output {
		match version {
			StateVersion::V0 => LayoutV0::<Keccak256>::ordered_trie_root(input),
			StateVersion::V1 => LayoutV1::<Keccak256>::ordered_trie_root(input),
		}
	}
}
