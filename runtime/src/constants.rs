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

#![allow(clippy::identity_op)]
use avail_core::currency::{Balance, AVL};
use frame_support::{
	dispatch::DispatchClass,
	parameter_types,
	traits::{ConstU16, ConstU32},
	weights::{constants::BlockExecutionWeight, Weight},
};
use sp_runtime::{transaction_validity::TransactionPriority, Perbill, Permill};
use static_assertions::const_assert;

use crate::BlockNumber;

/// cannot have validators higher than this count.
pub type MaxAuthorities = ConstU32<100_000>;
/// cannot have active validators higher than this count.
pub type MaxActiveValidators = ConstU32<1200>;

parameter_types! {
	/// We take the top 12500 nominators as electing voters..
	pub const MaxElectingVoters: u32 = 22_500;
	/// ... and all of the validators as electable targets. Whilst this is the case, we cannot and
	/// shall not increase the size of the validator intentions.
	pub const MaxElectableTargets: u16 = u16::MAX;
}

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
	#[cfg(not(feature = "fast-runtime"))]
	pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = 4 * HOURS;
	#[cfg(feature = "fast-runtime")]
	pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = 5 * MINUTES;

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
	use avail_core::NORMAL_DISPATCH_RATIO;
	use frame_support::weights::constants::{ExtrinsicBaseWeight, WEIGHT_REF_TIME_PER_SECOND};
	use frame_system::limits::BlockWeights as SystemBlockWeights;

	use super::*;

	pub type MaxConsumers = ConstU32<16>;
	pub type SS58Prefix = ConstU16<42>;

	/// We assume that ~10% of the block weight is consumed by `on_initialize` handlers.
	/// This is used to limit the maximal weight of a single extrinsic.
	const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);

	/// We allow for 2 seconds of compute with a 6 second average block time, with maximum proof size.
	#[cfg(feature = "fast-runtime")]
	const MAXIMUM_BLOCK_WEIGHT: Weight =
		Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2), u64::MAX);

	/// We allow for 5 seconds of compute with a 20 second average block time, with maximum proof size.
	#[cfg(not(feature = "fast-runtime"))]
	const MAXIMUM_BLOCK_WEIGHT: Weight =
		Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_mul(5), u64::MAX);

	parameter_types! {
		pub RuntimeBlockWeights: SystemBlockWeights = SystemBlockWeights::builder()
			.base_block(BlockExecutionWeight::get())
			.for_class(DispatchClass::all(), |weights| {
				weights.base_extrinsic = ExtrinsicBaseWeight::get();
			})
			.for_class(DispatchClass::Normal, |weights| {
				weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
			})
			.for_class(DispatchClass::Operational, |weights| {
				weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
				// Operational transactions have some extra reserved space, so that they
				// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
				weights.reserved = Some(
					MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
				);
			})
			.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
			.build_or_panic();
	}
	const_assert!(NORMAL_DISPATCH_RATIO.deconstruct() >= AVERAGE_ON_INITIALIZE_RATIO.deconstruct());
}

pub mod indices {
	use super::*;

	parameter_types! {
		pub const IndexDeposit :Balance =  1 * AVL;
	}
}

pub mod balances {
	use super::{currency::*, *};

	parameter_types! {
		pub const ExistentialDeposit :Balance =  10 * CENTS; // 0.1 AVLs
	}
}

pub mod council {

	use super::*;
	use crate::constants::system::RuntimeBlockWeights;

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
		pub MaxProposalWeight: Weight = Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
		pub const MaxMembers: u32 = 32;
	}
}

pub mod nomination_pools {
	use super::*;

	pub const MIN_CREATE_BOND: Balance = 10 * AVL;
	pub const MIN_JOIN_BOND: Balance = 1 * AVL;
	pub const MAX_POOLS: u32 = 16;
	pub const MAX_MEMBERS_PER_POOL: u32 = 100;
	pub const MAX_MEMBERS: u32 = MAX_POOLS * MAX_MEMBERS_PER_POOL;
}

pub mod staking {
	use crate::impls::RuntimeBlockLength;
	use sp_runtime::curve::PiecewiseLinear;
	use sp_std::vec;

	use super::{currency::*, time::*, *};

	pallet_staking_reward_curve::build! {
	const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_010_000, // minimum_inflation_rate = 1%
		max_inflation: 0_050_000, // maximum_inflation_rate = 5%
		ideal_stake: 0_500_000, // target_staking_rate = 50%
		falloff: 0_050_000,  // inflation_decay = 5%
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
	}

	frame_election_provider_support::generate_solution_type!(
		#[compact]
		pub struct NposSolution16::<
			VoterIndex = u32,
			TargetIndex = u16,
			Accuracy = sp_runtime::PerU16,
			MaxVoters = MaxElectingVoters,
		>(16)
	);

	#[cfg(feature = "fast-runtime")]
	parameter_types! {
		pub const SessionsPerEra: sp_staking::SessionIndex = 1;
		pub const BondingDuration: sp_staking::EraIndex = 2; // 2 eras
		pub const SlashDeferDuration: sp_staking::EraIndex = BondingDuration::get() - 1;
	}

	#[cfg(not(feature = "fast-runtime"))]
	parameter_types! {
		pub const SessionsPerEra: sp_staking::SessionIndex = 6;
		pub const BondingDuration: sp_staking::EraIndex = 28; // 28 days
		pub const SlashDeferDuration: sp_staking::EraIndex = BondingDuration::get() - 1; // 27 Days
	}

