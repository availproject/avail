use ethabi::{encode, Token};
use frame_support::{assert_err, assert_ok, BoundedVec};
use frame_system::submitted_data::{Message, MessageType};
use hex_literal::hex;
use primitive_types::U256;
use sp_core::crypto::AccountId32;
use sp_runtime::testing::H256;

use crate::mock::System;
use crate::mock::{new_test_ext, Bridge, RuntimeEvent, RuntimeOrigin, Test};
use crate::state::State;
use crate::{
	AccountProofLen, AccountProofMaxLen, Broadcasters, Error, Event, ExecutionStateRoots,
	InputMaxLen, OutputMaxLen, ProofMaxLen, SourceChainFrozen, StateStorage,
	StepVerificationKeyStorage, StorageProofLen, StorageProofMaxLen, Timestamps,
	VerificationKeyDef,
};

const TEST_SENDER_VEC: [u8; 32] = [2u8; 32];
const TEST_SENDER_ACCOUNT: AccountId32 = AccountId32::new(TEST_SENDER_VEC);

fn get_valid_input() -> BoundedVec<u8, InputMaxLen> {
	BoundedVec::truncate_from(
		hex!("0ab2afdc05c8b6ae1f2ab20874fb4159e25d5c1d4faa41aee232d6ab331332df0000000000747ffe")
			.to_vec(),
	)
}

fn get_valid_output() -> BoundedVec<u8, OutputMaxLen> {
	BoundedVec::truncate_from(hex!("e4566e0cf4edb171a3eedd59f9943bbcd0b1f6b648f1a6e26d5264b668ab41ec51e76629b32b943497207e7b7ccff8fbc12e9e6d758cc7eed972422c4cad02b90000000000747fa001fd").to_vec())
}

fn get_valid_proof() -> BoundedVec<u8, ProofMaxLen> {
	BoundedVec::truncate_from(hex!("0b496d04c0e12206bc846edd2077a20b8b55f65fc0e40bb8cf617d9b79ce39e508281ad49432300b3b7c8a95a0a63544f93f553fcfdeba38c82460888f4030ed1f67a1be666c12ee00658109c802042c58f645474fcee7d128277a4e35c1dd1504d33cb652ec23407cd3580eda0196dd97054eb5c2a817163d6997832d9abd422729b3e85a15941722baeb5ca8a42567a91c6a0b0cd64ac15431fde05071e90e0d30c12013d5803336cc2f433c16eaa5434e30b89ce7395c3c3cda29dde3be062281095f143d728486c71203b24fa6068e69aabf29d457ffadc6d682d51a4f08179d3240bc561ae7e2c005bb772a4d4c5ba6644986052fad554f042ab0074a8f").to_vec())
}

fn get_invalid_proof() -> BoundedVec<u8, ProofMaxLen> {
	BoundedVec::truncate_from(hex!("1b496d04c0e12206bc846edd2077a20b8b55f65fc0e40bb8cf617d9b79ce39e508281ad49432300b3b7c8a95a0a63544f93f553fcfdeba38c82460888f4030ed1f67a1be666c12ee00658109c802042c58f645474fcee7d128277a4e35c1dd1504d33cb652ec23407cd3580eda0196dd97054eb5c2a817163d6997832d9abd422729b3e85a15941722baeb5ca8a42567a91c6a0b0cd64ac15431fde05071e90e0d30c12013d5803336cc2f433c16eaa5434e30b89ce7395c3c3cda29dde3be062281095f143d728486c71203b24fa6068e69aabf29d457ffadc6d682d51a4f08179d3240bc561ae7e2c005bb772a4d4c5ba6644986052fad554f042ab0074a8f").to_vec())
}

const STEP_FN_ID: H256 = H256(hex!(
	"af44af6890508b3b7f6910d4a4570a0d524769a23ce340b2c7400e140ad168ab"
));

