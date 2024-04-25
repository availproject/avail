#![cfg(test)]

use avail_core::currency::{Balance, AVAIL};
use frame_support::{
	derive_impl, parameter_types,
	weights::{ConstantMultiplier, IdentityFee},
};
use frame_system::{
	mocking::MockUncheckedExtrinsic, native::hosted_header_builder::da::HeaderExtensionBuilder,
	test_utils::TestRandomness,
};
use pallet_transaction_payment::CurrencyAdapter;
use sp_runtime::BuildStorage;

use crate::{self as da_control, *};

/// An unchecked extrinsic type to be used in tests.
type Extrinsic = MockUncheckedExtrinsic<Test>;

/// An implementation of `sp_runtime::traits::Block` to be used in tests.
type Block = frame_system::mocking::MockDaBlock<Test>;

type BlockNumber = u32;

frame_support::construct_runtime!(
	pub struct Test {
		System: frame_system,
		Utility: pallet_utility,
		Balances: pallet_balances,
		TransactionPayment: pallet_transaction_payment,
		DataAvailability: da_control,
	}
);

parameter_types! {
	pub const BlockHashCount: BlockNumber = 250;
	pub static ExistentialDeposit: u64 = 1;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type AccountData = pallet_balances::AccountData<Balance>;
	type Block = Block;
	type BlockHashCount = BlockHashCount;
	type HeaderExtensionBuilder = HeaderExtensionBuilder<Test>;
	type PalletInfo = PalletInfo;
	type Randomness = TestRandomness<Test>;
	type Extrinsic = Extrinsic;
}

parameter_types! {
	pub const TransactionByteFee: Balance = 1;
}

#[derive_impl(pallet_transaction_payment::config_preludes::TestDefaultConfig as pallet_transaction_payment::DefaultConfig)]
impl pallet_transaction_payment::Config for Test {
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	type WeightToFee = IdentityFee<Balance>;
}

parameter_types! {
	pub const MaxReserves: u32 = 2;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig)]
impl pallet_balances::Config for Test {
	type AccountStore = System;
	type Balance = Balance;
	type ExistentialDeposit = ExistentialDeposit;
}

impl pallet_utility::Config for Test {
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

#[derive_impl(da_control::config_preludes::TestDefaultConfig)]
impl da_control::Config for Test {}

/// Create new externalities for `System` module tests.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 10_000 * AVAIL), (2, 5_000 * AVAIL), (3, 1_000 * AVAIL)],
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	da_control::GenesisConfig::<Test> {
		app_keys: vec![
			(b"Avail".to_vec(), (1, 0)),
			(b"Reserved-1".to_vec(), (2, 1)),
			(b"A Brave New World".to_vec(), (2, 2)),
		],
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(storage);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

/// Create new externalities for Benchmarks
pub fn new_benchmark_ext() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 10_000 * AVAIL), (2, 5_000 * AVAIL), (3, 1_000 * AVAIL)],
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	da_control::GenesisConfig::<Test> {
		app_keys: vec![
			(b"Avail".to_vec(), (1, 0)),
			(b"Reserved-1".to_vec(), (2, 1)),
			(b"Reserved-2".to_vec(), (2, 2)),
			(b"Reserved-3".to_vec(), (2, 3)),
			(b"Reserved-4".to_vec(), (2, 4)),
			(b"Reserved-5".to_vec(), (2, 5)),
			(b"Reserved-6".to_vec(), (2, 6)),
			(b"Reserved-7".to_vec(), (2, 7)),
			(b"Reserved-8".to_vec(), (2, 8)),
			(b"Reserved-9".to_vec(), (2, 9)),
		],
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(storage);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
