use crate::messages::{CircomProof, LightClientStep, PublicSignals};
use crate::verifier::Verifier;
use crate::Error;

use ark_std::string::ToString;
use ark_std::vec;
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
	verifier: Verifier,
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

	let inputs_string = U256::from_little_endian(t.as_slice()).to_string();

	let inputs = vec![inputs_string; 1];

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

#[cfg(test)]
mod tests {
	use crate::messages::{Groth16Proof, LightClientStep, State};
	use crate::verifier::Verifier;

	use crate::contract::zk_light_client_step;
	use ark_std::string::ToString;
	use ark_std::vec;
	use frame_support::assert_ok;
	use hex_literal::hex;
	use sp_core::{H256, U256};

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
		let vk = r#"{"vk_json":{
 "protocol": "groth16",
 "curve": "bn128",
 "nPublic": 1,
 "vk_alpha_1": [
  "20491192805390485299153009773594534940189261866228447918068658471970481763042",
  "9383485363053290200918347156157836566562967994039712273449902621266178545958",
  "1"
 ],
 "vk_beta_2": [
  [
   "6375614351688725206403948262868962793625744043794305715222011528459656738731",
   "4252822878758300859123897981450591353533073413197771768651442665752259397132"
  ],
  [
   "10505242626370262277552901082094356697409835680220590971873171140371331206856",
   "21847035105528745403288232691147584728191162732299865338377159692350059136679"
  ],
  [
   "1",
   "0"
  ]
 ],
 "vk_gamma_2": [
  [
   "10857046999023057135944570762232829481370756359578518086990519993285655852781",
   "11559732032986387107991004021392285783925812861821192530917403151452391805634"
  ],
  [
   "8495653923123431417604973247489272438418190587263600148770280649306958101930",
   "4082367875863433681332203403145435568316851327593401208105741076214120093531"
  ],
  [
   "1",
   "0"
  ]
 ],
 "vk_delta_2": [
  [
   "13909124302531010921185816266702828674819977847946098152869315744616458486564",
   "20132301864891590102651537900097603129841488311097169471951837821863335966377"
  ],
  [
   "9968363667543645393414941586581030294599633785037951467496223618072496422152",
   "19620890790369364323423864638476333921325558259845161848280036523505618212219"
  ],
  [
   "1",
   "0"
  ]
 ],
 "vk_alphabeta_12": [
  [
   [
    "2029413683389138792403550203267699914886160938906632433982220835551125967885",
    "21072700047562757817161031222997517981543347628379360635925549008442030252106"
   ],
   [
    "5940354580057074848093997050200682056184807770593307860589430076672439820312",
    "12156638873931618554171829126792193045421052652279363021382169897324752428276"
   ],
   [
    "7898200236362823042373859371574133993780991612861777490112507062703164551277",
    "7074218545237549455313236346927434013100842096812539264420499035217050630853"
   ]
  ],
  [
   [
    "7077479683546002997211712695946002074877511277312570035766170199895071832130",
    "10093483419865920389913245021038182291233451549023025229112148274109565435465"
   ],
   [
    "4595479056700221319381530156280926371456704509942304414423590385166031118820",
    "19831328484489333784475432780421641293929726139240675179672856274388269393268"
   ],
   [
    "11934129596455521040620786944827826205713621633706285934057045369193958244500",
    "8037395052364110730298837004334506829870972346962140206007064471173334027475"
   ]
  ]
 ],
 "IC": [
  [
   "14768330346746297840816367070658728893313212053739352195802618469166531204391",
   "226007277514949219964518589190903213280753732819328898150443666757283640566",
   "1"
  ],
  [
   "11579789275084599412171695990815953848893751967864880119324773293908098730772",
   "7016524000863123597202679959446996204295974709290664682467334394757983209848",
   "1"
  ]
 ]
}}"#;

		let verifier = Verifier::from_json_u8_slice(vk.as_bytes()).unwrap();

		let res = zk_light_client_step(&lcs, sync_committee_poseidon, verifier);

		assert_ok!(res.clone());
		assert_eq!(true, res.unwrap());
	}
}
