use frame_support::{derive_impl, parameter_types, traits::ConstU64, PalletId};
use frame_system::{header_builder::da, test_utils::TestRandomness};
use sp_runtime::{
	traits::{ConstU32, IdentityLookup},
	AccountId32, BuildStorage,
};

use crate as vector_bridge;
use crate::constants::{ROTATE_VK, STEP_VK, TEST_ROTATE_FUNCTION_ID, TEST_STEP_FUNCTION_ID};

type Balance = u128;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockDaBlock<Test>;

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
	type BaseCallFilter = frame_support::traits::Everything;
	type Block = Block;
	type BlockHashCount = BlockHashCount;
	type HeaderExtensionBuilder = da::HeaderExtensionBuilder<Test>;
	type Lookup = IdentityLookup<Self::AccountId>;
	type OnSetCode = ();
	type PalletInfo = PalletInfo;
	type Randomness = TestRandomness<Test>;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SubmittedDataExtractor = ();
	type UncheckedExtrinsic = UncheckedExtrinsic;
	type MaxDiffAppIdPerBlock = ConstU32<1_024>;
	type MaxTxPerAppIdPerBlock = ConstU32<8_192>;
}

parameter_types! {
	pub const MaxReserves: u32 = 2;
	pub static ExistentialDeposit: u128 = 1;
}

impl pallet_balances::Config for Test {
	type AccountStore = System;
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type FreezeIdentifier = [u8; 8];
	type MaxFreezes = ConstU32<2>;
	type MaxHolds = ConstU32<2>;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type RuntimeEvent = RuntimeEvent;
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type WeightInfo = ();
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

impl vector_bridge::Config for Test {
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type TimeProvider = Timestamp;
	type Currency = Balances;
	type MessageMappingStorageIndex = ConstU64<1>;
	type PalletId = BridgePalletId;
	type AvailDomain = ConstU32<1>;
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
		function_ids: (TEST_STEP_FUNCTION_ID, TEST_ROTATE_FUNCTION_ID),
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
