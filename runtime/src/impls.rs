use crate::voter_bags;
use crate::SessionKeys;
use crate::SLOT_DURATION;
use crate::{
	constants, prod_or_fast, weights, AccountId, AccountIndex, Babe, Balances, Block, BlockNumber,
	Bounties, ElectionProviderMultiPhase, GrandpaId, Hash, Historical, ImOnline, ImOnlineId, Index,
	Indices, Moment, NominationPools, Offences, OriginCaller, PalletInfo, Preimage,
	ReserveIdentifier, Runtime, RuntimeCall, RuntimeEvent, RuntimeHoldReason, RuntimeOrigin,
	RuntimeVersion, Session, Signature, SignedPayload, Staking, System, TechnicalCommittee,
	Timestamp, TransactionPayment, Treasury, UncheckedExtrinsic, VoterList, MINUTES, VERSION,
};
use avail_core::currency::{Balance, AVL, CENTS, NANO_AVL, PICO_AVL};
use avail_core::AppId;
use avail_core::OpaqueExtrinsic;
use avail_core::NORMAL_DISPATCH_RATIO;
use codec::Decode;
use constants::time::DAYS;
use frame_election_provider_support::onchain;
use frame_election_provider_support::BalancingConfig;
use frame_election_provider_support::ElectionDataProvider;
use frame_election_provider_support::SequentialPhragmen;
use frame_election_provider_support::VoteWeight;
use frame_support::pallet_prelude::Get;
use frame_support::pallet_prelude::Weight;
use frame_support::traits::tokens::Imbalance;
use frame_support::traits::ConstU32;
use frame_support::traits::ContainsLengthBound;
use frame_support::traits::EqualPrivilegeOnly;
use frame_support::traits::Everything;
use frame_support::traits::KeyOwnerProofSystem;
use frame_support::traits::SortedMembers;
use frame_support::traits::{Currency, OnUnbalanced};
use frame_support::weights::constants::RocksDbWeight;
use frame_support::weights::ConstantMultiplier;
use frame_support::{parameter_types, traits::EitherOfDiverse, PalletId};
use frame_system::limits::BlockLength;
use frame_system::submitted_data;
use frame_system::EnsureRoot;
use pallet_election_provider_multi_phase::SolutionAccuracyOf;
use pallet_succinct::{
	AvailAssetId, BridgePalletId, MaxBridgeDataLength, MaxProofLength, MaxVerificationKeyLength,
	MessageMappingStorageIndex, RotateFunctionId, StepFunctionId,
};
use pallet_transaction_payment::CurrencyAdapter;
use pallet_transaction_payment::Multiplier;
use pallet_transaction_payment::TargetedFeeAdjustment;
use sp_core::crypto::KeyTypeId;
use sp_runtime::generic::Era;
use sp_runtime::traits;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::traits::Bounded;
use sp_runtime::traits::Convert;
use sp_runtime::traits::OpaqueKeys;
use sp_runtime::FixedPointNumber;
use sp_runtime::FixedU128;
use sp_runtime::Perbill;
use sp_runtime::Perquintill;
use sp_runtime::{Percent, Permill};
use sp_std::rc::Rc;
use sp_std::vec;
use sp_std::vec::Vec;

pub type NegativeImbalance<T> = <pallet_balances::Pallet<T> as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

impl pallet_sudo::Config for Runtime {
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_sudo::weights::SubstrateWeight<Runtime>;
}

impl pallet_mandate::Config for Runtime {
	type ApprovedOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionMoreThan<AccountId, TechnicalCollective, 1, 2>,
	>;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_mandate::WeightInfo<Runtime>;
}

impl pallet_succinct::Config for Runtime {
	type MaxProofLength = MaxProofLength;
	type MaxVerificationKeyLength = MaxVerificationKeyLength;
	type MessageMappingStorageIndex = MessageMappingStorageIndex;
	type StepFunctionId = StepFunctionId;
	type RotateFunctionId = RotateFunctionId;

	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_succinct::WeightInfo<Runtime>;

	type TimeProvider = pallet_timestamp::Pallet<Runtime>;

	type Currency = Balances;

	type PalletId = BridgePalletId;

	type AvailAssetId = AvailAssetId;

	type MaxBridgeDataLength = MaxBridgeDataLength;
}

