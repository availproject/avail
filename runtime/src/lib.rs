// This file is part of Substrate.

// Copyright (C) 2018-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The Data Availability runtime. This can be compiled with `#[no_std]`, ready for Wasm.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 512.
#![recursion_limit = "512"]
#![feature(result_option_inspect)]
#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]

use codec::Decode;
use frame_election_provider_support::{
	onchain, BalancingConfig, ElectionDataProvider, SequentialPhragmen, VoteWeight,
};
pub use frame_support::{
	construct_runtime, debug,
	dispatch::DispatchClass,
	pallet_prelude::Get,
	parameter_types,
	traits::{
		ConstU32, Currency, EitherOfDiverse, EqualPrivilegeOnly, Everything, ExtrinsicCall,
		Imbalance, KeyOwnerProofSystem, OnUnbalanced, Randomness, U128CurrencyToVote,
	},
	weights::{
		constants::{
			BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
		},
		ConstantMultiplier, IdentityFee, Weight,
	},
	PalletId, RuntimeDebug, StorageValue,
};
use frame_system::{limits::BlockLength, submitted_data, EnsureRoot};
use pallet_election_provider_multi_phase::SolutionAccuracyOf;
use pallet_session::historical as pallet_session_historical;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
	create_runtime_str,
	traits::{self, BlakeTwo256, Bounded, OpaqueKeys},
	FixedPointNumber, FixedU128,
};
pub use sp_runtime::{Perbill, Percent, Permill, Perquintill};
use sp_std::{prelude::*, rc::Rc};
#[cfg(feature = "std")]
use sp_version::NativeVersion;

#[cfg(test)]
mod data_root_tests;

/// Import the DA pallet.
pub use da_primitives::{
	asdr::{AppExtrinsic, AppId, AppUncheckedExtrinsic, GetAppId},
	currency::{Balance, AVL, CENTS, MILLICENTS},
	well_known_keys::KATE_PUBLIC_PARAMS,
	DataProof, Header as DaHeader, OpaqueExtrinsic, NORMAL_DISPATCH_RATIO,
};
pub use pallet_balances::Call as BalancesCall;
use pallet_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
#[cfg(feature = "std")]
pub use pallet_staking::StakerStatus;
#[cfg(any(feature = "std", test))]
pub use pallet_sudo::Call as SudoCall;
// A few exports that help ease life for downstream crates.
pub use pallet_timestamp::Call as TimestampCall;
pub use pallet_transaction_payment::{CurrencyAdapter, Multiplier, TargetedFeeAdjustment};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_version::RuntimeVersion;

mod primitives;
pub use primitives::*;

/// Implementations of some helper traits passed into runtime modules as associated types.
pub mod impls;
use impls::Author;

/// APIs of this runtime.
pub mod apis;

/// Constant values used within the runtime.
pub mod constants;
use constants::{currency::*, time::*};
use sp_runtime::generic::Era;

mod migration;

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

/// Wasm binary unwrapped. If built with `SKIP_WASM_BUILD`, the function panics.
#[cfg(feature = "std")]
pub fn wasm_binary_unwrap() -> &'static [u8] {
	WASM_BINARY.expect(
		"Development wasm binary is not available. This means the client is built with \
		 `SKIP_WASM_BUILD` flag and it is only usable for production chains. Please rebuild with \
		 the flag disabled.",
	)
}

/// Runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("data-avail"),
	impl_name: create_runtime_str!("data-avail"),
	authoring_version: 11,
	// Per convention: if the runtime behavior changes, increment spec_version
	// and set impl_version to 0. If only runtime
	// implementation changes and behavior does not, then leave spec_version as
	// is and increment impl_version.
	spec_version: 10,
	impl_version: 0,
	apis: apis::runtime_api_versions(),
	transaction_version: 1,
	state_version: 1,
};

pub mod voter_bags;

/// The version information used to identify this runtime when compiled natively.
#[cfg(any(feature = "std", test))]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;

pub struct DealWithFees;
impl OnUnbalanced<NegativeImbalance> for DealWithFees {
	fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance>) {
		if let Some(fees) = fees_then_tips.next() {
			// for fees, 80% to treasury, 20% to author
			let mut split = fees.ration(80, 20);
			if let Some(tips) = fees_then_tips.next() {
				// for tips, if any, 80% to treasury, 20% to author (though this can be anything)
				tips.ration_merge_into(80, 20, &mut split);
			}
			Treasury::on_unbalanced(split.0);
			Author::on_unbalanced(split.1);
		}
	}
}

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	pub const Version: RuntimeVersion = VERSION;
	pub RuntimeBlockLength: BlockLength =
		BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub const MaximumBlockWeight: Weight = Weight::from_ref_time(WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2));
}

/// Filters and extracts `data` from `call` if it is a `DataAvailability::submit_data` type.
impl submitted_data::Filter<RuntimeCall> for Runtime {
	fn filter(call: RuntimeCall, metrics: submitted_data::RcMetrics) -> Option<Vec<Vec<u8>>> {
		metrics.borrow_mut().total_extrinsics += 1;

		match call {
			RuntimeCall::DataAvailability(da_control::Call::submit_data { data }) => {
				let mut metrics = metrics.borrow_mut();
				metrics.data_submit_leaves += 1;
				metrics.data_submit_extrinsics += 1;
				Some(vec![data.into_inner()])
			},
			RuntimeCall::Utility(pallet_utility::Call::batch { calls })
			| RuntimeCall::Utility(pallet_utility::Call::batch_all { calls })
			| RuntimeCall::Utility(pallet_utility::Call::force_batch { calls }) => {
				Self::process_calls(calls, &metrics)
			},
			_ => None,
		}
	}

