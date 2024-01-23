use ethabi::{encode, Token};
use frame_support::traits::fungible::Inspect;
use frame_support::traits::DefensiveTruncateFrom;
use frame_support::{assert_err, assert_ok, BoundedVec};
use frame_system::submitted_data::{Message, MessageType};
use hex_literal::hex;
use primitive_types::U256;
use sp_core::crypto::AccountId32;
use sp_core::{keccak_256, ByteArray};
use sp_runtime::testing::H256;
use sp_runtime::traits::BadOrigin;

use crate::mock::{new_test_ext, Bridge, RuntimeEvent, RuntimeOrigin, Test};
use crate::mock::{Balances, System};
use crate::state::Configuration;
use crate::target_amb::MessageStatusEnum;
use crate::{
	Broadcasters, ConfigurationStorage, Error, Event, ExecutionStateRoots, FunctionInput,
	FunctionOutput, FunctionProof, Head, Headers, MessageStatus, SourceChainFrozen,
	SyncCommitteePoseidons, ValidProof, WhitelistedDomains,
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
        BoundedVec::truncate_from(hex!("f90211a00b9369252d91d5240966845819a5e73a8e7b4b531b9f66bd652df2ef25999e8fa00fbedaed13519e0d8971ca3e8f54fa6250b8c60ae522924ea5b7bc54d7ab049ca0e9a41917172ecd79fbbc0db3c117a990464df2767be84bc5fd749cb50763834fa0420cf2bd2e7aac8cc33b0731d7546b738315ac482524aebc60e39a7d0359ce84a007e78eae37bbc3d25305685e8321582390d9631365f5c40612d111c7c1d2a35aa0f90094ebbf4d71dc00d9ff469a149d62529289b7a88d63f69ef73c45ffadd9b8a0bb5c4d709b179a0dec8c3e298249746c5f36f1ab4c7bb0985e4c565bfa96efd2a0b44d9462647f7f99afe5396fc6c049ca3d343748d9eb27f64e118a4d55d26d2ba08119035469c78ffe7336ba8c0117906ca64c9827f16ef71b5fcd99272d983779a067a02fa9917bda0acfb3646ba7e91c7e54e56e40342c552c0b069f949a64e122a0d2b39ca49bbaf2582e8c48243c86249ef034d14f8272952975c1f01773d508c4a0cd89136cd6f4203f52553a6856ba48ee08fe3d0165b5d020b4781ec0b0195bfda0bfdc0af814301e39e11e3c2186b3969b5c6ffdcb627d99f2e1e425d48300253ea0689a72bc1f0084b35849c938c2298e0d90384a071b6ef7bb145dab1b1205efffa0614a4f66f2451debf003b8584d9c2bbb87ee1d141cb240b494a840a91d32e126a01885c00eb4d6365b341f3c21bc44754f901ff56e957399069209b612c272a97880").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a0c3c1510781a6c8ba9dae550cd691248cbfe8e3071987f0da92f13d03402e68b7a0267ee923bf851b5f9776eebb0c09be120dcb22c804c9478542bb665b2d694d87a0437a44f13d49bbdcd7106f95b62b105478a4709bea251824a1058d082977ecfda03d257aafd460d893e9713d085a389be271e3162420802e7b8133e8f1f71dc38fa01c07a616c35b484e4abd791a6aecd9ece7d107a90d110754afe973b3d6b1bbf5a0e51e122b66863d3132ff63c7cacbb4d4449c788076e9f10f3d28de3ece92f5dea0521e7a7eee71a2d2855e898ba4a610b0fd6ce57bb4adf8e5d2634e033612aa5aa0c1401fa14ded62f7e1a695d505f4148beccd2be22bc25730ef75a5c94ff680cea0bda976ea47bd694d0cd116dc1aae5e56b8333b3d6b053ea8ded30e656725bfdea0e581d8d0e4b3b5ab0d441e1bec84eb205ab2f55ff06354dc47c3f53e3e91c79ba066a53d18e20d26fb3c45305e29d2cbd1ae40fe17f63eeae8ae1d2190a1d91fcda00df4111cf09b03686b76e192425627c394d7e75ac0dc96c6612366b8d68d3340a0c96d1de5da569fd8b08757d8393dca4a3000e001e674deda10d3072b1f8fff25a06ae859e5f232fd752442a5927c5433aaccd122971f59aca10c2ace8a46e5d9c5a0ad8a4773ddacac3510ee6c397f448fa9fd8f70b51d720d424313d6537f64b5eea0bc11c88bcccaf2086de82fe31e9fc2d225563c9252cd8a1b1660399627e642a980").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a01b824a90c181a4d552fec1cc2ece9e933e9c4f6d266097bec54ae98b368a61e0a01f4fba7f84c2e7ddab72e34805dc74c8bb02a003a92e0387b9775ed2a6302d53a06b3e05fa2cd4b374baeeedadc843119323986042d2822b7feb0af7db72926097a0a2495430fe5d48192b266760828d497f3a3c1fefb8b100483c33341a3b039697a0e1b97d100eab7cf1ee2ad00ebd941ec784ef75d2f37dc13e0a8eff47439f7223a04464b9e147b9ed333e9fc1f89f7b9c0d26d4aef49c4430f2d4f2852652d9bb94a056326b52b65780b847eed732801e43ab8f023a6bbdda9c07da3bda25b7afc88ba05f02ed93551f04ec5a1d98da11483897b27f4d252d8d27a616b483b98021c24da08a956e1065bdc19ad7c20878b2579dad7ea1f862e411007f3b464f77ce01c192a06f48ee881f5a1f687f8c6f2d15d36493d1bfa04e4a32cf22080dac94b3de9f1ea0f516759cd854e7847023c7d7924f2986fafbe20eb3f601beed3b406b1ba9e2c5a04366281a689a29f9ed771d45ea5f3ee66f8a09df21f1849055203030c7cb7036a023a3e5edc04df1eddf74604f6660df52b1bf48f29a4b97710d2f87b57ed4464ca045bd1a9b6617bc253fc689e98d5000de2d065f707ac7467607e654ba6042762ea020b05480454d83862510dd3c4696719ae3c8554f09980bf49b0e3a1576c9769ea05ffa88eb9f0947617248a029b087e89c72bd1f1cc645f0ce38356b9f5d0f9ac480").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a051b4b24ddc5e92b57fcb73192a8b66d9d0a682e1cc73d1a03272a9f4514b97d2a051b20ccb0db45c4f2a10588f5e8a27f5cd067394cdc16edfd4b808bae92a3534a0e1df6b6c81a8591be15c229e932f2edb46de5d80520308b952b757b55582a0dca0812a635747f148ed16fb42b8e4cf4d50db52270fe0e11cc7cb8cb92a0fc1b28ca03b098829ec412b08a1126da4495a8f755ebab82bc45ee43b92cdf4c36eac304ea06cd28b484d51aa518163bcc2d283d17833194168a3f11dcadf022410745c7e96a005b98558bfdd9059fdabd9f2ad3924ad005529a454438728b711ca76f858a3c9a0aec8e508057dd392fad463c3bfcca4b07206b151d53ca4a9876ae4a351663f89a0cbc58bc283fabbfa3018afac72b4b21d9a04a157efcfaea50b747f98e5836a53a00e26a1305e5039b385f353d4d89ede2f70b699ad262bc22eb88f7f22a43b87e4a0facb7e1f785a7056978aea57fa457856ba9771fb27f6950b6e36a6dac86335b6a0f257bb3e27acb6f1b0067832538536ec8cb6fa476184caf373169f64bcef398fa03b981c3a3b71e048c2217feed641e5501822d8ca73d07e12737ed4b178cd929ca0850c81bd936124ea2c3ccfd40be173599b2df58b0a5e5cddad9cbc40a6a6baa1a014b90165a798608acb60e89a66e3711d6beb3c65910e51bdcf6364803fd26747a0c2343b8c71ed71298edcc8bca4001250055bf1e3110d9f9e9bb2de2186ea666780").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a0cb0cdddaf997332e6bde2eed1e92624ebfd39d1954ebd82f6ac02ea91e9d1dd6a07d022abe2fb0047caa61729dff7a72d087019d18037b4e9a38d6c00dd7f853bba054b2ccdbfd7c9f548068ad017ca4f3a9477afc713af9190d55d3ff3c868da7bca062912825979c5160c38e7e1d658a72d45956af648aabcd7e325adfcc955cd608a04c4e18220e36d6f10d20d6ea82b3023a61bb512981a52c3135115d00572fd9a9a09ca3e2a669bf7d2b24a635d874ffa5990c372e8e5abc8bbeeee15020b9b424b1a072781fd5d2c736c7986fb59b69c7ca355d268cd5a73ddf9202a05632fe42435ea08777c905af547074715fd074e3748bb0d138944b0a94adee42b70b64330e1816a0d6745d1404c2246391bd72db719ac915518a4b26deeb0f4640e6fd8dced6f2b0a0685d6698523489b73b28ac2b30568a78bea6bfd5b483b22316da7c976ab9fdbba01d30a49c428f3c3212ae5dee0b95a79f873f1fa344aa83b438886c0c507cb091a0c69962a4ebce43581445d16f514db993046127ef98294326cb45fdc5a42cb82ca0030e05160ec3744ce8e6d13c4801edef8169be649f13045b6d40d4e9b743807aa06253d30ad1389e78120a12498b69a0267a41d025dae18281d3aacaedd41f33cea08cae98ed5dd53ce4452b340c85f32a7e0409f9d295ecfa877e71305962bced6ea084af1765acd884a8ca25266477e22054558a6cbca0d68420b3df3f63ac19a06980").to_vec()),
        BoundedVec::truncate_from(hex!("f90191a0f9f2953d2c1c40f457ae2a6894201961f616a42ee0e942b338dfa7435800444ca0e67c195c9dcf9ce6c5bbe14f941178e1f88bf8c4b96c10fcdb94011ce241bf63a034c4c4a975771155c62835e2c8368b6d78388a45e597647cc8f7a704f9e1a5d580a009ed37b7f91327e4defae392a273c53541b8b5bd9ed76b8945b8ffff1a5fc0b2a005970b28afcc6892c47900bf9ae403ecd5364f79af62a0cdfa8ca4e04925dbdea0c6894ea8fb30775c1cca24dbaee5365e3d3f3df625736c0784378190d4507d8580a0f840f68a60355397bb24369387aaaa776432dceb3db4da14de72d82db76d00f7a0bdba75a92da491a76173558d6e07dc24699534af0be478b0bb803acc76251d5780a064dc4f711ccf12b28b6304ee111d7ae5edc34ffca6688c0f25846edcf1b3468ca0a285ed0e46b78847cf1d0fd0c8b81580e9417a534357d6bd8b9496f9ccc529b7a0abc253bb447ace56fd185b004d2764e61fffb6bd21ba278c1e0a0864c35080a3a0184573b58579fcde692191ba18c0b30dd06326129c3861bf55d29737f646162a8080").to_vec()),
        BoundedVec::truncate_from(hex!("f8918080a0f2054bc8fc8c37e14cf4934ff82e49f5016007cf3c8a16d5699d04de84b692eb8080808080a0e53905b3f1ea5db82f82f442b71d07c6a97cfa5ee491e30f850be0ea3389466ca062f6d91b16d4d417f8643b7315ace6e05afb70aa59c0b19fc863c33f8c87100780808080a0cf5bc8e1a57bb56b9c38250e75864a1fd7a893ebecd6bb0ed2ed301c68d581a18080").to_vec()),
        BoundedVec::truncate_from(hex!("f8669d343c360e5679746f94d3583df82fee3a96e5a6b33dcf5e57421b01c9a8b846f8440280a0089abb6beda7a92401ee554da94e735920984a64e7403d5cd7c82098de059da9a054ccfa149cf4cf471e53bc5dfbfe8ae5c27fa96b0599f2747ffe6e1fb8bd4396").to_vec()),
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
	BoundedVec::truncate_from(vec![BoundedVec::truncate_from(hex!("f90131a02aa432bd6022e7da6d0188f41427645658b74a08227a036f80fa44bff9fc57af80808080a0c01b2d5e61b71c73bc7b1b73db39a4d89bea07f4594557fec2242da9c90c91a580a073768754f5d28b5a762b44f6c37233fbedbeab05779b7dfb407ede56d2812891a01f3fcf34e7a10de63f5fe300d0522504aa29e6a4a7ad39df02bfa94e958e5e458080a08f056b51124e5c81f7d86b4364a97f66324957dc1042f751bce474abfa1480e5a098b414db83b4c1efa80ad36b641c9c517b2225cb212fc236116b2f9dc69ce64fa09a0e693399670076dd7708930bdc19b9101b3f181c4bf62d25edf6e1fd6e881da079866ac4ff54c3062d8fbd4fa347961e9a905b4114a2ed9785e22a5c03f4ffb8a03a1e44d3669992d3ac3f750a1159442ba1d83432cb03e5815d7eed7d97c7e90380").to_vec()),
                                   BoundedVec::truncate_from(hex!("f851808080a08ddae10e810d2127e5d527cab77909cdc5b99e20544edfb6f0c7b46033fe17e080808080808080808080a0c43b94283974430fdacaadaea093cd1524e306af5fc8c15ae3b13528d8f332088080").to_vec()),
                                   BoundedVec::truncate_from(hex!("f843a020b5be412f275a18f6e4d622aee4ff40b21467c926224771b782d4c095d1444ba1a0efac9989593dfa1e64bac26dd75fd613470d99766ad2c954af658253a09d1ad8").to_vec()),
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
        BoundedVec::truncate_from(hex!("f90211a00b9369252d91d5240966845819a5e73a8e7b4b531b9f66bd652df2ef25999e8fa00fbedaed13519e0d8971ca3e8f54fa6250b8c60ae522924ea5b7bc54d7ab049ca0e9a41917172ecd79fbbc0db3c117a990464df2767be84bc5fd749cb50763834fa0420cf2bd2e7aac8cc33b0731d7546b738315ac482524aebc60e39a7d0359ce84a007e78eae37bbc3d25305685e8321582390d9631365f5c40612d111c7c1d2a35aa0f90094ebbf4d71dc00d9ff469a149d62529289b7a88d63f69ef73c45ffadd9b8a0bb5c4d709b179a0dec8c3e298249746c5f36f1ab4c7bb0985e4c565bfa96efd2a0b44d9462647f7f99afe5396fc6c049ca3d343748d9eb27f64e118a4d55d26d2ba08119035469c78ffe7336ba8c0117906ca64c9827f16ef71b5fcd99272d983779a067a02fa9917bda0acfb3646ba7e91c7e54e56e40342c552c0b069f949a64e122a0d2b39ca49bbaf2582e8c48243c86249ef034d14f8272952975c1f01773d508c4a0cd89136cd6f4203f52553a6856ba48ee08fe3d0165b5d020b4781ec0b0195bfda0bfdc0af814301e39e11e3c2186b3969b5c6ffdcb627d99f2e1e425d48300253ea0689a72bc1f0084b35849c938c2298e0d90384a071b6ef7bb145dab1b1205efffa0614a4f66f2451debf003b8584d9c2bbb87ee1d141cb240b494a840a91d32e126a01885c00eb4d6365b341f3c21bc44754f901ff56e957399069209b612c272a97880").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a0f965da070dc0173a61d34b21bea4dfde26c834257629677157ae7248b50cd81fa03a76449b49477746172ed14e2f996a8131b3e23de3315a1923f38b7ea415b979a0d074384d170d5abab0c1d9ef28cc3c0ff8f36851a7bcbaef501c6032df0943aaa0149b7581ab9bc820217566d4304c13c418893863964f1b2290390f2f4f9c07baa067bc808de655c5710908cbb592420df656ddb294a0ca123c5d1c074e977175aaa02965ae45e7ec302c8d1d24d07954074343df8247da2f6122939181ca3f187dd0a0edbf4144f7d295047f49dd10e218aa3d590cb406312310b0ef56184c1d046822a0915dbafe8ba1d369a3e24375095596455fbc9aacc468958f6928ce428ac31967a0b9e44b513ed8ca8ee7409e399c4a467dc7d260c28d1fba4685852f5e3b65f4d8a09567dcd6366635567c2e61fe0d984516a77a4b0c204bd9b2f85ff343216780afa08390c69ab6052b335f241b5b187af412395910213899dbbf6084987846204c7ba0d38a47f0e0b2b6267a7e360308cbe0ab49e22a0a8615439d3423d6c363080ff9a083e7617b1a68407db284cd4035e04b0637b727c949205cd9ed176319cb9c0e52a061503e01e7f1d1fa6ef925e561be0a4a6fd580263da8f811121b90b87708e562a09c0414ab3be0fee449c042627dac5fd0f3d228b53499a0fc181045f873e3403ca092962b27d984b35f646fa84d3ef49830c5ee8fb0d6db97a912aaf21da043187580").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a0b882b0bbc9b8e3d25a4829a3ce4d49ec7c784383a50f5ff5aa49afeb1ff11cbaa03a6a3554903a3fc21c4026084a3170c202a1c64230dc99b13fdf14985a28183aa057931ffba973f857ce9ff8fbb84958b8e4cf1477edd321e757002870535e308ca074c4acaec7053c4b57a5916049ee1c48b6177badf67cd10c76130a1b8c519329a09c64edf3c38e2de3724fcbae6235accfa9025f9cc0e15abd79d534094b4405aaa092a8e4d137cd6795a50516a76d7098981f1cbdf767fc7cf53f807d32b5101b3ba0c3db2eb22e70e19898ea197ff287371c8a98526f1fab57b446a9082abd5e77fea05cc5337564cd40ceb6f21e1529d46808a7f9dfd03702ed6ccaf31be9a20baeb1a0f453e0497c3ffddaae16012a300dece9147aed514e66316b603a8b438422011ba0fda6a467384c1277311ae9586e5baa65e3fb197cab01a9d839b81b5343f19290a00f02720543a48e733ca4f18777471b0840f1f25dc0c8e61de5715504903609e6a05c825f0b4465760d160e47f5f84e736c7b0d2196e52a01c3974b125fd7a3224ea04d04067ea381da30cadfd36aa2cb1804e8307254bc9a883ebfa39d159b61c119a07f829aa1f376ffe4373e40912576beaf801b035fade0eb9923aae9d2d4cdc4cea092664e0322a11173fa7da7d5149d9b042551e546f9cb9912e86db84b7a6f8ee2a0711815f7bc0dd6bbe61bebb5c93e8a0f5e07b193fe303acb318e4ef8ebd74a9380").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a0fa16c381c9407da90879bf32e2084769a102f4dd505cee8093a3f5f5d5d11897a02e7daca78cdd4bf6f895f54c08da29ec71c621dc987178f71cf88837422b9dbba027bb1b0aaaaec895c5f719e485905eda3119f4df0f3ba4fb53e560974d17f567a08ef21213d4f2b18f4afbf5e1a3823161a31971d18d0b83c6df044cb6676b7ceda03d9ab8d58ef85e067b9da2bc899df09dcd41a886076521fa8ee3afb25f13d7c2a06d355255fa840d7aca803bb9486a969879ef3f403facb0b38ee3e7c89255e043a01a289c3af5517fd34799b37159c365964ed3511031574af034788c76d7597bb7a09de99bcbf90daa9fbc363cc42606f1d303d3589c36e1a037758fed92a0186d51a0c6ee166b4fb46bf4272f5fb3d2947649db5042e999a4b1a579d781151797121da0ce52b5820413a2bf7de720841e6a7224857f9adcf5dea56fa443c0675264931fa0975c99cfe1455d01ee6f3199e30a4d60f0770588454a235956f114fe9a4cac3ea03ad3c23b37f0004e835b8f2e0435c0248fd039c20c33ddfbbd7541b891b4afaca0f93e9f7841ced51de169d29613da8e39dda1c486bf03674fe25c74a294a57675a03a1aa0730d32cc21169724a4cdbd4f2b97245e520814a8983d2752e15455238ba0dc1eb94acdc6fa937ca3cfe76a5d7f40935beea7d6bcf0ae0b9f95f9b96abc4da0b4ad959ea31afa024dd49a00c6c8b83b7192bd05c9e30d3382d6d47fcbe7f10280").to_vec()),
        BoundedVec::truncate_from(hex!("f90211a0480f6762979b1760658dfb6e799619c1782c056285565d909532cffdca48a7aca09722f0a6e84b4709d9d679709cf56c36030401959a68ca1200fca4fd45a7dd38a000835e044809c0a8e9ba3b91d2ff851051480178da4ec312c61aec9a246d3979a0669269114e17b007ea67b3fbf27aa2f84fd2c8003ce1f36b8d33d67a6eca7469a0171cdadb913b5d039f776caceaa312bbc8e8dc357e7d782dc3a0c6e6a2a0da5ba0eac5dec43c8a7a00e40aecbb0d0b13f31464ff1ad4a764c6ff4581cd6f2a7332a0df68af5b5502dd5ef713e5f670cb11950e713ae8c143a4f331b05bf5dfdadffca0215883bf3ae115539f7792c3187a21ba302928b2184e81b05b5594a917018774a0e4c394a136f0c0e1c9a5bf73f249f6f7df3b664323a302924bb5ba269f7ee61fa0035a6b61964189a1508e2ad03d910e90dfc995b8aaec8b7a43bc460c72a46606a08b4a9bc6de38ca5e2405d1dd2cf8709bdb4b92e969210bef2a89ab286799bcf3a053d23a5fddd8218e1c5d3664496e09dfa4e4ebe513014d05ab949558c18399fea03949393cafa9086ecc8f55516cd414034deb32b32e35bcf7c75c31116a544008a0d5c4b0586d91d02284d2343033f5b65572b4ec07b61e650bf0d15049bdd00ab0a01c4f252dd383c4d008e66839589716fece51f9748a761dbe2291c008896a2e28a0ec1f09945d1f38964901d79792ee69ae5300bb7cee5a5bdf6d9e6efdb8eec43080").to_vec()),
        BoundedVec::truncate_from(hex!("f90151a023b80573a6326250c3ae2efbe4af8207d8c1a20acfbfbb8444c7b728bff4e054a0aace3888a6f1f350b8d8ebc1e11687281610dd304eb527cbf884e12363a5a7a3a07cc000006215a0e9d69011c9c53a2b1ba6412a2b98c0439c8efde0a55601b58ca045d4fb94af2b968111c95fc4e55d1da32159431cf5e8b7d99334b90a8cb0efbd8080a00b092b863d6b5435d71a0d65ece895d7b115b9d6d76cd633def212627fe50dfc8080a0270c83c653946af43045281ed905ad9307ba019729f6b8c4712a30ce3b2b12918080a0c0efdb94ce26fd150d90d049e10595cac38e8f151198536fd9d4f8965ca772ffa006a6f61b1e3d71530b9b630d2fa306a480b49bdf83e14ac397398cf5f81c7e8da02acbd28e4f5185b16c60ce005bcb30acaf8e3e70e46db714869bb0f60e1a3a05a0db46a0c8fb8a7ed852f1fcffd35af11aa2bf758ffa8079ac1739d7c33e23511180").to_vec()),
        BoundedVec::truncate_from(hex!("f87180808080808080808080a0ab0da2d5bc9713d2a60975c6fdd916ba1729acc7e75110677583e7e37c2c6df68080a0c186d6514c8893dbbde6a0d9b5294857892ce0b49a44c0799edfba1567b0961080a0c37fb0f6ba6981de5e302ac5538f212162e0b885068c9f1d9ebe091c1d4e031d80").to_vec()),
        BoundedVec::truncate_from(hex!("f8669d32f0e5aa610617dbfcec97bc206d49dc85c62fdfcd084b1fec6dd67dc5b846f8440201a0dad0d599c05e47b3960c18fcb0ad5089027159340e6a86e62a1f618945433650a0636d977dc571e5f18377d0f398751d5ca05b1c5041b6bc97fc3564217d52ee53").to_vec()),
    ])
}