parameter_types! {
	pub const BasicDeposit: Balance = 10 * AVL;
	pub const FieldDeposit: Balance = 250 * CENTS;
	pub const SubAccountDeposit: Balance = 2 * AVL;
	pub const MaxSubAccounts: u32 = 100;
	pub const MaxAdditionalFields: u32 = 100;
	pub const MaxRegistrars: u32 = 20;
}

impl pallet_identity::Config for Runtime {
	/// The amount held on deposit for a registered identity.
	type BasicDeposit = BasicDeposit;
	type Currency = Balances;
	/// The amount held on deposit per additional field for a registered identity
	type FieldDeposit = FieldDeposit;
	/// The origin which may forcibly set or remove a name. Root can always do this.
	type ForceOrigin = EnsureRoot<AccountId>;
	/// Maximum number of additional fields that may be stored in an ID.
	type MaxAdditionalFields = MaxAdditionalFields;
	/// Maxmimum number of registrars allowed in the system.
	type MaxRegistrars = MaxRegistrars;
	/// The maximum number of sub-accounts allowed per identified account.
	type MaxSubAccounts = MaxSubAccounts;
	/// The origin which may add or remove registrars. Root can always do this.
	type RegistrarOrigin = EnsureRoot<AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type Slashed = Treasury;
	/// The amount held on deposit for a registered subaccount.
	type SubAccountDeposit = SubAccountDeposit;
	type WeightInfo = weights::pallet_identity::WeightInfo<Runtime>;
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
	type WeightInfo = weights::pallet_dactr::WeightInfo<Runtime>;
}

impl nomad_updater_manager::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

impl nomad_home::Config for Runtime {
	type MaxMessageBodyBytes = constants::nomad::MaxMessageBodyBytes;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::nomad_home::WeightInfo<Runtime>;
}

impl nomad_da_bridge::Config for Runtime {
	type DABridgePalletId = constants::nomad::DABridgePalletId;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::nomad_da_bridge::WeightInfo<Runtime>;
}

impl pallet_offences::Config for Runtime {
	type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = Staking;
	type RuntimeEvent = RuntimeEvent;
}

impl pallet_authority_discovery::Config for Runtime {
	type MaxAuthorities = constants::MaxAuthorities;
}

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: Balance = AVL;
	pub const SpendPeriod: BlockNumber = DAYS;
	pub const Burn: Permill = Permill::from_percent(0); // Not burning any funds for now
	pub const TipCountdown: BlockNumber = DAYS;
	pub const TipFindersFee: Percent = Percent::from_percent(20);
	pub const TipReportDepositBase: Balance = AVL;
	pub const DataDepositPerByte: Balance = CENTS;
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	pub const MaximumReasonLength: u32 = 16384;
	pub const MaxApprovals: u32 = 100;
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
	type WeightInfo = weights::pallet_bounties::WeightInfo<Runtime>;
}

pub struct Tippers;

impl SortedMembers<AccountId> for Tippers {
	fn sorted_members() -> Vec<AccountId> {
		let Some(account) = pallet_sudo::Pallet::<Runtime>::key() else {
			return vec![];
		};

		vec![account]
	}
}

impl ContainsLengthBound for Tippers {
	fn min_len() -> usize {
		0
	}

	fn max_len() -> usize {
		1
	}
}

impl pallet_tips::Config for Runtime {
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type RuntimeEvent = RuntimeEvent;
	type TipCountdown = TipCountdown;
	type TipFindersFee = TipFindersFee;
	type TipReportDepositBase = TipReportDepositBase;
	type Tippers = Tippers;
	type WeightInfo = weights::pallet_tips::WeightInfo<Runtime>;
}

parameter_types! {
	pub const WeightFee: Balance = PICO_AVL;
	pub const TransactionByteFee: Balance = 100 * NANO_AVL; // 100 nanoAVL
	pub const OperationalFeeMultiplier: u8 = 5u8;
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(50); // target_utilization 50%
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000); // 0.000001
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
	type OnChargeTransaction = CurrencyAdapter<Balances, DealWithFees<Runtime>>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type RuntimeEvent = RuntimeEvent;
	type WeightToFee = ConstantMultiplier<Balance, WeightFee>; // 1 weight = 1 picoAVL -> second_price = 1 AVL
}

parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
	type MinimumPeriod = MinimumPeriod;
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = Moment;
	type OnTimestampSet = Babe;
	type WeightInfo = weights::pallet_timestamp::WeightInfo<Runtime>;
}

