use da_primitives::NORMAL_DISPATCH_RATIO;
use frame_support::{
	construct_runtime, parameter_types,
	traits::{Get, Randomness},
	weights::{constants::WEIGHT_PER_SECOND, IdentityFee, RuntimeDbWeight, Weight},
};
use frame_system::{CheckEra, CheckNonce, CheckWeight};
use pallet_transaction_payment::CurrencyAdapter;
use sp_runtime::{
	generic::Block as SPBlock,
	traits::{BlakeTwo256, IdentityLookup, TrailingZeroInput},
};
use sp_std::{cell::RefCell, marker::PhantomData};

pub mod custom;
pub mod test_xt;

// Common Runtime Types
//

pub type AccountId = u64;
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Moment = u64;
pub type Header = da_primitives::Header<BlockNumber, BlakeTwo256>;
pub type Signature = sp_runtime::testing::sr25519::Signature;
pub type TestXt = test_xt::TestXt<Call, SignedExtra>;
pub type UncheckedExtrinsic = TestXt;
pub type Block = SPBlock<Header, UncheckedExtrinsic>;
pub type SignedExtra = (
	CheckEra<Runtime>,
	CheckNonce<Runtime>,
	CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
	da_control::CheckAppId<Runtime>,
);

/// Provides an implementation of [`frame_support::traits::Randomness`] that should only be used in
/// tests!
pub struct TestRandomness<T>(PhantomData<T>);

impl<Output: codec::Decode + Default, T> Randomness<Output, T::BlockNumber> for TestRandomness<T>
where
	T: frame_system::Config,
{
	fn random(subject: &[u8]) -> (Output, T::BlockNumber) {
		(
			Output::decode(&mut TrailingZeroInput::new(subject)).unwrap_or_default(),
			frame_system::Pallet::<T>::block_number(),
		)
	}
}

// Parameters

// Will contain `true` when the custom runtime logic was called.
pub const CUSTOM_ON_RUNTIME_KEY: &[u8] = &*b":custom:on_runtime";

/// This determines the average expected block time that we are targetting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_babe` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 20000;

pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 10 * MINUTES;
pub const EPOCH_DURATION_IN_SLOTS: u64 = {
	const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;

	(EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as u64
};

/// We allow for 2 seconds of compute with a 6 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = 2 * WEIGHT_PER_SECOND;

parameter_types! {
	pub const BlockHashCount: BlockNumber = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
			frame_system::limits::BlockWeights::with_sensible_defaults(MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO);
	pub const DbWeight: RuntimeDbWeight = RuntimeDbWeight {
		read: 10,
		write: 100,
	};
	pub const TransactionByteFee: Balance = 0;
	pub const OperationalFeeMultiplier: u8 = 5;
	pub const ExistentialDeposit: Balance = 1;
	pub const MaxAuthorities: u32 = 100;
	pub const SessionsPerEra: sp_staking::SessionIndex = 6;
	pub const BondingDuration: pallet_staking::EraIndex = 24 * 28;
	pub const EpochDuration: u64 = EPOCH_DURATION_IN_SLOTS;
	pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
	pub const ReportLongevity: u64 =
		BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();

	// DA Control
	pub const MaxAppKeyLength :u32 = 64;
	pub const MaxAppDataLength :u32 = 16 * 1024; // 16 Kb
	pub const MinBlockRows: u32 = 32;
	pub const MaxBlockRows: u32 = 1024;
	pub const MinBlockCols: u32 = 32;
	pub const MaxBlockCols: u32 = kate::config::MAX_BLOCK_COLUMNS;
}

pub struct RuntimeVersion;
impl Get<sp_version::RuntimeVersion> for RuntimeVersion {
	fn get() -> sp_version::RuntimeVersion { RUNTIME_VERSION.with(|v| v.borrow().clone()) }
}

thread_local! {
	pub static RUNTIME_VERSION: RefCell<sp_version::RuntimeVersion> =
		Default::default();
}

// Runtime
//
impl frame_system::Config for Runtime {
	type AccountData = pallet_balances::AccountData<Balance>;
	type AccountId = AccountId;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockNumber = BlockNumber;
	type BlockWeights = BlockWeights;
	type Call = Call;
	type DbWeight = ();
	type Event = Event;
	type Hash = sp_core::H256;
	type Hashing = BlakeTwo256;
	type Header = da_primitives::Header<Self::BlockNumber, Self::Hashing>;
	type HeaderExtensionBuilder = frame_system::header_builder::da::HeaderExtensionBuilder<Runtime>;
	type Index = u64;
	type Lookup = IdentityLookup<u64>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type Origin = Origin;
	type PalletInfo = PalletInfo;
	type Randomness = TestRandomness<Runtime>;
	type SS58Prefix = ();
	type SubmittedDataExtractor = ();
	type SystemWeightInfo = ();
	type Version = RuntimeVersion;
}

impl pallet_transaction_payment::Config for Runtime {
	type FeeMultiplierUpdate = ();
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type TransactionByteFee = TransactionByteFee;
	type WeightToFee = IdentityFee<Balance>;
}

impl pallet_balances::Config for Runtime {
	type AccountStore = System;
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}

impl da_control::Config for Runtime {
	type BlockLenProposalId = u32;
	type Event = Event;
	type MaxAppDataLength = MaxAppDataLength;
	type MaxAppKeyLength = MaxAppKeyLength;
	type MaxBlockCols = MaxBlockCols;
	type MaxBlockRows = MaxBlockRows;
	type MinBlockCols = MinBlockCols;
	type MinBlockRows = MinBlockRows;
	type WeightInfo = da_control::weights::SubstrateWeight<Runtime>;
}

impl custom::custom::Config for Runtime {}

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		TransactionPayment: pallet_transaction_payment::{Pallet, Storage},
		Custom: custom::custom::{Pallet, Call, ValidateUnsigned, Inherent},

		DataAvailability: da_control,
	}
);
