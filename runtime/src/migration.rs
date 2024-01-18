// This file is part of Data-Availability.

// Copyright (C) 2022
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

use crate::Runtime;
use frame_support::{traits::OnRuntimeUpgrade, weights::Weight};
#[cfg(feature = "try-runtime")]
use sp_runtime::TryRuntimeError;
#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

/// Implements `OnRuntimeUpgrade` trait.
pub struct Migration {}
impl OnRuntimeUpgrade for Migration {
	fn on_runtime_upgrade() -> Weight {
		nomad::on_runtime_upgrade()
		// Weight::zero()
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
		Ok(Vec::new())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), TryRuntimeError> {
		nomad::post_upgrade(_state)
		// Ok(())
	}
}

pub mod nomad {
	use super::*;
	#[allow(deprecated)]
	use frame_support::storage::unhashed::{clear_prefix, contains_prefixed_key};

	fn remove_storage(prefix: &[u8]) {
		if !contains_prefixed_key(prefix) {
			log::info!("No storage found for {:?}", prefix);
			return;
		}

		let res = clear_prefix(prefix, None, None);
		if res.maybe_cursor.is_none() {
			log::info!("Successfully removed {:?} storage", prefix)
		} else {
			log::info!("Failed to remove {:?} storage", prefix)
		}
	}

	pub fn on_runtime_upgrade() -> Weight {
		// Instead of checking what version we are on we can just query and see if the storage exists
		// NomadHome 			0x6ed9b11ccf1ed0fdd9fa3321cf0640d1
		// NomadUpateManager 	0x01e3d1ac52ab8d7b9bf5f1d707ae496e
		// NomadDABrige 		doesn't have storage

		remove_storage(&sp_io::hashing::twox_128(b"NomadHome"));
		remove_storage(&sp_io::hashing::twox_128(b"NomadUpdaterManager"));

		<Runtime as frame_system::Config>::DbWeight::get().reads_writes(1, 1)
	}
	#[cfg(feature = "try-runtime")]
	pub fn post_upgrade(_state: Vec<u8>) -> Result<(), TryRuntimeError> {
		if contains_prefixed_key(&sp_io::hashing::twox_128(b"NomadHome")) {
			return Err(TryRuntimeError::Other("NomadHome storage was not deleted"));
		}
		if contains_prefixed_key(&sp_io::hashing::twox_128(b"NomadUpdaterManager")) {
			return Err(TryRuntimeError::Other(
				"NomadUpdaterManager storage was not deleted",
			));
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::{Runtime, System};
	use frame_support::{
		migration::{get_storage_value, put_storage_value},
		traits::OnRuntimeUpgrade,
	};
	use sp_runtime::BuildStorage;

	pub fn new_test_ext() -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::<Runtime>::default()
			.build_storage()
			.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}

	#[test]
	fn nomad_test() {
		new_test_ext().execute_with(|| {
			put_storage_value(b"NomadHome", b"Item", b"", 100u32);
			put_storage_value(b"NomadUpdaterManager", b"Item", b"", 100u32);

			assert!(get_storage_value::<u32>(b"NomadHome", b"Item", b"").is_some());
			assert!(get_storage_value::<u32>(b"NomadUpdaterManager", b"Item", b"").is_some());

			super::Migration::on_runtime_upgrade();

			assert!(get_storage_value::<u32>(b"NomadHome", b"Item", b"").is_none());
			assert!(get_storage_value::<u32>(b"NomadUpdaterManager", b"Item", b"").is_none());
		});
	}
}
