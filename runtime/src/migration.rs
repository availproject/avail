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
use sp_runtime::Perbill;
#[cfg(feature = "try-runtime")]
use sp_runtime::TryRuntimeError;
#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

use crate::{Bounties, NominationPools, Runtime, Weight};

struct NominationPoolsMigrationV4OldPallet;
impl Get<Perbill> for NominationPoolsMigrationV4OldPallet {
	fn get() -> Perbill {
		Perbill::zero()
	}
}

/// Implements `OnRuntimeUpgrade` trait.
pub struct Migration {}
impl OnRuntimeUpgrade for Migration {
	fn on_runtime_upgrade() -> Weight {
		let weight1 = pallet_im_online::migration::v1::Migration::<Runtime>::on_runtime_upgrade();
		let weight2 = pallet_offences::migration::v1::MigrateToV1::<Runtime>::on_runtime_upgrade();
		let weight3 = nomination_pools::v1_to_v3::on_runtime_upgrade();
		let weight4 = pallet_nomination_pools::migration::v4::MigrateV3ToV5::<
			Runtime,
			NominationPoolsMigrationV4OldPallet,
		>::on_runtime_upgrade();
		let weight5 = scheduler::remove_corrupt_agenda_and_v3_to_v4::on_runtime_upgrade();
		let weight6 = bounties::v1_to_v4::on_runtime_upgrade();
		let weight7 = staking::on_runtime_upgrade();
		let weight8 = democracy::on_runtime_upgrade();

		weight1
			.saturating_add(weight2)
			.saturating_add(weight3)
			.saturating_add(weight4)
			.saturating_add(weight5)
			.saturating_add(weight6)
			.saturating_add(weight7)
			.saturating_add(weight8)
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
		bounties::v1_to_v4::pre_upgrade()?;
		nomination_pools::v1_to_v3::pre_upgrade()?;

		let state1 = pallet_im_online::migration::v1::Migration::<Runtime>::pre_upgrade()?;
		let state2 = pallet_offences::migration::v1::MigrateToV1::<Runtime>::pre_upgrade()?;
		let state3 = pallet_nomination_pools::migration::v4::MigrateV3ToV5::<
			Runtime,
			NominationPoolsMigrationV4OldPallet,
		>::pre_upgrade()?;
		let state4 = scheduler::remove_corrupt_agenda_and_v3_to_v4::pre_upgrade()?;

		let combined_state = (state1, state2, state3, state4).encode();

		Ok(combined_state)
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), TryRuntimeError> {
		bounties::v1_to_v4::post_upgrade(sp_std::vec![])?;
		nomination_pools::v1_to_v3::post_upgrade(sp_std::vec![])?;

		let (state1, state2, state3, state4): (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) =
			Decode::decode(&mut &state[..]).expect("Encoded state is always a valid tuple; qed");

		pallet_im_online::migration::v1::Migration::<Runtime>::post_upgrade(state1)?;
		pallet_offences::migration::v1::MigrateToV1::<Runtime>::post_upgrade(state2)?;
		pallet_nomination_pools::migration::v4::MigrateV3ToV5::<
			Runtime,
			NominationPoolsMigrationV4OldPallet,
		>::post_upgrade(state3)?;
		scheduler::remove_corrupt_agenda_and_v3_to_v4::post_upgrade(state4)?;

		Ok(())
	}
}

mod scheduler {
	use super::*;

	// We remove the calls that got scheduler but never triggered, they are not decodable and make the migration fail
	pub mod remove_corrupt_agenda_and_v3_to_v4 {
		use super::*;
		use crate::Runtime;

		// This `436_320` should get executed, if not migration should not happen, if it is, then this can safely be ignored
		const CORRUPTED_AGENDAS: [u32; 5] = [38_674, 86_664, 124_473, 128_931, 436_320];

		#[cfg(feature = "try-runtime")]
		pub fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
			let agendas = pallet_scheduler::Agenda::<Runtime>::iter().count() as u32;
			let mut corrupted_agendas: u32 = 0;
			for block in &CORRUPTED_AGENDAS {
				if pallet_scheduler::Agenda::<Runtime>::contains_key(block) {
					corrupted_agendas += 1;
				}
			}

			log::info!("Number of agendas: {}", agendas);
			log::info!("Number of corrupted agendas: {}", corrupted_agendas);

			// This print out some error logs cause 4 corrupted agendas cannot and should not be migrated
			// let _ = pallet_scheduler::migration::v3::MigrateToV4::<Runtime>::pre_upgrade();

