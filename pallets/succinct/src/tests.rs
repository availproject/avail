use ethabi::{encode, Token};
use frame_support::traits::fungible::Inspect;
use frame_support::{assert_err, assert_ok, BoundedVec};
use frame_system::submitted_data::MessageType::ArbitraryMessage;
use frame_system::submitted_data::{Message, MessageType};
use hex_literal::hex;
use primitive_types::U256;
use sp_core::crypto::AccountId32;
use sp_core::keccak_256;
use sp_runtime::testing::H256;

use crate::mock::{new_test_ext, Bridge, RuntimeEvent, RuntimeOrigin, Test};
use crate::mock::{Balances, System};
use crate::state::State;
use crate::target_amb::MessageStatusEnum;
use crate::{
	Broadcasters, Error, Event, ExecutionStateRoots, Head, Headers, InputMaxLen,
	MessageBytesMaxLen, MessageItemsMaxLen, MessageStatus, OutputMaxLen, ProofMaxLen,
	RotateVerificationKeyStorage, SourceChainFrozen, StateStorage, StepVerificationKeyStorage,
	SyncCommitteePoseidons, Timestamps, VerificationKeyDef,
};

const TEST_SENDER_VEC: [u8; 32] = [2u8; 32];
const TEST_SENDER_ACCOUNT: AccountId32 = AccountId32::new(TEST_SENDER_VEC);
const STEP_FN_ID: H256 = H256(hex!(
	"af44af6890508b3b7f6910d4a4570a0d524769a23ce340b2c7400e140ad168ab"
));

const ROTATE_FN_ID: H256 = H256(hex!(
	"9c1096d800fc42454d2d76e6ae1d461b5a67c7b474efb9d47989e47ed39b1b7b"
));

fn get_valid_step_input() -> BoundedVec<u8, InputMaxLen> {
	BoundedVec::truncate_from(
		hex!("0ab2afdc05c8b6ae1f2ab20874fb4159e25d5c1d4faa41aee232d6ab331332df0000000000747ffe")
			.to_vec(),
	)
}

fn get_valid_step_output() -> BoundedVec<u8, OutputMaxLen> {
	BoundedVec::truncate_from(hex!("e4566e0cf4edb171a3eedd59f9943bbcd0b1f6b648f1a6e26d5264b668ab41ec51e76629b32b943497207e7b7ccff8fbc12e9e6d758cc7eed972422c4cad02b90000000000747fa001fd").to_vec())
}

fn get_valid_step_proof() -> BoundedVec<u8, ProofMaxLen> {
	BoundedVec::truncate_from(hex!("0b496d04c0e12206bc846edd2077a20b8b55f65fc0e40bb8cf617d9b79ce39e508281ad49432300b3b7c8a95a0a63544f93f553fcfdeba38c82460888f4030ed1f67a1be666c12ee00658109c802042c58f645474fcee7d128277a4e35c1dd1504d33cb652ec23407cd3580eda0196dd97054eb5c2a817163d6997832d9abd422729b3e85a15941722baeb5ca8a42567a91c6a0b0cd64ac15431fde05071e90e0d30c12013d5803336cc2f433c16eaa5434e30b89ce7395c3c3cda29dde3be062281095f143d728486c71203b24fa6068e69aabf29d457ffadc6d682d51a4f08179d3240bc561ae7e2c005bb772a4d4c5ba6644986052fad554f042ab0074a8f").to_vec())
}

fn get_valid_rotate_input() -> BoundedVec<u8, InputMaxLen> {
	BoundedVec::truncate_from(
		hex!("e882fe800bed07205bf2cbf17f30148b335d143a91811ff65280c221c9f57856").to_vec(),
	)
}

fn get_valid_rotate_output() -> BoundedVec<u8, OutputMaxLen> {
	BoundedVec::truncate_from(
		hex!("2441c10b0b6605985c56ebf6dc1ca7e9a0ae20e617c931d72f2ec19aa40ccc8d").to_vec(),
	)
}