impl pallet_authorship::Config for Runtime {
	type EventHandler = (Staking, ImOnline);
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
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

/// Logic for the author to get a portion of fees.
pub struct Author<R>(sp_std::marker::PhantomData<R>);

impl<R> OnUnbalanced<NegativeImbalance<R>> for Author<R>
where
	R: pallet_balances::Config + pallet_authorship::Config,
	<R as frame_system::Config>::AccountId: From<AccountId>,
	<R as frame_system::Config>::AccountId: Into<AccountId>,
{
	fn on_nonzero_unbalanced(amount: NegativeImbalance<R>) {
		if let Some(author) = <pallet_authorship::Pallet<R>>::author() {
			<pallet_balances::Pallet<R>>::resolve_creating(&author, amount);
		}
	}
}

pub struct DealWithFees<R>(sp_std::marker::PhantomData<R>);

impl<R> OnUnbalanced<NegativeImbalance<R>> for DealWithFees<R>
where
	R: pallet_balances::Config + pallet_treasury::Config + pallet_authorship::Config,
	pallet_treasury::Pallet<R>: OnUnbalanced<NegativeImbalance<R>>,
	<R as frame_system::Config>::AccountId: From<AccountId>,
	<R as frame_system::Config>::AccountId: Into<AccountId>,
{
	fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance<R>>) {
		if let Some(fees) = fees_then_tips.next() {
			// for fees, 20% to author, 80% to treasury
			let mut split = fees.ration(80, 20);
			if let Some(tips) = fees_then_tips.next() {
				// for tips, if any, 100% to author
				tips.merge_into(&mut split.1);
			}
			use pallet_treasury::Pallet as Treasury;
			<Treasury<R> as OnUnbalanced<_>>::on_unbalanced(split.0);
			<Author<R> as OnUnbalanced<_>>::on_unbalanced(split.1);
		}
	}
}

impl pallet_utility::Config for Runtime {
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_utility::WeightInfo<Runtime>;
}

parameter_types! {
	pub const DepositBase: Balance = 2 * AVL;
	// Additional storage item size of 32 bytes.
	pub const DepositFactor: Balance = 5 * CENTS;
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
	type WeightInfo = weights::pallet_scheduler::WeightInfo<Runtime>;
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
	type EquivocationReportSystem = pallet_babe::EquivocationReportSystem<
		Self,
		Offences,
		Historical,
		constants::babe::ReportLongevity,
	>;
	type ExpectedBlockTime = constants::time::ExpectedBlockTime;
	type KeyOwnerProof =
		<Historical as KeyOwnerProofSystem<(KeyTypeId, pallet_babe::AuthorityId)>>::Proof;
	type MaxAuthorities = constants::MaxAuthorities;
	type WeightInfo = ();
}

impl pallet_indices::Config for Runtime {
	type AccountIndex = AccountIndex;
	type Currency = Balances;
	type Deposit = constants::indices::IndexDeposit;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_indices::WeightInfo<Runtime>;
}

impl pallet_balances::Config for Runtime {
	type AccountStore = frame_system::Pallet<Runtime>;
	/// The type for recording an account's balance.
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = constants::balances::ExistentialDeposit;
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type MaxHolds = ConstU32<2>;
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = ReserveIdentifier;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeHoldReason = RuntimeHoldReason;
	type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
}

impl pallet_im_online::Config for Runtime {
	type AuthorityId = ImOnlineId;
	type MaxKeys = ConstU32<10_000>;
	type MaxPeerInHeartbeats = ConstU32<10_000>;
	type NextSessionRotation = Babe;
	type ReportUnresponsiveness = Offences;
	type RuntimeEvent = RuntimeEvent;
	type UnsignedPriority = constants::im::ImOnlineUnsignedPriority;
	type ValidatorSet = Historical;
	type WeightInfo = pallet_im_online::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const MaxSetIdSessionEntries: u32 = constants::staking::BondingDuration::get() * constants::staking::SessionsPerEra::get();
}

