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

use codec::Encode;
use frame_support::{derive_impl, traits::ConstU32};
use frame_system::{
	mocking::MockUncheckedExtrinsic, native::hosted_header_builder::da::HeaderExtensionBuilder,
	test_utils::TestRandomness,
};
use sp_runtime::BuildStorage;

type Extrinsic = MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockDaBlock<Test>;

frame_support::construct_runtime!(
	pub struct Test {
		System: frame_system,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type Block = Block;
	type BlockHashCount = ();
	type HeaderExtensionBuilder = HeaderExtensionBuilder<Test>;
	type OnSetCode = ();
	type PalletInfo = PalletInfo;
	type Randomness = TestRandomness<Test>;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type MaxDiffAppIdPerBlock = ConstU32<1_024>;
	type MaxTxPerAppIdPerBlock = ConstU32<8_192>;
	type HeaderExtensionDataFilter = ();
	type Extrinsic = Extrinsic;
}

impl crate::Config for Test {}

struct MockedReadRuntimeVersion(Vec<u8>);

impl sp_core::traits::ReadRuntimeVersion for MockedReadRuntimeVersion {
	fn read_runtime_version(
		&self,
		_wasm_code: &[u8],
		_ext: &mut dyn sp_externalities::Externalities,
	) -> Result<Vec<u8>, String> {
		Ok(self.0.clone())
	}
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = RuntimeGenesisConfig::default()
		.system
		.build_storage()
		.expect("Genesis build should work");

	let version = sp_version::RuntimeVersion {
		spec_name: "".into(),
		spec_version: 10,
		impl_version: 420,
		..Default::default()
	};
	let mut ext = sp_io::TestExternalities::new(t);
	let read_runtime_version = MockedReadRuntimeVersion(version.encode());
	ext.register_extension(sp_core::traits::ReadRuntimeVersionExt::new(
		read_runtime_version,
	));

	ext
}