fn get_valid_rotate_proof() -> BoundedVec<u8, ProofMaxLen> {
	BoundedVec::truncate_from(hex!("14305744fb26a377656a947cae0874c14b086de9d407bdfaf415ca9f47402c04144589183b473537750e7211f93671e324825db673edcf5c0839b08eecba08202966ba52dc07e1bf9832a54770048b84999172d47c57628758d8fe43dd9fe1412e6f8c0e75a79cde28e0e24eb09f9d23309defb07f4a1761deb6598de77278971d2d914930ad2e3ad8b6264e595a0516a912fc9394c93fa61146efc54d61e5c32378a5d4460aa2164422702f9401fcfb3e2b991a0e5b847ede3ea9ffe70a55100203abc0636c101adb6546c2f7aaf32d79e69093afb40c3c1a674e44a1ece76a1183fc03ef9553a7728672de2aada5d5582b5bcf0859e8c312ab59429553ed6d").to_vec())
}

fn get_invalid_proof() -> BoundedVec<u8, ProofMaxLen> {
	BoundedVec::truncate_from(hex!("1b496d04c0e12206bc846edd2077a20b8b55f65fc0e40bb8cf617d9b79ce39e508281ad49432300b3b7c8a95a0a63544f93f553fcfdeba38c82460888f4030ed1f67a1be666c12ee00658109c802042c58f645474fcee7d128277a4e35c1dd1504d33cb652ec23407cd3580eda0196dd97054eb5c2a817163d6997832d9abd422729b3e85a15941722baeb5ca8a42567a91c6a0b0cd64ac15431fde05071e90e0d30c12013d5803336cc2f433c16eaa5434e30b89ce7395c3c3cda29dde3be062281095f143d728486c71203b24fa6068e69aabf29d457ffadc6d682d51a4f08179d3240bc561ae7e2c005bb772a4d4c5ba6644986052fad554f042ab0074a8f").to_vec())
}