impl pallet_grandpa::Config for Runtime {
	type EquivocationReportSystem = pallet_grandpa::EquivocationReportSystem<
		Self,
		Offences,
		Historical,
		constants::babe::ReportLongevity,
	>;
	type KeyOwnerProof = <Historical as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
	type MaxAuthorities = constants::MaxAuthorities;
	type MaxSetIdSessionEntries = MaxSetIdSessionEntries;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

parameter_types! {
	pub const TechnicalMotionDuration: BlockNumber = prod_or_fast!(5 * DAYS, 5 * MINUTES);
}
pub type TechnicalMaxMembers = ConstU32<100>;

pub type TechnicalCollective = pallet_collective::Instance2;

impl pallet_collective::Config<TechnicalCollective> for Runtime {
	type DefaultVote = pallet_collective::MoreThanMajorityThenPrimeDefaultVote;
	type MaxMembers = TechnicalMaxMembers;
	type MaxProposalWeight = constants::council::MaxProposalWeight;
	type MaxProposals = ConstU32<100>;
	type MotionDuration = TechnicalMotionDuration;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SetMembersOrigin = EnsureRoot<Self::AccountId>;
	type WeightInfo = pallet_collective::weights::SubstrateWeight<Runtime>;
}

impl pallet_membership::Config<pallet_membership::Instance1> for Runtime {
	type AddOrigin = EnsureRoot<AccountId>;
	type MaxMembers = TechnicalMaxMembers;
	type MembershipChanged = TechnicalCommittee;
	type MembershipInitialized = TechnicalCommittee;
	type PrimeOrigin = EnsureRoot<AccountId>;
	type RemoveOrigin = EnsureRoot<AccountId>;
	type ResetOrigin = EnsureRoot<AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type SwapOrigin = EnsureRoot<AccountId>;
	type WeightInfo = pallet_membership::weights::SubstrateWeight<Runtime>;
}

impl pallet_election_provider_multi_phase::MinerConfig for Runtime {
	type AccountId = AccountId;
	type MaxLength = constants::staking::MinerMaxLength;
	type MaxVotesPerVoter =
    <<Self as pallet_election_provider_multi_phase::Config>::DataProvider as ElectionDataProvider>::MaxVotesPerVoter;
	type MaxWeight = constants::staking::MinerMaxWeight;
	type MaxWinners = <Runtime as pallet_election_provider_multi_phase::Config>::MaxWinners;
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
	type ForceOrigin = EnsureRoot<AccountId>;
	type GovernanceFallback = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type MaxElectableTargets = constants::MaxElectableTargets;
	type MaxElectingVoters = constants::MaxElectingVoters;
	type MaxWinners = constants::MaxActiveValidators;
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

pub struct StakingBenchmarkingConfig;

impl pallet_staking::BenchmarkingConfig for StakingBenchmarkingConfig {
	type MaxNominators = constants::staking::MaxNominators;
	type MaxValidators = constants::staking::MaxValidators;
}

impl pallet_staking::Config for Runtime {
	/// A super-majority of the council can cancel the slash.
	type AdminOrigin = EnsureRoot<AccountId>;
	type BenchmarkingConfig = StakingBenchmarkingConfig;
	type BondingDuration = constants::staking::BondingDuration;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type CurrencyToVote = sp_staking::currency_to_vote::U128CurrencyToVote;
	type ElectionProvider = ElectionProviderMultiPhase;
	type EraPayout = pallet_staking::ConvertCurve<constants::staking::RewardCurve>;
	type EventListeners = NominationPools;
	type GenesisElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type HistoryDepth = constants::staking::HistoryDepth;
	type MaxNominations = constants::staking::MaxNominations;
	type MaxNominatorRewardedPerValidator = constants::staking::MaxNominatorRewardedPerValidator;
	type MaxUnlockingChunks = constants::staking::MaxUnlockingChunks;
	type NextNewSession = Session;
	type OffendingValidatorsThreshold = constants::staking::OffendingValidatorsThreshold;
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

/// The numbers configured here could always be more than the maximum limits of staking pallet
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
	type TargetsBound = constants::MaxElectableTargets;
	type VotersBound = constants::MaxElectingVoters;
	type WeightInfo = frame_election_provider_support::weights::SubstrateWeight<Runtime>;
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

pub struct BalanceToU256;

impl Convert<Balance, sp_core::U256> for BalanceToU256 {
	fn convert(balance: Balance) -> sp_core::U256 {
		sp_core::U256::from(balance)
	}
}

pub struct U256ToBalance;

impl Convert<sp_core::U256, Balance> for U256ToBalance {
	fn convert(n: sp_core::U256) -> Balance {
		n.try_into().unwrap_or(Balance::max_value())
	}
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

impl pallet_treasury::Config for Runtime {
	type ApproveOrigin = EnsureRoot<AccountId>;
	type Burn = Burn;
	type BurnDestination = ();
	type Currency = Balances;
	type MaxApprovals = MaxApprovals;
	type OnSlash = Treasury;
	type PalletId = TreasuryPalletId;
	type ProposalBond = ProposalBond;
	type ProposalBondMaximum = ();
	type ProposalBondMinimum = ProposalBondMinimum;
	type RejectOrigin = EnsureRoot<AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type SpendFunds = Bounties;
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<u128>;
	type SpendPeriod = SpendPeriod;
	type WeightInfo = weights::pallet_treasury::WeightInfo<Runtime>;
}

impl pallet_mmr::Config for Runtime {
	type Hashing = <Runtime as frame_system::Config>::Hashing;
	type LeafData = pallet_mmr::ParentNumberAndHash<Self>;
	type OnNewRoot = ();
	type WeightInfo = ();

