#![cfg(test)]

use avail_core::currency::{Balance, AVL};
use frame_support::{
	derive_impl, parameter_types,
	weights::{ConstantMultiplier, IdentityFee},
};
use frame_system::{
	header_builder::da::HeaderExtensionBuilder, mocking::MockUncheckedExtrinsic,
	test_utils::TestRandomness,
};
use pallet_transaction_payment::CurrencyAdapter;
use sp_runtime::{traits::ConstU32, BuildStorage};

use crate::{self as da_control, *};

/// An unchecked extrinsic type to be used in tests.
type UncheckedExtrinsic = MockUncheckedExtrinsic<Test>;

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
	type BaseCallFilter = frame_support::traits::Everything;
	type Block = Block;
	type BlockHashCount = BlockHashCount;
	type HeaderExtensionBuilder = HeaderExtensionBuilder<Test>;
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
	pub const TransactionByteFee: Balance = 1;
	pub const OperationalFeeMultiplier: u8 = 5;
}
impl pallet_transaction_payment::Config for Test {
	type FeeMultiplierUpdate = ();
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type RuntimeEvent = RuntimeEvent;
	type WeightToFee = IdentityFee<Balance>;
}

parameter_types! {
	pub const MaxReserves: u32 = 2;
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

impl pallet_utility::Config for Test {
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

parameter_types! {
	pub const MaxAppKeyLength: u32 = 32;
	pub const MaxAppDataLength: u32 = 512 * 1024; // 512 Kb
	pub const MinBlockRows: BlockLengthRows = BlockLengthRows(32);
	pub const MaxBlockRows: BlockLengthRows = BlockLengthRows(1024);
	pub const MinBlockCols: BlockLengthColumns = BlockLengthColumns(32);
	pub const MaxBlockCols: BlockLengthColumns = kate::config::MAX_BLOCK_COLUMNS;
}

#[derive_impl(da_control::config_preludes::TestDefaultConfig)]
impl da_control::Config for Test {
	type MaxAppDataLength = MaxAppDataLength;
	type MaxAppKeyLength = MaxAppKeyLength;
	type MaxBlockCols = MaxBlockCols;
	type MaxBlockRows = MaxBlockRows;
	type MinBlockCols = MinBlockCols;
	type MinBlockRows = MinBlockRows;
	type RuntimeEvent = RuntimeEvent;
}

/// Create new externalities for `System` module tests.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 10_000 * AVL), (2, 5_000 * AVL), (3, 1_000 * AVL)],
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	da_control::GenesisConfig::<Test> {
		app_keys: vec![
			(b"Data Avail".to_vec(), (1, 0)),
			(b"Ethereum".to_vec(), (2, 1)),
			(b"Polygon".to_vec(), (2, 2)),
		],
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(storage);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