			let encoded_data = (agendas, corrupted_agendas).encode();
			Ok(encoded_data)
		}

		pub fn on_runtime_upgrade() -> Weight {
			let weight: Weight = Weight::zero();

			for block in &CORRUPTED_AGENDAS {
				if pallet_scheduler::Agenda::<Runtime>::contains_key(block) {
					pallet_scheduler::Agenda::<Runtime>::remove(block);
					weight.saturating_add(
						<Runtime as frame_system::Config>::DbWeight::get().reads_writes(1, 1),
					);
					log::info!("Removed agenda at block: {:?}", block);
				} else {
					weight.saturating_add(
						<Runtime as frame_system::Config>::DbWeight::get().reads(1),
					);
					log::info!("No agenda at block: {:?}", block);
				}
			}

			let migration_weight =
				pallet_scheduler::migration::v3::MigrateToV4::<Runtime>::on_runtime_upgrade();

			weight.saturating_add(migration_weight)
		}

		#[cfg(feature = "try-runtime")]
		pub fn post_upgrade(state: Vec<u8>) -> Result<(), TryRuntimeError> {
			let (agendas, corrupted_agendas_blocks): (u32, u32) =
				Decode::decode(&mut &state[..]).expect("Encoded data is always a valid tuple; qed");
			let valid_agendas = agendas - corrupted_agendas_blocks;
			let current_agendas = pallet_scheduler::Agenda::<Runtime>::iter().count() as u32;

			ensure!(
				current_agendas == valid_agendas,
				"Corrupted agendas did not get cleared"
			);
			let encoded_data = (valid_agendas).encode();
			pallet_scheduler::migration::v3::MigrateToV4::<Runtime>::post_upgrade(encoded_data)?;

			Ok(())
		}
	}
}

mod bounties {
	use super::*;

	// Migrations that set `StorageVersion`s which was missed to set.
	pub mod v1_to_v4 {
		use super::*;

		#[cfg(feature = "try-runtime")]
		pub fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
			let storage_version = Bounties::on_chain_storage_version();
			log::info!("Old bounties storage version: {:?}", storage_version);
			Ok(sp_std::vec![])
		}

		// Here we update the storage version to be consistent even if pallet was initialized correctly
		pub fn on_runtime_upgrade() -> Weight {
			let storage_version = Bounties::on_chain_storage_version();
			if storage_version < 4 {
				StorageVersion::new(4).put::<Bounties>();
			}

			<Runtime as frame_system::Config>::DbWeight::get().reads_writes(1, 1)
		}

		#[cfg(feature = "try-runtime")]
		pub fn post_upgrade(_: Vec<u8>) -> Result<(), &'static str> {
			let storage_version = Bounties::on_chain_storage_version();
			log::info!("New bounties storage version: {:?}", storage_version);
			Ok(())
		}
	}
}

mod nomination_pools {
	use super::*;

	#[allow(dead_code)]
	pub mod v0_to_v1 {
		use super::*;
		use crate::{constants::nomination_pools::*, Runtime};

		/// The current storage version.
		pub const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

		#[cfg(feature = "try-runtime")]
		pub(crate) fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
			Ok(sp_std::vec![])
		}

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

		/// It sets `min_create_bond = 10 AVL` and
		pub(crate) fn on_runtime_upgrade() -> Weight {
			log::info!(target: "runtime::migration", "Nomination pools migration from V0 to V1");
			MinJoinBond::<Runtime>::put(MIN_JOIN_BOND);
			MinCreateBond::<Runtime>::put(MIN_CREATE_BOND);
			MaxPools::<Runtime>::put(MAX_POOLS);
			MaxPoolMembersPerPool::<Runtime>::put(MAX_MEMBERS_PER_POOL);
			MaxPoolMembers::<Runtime>::put(MAX_MEMBERS);

			<Runtime as frame_system::Config>::DbWeight::get().writes(5u64)
		}