#[test]
fn test_execute_message_via_storage() {
	new_test_ext().execute_with(|| {
		let data = &[
			Token::FixedBytes(H256::zero().as_bytes().to_vec()),
			Token::Uint(U256::from(1000000000000000000u128)),
		];
		let encoded_data = encode(data);

		let message = Message {
			message_type: MessageType::FungibleToken,
			from: H256(hex!(
				"f39Fd6e51aad88F6F4ce6aB8827279cffFb92266000000000000000000000000"
			)),
			to: H256(hex!(
				"0000000000000000000000000000000000000000000000000000000000000001"
			)),
			original_domain: 2,
			destination_domain: 1,
			data: BoundedVec::truncate_from(encoded_data),
			id: 0,
		};

		let slot = 8581263;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"cce0149821d9be3c3e5d2ebd159771746ad4762f8b7fb18454a7a498eb4a82c7"
			)),
		);
		Broadcasters::<Test>::set(
			2,
			H256(hex!(
				"cf7ed3acca5a467e9e704c703e8d87f634fb0fc9000000000000000000000000"
			)),
		);

		Timestamps::<Test>::set(slot, 1701327753);

		// todo Message must be valid
		// let message_bytes = BoundedVec::truncate_from(message.abi_encode().to_vec());

		let account_proof = get_valid_account_proof();
		let storage_proof = get_valid_storage_proof();

		let success = Bridge::execute(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			slot,
			message,
			account_proof,
			storage_proof,
		);

		assert_ok!(success);
	});
}

#[test]
fn test_execute_message_with_frozen_chain() {
	new_test_ext().execute_with(|| {
		let slot = 8581263;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"cd187a0c3dddad24f1bb44211849cc55b6d2ff2713be85f727e9ab8c491c621c"
			)),
		);
		Broadcasters::<Test>::set(
			2,
			H256(hex!(
				"43f0222552e8114ad8f224dea89976d3bf41659d000000000000000000000000"
			)),
		);

		Timestamps::<Test>::set(slot, 1701327753);

		let message = get_valid_message();

		//BoundedVec::truncate_from(hex!("01000000000000005400000005e2b19845fe2b7bb353f377d12dd51af012fbba2000000064000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000064").to_vec());
		let account_proof = get_valid_account_proof();
		let storage_proof = get_valid_storage_proof();

		// Goal: Prevent from executing message
		SourceChainFrozen::<Test>::set(2, true);
		let error = Bridge::execute(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			slot,
			message,
			account_proof,
			storage_proof,
		);

		assert_err!(error, Error::<Test>::SourceChainFrozen);
	});
}

// #[test]
// fn test_execute_message_with_faulty_account_proof() {
// 	fail!("todo implement")
// }
//
// #[test]
// fn test_execute_message_with_faulty_storage_proof() {
// 	fail!("todo implement")
// }
//
// #[test]
// fn test_execute_message_with_already_executed_message() {
// 	fail!("todo implement")
// }
//
// #[test]
// fn test_execute_message_with_already_() {
// 	fail!("todo implement")
// }

#[test]
fn test_full_fill_step_call() {
	new_test_ext().execute_with(|| {
		let slot = 7634942;

		StepVerificationKeyStorage::<Test>::set(get_step_verification_key());

		StateStorage::<Test>::set(State {
			slots_per_period: 8192,
			finality_threshold: 461,
		});

		let result = Bridge::fulfill_call(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			STEP_FN_ID,
			get_valid_input(),
			get_valid_output(),
			get_valid_proof(),
			slot,
		);

		assert_ok!(result);

		// ensure that event is fired
		let expected_event = RuntimeEvent::Bridge(Event::HeaderUpdate {
			slot,
			finalization_root: H256(hex!(
				"e4566e0cf4edb171a3eedd59f9943bbcd0b1f6b648f1a6e26d5264b668ab41ec"
			)),
		});

		assert_eq!(expected_event, System::events()[0].event);
	});
}