	/// This function processes a list of calls and returns their data as Vec<Vec<u8>>
	fn process_calls(
		calls: Vec<RuntimeCall>,
		metrics: &submitted_data::RcMetrics,
	) -> Option<Vec<Vec<u8>>> {
		let mut result = Vec::with_capacity(calls.len());
		for call in calls {
			if let Some(data) = Self::filter(call, Rc::clone(metrics)) {
				result.extend(data);
			}
		}
		if !result.is_empty() {
			Some(result)
		} else {
			None
		}
	}
}

/// Decodes and extracts the `data` of `DataAvailability::submit_data` extrinsics.
impl submitted_data::Extractor for Runtime {
	type Error = codec::Error;

	fn extract(
		opaque: &OpaqueExtrinsic,
		metrics: submitted_data::RcMetrics,
	) -> Result<Vec<Vec<u8>>, Self::Error> {
		let extrinsic = UncheckedExtrinsic::try_from(opaque)?;
		let data =
			<Runtime as submitted_data::Filter<RuntimeCall>>::filter(extrinsic.function, metrics)
				.unwrap_or_default();

		Ok(data)
	}
}

// Configure FRAME pallets to include in runtime.
impl frame_system::Config for Runtime {
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// The maximum length of a block (in bytes).
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = Everything;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	type BlockLength = RuntimeBlockLength;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = constants::system::RuntimeBlockWeights;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = DaHeader<BlockNumber, BlakeTwo256>;
	/// The header builder type.
	type HeaderExtensionBuilder = frame_system::header_builder::da::HeaderExtensionBuilder<Runtime>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = Indices;
	type MaxConsumers = constants::system::MaxConsumers;
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// What to do if a new account is created.
	type OnNewAccount = ();
	// What to do if runtime code change: Default behaviour.
	type OnSetCode = ();
	/// Converts a module to the index of the module in `construct_runtime!`.
	///
	/// This type is being generated by `construct_runtime!`.
	type PalletInfo = PalletInfo;
	/// Randomness
	type Randomness = pallet_babe::RandomnessFromOneEpochAgo<Runtime>;
	/// The aggregated dispatch type that is available for extrinsics.
	type RuntimeCall = RuntimeCall;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	/// The ubiquitous origin type.
	type RuntimeOrigin = RuntimeOrigin;
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = constants::system::SS58Prefix;
	/// Data Root
	type SubmittedDataExtractor = Runtime;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = frame_system::weights::SubstrateWeight<Runtime>;
	type UncheckedExtrinsic = UncheckedExtrinsic;
	/// Version of the runtime.
	type Version = Version;
}

impl pallet_utility::Config for Runtime {
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	// One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes.
	pub const DepositBase: Balance = deposit(1, 88);
	// Additional storage item size of 32 bytes.
	pub const DepositFactor: Balance = deposit(0, 32);
}

impl pallet_multisig::Config for Runtime {
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type MaxSignatories = ConstU32<100>;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_multisig::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
		constants::system::RuntimeBlockWeights::get().max_block;
	pub const MaxScheduledPerBlock: u32 = 50;
}

impl pallet_scheduler::Config for Runtime {
	type MaxScheduledPerBlock = ConstU32<512>;
	type MaximumWeight = MaximumSchedulerWeight;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type PalletsOrigin = OriginCaller;
	type Preimages = Preimage;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type WeightInfo = pallet_scheduler::weights::SubstrateWeight<Runtime>;
}

impl pallet_preimage::Config for Runtime {
	type BaseDeposit = constants::preimage::PreimageBaseDeposit;
	type ByteDeposit = constants::preimage::PreimageByteDeposit;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_preimage::weights::SubstrateWeight<Runtime>;
}

impl pallet_babe::Config for Runtime {
	type DisabledValidators = Session;
	type EpochChangeTrigger = pallet_babe::ExternalTrigger;
	type EpochDuration = constants::time::EpochDuration;
	type ExpectedBlockTime = constants::time::ExpectedBlockTime;
	type HandleEquivocation = pallet_babe::EquivocationHandler<
		Self::KeyOwnerIdentification,
		Offences,
		constants::babe::ReportLongevity,
	>;
	type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		pallet_babe::AuthorityId,
	)>>::IdentificationTuple;
	type KeyOwnerProof = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		pallet_babe::AuthorityId,
	)>>::Proof;
	type KeyOwnerProofSystem = Historical;
	type MaxAuthorities = constants::system::MaxAuthorities;
	type WeightInfo = ();
}

impl pallet_indices::Config for Runtime {
	type AccountIndex = AccountIndex;
	type Currency = Balances;
	type Deposit = constants::indices::IndexDeposit;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_indices::weights::SubstrateWeight<Runtime>;
}

impl pallet_balances::Config for Runtime {
	type AccountStore = frame_system::Pallet<Runtime>;
	/// The type for recording an account's balance.
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = constants::balances::ExistentialDeposit;
	type MaxLocks = constants::balances::MaxLocks;
	type MaxReserves = constants::balances::MaxReserves;
	type ReserveIdentifier = ReserveIdentifier;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const TransactionByteFee: Balance = 10 * MILLICENTS;
	pub const OperationalFeeMultiplier: u8 = 5u8;
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(65, 1_000_000);
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000_000u128);
	pub MaximumMultiplier: Multiplier = Bounded::max_value();
}