	const INDEXING_PREFIX: &'static [u8] = b"mmr";
}

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	pub const Version: RuntimeVersion = VERSION;
	pub RuntimeBlockLength: BlockLength =
		BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
}

/// Filters and extracts `data` from `call` if it is a `DataAvailability::submit_data` type.
impl submitted_data::Filter<RuntimeCall> for Runtime {
	fn filter(
		call: RuntimeCall,
		metrics: submitted_data::RcMetrics,
	) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
		metrics.borrow_mut().total_extrinsics += 1;

		match call {
			RuntimeCall::Succinct(pallet_succinct::Call::submit_bridge_data { data })
				if !data.is_empty() =>
			{
				let mut metrics = metrics.borrow_mut();
				metrics.data_submit_leaves += 1;
				metrics.data_submit_extrinsics += 1;
				(vec![], vec![data.into_inner()])
			},
			RuntimeCall::DataAvailability(da_control::Call::submit_data { data })
				if !data.is_empty() =>
			{
				let mut metrics = metrics.borrow_mut();
				metrics.data_submit_leaves += 1;
				metrics.data_submit_extrinsics += 1;
				(vec![data.into_inner()], vec![])
			},
			RuntimeCall::Utility(pallet_utility::Call::batch { calls })
			| RuntimeCall::Utility(pallet_utility::Call::batch_all { calls })
			| RuntimeCall::Utility(pallet_utility::Call::force_batch { calls }) => {
				Self::process_calls(calls, &metrics)
			},
			_ => (vec![], vec![]),
		}
	}

	/// This function processes a list of calls and returns their data as Vec<Vec<u8>>
	fn process_calls(
		calls: Vec<RuntimeCall>,
		metrics: &submitted_data::RcMetrics,
	) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
		let (blob_data, bridge_data): (Vec<_>, Vec<_>) = calls
			.into_iter()
			.map(|call| Self::filter(call, Rc::clone(metrics)))
			.unzip();

		(
			blob_data.into_iter().flatten().collect(),
			bridge_data.into_iter().flatten().collect(),
		)
	}
}

/// Decodes and extracts the `data` of `DataAvailability::submit_data` extrinsics.
impl submitted_data::Extractor for Runtime {
	type Error = codec::Error;

	fn extract(
		opaque: &OpaqueExtrinsic,
		metrics: submitted_data::RcMetrics,
	) -> Result<(Vec<Vec<u8>>, Vec<Vec<u8>>), Self::Error> {
		let extrinsic = UncheckedExtrinsic::try_from(opaque)?;
		let data =
			<Runtime as submitted_data::Filter<RuntimeCall>>::filter(extrinsic.function, metrics);

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
	/// The Block type used by the runtime
	type Block = Block;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	type BlockLength = RuntimeBlockLength;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = constants::system::RuntimeBlockWeights;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header builder type.
	type HeaderExtensionBuilder = frame_system::header_builder::da::HeaderExtensionBuilder<Runtime>;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = Indices;
	type MaxConsumers = constants::system::MaxConsumers;
	/// The index type for storing how many extrinsics an account has signed.
	type Nonce = Index;
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
	type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
	type UncheckedExtrinsic = UncheckedExtrinsic;
	/// Version of the runtime.
	type Version = Version;
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