#[test]
fn test_full_fill_step_call_no_verification_key_set() {
	new_test_ext().execute_with(|| {
		let slot = 7634942;

		StateStorage::<Test>::set(State {
			slots_per_period: 8192,
			finality_threshold: 461,
		});

		let result = Bridge::fulfill_call(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			STEP_FN_ID,
			get_valid_input(),
			get_valid_output(),
			get_valid_proof(),
			slot,
		);

		assert_err!(result, Error::<Test>::VerificationKeyIsNotSet);
	});
}

#[test]
fn test_full_fill_step_call_proof_not_valid() {
	new_test_ext().execute_with(|| {
		let slot = 7634942;
		StepVerificationKeyStorage::<Test>::set(get_step_verification_key());

		StateStorage::<Test>::set(State {
			slots_per_period: 8192,
			finality_threshold: 461,
		});

		let result = Bridge::fulfill_call(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			STEP_FN_ID,
			get_valid_input(),
			get_valid_output(),
			get_invalid_proof(),
			slot,
		);

		assert_err!(result, Error::<Test>::VerificationFailed);
	});
}

#[test]
fn test_full_fill_step_call_not_valid_function_id() {
	new_test_ext().execute_with(|| {
		let slot = 7634942;
		StepVerificationKeyStorage::<Test>::set(get_step_verification_key());

		StateStorage::<Test>::set(State {
			slots_per_period: 8192,
			finality_threshold: 461,
		});
		let invalid_function_id: H256 = H256(hex!(
			"bf44af6890508b3b7f6910d4a4570a0d524769a23ce340b2c7400e140ad168ab"
		));
		let result = Bridge::fulfill_call(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			invalid_function_id,
			get_valid_input(),
			get_valid_output(),
			get_valid_proof(),
			slot,
		);

		assert_err!(result, Error::<Test>::FunctionIdNotKnown);
	});
}

#[test]
fn test_full_fill_step_call_finality_not_met() {
	new_test_ext().execute_with(|| {
		let slot = 7634942;
		StepVerificationKeyStorage::<Test>::set(get_step_verification_key());

		StateStorage::<Test>::set(State {
			slots_per_period: 8192,
			finality_threshold: 512, // max finality
		});
		let result = Bridge::fulfill_call(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			STEP_FN_ID,
			get_valid_input(),
			get_valid_output(),
			get_valid_proof(),
			slot,
		);

		assert_err!(result, Error::<Test>::NotEnoughParticipants);
	});
}

fn get_step_verification_key() -> VerificationKeyDef<Test> {
	let step_vk = r#"{"vk_json":{
    "protocol": "groth16",
    "curve": "bn128",
    "nPublic": 2,
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
            "677302577815076814357170457144294271294364985082280272249076505900964830740",
            "5628948730667472013190771331033856457010306836153142947462627646651446565415"
        ],
        [
            "5877290568297658003612857476419103064356778304319760331670835003648166891449",
            "10874997846396459971354014654692242947705540424071616448481145872912634110727"
        ],
        [
            "1",
            "0"
        ]
    ],
    "vk_alphabeta_12": [],
    "IC": [
        [
            "202333273032481017331373350816007583026713320195536354260471885571526195724",
            "8246242704115088390751476790768744984402990892657920674334938931948100192840",
            "1"
        ],
        [
            "12901454334783146822957332552289769626984444933652541503990843020723194328882",
            "12436078488518552293095332739673622487901350475115357313978341690183990059269",
            "1"
        ],
        [
            "12828056956769114977702246128118682473179646035440405756936949778100648490262",
            "7351319165217643779735289066901404053730163225836026220896225559268517203790",
            "1"
        ]
    ]
}}"#;

	BoundedVec::truncate_from(step_vk.as_bytes().to_vec())
}

fn get_valid_message() -> Message {
	Message {
		message_type: MessageType::FungibleToken,
		from: Default::default(),
		to: Default::default(),
		original_domain: 2,
		destination_domain: 1,
		data: Default::default(),
		id: 0,
	}
}

