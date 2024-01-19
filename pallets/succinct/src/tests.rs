use ethabi::{encode, Token};
use frame_support::traits::fungible::Inspect;
use frame_support::traits::DefensiveTruncateFrom;
use frame_support::{assert_err, assert_ok, BoundedVec};
use frame_system::submitted_data::{Message, MessageType};
use hex_literal::hex;
use primitive_types::U256;
use sp_core::crypto::AccountId32;
use sp_core::keccak_256;
use sp_runtime::testing::H256;
use sp_runtime::traits::BadOrigin;

use crate::mock::{new_test_ext, Bridge, RuntimeEvent, RuntimeOrigin, Test};
use crate::mock::{Balances, System};
use crate::state::Configuration;
use crate::target_amb::MessageStatusEnum;
use crate::{
	Broadcasters, ConfigurationStorage, Error, Event, ExecutionStateRoots, FunctionInput,
	FunctionOutput, FunctionProof, Head, Headers, MessageStatus, SourceChainFrozen,
	SyncCommitteePoseidons, Timestamps, ValidProof, WhitelistedDomains,
};
use frame_system::RawOrigin;

const TEST_SENDER_VEC: [u8; 32] = [2u8; 32];
const TEST_SENDER_ACCOUNT: AccountId32 = AccountId32::new(TEST_SENDER_VEC);
const STEP_FN_ID: H256 = H256(hex!(
	"af44af6890508b3b7f6910d4a4570a0d524769a23ce340b2c7400e140ad168ab"
));

const ROTATE_FN_ID: H256 = H256(hex!(
	"9c1096d800fc42454d2d76e6ae1d461b5a67c7b474efb9d47989e47ed39b1b7b"
));

fn get_valid_step_input() -> FunctionInput {
	BoundedVec::truncate_from(
		hex!("0ab2afdc05c8b6ae1f2ab20874fb4159e25d5c1d4faa41aee232d6ab331332df0000000000747ffe")
			.to_vec(),
	)
}

fn get_valid_step_output() -> FunctionOutput {
	BoundedVec::truncate_from(hex!("e4566e0cf4edb171a3eedd59f9943bbcd0b1f6b648f1a6e26d5264b668ab41ec51e76629b32b943497207e7b7ccff8fbc12e9e6d758cc7eed972422c4cad02b90000000000747fa001fd").to_vec())
}

fn get_valid_step_proof() -> FunctionProof {
	BoundedVec::truncate_from(hex!("0b496d04c0e12206bc846edd2077a20b8b55f65fc0e40bb8cf617d9b79ce39e508281ad49432300b3b7c8a95a0a63544f93f553fcfdeba38c82460888f4030ed1f67a1be666c12ee00658109c802042c58f645474fcee7d128277a4e35c1dd1504d33cb652ec23407cd3580eda0196dd97054eb5c2a817163d6997832d9abd422729b3e85a15941722baeb5ca8a42567a91c6a0b0cd64ac15431fde05071e90e0d30c12013d5803336cc2f433c16eaa5434e30b89ce7395c3c3cda29dde3be062281095f143d728486c71203b24fa6068e69aabf29d457ffadc6d682d51a4f08179d3240bc561ae7e2c005bb772a4d4c5ba6644986052fad554f042ab0074a8f").to_vec())
}

fn get_valid_rotate_input() -> FunctionInput {
	BoundedVec::truncate_from(
		hex!("e882fe800bed07205bf2cbf17f30148b335d143a91811ff65280c221c9f57856").to_vec(),
	)
}

fn get_valid_rotate_output() -> FunctionOutput {
	BoundedVec::truncate_from(
		hex!("2441c10b0b6605985c56ebf6dc1ca7e9a0ae20e617c931d72f2ec19aa40ccc8d").to_vec(),
	)
}

fn get_valid_rotate_proof() -> FunctionProof {
	BoundedVec::truncate_from(hex!("14305744fb26a377656a947cae0874c14b086de9d407bdfaf415ca9f47402c04144589183b473537750e7211f93671e324825db673edcf5c0839b08eecba08202966ba52dc07e1bf9832a54770048b84999172d47c57628758d8fe43dd9fe1412e6f8c0e75a79cde28e0e24eb09f9d23309defb07f4a1761deb6598de77278971d2d914930ad2e3ad8b6264e595a0516a912fc9394c93fa61146efc54d61e5c32378a5d4460aa2164422702f9401fcfb3e2b991a0e5b847ede3ea9ffe70a55100203abc0636c101adb6546c2f7aaf32d79e69093afb40c3c1a674e44a1ece76a1183fc03ef9553a7728672de2aada5d5582b5bcf0859e8c312ab59429553ed6d").to_vec())
}

fn get_invalid_proof() -> FunctionProof {
	BoundedVec::truncate_from(hex!("1b496d04c0e12206bc846edd2077a20b8b55f65fc0e40bb8cf617d9b79ce39e508281ad49432300b3b7c8a95a0a63544f93f553fcfdeba38c82460888f4030ed1f67a1be666c12ee00658109c802042c58f645474fcee7d128277a4e35c1dd1504d33cb652ec23407cd3580eda0196dd97054eb5c2a817163d6997832d9abd422729b3e85a15941722baeb5ca8a42567a91c6a0b0cd64ac15431fde05071e90e0d30c12013d5803336cc2f433c16eaa5434e30b89ce7395c3c3cda29dde3be062281095f143d728486c71203b24fa6068e69aabf29d457ffadc6d682d51a4f08179d3240bc561ae7e2c005bb772a4d4c5ba6644986052fad554f042ab0074a8f").to_vec())
}