	parameter_types! {
		pub MaxNominations: u32 = <NposSolution16 as frame_election_provider_support::NposSolution>::LIMIT as u32;
		pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(17);
		pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;

		// phase durations. 1/4 of the last session for each.
		pub const SignedPhase: u32 = EPOCH_DURATION_IN_SLOTS / 4;
		pub const UnsignedPhase: u32 = EPOCH_DURATION_IN_SLOTS / 4;

		pub const SignedRewardBase: Balance = AVL;
		pub const SignedDepositBase: Balance = AVL;
		pub const SignedDepositByte: Balance = CENTS;

		pub BetterUnsignedThreshold: Perbill = Perbill::from_rational(1u32, 10_000);

		pub SolutionImprovementThreshold: Perbill = Perbill::from_rational(1u32, 10_000);
		// miner configs		/// We prioritize im-online heartbeats over election solution submission.
		pub const StakingUnsignedPriority: TransactionPriority = TransactionPriority::max_value() / 2;
		pub const MultiPhaseUnsignedPriority: TransactionPriority = StakingUnsignedPriority::get() - 1u64;
		pub MinerMaxWeight: Weight = system::RuntimeBlockWeights::get()
			.get(DispatchClass::Normal)
			.max_extrinsic.expect("Normal extrinsics have a weight limit configured; qed")
			.saturating_sub(BlockExecutionWeight::get());
		// Solution can occupy 90% of normal block size
		pub MinerMaxLength: u32 = Perbill::from_rational(9u32, 10) *
			*RuntimeBlockLength::get()
			.max
			.get(DispatchClass::Normal);
		pub const OffchainRepeat: BlockNumber = 5;
	}

	pub type MaxNominatorRewardedPerValidator = ConstU32<256>;
	pub type MaxUnlockingChunks = ConstU32<32>;
	pub type HistoryDepth = ConstU32<84>;
	pub type MaxNominators = ConstU32<1_024>;
	pub type MaxValidators = ConstU32<32>;

	// OnChain values are lower.
	pub type MaxOnChainElectingVoters = ConstU32<1_024>;
	pub type MaxOnChainElectableTargets = ConstU16<512>;
	// The maximum winners that can be elected by the Election pallet which is equivalent to the
	// maximum active validators the staking pallet can have.
	pub type MaxActiveValidators = MaxAuthorities;

	// signed config
	pub type SignedMaxSubmissions = ConstU32<16>;
	pub type SignedMaxRefunds = ConstU32<4>;
}

pub mod babe {
	use sp_consensus_babe::{AllowedSlots, BabeEpochConfiguration};

	use super::*;

	/// The BABE epoch configuration at genesis.
	pub const GENESIS_EPOCH_CONFIG: BabeEpochConfiguration = BabeEpochConfiguration {
		c: time::PRIMARY_PROBABILITY,
		allowed_slots: AllowedSlots::PrimaryAndSecondaryVRFSlots,
	};

	parameter_types! {
		pub const ReportLongevity: BlockNumber =
			staking::BondingDuration::get() * staking::SessionsPerEra::get() * time::EpochDuration::get();
	}
}

pub mod im {

	use super::*;

	parameter_types! {
		pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();

	}

	pub type MaxPeerDataEncodingSize = ConstU32<1_024>;
}

pub mod preimage {
	use super::{currency::*, *};

	parameter_types! {
		pub const PreimageBaseDeposit: Balance = 1 * AVL;
		// One cent: $10,000 / MB
		pub const PreimageByteDeposit: Balance = 1 * CENTS;
	}
}

pub mod bounty {
	use super::{time::*, *};

	#[cfg(not(feature = "fast-runtime"))]
	parameter_types! {
		pub const DepositPayoutDelay: BlockNumber = 8 * DAYS;
		pub const UpdatePeriod: BlockNumber = 90 * DAYS;
	}

	#[cfg(feature = "fast-runtime")]
	parameter_types! {
		pub const DepositPayoutDelay: BlockNumber = 1 * MINUTES;
		pub const UpdatePeriod: BlockNumber = 10 * MINUTES;
	}

	parameter_types! {
		pub const ValueMinimum: Balance = 5 * AVL;
		pub const DepositBase: Balance = AVL;
		pub const CuratorDepositMultiplier: Permill = Permill::from_percent(50);
		pub const CuratorDepositMin: Balance = 1 * AVL;
		pub const CuratorDepositMax: Balance = 100 * AVL;
	}
}

pub mod da {
	use avail_core::{BlockLengthColumns, BlockLengthRows};

	use super::*;

	parameter_types! {
		pub const MinBlockRows: BlockLengthRows = BlockLengthRows(32);
		pub const MaxBlockRows: BlockLengthRows = BlockLengthRows(1024);
		pub const MinBlockCols: BlockLengthColumns = BlockLengthColumns(32);
		pub const MaxBlockCols: BlockLengthColumns = kate::config::MAX_BLOCK_COLUMNS;
	}
	pub type MaxAppKeyLength = ConstU32<64>;
	pub type MaxAppDataLength = ConstU32<524_288>; // 512 Kb
}

pub mod nomad {
	use sp_core::H256;

	use super::*;

	parameter_types! {
		pub const DABridgePalletId: H256 = H256::repeat_byte(1);
	}
	pub type MaxMessageBodyBytes = ConstU32<2048>;
}

/// Macro to set a value (e.g. when using the `parameter_types` macro) to either a production value
/// or to an environment variable or testing value (in case the `fast-runtime` feature is selected).
/// Note that the environment variable is evaluated _at compile time_.
///
/// Usage:
/// ```Rust
/// parameter_types! {
///    // Note that the env variable version parameter cannot be const.
///     pub LaunchPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1, "KSM_LAUNCH_PERIOD");
///     pub const VotingPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1 * MINUTES);
/// }
/// ```
#[macro_export]
macro_rules! prod_or_fast {
	($prod:expr, $test:expr) => {
		if cfg!(feature = "fast-runtime") {
			$test
		} else {
			$prod
		}
	};
}