impl pallet_transaction_payment::Config for Runtime {
	type FeeMultiplierUpdate = TargetedFeeAdjustment<
		Self,
		TargetBlockFullness,
		AdjustmentVariable,
		MinimumMultiplier,
		MaximumMultiplier,
	>;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type OnChargeTransaction = CurrencyAdapter<Balances, DealWithFees>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type RuntimeEvent = RuntimeEvent;
	type WeightToFee = IdentityFee<Balance>;
}

parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
	type MinimumPeriod = MinimumPeriod;
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = Moment;
	type OnTimestampSet = Babe;
	type WeightInfo = pallet_timestamp::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const UncleGenerations: BlockNumber = 5;
}

impl pallet_authorship::Config for Runtime {
	type EventHandler = (Staking, ImOnline);
	type FilterUncle = ();
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
	type UncleGenerations = UncleGenerations;
}

impl pallet_session::Config for Runtime {
	type Keys = SessionKeys;
	type NextSessionRotation = Babe;
	type RuntimeEvent = RuntimeEvent;
	type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
	type ShouldEndSession = Babe;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_staking::StashOf<Self>;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}

impl pallet_session::historical::Config for Runtime {
	type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
}

pub struct StakingBenchmarkingConfig;
impl pallet_staking::BenchmarkingConfig for StakingBenchmarkingConfig {
	type MaxNominators = constants::staking::MaxNominators;
	type MaxValidators = constants::staking::MaxValidators;
}

impl pallet_staking::Config for Runtime {
	/// A super-majority of the council can cancel the slash.
	type AdminOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 3, 4>,
	>;
	type BenchmarkingConfig = StakingBenchmarkingConfig;
	type BondingDuration = constants::staking::BondingDuration;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type CurrencyToVote = U128CurrencyToVote;
	type ElectionProvider = ElectionProviderMultiPhase;
	type EraPayout = pallet_staking::ConvertCurve<constants::staking::RewardCurve>;
	type GenesisElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type HistoryDepth = constants::staking::HistoryDepth;
	type MaxNominations = constants::staking::MaxNominations;
	type MaxNominatorRewardedPerValidator = constants::staking::MaxNominatorRewardedPerValidator;
	type MaxUnlockingChunks = constants::staking::MaxUnlockingChunks;
	type NextNewSession = Session;
	type OffendingValidatorsThreshold = constants::staking::OffendingValidatorsThreshold;
	type OnStakerSlash = NominationPools;
	// send the slashed funds to the treasury.
	type Reward = ();
	type RewardRemainder = Treasury;
	type RuntimeEvent = RuntimeEvent;
	type SessionInterface = Self;
	// rewards are minted from the void
	type SessionsPerEra = constants::staking::SessionsPerEra;
	type Slash = Treasury;
	type SlashDeferDuration = constants::staking::SlashDeferDuration;
	// This a placeholder, to be introduced in the next PR as an instance of bags-list
	type TargetList = pallet_staking::UseValidatorsMap<Self>;
	type UnixTime = Timestamp;
	type VoterList = VoterList;
	type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
}

/// The numbers configured here could always be more than the the maximum limits of staking pallet
/// to ensure election snapshot will not run out of memory. For now, we set them to smaller values
/// since the staking is bounded and the weight pipeline takes hours for this single pallet.
pub struct ElectionProviderBenchmarkConfig;
impl pallet_election_provider_multi_phase::BenchmarkingConfig for ElectionProviderBenchmarkConfig {
	const ACTIVE_VOTERS: [u32; 2] = [500, 800];
	const DESIRED_TARGETS: [u32; 2] = [200, 400];
	const MAXIMUM_TARGETS: u32 = 300;
	const MINER_MAXIMUM_VOTERS: u32 = 1000;
	const SNAPSHOT_MAXIMUM_VOTERS: u32 = 1000;
	const TARGETS: [u32; 2] = [500, 1000];
	const VOTERS: [u32; 2] = [1000, 2000];
}

/// Maximum number of iterations for balancing that will be executed in the embedded OCW
/// miner of election provider multi phase.
pub const MINER_MAX_ITERATIONS: u32 = 10;

/// A source of random balance for NposSolver, which is meant to be run by the OCW election miner.
pub struct OffchainRandomBalancing;
impl Get<Option<BalancingConfig>> for OffchainRandomBalancing {
	fn get() -> Option<BalancingConfig> {
		use sp_runtime::traits::TrailingZeroInput;
		let iterations = match MINER_MAX_ITERATIONS {
			0 => 0,
			max => {
				let seed = sp_io::offchain::random_seed();
				let random = <u32>::decode(&mut TrailingZeroInput::new(&seed))
					.expect("input is padded with zeroes; qed")
					% max.saturating_add(1);
				random as usize
			},
		};

		let config = BalancingConfig {
			iterations,
			tolerance: 0,
		};
		Some(config)
	}
}

pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
	type DataProvider = <Runtime as pallet_election_provider_multi_phase::Config>::DataProvider;
	type MaxWinners = <Runtime as pallet_election_provider_multi_phase::Config>::MaxWinners;
	type Solver = SequentialPhragmen<
		AccountId,
		pallet_election_provider_multi_phase::SolutionAccuracyOf<Runtime>,
	>;
	type System = Runtime;
	type TargetsBound = constants::staking::MaxValidators;
	type VotersBound = constants::staking::MaxOnChainElectingVoters;
	type WeightInfo = frame_election_provider_support::weights::SubstrateWeight<Runtime>;
}

impl pallet_election_provider_multi_phase::MinerConfig for Runtime {
	type AccountId = AccountId;
	type MaxLength = constants::staking::MinerMaxLength;
	type MaxVotesPerVoter =
	<<Self as pallet_election_provider_multi_phase::Config>::DataProvider as ElectionDataProvider>::MaxVotesPerVoter;
	type MaxWeight = constants::staking::MinerMaxWeight;
	type Solution = constants::staking::NposSolution16;

	// The unsigned submissions have to respect the weight of the submit_unsigned call, thus their
	// weight estimate function is wired to this call's weight.
	fn solution_weight(v: u32, t: u32, a: u32, d: u32) -> Weight {
		<
			<Self as pallet_election_provider_multi_phase::Config>::WeightInfo
			as
			pallet_election_provider_multi_phase::WeightInfo
		>::submit_unsigned(v, t, a, d)
	}
}

impl pallet_election_provider_multi_phase::Config for Runtime {
	type BenchmarkingConfig = ElectionProviderBenchmarkConfig;
	type BetterSignedThreshold = ();
	type BetterUnsignedThreshold = constants::staking::BetterUnsignedThreshold;
	type Currency = Balances;
	// nothing to do upon rewards
	type DataProvider = Staking;
	type EstimateCallFee = TransactionPayment;
	type Fallback = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type ForceOrigin = EnsureRootOrHalfCouncil;
	type GovernanceFallback = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type MaxElectableTargets = constants::staking::MaxOnChainElectableTargets;
	type MaxElectingVoters = constants::staking::MaxElectingVoters;
	type MaxWinners = constants::staking::MaxActiveValidators;
	type MinerConfig = Self;
	type MinerTxPriority = constants::staking::MultiPhaseUnsignedPriority;
	type OffchainRepeat = constants::staking::OffchainRepeat;
	// burn slashes
	type RewardHandler = ();
	type RuntimeEvent = RuntimeEvent;
	type SignedDepositBase = constants::staking::SignedDepositBase;
	type SignedDepositByte = constants::staking::SignedDepositByte;
	type SignedDepositWeight = ();
	type SignedMaxRefunds = constants::staking::SignedMaxRefunds;
	type SignedMaxSubmissions = constants::staking::SignedMaxSubmissions;
	type SignedMaxWeight = constants::staking::MinerMaxWeight;
	type SignedPhase = constants::staking::SignedPhase;
	type SignedRewardBase = constants::staking::SignedRewardBase;
	type SlashHandler = ();
	type Solver = SequentialPhragmen<AccountId, SolutionAccuracyOf<Self>, OffchainRandomBalancing>;
	type UnsignedPhase = constants::staking::UnsignedPhase;
	type WeightInfo = pallet_election_provider_multi_phase::weights::SubstrateWeight<Self>;
}

parameter_types! {
	pub const BagThresholds: &'static [u64] = &voter_bags::THRESHOLDS;
}

type VoterBagsListInstance = pallet_bags_list::Instance1;
impl pallet_bags_list::Config<VoterBagsListInstance> for Runtime {
	type BagThresholds = BagThresholds;
	type RuntimeEvent = RuntimeEvent;
	type Score = VoteWeight;
	/// The voter bags-list is loosely kept up to date, and the real source of truth for the score
	/// of each node is the staking pallet.
	type ScoreProvider = Staking;
	type WeightInfo = pallet_bags_list::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const PostUnbondPoolsWindow: u32 = 4;
	pub const NominationPoolsPalletId: PalletId = PalletId(*b"py/nopls");
	pub const MaxPointsToBalance: u8 = 10;
}

use sp_runtime::traits::Convert;
pub struct BalanceToU256;
impl Convert<Balance, sp_core::U256> for BalanceToU256 {
	fn convert(balance: Balance) -> sp_core::U256 { sp_core::U256::from(balance) }
}
pub struct U256ToBalance;
impl Convert<sp_core::U256, Balance> for U256ToBalance {
	fn convert(n: sp_core::U256) -> Balance { n.try_into().unwrap_or(Balance::max_value()) }
}

impl pallet_nomination_pools::Config for Runtime {
	type BalanceToU256 = BalanceToU256;
	type Currency = Balances;
	type MaxMetadataLen = ConstU32<256>;
	type MaxPointsToBalance = MaxPointsToBalance;
	type MaxUnbonding = ConstU32<8>;
	type PalletId = NominationPoolsPalletId;
	type PostUnbondingPoolsWindow = PostUnbondPoolsWindow;
	type RewardCounter = FixedU128;
	type RuntimeEvent = RuntimeEvent;
	type Staking = Staking;
	type U256ToBalance = U256ToBalance;
	type WeightInfo = ();
}

