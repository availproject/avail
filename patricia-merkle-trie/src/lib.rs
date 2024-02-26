//! This crate exposes an implementation of [`trie_db::TrieLayout`] that allows [`trie_db::TrieDb`]
//! be used for verifying Ethereum state proofs as per [EIP-1186](https://eips.ethereum.org/EIPS/eip-1186)

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use core::marker::PhantomData;
use hash_db::Hasher;
use primitive_types::H256;
use trie_db::TrieLayout;

mod node_codec;
mod storage_proof;

#[cfg(test)]
mod tests;

pub use storage_proof::{MemoryDB, StorageProof};

/// Trie layout for EIP-1186 state proof nodes.
#[derive(Default, Clone)]
pub struct EIP1186Layout<H>(PhantomData<H>);

impl<H: Hasher<Out = H256>> TrieLayout for EIP1186Layout<H> {
	const USE_EXTENSION: bool = true;
	const ALLOW_EMPTY: bool = false;
	const MAX_INLINE_VALUE: Option<u32> = None;
	type Hash = H;
	type Codec = node_codec::RlpNodeCodec<H>;
}

pub mod keccak256 {
	use hash256_std_hasher::Hash256StdHasher;
	use sp_io::hashing::keccak_256;

	use super::*;

	/// Concrete implementation of Hasher using Keccak 256-bit hashes
	#[derive(Debug)]
	pub struct KeccakHasher;

	impl hash_db::Hasher for KeccakHasher {
		type Out = H256;
		type StdHasher = Hash256StdHasher;
		const LENGTH: usize = 32;

		fn hash(x: &[u8]) -> Self::Out {
			keccak_256(x).into()
		}
	}
}

/// Keccak hasher implementation, but only for std uses. You'd probably want to delegate
/// hashing to wasm host functions in `no_std`.
#[cfg(feature = "std")]
pub mod keccak {
	use super::*;
	use hash256_std_hasher::Hash256StdHasher;
	use tiny_keccak::{Hasher, Keccak};

	/// Concrete implementation of Hasher using Keccak 256-bit hashes
	#[derive(Debug)]
	pub struct KeccakHasher;

	impl hash_db::Hasher for KeccakHasher {
		type Out = H256;
		type StdHasher = Hash256StdHasher;
		const LENGTH: usize = 32;

		fn hash(x: &[u8]) -> Self::Out {
			keccak_256(x).into()
		}
	}

	/// Performs a Keccak-256 hash on the given input.
	pub fn keccak_256(input: &[u8]) -> [u8; 32] {
		let mut out = [0u8; 32];
		let mut k = Keccak::v256();
		k.update(input);
		k.finalize(&mut out);
		out
	}
}