fn get_valid_account_proof() -> ValidProof {
	BoundedVec::truncate_from(vec![
		BoundedVec::truncate_from(hex!("f90211a038645b8e6c46b9a14c41c828f60059af187133d96aee55d0d3cf6395688825c5a0490c98619b5bba86c1b9848ba341036acec68352acf260386b4f355b39704401a05ac35eb29ff70c8b5f28f779b19c65c2a063ceb7fe5bda442bbb5f12abb35720a07c34eaea408909cb43c23a0b3f62a1c2decdeab3d170448e6cdabb4ced568c0fa0132a845bda825e6b08795269b6d2427e2a8441fb10fa2252b486f2570052ee74a0f4f964c01a6cb02ac89309c8da622391af385b11335e689b41efe6c3bc4a0472a08d1f8a5fb822eaad95c960f94ff2d0ff044af3571e15a5e76c9d825f809a3977a0c72ab2ac0efc6ee7f592f648f16de6e544fa353d36cdf860f7a81f78087c8726a0b089c3a8d71f5922c23c4a104177dd54f1dc36ea70638d7a4b9f151a6570dea7a09f6a4274a32570f3baae2e98dd182a7fec3e5eec1e696a590c7dd940e8c73373a09ea9471274377a2e7984ae1ff9ef37f5f3e76d70435055eda5dceb075ef1b6f1a0290723fb9572377e319078de523b7aa38fdffe5dcee03e6e1fd3818f3d67c48ba00cc52aac249c233c20c14ded153a7a51c82c58b964291566577fe121ac457765a00a373e60f23013ad9b4d7dc2a724e015d68127a766f2bfe8f34a0b3c3b600ce3a0be4a8480ae62ed2e190c5de77cf5baee598de98663fc6cd750cc2b39f73da8cda02a6b9855b1510c899018c16bc0e3b088be6b29020f00d585aad65be22c90cf2080").to_vec()),
		BoundedVec::truncate_from(hex!("f90211a0eb1e0c5a95a213c5e66de37408cd98b09ffc32c583464aa43771cd8cd4f63cfca059030aec14fee0a7f00e60baeb603a1759aeb997cdbc229548ae758d7b351a58a03dd7b95b8411564690b5cfd84ccd47f8312c84d78d9bfe1924221a70fc0d3f59a007ac08140b3e664b3ed7f023ad0b7e7532eb5450b0117dace52413b321dace58a07e5bbaa3f29f9912adc4bb8019d019b420d57d2ff3cc7922b8b7dc9aa6975f19a013a1a09b85139703bd85a6e855d6858bad57c21b616ab5e3ee19cddbdfe3af82a0b69f7af308a6f1f30c7296863a6b28032e04b3b96578a4fcc9b9703ca6368a53a0cac2d6457c2c1bfa916d9a953e9c542e4308581e6371408dfeb6d4fec5105400a0e98c2688638f9e95e76b69c772c8a0025bd1c2b7c10e2ac167e5634cebafa81ca0e1f121af1b8ecd4cbc2d2fe226e24519d1931351afdd1bfbafcdcd259f50ffcfa085dd69ce4daa8d485c5269694b8e851e507bde5f498608e1e85405b4e0082d42a0fd46d5f296667de0dd5cda451d12eb7e902d3725f81420cb9aacb305f120152aa04964a50f96c4c79adf729072167f01f3334ccea6a302231bcb5cc1bdbe909275a0eada9559673774b757867ced143825d778ca2dc266be7b6a08774943e3f55e79a0fabc3e99ec7b5f469a94968d64f22e5ef8e62ad92888783bb17f954c52656b2ea0975cab4b17bccf4e3f89b8434deade58c862b4c39707c8895766d46a3d5bf7c580").to_vec()),
		BoundedVec::truncate_from(hex!("f90211a0f42cb2c52e1cdf0b89e330b051690d8d6f065fe04b81e2888e75388a2a9b3a7aa0f2e2a9b33ff3a80c02786f7eb16629101ecc6d597177113665fadf691b02d62da08f9b93a1fe9d4f5b4c3236d7d74b9b85798f702f04c23a6ad22f79a53afec507a078528bf574e9d77eaaf22206f231c6946b0034b22a631e11ab50ac4550ff657ca02449cf2c48a007e11e9ed983b7c7d0c585b649678d28c9b57760dc7085b6f0c0a0350ce21220c9cdcc4051198d3304539cd0f65a1e4c450777c29713b2146a1109a0dc813e7cfb36f9940bf90aec22000df18fcb21cd550e9cd69f6a2040d85f76d3a05547fe97f727c2d9ab4d22953be099229d5df7dbb6fdc6266022fa75ce9bbe2da055d5b1a12344590b5f9f870c51ca9174a2bb051288e61a9c97aadacfdefe4c60a0549d85414ff9ef12794e11350650afbc4f84eb191ad788a063bab53e309d95fea007c5feb4db19ab4773e6f55a1fabffdecd3c1859fbd104e6e4859155e3d93987a0b484610d78bd5e23e8c264870196f1e40bd5f22f18052746f2d7d1fe6dd7e37ca07fb1da0f84ddd3cd1fed390678306b2f8ce8e55a6930f431574be643568351f9a02115fe4fb908557d270e90c008b38adeef56d7e28b6db3f64563938e581dd096a03f7e4c1dbfdf4069f399cc6a02eb21aa663bc1d3f2972cceaa5c9ccb5422edf6a02f752693c97e8168e94e73ec6bea04a1d7d29531a1547b3108f5092b252802b880").to_vec()),
		BoundedVec::truncate_from(hex!("f90211a05563b9e3e4bec451c82c8c0f7b152b1fdc7db786161ebc166e5aa24d03d0af60a093db5d95c020b5335462faeeb34059b2c9f6a8d7941c524c81e60cb74c00409ba08f7e91fc2911cc55599263d5fe4809f0ebd7d900b36671701ba7eaebc79dc76fa023048ea0353a144a203ed9ad6b305825a742eb5b87577b7251e76af118178441a01548e41366378f73a46799d7af8c6f33a5d9a3c396fd599a6867c857786b46eba0b3f7908d88f6b141314b6d50231cc4a881cdc85a022020d0699e5789313c0c8ca03e1316cc4fb44bfb4efc05ba6b2f714653a60c6b1e0a243fca922fcd431701a4a049f07258ab83cc1e04de395a9c497671622545669666dea00280f358a37cd9c4a095207deddceb6c3aedfa914cd4652ad16f1d69370608e6c59e33a014093e6756a080dcb09962b0e2e3f1d4c87a16597286e1a46696abf79f16accdcf6e2ef5a5cda04490b4e4d6abd01953c1c8b2de12c7889bec9330b8bebfb3438f1a16e604d5d4a0a58195f9d5e694d7c5496522603560da704034ec12db34356474b2d09211df77a0594b9f0ce05ba9e619e3cde8a228affa2470f802d8f03a4a15437cadb1a2f74fa0aea22c35e10ac158b10923972ea6280adeeac8b56219e3bb90ca4c44dd602378a07df6ddc4ff77b143796c136fcdd336eff1432e6413983345ca15cfa03e97ed45a0514d3455cee39a69a719906411942520cf9d18b43519391361dc8d4a2ffad9b280").to_vec()),
		BoundedVec::truncate_from(hex!("f90211a0849dc399622c63d320d559448cfad78315a2170ac2785dd74f2c49bb66a7af32a0f68193dd80ff71a112cd40b53a25f6f2372a75dde0f28e177655355946c59bc4a0f2d34513a530a9f56d784fba114aa2b3a5c978b294effaa5a89137d21f03b587a026ff44962bb4654009eb194cb897bdfa18b3b917d2986b1da6d24c15896a457ba0f6197efa441920522b66404dda6d16d8c711b99aa7d4d3468845d4d7b0cf232aa0bc3999ad4ef55310671b17ee44a72b1a26bd571eac53bd6da4001053fff55069a03d1d607c0cc1adb638329ef9bedecd871f5ce7415eb4d80909c4f4b0807d5c18a0136abb72b80195bb09498adbae02392f6150235bdb0cade998f3735e95f9afc0a08947a5210863f90607e66529624c15ce4a0564b3b7702d85f2fdd6a1c12f6b0ca0af0b8e591083892f84d3e1abc48b240fe1aeec8f9c935475401fd1c01026779fa0eb3ab7886f06efd3dc5e99a7144fb7d801a0e28b2391c134edb63883a0fcef24a082b06ba84cc4342e7c5ad4275fd0581bd04477044465ff25513ff2e38ba8eac4a0705f5c0dbb44ba5641064b839753331b8c0221b7ab38790c3f71d7aad531611aa08f02619e4f42e405fa0fe305b667173c2917f4659101c59352ca8f064e1af5f7a05c9bb0630ed3b10b28dbd6bdabd846832b134b60eeb6ca721483b170c402d69ca02ba210da4d786618401423df4b0c54bd7dd55f3a2f9e5fc275f21b955c10248580").to_vec()),
		BoundedVec::truncate_from(hex!("f901b180a012d4abe8e0be9f1ca3024c8c734d5201d4312415207baa191632aaaa22f877a780a0d7e0b129d1581066c1ad75df317f8f362bccbb9de3bfff562bba3e8797eb5538a0398b1d9a44c778bd0f658f430097347d57b9f64ce5943e06c8930d060445e666a0265da05c525fc7b1627187a0a4fb7e60ff256e45be9564578fdec72c084c838da0c49cb31cd553fc7d54c9e2e8335465a7607377c5844c424790cfdfb82060cf7aa0837b642b959d35cbe90cc912de29a9032e837c47c0be04fdea0d38b023084b6ea0044c45cabab3dc0d4669326119b78dca51316bf704837ac712739c73464831c8a0855ec46da10e0b04fcc94f2b54795b35f71891802ef5599afb92e3d2909a0ea5a004c86802b3e15cf748062ace9ce5e02acd0131ed86e30f12e6edf9fe0ab4fd6fa010c903eabf207c4f03cd97b3377ca35127e657bedd80991ed3015ca6d13c3553a00d3a77a656914bbddfa583e3841296c1a65a6977b6c4c0cda6bc34fc8bdbdb9580a05ffe8ebced571d7346527142d9b3e1730437bd063ed35aa683625d9716784e37a09fec2225beb9bacdcb4dfb5a186e412b7017c5bc41aefb2bc7604018b46c5c7280").to_vec()),
		BoundedVec::truncate_from(hex!("f851808080808080808080a0e505cae6797de2705a3a65c77ea7c9948f0f8930c14cf7b8e5679504de51062fa038a3cecdf51a92be5c24f16c7776211bf79a7a3bab9d0b9178b00eca71ddd76a808080808080").to_vec()),
		BoundedVec::truncate_from(hex!("f8669d3cdcf5995402ae9b130ce74b48fec0dff98226c7106c2618b3923cf8c4b846f8440201a07f9340d34eb810527d82b795d1ce74ee9fc8a727f5dbab7ad1399ff2ffe8fc87a0c95cf5b2451d3c325fc0b238248ce5d59c59374241561d45b7042b3189e4c413").to_vec()),
    ])
}

