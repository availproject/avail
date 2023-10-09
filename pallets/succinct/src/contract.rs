use ark_std::string::ToString;
use ark_std::vec;
use codec::{Decode, Encode};
use frame_support::dispatch::TypeInfo;
use frame_support::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sp_core::U256;

use frame_system::Config;

use crate::state::{CircomProof, LightClientRotate, LightClientStep, PublicSignals};
use crate::verifier::Verifier;
use crate::Error;

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

pub fn zk_light_client_rotate(
	update: &LightClientRotate,
	verifier: Verifier,
) -> Result<bool, ContractError> {
	let mut inputs = vec!["0".to_string(); 65];

	let m: &mut [u8; 32] = &mut [0u8; 32];
	update.sync_committee_ssz.to_big_endian(m);

	for i in 0..32 {
		inputs[i] = m[i].to_string();
	}

	let header_root = update.step.finalized_header_root.as_bytes();
	let finalized_header_root_numeric = U256::from_big_endian(header_root);

	let m1: &mut [u8; 32] = &mut [0u8; 32];
	finalized_header_root_numeric.to_big_endian(m1);

	for i in 0..32 {
		inputs[32 + i] = m1[i].to_string();
	}

	inputs[64] = U256::from_little_endian(&update.sync_committee_poseidon.encode()).to_string();

	let groth_16_proof = update.proof.clone();

	let circom_proof = CircomProof::new(groth_16_proof.a, groth_16_proof.b, groth_16_proof.c);

	let proof = circom_proof.to_proof();

	let public_signals = PublicSignals::from(inputs);

	let res = verifier.verify_proof(proof.clone(), &public_signals.get());

	res
}

pub fn zk_light_client_step(
	update: &LightClientStep,
	sync_committee_poseidon: U256,
	verifier: Verifier,
) -> Result<bool, ContractError> {
	let mut fs: [u8; 32] = [0u8; 32];
	let mut pc: [u8; 32] = [0u8; 32];

	let finalized_slot_le = update.finalized_slot.to_le_bytes();
	let participation_le = update.participation.to_le_bytes();
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

	let circom_proof = CircomProof::new(groth_16_proof.a, groth_16_proof.b, groth_16_proof.c);
	let proof = circom_proof.to_proof();
	let public_signals = PublicSignals::from(inputs);

	verifier.verify_proof(proof, &public_signals.get())
}

#[cfg(test)]
mod tests {
	use ark_std::string::ToString;
	use ark_std::vec;
	use frame_support::assert_ok;
	use hex_literal::hex;
	use sp_core::{H256, U256};

	use crate::contract::{zk_light_client_rotate, zk_light_client_step};
	use crate::state::{Groth16Proof, LightClientRotate, LightClientStep, State};
	use crate::verifier::Verifier;

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

	#[test]
	fn test_zk_rotate() {
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
			"b6c60352d13b5a1028a99f11ec314004da83c9dbc58b7eba72ae71b3f3373c30"
		));
		let execution_state_root = H256(hex!(
			"ef6dc7ca7a8a7d3ab379fa196b1571398b0eb9744e2f827292c638562090f0cb"
		));
		let sync_committee_poseidon = U256::from_dec_str(
			"13340003662261458565835017692041308090002736850267009725732232370707087749826",
		)
		.unwrap();

		println!(
			"sszn {:?}",
			"c1c5193ee38508e60af26d51b83e2c6ba6934fd00d2bb8cb36e95d5402fbfc94".as_bytes()
		);

		let h = hex!("c1c5193ee38508e60af26d51b83e2c6ba6934fd00d2bb8cb36e95d5402fbfc94");

		let sync_committee_ssz = U256::from_big_endian(h.as_slice());
		println!("sszn {:?}", sync_committee_ssz);

		let proof = Groth16Proof {
			a: vec![
				"2389393404492058253160068022258603729350770245558596428430133000235269498543"
					.to_string(),
				"10369223312690872346127509312343439494640770569110984786213351208635909948543"
					.to_string(),
			],
			b: vec![
				vec![
					"11815959921059098071620606293769973610509565967606374482200288258603855668773"
						.to_string(),
					"10181085549071219170085204492459257955822340639736743687662735377741773005552"
						.to_string(),
				],
				vec![
					"4596699114942981172597823241348081341260261170814329779716288274614793962155"
						.to_string(),
					"14404189974461708010365785617881368513005872936409632496299813856721680720909"
						.to_string(),
				],
			],
			c: vec![
				"9035222358509333553848504918662877956429157268124015769960938782858405579405"
					.to_string(),
				"10878155942650055578211805190943912843265267774943864267206635407924778282720"
					.to_string(),
			],
		};

		let ssz_proof = Groth16Proof {
			a: vec![
				"19432175986645681540999611667567820365521443728844489852797484819167568900221"
					.to_string(),
				"17819747348018194504213652705429154717568216715442697677977860358267208774881"
					.to_string(),
			],
			b: vec![
				vec![
					"19517979001366784491262985007208187156868482446794264383959847800886523509877"
						.to_string(),
					"18685503971201701637279255177672737459369364286579884138384195256096640826544"
						.to_string(),
				],
				vec![
					"16475201747689810182851523453109345313415173394858409181213088485065940128783"
						.to_string(),
					"12866135194889417072846904485239086915117156987867139218395654387586559304324"
						.to_string(),
				],
			],
			c: vec![
				"5276319441217508855890249255054235161211918914051110197093775833187899960891"
					.to_string(),
				"14386728697935258641600181574898746001129655942955900029040036823246860905307"
					.to_string(),
			],
		};

		let lcs = LightClientRotate {
			step: LightClientStep {
				attested_slot: 0,
				finalized_slot: 4360032,
				participation: 413,
				finalized_header_root,
				execution_state_root,
				proof,
			},

			sync_committee_ssz,
			sync_committee_poseidon,
			proof: ssz_proof,
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

		// let verifier = Verifier::from_json_u8_slice(vk.as_bytes()).unwrap();
		let verifier = Verifier::new_rotate_verifier();

		let res = zk_light_client_rotate(&lcs, verifier);

		assert_ok!(res.clone());
		assert_eq!(true, res.unwrap());
	}
}