fn get_valid_account_proof() -> BoundedVec<BoundedVec<u8, MessageBytesMaxLen>, MessageItemsMaxLen> {
	BoundedVec::truncate_from(vec![
        BoundedVec::truncate_from(hex!("f90211a00089429375db917315fb4b8d67055bdf76e13d11292801af4a4a151f5760ff7aa02ebce9bb13a075ff89c5aae6b67f4d457525c53dfcc016ce72ea17e0e15a3718a04201c7d41a78f6906183b252fecbb231305d4e22c7e5b729b95a5a6ac53f4d46a06b61a1f5e208c3babf5fc1c9c4180af47769ec421c2c3125f313b5394014fa8aa0b2f35b0e2a84ce9e685b3e9558a0495552c80baec0bd687092220314850f543ba0244dca6d79c72abe8e3a12d49f2cf1976ee7bef58c5c6eb9ff6708fa138abfcca005631aa85658a9962bfee9a4827df5ca6f5461c4bc533591c897a66421f9abbfa0478ef142f553c91d672d865bed8d5175ebbbfc72be010d23b8d81cdcb41247e0a0365a9b70e7c6d82d3246b130bc27453ba77f0bcb4301d43c719eae676a7e0d17a001768b342f6cbc790d57276817d0853c94a682e295930951059bd1c24352b46ea0e3d9b775f71b4c1b2a0c35b1e492b0f2c6ce66c94cf2c8320276fe5cd5e427c8a03bd4160a5626c0d56a4435cb13b6cd3adb5f93793b71148cafa16e07f554fa41a052ab349de3157030b412abdd7353ee1d6476c09c153ddb1dba487294f11a5c7ca0ab71e81c1fc9e656fa8f0df6ee16efa5f105acce3c43ef172a04534f00e5d25ea05306a9ed38acb653787765466a764d4c8748c29b4e7a9ad4a75c61c0840b4a17a0699307b9c473f45858fec9fecd034fa0b3427c0efdd02d407c03201dcdaca02380").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a0f7c14d7714348be36359dd28afd64e2fb72679a7ae09a27027fc95e335bcde1ca0824329840722c728e0f19ae424caad4581ac42015a4ab8e9d3ea550c857da804a040d48c9df564c00b11d304e2a2597a35b17b25429c3850c4e3fe4e9a278bec88a0a497297590785cfaa8491579745c077b1095348912d4e3288d8f00857ed9db5da0b0ea3abfcdab8c6cf03152cc7a57f602f85d86f4bdb3d1ca2242a5e737561bbda06bbe0e0416b59f1c4cba36afdee766ea4689f1c1ac8e2245f45c2631e2478119a0222dec72b36685a0ca89e49ce87262957f7f891e695ea8ec52e25fbc3a328589a00b3cac878feb2bcd5fc3d49fe5f607eabf75f014df74a268d4aaa1d25654d030a000deffa5e2879748ef9a634a3573484b4dd259c0d4c10453a7e1e3504b56322ea05c356b24b3b36089583f650cb954f884b05275b09b7715a2eb3cf6fa9175738ea093abf2b2cb15649c192d0d79f62627ce634843f84ec98eee99267c1354b5135aa059e9c60388154b3b810ffd41f81ed9128c8091a12e0c53062d9e7430fedf5939a06855c9a5622a40b5bce572522e4774986c7061557d2f1b8f7070d8d397888b4ea04d220a5fb22e38d64cdf4b46a42898b9f1ce9f316f1d332eebebd32c0cc59000a09004930139d4ae94070b29245230d5b28b25ac59c11339928a2eb547f0828341a00f37af44fb487a5ed675e12f0566a54e59cc025466e91cf56dcf348ff4049ed980").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a0e9fa1abfa1f1d84a27da9448b42e3c0f5c60c54a1e8cb90c9e28b60824157380a05e977e1d37e502ac74fd54a2debf7e9b7b6e64c261e45e9b0610bcc201ddbe93a02f8a351ea5204d62c85fe6b564eab729fd556b1941a4f83f6f4b6e40e4102869a0a4b62da8ab84fcd0cf425fba4fd03ad7f1350217679e105e57ee146f64b07e07a061049f894647148c39ec3d8c4563d22670ee697f2e4a003513595f5074fe0166a0de1551dd310c9206da56ff9288dc518cccf7cdfa259cc3ff0318a6f3f7539988a00e600d8cb072056fbf1f5bf7d18aec2eb2ba57e93b5e6bb3f0d36042ec8fbe9ba0fa02eb32060ca2e3fd46e39a8456f02156b8efb457c74ccab5789bce1d142613a0919bb37876273e3283660eb2c575ddcfa99239ab79cf7edaf64d5591689c7777a052a8ee269c13ef214ba56ff0ef6b3cb11da6b12ddadbf1883831e91c6768bf60a0028fdfd852916e9cfa13eee9bf6c540bdc7f6d9b18eee15e11da66a8cdfc933ba09d581d74aa42d7974e122d3a3ec6febaa74ca9f714ddf5c52a5bfa9ee41471e5a0c5608d4aef23664aaaa38aa2425cf959b62d30cf22a0d14147a3cab3d4178fc3a0beb1d967ae4415f30d7730c1bfd43446e24c5f0210cb3a0f5a9bc67e9f63228ea03117ae91a22815aac4b1c9210ba7a6682697a73cd68d7741d693c1cbd1925063a032cf653822d7a618300ef2113f0ff0be132dda944106f25350b5f37451c740a280").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a0f284a2e627542f07910ea0cb276b0b7813f3b9c056aafe496b3e7f93d1b3aa67a0d45d246efac9fb2e0c8052354aa0eebd68a28e9606efbbd4a5c2f9e990dc4d3ea0fd5d8349c16fda7a90a9c778cc74126188887aeacec8761349e1863d4008602fa022796160a8b1259fca46b22aa863131e970b077a449a5be4c486c9384335826da0b28076746e56b0bc37fb7586e2c4f23b624523d8e2f7abdffa73859cd531c12da08af556fb72bb802fde89a5562659959ef83a7846f0ced10ed6e139b44016bae9a0f948d4f88be556c183e053c131cd62aa278bcc83845437bfc03721828a3e2082a038c90f875a89a76b5b42d7c843ee790b759e482570a0bcb4d291496a40815093a031b88038ca3cd315ba56e783d4423c7c306cd9567f5a9eca972ac631c4c58e83a0858cbce5374ea0469281ee65c5a1aa5cfa19e7f7df02635821be244a5d39a38ea00cefc972ac8009f230bd9c8015753e98072b5f71d3a09093309ac6f09002f420a0e5fb8ae4800ad431a827003be4d719efcc29424f3ad2fbe483a42ab724a8610ea01a584c371a17ffc56a7713b2c6bb65bbcbf63c9d6382e0423dd577031c63842da0104f13e37d23eed61ebe6b78ee93ee9c30c3a92dab0ccbc57715051e9744eb58a0b211502efd34235ac7f948856c809f8aaf5e299df97ff24d4fb0d53caa3d1e83a043d845df46ad73ae3a9f2bfa319c19e7f760922f1268d8b96f0a54cb8ae88ab880").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a071241195c881f3437ebd19a9eccd009595c10537df66917a8fab0eb664f834dda0122c775309b9cff05db80ba77a60604d0fcb8a836a5e79999943f0d150297e19a0c32190d1506259a9ffa2ec1fbff6b23bd35d4e6bcb063b19a22ec10b914981f4a022a77ca63522f76d016d04e680d4c27c3ceee14bc4548f9e08c2cc10f9e1b789a0c646ec46e8f8d5fb7de785fe967200994afec4c48b2bcb001b5aed20db936326a0e20c61d63a3ac612051c43ed1acce68e185a08154e5f44e8eceebac0f454202da05b17a5f4ba7ed711f694536b96a69549fe097ba32dee1f9c71eb19a0533d46baa04da0bc8c8f03ad8f1efdf0da738f24c9ec4549acc71d43ae6607f22601ac4f38a08ea8a34e48a70ccac672eaa2c3a4538d61d38cb5a143a4596d571904b6e3181ea0148252504cc36b4f6b1ef7183df2ce176963bacfc97ad3949fcb6da7d4095821a03d63131beaa2c1137d599528084b0aeb4bea87ee8da16f424dd93c3b90087a75a059f94b55179b81bb657f5021b161ab30fffc8620706a858de7103a0da99a262ea0bb62efd30271c9e2bfc8a4938ebcf4d90623d1d55ffb97399f6456c597599464a024a60032c223c88b91e1fc98db296e58468ebf38eed7bdab0e114cdd754bdc80a0271ec93cc3efaacce706f26a3aa42d6f7c9d8fd6944329149ad63b43c78aae34a07caa42499d46895c9b948f37479c6572573db5b644a0862168e25e4e3bfdb57e80").to_vec()),
        BoundedVec::truncate_from(hex!("f9015180a09089f0d1272f06751d391dfbc7b6d49b39731b8a14b5e5e97d45e34d89df0f3fa0820bbc641b62cf0f6a4c3836017cdef0bf7f43c1ee8cbc76ce7b5dcd80f58b9480a0fbe1f0ac8158473558c7b9964cc295027449f6e960f5f6407d9ca1c9ef15f7bca0a2fb890c487021019f73371bf6798e8db8b612ca3c7b30fc3495441a1f9518c4a02cd1ca2531caa6e63ac5f16e5ea76018826683f10442ab5c2b1f9963f23b011ca0429bcf37f564e67dd5764f96fa79532113668cbb32059affdfdc82cfdfd5d1e18080a09be000de088393ee33eac568ba00e318f0ed370eded1cdf38aa75ad55e63945380a0a9138320438845382842e94a5b4ea6756af0c82a0f6b4f17eaf049d617aba98ea0229898dbbae35aa9ef23f2a46c26d419257c35ba11aff1b02ca2024a057f8acaa0cc4c22a6806f250facbdecc1d8874d430ccc277d68ca91b5fb10b4d9f7c681578080").to_vec()),
        BoundedVec::truncate_from(hex!("f891808080a076082e119bb693f858172779676f80da4deb1fd75b39db89ec6c96e36125cf6a8080a02b87e60a23ebea051ea7f029c26c5fad0ba86fb8d6d5d4bb563f48ddbf7fa6aca0d9693138b984cccc06a7461c7f39cc28947c9dd95d94bdea1047ddd420b81360808080808080a0ae23c016152c96bfa600e365cd62d6ce721f0b0d310e3c7c18b8a293b722a4ab8080").to_vec()),
        BoundedVec::truncate_from(hex!("f8669d3e80870bed23e92a482b9f577efea539b7865c0383284e1bf8cb8ae0e3b846f8440280a06801798586ca88b0ef3b4fb3f83162a9f13e5e242b4c8024c490006054e43933a0f99c7a628a59cf1d27d3a906618656d06e3cdcbcd5f91503c002ea2f2420bc01").to_vec()),
    ])
}

