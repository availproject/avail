use super::{get_account_id_from_seed, AuthorityKeys};
use avail_core::{BLOCK_CHUNK_SIZE, NORMAL_DISPATCH_RATIO};
use kate::config::{MAX_BLOCK_COLUMNS, MAX_BLOCK_ROWS};

use da_runtime::{
	constants, wasm_binary_unwrap, AccountId, BabeConfig, Balance, BalancesConfig,
	DataAvailabilityConfig, NomadHomeConfig, NomadUpdaterManagerConfig, NominationPoolsConfig,
	RuntimeGenesisConfig, SessionConfig, StakerStatus, StakingConfig, SuccinctConfig, SudoConfig,
	SystemConfig, TechnicalCommitteeConfig, AVL,
};
use frame_system::limits::BlockLength;
use hex_literal::hex;
use primitive_types::{H160, U256};
use sc_telemetry::TelemetryEndpoints;
use sp_core::crypto::AccountId32;
use sp_core::sr25519::Public;

pub const PROTOCOL_ID: Option<&str> = Some("Avail");
pub const TELEMETRY_URL: &str = "ws://telemetry.avail.tools:8001/submit";
const NOMAD_LOCAL_DOMAIN: u32 = 2000;
const NOMAD_UPDATER: H160 = H160(hex!("695dFcFc604F9b2992642BDC5b173d1a1ed60b03"));

const ENDOWMENT: Balance = 1_000_000 * AVL;
const STASH_BOND: Balance = ENDOWMENT / 100;
const DEFAULT_ENDOWED_SEEDS: [&str; 12] = [
	"Alice",
	"Bob",
	"Charlie",
	"Dave",
	"Eve",
	"Ferdie",
	"Alice//stash",
	"Bob//stash",
	"Charlie//stash",
	"Dave//stash",
	"Eve//stash",
	"Ferdie//stash",
];
const INIT_APP_IDS: [(u32, &str); 3] = [(0, "Data Avail"), (1, "Ethereum"), (2, "Polygon")];

fn get_rotate_vk() -> Vec<u8> {
	let rotate_vk = r#"{"vk_json":{
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
	rotate_vk.as_bytes().to_vec()
}

fn get_step_vk() -> Vec<u8> {
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

	step_vk.as_bytes().to_vec()
}

fn standard_system_configuration() -> (Vec<u8>, BlockLength) {
	let code = wasm_binary_unwrap().to_vec();

	let block_length = BlockLength::with_normal_ratio(
		MAX_BLOCK_ROWS,
		MAX_BLOCK_COLUMNS,
		BLOCK_CHUNK_SIZE,
		NORMAL_DISPATCH_RATIO,
	)
	.expect("Valid `BlockLength` genesis definition .qed");

	(code, block_length)
}

pub fn to_telemetry_endpoint(s: String) -> TelemetryEndpoints {
	TelemetryEndpoints::new(vec![(s, 0)]).unwrap()
}

/// Generates a default endowed accounts.
fn dev_endowed_accounts() -> Vec<(AccountId, Balance)> {
	DEFAULT_ENDOWED_SEEDS
		.iter()
		.map(|seed| (get_account_id_from_seed::<Public>(seed), ENDOWMENT))
		.collect()
}

fn make_data_avail_config(owner: AccountId) -> DataAvailabilityConfig {
	let app_keys = INIT_APP_IDS
		.iter()
		.map(|(id, app)| (app.as_bytes().to_vec(), (owner.clone(), *id)))
		.collect();

	DataAvailabilityConfig { app_keys }
}

pub fn runtime_genesis_config(
	sudo: AccountId32,
	technical_committee: Vec<AccountId32>,
	session_keys: Vec<AuthorityKeys>,
) -> RuntimeGenesisConfig {
	let balances = dev_endowed_accounts();
	let stakers = session_keys
		.iter()
		.map(|k| {
			(
				k.stash.clone(),
				k.controller.clone(),
				STASH_BOND,
				StakerStatus::Validator,
			)
		})
		.collect();
	let validator_count = session_keys.len() as u32;
	let session_keys = session_keys.into_iter().map(|k| k.into()).collect();

	let (code, block_length) = standard_system_configuration();
	RuntimeGenesisConfig {
		// General
		system: SystemConfig {
			code,
			block_length,
			..Default::default()
		},
		babe: BabeConfig {
			epoch_config: Some(da_runtime::constants::babe::GENESIS_EPOCH_CONFIG),
			..Default::default()
		},
		balances: BalancesConfig { balances },
		staking: StakingConfig {
			stakers,
			validator_count,
			minimum_validator_count: 1,
			..Default::default()
		},
		session: SessionConfig { keys: session_keys },
		technical_committee: TechnicalCommitteeConfig {
			members: technical_committee,
			..Default::default()
		},
		sudo: SudoConfig {
			key: Some(sudo.clone()),
		},
		nomad_home: NomadHomeConfig {
			local_domain: NOMAD_LOCAL_DOMAIN,
			updater: NOMAD_UPDATER,
			..Default::default()
		},
		nomad_updater_manager: NomadUpdaterManagerConfig {
			updater: NOMAD_UPDATER,
			..Default::default()
		},
		succinct: SuccinctConfig {
			//TODO check all values
			slots_per_period: 8192,
			finality_threshold: 342,
			period: 931,
			sync_committee_poseidon: U256::from(hex!(
				"0ab2afdc05c8b6ae1f2ab20874fb4159e25d5c1d4faa41aee232d6ab331332df"
			)),
			whitelisted_domains: vec![2],
			step_vk: get_step_vk(),
			rotate_vk: get_rotate_vk(),
			..Default::default()
		},
		nomination_pools: NominationPoolsConfig {
			min_create_bond: constants::nomination_pools::MIN_CREATE_BOND,
			min_join_bond: constants::nomination_pools::MIN_JOIN_BOND,
			max_pools: Some(constants::nomination_pools::MAX_POOLS),
			max_members_per_pool: Some(constants::nomination_pools::MAX_MEMBERS_PER_POOL),
			max_members: Some(constants::nomination_pools::MAX_MEMBERS),
			..Default::default()
		},
		grandpa: Default::default(),
		treasury: Default::default(),
		im_online: Default::default(),
		authority_discovery: Default::default(),
		transaction_payment: Default::default(),
		indices: Default::default(),
		data_availability: make_data_avail_config(sudo),
		technical_membership: Default::default(),
	}
}