fn get_invalid_account_proof() -> ValidProof {
	BoundedVec::truncate_from(vec![
        BoundedVec::truncate_from(hex!("f90211a050da92c339db0b71cd6a8ac7893a6b8689ec5a3a46a0231b3ee2bd1baee75e1da045a3d973eb74a02b762d8b1ba683f39bca3965806276c8ceffe2d2ebc6cce233a0e88ad29ca98fa08f59f2a7f0110d63505d99a173628643290df869c4d1fa312ba00bb4cc9dc0b1de6ae0d80424b1fa992efb400a07a0e84615c91762fe734b2d0ca0a07e495d39bf2b779405790c6c7e7eb1cc3c803a88db36d1ec600fb0e555b5bba09a1c776e89c8be75d0a9ea022c05fd2ff095869d549e74a8fff7f2fb2deaf738a073b874e49e77dfd9312d7b1afd1ac10e02021a1ba2ab7c97ecaeaa0e26a34027a07e3424405c13aa33a2eb9ec6d8640aa1f67fdd8c8e9e4276334515b1cf1df65ca0246b93b2e3cc625a5e75b40165c6cb95ae8ffb9406563d34092d6359c7616aeea04d2fd8fdb1ab7d8f8fc6079400396fec828914230fade3794f13dc5ae7f6bbb8a04811b9efbfa8d495c5be91be78372b4a29140bd1e092e793db50ed9c495a6d54a02e1b3a417e8341dc8e1ade6ca527778192d33c7c827cfa63a366d007f2884e24a0845f4f33a4993d85766a14222cde1d124bd0f15523d239572883258a7bbcccd9a0ed2021cc2206fcfd9f80d592890b1b4eb615fae4f11d4e4a66d54a6767908901a07d46bf6e9dc9599eb7ca036aa976ef9cc63f02e9097252799f5d3a8792c49620a00b58d1d2cc72401c7cb978d34e15f74038ac63355e415d53b894179b8938dbb780").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a0f7c14d7714348be36359dd28afd64e2fb72679a7ae09a27027fc95e335bcde1ca0824329840722c728e0f19ae424caad4581ac42015a4ab8e9d3ea550c857da804a040d48c9df564c00b11d304e2a2597a35b17b25429c3850c4e3fe4e9a278bec88a0a497297590785cfaa8491579745c077b1095348912d4e3288d8f00857ed9db5da0b0ea3abfcdab8c6cf03152cc7a57f602f85d86f4bdb3d1ca2242a5e737561bbda06bbe0e0416b59f1c4cba36afdee766ea4689f1c1ac8e2245f45c2631e2478119a0222dec72b36685a0ca89e49ce87262957f7f891e695ea8ec52e25fbc3a328589a00b3cac878feb2bcd5fc3d49fe5f607eabf75f014df74a268d4aaa1d25654d030a000deffa5e2879748ef9a634a3573484b4dd259c0d4c10453a7e1e3504b56322ea05c356b24b3b36089583f650cb954f884b05275b09b7715a2eb3cf6fa9175738ea093abf2b2cb15649c192d0d79f62627ce634843f84ec98eee99267c1354b5135aa059e9c60388154b3b810ffd41f81ed9128c8091a12e0c53062d9e7430fedf5939a06855c9a5622a40b5bce572522e4774986c7061557d2f1b8f7070d8d397888b4ea04d220a5fb22e38d64cdf4b46a42898b9f1ce9f316f1d332eebebd32c0cc59000a09004930139d4ae94070b29245230d5b28b25ac59c11339928a2eb547f0828341a00f37af44fb487a5ed675e12f0566a54e59cc025466e91cf56dcf348ff4049ed980").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a0e9fa1abfa1f1d84a27da9448b42e3c0f5c60c54a1e8cb90c9e28b60824157380a05e977e1d37e502ac74fd54a2debf7e9b7b6e64c261e45e9b0610bcc201ddbe93a02f8a351ea5204d62c85fe6b564eab729fd556b1941a4f83f6f4b6e40e4102869a0a4b62da8ab84fcd0cf425fba4fd03ad7f1350217679e105e57ee146f64b07e07a061049f894647148c39ec3d8c4563d22670ee697f2e4a003513595f5074fe0166a0de1551dd310c9206da56ff9288dc518cccf7cdfa259cc3ff0318a6f3f7539988a00e600d8cb072056fbf1f5bf7d18aec2eb2ba57e93b5e6bb3f0d36042ec8fbe9ba0fa02eb32060ca2e3fd46e39a8456f02156b8efb457c74ccab5789bce1d142613a0919bb37876273e3283660eb2c575ddcfa99239ab79cf7edaf64d5591689c7777a052a8ee269c13ef214ba56ff0ef6b3cb11da6b12ddadbf1883831e91c6768bf60a0028fdfd852916e9cfa13eee9bf6c540bdc7f6d9b18eee15e11da66a8cdfc933ba09d581d74aa42d7974e122d3a3ec6febaa74ca9f714ddf5c52a5bfa9ee41471e5a0c5608d4aef23664aaaa38aa2425cf959b62d30cf22a0d14147a3cab3d4178fc3a0beb1d967ae4415f30d7730c1bfd43446e24c5f0210cb3a0f5a9bc67e9f63228ea03117ae91a22815aac4b1c9210ba7a6682697a73cd68d7741d693c1cbd1925063a032cf653822d7a618300ef2113f0ff0be132dda944106f25350b5f37451c740a280").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a0f284a2e627542f07910ea0cb276b0b7813f3b9c056aafe496b3e7f93d1b3aa67a0d45d246efac9fb2e0c8052354aa0eebd68a28e9606efbbd4a5c2f9e990dc4d3ea0fd5d8349c16fda7a90a9c778cc74126188887aeacec8761349e1863d4008602fa022796160a8b1259fca46b22aa863131e970b077a449a5be4c486c9384335826da0b28076746e56b0bc37fb7586e2c4f23b624523d8e2f7abdffa73859cd531c12da08af556fb72bb802fde89a5562659959ef83a7846f0ced10ed6e139b44016bae9a0f948d4f88be556c183e053c131cd62aa278bcc83845437bfc03721828a3e2082a038c90f875a89a76b5b42d7c843ee790b759e482570a0bcb4d291496a40815093a031b88038ca3cd315ba56e783d4423c7c306cd9567f5a9eca972ac631c4c58e83a0858cbce5374ea0469281ee65c5a1aa5cfa19e7f7df02635821be244a5d39a38ea00cefc972ac8009f230bd9c8015753e98072b5f71d3a09093309ac6f09002f420a0e5fb8ae4800ad431a827003be4d719efcc29424f3ad2fbe483a42ab724a8610ea01a584c371a17ffc56a7713b2c6bb65bbcbf63c9d6382e0423dd577031c63842da0104f13e37d23eed61ebe6b78ee93ee9c30c3a92dab0ccbc57715051e9744eb58a0b211502efd34235ac7f948856c809f8aaf5e299df97ff24d4fb0d53caa3d1e83a043d845df46ad73ae3a9f2bfa319c19e7f760922f1268d8b96f0a54cb8ae88ab880").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a071241195c881f3437ebd19a9eccd009595c10537df66917a8fab0eb664f834dda0122c775309b9cff05db80ba77a60604d0fcb8a836a5e79999943f0d150297e19a0c32190d1506259a9ffa2ec1fbff6b23bd35d4e6bcb063b19a22ec10b914981f4a022a77ca63522f76d016d04e680d4c27c3ceee14bc4548f9e08c2cc10f9e1b789a0c646ec46e8f8d5fb7de785fe967200994afec4c48b2bcb001b5aed20db936326a0e20c61d63a3ac612051c43ed1acce68e185a08154e5f44e8eceebac0f454202da05b17a5f4ba7ed711f694536b96a69549fe097ba32dee1f9c71eb19a0533d46baa04da0bc8c8f03ad8f1efdf0da738f24c9ec4549acc71d43ae6607f22601ac4f38a08ea8a34e48a70ccac672eaa2c3a4538d61d38cb5a143a4596d571904b6e3181ea0148252504cc36b4f6b1ef7183df2ce176963bacfc97ad3949fcb6da7d4095821a03d63131beaa2c1137d599528084b0aeb4bea87ee8da16f424dd93c3b90087a75a059f94b55179b81bb657f5021b161ab30fffc8620706a858de7103a0da99a262ea0bb62efd30271c9e2bfc8a4938ebcf4d90623d1d55ffb97399f6456c597599464a024a60032c223c88b91e1fc98db296e58468ebf38eed7bdab0e114cdd754bdc80a0271ec93cc3efaacce706f26a3aa42d6f7c9d8fd6944329149ad63b43c78aae34a07caa42499d46895c9b948f37479c6572573db5b644a0862168e25e4e3bfdb57e80").to_vec()),
        BoundedVec::truncate_from(hex!("f9015180a09089f0d1272f06751d391dfbc7b6d49b39731b8a14b5e5e97d45e34d89df0f3fa0820bbc641b62cf0f6a4c3836017cdef0bf7f43c1ee8cbc76ce7b5dcd80f58b9480a0fbe1f0ac8158473558c7b9964cc295027449f6e960f5f6407d9ca1c9ef15f7bca0a2fb890c487021019f73371bf6798e8db8b612ca3c7b30fc3495441a1f9518c4a02cd1ca2531caa6e63ac5f16e5ea76018826683f10442ab5c2b1f9963f23b011ca0429bcf37f564e67dd5764f96fa79532113668cbb32059affdfdc82cfdfd5d1e18080a09be000de088393ee33eac568ba00e318f0ed370eded1cdf38aa75ad55e63945380a0a9138320438845382842e94a5b4ea6756af0c82a0f6b4f17eaf049d617aba98ea0229898dbbae35aa9ef23f2a46c26d419257c35ba11aff1b02ca2024a057f8acaa0cc4c22a6806f250facbdecc1d8874d430ccc277d68ca91b5fb10b4d9f7c681578080").to_vec()),
        BoundedVec::truncate_from(hex!("f891808080a076082e119bb693f858172779676f80da4deb1fd75b39db89ec6c96e36125cf6a8080a02b87e60a23ebea051ea7f029c26c5fad0ba86fb8d6d5d4bb563f48ddbf7fa6aca0d9693138b984cccc06a7461c7f39cc28947c9dd95d94bdea1047ddd420b81360808080808080a0ae23c016152c96bfa600e365cd62d6ce721f0b0d310e3c7c18b8a293b722a4ab8080").to_vec()),
        BoundedVec::truncate_from(hex!("f8669d3e80870bed23e92a482b9f577efea539b7865c0383284e1bf8cb8ae0e3b846f8440280a06801798586ca88b0ef3b4fb3f83162a9f13e5e242b4c8024c490006054e43933a0f99c7a628a59cf1d27d3a906618656d06e3cdcbcd5f91503c002ea2f2420bc01").to_vec()),
    ])
}

