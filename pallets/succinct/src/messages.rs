use ark_bn254::{Bn254, Fr, G1Affine, G2Affine};
use ark_ff::{Fp256, QuadExtField};
use ark_groth16::Proof;
use ark_std::str::FromStr;
use ark_std::string::String;
use ark_std::vec;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{Deserialize, Serialize};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_std::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, TypeInfo)]
pub struct LightClientStep {
	// TODO U256?
	pub attested_slot: u64,
	pub finalized_slot: u64,
	pub participation: u16,
	pub finalized_header_root: H256,
	pub execution_state_root: H256,
	pub proof: Groth16Proof,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, TypeInfo)]
pub struct Groth16Proof {
	pub a: Vec<String>,
	pub b: Vec<Vec<String>>,
	pub c: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircomProof {
	#[serde(rename = "pi_a")]
	pub pi_a: Vec<String>,
	#[serde(rename = "pi_b")]
	pub pi_b: Vec<Vec<String>>,
	#[serde(rename = "pi_c")]
	pub pi_c: Vec<String>,
	pub protocol: String,
	pub curve: String,
}

impl CircomProof {
	pub fn to_proof(self) -> Proof<Bn254> {
		let a = G1Affine::new(
			Fp256::from_str(&self.pi_a[0]).unwrap(),
			Fp256::from_str(&self.pi_a[1]).unwrap(),
			false,
		);
		let b = G2Affine::new(
			QuadExtField::new(
				Fp256::from_str(&self.pi_b[0][0]).unwrap(),
				Fp256::from_str(&self.pi_b[0][1]).unwrap(),
			),
			QuadExtField::new(
				Fp256::from_str(&self.pi_b[1][0]).unwrap(),
				Fp256::from_str(&self.pi_b[1][1]).unwrap(),
			),
			false,
		);

		let c = G1Affine::new(
			Fp256::from_str(&self.pi_c[0]).unwrap(),
			Fp256::from_str(&self.pi_c[1]).unwrap(),
			false,
		);
		Proof { a, b, c }
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicSignals(pub Vec<String>);

impl PublicSignals {
	pub fn from(public_signals: Vec<String>) -> Self {
		PublicSignals(public_signals)
	}

	pub fn get(self) -> Vec<Fr> {
		let mut inputs: Vec<Fr> = Vec::new();
		for input in self.0 {
			inputs.push(Fr::from_str(&input).unwrap());
		}
		inputs
	}
}

// =========

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, TypeInfo)]
pub struct LightClientRotate {
	pub step: LightClientStep,
	pub sync_committee_ssz: u64,
	pub sync_committee_poseidon: H256,
	pub proof: Vec<u8>,
}

#[derive(Clone, Copy, Encode, Decode, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct State {
	pub updater: H256,
	pub genesis_validators_root: H256,
	pub genesis_time: u64,
	pub seconds_per_slot: u64,
	pub slots_per_period: u64,
	pub source_chain_id: u32,
	pub finality_threshold: u16,
	pub head: u64,
	pub consistent: bool,
}

impl Default for State {
	fn default() -> Self {
		Self {
			updater: H256([0u8; 32]),
			genesis_validators_root: H256([0u8; 32]),
			genesis_time: Default::default(),
			seconds_per_slot: Default::default(),
			slots_per_period: Default::default(),
			source_chain_id: Default::default(),
			finality_threshold: Default::default(),
			head: Default::default(),
			consistent: Default::default(),
		}
	}
}
