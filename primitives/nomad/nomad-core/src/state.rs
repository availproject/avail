use frame_support::pallet_prelude::*;

#[derive(Clone, Copy, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum NomadState {
	/// Contract is active
	Active,
	/// Contract has failed
	Failed,
}

impl Default for NomadState {
	fn default() -> Self { Self::Active }
}
