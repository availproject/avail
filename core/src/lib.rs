#![cfg_attr(not(feature = "std"), no_std)]
#![deny(clippy::arithmetic_side_effects)]

use codec::{Decode, Encode, MaxEncodedLen};
use derive_more::{Add, Constructor, Deref, Display, Into, Mul};
use scale_info::TypeInfo;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use sp_arithmetic::traits::Zero;
use sp_core::RuntimeDebug;

pub mod opaque_extrinsic;
pub use opaque_extrinsic::*;

/// DA Block
#[cfg(feature = "runtime")]
pub mod da_block;
#[cfg(feature = "runtime")]
pub use da_block::*;

/// Customized headers.
#[cfg(feature = "runtime")]
pub mod header;

/// Kate Commitment on Headers.
pub mod kate_commitment;
pub use kate_commitment::*;

/// Application Specific Data Retrieval
#[cfg(feature = "runtime")]
pub mod asdr;

pub mod sha2;
pub use sha2::ShaTwo256;

pub mod traits;

pub mod keccak256;
pub use keccak256::Keccak256;

pub mod data_proof;
pub use data_proof::DataProof;

pub mod data_lookup;
pub use data_lookup::*;

pub mod app_extrinsic;
pub use app_extrinsic::AppExtrinsic;

pub mod constants;
pub use constants::*;

pub mod header_version;
pub use header_version::HeaderVersion;

#[cfg(feature = "runtime")]
pub mod bench_randomness;

#[repr(u8)]
pub enum InvalidTransactionCustomId {
	/// The AppId is not registered.
	InvalidAppId = 137,
	/// Extrinsic is not allowed for the given `AppId`.
	ForbiddenAppId = 138,
	/// Max recursion was reached for a call with AppId != 0.
	MaxRecursionExceeded = 139,
}

#[derive(
	Clone,
	Copy,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Add,
	Deref,
	TypeInfo,
	Encode,
	Decode,
	Default,
	Into,
	MaxEncodedLen,
	RuntimeDebug,
	Display,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AppId(#[codec(compact)] pub u32);

impl Zero for AppId {
	fn zero() -> Self {
		AppId(Zero::zero())
	}

	fn is_zero(&self) -> bool {
		self.0.is_zero()
	}
}

/// Strong type for `BlockLength::cols`
#[derive(
	Clone,
	Copy,
	Add,
	Mul,
	PartialEq,
	Eq,
	Encode,
	Decode,
	TypeInfo,
	PartialOrd,
	Ord,
	Into,
	Constructor,
	MaxEncodedLen,
	Display,
	Default,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize, Debug))]
#[mul(forward)]
pub struct BlockLengthColumns(#[codec(compact)] pub u32);

/// Strong type for `BlockLength::rows`
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize, Debug))]
#[derive(
	Encode,
	Decode,
	TypeInfo,
	MaxEncodedLen,
	Clone,
	Copy,
	Add,
	Mul,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Into,
	Constructor,
	Display,
	Default,
)]
#[mul(forward)]
pub struct BlockLengthRows(#[codec(compact)] pub u32);

/// Return Err of the expression: `return Err($expression);`.
///
/// Used as `fail!(expression)`.
#[macro_export]
macro_rules! fail {
	( $y:expr ) => {{
		return Err($y.into());
	}};
}

/// Evaluate `$x:expr` and if not true return `Err($y:expr)`.
///
/// Used as `ensure!(expression_to_ensure, expression_to_return_on_false)`.
#[macro_export]
macro_rules! ensure {
	( $x:expr, $y:expr $(,)? ) => {{
		if !$x {
			$crate::fail!($y);
		}
	}};
}

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