fn get_valid_storage_proof() -> ValidProof {
	BoundedVec::truncate_from(vec![BoundedVec::truncate_from(hex!("f901318080a006a2e0d8e156dfbe4422b119dedc75f489184d9016e2608b4a5311416cdb51c080a09ddd70915eb71e1c868c88a5e19e1b60b8f7c12727c5db3829b5e38d770661aba0b15cf01f62cbd104e15831a1d5a4ee37093c493251983f17be2c773ec03760af80a053d7827419966e58ff537e3188131e441342bed0f491cbce461183c2cc94b507a01e2f406ae757e5ad8854495c415a87688cb69fbe6560af8e14fa31fc2dcd44b88080a09943ce5c17ce69fe140a111cc8436dd434034352c4dbefa7623d48593eae2987a013674f92f711a51bfd038a63208ef702497ccbf468405a9ad9378a57e03ab67ba005af7977817c87942f8cd44f542cf831671b8bc88763dc3fd3efb10c41497b02a079866ac4ff54c3062d8fbd4fa347961e9a905b4114a2ed9785e22a5c03f4ffb88080").to_vec()),
								   BoundedVec::truncate_from(hex!("f8518080a0b2edb5bd491832537cca93fe2114ffd8630ab03a75062bc5917fbe9c51887bfb80808080808080a04c1bf5ce293345af77adc5363ffddf883410cb45a80bd14cfa4e4a7f7ea9bc7a808080808080").to_vec()),
								   BoundedVec::truncate_from(hex!("f843a020b3c568e6b9e23c87101e15642000038e2a634c0eba9355f868407d119483c2a1a0874702fa5c283bf8fa7df1401c628005c8501b3375b04d23b7732f280953adc5").to_vec()),
    ])
}

fn get_invalid_storage_proof() -> ValidProof {
	BoundedVec::truncate_from(vec![BoundedVec::truncate_from(hex!("f90211a0f0a16ee9b11528f3da8796229dad134b9085ed9428d868e6988f9b2473b59d6fa0f8175015d0a3df8fc451d2bd3d64a34e0836f3203129ac567e869f1157b488dfa0f9d56e943c6962cf8e2ca51b94b54307eb45424ebb84ed079b417cf03a85e298a0408af9f1c5f64ed6c517b1dbf661b75a705ef7d78bcae67b9a54c1e8052b56b2a02157d476a9a077cfc9eb00ead5ab65dcbfe363a71e993c3602a66c0fccf13e4aa00772697ebf25f2e83830918bd52bbb9600c077ae289e740ae76c7bdfd34b7ebea0a1dd0da76aacf7c82629c55e4b956b2e9ef77d7fdcee1adeb23d022f0950d554a0695cb723c857d98ad1c96a372f7983bf771556f4608674266a0698531543217ba05c0fb347305720b81c7d39be6fd5b2083af607654098a0f1418ec111a846510aa0ecd30808bffcb164a258c332a29f3050e9e85d28e988305b7f643dcad4f32c8fa0ec5ee93a7ede0a9c641dcd7515c1408ab48f86b5295cd26b3d738e8d8ac7829fa01434a5f6054456bbce0a59ba1c182eeee8e64fd6762ff365e550ca7cd8cedad0a0b4fefcb325f044a6663c9441ec9f025718d0f2d7fc1c29ec819f4a366cafbb6fa0cc26bfb18151569b0f765335474fa3840f9093385816bd14a4a3c553fae62949a06a28c02f7b649bad24b39d9a4e9fc4c8e93b1ae2b043af4f5bbcb8238e193eaba011ef889094bf6ca740810423041169453b7daea3df98b3018523f86e96bf033580").to_vec()),
                                   BoundedVec::truncate_from(hex!("e219a0053d037613f1c22bb588aaa70237b3798774d2b20413c686e2263daef21ec226").to_vec()),
                                   BoundedVec::truncate_from(hex!("f851a0c45dca792d516550b57f7f31e33c67f0e6debfe0bdb3076fe0078c65c5afbf8280808080a022e43fa2c06d3d498253aadec7a7db94183eec2aabbdf2afc67a45107d19932b8080808080808080808080").to_vec()),
                                   BoundedVec::truncate_from(hex!("f8429f3841a49a1089f4b560f91cfbb0133326654dcbb1041861fc5dde96c724a22fa1a0efac9989593dfa1e64bac26dd75fd613470d99766ad2c954af658253a09d1ad8").to_vec()),
    ])
}