impl pallet_democracy::Config for Runtime {
	type BlacklistOrigin = EnsureRoot<AccountId>;
	// To cancel a proposal before it has been passed, the technical committee must be unanimous or
	// Root must agree.
	type CancelProposalOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 1, 1>,
	>;
	// To cancel a proposal which has been passed, 2/3 of the council must agree to it.
	type CancellationOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 2, 3>;
	type CooloffPeriod = constants::democracy::CooloffPeriod;
	type Currency = Balances;
	type EnactmentPeriod = constants::democracy::EnactmentPeriod;
	/// A unanimous council can have the next scheduled referendum be a straight default-carries
	/// (NTB) vote.
	type ExternalDefaultOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 1>;
	/// A super-majority can have the next scheduled referendum be a straight majority-carries vote.
	type ExternalMajorityOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 3, 4>;
	/// A straight majority of the council can decide what their next motion is.
	type ExternalOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>;
	/// Two thirds of the technical committee can have an ExternalMajority/ExternalDefault vote
	/// be tabled immediately and with a shorter voting/enactment period.
	type FastTrackOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 2, 3>;
	type FastTrackVotingPeriod = constants::democracy::FastTrackVotingPeriod;
	type InstantAllowed = constants::democracy::InstantAllowed;
	type InstantOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 1, 1>;
	type LaunchPeriod = constants::democracy::LaunchPeriod;
	type MaxBlacklisted = constants::democracy::MaxBlacklisted;
	type MaxDeposits = constants::democracy::MaxDeposits;
	type MaxProposals = constants::democracy::MaxProposals;
	type MaxVotes = constants::democracy::MaxVotes;
	// Same as EnactmentPeriod
	type MinimumDeposit = constants::democracy::MinimumDeposit;
	type PalletsOrigin = OriginCaller;
	type Preimages = Preimage;
	type RuntimeEvent = RuntimeEvent;
	type Scheduler = Scheduler;
	type Slash = Treasury;
	// Any single technical committee member may veto a coming council proposal, however they can
	// only do it once and it lasts only for the cool-off period.
	type VetoOrigin = pallet_collective::EnsureMember<AccountId, TechnicalCollective>;
	type VoteLockingPeriod = constants::democracy::EnactmentPeriod;
	type VotingPeriod = constants::democracy::VotingPeriod;
	type WeightInfo = pallet_democracy::weights::SubstrateWeight<Runtime>;
}

type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Config<CouncilCollective> for Runtime {
	type DefaultVote = pallet_collective::MoreThanMajorityThenPrimeDefaultVote;
	type MaxMembers = constants::council::MaxMembers;
	type MaxProposals = constants::council::MaxProposals;
	type MotionDuration = constants::council::MotionDuration;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type WeightInfo = pallet_collective::weights::SubstrateWeight<Runtime>;
}

impl pallet_elections_phragmen::Config for Runtime {
	type CandidacyBond = constants::elections::CandidacyBond;
	type ChangeMembers = Council;
	type Currency = Balances;
	type CurrencyToVote = U128CurrencyToVote;
	type DesiredMembers = constants::elections::DesiredMembers;
	type DesiredRunnersUp = constants::elections::DesiredRunnersUp;
	// NOTE: this implies that council's genesis members cannot be set directly and must come from
	// this module.
	type InitializeMembers = Council;
	type KickedMember = Treasury;
	type LoserCandidate = Treasury;
	type MaxCandidates = constants::elections::MaxCandidates;
	type MaxVoters = constants::elections::MaxVoters;
	type PalletId = constants::elections::PalletId;
	type RuntimeEvent = RuntimeEvent;
	type TermDuration = constants::elections::TermDuration;
	type VotingBondBase = constants::elections::VotingBondBase;
	type VotingBondFactor = constants::elections::VotingBondFactor;
	type WeightInfo = pallet_elections_phragmen::weights::SubstrateWeight<Runtime>;
}

type TechnicalCollective = pallet_collective::Instance2;
impl pallet_collective::Config<TechnicalCollective> for Runtime {
	type DefaultVote = pallet_collective::MoreThanMajorityThenPrimeDefaultVote;
	type MaxMembers = constants::technical::TechnicalMaxMembers;
	type MaxProposals = constants::technical::TechnicalMaxProposals;
	type MotionDuration = constants::technical::TechnicalMotionDuration;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type WeightInfo = pallet_collective::weights::SubstrateWeight<Runtime>;
}

type EnsureRootOrHalfCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>,
>;
impl pallet_membership::Config<pallet_membership::Instance1> for Runtime {
	type AddOrigin = EnsureRootOrHalfCouncil;
	type MaxMembers = constants::technical::TechnicalMaxMembers;
	type MembershipChanged = TechnicalCommittee;
	type MembershipInitialized = TechnicalCommittee;
	type PrimeOrigin = EnsureRootOrHalfCouncil;
	type RemoveOrigin = EnsureRootOrHalfCouncil;
	type ResetOrigin = EnsureRootOrHalfCouncil;
	type RuntimeEvent = RuntimeEvent;
	type SwapOrigin = EnsureRootOrHalfCouncil;
	type WeightInfo = pallet_membership::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: Balance = AVL;
	pub const SpendPeriod: BlockNumber = DAYS;
	pub const Burn: Permill = Permill::from_percent(50);
	pub const TipCountdown: BlockNumber = DAYS;
	pub const TipFindersFee: Percent = Percent::from_percent(20);
	pub const TipReportDepositBase: Balance = AVL;
	pub const DataDepositPerByte: Balance = CENTS;
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	pub const MaximumReasonLength: u32 = 16384;
	pub const MaxApprovals: u32 = 100;
}

