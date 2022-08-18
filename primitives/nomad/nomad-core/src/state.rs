use frame_support::pallet_prelude::*;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum NomadState {
	/// Contract is active
	Active,
	/// Contract has failed
	Failed,
}

impl Default for NomadState {
	fn default() -> Self { Self::Active }
}
