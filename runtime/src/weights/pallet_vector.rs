// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet_vector`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-03-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-12-189`, CPU: `Intel(R) Xeon(R) Platinum 8175M CPU @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/avail-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_vector
// --extrinsic=*
// --heap-pages=4096
// --header=./HEADER-APACHE2
// --log=warn
// --output
// ./output/pallet_vector.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_vector`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vector::WeightInfo for WeightInfo<T> {
	/// Storage: `Vector::WhitelistedDomains` (r:1 w:0)
	/// Proof: `Vector::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 102400]`.
	fn send_message_arbitrary_message(_l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `41487`
		// Minimum execution time: 19_088_000 picoseconds.
		Weight::from_parts(20_213_539, 0)
			.saturating_add(Weight::from_parts(0, 41487))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Vector::WhitelistedDomains` (r:1 w:0)
	/// Proof: `Vector::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn send_message_fungible_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `586`
		//  Estimated: `41487`
		// Minimum execution time: 87_670_000 picoseconds.
		Weight::from_parts(88_934_000, 0)
			.saturating_add(Weight::from_parts(0, 41487))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Vector::SyncCommitteePoseidons` (r:0 w:1)
	/// Proof: `Vector::SyncCommitteePoseidons` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	fn set_poseidon_hash() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 14_907_000 picoseconds.
		Weight::from_parts(15_286_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::Broadcasters` (r:1 w:1)
	/// Proof: `Vector::Broadcasters` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn set_broadcaster() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `380`
		//  Estimated: `3501`
		// Minimum execution time: 22_860_000 picoseconds.
		Weight::from_parts(23_531_000, 0)
			.saturating_add(Weight::from_parts(0, 3501))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::WhitelistedDomains` (r:0 w:1)
	/// Proof: `Vector::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	fn set_whitelisted_domains() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_749_000 picoseconds.
		Weight::from_parts(12_079_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::ConfigurationStorage` (r:0 w:1)
	/// Proof: `Vector::ConfigurationStorage` (`max_values`: Some(1), `max_size`: Some(10), added: 505, mode: `MaxEncodedLen`)
	fn set_configuration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_193_000 picoseconds.
		Weight::from_parts(11_624_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::SourceChainFrozen` (r:0 w:1)
	/// Proof: `Vector::SourceChainFrozen` (`max_values`: None, `max_size`: Some(5), added: 2480, mode: `MaxEncodedLen`)
	fn source_chain_froze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_888_000 picoseconds.
		Weight::from_parts(13_433_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::ConfigurationStorage` (r:1 w:0)
	/// Proof: `Vector::ConfigurationStorage` (`max_values`: Some(1), `max_size`: Some(10), added: 505, mode: `MaxEncodedLen`)
	/// Storage: `Vector::FunctionIds` (r:1 w:0)
	/// Proof: `Vector::FunctionIds` (`max_values`: Some(1), `max_size`: Some(65), added: 560, mode: `MaxEncodedLen`)
	/// Storage: `Vector::StepVerificationKey` (r:1 w:0)
	/// Proof: `Vector::StepVerificationKey` (`max_values`: Some(1), `max_size`: Some(10003), added: 10498, mode: `MaxEncodedLen`)
	/// Storage: `Vector::SyncCommitteePoseidons` (r:1 w:0)
	/// Proof: `Vector::SyncCommitteePoseidons` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Vector::Head` (r:1 w:1)
	/// Proof: `Vector::Head` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Vector::Headers` (r:1 w:1)
	/// Proof: `Vector::Headers` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Vector::ExecutionStateRoots` (r:1 w:1)
	/// Proof: `Vector::ExecutionStateRoots` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Vector::Timestamps` (r:0 w:1)
	/// Proof: `Vector::Timestamps` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	fn fulfill_call_step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2599`
		//  Estimated: `11488`
		// Minimum execution time: 25_375_236_000 picoseconds.
		Weight::from_parts(25_538_659_000, 0)
			.saturating_add(Weight::from_parts(0, 11488))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Vector::ConfigurationStorage` (r:1 w:0)
	/// Proof: `Vector::ConfigurationStorage` (`max_values`: Some(1), `max_size`: Some(10), added: 505, mode: `MaxEncodedLen`)
	/// Storage: `Vector::FunctionIds` (r:1 w:0)
	/// Proof: `Vector::FunctionIds` (`max_values`: Some(1), `max_size`: Some(65), added: 560, mode: `MaxEncodedLen`)
	/// Storage: `Vector::RotateVerificationKey` (r:1 w:0)
	/// Proof: `Vector::RotateVerificationKey` (`max_values`: Some(1), `max_size`: Some(10003), added: 10498, mode: `MaxEncodedLen`)
	/// Storage: `Vector::Headers` (r:1 w:0)
	/// Proof: `Vector::Headers` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Vector::SyncCommitteePoseidons` (r:1 w:1)
	/// Proof: `Vector::SyncCommitteePoseidons` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	fn fulfill_call_rotate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3042`
		//  Estimated: `11488`
		// Minimum execution time: 25_427_483_000 picoseconds.
		Weight::from_parts(25_617_463_000, 0)
			.saturating_add(Weight::from_parts(0, 11488))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::MessageStatus` (r:1 w:1)
	/// Proof: `Vector::MessageStatus` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Vector::WhitelistedDomains` (r:1 w:0)
	/// Proof: `Vector::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	/// Storage: `Vector::Broadcasters` (r:1 w:0)
	/// Proof: `Vector::Broadcasters` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `Vector::SourceChainFrozen` (r:1 w:0)
	/// Proof: `Vector::SourceChainFrozen` (`max_values`: None, `max_size`: Some(5), added: 2480, mode: `MaxEncodedLen`)
	/// Storage: `Vector::ExecutionStateRoots` (r:1 w:0)
	/// Proof: `Vector::ExecutionStateRoots` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn execute_fungible_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `672`
		//  Estimated: `41487`
		// Minimum execution time: 201_610_000 picoseconds.
		Weight::from_parts(209_679_000, 0)
			.saturating_add(Weight::from_parts(0, 41487))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Vector::MessageStatus` (r:1 w:1)
	/// Proof: `Vector::MessageStatus` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Vector::WhitelistedDomains` (r:1 w:0)
	/// Proof: `Vector::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	/// Storage: `Vector::Broadcasters` (r:1 w:0)
	/// Proof: `Vector::Broadcasters` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `Vector::SourceChainFrozen` (r:1 w:0)
	/// Proof: `Vector::SourceChainFrozen` (`max_values`: None, `max_size`: Some(5), added: 2480, mode: `MaxEncodedLen`)
	/// Storage: `Vector::ExecutionStateRoots` (r:1 w:0)
	/// Proof: `Vector::ExecutionStateRoots` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 102400]`.
	fn execute_arbitrary_message(_l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `494`
		//  Estimated: `41487`
		// Minimum execution time: 126_747_000 picoseconds.
		Weight::from_parts(130_246_106, 0)
			.saturating_add(Weight::from_parts(0, 41487))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::FunctionIds` (r:0 w:1)
	/// Proof: `Vector::FunctionIds` (`max_values`: Some(1), `max_size`: Some(65), added: 560, mode: `MaxEncodedLen`)
	fn set_function_ids() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 13_916_000 picoseconds.
		Weight::from_parts(14_493_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::StepVerificationKey` (r:0 w:1)
	/// Proof: `Vector::StepVerificationKey` (`max_values`: Some(1), `max_size`: Some(10003), added: 10498, mode: `MaxEncodedLen`)
	fn set_step_verification_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 38_572_000 picoseconds.
		Weight::from_parts(39_816_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::RotateVerificationKey` (r:0 w:1)
	/// Proof: `Vector::RotateVerificationKey` (`max_values`: Some(1), `max_size`: Some(10003), added: 10498, mode: `MaxEncodedLen`)
	fn set_rotate_verification_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 39_842_000 picoseconds.
		Weight::from_parts(41_367_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::Updater` (r:1 w:1)
	/// Proof: `Vector::Updater` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn set_updater() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388`
		//  Estimated: `1517`
		// Minimum execution time: 15_000_000 picoseconds.
		Weight::from_parts(16_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1517))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::SP1VerificationKey` (r:1 w:1)
	/// Proof: `Vector::SP1VerificationKey` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn set_sp1_verification_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `1517`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(8_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1517))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::SyncCommitteeHashes` (r:0 w:1)
	/// Proof: `Vector::SyncCommitteeHashes` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	fn set_sync_committee_hash() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(5_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::Updater` (r:1 w:0)
	/// Proof: `Vector::Updater` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Vector::Head` (r:1 w:1)
	/// Proof: `Vector::Head` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Vector::SP1VerificationKey` (r:1 w:0)
	/// Proof: `Vector::SP1VerificationKey` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Vector::Headers` (r:1 w:1)
	/// Proof: `Vector::Headers` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Vector::ExecutionStateRoots` (r:1 w:1)
	/// Proof: `Vector::ExecutionStateRoots` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Vector::ConfigurationStorage` (r:1 w:0)
	/// Proof: `Vector::ConfigurationStorage` (`max_values`: Some(1), `max_size`: Some(13), added: 508, mode: `MaxEncodedLen`)
	/// Storage: `Vector::SyncCommitteeHashes` (r:2 w:1)
	/// Proof: `Vector::SyncCommitteeHashes` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Vector::Timestamps` (r:0 w:1)
	/// Proof: `Vector::Timestamps` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	fn fulfill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `614`
		//  Estimated: `6020`
		// Minimum execution time: 521_131_000_000 picoseconds.
		Weight::from_parts(523_158_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6020))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(5))
	}
}

