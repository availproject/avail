use frame_support::dispatch::RawOrigin;
use frame_support::{assert_err, assert_ok, BoundedVec};
use frame_system::submitted_data::{Message, MessageType};
use hex_literal::hex;
use sp_core::crypto::AccountId32;
use sp_core::ByteArray;
use sp_runtime::testing::H256;
use sp_runtime::DispatchError::BadOrigin;

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
fn test_set_updater() {
	new_test_ext().execute_with(|| {
		// Goal: Set updater - bad origin.
		let new_updater = H256(hex!(
			"d54593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
		));
		let old_updater = H256(hex!(
			"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
		));

		let before = StateStorage::<Test>::get();
		assert_eq!(before.updater, old_updater);
		let bad_origin = RuntimeOrigin::from(RawOrigin::None);
		let bad_result = Bridge::set_updater(bad_origin, new_updater);
		assert_err!(bad_result, BadOrigin);
		assert_eq!(before.updater, old_updater);

		// Goal: Set updater - success.
		let root_origin = RuntimeOrigin::from(RawOrigin::Root);
		let success = Bridge::set_updater(root_origin, new_updater);
		assert_ok!(success);
		let after = StateStorage::<Test>::get();
		assert_eq!(after.updater, new_updater);
	});
}

#[test]
fn test_execute_message_via_storage() {
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

		// todo Message must be valid
		let message_bytes = get_valid_message();

		let account_proof = get_valid_account_proof();
		let storage_proof = get_valid_storage_proof();

		let success = Bridge::execute(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			slot,
			message_bytes,
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
			updater: H256::from_slice(TEST_SENDER_ACCOUNT.as_slice()),
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
			updater: H256::from_slice(TEST_SENDER_ACCOUNT.as_slice()),
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
			updater: H256::from_slice(TEST_SENDER_ACCOUNT.as_slice()),
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
			updater: H256::from_slice(TEST_SENDER_ACCOUNT.as_slice()),
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
			updater: H256::from_slice(TEST_SENDER_ACCOUNT.as_slice()),
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
        BoundedVec::truncate_from(hex!("f90211a050da92c339db0b71cd6a8ac7893a6b8689ec5a3a46a0231b3ee2bd1baee75e1da045a3d973eb74a02b762d8b1ba683f39bca3965806276c8ceffe2d2ebc6cce233a0e88ad29ca98fa08f59f2a7f0110d63505d99a173628643290df869c4d1fa312ba00bb4cc9dc0b1de6ae0d80424b1fa992efb400a07a0e84615c91762fe734b2d0ca0a07e495d39bf2b779405790c6c7e7eb1cc3c803a88db36d1ec600fb0e555b5bba09a1c776e89c8be75d0a9ea022c05fd2ff095869d549e74a8fff7f2fb2deaf738a073b874e49e77dfd9312d7b1afd1ac10e02021a1ba2ab7c97ecaeaa0e26a34027a07e3424405c13aa33a2eb9ec6d8640aa1f67fdd8c8e9e4276334515b1cf1df65ca0246b93b2e3cc625a5e75b40165c6cb95ae8ffb9406563d34092d6359c7616aeea04d2fd8fdb1ab7d8f8fc6079400396fec828914230fade3794f13dc5ae7f6bbb8a04811b9efbfa8d495c5be91be78372b4a29140bd1e092e793db50ed9c495a6d54a02e1b3a417e8341dc8e1ade6ca527778192d33c7c827cfa63a366d007f2884e24a0845f4f33a4993d85766a14222cde1d124bd0f15523d239572883258a7bbcccd9a0ed2021cc2206fcfd9f80d592890b1b4eb615fae4f11d4e4a66d54a6767908901a07d46bf6e9dc9599eb7ca036aa976ef9cc63f02e9097252799f5d3a8792c49620a00b58d1d2cc72401c7cb978d34e15f74038ac63355e415d53b894179b8938dbb780").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a03e22056b0eefc94898d516b45ea579bd298291aa521c8665f3d5215da5619513a0b5f3f839320c3d63436a8f27a07bc47a5e7ace4f5d80e437ce2084a007e0fd37a0e666b7198d6b6023de9a698f4bc90a9595e57f7f4e70d07d0366c693d53994c5a05b14820b719fbb37f5e9ff55770d6ceb63ef90af46934377c0364ca72335852ea09c4a1a1d5b1e58e9be1c9b4ea4e943c514b4ae8a382be6dd16e53336354e0500a0058c24b25f97ed51ca2c44e016631753eb97197733b23aea23aef112a2323321a03347d79447b18678fbbedd01b48e52747a5301d32223c4be91f5681d2a69d7b2a04182f6e242615804a49f3a54399e285d84a6e7692cca41008d2b638be30fe00fa0c64a1e71e7512d73008d4cce2a2ba0981023c4ff5f821ba97fcf8059f4699bb5a0673bee8a446cac15221e9292a904ed44762ccb19dac57bbef085d76c6c5b9bb0a065d1ccec63163a4e5ea501f3951a384daaa9aaf4c9c976f963e3597b3e8ce4eca0fb4a788676b5a593e7db6c1149e3c89c774ef9915010846bcb53563736ccde70a0d5274ce6a4e744adab98139ed9d6b5846a449721f32d0f49e020061f5abb094ba0bbf7fd5e93a74f6d8ec4df6f2b0c7f6ff2b387a1a2cb2fd1f26545208c099443a0ddac5ec494b529e87a014e9f80869493008eba559e8ed9e9691fcf219bea14d0a06092b5dc5dd24f768b0c0bf74a6deb0e4e9a5fa3c474d06d52a63ace81d272c980").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a0d7b01a1c5e66b3cbb35832888bdb5c1312968a2849b942aad3433c6c21990faca067d17e56fc092306254b21e6101503c64326bbba467c714cadee8c9978aa2b57a0344929c8674281f336f38f5116469a7440bc41695916bd3faaf871716973a257a0e829cbcc6b207df95879af17d6df49a1327a63be6a2b2e8a2c1f8a8485a996e6a03a7f6e4ebd66e0377e7881a2de4361a34ac09116b0ccfe7bf2a96ab5100c4a21a0707b3b93b7aeae349737613b49037c406d411017fcf99c0877225338437fa549a0dce10f297e8bd76ff379c9ef548d51f491db677b566ceb5f83a139bd0b60ae4ea0c4f1e68723d248195d4439942c35f373ddd2889cd97a224ff1a3d379229b79aea0d1716748894608fdb98067c7daaad0e703cb42bd8bc57f39785b155f6914c2aca0c39df4d8b0242b1eaf733f3cd6237211c26b595a18d5e831c062a070ea3a4807a0b2e51fcaee45d252a96baf975e0e506dce7c7e3ddc39e30f7bb9de8955f602dba06ef05cdd0a80b246a4d91bc0dde4df927959474d567fdc9b11a586eede643191a07754dd15ffae315ed9f309f2e2722140fc1989c783fdda3f454fe8d5e7bf0e3ba06ca8810923e01ec88b0a14535fe248d6680df5de9becc5962b97a3c755bb2f84a07cdfc9857d06ba074ad5ca1769ac041c7c99e25a41331f625f16c6ce86bb1ba8a09d779a55977e48cd90d6c6b73b9b86301ad54bce224c4e1abcd7667dfa44347b80").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a0a9088ce9294db8a3f65adf5a3ceb5d1cd34c7804f8fe9a69eaf66bb860c5df91a0d6ad86f7ef958121aab83506dd9d5742f5980477e4db503c8a0eee7359d69857a00e72d2f638a2b873689a06afd5c080893e05ee6f8922b495d41b43727879cf3ea0e6f398effbe276d71947a920fc816602b255df3fb73bd59acfd3c036ae0f7996a0a84e9d20d33bb5d5db857f3ace6e32b54c93f7d148ddece8777d01aca293a9c4a0e3e7126f8ebb286919b3cfb2189a22f63fa475fc0fa7b36e79526f28993d089ea0af9c84fa15d80d5cd8462cc342072257f8eac2161a113da401536dc4b4de5ceba0df7cf975aa213b6ca4e655a99f4d074b24b5412e3056d1b39188550b49dea0f7a0c76abf47096e3b2660f935061b4e136378126aefecbcb348c895a4c678192536a0cf16074b69b96652074546708053506e6b2d5b7d6f3564f2091ddb690b701409a042330d1d46d74569cb62f900c06bb2659503d73b93be83371390904004897f11a00eefdc7fd5890c2051dd6f6326e036268613b8209b46c5f31dd4ca57e270a0eda05da0e6248c96f367e2b139c2329ca8ea2d4a9ba4c6438e2d33a9ed37f3d63104a0d43d6814ed1f765fd5d204ce91a92996adef6e65c563af59271b59bd933719eea04e6c678da69cd38894f2574d9b30d8871cd7eddd62b718e0941679a85a85b17da0300aa0769fa573f8c40bc841597e33d763ff32bc044f98aa6559e2df09b3174980").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a0bc80d8ae6ccac93ee4b2c020cdc98e960c2f840719e4eed518a28462f5c2e042a01481627b435734196f94592d10ca71c7dcb36eab7c2b39df29aa2c10ea944bc7a0627368fcc64ca6ddc311ddf7310625ef5d02a7a660a739047c4dde24d7f375aaa0ab6529dbec1ad45c32851fe70e17ca08332d8316453884c68f74e7a889ba46c0a0d73c5946469b9925e7681f45580a8d957f98a05f80a1a9bd7fe229ab79fbb7eca0406818fe909531d28e0461f93b428f6b530aa411529a53de213016e0d47693c4a0a2c63d00409e11dce2433521617080599719f65e727fb1b966d288fc5515515ba016e46c67a3b4aba26ed57a38bb0ac50d40301bd4e4482e3eec0667f2d70d4f9aa09262644352b4c7e435f2c77566b0f03b09b3109b0ba4fdb3c18f9f5b3ff83a68a006dc0a9848791e8068f25b0fca1a8f2a17c6421415f73355ee585f69e48dd9c0a0c158363b7c36d9abf2c07fac52c43ad8cbb3708af4c8375c64408da4b1c6112ea020290f03df9348a45be69b11f43ef60239aba95f31bc348439b4827c5a94ea1aa0950c0b5eb46cb26804706efb963b3e8cf3bbf0b0ce78fbfe4232f88e1cc4980fa0dfd3aa0540319f45916236f460f76831bfc526e8c0279fe798c3674ad08998eea0d68b134cb5a9433729bb46521b46e9bf737fabe2c1568185dc0d62cb2df23633a072708353bc10a239c80991deefd9a08158902b0d4ddd81857541368358e71ab280").to_vec()),
        BoundedVec::truncate_from(hex!("f901518080a06b3861e939ffd924c512e631febac4b7573840910a230356701f5d9876d462f78080a0644b04a89b048be9044f7ddf0ddfcfdf16eb859770c59bea283be83efc0ab852a04783d2f6f95d2df8ecfe9cd176aabf0d5ce6e1a52009c0d7d8016a9c897cd996a05ebf2e95f0ce88623be1b9df655ddff6032bb68530ce80fc060914a26c983ed6a0b2cda30c80dadf34909d937dc977928bef8b702bcf64ac7cbfb14a1c55444898a0de3bef8b9dfce8c4a2d24b6ca802f5116e7e873ea2d0863f1cf72c23672f82c280a04e75b47f705d7811a0d326440a499b2dfeb0959cd151f91b71896111bfe8ae6580a05fccb9d0c6524886af03bb1f68990c9f54c098f57c664a5c51994052fd563aeca0cbab9ef5e83548e993c5cd9b688af2f34c6d9c5c632b59b687fa5a5e87b6bbf2a0fb82bb552d3eec458a68d01642f0e7df3d88d5b3040f69fa79b2e402adf412fa80").to_vec()),
        BoundedVec::truncate_from(hex!("f851808080808080808080808080a035d937961d73f8a0eea9ae41b2f4cbb73c1d2c0666ea35f1ae05c43b5896b1098080a0e05c86fffb9aada22f0429326d6eda556e23f655917975b4f859bc258d32f67f80").to_vec()),
        BoundedVec::truncate_from(hex!("f8669d399e1ef4313dc3558aee86cc911474c2262f1dbe387aea254422552a5fb846f8440180a0a03e10dfba89f79567f7c9a238ee7fe66ed32e711be4db6e73d7211601dec360a0356c7854fe7a483ece02a531c58b63aa2bdbab40df89c9f919f0d524b54dd494").to_vec()),
    ])
}

