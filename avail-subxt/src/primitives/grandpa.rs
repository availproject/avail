use codec::{Codec, Decode};
use serde::{Serialize, Serializer};

use crate::api::runtime_types::sp_consensus_grandpa::app::Public;

#[derive(Decode)]
pub struct AuthorityId(pub Public);

impl Serialize for AuthorityId {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let raw = hex::encode(self.0 .0 .0);
		serializer.serialize_str(&raw)
	}
}

pub type AuthorityIndex = u64;
pub type AuthorityWeight = u64;
pub type AuthorityList = Vec<(AuthorityId, AuthorityWeight)>;

#[derive(Decode, Serialize)]
pub struct ScheduledChange<N> {
	/// The new authorities after the change, along with their respective weights.
	pub next_authorities: AuthorityList,
	/// The number of blocks to delay.
	pub delay: N,
}
/// An consensus log item for GRANDPA.
#[derive(Decode, Serialize)]
pub enum ConsensusLog<N: Codec> {
	#[codec(index = 1)]
	ScheduledChange(ScheduledChange<N>),
	#[codec(index = 2)]
	ForcedChange(N, ScheduledChange<N>),
	#[codec(index = 3)]
	OnDisabled(AuthorityIndex),
	#[codec(index = 4)]
	Pause(N),
	#[codec(index = 5)]
	Resume(N),
}
