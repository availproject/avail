#![cfg(test)]

use da_primitives::{
	currency::{Balance, AVL},
	Header,
};
use frame_support::{
	parameter_types,
	weights::{ConstantMultiplier, IdentityFee, Weight},
};
use frame_system::{header_builder::da::HeaderExtensionBuilder, test_utils::TestRandomness};
use pallet_transaction_payment::CurrencyAdapter;
use sp_core::H256;
use sp_runtime::{
	generic,
	traits::{BlakeTwo256, ConstU32, IdentityLookup},
};

use crate::{self as da_control, *};

/// An unchecked extrinsic type to be used in tests.
pub type UncheckedExtrinsic<T, Signature = (), Extra = ()> = generic::UncheckedExtrinsic<
	<T as frame_system::Config>::AccountId,
	<T as frame_system::Config>::RuntimeCall,
	Signature,
	Extra,
>;

/// An implementation of `sp_runtime::traits::Block` to be used in tests.
pub type Block<T> = generic::Block<
	Header<<T as frame_system::Config>::BlockNumber, sp_runtime::traits::BlakeTwo256>,
	UncheckedExtrinsic<T>,
>;

type BlockNumber = u32;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block<Test>,
		NodeBlock = Block<Test>,
		UncheckedExtrinsic = UncheckedExtrinsic<Test>,
	{
		System: frame_system,
		Balances: pallet_balances,
		TransactionPayment: pallet_transaction_payment,
		DataAvailability: da_control,
	}
);

parameter_types! {
	pub const BlockHashCount: BlockNumber = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(Weight::from_ref_time(1_024));
	pub static ExistentialDeposit: u64 = 0;
}

impl frame_system::Config for Test {
	type AccountData = pallet_balances::AccountData<Balance>;
	type AccountId = u64;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockNumber = BlockNumber;
	type BlockWeights = ();
	type DbWeight = ();
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;
	type HeaderExtensionBuilder = HeaderExtensionBuilder<Test>;
	type Index = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type MaxConsumers = ConstU32<16>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type PalletInfo = PalletInfo;
	type Randomness = TestRandomness<Test>;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SS58Prefix = ();
	type SubmittedDataExtractor = ();
	type SystemWeightInfo = ();
	type Version = ();
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
	type AccountStore = frame_system::Pallet<Test>;
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

parameter_types! {
	pub const MaxAppKeyLength: u32 = 32;
	pub const MaxAppDataLength: u32 = 16 * 1024; // 16K
	pub const MinBlockRows: BlockLengthRows = BlockLengthRows(32);
	pub const MaxBlockRows: BlockLengthRows = BlockLengthRows(1024);
	pub const MinBlockCols: BlockLengthColumns = BlockLengthColumns(32);
	pub const MaxBlockCols: BlockLengthColumns = kate::config::MAX_BLOCK_COLUMNS;
}

impl da_control::Config for Test {
	type BlockLenProposalId = u32;
	type MaxAppDataLength = MaxAppDataLength;
	type MaxAppKeyLength = MaxAppKeyLength;
	type MaxBlockCols = MaxBlockCols;
	type MaxBlockRows = MaxBlockRows;
	type MinBlockCols = MinBlockCols;
	type MinBlockRows = MinBlockRows;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

/// Create new externalities for `System` module tests.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 10_000 * AVL), (2, 5_000 * AVL), (3, 1_000 * AVL)],
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	da_control::GenesisConfig::<Test> {
		app_keys: vec![
			(b"Data Avail".to_vec(), AppKeyInfo {
				owner: 1,
				id: 0.into(),
			}),
			(b"Ethereum".to_vec(), AppKeyInfo {
				owner: 2,
				id: 1.into(),
			}),
			(b"Polygon".to_vec(), AppKeyInfo {
				owner: 2,
				id: 2.into(),
			}),
		],
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(storage);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