fn get_valid_amb_account_proof() -> ValidProof {
	BoundedVec::truncate_from(vec![
		BoundedVec::truncate_from(hex!("f90211a00b6f30b08dfa4ac78e27797893a5ce85510689928994a50fee07aa93873eddaaa0c569a854a6728ab4b4d9cb380cfb7a4cdfff6b5fcb92e59f3dc874919e4a9030a06b60fff88860165aba1e23fd69faf4c1470c734fbc8e549dd47324dc69aad4d3a0a81d213978cc344adf44a4ba4d775083e9792f1b025a630064148b4b835a8c68a0ff020c57e7d889a6d4a3f697ac5d4b8460aad8e8c3bb9223125ac291be6918f4a058f34f0aea9b5b6a72ea35626addf9b2658a937aebe0e4e3a31aa44154eff9a6a0242b9118e17a6f3124bea0351382655a834bbde6e8ec093fda8489254785f340a0095736bfc8db810935362da22bc123689713946ddde282c90cd823761a1a93caa0784e0a50a0909e8534fbe6fcf1a32a01b67e1c2fa10aeca6b98518d508309595a09f7b741a1b337f76e4cd5c7db97cb0407744713bce40be7c48b5125159659cc8a034ceba69e03bad2333a9de9351565b320d4bbeaa8e70dbb1e95967f3733ae3c9a0e689522034266a72a483b3013d9f4de43cbb12e7ae5abf1275c68909c4d3453da0f93ad30781fffe999b9362dabe417c9cd44fc0f6cf864f39269bf7679dd15b45a093bffbaa584c23973633a4ab4fc8c0ec65608d01438a4ec5f0743d051ecdbb6ea0aedd4701672a9797358c327cdc00407a886f124363e653d7845ef0a07869e7a4a0e397e94ad5414179b62e1f3b5ef4a3bb4802ec6bf8d220f487c5780baf11708780").to_vec()),
		BoundedVec::truncate_from(hex!("f90211a06a54ae7917babfd0a3c0b11e509073197e143b7d275a4eff899e4e124b3c167fa0af2619e44118da5f54d381658207a4d28fdac6e5f4da82a8382f27cca7a548a4a017b15c827c6ec9d62134390aa6d284319a62e9ad659f5c7df395d9d62e9703b5a08357f8027dc0a05c83a24f1f127be020a3862e1795c36a0a0301a608bd4e4ee0a0bab6685ce515915e6b9fa04d49c90041337fe02f3bfdc933b78e0b5a9af77f8ea0b78ec28f033b3524e3662e043a19d4404b55f5a8945050a93036fb7af61b9320a09b48f892d8348ca9cf7c96d13be0cc07ec7c7d5b6bb65e94d34ab4dbb32ed3b1a0d422e3f5d8ebd45e7104e2c13a1fe1225e9998cdadc1a031de09938b45fe4cc7a0d54b2e7f2140d03c812db93cee604e9ad84ed81ead8db57e6589f379552dec05a0b05799035644fc46fa57fe28a227153a566037fd1b794b8e530f2610043f6ebaa0c5ebd8a318a1c6f8a15b32ee0209c7c0602a107886e3fbb4c71bbe287f511287a07285d66e91f89ff0af7566b78ec0374b169bd71bb05308a7624a12d842b33b11a06b3468e6defb2c3312f869c8748edf5dea499047bc25a1a17c8dac0e40d21524a0f5a85b7776d6ca67cd84ba96ec78fdae8410466341760825b93caf14de3dd3d2a065f679fe0668a42cd9789e2f7f029e5a32d261f6fcea80d6ff884c39049df36ea012808cd8af54ef89a46ebc25ef4953c77b91ffd3ab99ef36d4e8f7776eb44eb180").to_vec()),
		BoundedVec::truncate_from(hex!("f90211a07ce45690d758979891edbdd004e5bc68f1e997eed83ace1c106b0499de1ddcaaa03673edfcf0df1d1e59216a78475bb8a9f1f813e7d404b5f7d28d2dc5e8934164a04cf5d36b738e617123d76b4c445bc686ebf7e93da0adb5e5b81f622efe37843da035b4e9465ac5966f731edab081e09a227173f1d7452b253f97c1c4fd9e1f2fa3a09799ef3892d802f1a8672ce6b0bdf50a9ad339ae7f0b8bd3c16330b70ef820eca01fbd7ac833702f49138c14f1f9717c252929a1aedca96e32018c8f30f23f9348a0c162f7d02e366a90ef133e634f66c25e46b0ec3b98d6d4cf687ac8d093d4acb9a082956aaece11b9aeb7b1d6b82d4f94b0b12ab42597a30afba4043d6b7c151fada0d198d259e580615f1f92336523751485ffce9657e0f9b43132f6ff140ba90410a0b3c41aa154834f33172627c24b4a9c6428ec36ae004fd02f92948515095e2206a0b8d9aa9329ec16832d0e26227b40a2a965bfde84565168b6eaf12e8bb5868511a055cc80258b9913038beeffede6b145c79ffbec220fdc5b7093bc52f551e69d5aa01143aa5653f17e924e94843127aada1570355e5fd03a939574cadc1a2c8a4f0ba025b94c1e1e0057399d77c89535fb1bec53b139f6c3e40a888fcb478f4128592ea0bcfb0ac9c0e87acf3c573aa1b796fbd82d79960e8a6e03bf9353f629e38d67d8a082c268e4e6c5b02b442c10cc047c6cf7294d9cc269b0c76aa940fad2cbd87e9280").to_vec()),
		BoundedVec::truncate_from(hex!("f90211a05563b9e3e4bec451c82c8c0f7b152b1fdc7db786161ebc166e5aa24d03d0af60a0501e97f3c3a4fd6c4d50c361208d482661d1c0ae421fae6fc3b50348d5c90142a08f7e91fc2911cc55599263d5fe4809f0ebd7d900b36671701ba7eaebc79dc76fa023048ea0353a144a203ed9ad6b305825a742eb5b87577b7251e76af118178441a073d28362a079e57422d1a3350e1a510fe5cd9ed2ee79f58446a4c7317d442a4fa0b3f7908d88f6b141314b6d50231cc4a881cdc85a022020d0699e5789313c0c8ca097baf0b13431ac58b9305a86d96ebe57213e6c413c0cd5484fc74557cda3668aa0d333381c498cbc66cc97889815cbaad56deef9498c3aebf6af39c818871197a0a095207deddceb6c3aedfa914cd4652ad16f1d69370608e6c59e33a014093e6756a080dcb09962b0e2e3f1d4c87a16597286e1a46696abf79f16accdcf6e2ef5a5cda04490b4e4d6abd01953c1c8b2de12c7889bec9330b8bebfb3438f1a16e604d5d4a0a58195f9d5e694d7c5496522603560da704034ec12db34356474b2d09211df77a0594b9f0ce05ba9e619e3cde8a228affa2470f802d8f03a4a15437cadb1a2f74fa0aea22c35e10ac158b10923972ea6280adeeac8b56219e3bb90ca4c44dd602378a020f568f44a963586f840e051278b87384e6df051ede75fc691d5bce2c9f36b3ba02e2721827c3233ba86b3d52599e689d8d7cefde6c5cfcf4ba19614120c4ca6ee80").to_vec()),
		BoundedVec::truncate_from(hex!("f90211a0849dc399622c63d320d559448cfad78315a2170ac2785dd74f2c49bb66a7af32a0f68193dd80ff71a112cd40b53a25f6f2372a75dde0f28e177655355946c59bc4a0f2d34513a530a9f56d784fba114aa2b3a5c978b294effaa5a89137d21f03b587a026ff44962bb4654009eb194cb897bdfa18b3b917d2986b1da6d24c15896a457ba0f6197efa441920522b66404dda6d16d8c711b99aa7d4d3468845d4d7b0cf232aa0127fd01115456b93d588ff203b3edb888da66fca0682e325832227d74500a56aa03d1d607c0cc1adb638329ef9bedecd871f5ce7415eb4d80909c4f4b0807d5c18a0136abb72b80195bb09498adbae02392f6150235bdb0cade998f3735e95f9afc0a08947a5210863f90607e66529624c15ce4a0564b3b7702d85f2fdd6a1c12f6b0ca0af0b8e591083892f84d3e1abc48b240fe1aeec8f9c935475401fd1c01026779fa0eb3ab7886f06efd3dc5e99a7144fb7d801a0e28b2391c134edb63883a0fcef24a082b06ba84cc4342e7c5ad4275fd0581bd04477044465ff25513ff2e38ba8eac4a0705f5c0dbb44ba5641064b839753331b8c0221b7ab38790c3f71d7aad531611aa08f02619e4f42e405fa0fe305b667173c2917f4659101c59352ca8f064e1af5f7a05c9bb0630ed3b10b28dbd6bdabd846832b134b60eeb6ca721483b170c402d69ca02ba210da4d786618401423df4b0c54bd7dd55f3a2f9e5fc275f21b955c10248580").to_vec()),
		BoundedVec::truncate_from(hex!("f901b180a0df708c37fbf35a98aa1e007b278cbed7093451677b9263c5f3f3462c59c817d980a0d7e0b129d1581066c1ad75df317f8f362bccbb9de3bfff562bba3e8797eb5538a0398b1d9a44c778bd0f658f430097347d57b9f64ce5943e06c8930d060445e666a0265da05c525fc7b1627187a0a4fb7e60ff256e45be9564578fdec72c084c838da0c49cb31cd553fc7d54c9e2e8335465a7607377c5844c424790cfdfb82060cf7aa0837b642b959d35cbe90cc912de29a9032e837c47c0be04fdea0d38b023084b6ea0044c45cabab3dc0d4669326119b78dca51316bf704837ac712739c73464831c8a0855ec46da10e0b04fcc94f2b54795b35f71891802ef5599afb92e3d2909a0ea5a004c86802b3e15cf748062ace9ce5e02acd0131ed86e30f12e6edf9fe0ab4fd6fa010c903eabf207c4f03cd97b3377ca35127e657bedd80991ed3015ca6d13c3553a00d3a77a656914bbddfa583e3841296c1a65a6977b6c4c0cda6bc34fc8bdbdb9580a05ffe8ebced571d7346527142d9b3e1730437bd063ed35aa683625d9716784e37a09fec2225beb9bacdcb4dfb5a186e412b7017c5bc41aefb2bc7604018b46c5c7280").to_vec()),
		BoundedVec::truncate_from(hex!("f851808080808080808080a02333c705973d6cc84f4125f44e430f3b8ff97ac999dfe2281dacef0c13a8041da038a3cecdf51a92be5c24f16c7776211bf79a7a3bab9d0b9178b00eca71ddd76a808080808080").to_vec()),
		BoundedVec::truncate_from(hex!("f8669d3cdcf5995402ae9b130ce74b48fec0dff98226c7106c2618b3923cf8c4b846f8440280a05ad2ac9e12fad11a193c2bbb0a454c01707fe0a62302d3273524ef9b3dc2bf69a0c95cf5b2451d3c325fc0b238248ce5d59c59374241561d45b7042b3189e4c413").to_vec()),
	])
}

fn get_valid_amb_storage_proof() -> ValidProof {
	BoundedVec::truncate_from(vec![
		BoundedVec::truncate_from(hex!("f901318080a006a2e0d8e156dfbe4422b119dedc75f489184d9016e2608b4a5311416cdb51c080a015503e91f9250654cf72906e38a7cb14c3f1cc06658379d37f0c5b5c32482880a0e65c16d5aec6085f3d03f39640b1cfa978134e271b7124b6d7932acb8e55065880a053d7827419966e58ff537e3188131e441342bed0f491cbce461183c2cc94b507a01e2f406ae757e5ad8854495c415a87688cb69fbe6560af8e14fa31fc2dcd44b88080a09943ce5c17ce69fe140a111cc8436dd434034352c4dbefa7623d48593eae2987a013674f92f711a51bfd038a63208ef702497ccbf468405a9ad9378a57e03ab67ba005af7977817c87942f8cd44f542cf831671b8bc88763dc3fd3efb10c41497b02a079866ac4ff54c3062d8fbd4fa347961e9a905b4114a2ed9785e22a5c03f4ffb88080").to_vec()),
		BoundedVec::truncate_from(hex!("f851a007396135100eb53ae6524f6e1f2d88310e293d024e037c81a32ff956c390238c8080808080a0a5aa14d7843fafb099d9e4eb450962a221f8240f60a5d47ad1cc23510e90d20480808080808080808080").to_vec()),
		BoundedVec::truncate_from(hex!("f8518080808080808080a08e282b6ac30946def3a85aed7a185ba99f8dfdd3d50558fd473563e15c04fb7280a0f3c38f907a3fdb41207f6dfe465cd9d2aa30390cab355a8fe6de31add4633f07808080808080").to_vec()),
		BoundedVec::truncate_from(hex!("f8429f3c889a2b804c67887cd70e57ff036e6bc341281711f6587c117607d171d093a1a05774ba3f9618e2da3885b0e2853e4005c3e836625e8be0f69bf3d93f51fac58d").to_vec()),
	])
}

