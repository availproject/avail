use frame_support::{derive_impl, parameter_types, traits::ConstU64, PalletId};
use frame_system::{native::hosted_header_builder::da, test_utils::TestRandomness};
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
		whitelisted_domains: vec![2],
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

/// Externalities for a chain whose Vector genesis starts mid-history: `head`, that head's
/// header, and the sync committee hash for its period are all seeded, which is what
/// `fulfill` needs before it can accept a first update. Genesis requires all three or none
/// of them, so passing a zero for any one is how the half-configured cases are exercised.
pub fn new_test_ext_with_genesis_head(
	head: u64,
	header: H256,
	sync_committee_hash: H256,
	slots_per_period: u64,
) -> sp_io::TestExternalities {
	let mut t = RuntimeGenesisConfig::default()
		.system
		.build_storage()
		.expect("Genesis build should work");

	vector_bridge::GenesisConfig::<Test> {
		whitelisted_domains: vec![2],
		head,
		header,
		sync_committee_hash,
		slots_per_period,
		period: head / slots_per_period,
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
