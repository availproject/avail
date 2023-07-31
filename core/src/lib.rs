#![cfg_attr(not(feature = "std"), no_std)]
#![deny(clippy::integer_arithmetic)]

use codec::{Decode, Encode, MaxEncodedLen};
use derive_more::{Add, Constructor, Deref, Display, Into, Mul};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_arithmetic::traits::Zero;
use sp_core::RuntimeDebug;

pub mod opaque_extrinsic;
pub use opaque_extrinsic::*;

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
pub use app_extrinsic::*;

pub mod constants;
pub use constants::*;

#[cfg(feature = "runtime")]
pub mod bench_randomness;

#[repr(u8)]
pub enum InvalidTransactionCustomId {
	/// The AppId is not registered.
	InvalidAppId = 137,
	/// Extrinsic is not allowed for the given `AppId`.
	ForbiddenAppId,
	/// Max padded length was exceeded.
	MaxPaddedLenExceeded,
	/// Max recursion was reached for a call with AppId != 0.
	MaxRecursionExceeded,
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
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
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
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[mul(forward)]
pub struct BlockLengthColumns(#[codec(compact)] pub u32);

/// Strong type for `BlockLength::rows`
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
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
