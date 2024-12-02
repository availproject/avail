use frame_support::{derive_impl, parameter_types, PalletId};
use frame_system::EnsureRoot;
use sp_runtime::BuildStorage;
use sp_staking::EraIndex;

use crate::{self as pallet_fusion};

type Extrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockDaBlock<Test>;
type Balance = u64;
type AccountId = u64;

frame_support::construct_runtime!(
	pub struct Test {
		System: frame_system,
		Balances: pallet_balances,
		Fusion: pallet_fusion,
	}
);

parameter_types! {
	pub const BlockHashCount: u32 = 250;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type AccountData = pallet_balances::AccountData<Balance>;
	type Block = Block;
	type BlockHashCount = BlockHashCount;
	type HeaderExtensionBuilder =
		frame_system::native::hosted_header_builder::da::HeaderExtensionBuilder<Test>;
	type OnSetCode = ();
	type PalletInfo = PalletInfo;
	type Randomness = frame_system::test_utils::TestRandomness<Test>;
	type Extrinsic = Extrinsic;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig)]
impl pallet_balances::Config for Test {
	type AccountStore = System;
}

parameter_types! {
	pub const FusionPalletId: PalletId = PalletId(*b"avl/fusi");
	pub const MaxCurrencyNameLength: u32 = 32;
	pub const MaxMembersPerPool: u32 = 10;
	pub const MaxTargets: u32 = 16;
	pub const MaxUnbonding: u32 = 8;
	pub const BondingDuration: EraIndex = 3;
	pub const SlashDeferDuration: EraIndex = 0;
	pub const HistoryDepth: u32 = 20;
	pub const MaxSlashesPerPool: u32 = 100;
	pub const MaxPoolsPerValidator: u32 = 100;
}
impl pallet_fusion::Config for Test {
	type Currency = Balances;
	type CurrencyToVote = ();
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type ApprovedOrigin = EnsureRoot<AccountId>;
	type PalletId = FusionPalletId;
	type MaxCurrencyNameLength = MaxCurrencyNameLength;
	type MaxMembersPerPool = MaxMembersPerPool;
	type MaxTargets = MaxTargets;
	type MaxUnbonding = MaxUnbonding;
	type MaxPoolsPerValidator = MaxPoolsPerValidator;
	type MaxSlashesPerPool = MaxSlashesPerPool;
	type BondingDuration = BondingDuration;
	type SlashDeferDuration = SlashDeferDuration;
	type RewardRemainder = ();
	type HistoryDepth = HistoryDepth;
	type StakingFusionDataProvider = ();
	type WeightInfo = ();
}

/// Create new externalities for `Fusion` module tests.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
