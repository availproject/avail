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

use crate::{Runtime, Vector};
use frame_support::{pallet_prelude::*, traits::OnRuntimeUpgrade, weights::Weight};
use sp_runtime::Perbill;
#[cfg(feature = "try-runtime")]
use sp_runtime::TryRuntimeError;
#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

struct NominationPoolsMigrationV4OldPallet;
impl Get<Perbill> for NominationPoolsMigrationV4OldPallet {
	fn get() -> Perbill {
		Perbill::zero()
	}
}

const BRIDGE_OLD_PREFIX: &str = "Succinct";

/// Implements `OnRuntimeUpgrade` trait.
pub struct Migration {}
impl OnRuntimeUpgrade for Migration {
	fn on_runtime_upgrade() -> Weight {
		bridge::migrate::<Runtime, Vector, _>(BRIDGE_OLD_PREFIX)
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
		bridge::pre_migrate::<Vector, _>(BRIDGE_OLD_PREFIX);
		Ok(Vec::new())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), TryRuntimeError> {
		bridge::post_migrate::<Vector, _>(BRIDGE_OLD_PREFIX);
		Ok(())
	}
}

pub mod bridge {
	use crate::migration::Weight;
	use frame_support::pallet_prelude::PalletInfoAccess;
	use frame_support::traits::STORAGE_VERSION_STORAGE_KEY_POSTFIX;
	use sp_core::Get;
	use sp_io::hashing::twox_128;

	/// Migrate the entire storage of this pallet to a new prefix.
	pub fn migrate<T: frame_system::Config, P: PalletInfoAccess, N: AsRef<str>>(
		old_pallet_name: N,
	) -> Weight {
		let old_pallet_name = old_pallet_name.as_ref();
		let new_pallet_name = <P as PalletInfoAccess>::name();

		if new_pallet_name == old_pallet_name {
			log::info!(
				target: "runtime::vector",
				"New pallet name is equal to the old pallet name. No migration needs to be done.",
			);
			return Weight::from_parts(0, 0);
		}

		let old_pallet_prefix = twox_128(old_pallet_name.as_bytes());
		let old_pallet_prefix_iter = frame_support::storage::KeyPrefixIterator::new(
			old_pallet_prefix.to_vec(),
			old_pallet_prefix.to_vec(),
			|_| Ok(()),
		);

		if old_pallet_prefix_iter.count() < 1 {
			log::info!(
				target: "runtime::vector",
				"Old pallet name does not have any keys. No migration needs to be done.",
			);
			return Weight::from_parts(0, 0);
		}

		frame_support::storage::migration::move_pallet(
			old_pallet_name.as_bytes(),
			new_pallet_name.as_bytes(),
		);
		log_migration("migration", old_pallet_name, new_pallet_name);

		<T as frame_system::Config>::DbWeight::get().reads_writes(10, 10)
	}

	pub fn pre_migrate<P: PalletInfoAccess, N: AsRef<str>>(old_pallet_name: N) {
		let old_pallet_name = old_pallet_name.as_ref();
		let new_pallet_name = <P as PalletInfoAccess>::name();
		log_migration("pre-migration", old_pallet_name, new_pallet_name);

		if new_pallet_name == old_pallet_name {
			return;
		}

		let new_pallet_prefix = twox_128(new_pallet_name.as_bytes());
		let storage_version_key = twox_128(STORAGE_VERSION_STORAGE_KEY_POSTFIX);

		let mut new_pallet_prefix_iter = frame_support::storage::KeyPrefixIterator::new(
			new_pallet_prefix.to_vec(),
			new_pallet_prefix.to_vec(),
			|key| Ok(key.to_vec()),
		);

		// Ensure nothing except the storage_version_key is stored in the new prefix.
		assert!(new_pallet_prefix_iter.all(|key| key == storage_version_key));
	}

	pub fn post_migrate<P: PalletInfoAccess, N: AsRef<str>>(old_pallet_name: N) {
		let old_pallet_name = old_pallet_name.as_ref();
		let new_pallet_name = <P as PalletInfoAccess>::name();
		log_migration("post-migration", old_pallet_name, new_pallet_name);

		if new_pallet_name == old_pallet_name {
			return;
		}

		// Assert that nothing remains at the old prefix.
		let old_pallet_prefix = twox_128(old_pallet_name.as_bytes());
		let old_pallet_prefix_iter = frame_support::storage::KeyPrefixIterator::new(
			old_pallet_prefix.to_vec(),
			old_pallet_prefix.to_vec(),
			|_| Ok(()),
		);
		assert_eq!(old_pallet_prefix_iter.count(), 0);

		// NOTE: storage_version_key is already in the new prefix.
		let new_pallet_prefix = twox_128(new_pallet_name.as_bytes());
		let new_pallet_prefix_iter = frame_support::storage::KeyPrefixIterator::new(
			new_pallet_prefix.to_vec(),
			new_pallet_prefix.to_vec(),
			|_| Ok(()),
		);
		assert!(new_pallet_prefix_iter.count() >= 1);
	}

	fn log_migration(stage: &str, old_pallet_name: &str, new_pallet_name: &str) {
		log::info!(
			target: "runtime::vector",
			"{}, prefix: '{}' ==> '{}'",
			stage,
			old_pallet_name,
			new_pallet_name,
		);
	}
}