pub fn get_valid_message() -> Message {
	let data = &[
		Token::FixedBytes(
			H256(hex!(
				"4554480000000000000000000000000000000000000000000000000000000000"
			))
			.as_bytes()
			.to_vec(),
		),
		Token::Uint(U256::from(1u128)),
	];

	let encoded = encode(data);
	Message {
		message_type: MessageType::FungibleToken,
		from: H256(hex!(
			"681257BED628425a28B469114Dc21A7c30205cFD000000000000000000000000"
		)),
		to: H256(hex!(
			"3547517355657647456b6f7847444a5044576251694b4478714b6d675a357047"
		)),
		origin_domain: 2,
		destination_domain: 1,
		data: BoundedVec::truncate_from(encoded.to_vec()),
		id: 1,
	}
}

fn get_valid_amb_message() -> Message {
	let recipient = H256(hex!(
		"3547517355657647456b6f7847444a5044576251694b4478714b6d675a357047"
	));
	let encoded_data = BoundedVec::defensive_truncate_from("Hello, World!".as_bytes().to_vec());

	Message {
		message_type: MessageType::ArbitraryMessage,
		from: H256(hex!(
			"681257BED628425a28B469114Dc21A7c30205cFD000000000000000000000000"
		)),
		to: recipient,
		origin_domain: 2,
		destination_domain: 1,
		data: encoded_data,
		id: 0,
	}
}

#[test]
fn test_fulfill_step_call_proof_not_valid() {
	new_test_ext().execute_with(|| {
		let slot = 7634942;

		ConfigurationStorage::<Test>::set(Configuration {
			slots_per_period: 8192,
			finality_threshold: 461,
		});

		let result = Bridge::fulfill_call(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			STEP_FN_ID,
			get_valid_step_input(),
			get_valid_step_output(),
			get_invalid_proof(),
			slot,
		);

		assert_err!(result, Error::<Test>::VerificationFailed);
	});
}

#[test]
fn test_fulfill_step_call_not_valid_function_id() {
	new_test_ext().execute_with(|| {
		let slot = 7634942;

		ConfigurationStorage::<Test>::set(Configuration {
			slots_per_period: 8192,
			finality_threshold: 461,
		});
		let invalid_function_id: H256 = H256(hex!(
			"bf44af6890508b3b7f6910d4a4570a0d524769a23ce340b2c7400e140ad168ab"
		));
		let result = Bridge::fulfill_call(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			invalid_function_id,
			get_valid_step_input(),
			get_valid_step_output(),
			get_valid_step_proof(),
			slot,
		);

		assert_err!(result, Error::<Test>::FunctionIdNotKnown);
	});
}

#[test]
fn test_fulfill_step_call_finality_not_met() {
	new_test_ext().execute_with(|| {
		let slot = 7634942;
		SyncCommitteePoseidons::<Test>::insert(
			931,
			U256::from(hex!(
				"0ab2afdc05c8b6ae1f2ab20874fb4159e25d5c1d4faa41aee232d6ab331332df"
			)),
		);

		ConfigurationStorage::<Test>::set(Configuration {
			slots_per_period: 8192,
			finality_threshold: 512, // max finality
		});
		let result = Bridge::fulfill_call(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			STEP_FN_ID,
			get_valid_step_input(),
			get_valid_step_output(),
			get_valid_step_proof(),
			slot,
		);

		assert_err!(result, Error::<Test>::NotEnoughParticipants);
	});
}

// TODO add another test when the valid proof is available where it checks that amount was successfully sent
#[test]
fn test_execute_message_via_storage() {
	new_test_ext().execute_with(|| {
		let balance_before = Balances::balance(&Bridge::account_id());
		Broadcasters::<Test>::set(
			2,
			H256(hex!(
				"8F8d47bF15953E26c622F36F3366e43e26B9b78b000000000000000000000000"
			)),
		);

		let slot = 8581263;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"0dd1e0bfab60f2c887dab9d9d1a9d071a44784bc92e4a32e0eeded838cdada4b"
			)),
		);

		let account_proof = get_valid_account_proof();
		let storage_proof = get_valid_storage_proof();
		let message = get_valid_message();
		// amount in message 1000000000000000000
		let err = Bridge::execute(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			slot,
			message,
			account_proof,
			storage_proof,
		);
		assert_err!(err, Error::<Test>::AssetNotSupported);

		let balance_left = Balances::balance(&Bridge::account_id());
		assert_eq!(balance_before, balance_left)
	});
}

#[test]
fn test_execute_arb_message_invalid_hash() {
	new_test_ext().execute_with(|| {
		let balance_before = Balances::balance(&Bridge::account_id());
		Broadcasters::<Test>::set(
			2,
			H256(hex!(
				"8F8d47bF15953E26c622F36F3366e43e26B9b78b000000000000000000000000"
			)),
		);

		let slot = 8581263;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"18fff1775245ec2b37fdf098c5f576d3e582a4275909d8899735e0eb2d7670a3"
			)),
		);

		let account_proof = get_valid_amb_account_proof();
		let storage_proof = get_valid_amb_storage_proof();
		let mut message = get_valid_amb_message();

		// change message type
		message.message_type = MessageType::FungibleToken;

		let err = Bridge::execute(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			slot,
			message,
			account_proof,
			storage_proof,
		);
		let balance_left = Balances::balance(&Bridge::account_id());
		assert_eq!(balance_before, balance_left);
		assert_err!(err, Error::<Test>::InvalidMessageHash)
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

#[test]
fn test_execute_message_with_faulty_account_proof() {
	new_test_ext().execute_with(|| {
		Broadcasters::<Test>::set(
			2,
			H256(hex!(
				"426bde66abd85741be832b824ea65a3aad70113e000000000000000000000000"
			)),
		);

		let slot = 8581263;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"d6b8a2fb20ade94a56d9d87a07ca11e46cc169ed43dc0d2527a0d3ca2309ba9c"
			)),
		);

		let account_proof = get_invalid_account_proof();
		let storage_proof = get_valid_storage_proof();
		let message = get_valid_message();

		let fail = Bridge::execute(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			slot,
			message,
			account_proof,
			storage_proof,
		);

		// invalid proof should return error
		assert_err!(fail, Error::<Test>::CannotGetStorageRoot);
	});
}

#[test]
fn test_execute_message_with_faulty_storage_proof() {
	new_test_ext().execute_with(|| {
		Broadcasters::<Test>::set(
			2,
			H256(hex!(
				"8F8d47bF15953E26c622F36F3366e43e26B9b78b000000000000000000000000"
			)),
		);

		let slot = 8581263;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"0dd1e0bfab60f2c887dab9d9d1a9d071a44784bc92e4a32e0eeded838cdada4b"
			)),
		);

		let account_proof = get_valid_account_proof();
		let storage_proof = get_invalid_storage_proof();
		let message = get_valid_message();

		let fail = Bridge::execute(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			slot,
			message,
			account_proof,
			storage_proof,
		);

		// invalid storage proof should return error
		assert_err!(fail, Error::<Test>::CannotGetStorageValue);
	});
}

//
#[test]
fn test_execute_message_with_already_executed_message() {
	new_test_ext().execute_with(|| {
		Broadcasters::<Test>::set(
			2,
			H256(hex!(
				"426bde66abd85741be832b824ea65a3aad70113e000000000000000000000000"
			)),
		);

		let slot = 8581263;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"d6b8a2fb20ade94a56d9d87a07ca11e46cc169ed43dc0d2527a0d3ca2309ba9c"
			)),
		);

		Timestamps::<Test>::set(slot, 1701327753);
		let message = get_valid_message();
		let abi_encoded = message.clone().abi_encode();
		let message_root = keccak_256(abi_encoded.as_slice());
		MessageStatus::<Test>::set(H256(message_root), MessageStatusEnum::ExecutionSucceeded);

		let account_proof = get_valid_account_proof();
		let storage_proof = get_valid_storage_proof();

		let fail = Bridge::execute(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			slot,
			message,
			account_proof,
			storage_proof,
		);

		assert_err!(fail, Error::<Test>::MessageAlreadyExecuted);
	});
}

#[test]
fn test_execute_message_with_unsupported_domain() {
	new_test_ext().execute_with(|| {
		Broadcasters::<Test>::set(
			2,
			H256(hex!(
				"426bde66abd85741be832b824ea65a3aad70113e000000000000000000000000"
			)),
		);

		let slot = 8581263;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"d6b8a2fb20ade94a56d9d87a07ca11e46cc169ed43dc0d2527a0d3ca2309ba9c"
			)),
		);

		Timestamps::<Test>::set(slot, 1701327753);
		let mut message = get_valid_message();
		// alter
		message.origin_domain = 4;

		let account_proof = get_valid_account_proof();
		let storage_proof = get_valid_storage_proof();

		let fail = Bridge::execute(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			slot,
			message,
			account_proof,
			storage_proof,
		);

		assert_err!(fail, Error::<Test>::UnsupportedOriginChain);
	});
}

