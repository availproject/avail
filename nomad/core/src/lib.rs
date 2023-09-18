#![cfg_attr(not(feature = "std"), no_std)]

/// Variadic macro used by `keccak256_concat` internally.
#[macro_export]
macro_rules! keccak256_concat_update {
	($hasher:ident, $e:expr) => {{
		$hasher.update($e.as_ref());
	}};

	($hasher:ident, $e:expr, $($es:expr),+) => {{
		$hasher.update($e.as_ref());
		$crate::keccak256_concat_update!($hasher, $($es),+);
	}};
}

/// Calculates the Kecck 256 of arguments with NO extra allocations to join inputs.
#[macro_export]
macro_rules! keccak256_concat{
	($($arg:tt)*) => {{
		{
			use tiny_keccak::Hasher as _;
			let mut output = [0u8; 32];
			let mut hasher = tiny_keccak::Keccak::v256();
			$crate::keccak256_concat_update!(hasher, $($arg)*);
			hasher.finalize(&mut output);
			sp_core::H256::from(output)
		}
	}}
}

extern crate alloc;

mod update_v2;

mod update;
pub use update::*;

mod state;
pub use state::*;

mod nomad_message;
pub use nomad_message::*;

mod typed_message;
pub use typed_message::*;

mod utils;
pub use utils::*;

#[cfg(feature = "std")]
pub mod test_utils;

#[cfg(test)]
mod test {
	use sp_core::H256;
	use sp_runtime::traits::{Hash, Keccak256};

	/// Tests `keccak256_concat` macro expansion with several arguments.
	#[test]
	fn it_keccak_256_contact() {
		let a = b"123";
		let b = b"456";
		let c = b"789";
		let abc = vec![a, b, c]
			.into_iter()
			.flatten()
			.copied()
			.collect::<Vec<_>>();

		// Single
		let output = keccak256_concat!(a);
		let expected = Keccak256::hash(&a[..]);
		assert_eq!(output, expected);

		// Variadic
		let output = keccak256_concat!(a, b, c);
		let expected = Keccak256::hash(abc.as_slice());
		assert_eq!(output, expected);

		// Test `as_ref()`.
		let output = keccak256_concat!(H256::default(), a, b);
		let concat = vec![H256::default().as_bytes(), a, b]
			.into_iter()
			.flatten()
			.copied()
			.collect::<Vec<_>>();
		let expected = Keccak256::hash(concat.as_slice());
		assert_eq!(output, expected);
	}
}
