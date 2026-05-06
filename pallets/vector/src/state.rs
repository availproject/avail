use codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use frame_support::{Deserialize, Serialize};
use scale_info::TypeInfo;

/// Configuration struct that holds basic pallet configuration.
#[derive(
	Clone,
	Copy,
	Encode,
	Decode,
	DecodeWithMemTracking,
	Debug,
	PartialEq,
	Eq,
	TypeInfo,
	MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Default)]
pub struct Configuration {
	#[codec(compact)]
	pub slots_per_period: u64,
	#[codec(compact)]
	pub finality_threshold: u16,
}
