use crate::messages::{CircomProof, Groth16Proof, LightClientStep, PublicSignals, State};
use crate::verifier::Verifier;
use crate::Error;
use ark_std::string::ToString;
use ark_std::vec;
use codec::{Decode, Encode};
use frame_support::dispatch::TypeInfo;
use frame_support::{assert_ok, Deserialize, Serialize};
use frame_system::Config;
use hex_literal::hex;
use sha2::{Digest, Sha256};
use sp_core::{H256, U256};

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

#[test]
fn test_zk_step() {
	let state: State = State {
		updater: H256([0u8; 32]),
		genesis_validators_root: H256([0u8; 32]),
		genesis_time: 1696404557,
		seconds_per_slot: 12000,
		slots_per_period: 8192,
		source_chain_id: 0,
		finality_threshold: 1,
		head: 0,
		consistent: true,
	};

	let finalized_header_root = H256(hex!(
		"70d0a7f53a459dd88eb37c6cfdfb8c48f120e504c96b182357498f2691aa5653"
	));
	let execution_state_root = H256(hex!(
		"69d746cb81cd1fb4c11f4dcc04b6114596859b518614da0dd3b4192ff66c3a58"
	));
	let sync_committee_poseidon = U256::from_dec_str(
		"7032059424740925146199071046477651269705772793323287102921912953216115444414",
	)
	.unwrap();

	let lcs = LightClientStep {
		attested_slot: 0,
		finalized_slot: 4359840,
		participation: 432,
		finalized_header_root,
		execution_state_root,
		proof: Groth16Proof {
			a: vec![
				"14717729948616455402271823418418032272798439132063966868750456734930753033999"
					.to_string(),
				"10284862272179454279380723177303354589165265724768792869172425850641532396958"
					.to_string(),
			],
			b: vec![
				vec![
					"11269943315518713067124801671029240901063146909738584854987772776806315890545"
						.to_string(),
					"20094085308485991030092338753416508135313449543456147939097124612984047201335"
						.to_string(),
				],
				vec![
					"8122139689435793554974799663854817979475528090524378333920791336987132768041"
						.to_string(),
					"5111528818556913201486596055325815760919897402988418362773344272232635103877"
						.to_string(),
				],
			],
			c: vec![
				"6410073677012431469384941862462268198904303371106734783574715889381934207004"
					.to_string(),
				"11977981471972649035068934866969447415783144961145315609294880087827694234248"
					.to_string(),
			],
		},
	};

	let res = zk_light_client_step(&lcs, sync_committee_poseidon);

	assert_ok!(res.clone());
	assert_eq!(true, res.unwrap());
}
