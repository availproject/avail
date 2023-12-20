use frame_support::{derive_impl, parameter_types, traits::ConstU64, PalletId};
use frame_system::{header_builder::da, test_utils::TestRandomness};
use hex_literal::hex;
use sp_core::{H256, U256};
use sp_runtime::{
	traits::{ConstU32, IdentityLookup},
	AccountId32, BuildStorage,
};

use crate as succinct_bridge;
use crate::{
	AvailDomain, MaxBridgeDataLength, MaxProofLength, MessageMappingStorageIndex, SupportedDomain,
};

type Balance = u128;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockDaBlock<Test>;

frame_support::construct_runtime!(
	pub struct Test {
		System: frame_system,
		Timestamp: pallet_timestamp,
		Balances: pallet_balances,
		Bridge: succinct_bridge,
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
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type RuntimeEvent = RuntimeEvent;
	type RuntimeHoldReason = [u8; 8];
	type WeightInfo = ();
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}

parameter_types! {
	pub const MaxVerificationKeyLength: u32 = 4143;
	pub const StepFunctionId: H256 = H256(hex!("af44af6890508b3b7f6910d4a4570a0d524769a23ce340b2c7400e140ad168ab"));
	pub const RotateFunctionId: H256 = H256(hex!("9aed23f9e6e8f8b98751cf508069b5b7f015d4d510b6a4820d41ba1ce88190d9"));
	pub const BridgePalletId: PalletId = PalletId(*b"avl/brdg");
}

impl succinct_bridge::Config for Test {
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type TimeProvider = Timestamp;
	type MaxVerificationKeyLength = MaxVerificationKeyLength;
	type Currency = Balances;

	type MessageMappingStorageIndex = MessageMappingStorageIndex;
	type MaxProofLength = MaxProofLength;
	type MaxBridgeDataLength = MaxBridgeDataLength;
	type RotateFunctionId = RotateFunctionId;
	type StepFunctionId = StepFunctionId;
	type PalletId = BridgePalletId;

	type AvailDomain = AvailDomain;
	type SupportedDomain = SupportedDomain;
}

/// Create new externalities for `Succinct` module tests.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = RuntimeGenesisConfig::default()
		.system
		.build_storage()
		.expect("Genesis build should work");
	succinct_bridge::GenesisConfig::<Test> {
		slots_per_period: 8192,
		updater: H256(hex!(
			"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
		)),
		finality_threshold: 461,
		period: 931,
		sync_committee_poseidon: U256::from(hex!(
			"0ab2afdc05c8b6ae1f2ab20874fb4159e25d5c1d4faa41aee232d6ab331332df"
		)),
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