impl pallet_treasury::Config for Runtime {
	type ApproveOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 3, 5>,
	>;
	type Burn = Burn;
	type BurnDestination = ();
	type Currency = Balances;
	type MaxApprovals = MaxApprovals;
	type OnSlash = Treasury;
	type PalletId = TreasuryPalletId;
	type ProposalBond = ProposalBond;
	type ProposalBondMaximum = ();
	type ProposalBondMinimum = ProposalBondMinimum;
	type RejectOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>,
	>;
	type RuntimeEvent = RuntimeEvent;
	type SpendFunds = Bounties;
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<u128>;
	type SpendPeriod = SpendPeriod;
	type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>;
}

impl pallet_bounties::Config for Runtime {
	type BountyDepositBase = constants::bounty::DepositBase;
	type BountyDepositPayoutDelay = constants::bounty::DepositPayoutDelay;
	type BountyUpdatePeriod = constants::bounty::UpdatePeriod;
	type BountyValueMinimum = constants::bounty::ValueMinimum;
	type ChildBountyManager = ();
	type CuratorDepositMax = constants::bounty::CuratorDepositMax;
	type CuratorDepositMin = constants::bounty::CuratorDepositMin;
	type CuratorDepositMultiplier = constants::bounty::CuratorDepositMultiplier;
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_bounties::weights::SubstrateWeight<Runtime>;
}

impl pallet_tips::Config for Runtime {
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type RuntimeEvent = RuntimeEvent;
	type TipCountdown = TipCountdown;
	type TipFindersFee = TipFindersFee;
	type TipReportDepositBase = TipReportDepositBase;
	type Tippers = Elections;
	type WeightInfo = pallet_tips::weights::SubstrateWeight<Runtime>;
}

impl pallet_sudo::Config for Runtime {
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: RuntimeCall,
		public: <Signature as traits::Verify>::Signer,
		account: AccountId,
		nonce: Index,
	) -> Option<(
		RuntimeCall,
		<UncheckedExtrinsic as traits::Extrinsic>::SignaturePayload,
	)> {
		use codec::Encode;
		use sp_runtime::{traits::StaticLookup, SaturatedConversion as _};

		let tip = 0;
		// take the biggest period possible.
		let period = BlockHashCount::get()
			.checked_next_power_of_two()
			.map(|c| c / 2)
			.unwrap_or(2) as u64;
		let current_block = System::block_number()
			.saturated_into::<u64>()
			// The `System::block_number` is initialized with `n+1`,
			// so the actual block number is `n`.
			.saturating_sub(1);
		let era = Era::mortal(period, current_block);
		let extra = (
			frame_system::CheckNonZeroSender::<Runtime>::new(),
			frame_system::CheckSpecVersion::<Runtime>::new(),
			frame_system::CheckTxVersion::<Runtime>::new(),
			frame_system::CheckGenesis::<Runtime>::new(),
			frame_system::CheckEra::<Runtime>::from(era),
			frame_system::CheckNonce::<Runtime>::from(nonce),
			frame_system::CheckWeight::<Runtime>::new(),
			pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
			da_control::CheckAppId::<Runtime>::from(AppId(0)),
		);
		let raw_payload = SignedPayload::new(call, extra)
			.map_err(|e| {
				log::warn!("Unable to create signed payload: {:?}", e);
			})
			.ok()?;
		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
		let address = Indices::unlookup(account);
		let (call, extra, _) = raw_payload.deconstruct();
		Some((call, (address, signature, extra)))
	}
}

impl frame_system::offchain::SigningTypes for Runtime {
	type Public = <Signature as traits::Verify>::Signer;
	type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
	RuntimeCall: From<C>,
{
	type Extrinsic = UncheckedExtrinsic;
	type OverarchingCall = RuntimeCall;
}

impl pallet_im_online::Config for Runtime {
	type AuthorityId = ImOnlineId;
	type MaxKeys = constants::im::MaxKeys;
	type MaxPeerDataEncodingSize = constants::im::MaxPeerDataEncodingSize;
	type MaxPeerInHeartbeats = constants::im::MaxPeerInHeartbeats;
	type NextSessionRotation = Babe;
	type ReportUnresponsiveness = Offences;
	type RuntimeEvent = RuntimeEvent;
	type UnsignedPriority = constants::im::ImOnlineUnsignedPriority;
	type ValidatorSet = Historical;
	type WeightInfo = pallet_im_online::weights::SubstrateWeight<Runtime>;
}

impl pallet_offences::Config for Runtime {
	type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = Staking;
	type RuntimeEvent = RuntimeEvent;
}

impl pallet_authority_discovery::Config for Runtime {
	type MaxAuthorities = constants::system::MaxAuthorities;
}

impl pallet_grandpa::Config for Runtime {
	type HandleEquivocation = pallet_grandpa::EquivocationHandler<
		Self::KeyOwnerIdentification,
		Offences,
		constants::babe::ReportLongevity,
	>;
	type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		GrandpaId,
	)>>::IdentificationTuple;
	type KeyOwnerProof =
		<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
	type KeyOwnerProofSystem = Historical;
	type MaxAuthorities = constants::system::MaxAuthorities;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

impl pallet_mmr::Config for Runtime {
	type Hash = <Runtime as frame_system::Config>::Hash;
	type Hashing = <Runtime as frame_system::Config>::Hashing;
	type LeafData = pallet_mmr::ParentNumberAndHash<Self>;
	type OnNewRoot = ();
	type WeightInfo = ();