#[test]
fn test_fulfill_step_call() {
	new_test_ext().execute_with(|| {
		let slot = 7634942;

		SyncCommitteePoseidons::<Test>::insert(
			931,
			U256::from(hex!(
				"0ab2afdc05c8b6ae1f2ab20874fb4159e25d5c1d4faa41aee232d6ab331332df"
			)),
		);

		ConfigurationStorage::<Test>::set(Configuration {
			slots_per_period: 8192,
			finality_threshold: 461,
		});

		let result = Bridge::fulfill_call(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			STEP_FN_ID,
			get_valid_step_input(),
			get_valid_step_output(),
			get_valid_step_proof(),
			slot,
		);

		assert_ok!(result);
		let finalized_slot = 7634848;
		// ensure that event is fired
		let expected_event = RuntimeEvent::Bridge(Event::HeaderUpdate {
			slot: finalized_slot,
			finalization_root: H256(hex!(
				"e4566e0cf4edb171a3eedd59f9943bbcd0b1f6b648f1a6e26d5264b668ab41ec"
			)),
			execution_state_root: H256(hex!(
				"51e76629b32b943497207e7b7ccff8fbc12e9e6d758cc7eed972422c4cad02b9"
			)),
		});

		let finalized_slot = 7634848;

		let header = Headers::<Test>::get(finalized_slot);
		let head = Head::<Test>::get();
		let ex_state_root = ExecutionStateRoots::<Test>::get(finalized_slot);

		assert_eq!(
			header,
			H256(hex!(
				"e4566e0cf4edb171a3eedd59f9943bbcd0b1f6b648f1a6e26d5264b668ab41ec"
			))
		);
		assert_eq!(
			ex_state_root,
			H256(hex!(
				"51e76629b32b943497207e7b7ccff8fbc12e9e6d758cc7eed972422c4cad02b9"
			))
		);
		assert_eq!(head, finalized_slot);
		assert_eq!(expected_event, System::events()[0].event);
	});
}

#[test]
fn test_fulfill_step_call_slot_behind_head() {
	new_test_ext().execute_with(|| {
		let slot = 7634942;
		SyncCommitteePoseidons::<Test>::insert(
			931,
			U256::from(hex!(
				"0ab2afdc05c8b6ae1f2ab20874fb4159e25d5c1d4faa41aee232d6ab331332df"
			)),
		);

		// move head forward
		Head::<Test>::set(8634942);

		ConfigurationStorage::<Test>::set(Configuration {
			slots_per_period: 8192,
			finality_threshold: 461,
		});

		let result = Bridge::fulfill_call(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			STEP_FN_ID,
			get_valid_step_input(),
			get_valid_step_output(),
			get_valid_step_proof(),
			slot,
		);

		assert_err!(result, Error::<Test>::SlotBehindHead);
	});
}

#[test]
fn test_fulfill_rotate_call() {
	new_test_ext().execute_with(|| {
		let slot = 7634942;

		ConfigurationStorage::<Test>::set(Configuration {
			slots_per_period: 8192,
			finality_threshold: 342,
		});

		Headers::<Test>::set(
			slot,
			H256(hex!(
				"e882fe800bed07205bf2cbf17f30148b335d143a91811ff65280c221c9f57856"
			)),
		);

		let result = Bridge::fulfill_call(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			ROTATE_FN_ID,
			get_valid_rotate_input(),
			get_valid_rotate_output(),
			get_valid_rotate_proof(),
			slot,
		);

		assert_ok!(result);
		// ensure that event is fired
		let expected_poseidon = U256::from_dec_str(
			"16399439943012933445970260519503780180385945954293268151243539801891563949197",
		)
		.unwrap();

		let current_period = 931;
		let expected_event = RuntimeEvent::Bridge(Event::SyncCommitteeUpdate {
			period: current_period,
			root: expected_poseidon,
		});

		let poseidon = SyncCommitteePoseidons::<Test>::get(current_period + 1);

		assert_eq!(expected_event, System::events()[0].event);
		assert_eq!(poseidon, expected_poseidon);
	});
}

#[test]
fn test_fulfill_rotate_call_domain_disabled() {
	new_test_ext().execute_with(|| {
		let slot = 7634942;

		SourceChainFrozen::<Test>::set(2, true);

		let result = Bridge::fulfill_call(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			ROTATE_FN_ID,
			get_valid_rotate_input(),
			get_valid_rotate_output(),
			get_valid_rotate_proof(),
			slot,
		);
		assert_err!(result, Error::<Test>::SourceChainFrozen)
	});
}

#[test]
fn set_whitelisted_domains_works_with_root() {
	new_test_ext().execute_with(|| {
		let domains = BoundedVec::try_from([0, 1, 2, 3].to_vec()).unwrap();
		assert_ne!(WhitelistedDomains::<Test>::get(), domains);

		let ok = Bridge::set_whitelisted_domains(RawOrigin::Root.into(), domains.clone());
		assert_ok!(ok);
		assert_eq!(WhitelistedDomains::<Test>::get(), domains);

		let expected_event = RuntimeEvent::Bridge(Event::WhitelistedDomainsUpdated);
		System::assert_last_event(expected_event);
	});
}

#[test]
fn set_whitelisted_domains_does_not_work_with_non_root() {
	new_test_ext().execute_with(|| {
		let domains = BoundedVec::try_from([0, 1, 2, 3].to_vec()).unwrap();
		let origin = RuntimeOrigin::signed(TEST_SENDER_VEC.into());
		let ok = Bridge::set_whitelisted_domains(origin, domains.clone());
		assert_err!(ok, BadOrigin);
	});
}

#[test]
fn set_configuration_works_with_root() {
	new_test_ext().execute_with(|| {
		let conf = Configuration {
			slots_per_period: 1,
			finality_threshold: 69,
		};
		assert_ne!(ConfigurationStorage::<Test>::get(), conf);

		let ok = Bridge::set_configuration(RawOrigin::Root.into(), conf.clone());
		assert_ok!(ok);
		assert_eq!(ConfigurationStorage::<Test>::get(), conf);

		let expected_event = RuntimeEvent::Bridge(Event::ConfigurationUpdated {
			slots_per_period: conf.slots_per_period,
			finality_threshold: conf.finality_threshold,
		});
		System::assert_last_event(expected_event);
	});
}

#[test]
fn set_configuration_does_not_work_with_non_root() {
	new_test_ext().execute_with(|| {
		let conf = Configuration {
			slots_per_period: 1,
			finality_threshold: 69,
		};

		let origin = RuntimeOrigin::signed(TEST_SENDER_VEC.into());
		let ok = Bridge::set_configuration(origin, conf);
		assert_err!(ok, BadOrigin);
	});
}

#[test]
fn set_broadcaster_works_with_root() {
	new_test_ext().execute_with(|| {
		let domain = 2;
		let old = Broadcasters::<Test>::get(domain);
		assert_ne!(old, STEP_FN_ID);

		let ok = Bridge::set_broadcaster(RawOrigin::Root.into(), domain, STEP_FN_ID);
		assert_ok!(ok);
		assert_eq!(Broadcasters::<Test>::get(domain), STEP_FN_ID);

		let expected_event = RuntimeEvent::Bridge(Event::BroadcasterUpdate {
			old,
			new: STEP_FN_ID,
			domain,
		});
		System::assert_last_event(expected_event);
	});
}

#[test]
fn set_broadcaster_does_not_work_with_non_root() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::signed(TEST_SENDER_VEC.into());
		let ok = Bridge::set_broadcaster(origin, 2, STEP_FN_ID);
		assert_err!(ok, BadOrigin);
	});
}

#[test]
fn set_poseidon_hash_works_with_root() {
	new_test_ext().execute_with(|| {
		let period = 2;
		let poseidon_hash = BoundedVec::try_from([0, 1, 2, 3, 4].to_vec()).unwrap();
		let root = U256::from(16909060u128);
		assert_ne!(SyncCommitteePoseidons::<Test>::get(period), root);

		let ok = Bridge::set_poseidon_hash(RawOrigin::Root.into(), period, poseidon_hash);
		assert_ok!(ok);
		assert_eq!(SyncCommitteePoseidons::<Test>::get(period), root);

		let expected_event = RuntimeEvent::Bridge(Event::SyncCommitteeUpdate { period, root });
		System::assert_last_event(expected_event);
	});
}

#[test]
fn set_poseidon_hash_does_not_work_with_non_root() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::signed(TEST_SENDER_VEC.into());
		let root = BoundedVec::try_from([0, 1, 2, 3, 4].to_vec()).unwrap();

		let ok = Bridge::set_poseidon_hash(origin, 2, root);
		assert_err!(ok, BadOrigin);
	});
}

