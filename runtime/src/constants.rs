// This file is part of Substrate.

// Copyright (C) 2019-2022 Parity Technologies (UK) Ltd.
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

//! A set of constant values used in substrate runtime.

use da_primitives::currency::{Balance, AVL};
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU32},
};

use crate::BlockNumber;

/// Money matters.
pub mod currency {
	use super::Balance;

	pub const MILLICENTS: Balance = 1_000_000_000;
	pub const CENTS: Balance = 1_000 * MILLICENTS; // assume this is worth about a cent.
	pub const DOLLARS: Balance = 100 * CENTS;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
	}
}

/// Time.
pub mod time {
	use super::*;
	use crate::Moment;

	/// Since BABE is probabilistic this is the average expected block time that
	/// we are targeting. Blocks will be produced at a minimum duration defined
	/// by `SLOT_DURATION`, but some slots will not be allocated to any
	/// authority and hence no block will be produced. We expect to have this
	/// block time on average following the defined slot duration and the value
	/// of `c` configured for BABE (where `1 - c` represents the probability of
	/// a slot being empty).
	/// This value is only used indirectly to define the unit constants below
	/// that are expressed in blocks. The rest of the code should use
	/// `SLOT_DURATION` instead (like the Timestamp pallet for calculating the
	/// minimum period).
	///
	/// If using BABE with secondary slots (default) then all of the slots will
	/// always be assigned, in which case `MILLISECS_PER_BLOCK` and
	/// `SLOT_DURATION` should have the same value.
	///
	/// <https://research.web3.foundation/en/latest/polkadot/block-production/Babe.html#-6.-practical-results>
	#[cfg(not(feature = "fast-runtime"))]
	pub const MILLISECS_PER_BLOCK: Moment = 20_000;
	#[cfg(feature = "fast-runtime")]
	pub const MILLISECS_PER_BLOCK: Moment = 6_000;

	pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;

	// NOTE: Currently it is not possible to change the slot duration after the chain has started.
	//       Attempting to do so will brick block production.
	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

	// 1 in 4 blocks (on average, not counting collisions) will be primary BABE blocks.
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

	// NOTE: Currently it is not possible to change the epoch duration after the chain has started.
	//       Attempting to do so will brick block production.
	// pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 10 * MINUTES;
	#[allow(clippy::identity_op)]
	#[cfg(not(feature = "fast-runtime"))]
	pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = 1 * HOURS;
	#[cfg(feature = "fast-runtime")]
	pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = 15 * MINUTES;

	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;

	parameter_types! {
		pub const EpochDuration: BlockNumber = EPOCH_DURATION_IN_SLOTS;
		pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
	}
}

pub mod system {
	use super::*;

	pub type MaxConsumers = ConstU32<16>;
	pub type SS58Prefix = ConstU16<42>;
	pub type MaxAuthorities = ConstU32<128>;
}

pub mod indices {
	use super::*;

	parameter_types! {
		pub const IndexDeposit :Balance =  1 * AVL;
	}
}

pub mod balances {
	use super::*;

	parameter_types! {
		pub const ExistentialDeposit :Balance =  1 * AVL;
	}

	pub type MaxLocks = ConstU32<32>;
	pub type MaxReserves = ConstU32<32>;
}

pub mod council {
	use super::*;

	#[cfg(not(feature = "fast-runtime"))]
	parameter_types! {
		pub const MotionDuration :BlockNumber = 14 * super::time::DAYS;
	}

	#[cfg(feature = "fast-runtime")]
	parameter_types! {
		pub const MotionDuration :BlockNumber = 5 * super::time::MINUTES;
	}

	pub type MaxProposals = ConstU32<128>;

	parameter_types! {
		pub const MaxMembers: u32 = 32;
	}
}

pub mod nomination_pools {
	use super::*;

	pub const MIN_CREATE_BOND: Balance = 10 * AVL;
	#[allow(clippy::identity_op)]
	pub const MIN_JOIN_BOND: Balance = 1 * AVL;
	pub const MAX_POOLS: u32 = 16;
	pub const MAX_MEMBERS_PER_POOL: u32 = 100;
	pub const MAX_MEMBERS: u32 = MAX_POOLS * MAX_MEMBERS_PER_POOL;
}

/// The BABE epoch configuration at genesis.
pub const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
	sp_consensus_babe::BabeEpochConfiguration {
		c: time::PRIMARY_PROBABILITY,
		allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryVRFSlots,
	};