fn get_valid_amb_storage_proof() -> ValidProof {
	BoundedVec::truncate_from(vec![
        BoundedVec::truncate_from(hex!("f90131a0358aed89bfaa22b0179d1015fa6034c7ad29702af20b05b02f3cf63f69c2811280808080a08c857f53d31e0c1b681ff481d435a48986faa0615d611b05db5d0b3f4691f6d1a0d40ba3e2c5ca0722bb231d4a661b452920758ab202dbcfbc8e505d9f436e5600a0c3d20e38630b6b8f49ea6ca919b9077081557a4ca337c374e42219e4738a8970a0d2059c3c7e4dacb02dec49822ce85568a4419849a093ec956598dc2c3269238d808080a08c675371a85fc8f524a707007b05d0b87d7ecbde37f10e953a03da38431d31f8a0079a6b40f411dbc043c20594864f4e7f3e5ef2ef8e27f230c3431ec771b2d001a079866ac4ff54c3062d8fbd4fa347961e9a905b4114a2ed9785e22a5c03f4ffb8a0fef4138a6a9993fb0418e252583a03d2586caf404c7a16f7083600f49aac8cf280").to_vec()),
        BoundedVec::truncate_from(hex!("f851808080a0aec544652aa67b55271eec87a45f5ca89f6a6ea762450ca63b014ceb073e4e9d80808080808080808080a08c06dc4d3d3e8d7fe5a8a88222594ba9f4cdb19baaa8e60919b5617770423f828080").to_vec()),
        BoundedVec::truncate_from(hex!("f843a020b5be412f275a18f6e4d622aee4ff40b21467c926224771b782d4c095d1444ba1a05774ba3f9618e2da3885b0e2853e4005c3e836625e8be0f69bf3d93f51fac58d").to_vec()),
    ])
}

