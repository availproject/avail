use frame_support::{derive_impl, parameter_types, traits::ConstU64, PalletId};
use frame_system::{native::hosted_header_builder::da, test_utils::TestRandomness};
use hex_literal::hex;
use primitive_types::H256;
use sp_runtime::{
	traits::{Block as BlockT, IdentityLookup},
	AccountId32, BuildStorage,
};

use crate as vector_bridge;

type Balance = u128;
type Extrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockDaBlock<Test>;
type Header = <Block as BlockT>::Header;

pub const STEP_FUNCTION_ID: H256 = H256(hex!(
	"af44af6890508b3b7f6910d4a4570a0d524769a23ce340b2c7400e140ad168ab"
));
pub const ROTATE_FUNCTION_ID: H256 = H256(hex!(
	"9c1096d800fc42454d2d76e6ae1d461b5a67c7b474efb9d47989e47ed39b1b7b"
));
pub const STEP_VK: &str = r#"{"vk_json":{
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

pub const ROTATE_VK: &str = r#"{"vk_json":{
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

frame_support::construct_runtime!(
	pub struct Test {
		System: frame_system,
		Timestamp: pallet_timestamp,
		Balances: pallet_balances,
		Bridge: vector_bridge,
	}
);

parameter_types! {
	pub const BlockHashCount: u32 = 250;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type AccountData = pallet_balances::AccountData<Balance>;
	type AccountId = AccountId32;
	type Block = Block;
	type BlockHashCount = BlockHashCount;
	type HeaderExtensionBuilder = da::HeaderExtensionBuilder<Test>;
	type Lookup = IdentityLookup<Self::AccountId>;
	type PalletInfo = PalletInfo;
	type Randomness = TestRandomness<Test>;
	type Header = Header;
	type Extrinsic = Extrinsic;
}

parameter_types! {
	pub const MaxReserves: u32 = 2;
	pub static ExistentialDeposit: u128 = 1;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig)]
impl pallet_balances::Config for Test {
	type AccountStore = System;
	type Balance = Balance;
	type ExistentialDeposit = ExistentialDeposit;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}

parameter_types! {
	pub const BridgePalletId: PalletId = PalletId(*b"avl/brdg");
}

#[derive_impl(crate::config_preludes::TestDefaultConfig as crate::DefaultConfig)]
impl vector_bridge::Config for Test {
	type TimeProvider = Timestamp;
	type Currency = Balances;
}

/// Create new externalities for `Vector` module tests.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = RuntimeGenesisConfig::default()
		.system
		.build_storage()
		.expect("Genesis build should work");

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(Bridge::account_id(), 2_000 * 1000000000000000000)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	vector_bridge::GenesisConfig::<Test> {
		finality_threshold: 461,
		function_ids: (STEP_FUNCTION_ID, ROTATE_FUNCTION_ID),
		slots_per_period: 8192,
		step_verification_key: STEP_VK.as_bytes().to_vec(),
		rotate_verification_key: ROTATE_VK.as_bytes().to_vec(),
		whitelisted_domains: vec![2],
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