		/// Wrapper for all migrations of this pallet.
		pub(crate) fn migrate() -> Weight {
			let onchain_version = Pallet::<Runtime>::on_chain_storage_version();
			let mut weight: Weight = Weight::zero();

			if onchain_version < 1 {
				weight = weight.saturating_add(v0_to_v1::on_runtime_upgrade());
			}

			v0_to_v1::STORAGE_VERSION.put::<Pallet<Runtime>>();
			weight.saturating_add(<Runtime as frame_system::Config>::DbWeight::get().writes(1))
		}
	}

	// Migrations that set `StorageVersion`s which was missed to set.
	pub mod v1_to_v3 {
		use super::*;

		#[cfg(feature = "try-runtime")]
		pub fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
			let storage_version = NominationPools::on_chain_storage_version();
			log::info!(
				"Old nomination pools storage version: {:?}",
				storage_version
			);
			Ok(sp_std::vec![])
		}

		// Here we update the storage version to be consistent even if pallet was initialized correctly
		pub fn on_runtime_upgrade() -> Weight {
			let storage_version = NominationPools::on_chain_storage_version();
			if storage_version < 3 {
				StorageVersion::new(3).put::<NominationPools>();
			}

			<Runtime as frame_system::Config>::DbWeight::get().reads_writes(1, 1)
		}

		#[cfg(feature = "try-runtime")]
		pub fn post_upgrade(_: Vec<u8>) -> Result<(), &'static str> {
			let storage_version = NominationPools::on_chain_storage_version();
			log::info!(
				"New nomination pools storage version: {:?}",
				storage_version
			);
			Ok(())
		}
	}
}

pub mod staking {
	use super::*;

	pub fn on_runtime_upgrade() -> Weight {
		let current_state = pallet_staking::MinimumValidatorCount::<Runtime>::get();
		let target_state = 1;
		if current_state > 1 {
			log::info!(
				"Changing minimum validator count from {} to {}",
				current_state,
				target_state
			);
			pallet_staking::MinimumValidatorCount::<Runtime>::set(target_state);
		} else {
			log::info!("No change was applied to MinimumValidatorCount storage.")
		}

		<Runtime as frame_system::Config>::DbWeight::get().reads_writes(1, 1)
	}
}

pub mod democracy {
	use super::*;
	#[allow(deprecated)]
	use frame_support::storage::unhashed::kill_prefix;
	use sp_io::KillStorageResult;

	/// It sets `min_create_bond = 10 AVL` and
	pub fn on_runtime_upgrade() -> Weight {
		#[allow(deprecated)]
		let res1 = kill_prefix(&sp_io::hashing::twox_128(b"Elections"), None);
		#[allow(deprecated)]
		let res2 = kill_prefix(&sp_io::hashing::twox_128(b"Democracy"), None);
		#[allow(deprecated)]
		let res3 = kill_prefix(&sp_io::hashing::twox_128(b"Council"), None);

		match res1 {
			KillStorageResult::AllRemoved(_) => {
				log::info!("Successfully removed Elections storage")
			},
			KillStorageResult::SomeRemaining(_) => {
				log::info!("Failed to remove Elections storage")
			},
		}

		match res2 {
			KillStorageResult::AllRemoved(_) => {
				log::info!("Successfully removed Democracy storage")
			},
			KillStorageResult::SomeRemaining(_) => {
				log::info!("Failed to remove Democracy storage")
			},
		}

		match res3 {
			KillStorageResult::AllRemoved(_) => {
				log::info!("Successfully removed Council storage")
			},
			KillStorageResult::SomeRemaining(_) => {
				log::info!("Failed to remove Council storage")
			},
		}

		<Runtime as frame_system::Config>::DbWeight::get().reads_writes(1, 1)
	}
}

#[cfg(test)]
mod tests {
	use frame_support::migration::{get_storage_value, put_storage_value};
	use sp_runtime::BuildStorage;

	use crate::{Runtime, System};

	pub fn new_test_ext() -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::<Runtime>::default()
			.build_storage()
			.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}

	#[test]
	fn democracy_test() {
		new_test_ext().execute_with(|| {
			put_storage_value(b"Elections", b"Item", b"", 100u32);
			put_storage_value(b"Democracy", b"Item", b"", 100u32);
			put_storage_value(b"Council", b"Item", b"", 100u32);

			assert!(get_storage_value::<u32>(b"Elections", b"Item", b"").is_some());
			assert!(get_storage_value::<u32>(b"Democracy", b"Item", b"").is_some());
			assert!(get_storage_value::<u32>(b"Council", b"Item", b"").is_some());

			super::democracy::on_runtime_upgrade();

			assert!(get_storage_value::<u32>(b"Elections", b"Item", b"").is_none());
			assert!(get_storage_value::<u32>(b"Democracy", b"Item", b"").is_none());
			assert!(get_storage_value::<u32>(b"Council", b"Item", b"").is_none());
		});
	}
}
