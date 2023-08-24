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

use frame_support::{pallet_prelude::*, traits::OnRuntimeUpgrade};
use pallet_nomination_pools::{
	MaxPoolMembers, MaxPoolMembersPerPool, MaxPools, MinCreateBond, MinJoinBond, Pallet,
};
#[cfg(feature = "try-runtime")]
use sp_runtime::TryRuntimeError;
#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

use crate::Weight;

/// Implements `OnRuntimeUpgrade` trait.
pub struct Migration {}

impl OnRuntimeUpgrade for Migration {
	fn on_runtime_upgrade() -> Weight { nomination_pools::migrate() }

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> { Ok(nomination_pools::pre_upgrade()?) }

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(prev_count: Vec<u8>) -> Result<(), TryRuntimeError> {
		Ok(nomination_pools::post_upgrade(prev_count)?)
	}
}

mod nomination_pools {
	use super::*;
	use crate::Runtime;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	/// Wrapper for all migrations of this pallet.
	pub(crate) fn migrate() -> Weight {
		let onchain_version = Pallet::<Runtime>::on_chain_storage_version();
		let mut weight: Weight = Weight::zero();

		if onchain_version < 1 {
			weight = weight.saturating_add(v0_to_v1::migrate());
		}

		STORAGE_VERSION.put::<Pallet<Runtime>>();
		weight.saturating_add(<Runtime as frame_system::Config>::DbWeight::get().writes(1))
	}

	#[cfg(feature = "try-runtime")]
	pub(crate) fn pre_upgrade() -> Result<Vec<u8>, &'static str> { Ok(sp_std::vec![]) }

	#[cfg(feature = "try-runtime")]
	pub(crate) fn post_upgrade(_state: Vec<u8>) -> Result<(), &'static str> {
		use crate::constants::nomination_pools::*;

		ensure!(
			MinJoinBond::<Runtime>::get() == MIN_JOIN_BOND,
			"Expected `nomination_pools::MinJoinBond == 1 * AVL`"
		);
		ensure!(
			MinCreateBond::<Runtime>::get() == MIN_CREATE_BOND,
			"Expected `nomination_pools::MinCreateBond == 10 * AVL`"
		);
		ensure!(
			MaxPools::<Runtime>::get() == Some(MAX_POOLS),
			"Expected `nomination_pools::MaxPools == 16`"
		);
		ensure!(
			MaxPoolMembersPerPool::<Runtime>::get() == Some(MAX_MEMBERS_PER_POOL),
			"Expected `nomination_pools::MaxPoolMembersPerPool == 100`"
		);
		ensure!(
			MaxPoolMembers::<Runtime>::get() == Some(MAX_MEMBERS),
			"Expected `nomination_pools::MaxMembers == 1600`"
		);

		Ok(())
	}

	mod v0_to_v1 {
		use super::*;
		use crate::constants::nomination_pools::*;

		/// It sets `min_create_bond = 10 AVL` and
		pub(crate) fn migrate() -> Weight {
			log::info!(target: "runtime::migration", "Nomination pools migration from V0 to V1");
			MinJoinBond::<Runtime>::put(MIN_JOIN_BOND);
			MinCreateBond::<Runtime>::put(MIN_CREATE_BOND);
			MaxPools::<Runtime>::put(MAX_POOLS);
			MaxPoolMembersPerPool::<Runtime>::put(MAX_MEMBERS_PER_POOL);
			MaxPoolMembers::<Runtime>::put(MAX_MEMBERS);

			<Runtime as frame_system::Config>::DbWeight::get().writes(5u64)
		}
	}
}
