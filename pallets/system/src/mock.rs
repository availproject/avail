// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use frame_support::{parameter_types, traits::ConstU32};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};

use crate::{self as frame_system, test_utils::TestRandomness, *};

type UncheckedExtrinsic = mocking::MockUncheckedExtrinsic<Test>;
type Block = mocking::MockBlock<Test>;
type BlockNumber = u32;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
	}
);

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
const MAX_BLOCK_WEIGHT: Weight = Weight::from_ref_time(1024).set_proof_size(u64::MAX);

parameter_types! {
	pub Version: RuntimeVersion = RuntimeVersion {
		spec_name: sp_version::create_runtime_str!("test"),
		impl_name: sp_version::create_runtime_str!("system-test"),
		authoring_version: 1,
		spec_version: 1,
		impl_version: 1,
		apis: sp_version::create_apis_vec!([]),
		transaction_version: 1,
		state_version: 1,
	};
	pub const DbWeight: RuntimeDbWeight = RuntimeDbWeight {
		read: 10,
		write: 100,
	};
	pub RuntimeBlockWeights: limits::BlockWeights = limits::BlockWeights::builder()
		.base_block(Weight::from_ref_time(10))
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = Weight::from_ref_time(5);
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAX_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.base_extrinsic = Weight::from_ref_time(10);
			weights.max_total = Some(MAX_BLOCK_WEIGHT);
			weights.reserved = Some(
				MAX_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAX_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(Perbill::from_percent(0))
		.build_or_panic();
	pub RuntimeBlockLength: limits::BlockLength =
		limits::BlockLength::max_with_normal_ratio(1024, NORMAL_DISPATCH_RATIO);
}

parameter_types! {
	pub static Killed: Vec<u64> = vec![];
}

pub struct RecordKilled;
impl OnKilledAccount<u64> for RecordKilled {
	fn on_killed_account(who: &u64) { Killed::mutate(|r| r.push(*who)) }
}

impl Config for Test {
	type AccountData = u32;
	type AccountId = u64;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockHashCount = ConstU32<10>;
	type BlockLength = RuntimeBlockLength;
	type BlockNumber = u32;
	type BlockWeights = RuntimeBlockWeights;
	type DbWeight = DbWeight;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = da_primitives::Header<BlockNumber, BlakeTwo256>;
	type HeaderExtensionBuilder = frame_system::header_builder::da::HeaderExtensionBuilder<Test>;
	type Index = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type MaxConsumers = ConstU32<16>;
	type OnKilledAccount = RecordKilled;
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
	type Version = Version;
}

#[allow(dead_code)]
pub type SysEvent = frame_system::Event<Test>;

/// A simple call, which one doesn't matter.
#[allow(dead_code)]
pub const CALL: &<Test as Config>::RuntimeCall =
	&RuntimeCall::System(frame_system::Call::set_heap_pages { pages: 0u64 });

/// Create new externalities for `System` module tests.
#[allow(dead_code)]
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities = GenesisConfig::default()
		.system
		.build_storage::<Test>()
		.unwrap()
		.into();
	// Add to each test the initial weight of a block
	ext.execute_with(|| {
		System::register_extra_weight_unchecked(
			<Test as crate::Config>::BlockWeights::get().base_block,
			DispatchClass::Mandatory,
		)
	});
	ext
}
