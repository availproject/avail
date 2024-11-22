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
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

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
// ./output/pallet_vector_weights.rs
// --template
// ./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_vector`.
pub trait WeightInfo {
	fn send_message_arbitrary_message(l: u32, ) -> Weight;
	fn send_message_fungible_token() -> Weight;
	fn set_poseidon_hash() -> Weight;
	fn set_broadcaster() -> Weight;
	fn set_whitelisted_domains() -> Weight;
	fn set_configuration() -> Weight;
	fn source_chain_froze() -> Weight;
	fn fulfill_call_step() -> Weight;
	fn fulfill_call_rotate() -> Weight;
	fn execute_fungible_token() -> Weight;
	fn execute_arbitrary_message(l: u32, ) -> Weight;
	fn set_function_ids() -> Weight;
	fn failed_tx_index(_l: u32) -> Weight { Weight::zero() }
	fn set_step_verification_key() -> Weight;
	fn set_rotate_verification_key() -> Weight;
	fn set_updater() -> Weight;
	fn set_sp1_verification_key() -> Weight;
	fn set_sync_committee_hash() -> Weight;
	fn fulfill() -> Weight;
}

/// Weights for `pallet_vector` using the Avail node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Vector::WhitelistedDomains` (r:1 w:0)
	/// Proof: `Vector::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 102400]`.
	fn send_message_arbitrary_message(_l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `41487`
		// Minimum execution time: 19_199_000 picoseconds.
		Weight::from_parts(20_257_871, 41487)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: `Vector::WhitelistedDomains` (r:1 w:0)
	/// Proof: `Vector::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn send_message_fungible_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `586`
		//  Estimated: `41487`
		// Minimum execution time: 87_666_000 picoseconds.
		Weight::from_parts(88_852_000, 41487)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Vector::SyncCommitteePoseidons` (r:0 w:1)
	/// Proof: `Vector::SyncCommitteePoseidons` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	fn set_poseidon_hash() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 14_835_000 picoseconds.
		Weight::from_parts(15_249_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::Broadcasters` (r:1 w:1)
	/// Proof: `Vector::Broadcasters` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn set_broadcaster() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `380`
		//  Estimated: `3501`
		// Minimum execution time: 23_024_000 picoseconds.
		Weight::from_parts(23_687_000, 3501)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::WhitelistedDomains` (r:0 w:1)
	/// Proof: `Vector::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	fn set_whitelisted_domains() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_569_000 picoseconds.
		Weight::from_parts(11_979_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::ConfigurationStorage` (r:0 w:1)
	/// Proof: `Vector::ConfigurationStorage` (`max_values`: Some(1), `max_size`: Some(10), added: 505, mode: `MaxEncodedLen`)
	fn set_configuration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_246_000 picoseconds.
		Weight::from_parts(11_880_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::SourceChainFrozen` (r:0 w:1)
	/// Proof: `Vector::SourceChainFrozen` (`max_values`: None, `max_size`: Some(5), added: 2480, mode: `MaxEncodedLen`)
	fn source_chain_froze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_524_000 picoseconds.
		Weight::from_parts(12_985_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
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
		// Minimum execution time: 25_210_559_000 picoseconds.
		Weight::from_parts(25_282_049_000, 11488)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
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
		// Minimum execution time: 25_036_650_000 picoseconds.
		Weight::from_parts(25_145_989_000, 11488)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
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
		// Minimum execution time: 202_005_000 picoseconds.
		Weight::from_parts(204_185_000, 41487)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
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
	fn execute_arbitrary_message(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `494`
		//  Estimated: `41487`
		// Minimum execution time: 126_100_000 picoseconds.
		Weight::from_parts(128_756_688, 41487)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(9, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::FunctionIds` (r:0 w:1)
	/// Proof: `Vector::FunctionIds` (`max_values`: Some(1), `max_size`: Some(65), added: 560, mode: `MaxEncodedLen`)
	fn set_function_ids() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 13_936_000 picoseconds.
		Weight::from_parts(14_330_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::StepVerificationKey` (r:0 w:1)
	/// Proof: `Vector::StepVerificationKey` (`max_values`: Some(1), `max_size`: Some(10003), added: 10498, mode: `MaxEncodedLen`)
	fn set_step_verification_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 38_514_000 picoseconds.
		Weight::from_parts(38_853_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::RotateVerificationKey` (r:0 w:1)
	/// Proof: `Vector::RotateVerificationKey` (`max_values`: Some(1), `max_size`: Some(10003), added: 10498, mode: `MaxEncodedLen`)
	fn set_rotate_verification_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 39_299_000 picoseconds.
		Weight::from_parts(40_176_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::Updater` (r:1 w:1)
	/// Proof: `Vector::Updater` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn set_updater() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388`
		//  Estimated: `1517`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(15_000_000, 1517)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::SP1VerificationKey` (r:1 w:1)
	/// Proof: `Vector::SP1VerificationKey` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn set_sp1_verification_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `1517`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(8_000_000, 1517)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::SyncCommitteeHashes` (r:0 w:1)
	/// Proof: `Vector::SyncCommitteeHashes` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	fn set_sync_committee_hash() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_000_000 picoseconds.
		Weight::from_parts(5_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
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
	/// Proof: `Vector::ConfigurationStorage` (`max_values`: Some(1), `max_size`: Some(10), added: 505, mode: `MaxEncodedLen`)
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
		// Minimum execution time: 526_071_000_000 picoseconds.
		Weight::from_parts(529_512_000_000, 6020)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Vector::WhitelistedDomains` (r:1 w:0)
	/// Proof: `Vector::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 102400]`.
	fn send_message_arbitrary_message(_l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `41487`
		// Minimum execution time: 19_199_000 picoseconds.
		Weight::from_parts(20_257_871, 41487)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: `Vector::WhitelistedDomains` (r:1 w:0)
	/// Proof: `Vector::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn send_message_fungible_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `586`
		//  Estimated: `41487`
		// Minimum execution time: 87_666_000 picoseconds.
		Weight::from_parts(88_852_000, 41487)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Vector::SyncCommitteePoseidons` (r:0 w:1)
	/// Proof: `Vector::SyncCommitteePoseidons` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	fn set_poseidon_hash() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 14_835_000 picoseconds.
		Weight::from_parts(15_249_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::Broadcasters` (r:1 w:1)
	/// Proof: `Vector::Broadcasters` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn set_broadcaster() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `380`
		//  Estimated: `3501`
		// Minimum execution time: 23_024_000 picoseconds.
		Weight::from_parts(23_687_000, 3501)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::WhitelistedDomains` (r:0 w:1)
	/// Proof: `Vector::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	fn set_whitelisted_domains() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_569_000 picoseconds.
		Weight::from_parts(11_979_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::ConfigurationStorage` (r:0 w:1)
	/// Proof: `Vector::ConfigurationStorage` (`max_values`: Some(1), `max_size`: Some(10), added: 505, mode: `MaxEncodedLen`)
	fn set_configuration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_246_000 picoseconds.
		Weight::from_parts(11_880_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::SourceChainFrozen` (r:0 w:1)
	/// Proof: `Vector::SourceChainFrozen` (`max_values`: None, `max_size`: Some(5), added: 2480, mode: `MaxEncodedLen`)
	fn source_chain_froze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_524_000 picoseconds.
		Weight::from_parts(12_985_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
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
		// Minimum execution time: 25_210_559_000 picoseconds.
		Weight::from_parts(25_282_049_000, 11488)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
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
		// Minimum execution time: 25_036_650_000 picoseconds.
		Weight::from_parts(25_145_989_000, 11488)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
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
		// Minimum execution time: 202_005_000 picoseconds.
		Weight::from_parts(204_185_000, 41487)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
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
	fn execute_arbitrary_message(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `494`
		//  Estimated: `41487`
		// Minimum execution time: 126_100_000 picoseconds.
		Weight::from_parts(128_756_688, 41487)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(9, 0).saturating_mul(l.into()))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::FunctionIds` (r:0 w:1)
	/// Proof: `Vector::FunctionIds` (`max_values`: Some(1), `max_size`: Some(65), added: 560, mode: `MaxEncodedLen`)
	fn set_function_ids() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 13_936_000 picoseconds.
		Weight::from_parts(14_330_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::StepVerificationKey` (r:0 w:1)
	/// Proof: `Vector::StepVerificationKey` (`max_values`: Some(1), `max_size`: Some(10003), added: 10498, mode: `MaxEncodedLen`)
	fn set_step_verification_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 38_514_000 picoseconds.
		Weight::from_parts(38_853_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::RotateVerificationKey` (r:0 w:1)
	/// Proof: `Vector::RotateVerificationKey` (`max_values`: Some(1), `max_size`: Some(10003), added: 10498, mode: `MaxEncodedLen`)
	fn set_rotate_verification_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 39_299_000 picoseconds.
		Weight::from_parts(40_176_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::Updater` (r:1 w:1)
	/// Proof: `Vector::Updater` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn set_updater() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388`
		//  Estimated: `1517`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(15_000_000, 1517)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::SP1VerificationKey` (r:1 w:1)
	/// Proof: `Vector::SP1VerificationKey` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn set_sp1_verification_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `1517`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(8_000_000, 1517)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::SyncCommitteeHashes` (r:0 w:1)
	/// Proof: `Vector::SyncCommitteeHashes` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	fn set_sync_committee_hash() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_000_000 picoseconds.
		Weight::from_parts(5_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
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
	/// Proof: `Vector::ConfigurationStorage` (`max_values`: Some(1), `max_size`: Some(10), added: 505, mode: `MaxEncodedLen`)
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
		// Minimum execution time: 526_071_000_000 picoseconds.
		Weight::from_parts(529_512_000_000, 6020)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
}