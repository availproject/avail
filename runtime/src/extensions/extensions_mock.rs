#![cfg(test)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::RuntimeDebug;
use frame_support::parameter_types;
use frame_support::traits::fungible::HoldConsideration;
use frame_support::traits::EqualPrivilegeOnly;
use frame_support::traits::InstanceFilter;
use frame_support::traits::LinearStoragePrice;
use frame_support::weights::WeightToFee;
use frame_support::{derive_impl, weights::IdentityFee};
use frame_system::EnsureRoot;
use frame_system::{
	mocking::MockUncheckedExtrinsic, native::hosted_header_builder::da::HeaderExtensionBuilder,
	test_utils::TestRandomness,
};
use pallet_transaction_payment::FungibleAdapter;
use sp_core::ConstU32;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::traits::Convert;
use sp_runtime::Perbill;
use sp_runtime::{AccountId32, BuildStorage};
use sp_std::marker::PhantomData;

use crate::Weight;
use da_control::DefaultConfig;

/// An unchecked extrinsic type to be used in tests.
type Extrinsic = MockUncheckedExtrinsic<Test>;
/// An implementation of `sp_runtime::traits::Block` to be used in tests.
type Block = frame_system::mocking::MockDaBlock<Test>;
type Balance = u64;

pub struct IdentitySome<A>(PhantomData<A>);
impl<A> Convert<A, Option<A>> for IdentitySome<A> {
	fn convert(a: A) -> Option<A> {
		Some(a)
	}
}
pub struct DummyValidatorSet<T>(PhantomData<T>);
impl<T> frame_support::traits::ValidatorSet<<T as frame_system::Config>::AccountId>
	for DummyValidatorSet<T>
where
	T: frame_system::Config,
{
	type ValidatorId = <T as frame_system::Config>::AccountId;
	type ValidatorIdOf = IdentitySome<<T as frame_system::Config>::AccountId>;

	fn session_index() -> sp_staking::SessionIndex {
		0
	}
	fn validators() -> Vec<Self::ValidatorId> {
		Vec::new()
	}
}

impl<T>
	frame_support::traits::ValidatorSetWithIdentification<<T as frame_system::Config>::AccountId>
	for DummyValidatorSet<T>
where
	T: frame_system::Config,
{
	type Identification = <T as frame_system::Config>::AccountId;
	type IdentificationOf = IdentitySome<<T as frame_system::Config>::AccountId>;
}

frame_support::construct_runtime!(
	pub struct Test {
		Timestamp: pallet_timestamp,
		System: frame_system,
		Utility: pallet_utility,
		Balances: pallet_balances,
		TransactionPayment: pallet_transaction_payment,
		DataAvailability: da_control,
		Vector: pallet_vector,
		Multisig: pallet_multisig,
		Proxy: pallet_proxy,
		Scheduler: pallet_scheduler,
		Preimage: pallet_preimage,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type AccountData = pallet_balances::AccountData<Balance>;
	type Block = Block;
	type HeaderExtensionBuilder = HeaderExtensionBuilder<Test>;
	type PalletInfo = PalletInfo;
	type Randomness = TestRandomness<Test>;
	type Extrinsic = Extrinsic;
	type AccountId = AccountId32;
	type Lookup = sp_runtime::traits::IdentityLookup<AccountId32>;
}

pub struct TestLengthToFeeU64;

impl WeightToFee for TestLengthToFeeU64 {
	type Balance = u64;

	fn weight_to_fee(_weight: &Weight) -> Self::Balance {
		0
	}
}

#[derive_impl(pallet_transaction_payment::config_preludes::TestDefaultConfig as pallet_transaction_payment::DefaultConfig)]
impl pallet_transaction_payment::Config for Test {
	type LengthToFee = TestLengthToFeeU64;
	type OnChargeTransaction = FungibleAdapter<Balances, ()>;
	type WeightToFee = IdentityFee<Balance>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig)]
