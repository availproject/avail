#![cfg(test)]

use frame_support::weights::WeightToFee;
use frame_support::{derive_impl, weights::IdentityFee};
use frame_system::{
	mocking::MockUncheckedExtrinsic, native::hosted_header_builder::da::HeaderExtensionBuilder,
	test_utils::TestRandomness,
};
use pallet_transaction_payment::CurrencyAdapter;
use sp_runtime::{AccountId32, BuildStorage};

use crate::{self as da_control, *};

/// An unchecked extrinsic type to be used in tests.
type Extrinsic = MockUncheckedExtrinsic<Test>;
/// An implementation of `sp_runtime::traits::Block` to be used in tests.
type Block = frame_system::mocking::MockDaBlock<Test>;
type Balance = u64;

frame_support::construct_runtime!(
	pub struct Test {
		Timestamp: pallet_timestamp,
		System: frame_system,
		Utility: pallet_utility,
		Balances: pallet_balances,
		TransactionPayment: pallet_transaction_payment,
		DataAvailability: da_control,
		Vector: pallet_vector,
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
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
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
impl da_control::Config for Test {}

#[derive_impl(pallet_vector::config_preludes::TestDefaultConfig as pallet_vector::DefaultConfig)]
impl pallet_vector::Config for Test {
	type TimeProvider = Timestamp;
	type Currency = Balances;
}

#[derive_impl(pallet_timestamp::config_preludes::TestDefaultConfig as pallet_timestamp::DefaultConfig)]
impl pallet_timestamp::Config for Test {}

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