fn get_valid_account_proof() -> BoundedVec<BoundedVec<u8, AccountProofMaxLen>, AccountProofLen> {
	BoundedVec::truncate_from(vec![
		BoundedVec::truncate_from(hex!("b9016efef3cb80e38446d0bb2a793fbd47807ac2abbf937e1e797ab1256c15ad092a54be3a49ce807b511c8799e32a00808434087c343511dcd07849cbb5f45a4f275baab66133be8012461217af10f347e4e22f0b89f48fb3a2a75db858671985370bbf6f643746038080c42d34644a10ef9d9e16eb11a2348d118958469085f2e31decf794bf654bbe80b91924b665944c17f4b8ff708421148214fcb4c215735db09be677ef4083087880f9aa61ab0c6925e3e5c41da7baeb801127473d104a31b17062bc0b6ecab5b33c8072f49529c0f99a3d46e0a0dce3f3527c53a12c96bfcf30db993d18231e12069680792abc6dc5c7876958852dcdcecf5877b5799d474135a9ef08a2cf2a188592eb80fc1cf0b15da7e492537f17aa327f0a4092b1ed815fc4625ff86c8b1b3f5cd02780a8c5f900a733c1ceeca216b9a0679860744a1648e743af116143d6a9bedc8855802cdf23f2352e238fa8c851a46f520d80daedfac696e2ee8d3aca3c9c90b07a56").to_vec()),
		BoundedVec::truncate_from(hex!("b845fe1040808cea81a219a305a25726030d68f3218cc86888a0c234ce73303a23d1704b2d3c80eef84470c01af7499084140827efb062315f7cbfa8721abffb9b91768d1775e2").to_vec()),
		BoundedVec::truncate_from(hex!("b8683f007bdba68bf70aaeb167af64e1fe093e2da7911c8fdd2ca2cbd155105564661901f8440280a05a71fceba2936bd729446581d6df9f9f3a417fc1373a3155786795e76256f33ea0cce0149821d9be3c3e5d2ebd159771746ad4762f8b7fb18454a7a498eb4a82c7").to_vec()),
    ])
}

fn get_valid_storage_proof() -> BoundedVec<BoundedVec<u8, StorageProofMaxLen>, StorageProofLen> {
	BoundedVec::truncate_from(vec![BoundedVec::truncate_from(hex!("b8c9feb44880d1a0755b4d9b7a2e3e09cc526d1926ae1a4fae18ff595001803b8a1f47624e5a8063cd91cdefd0ca9d5556df2eeb1c90f3b6af7cea073bc547ae584b2bf205584880e070fe2a57e1affbfa6eff5b88511e31611becbd15519297e2f548706ba00de9802508d1271c5447fd181ed328ee3ca314d5f612f04a7cc5e70c44eaf2bed8f47380d77591f02663f103ac4beb66729d3d31b47e6c1d5eec8eb481c720c2aed95a04801d13218cebc912109e31838a10450317e8207d792a7936d920b3892293f541c3").to_vec()),
								   BoundedVec::truncate_from(hex!("a3810980bf651b6687fc056e290daf2bfdcf69ab146f45ae6279c9a1d95877f33d1e1c25").to_vec()),
								   BoundedVec::truncate_from(hex!("b845fe2100800726b343ad35afcb092946d73774da34b18485c7fc4ddcdc71a017d7aa25d26680dfc2893c391cec38ffdf1c70b1e2702e79cd1433ea341a1e7ffb4762e2a73678").to_vec()),
								   BoundedVec::truncate_from(hex!("b8423e0841a49a1089f4b560f91cfbb0133326654dcbb1041861fc5dde96c724a22f84a0e95f18aa25b460d35dff989a602606a7e2f6e05be655a3c94580f7aa6dc97ef7").to_vec()),
    ])
}