#[test]
fn source_chain_froze_works_with_root() {
	new_test_ext().execute_with(|| {
		let source_chain_id = 2;
		let frozen = true;
		assert_ne!(SourceChainFrozen::<Test>::get(source_chain_id), frozen);

		let ok = Bridge::source_chain_froze(RawOrigin::Root.into(), source_chain_id, frozen);
		assert_ok!(ok);
		assert_eq!(SourceChainFrozen::<Test>::get(source_chain_id), frozen);

		let expected_event = RuntimeEvent::Bridge(Event::SourceChainFrozen {
			source_chain_id,
			frozen,
		});
		System::assert_last_event(expected_event);
	});
}

#[test]
fn source_chain_froze_does_not_work_with_non_root() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::signed(TEST_SENDER_VEC.into());

		let ok = Bridge::source_chain_froze(origin, 2, true);
		assert_err!(ok, BadOrigin);
	});
}

#[test]
fn send_message_arbitrary_message_works() {
	new_test_ext().execute_with(|| {
		let origin = RuntimeOrigin::signed(TEST_SENDER_VEC.into());
		let message_type = MessageType::ArbitraryMessage;
		let to = ROTATE_FN_ID;
		let domain = 2;
		let data = Some(BoundedVec::try_from([0, 1, 2, 3].to_vec()).unwrap());

		let ok = Bridge::send_message(
			origin,
			message_type.clone(),
			to,
			domain,
			None,
			None,
			data.clone(),
		);
		assert_ok!(ok);

		let expected_event = RuntimeEvent::Bridge(Event::MessageSubmitted {
			from: TEST_SENDER_VEC.into(),
			to,
			message_type,
			destination_domain: domain,
		});
		System::assert_last_event(expected_event);
	});
}

#[test]
fn send_message_arbitrary_message_doesnt_accept_value() {
	new_test_ext().execute_with(|| {
		use crate::Error;

		let origin = RuntimeOrigin::signed(TEST_SENDER_VEC.into());
		let data = Some(BoundedVec::try_from([0, 1, 2, 3].to_vec()).unwrap());

		let ok = Bridge::send_message(
			origin,
			MessageType::ArbitraryMessage,
			ROTATE_FN_ID,
			2,
			Some(100u128),
			None,
			data,
		);
		assert_err!(ok, Error::<Test>::InvalidBridgeInputs);
	});
}

#[test]
fn send_message_arbitrary_message_doesnt_accept_asset_id() {
	new_test_ext().execute_with(|| {
		use crate::Error;

		let origin = RuntimeOrigin::signed(TEST_SENDER_VEC.into());
		let data = Some(BoundedVec::try_from([0, 1, 2, 3].to_vec()).unwrap());

		let ok = Bridge::send_message(
			origin,
			MessageType::ArbitraryMessage,
			ROTATE_FN_ID,
			2,
			None,
			Some(ROTATE_FN_ID),
			data,
		);
		assert_err!(ok, Error::<Test>::InvalidBridgeInputs);
	});
}

#[test]
fn send_message_arbitrary_message_doesnt_accept_empty_data() {
	new_test_ext().execute_with(|| {
		use crate::Error;

		let origin = RuntimeOrigin::signed(TEST_SENDER_VEC.into());

		let ok = Bridge::send_message(
			origin,
			MessageType::ArbitraryMessage,
			ROTATE_FN_ID,
			2,
			None,
			None,
			None,
		);
		assert_err!(ok, Error::<Test>::InvalidBridgeInputs);
	});
}

#[test]
fn send_message_fungible_token_works() {
	new_test_ext().execute_with(|| {
		use crate::BalanceOf;
		use frame_support::traits::Currency;

		let origin = RuntimeOrigin::signed(TEST_SENDER_VEC.into());
		let message_type = MessageType::FungibleToken;
		let to = ROTATE_FN_ID;
		let domain = 2;

		Balances::make_free_balance_be(
			&TEST_SENDER_VEC.into(),
			BalanceOf::<Test>::max_value() / 2u128,
		);

		let ok = Bridge::send_message(
			origin,
			message_type.clone(),
			to,
			domain,
			Some(100u128),
			Some(ROTATE_FN_ID),
			None,
		);
		assert_ok!(ok);

		let expected_event = RuntimeEvent::Bridge(Event::MessageSubmitted {
			from: TEST_SENDER_VEC.into(),
			to,
			message_type,
			destination_domain: domain,
		});
		System::assert_last_event(expected_event);
	});
}

#[test]
fn send_message_fungible_token_doesnt_accept_data() {
	new_test_ext().execute_with(|| {
		use crate::Error;

		let origin = RuntimeOrigin::signed(TEST_SENDER_VEC.into());
		let data = Some(BoundedVec::try_from([0, 1, 2, 3].to_vec()).unwrap());

		let ok = Bridge::send_message(
			origin,
			MessageType::FungibleToken,
			ROTATE_FN_ID,
			2,
			Some(100u128),
			Some(ROTATE_FN_ID),
			data,
		);
		assert_err!(ok, Error::<Test>::InvalidBridgeInputs);
	});
}

#[test]
fn send_message_fungible_token_doesnt_accept_empty_asset_id() {
	new_test_ext().execute_with(|| {
		use crate::Error;

		let origin = RuntimeOrigin::signed(TEST_SENDER_VEC.into());

		let ok = Bridge::send_message(
			origin,
			MessageType::FungibleToken,
			ROTATE_FN_ID,
			2,
			Some(100u128),
			None,
			None,
		);
		assert_err!(ok, Error::<Test>::InvalidBridgeInputs);
	});
}

#[test]
fn send_message_fungible_token_doesnt_accept_empty_value() {
	new_test_ext().execute_with(|| {
		use crate::Error;

		let origin = RuntimeOrigin::signed(TEST_SENDER_VEC.into());

		let ok = Bridge::send_message(
			origin,
			MessageType::FungibleToken,
			ROTATE_FN_ID,
			2,
			None,
			Some(ROTATE_FN_ID),
			None,
		);
		assert_err!(ok, Error::<Test>::InvalidBridgeInputs);
	});
}

#[test]
fn execute_arbitary_message_works() {
	new_test_ext().execute_with(|| {
		use crate::BalanceOf;
		use frame_support::traits::Currency;

		let origin = RuntimeOrigin::signed(TEST_SENDER_VEC.into());
		Balances::make_free_balance_be(
			&TEST_SENDER_VEC.into(),
			BalanceOf::<Test>::max_value() / 2u128,
		);

		Broadcasters::<Test>::set(
			2,
			H256(hex!(
				"8F8d47bF15953E26c622F36F3366e43e26B9b78b000000000000000000000000"
			)),
		);

		let slot = 5085118;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"18fff1775245ec2b37fdf098c5f576d3e582a4275909d8899735e0eb2d7670a3"
			)),
		);

		let message = get_valid_amb_message();

		let account_proof = get_valid_amb_account_proof();
		let storage_proof = get_valid_amb_storage_proof();

		let ok = Bridge::execute(
			origin,
			slot,
			message.clone(),
			account_proof.clone(),
			storage_proof.clone(),
		);
		assert_ok!(ok);

		let encoded_data = message.clone().abi_encode();
		let message_root = H256(keccak_256(encoded_data.as_slice()));

		let expected_event = RuntimeEvent::Bridge(Event::ExecutedMessage {
			message,
			message_root,
		});
		System::assert_last_event(expected_event);
	});
}

#[test]
fn test_double_execute_arbitary_message() {
	new_test_ext().execute_with(|| {
		use crate::BalanceOf;
		use frame_support::traits::Currency;

		let origin1 = RuntimeOrigin::signed(TEST_SENDER_VEC.into());
		let origin2 = RuntimeOrigin::signed(TEST_SENDER_VEC.into());
		Balances::make_free_balance_be(
			&TEST_SENDER_VEC.into(),
			BalanceOf::<Test>::max_value() / 2u128,
		);

		Broadcasters::<Test>::set(
			2,
			H256(hex!(
				"8F8d47bF15953E26c622F36F3366e43e26B9b78b000000000000000000000000"
			)),
		);

		let slot = 5085118;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"18fff1775245ec2b37fdf098c5f576d3e582a4275909d8899735e0eb2d7670a3"
			)),
		);

		let message = get_valid_amb_message();

		let account_proof = get_valid_amb_account_proof();
		let storage_proof = get_valid_amb_storage_proof();

		let ok = Bridge::execute(
			origin1,
			slot,
			message.clone(),
			account_proof.clone(),
			storage_proof.clone(),
		);
		assert_ok!(ok);

		let err = Bridge::execute(
			origin2,
			slot,
			message.clone(),
			account_proof.clone(),
			storage_proof.clone(),
		);
		assert_err!(err, Error::<Test>::MessageAlreadyExecuted);

		let encoded_data = message.clone().abi_encode();
		let message_root = H256(keccak_256(encoded_data.as_slice()));

		let expected_event = RuntimeEvent::Bridge(Event::ExecutedMessage {
			message,
			message_root,
		});
		System::assert_last_event(expected_event);
	});
}
