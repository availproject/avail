use frame_support::{derive_impl, parameter_types};
use sp_runtime::{BuildStorage, Perbill};
use sp_staking::EraIndex;

use crate::{self as pallet_fusion};

type Extrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockDaBlock<Test>;
type Balance = u64;

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

pub struct MockEraProvider;
impl pallet_fusion::EraProvider for MockEraProvider {
    fn current_era() -> EraIndex {
        0
    }
}

parameter_types! {
	pub const FusionPayoutPercentage: Perbill = Perbill::from_percent(10);
}
impl pallet_fusion::Config for Test {
	type Currency = Balances;
	type FusionPayoutPercentage = FusionPayoutPercentage;
	type EraProvider = MockEraProvider;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
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