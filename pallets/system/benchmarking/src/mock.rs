// This file is part of Substrate.

// Copyright (C) 2020-2022 Parity Technologies (UK) Ltd.
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

//! Mock file for system benchmarking.

#![cfg(test)]

use da_primitives::Header as DaHeader;
use frame_system::{
	header_builder::da::HeaderExtensionBuilder, mocking::MockUncheckedExtrinsic,
	test_utils::TestRandomness,
};
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, IdentityLookup};

type AccountId = u64;
type AccountIndex = u32;
type BlockNumber = u32;

type UncheckedExtrinsic = MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
	}
);

impl frame_system::Config for Test {
	type AccountData = ();
	type AccountId = AccountId;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockHashCount = ();
	type BlockLength = ();
	type BlockNumber = BlockNumber;
	type BlockWeights = ();
	type DbWeight = ();
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = DaHeader<BlockNumber, BlakeTwo256>;
	type HeaderExtensionBuilder = HeaderExtensionBuilder<Test>;
	type Index = AccountIndex;
	type Lookup = IdentityLookup<Self::AccountId>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
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
	type UncheckedExtrinsic = UncheckedExtrinsic;
	type Version = ();
}

impl crate::Config for Test {}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();
	sp_io::TestExternalities::new(t)
}