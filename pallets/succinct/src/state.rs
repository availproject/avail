use ark_bn254::{Bn254, Fr, G1Affine, G2Affine};
use ark_ff::QuadExtField;
use ark_groth16::Proof;
use ark_std::str::FromStr;
use ark_std::string::String;
use ark_std::string::ToString;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{Deserialize, Serialize};
use scale_info::TypeInfo;
use sp_core::{H256, U256};
use sp_std::prelude::*;

use crate::verifier::{str_to_fq, VerificationError};

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
	pub fn new(a: Vec<String>, b: Vec<Vec<String>>, c: Vec<String>) -> Self {
		CircomProof {
			pi_a: a,
			pi_b: b,
			pi_c: c,
			protocol: "groth16".to_string(),
			curve: "bn128".to_string(),
		}
	}

	pub fn proof(self) -> Result<Proof<Bn254>, VerificationError> {
		let a = G1Affine::new(str_to_fq(&self.pi_a[0])?, str_to_fq(&self.pi_a[1])?, false);
		let b = G2Affine::new(
			QuadExtField::new(str_to_fq(&self.pi_b[0][0])?, str_to_fq(&self.pi_b[0][1])?),
			QuadExtField::new(str_to_fq(&self.pi_b[1][0])?, str_to_fq(&self.pi_b[1][1])?),
			false,
		);

		let c = G1Affine::new(str_to_fq(&self.pi_c[0])?, str_to_fq(&self.pi_c[1])?, false);
		Ok(Proof { a, b, c })
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicSignals(pub Vec<String>);

impl PublicSignals {
	pub fn from(public_signals: Vec<String>) -> Self {
		PublicSignals(public_signals)
	}

	pub fn get(self) -> Result<Vec<Fr>, VerificationError> {
		let mut inputs: Vec<Fr> = Vec::new();
		for input in self.0 {
			let fr = Fr::from_str(&input).map_err(|_| VerificationError::InvalidVK)?;
			inputs.push(fr);
		}
		Ok(inputs)
	}
}

#[derive(Clone, Copy, Encode, Decode, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Default)]
pub struct Configuration {
	pub slots_per_period: u64,
	pub finality_threshold: u16,
}

/// VerifiedStep struct that holds verified params from a step call.
#[derive(Default, Debug)]
pub struct VerifiedStep {
	pub verified_function_id: H256,
	pub verified_input_hash: H256,
	pub verified_output: VerifiedStepOutput,
}

impl VerifiedStep {
	pub(crate) const fn new(
		verified_function_id: H256,
		verified_input_hash: H256,
		verified_output: VerifiedStepOutput,
	) -> VerifiedStep {
		VerifiedStep {
			verified_function_id,
			verified_input_hash,
			verified_output,
		}
	}
}

/// VerifiedRotate struct that holds verified params from a rotate call.
#[derive(Default)]
pub struct VerifiedRotate {
	pub verified_function_id: H256,
	pub verified_input_hash: H256,
	pub sync_committee_poseidon: U256,
}

impl VerifiedRotate {
	pub(crate) const fn new(
		verified_function_id: H256,
		verified_input_hash: H256,
		sync_committee_poseidon: U256,
	) -> VerifiedRotate {
		VerifiedRotate {
			verified_function_id,
			verified_input_hash,
			sync_committee_poseidon,
		}
	}
}

/// VerifiedStepOutput struct that holds a step output params.
#[derive(Clone, Copy, Encode, Decode, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Default)]
pub struct VerifiedStepOutput {
	pub finalized_header_root: H256,
	pub execution_state_root: H256,
	pub finalized_slot: u64,
	pub participation: u16,
}

pub fn parse_rotate_output(output: Vec<u8>) -> U256 {
	U256::from_big_endian(output.as_slice())
}

pub fn parse_step_output(output: Vec<u8>) -> VerifiedStepOutput {
	let mut finalized_header_root: [u8; 32] = [0; 32];
	let mut execution_state_root: [u8; 32] = [0; 32];
	let mut finalized_slot: [u8; 8] = [0; 8];
	let mut participation: [u8; 2] = [0; 2];

	finalized_header_root[..32].copy_from_slice(&output[..32]);
	execution_state_root[..32].copy_from_slice(&output[32..64]);

	finalized_slot[..8].copy_from_slice(&output[64..72]);
	participation[..2].copy_from_slice(&output[72..74]);

	VerifiedStepOutput {
		finalized_header_root: H256(finalized_header_root),
		execution_state_root: H256(execution_state_root),
		finalized_slot: u64::from_be_bytes(finalized_slot),
		participation: u16::from_be_bytes(participation),
	}
}

#[cfg(test)]
mod tests {
	use hex_literal::hex;
	use sp_core::H256;

	use crate::state::{parse_rotate_output, parse_step_output};

	#[test]
	fn test_step_input() {
		let input = hex!("e4566e0cf4edb171a3eedd59f9943bbcd0b1f6b648f1a6e26d5264b668ab41ec51e76629b32b943497207e7b7ccff8fbc12e9e6d758cc7eed972422c4cad02b90000000000747fa001fd");
		let pars = parse_step_output(input.to_vec());

		assert_eq!(509, pars.participation);
		assert_eq!(7634848, pars.finalized_slot);
		assert_eq!(
			H256(hex!(
				"e4566e0cf4edb171a3eedd59f9943bbcd0b1f6b648f1a6e26d5264b668ab41ec"
			)),
			pars.finalized_header_root
		);
		assert_eq!(
			H256(hex!(
				"51e76629b32b943497207e7b7ccff8fbc12e9e6d758cc7eed972422c4cad02b9"
			)),
			pars.execution_state_root
		);
	}

	#[test]
	fn test_rotate_input() {
		let input = hex!("7797dbd1eecad8fe38dd849c43b7ea9a6e9e656c968056415132be4e3bfcd4ed");
		let poseidon = parse_rotate_output(input.to_vec());

		assert_eq!(
			"54093540030416808909802883566252424299549864556922470137474442232175269827821",
			poseidon.to_string()
		);
	}
}