impl pallet_balances::Config for Test {
	type AccountStore = System;
}

impl pallet_utility::Config for Test {
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

#[derive_impl(da_control::config_preludes::TestDefaultConfig)]
impl da_control::Config for Test {
	type Currency = ();
	type ValidatorSet = DummyValidatorSet<Test>;
	type ReportOffence = ();
}

#[derive_impl(pallet_vector::config_preludes::TestDefaultConfig as pallet_vector::DefaultConfig)]
impl pallet_vector::Config for Test {
	type TimeProvider = Timestamp;
	type Currency = Balances;
}

#[derive_impl(pallet_timestamp::config_preludes::TestDefaultConfig as pallet_timestamp::DefaultConfig)]
impl pallet_timestamp::Config for Test {}

parameter_types! {
	// One storage item; key size 32, value size 8; .
	pub const ProxyDepositBase: Balance = 10 * 1;
	// Additional storage item size of 33 bytes.
	pub const ProxyDepositFactor: Balance = 3 * 1;
	pub const AnnouncementDepositBase: Balance = 10 * 1;
	pub const AnnouncementDepositFactor: Balance = 5 * 1;
}

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Encode,
	Decode,
	RuntimeDebug,
	MaxEncodedLen,
	scale_info::TypeInfo,
)]
pub enum ProxyType {
	Any,
	NonTransfer,
	Governance,
	Staking,
	IdentityJudgement,
	NominationPools,
}
impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}
impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, _c: &RuntimeCall) -> bool {
		match self {
			_ => true,
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			(ProxyType::NonTransfer, _) => true,
			_ => false,
		}
	}
}

impl pallet_proxy::Config for Test {
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ProxyType = ProxyType;
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type MaxProxies = ConstU32<32>;
	type WeightInfo = ();
	type MaxPending = ConstU32<32>;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
}

parameter_types! {
	pub MaximumSchedulerWeight: Weight = (Perbill::from_percent(80) * 1).into();
	pub const MaxScheduledPerBlock: u32 = 50;
}

impl pallet_scheduler::Config for Test {
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MaxScheduledPerBlock = ConstU32<512>;
	type MaximumWeight = MaximumSchedulerWeight;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type PalletsOrigin = OriginCaller;
	type Preimages = Preimage;
	type RuntimeOrigin = RuntimeOrigin;
	type ScheduleOrigin = EnsureRoot<AccountId32>;
	type WeightInfo = ();
}

parameter_types! {
	pub const PreimageBaseDeposit: Balance = 1 * 1;
	// One cent: $10,000 / MB
	pub const PreimageByteDeposit: Balance = 1 * 1;
	pub const PreimageHoldReason: RuntimeHoldReason = RuntimeHoldReason::Preimage(pallet_preimage::HoldReason::Preimage);
}

impl pallet_preimage::Config for Test {
	type Consideration = HoldConsideration<
		AccountId32,
		Balances,
		PreimageHoldReason,
		LinearStoragePrice<PreimageBaseDeposit, PreimageByteDeposit, Balance>,
	>;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<AccountId32>;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

parameter_types! {
	pub const DepositBase: Balance = 2 * 1;
	// Additional storage item size of 32 bytes.
	pub const DepositFactor: Balance = 5 * 1;
}

impl pallet_multisig::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type MaxSignatories = ConstU32<100>;
	#[doc = " Weight information for extrinsics in this pallet."]
	type WeightInfo = ();
}

fn u8_to_account_id(value: u8) -> AccountId32 {
	let mut account = [0u8; 32];
	account[0] = value;

	AccountId32::new(account)
}

/// Create new externalities for `System` module tests.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();

	let alice = u8_to_account_id(1);

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(alice.clone(), 1_000_000u64)],
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	da_control::GenesisConfig::<Test> {
		app_keys: vec![
			(b"Avail".to_vec(), (alice.clone(), 0)),
			(b"Kinder".to_vec(), (alice, 1)),
		],
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(storage);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