pub fn get_valid_message() -> Message {
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
fn test_full_fill_step_call_proof_not_valid() {
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
fn test_full_fill_step_call_not_valid_function_id() {
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
fn test_full_fill_step_call_finality_not_met() {
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

#[test]
fn test_execute_fungible_token_via_storage() {
	new_test_ext().execute_with(|| {
		let balance_before = Balances::balance(&Bridge::account_id());
		Broadcasters::<Test>::set(
			2,
			H256(hex!(
				"DC3542b6fcC39dC0d51ecdCbc6Fbb130D5e48d95000000000000000000000000"
			)),
		);

		let slot = 8581263;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"c42310d65b1e953e8864480367a03179d6bd78d4ca522a5a977d2801b9b2e1d9"
			)),
		);

		let account_proof = get_valid_account_proof();
		let storage_proof = get_valid_storage_proof();
		let message = get_valid_message();
		let message_encoded = message.clone().abi_encode();
		let message_root = H256(keccak_256(message_encoded.as_slice()));

		// amount in message 1000000000000000000
		let result = Bridge::execute(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			slot,
			message,
			account_proof,
			storage_proof,
		);

		let expected_message_root: H256 = H256(hex!(
			"efac9989593dfa1e64bac26dd75fd613470d99766ad2c954af658253a09d1ad8"
		));
		let balance_left = Balances::balance(&Bridge::account_id());
		assert_ok!(result);
		assert_eq!(
			balance_before.saturating_sub(1000000000000000000u128),
			balance_left
		);
		assert_eq!(expected_message_root, message_root);
		assert_eq!(
			MessageStatus::<Test>::get(message_root),
			MessageStatusEnum::ExecutionSucceeded
		);
	});
}

#[test]
fn test_execute_arb_message_invalid_hash() {
	new_test_ext().execute_with(|| {
		let balance_before = Balances::balance(&Bridge::account_id());
		Broadcasters::<Test>::set(
			2,
			H256(hex!(
				"Aa8c1bFC413e00884A7ac991851686D27b387997000000000000000000000000"
			)),
		);

		let slot = 5085118;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"c42310d65b1e953e8864480367a03179d6bd78d4ca522a5a977d2801b9b2e1d9"
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
		Broadcasters::<Test>::set(
			2,
			H256(hex!(
				"DC3542b6fcC39dC0d51ecdCbc6Fbb130D5e48d95000000000000000000000000"
			)),
		);

		let slot = 8581263;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"c42310d65b1e953e8864480367a03179d6bd78d4ca522a5a977d2801b9b2e1d9"
			)),
		);

		let message = get_valid_message();
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
				"DC3542b6fcC39dC0d51ecdCbc6Fbb130D5e48d95000000000000000000000000"
			)),
		);

		let slot = 8581263;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"c42310d65b1e953e8864480367a03179d6bd78d4ca522a5a977d2801b9b2e1d9"
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
				"DC3542b6fcC39dC0d51ecdCbc6Fbb130D5e48d95000000000000000000000000"
			)),
		);

		let slot = 8581263;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"c42310d65b1e953e8864480367a03179d6bd78d4ca522a5a977d2801b9b2e1d9"
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

