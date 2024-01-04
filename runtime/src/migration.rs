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

/// Implements `OnRuntimeUpgrade` trait.
pub struct Migration {}
impl OnRuntimeUpgrade for Migration {
	fn on_runtime_upgrade() -> Weight {
		Weight::zero()
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
		Ok(Vec::new())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), TryRuntimeError> {
		Ok(())
	}
}