fn get_valid_storage_proof() -> BoundedVec<BoundedVec<u8, StorageProofMaxLen>, StorageProofLen> {
	BoundedVec::truncate_from(vec![BoundedVec::truncate_from(hex!("f90211a0f0a16ee9b11528f3da8796229dad134b9085ed9428d868e6988f9b2473b59d6fa0f8175015d0a3df8fc451d2bd3d64a34e0836f3203129ac567e869f1157b488dfa0f9d56e943c6962cf8e2ca51b94b54307eb45424ebb84ed079b417cf03a85e298a0408af9f1c5f64ed6c517b1dbf661b75a705ef7d78bcae67b9a54c1e8052b56b2a02157d476a9a077cfc9eb00ead5ab65dcbfe363a71e993c3602a66c0fccf13e4aa00772697ebf25f2e83830918bd52bbb9600c077ae289e740ae76c7bdfd34b7ebea0a1dd0da76aacf7c82629c55e4b956b2e9ef77d7fdcee1adeb23d022f0950d554a0695cb723c857d98ad1c96a372f7983bf771556f4608674266a0698531543217ba05c0fb347305720b81c7d39be6fd5b2083af607654098a0f1418ec111a846510aa0ecd30808bffcb164a258c332a29f3050e9e85d28e988305b7f643dcad4f32c8fa0ec5ee93a7ede0a9c641dcd7515c1408ab48f86b5295cd26b3d738e8d8ac7829fa01434a5f6054456bbce0a59ba1c182eeee8e64fd6762ff365e550ca7cd8cedad0a0b4fefcb325f044a6663c9441ec9f025718d0f2d7fc1c29ec819f4a366cafbb6fa0cc26bfb18151569b0f765335474fa3840f9093385816bd14a4a3c553fae62949a06a28c02f7b649bad24b39d9a4e9fc4c8e93b1ae2b043af4f5bbcb8238e193eaba011ef889094bf6ca740810423041169453b7daea3df98b3018523f86e96bf033580").to_vec()),
                                   BoundedVec::truncate_from(hex!("f8d180808080a0053a80e0ec0645b0acdddd1650b28104de2a51e7144bc5c7f7f69d44c544587a80a0bb2d4c2215259ba0a7fba5e750be34f510fb4494a19b4fbabc8b419f6a35346e808080a01a9817fbc2f3624eb22a44d5b6643c370eac51c77ff3a8d59f42b1d9fe5ea925a09c851efdcfd1d623fd4a3e5ef7f041b1f59b6ae7d60740291cc2e25bccc0a9b38080a0ddf637c0efd4778239f93a609faa694809faf5420e462488de85b0a2ba5bcf66a0fc31bff1855e70288e2c52383e1841cebc68bbcc08da7507c6112f2d2007231680").to_vec()),
                                   BoundedVec::truncate_from(hex!("f843a0204effc936259a57c56ffc97bf601a6f6ee129ac5cd39809a889df1a8ad3fdc1a1a03617643cdff88aaf66c6d09fd11c1a73ce69dd905086afd692a62c4ba800fdd4").to_vec()),
    ])
}