	const INDEXING_PREFIX: &'static [u8] = b"mmr";
}

impl da_control::Config for Runtime {
	type BlockLenProposalId = u32;
	type MaxAppDataLength = constants::da::MaxAppDataLength;
	type MaxAppKeyLength = constants::da::MaxAppKeyLength;
	type MaxBlockCols = constants::da::MaxBlockCols;
	type MaxBlockRows = constants::da::MaxBlockRows;
	type MinBlockCols = constants::da::MinBlockCols;
	type MinBlockRows = constants::da::MinBlockRows;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = da_control::weights::SubstrateWeight<Runtime>;
}

impl nomad_updater_manager::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

impl nomad_home::Config for Runtime {
	type MaxMessageBodyBytes = constants::nomad::MaxMessageBodyBytes;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = nomad_home::weights::SubstrateWeight<Runtime>;
}

impl nomad_da_bridge::Config for Runtime {
	type DABridgePalletId = constants::nomad::DABridgePalletId;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = nomad_da_bridge::weights::SubstrateWeight<Runtime>;
}

// TODO @miguel Aline this with previous order and ID to keep the compatibility.
// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = NodeBlock,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system = 0,
		Utility: pallet_utility = 1,
		Babe: pallet_babe = 2,
		Timestamp: pallet_timestamp = 3,
		Authorship: pallet_authorship = 4,
		Indices: pallet_indices = 5,
		Balances: pallet_balances = 6,
		TransactionPayment: pallet_transaction_payment = 7,

		ElectionProviderMultiPhase: pallet_election_provider_multi_phase = 9,
		Staking: pallet_staking = 10,
		Session: pallet_session = 11,
		Democracy: pallet_democracy = 12,
		Council: pallet_collective::<Instance1> = 13,
		TechnicalCommittee: pallet_collective::<Instance2> = 14,
		Elections: pallet_elections_phragmen = 15,
		TechnicalMembership: pallet_membership::<Instance1> = 16,
		Grandpa: pallet_grandpa = 17,
		Treasury: pallet_treasury = 18,

		Sudo: pallet_sudo = 19,
		ImOnline: pallet_im_online = 20,
		AuthorityDiscovery: pallet_authority_discovery = 21,
		Offences: pallet_offences = 22,
		Historical: pallet_session_historical = 23,

		Scheduler: pallet_scheduler = 24,
		Bounties: pallet_bounties = 25,
		Tips: pallet_tips = 26,
		Mmr: pallet_mmr = 27,
		// BagsList: pallet_bags_list = 28,

		// DA module
		DataAvailability: da_control = 29,

		// Nomad
		NomadUpdaterManager: nomad_updater_manager = 30,
		NomadHome: nomad_home = 31,
		NomadDABridge: nomad_da_bridge = 32,

		// More from upgrade to v0.9.33
		Preimage: pallet_preimage = 33,
		Multisig: pallet_multisig = 34,
		VoterList: pallet_bags_list::<Instance1> = 35,
		NominationPools: pallet_nomination_pools = 36,
	}
);

/// MMR helper types.
pub(crate) mod mmr {
	pub use pallet_mmr::primitives::*;

	use super::Runtime;

	pub type Leaf = <<Runtime as pallet_mmr::Config>::LeafData as LeafDataProvider>::LeafData;
	pub type Hash = <Runtime as pallet_mmr::Config>::Hash;
	pub type Hashing = <Runtime as pallet_mmr::Config>::Hashing;
}

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	define_benchmarks!(
		[frame_benchmarking, BaselineBench::<Runtime>]
		[frame_system, SystemBench::<Runtime>]
		[da_control, crate::DataAvailability]
		[nomad_home, crate::NomadHome]
		[nomad_da_bridge, crate::NomadDABridge]
	);
}

#[cfg(test)]
mod tests {
	use core::mem::size_of;
	use std::collections::HashSet;

	use frame_election_provider_support::NposSolution;
	use frame_support::traits::WhitelistedStorageKeys;
	use frame_system::offchain::CreateSignedTransaction;
	use hex_literal::hex;
	use sp_core::hexdisplay::HexDisplay;
	use sp_keyring::AccountKeyring::Bob;
	use sp_runtime::{MultiAddress, UpperOf};
	use test_case::test_case;

	use super::*;

	/// This test was used to detect any missing support of `TryState` needed for `try-runtime`
	/// feature.
	#[cfg(feature = "try-runtime")]
	#[allow(dead_code)]
	fn check_try_runtime_support_on_pallets() -> Result<(), &'static str> {
		use frame_support::traits::{TryState, TryStateSelect::All};
		use sp_runtime::traits::Zero;

		let block = Zero::zero();

		<frame_system::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;