#[test]
fn test_execute_message_with_already_executed_message() {
	new_test_ext().execute_with(|| {
		let balance_before_transfer = Balances::balance(&Bridge::account_id());

		Broadcasters::<Test>::set(
			2,
			H256(hex!(
				"DC3542b6fcC39dC0d51ecdCbc6Fbb130D5e48d95000000000000000000000000"
			)),
		);

		let slot = 8581263;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"c42310d65b1e953e8864480367a03179d6bd78d4ca522a5a977d2801b9b2e1d9"
			)),
		);

		let message = get_valid_message();
		let account_proof = get_valid_account_proof();
		let storage_proof = get_valid_storage_proof();
		let account: AccountId32 = AccountId32::from_slice(message.to.as_bytes()).unwrap();
		let account_balance_before = Balances::balance(&account);

		let ok = Bridge::execute(
			RuntimeOrigin::signed(TEST_SENDER_ACCOUNT),
			slot,
			message.clone(),
			account_proof.clone(),
			storage_proof.clone(),
		);

		assert_ok!(ok);
		let balance_after_transfer = Balances::balance(&Bridge::account_id());
		let expected_transfered_value = 1000000000000000000u128;
		assert_eq!(
			balance_before_transfer,
			balance_after_transfer.saturating_add(expected_transfered_value)
		);
		let account_balance = Balances::balance(&account);
		assert_eq!(account_balance_before, 0);
		assert_eq!(account_balance, expected_transfered_value);

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
				"DC3542b6fcC39dC0d51ecdCbc6Fbb130D5e48d95000000000000000000000000"
			)),
		);

		let slot = 8581263;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"c42310d65b1e953e8864480367a03179d6bd78d4ca522a5a977d2801b9b2e1d9"
			)),
		);

		let mut message = get_valid_message();
		// alter message
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
fn test_full_fill_step_call() {
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
fn test_full_fill_rotate_call() {
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
fn execute_arbitrary_message_works() {
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
				"Aa8c1bFC413e00884A7ac991851686D27b387997000000000000000000000000"
			)),
		);

		let slot = 5085118;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"c42310d65b1e953e8864480367a03179d6bd78d4ca522a5a977d2801b9b2e1d9"
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
			from: message.from,
			to: message.to,
			message_id: message.id,
			message_root,
		});
		System::assert_last_event(expected_event);
		assert_eq!(
			MessageStatus::<Test>::get(message_root),
			MessageStatusEnum::ExecutionSucceeded
		)
	});
}

#[test]
fn test_double_execute_arbitrary_message() {
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
				"Aa8c1bFC413e00884A7ac991851686D27b387997000000000000000000000000"
			)),
		);

		let slot = 5085118;
		ExecutionStateRoots::<Test>::set(
			slot,
			H256(hex!(
				"c42310d65b1e953e8864480367a03179d6bd78d4ca522a5a977d2801b9b2e1d9"
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
			from: message.from,
			to: message.to,
			message_id: message.id,
			message_root,
		});
		System::assert_last_event(expected_event);
	});
}
