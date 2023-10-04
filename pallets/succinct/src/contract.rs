use crate::messages::{CircomProof, LightClientStep, PublicSignals};
use crate::verifier::Verifier;
use crate::Error;
use codec::{Decode, Encode};
use frame_support::dispatch::TypeInfo;
use frame_support::{Deserialize, Serialize};
use frame_system::Config;
use sha2::{Digest, Sha256};
use sp_core::U256;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, TypeInfo)]
pub enum ContractError {
	InvalidStepProof,
	SyncCommitteeNotInitialized,
	NotEnoughSyncCommitteeParticipants,
	ProofNotValid,
	VerificationError,
}

impl<T: Config> From<ContractError> for Error<T> {
	fn from(e: ContractError) -> Error<T> {
		match e {
			ContractError::InvalidStepProof => Error::<T>::VerificationError,
			// ContractError::SyncCommitteeNotInitialized => {}
			// ContractError::NotEnoughSyncCommitteeParticipants => {}
			// ContractError::ProofNotValid => {}
			// ContractError::VerificationError => {}
			_ => Error::<T>::VerificationError,
		}
	}
}

pub fn zk_light_client_step(
	update: &LightClientStep,
	sync_committee_poseidon: U256,
) -> Result<bool, ContractError> {
	let mut fs: [u8; 32] = [0u8; 32];
	let mut pc: [u8; 32] = [0u8; 32];

	let finalized_slot_le: [u8; 8] = update.finalized_slot.to_le_bytes();
	let participation_le: [u8; 2] = update.participation.to_le_bytes();
	fs[..finalized_slot_le.len()].copy_from_slice(&finalized_slot_le);
	pc[..participation_le.len()].copy_from_slice(&participation_le);

	let mut h = [0u8; 32];
	let mut temp = [0u8; 64];
	// sha256 & combine inputs
	temp[..32].copy_from_slice(&fs);
	temp[32..].copy_from_slice(&update.finalized_header_root.as_bytes());
	h.copy_from_slice(&Sha256::digest(temp));

	temp[..32].copy_from_slice(&h);
	temp[32..].copy_from_slice(&pc);
	h.copy_from_slice(&Sha256::digest(temp));

	temp[..32].copy_from_slice(&h);
	temp[32..].copy_from_slice(&update.execution_state_root.as_bytes());
	h.copy_from_slice(&Sha256::digest(temp));

	temp[..32].copy_from_slice(&h);
	temp[32..].copy_from_slice(&sync_committee_poseidon.encode());
	h.copy_from_slice(&Sha256::digest(temp));

	// TODO: Confirm this is the correct math!
	let mut t = [255u8; 32];
	t[31] = 0b00011111;

	for i in 0..32 {
		t[i] &= h[i];
	}

	// Set proof
	let inputs_string = U256::from_little_endian(t.as_slice()).to_string();

	let inputs = vec![inputs_string; 1];
	let verifier = Verifier::new_step_verifier();

	let groth_16_proof = update.proof.clone();

	let circom_proof = CircomProof {
		pi_a: groth_16_proof.a,
		pi_b: groth_16_proof.b,
		pi_c: groth_16_proof.c,
		protocol: "groth16".to_string(),
		curve: "bn128".to_string(),
	};

	let proof = circom_proof.to_proof();
	let public_signals = PublicSignals::from(inputs);

	verifier.verify_proof(proof, &public_signals.get())
}