		<pallet_utility::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_babe::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_timestamp::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_authorship::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_indices::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_balances::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_transaction_payment::Pallet<Runtime> as TryState<BlockNumber>>::try_state(
			block, All,
		)?;
		<pallet_election_provider_multi_phase::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_staking::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_session::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_democracy::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_collective::Pallet<Runtime, CouncilCollective> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_collective::Pallet<Runtime, TechnicalCollective> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_elections_phragmen::Pallet<Runtime> as TryState<BlockNumber>>::try_state(
			block, All,
		)?;
		<pallet_membership::Pallet<Runtime, pallet_membership::Instance1> as TryState<
			BlockNumber,
		>>::try_state(block, All)?;
		<pallet_grandpa::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_treasury::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_sudo::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_im_online::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_authority_discovery::Pallet<Runtime> as TryState<BlockNumber>>::try_state(
			block, All,
		)?;
		<pallet_offences::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_session::historical::Pallet<Runtime> as TryState<BlockNumber>>::try_state(
			block, All,
		)?;
		<pallet_scheduler::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_bounties::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_tips::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_mmr::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<da_control::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<nomad_updater_manager::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<nomad_home::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<nomad_da_bridge::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_preimage::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_multisig::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)?;
		<pallet_bags_list::Pallet<Runtime, pallet_bags_list::Instance1> as TryState<
			BlockNumber,
		>>::try_state(block, All)?;
		<pallet_nomination_pools::Pallet<Runtime> as TryState<BlockNumber>>::try_state(block, All)
	}

	#[test]
	fn check_whitelist() {
		let whitelist: HashSet<String> = AllPalletsWithSystem::whitelisted_storage_keys()
			.iter()
			.map(|e| HexDisplay::from(&e.key).to_string())
			.collect();

		// Block Number
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac")
		);
		// Total Issuance
		assert!(
			whitelist.contains("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80")
		);
		// Execution Phase
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a")
		);
		// Event Count
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850")
		);
		// System Events
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7")
		);
		// System BlockWeight
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef734abf5cb34d6244378cddbf18e849d96")
		);
	}

	#[test]
	fn validate_transaction_submitter_bounds() {
		fn is_submit_signed_transaction<T>()
		where
			T: CreateSignedTransaction<RuntimeCall>,
		{
		}

		is_submit_signed_transaction::<Runtime>();
	}

	#[test]
	fn perbill_as_onchain_accuracy() {
		type OnChainAccuracy =
			<<Runtime as pallet_election_provider_multi_phase::MinerConfig>::Solution as NposSolution>::Accuracy;
		let maximum_chain_accuracy: Vec<UpperOf<OnChainAccuracy>> = (0
			..constants::staking::MaxNominations::get())
			.map(|_| <UpperOf<OnChainAccuracy>>::from(OnChainAccuracy::one().deconstruct()))
			.collect();
		let _: UpperOf<OnChainAccuracy> = maximum_chain_accuracy
			.iter()
			.fold(0, |acc, x| acc.checked_add(*x).unwrap());
	}

	const RUNTIME_CALL_SIZE: usize = size_of::<RuntimeCall>();
	const DA_CALL_SIZE: usize = size_of::<da_control::Call<Runtime>>();
	const SYSTEM_CALL_SIZE: usize = size_of::<frame_system::Call<Runtime>>();
	const NOMAD_UPDATER_MANAGER_CALL_SIZE: usize =
		size_of::<nomad_updater_manager::Call<Runtime>>();
	const NOMAD_HOME_CALL_SIZE: usize = size_of::<nomad_home::Call<Runtime>>();
	const NOMAD_BRIDGE_CALL_SIZE: usize = size_of::<nomad_da_bridge::Call<Runtime>>();

	#[test_case( RUNTIME_CALL_SIZE => 160)]
	#[test_case( DA_CALL_SIZE => 32)]
	#[test_case( SYSTEM_CALL_SIZE => 32)]
	#[test_case( NOMAD_UPDATER_MANAGER_CALL_SIZE => 0)]
	#[test_case( NOMAD_HOME_CALL_SIZE => 152)]
	#[test_case( NOMAD_BRIDGE_CALL_SIZE => 48)]
	fn call_size(size: usize) -> usize {
		const MAX_CALL_SIZE: usize = 208;
		assert!(
			size <= MAX_CALL_SIZE,
			"size of RuntimeCall {} is more than 208 bytes: some calls have too big arguments, use Box to reduce the
			size of RuntimeCall.
			If the limit is too strong, maybe consider increase the limit to 300.",
			size,
		);
		size
	}

	const TRANSFER_RAW : &[u8]= &hex!("b4040600008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a4813000064a7b3b6e00d");
	/// Creates a transfer tx of 1 AVL to Bob.
	fn transfer_expected() -> RuntimeCall {
		RuntimeCall::Balances(pallet_balances::Call::transfer {
			dest: MultiAddress::Id(Bob.to_account_id()),
			value: 1 * AVL,
		})
	}

	const SET_TIMESTAMP_RAW: &[u8] = &hex!("280403000ba302ac318301");
	// `set_timestamp` extrinsic from block 13852 on DevNet.
	fn set_timestamp_expected() -> RuntimeCall {
		RuntimeCall::Timestamp(pallet_timestamp::Call::set {
			now: 1_662_985_700_003,
		})
	}

	#[test_case( &TRANSFER_RAW => transfer_expected(); "Transfer 1 AVL to Bob")]
	#[test_case( &SET_TIMESTAMP_RAW => set_timestamp_expected(); "set_timestamp_block_242")]
	fn decode_app_unchecked_extrinsics(mut raw_ext: &[u8]) -> RuntimeCall {
		let app_ext = UncheckedExtrinsic::decode(&mut raw_ext).expect("Valid raw extrinsic .qed");
		app_ext.function
	}
}
