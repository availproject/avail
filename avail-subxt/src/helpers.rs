pub mod democracy {
	use derive_more::Constructor;
	use num_enum::TryFromPrimitive;

	/// A number of lock periods, plus a vote, one way or the other.
	#[derive(Copy, Clone, Eq, PartialEq, Default, Constructor)]
	pub struct Vote {
		pub aye: bool,
		pub conviction: Conviction,
	}

	#[repr(u8)]
	#[derive(Copy, Clone, Eq, PartialEq, Default, TryFromPrimitive)]
	pub enum Conviction {
		/// 0.1x votes, unlocked.
		#[default]
		None = 0,
		/// 1x votes, locked for an enactment period following a successful vote.
		Locked1x,
		/// 2x votes, locked for 2x enactment periods following a successful vote.
		Locked2x,
		/// 3x votes, locked for 4x...
		Locked3x,
		/// 4x votes, locked for 8x...
		Locked4x,
		/// 5x votes, locked for 16x...
		Locked5x,
		/// 6x votes, locked for 32x...
		Locked6x,
	}
}
