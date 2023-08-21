use codec::{Decode, Encode};
use frame_support::{Deserialize, Serialize};
use scale_info::TypeInfo;
use sp_core::H256;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, TypeInfo)]
pub struct LightClientStep {
	pub attested_slot: u64,
	pub finalized_slot: u64,
	pub participation: u64,
	pub finalized_header_root: H256,
	pub execution_state_root: H256,
	pub proof: Groth16Proof,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, TypeInfo)]
pub struct LightClientRotate {
	pub step: LightClientStep,
	pub sync_committee_ssz: u64,
	pub sync_committee_poseidon: H256,
	pub proof: Groth16Proof,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, TypeInfo)]
pub struct Groth16Proof {
	pub a: [u64; 2],
	pub b: [[u64; 2]; 2],
	pub c: [u64; 2],
}