fn get_invalid_account_proof() -> BoundedVec<BoundedVec<u8, MessageBytesMaxLen>, MessageItemsMaxLen>
{
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

fn get_valid_storage_proof() -> BoundedVec<BoundedVec<u8, MessageBytesMaxLen>, MessageItemsMaxLen> {
	BoundedVec::truncate_from(vec![BoundedVec::truncate_from(hex!("f8d18080a0fc8644862938b67a6de59daee2ca86a4a43c8c4fe6d7ca5f71ea19a3e85565c080a002116e22ba81d7274dc866a4612e9b4e3f10345d5164d4c6e02fd6b672446f4da0b23f6176235c786974b40b6a64b3428c26e7ecc9530b122dd26ebe148d12c33380a04ee52d46ac712e1be0869a689dd6116bed17180e70d9d327d0e335e4098c0397808080a072b7b4fabd398c9b5c05e5f329038a9a9bda658b15a56a3d6a298755511538b18080a079866ac4ff54c3062d8fbd4fa347961e9a905b4114a2ed9785e22a5c03f4ffb88080").to_vec()),
                                   BoundedVec::truncate_from(hex!("e219a0053d037613f1c22bb588aaa70237b3798774d2b20413c686e2263daef21ec226").to_vec()),
                                   BoundedVec::truncate_from(hex!("f851a0c45dca792d516550b57f7f31e33c67f0e6debfe0bdb3076fe0078c65c5afbf8280808080a022e43fa2c06d3d498253aadec7a7db94183eec2aabbdf2afc67a45107d19932b8080808080808080808080").to_vec()),
                                   BoundedVec::truncate_from(hex!("f8429f3841a49a1089f4b560f91cfbb0133326654dcbb1041861fc5dde96c724a22fa1a0efac9989593dfa1e64bac26dd75fd613470d99766ad2c954af658253a09d1ad8").to_vec()),
    ])
}

fn get_invalid_storage_proof() -> BoundedVec<BoundedVec<u8, MessageBytesMaxLen>, MessageItemsMaxLen>
{
	BoundedVec::truncate_from(vec![BoundedVec::truncate_from(hex!("f90211a0f0a16ee9b11528f3da8796229dad134b9085ed9428d868e6988f9b2473b59d6fa0f8175015d0a3df8fc451d2bd3d64a34e0836f3203129ac567e869f1157b488dfa0f9d56e943c6962cf8e2ca51b94b54307eb45424ebb84ed079b417cf03a85e298a0408af9f1c5f64ed6c517b1dbf661b75a705ef7d78bcae67b9a54c1e8052b56b2a02157d476a9a077cfc9eb00ead5ab65dcbfe363a71e993c3602a66c0fccf13e4aa00772697ebf25f2e83830918bd52bbb9600c077ae289e740ae76c7bdfd34b7ebea0a1dd0da76aacf7c82629c55e4b956b2e9ef77d7fdcee1adeb23d022f0950d554a0695cb723c857d98ad1c96a372f7983bf771556f4608674266a0698531543217ba05c0fb347305720b81c7d39be6fd5b2083af607654098a0f1418ec111a846510aa0ecd30808bffcb164a258c332a29f3050e9e85d28e988305b7f643dcad4f32c8fa0ec5ee93a7ede0a9c641dcd7515c1408ab48f86b5295cd26b3d738e8d8ac7829fa01434a5f6054456bbce0a59ba1c182eeee8e64fd6762ff365e550ca7cd8cedad0a0b4fefcb325f044a6663c9441ec9f025718d0f2d7fc1c29ec819f4a366cafbb6fa0cc26bfb18151569b0f765335474fa3840f9093385816bd14a4a3c553fae62949a06a28c02f7b649bad24b39d9a4e9fc4c8e93b1ae2b043af4f5bbcb8238e193eaba011ef889094bf6ca740810423041169453b7daea3df98b3018523f86e96bf033580").to_vec()),
                                   BoundedVec::truncate_from(hex!("e219a0053d037613f1c22bb588aaa70237b3798774d2b20413c686e2263daef21ec226").to_vec()),
                                   BoundedVec::truncate_from(hex!("f851a0c45dca792d516550b57f7f31e33c67f0e6debfe0bdb3076fe0078c65c5afbf8280808080a022e43fa2c06d3d498253aadec7a7db94183eec2aabbdf2afc67a45107d19932b8080808080808080808080").to_vec()),
                                   BoundedVec::truncate_from(hex!("f8429f3841a49a1089f4b560f91cfbb0133326654dcbb1041861fc5dde96c724a22fa1a0efac9989593dfa1e64bac26dd75fd613470d99766ad2c954af658253a09d1ad8").to_vec()),
    ])
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

fn get_rotate_verification_key() -> VerificationKeyDef<Test> {
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
            "2864156988502350018268114524769442611229738724281856064310359811414088775164",
            "19784911050814990253005325251017779746002278450060367709911093357779852409724"
        ],
        [
            "2320747355788118605608963241136772405889379999161258135797985959373766905799",
            "7118041328407665643077665093375077236507031390654037220453830314560753892708"
        ],
        [
            "1",
            "0"
        ]
    ],
    "vk_alphabeta_12": [],
    "IC": [
        [
            "15615341388138779177592192310982411536626378440854127969627902314302018589756",
            "15825561397777957655855081872509949298182852212017977148985160662370122761845",
            "1"
        ],
        [
            "21866659777455953012076240694890418723891531368136637553921599064988704009798",
            "18794682133425820197214508210971026410261369883290190279860606526851568182754",
            "1"
        ],
        [
            "17134706853007662603932468543386586959990776778768283640697616786730646170163",
            "20580957029031123131958004810864543174606183854578157485523871304119815226629",
            "1"
        ]
    ]
}}"#;

	BoundedVec::truncate_from(step_vk.as_bytes().to_vec())
}

