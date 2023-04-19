pub mod democracy {
	use derive_more::Constructor;
	use num_enum::TryFromPrimitive;

	use crate::api::runtime_types::pallet_democracy::vote::Vote as NativeVote;

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

	impl TryFrom<NativeVote> for Vote {
		type Error = &'static str;

		fn try_from(vote: NativeVote) -> Result<Self, Self::Error> {
			let byte = vote.0;
			Ok(Vote {
				aye: (byte & 0b1000_0000) == 0b1000_0000,
				conviction: Conviction::try_from(byte & 0b0111_1111)
					.map_err(|_| "Invalid conviction")?,
			})
		}
	}

	impl Into<NativeVote> for Vote {
		fn into(self) -> NativeVote {
			let flag = if self.aye { 0b1000_0000 } else { 0u8 };
			NativeVote(self.conviction as u8 | flag)
		}
	}
}