fn get_valid_message() -> Message {
	let data = &[
		Token::FixedBytes(H256::zero().as_bytes().to_vec()),
		Token::Uint(U256::from(1000000000000000000u128)),
	];

	// message = Message(0x02, bytes32(bytes20(0x681257BED628425a28B469114Dc21A7c30205cFD)), bytes32(uint256(1)), 2, 1, abi.encode(bytes32(0), 1 ether), 0)
	let encoded = encode(data);
	Message {
		message_type: MessageType::FungibleToken,
		from: H256(hex!(
			"681257BED628425a28B469114Dc21A7c30205cFD000000000000000000000000"
		)),
		to: H256(hex!(
			"0000000000000000000000000000000000000000000000000000000000000001"
		)),
		origin_domain: 2,
		destination_domain: 1,
		data: BoundedVec::truncate_from(encoded.to_vec()),
		id: 0,
	}
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
			get_valid_step_input(),
			get_valid_step_output(),
			get_valid_step_proof(),
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
			get_valid_step_input(),
			get_valid_step_output(),
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
			get_valid_step_input(),
			get_valid_step_output(),
			get_valid_step_proof(),
			slot,
		);

		assert_err!(result, Error::<Test>::FunctionIdNotKnown);
	});
}

#[test]
fn test_full_fill_step_call_finality_not_met() {
	new_test_ext().execute_with(|| {
		let slot = 7634942;
		SyncCommitteePoseidons::<Test>::insert(
			931,
			U256::from(hex!(
				"0ab2afdc05c8b6ae1f2ab20874fb4159e25d5c1d4faa41aee232d6ab331332df"
			)),
		);
		StepVerificationKeyStorage::<Test>::set(get_step_verification_key());

		StateStorage::<Test>::set(State {
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

#[test]
fn test_execute_message_via_storage() {
	new_test_ext().execute_with(|| {
		let balance_before = Balances::balance(&Bridge::account_id());
		Broadcasters::<Test>::set(
			2,
			H256(hex!(
				"426bde66abd85741be832b824ea65a3aad70113e000000000000000000000000"
			)),
		);

		let slot = 8581263;
		Timestamps::<Test>::set(slot, 1704180594);
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"d6b8a2fb20ade94a56d9d87a07ca11e46cc169ed43dc0d2527a0d3ca2309ba9c"
			)),
		);

		let account_proof = get_valid_account_proof();
		let storage_proof = get_valid_storage_proof();
		let message = get_valid_message();
		// amount in message 1000000000000000000
		let success = Bridge::execute(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			slot,
			message,
			account_proof,
			storage_proof,
		);
		assert_ok!(success);

		let balance_left = Balances::balance(&Bridge::account_id());
		assert_eq!(
			balance_before.saturating_sub(1000000000000000000),
			balance_left
		)
	});
}

#[test]
fn test_execute_message_via_storage_min_wait_time_not_met() {
	new_test_ext().execute_with(|| {
		let balance_before = Balances::balance(&Bridge::account_id());
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
		assert_err!(err, Error::<Test>::MustWaitLonger);
	});
}

#[test]
fn test_execute_arb_message() {
	new_test_ext().execute_with(|| {
		let balance_before = Balances::balance(&Bridge::account_id());
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

		let account_proof = get_valid_account_proof();
		let storage_proof = get_valid_storage_proof();
		let mut message = get_valid_message();

		// change message type
		message.message_type = MessageType::ArbitraryMessage;

		// amount in message 1000000000000000000
		let success = Bridge::execute(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			slot,
			message,
			account_proof,
			storage_proof,
		);
		assert_ok!(success);

		// Currently not supported and it shouldn not affect balance
		let balance_left = Balances::balance(&Bridge::account_id());
		assert_eq!(balance_before, balance_left)
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

		Timestamps::<Test>::set(slot, 1701327753);

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

//
#[test]
fn test_execute_message_with_faulty_storage_proof() {
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
fn test_full_fill_step_call() {
	new_test_ext().execute_with(|| {
		let slot = 7634942;

		SyncCommitteePoseidons::<Test>::insert(
			931,
			U256::from(hex!(
				"0ab2afdc05c8b6ae1f2ab20874fb4159e25d5c1d4faa41aee232d6ab331332df"
			)),
		);
		StepVerificationKeyStorage::<Test>::set(get_step_verification_key());

		StateStorage::<Test>::set(State {
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

		// ensure that event is fired
		let expected_event = RuntimeEvent::Bridge(Event::HeaderUpdate {
			slot,
			finalization_root: H256(hex!(
				"e4566e0cf4edb171a3eedd59f9943bbcd0b1f6b648f1a6e26d5264b668ab41ec"
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
fn test_full_fill_step_call_slot_behind_head() {
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

		StepVerificationKeyStorage::<Test>::set(get_step_verification_key());

		StateStorage::<Test>::set(State {
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
fn test_full_fill_rotate_call() {
	new_test_ext().execute_with(|| {
		let slot = 7634942;

		RotateVerificationKeyStorage::<Test>::set(get_rotate_verification_key());

		StateStorage::<Test>::set(State {
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
		let expected_event = RuntimeEvent::Bridge(Event::SyncCommitteeUpdate {
			period: 931,
			root: U256::from_dec_str(
				"16399439943012933445970260519503780180385945954293268151243539801891563949197",
			)
			.unwrap(),
		});

		assert_eq!(expected_event, System::events()[0].event);
	});
}
