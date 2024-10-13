#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
mod traits;
mod types;
mod weights;

use crate::types::*;
use alloc::collections::BTreeMap;
use frame_support::{
	dispatch::GetDispatchInfo,
	pallet_prelude::*,
	traits::{
		Currency, ExistenceRequirement, LockableCurrency, OnUnbalanced, UnfilteredDispatchable,
		WithdrawReasons,
	},
	PalletId,
};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use sp_core::U256;
use sp_runtime::{
	traits::{AccountIdConversion, Bounded, Zero},
	Perbill, Saturating,
};
use sp_staking::{currency_to_vote::CurrencyToVote, EraIndex, OnStakingUpdate};
use sp_std::{vec, vec::Vec};
pub use traits::{FusionExt, StakingFusionDataProvider};
pub use weights::WeightInfo;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

pub const AVAIL_CURRENCY_ID: u32 = 0;
pub const AVAIL_POOL_ID: u32 = 0;
pub const MILLISECONDS_PER_YEAR: u64 = 1000 * 3600 * 24 * 36525 / 100;

#[frame_support::pallet]
pub mod pallet {

	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// A sudo-able call.
		type RuntimeCall: Parameter
			+ UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ GetDispatchInfo;

		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		/// Someone who can call the admin extrinsics.
		type ApprovedOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Currency type for this pallet.
		type Currency: LockableCurrency<Self::AccountId, Moment = BlockNumberFor<Self>>;

		/// Convert a balance into a number used for election calculation.
		type CurrencyToVote: sp_staking::currency_to_vote::CurrencyToVote<BalanceOf<Self>>;

		/// The destination for rewards that were not claimed after 'HistoryDepth' eras.
		type RewardRemainder: OnUnbalanced<NegativeImbalanceOf<Self>>;

		/// Pallet id used to derive accounts used by the pallet
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Maximum allowed for the currency name
		#[pallet::constant]
		type MaxCurrencyName: Get<u32>;

		/// Maximum number of members in a pool
		#[pallet::constant]
		type MaxMembersPerPool: Get<u32>;

		/// Maximum number of selectable targets for a pool
		#[pallet::constant]
		type MaxTargets: Get<u32>;

		/// Maximum number of parallel partial unbonds
		#[pallet::constant]
		type MaxUnbonding: Get<u32>;

		/// Maximum number of parallel slashes
		#[pallet::constant]
		type MaxSlashes: Get<u32>;

		/// Maximum of number of pools behind one validator, mainly used for slashing
		#[pallet::constant]
		type MaxPoolsPerValidator: Get<u32>;

		/// Period for funds to be available after unbonding
		#[pallet::constant]
		type BondingDuration: Get<EraIndex>;

		/// Number of era for which to keep Fusion data
		#[pallet::constant]
		type HistoryDepth: Get<u32>;

		/// A provider that gives the current era.
		type StakingFusionDataProvider: StakingFusionDataProvider<Self::AccountId>;

		/// Number of eras that slashes are deferred by, after computation.
		///
		/// This should be less than the bonding duration. Set to 0 if slashes
		/// should be applied immediately, without opportunity for intervention.
		#[pallet::constant]
		type SlashDeferDuration: Get<EraIndex>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Stores the total value locked in avail and the maximum total value locked authorized
	#[pallet::storage]
	#[pallet::getter(fn tvl_data)]
	pub type TotalValueLockedData<T> = StorageValue<_, TVLData<T>, ValueQuery>;

	/// Stores all the fusion currencies
	#[pallet::storage]
	#[pallet::getter(fn fusion_currencies)]
	pub type FusionCurrencies<T: Config> =
		StorageMap<_, Twox64Concat, CurrencyId, FusionCurrency<T>, OptionQuery>;

	/// Stores all the fusion pools
	#[pallet::storage]
	#[pallet::getter(fn fusion_pools)]
	pub type FusionPools<T: Config> =
		StorageMap<_, Twox64Concat, PoolId, FusionPool<T>, OptionQuery>;

	/// Mapping from the pools funds account address to the pool id
	#[pallet::storage]
	#[pallet::getter(fn fusion_pool_account_to_id)]
	pub type FusionPoolsAccountToId<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, PoolId, OptionQuery>;

	/// Stores all the membership of users in pools
	#[pallet::storage]
	#[pallet::getter(fn fusion_memberships)]
	pub type FusionMemberships<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		EvmAddress,
		Twox64Concat,
		PoolId,
		FusionMembership<T>,
		OptionQuery,
	>;

	/// Stores all the users idle balances
	#[pallet::storage]
	#[pallet::getter(fn fusion_member_currency_balances)]
	pub type FusionMemberCurrencyBalances<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		EvmAddress,
		Twox64Concat,
		CurrencyId,
		FusionMemberCurrencyBalance,
		OptionQuery,
	>;

	/// Stores era rewards for each pool
	#[pallet::storage]
	#[pallet::getter(fn fusion_era_rewards)]
	pub type FusionEraRewards<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		EraIndex,
		Twox64Concat,
		PoolId,
		EraReward<T>,
		OptionQuery,
	>;

	/// Stores the conversion rates for currencies
	/// How much one unit of currency is equal in AVAIL
	#[pallet::storage]
	#[pallet::getter(fn fusion_currency_rates)]
	pub type FusionCurrencyRates<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		CurrencyId,
		Twox64Concat,
		EraIndex,
		BalanceOf<T>,
		OptionQuery,
	>;

	/// Stores the next currency changes to be applied next era
	#[pallet::storage]
	#[pallet::getter(fn fusion_currency_rate_changes)]
	pub type FusionCurrencyRateChanges<T: Config> =
		StorageMap<_, Twox64Concat, CurrencyId, BalanceOf<T>, OptionQuery>;

	/// Mapping from EVM Address to Substrate address
	#[pallet::storage]
	#[pallet::getter(fn fusion_evm_to_substrate_address)]
	pub type FusionEVMToSubstrateAddress<T: Config> =
		StorageMap<_, Blake2_128Concat, EvmAddress, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn era_durations)]
	pub type EraDurations<T: Config> = StorageMap<_, Twox64Concat, EraIndex, u64, OptionQuery>;

	/// Stores the fusion era exposure for HistoryDepth eras
	#[pallet::storage]
	#[pallet::getter(fn fusion_era_data)]
	pub type FusionExposures<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		EraIndex,
		Twox64Concat,
		PoolId,
		FusionExposure<T>,
		OptionQuery,
	>;

	/// Store the pools that backed the validator set as the key
	#[pallet::storage]
	#[pallet::getter(fn fusion_pools_from_validator)]
	pub type FusionPoolsFromValidator<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		EraIndex,
		Twox64Concat,
		T::AccountId,
		BoundedVec<PoolId, T::MaxPoolsPerValidator>,
		ValueQuery,
	>;

	/// Stores the fusion claimed rewards for HistoryDepth eras
	#[pallet::storage]
	#[pallet::getter(fn claimed_rewards)]
	pub type ClaimedRewards<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, EvmAddress>,
			NMapKey<Twox64Concat, PoolId>,
			NMapKey<Twox64Concat, EraIndex>,
		),
		BalanceOf<T>,
		OptionQuery,
	>;

	/// Stores EVM Address of the slash destination
	/// It can be controlled with technical committee
	#[pallet::storage]
	#[pallet::getter(fn slash_destination)]
	pub type SlashDestination<T> = StorageValue<_, EvmAddress, OptionQuery>;

	/// Storage for slashes that need to be applied.
	/// This storage holds an ordered queue of `FusionSlash` and is bounded by `MaxSlashes`.
	#[pallet::storage]
	#[pallet::getter(fn pending_slashes)]
	pub(super) type PendingSlashes<T: Config> =
		StorageValue<_, BoundedVec<FusionSlash, T::MaxSlashes>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event triggered when the funding account is filled with new funds
		FundsAccountFilled {
			sender: T::AccountId,
			amount: BalanceOf<T>,
		},
		/// Event triggered when a new currency is created
		CurrencyCreated {
			currency_id: CurrencyId,
			name: BoundedVec<u8, T::MaxCurrencyName>,
			nb_decimals: u8,
			max_amount: FusionCurrencyBalance,
			min_amount: FusionCurrencyBalance,
			initial_conversion_rate: BalanceOf<T>,
		},
		/// Event triggered when a currency's properties are updated
		CurrencySet {
			currency_id: CurrencyId,
			name: Option<BoundedVec<u8, T::MaxCurrencyName>>,
			max_amount: Option<FusionCurrencyBalance>,
			min_amount: Option<FusionCurrencyBalance>,
		},
		/// Event triggered when a currency is deleted
		CurrencyDeleted { currency_id: CurrencyId },
		/// Event triggered when a conversion rate is set for a currency
		CurrencyConversionRateSet {
			currency_id: CurrencyId,
			conversion_rate: BalanceOf<T>,
		},
		/// Event triggered when a currency is deposited into the system
		CurrencyDeposited {
			currency_id: CurrencyId,
			evm_address: EvmAddress,
			amount: FusionCurrencyBalance,
		},
		/// Event triggered when a user unbonds currency from a pool
		CurrencyUnbonded {
			pool_id: PoolId,
			currency_id: CurrencyId,
			evm_address: EvmAddress,
			unbonded_amount: FusionCurrencyBalance,
			points: Points,
			era: EraIndex,
		},
		/// Event triggered when a user withdraws unbonded currency
		CurrencyWithdrawn {
			pool_id: PoolId,
			currency_id: CurrencyId,
			evm_address: EvmAddress,
			amount: FusionCurrencyBalance,
		},
		/// Event triggered when the controller address for a user is changed
		ControllerAddressSet {
			evm_address: EvmAddress,
			new_controller_address: Option<T::AccountId>,
		},
		/// Event triggered when the Evm address and controller address are set for the Slash destination
		SlashDestinationSet {
			evm_address: Option<EvmAddress>,
			controller_address: Option<T::AccountId>,
		},
		/// Event triggered when the compounding value is changed for a pool member
		CompoundingSet {
			pool_id: PoolId,
			evm_address: EvmAddress,
			compound: bool,
		},
		/// Event triggered when a new Fusion pool is created
		PoolCreated {
			pool_id: PoolId,
			currency_id: CurrencyId,
			apy: Perbill,
			state: FusionPoolState,
			nominator: Option<T::AccountId>,
			funds_account: T::AccountId,
			claimable_account: T::AccountId,
		},
		/// Event triggered when a Fusion pool's properties are updated
		PoolSet {
			pool_id: PoolId,
			apy: Option<Perbill>,
			state: Option<FusionPoolState>,
			nominator: Option<Option<T::AccountId>>,
		},
		/// Event triggered when a user joins a pool
		PoolJoined {
			pool_id: PoolId,
			evm_address: EvmAddress,
			currency_id: CurrencyId,
			amount: FusionCurrencyBalance,
			points: Points,
		},
		/// Event triggered when a user bonds extra currency into a pool
		PoolBondExtra {
			pool_id: PoolId,
			evm_address: EvmAddress,
			currency_id: CurrencyId,
			amount: FusionCurrencyBalance,
			points: Points,
		},
		/// Event triggered when a user's pool membership is removed
		PoolMembershipRemoved {
			pool_id: PoolId,
			evm_address: EvmAddress,
		},
		/// Event triggered when a pool is deleted
		PoolDeleted { pool_id: PoolId },
		/// Event triggered when a pool state was changed to destroying
		PoolDestroying { pool_id: PoolId },
		/// Event triggered when a pool nominates a list of targets (validators)
		Nominated {
			pool_id: PoolId,
			targets: BoundedVec<T::AccountId, T::MaxTargets>,
		},
		/// Event triggered when unclaimed rewards are sent to the remainder
		RewardRemainderSent { amount: BalanceOf<T> },
		/// Event triggered when rewards are set for an era
		RewardSet {
			era: EraIndex,
			rewarded_pools: Vec<PoolId>,
			total_rewarded: BalanceOf<T>,
			paused_pools: Vec<PoolId>,
			paused_pools_missed_rewards: Vec<BalanceOf<T>>,
			retry: bool,
		},
		/// Event triggered when a user claims rewards for a pool and era
		RewardClaimed {
			pool_id: PoolId,
			evm_address: EvmAddress,
			era: EraIndex,
			reward: BalanceOf<T>,
		},
		/// Event triggered when exposures are set for an era
		ExposuresSet { era: EraIndex },
		/// Event triggered when AVAIL is withdrawn to the controller account
		AvailWithdrawnToController {
			evm_address: EvmAddress,
			controller: T::AccountId,
			amount: BalanceOf<T>,
		},
		/// A slash was created
		SlashCreated { slash: FusionSlash },
		/// A slash was applied
		SlashApplied { slash: FusionSlash },
		/// A slash was manually cancelled
		SlashCanceled { slash: FusionSlash },
		/// Event triggered when the maximum total value locked authorized is updated.
		MaxTVLUpdated(BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The id is already used.
		CurrencyAlreadyExists,
		/// No currency with the specified id
		CurrencyNotFound,
		/// The currency is already destroyed
		CurrencyDestroyed,
		/// Currency is not deletable cause a pool exist with the currency id
		PoolExistsForCurrency,
		/// The maximum amount of the currency is lower than what's already in the system
		InvalidMaxAmount,
		/// The minimum amount stakeable for this currency is greater than the maximum amount allowed in the system
		InvalidMinAmount,
		/// Pool id is already taken
		PoolAlreadyExists,
		/// Cannot create a pool in state destroying
		CannotSetPoolToDestroying,
		/// Pool was not found in storage
		PoolNotFound,
		/// The currency rate was not found
		CurrencyRateNotFound,
		/// Arithmetic error when doing conversions
		ArithmeticError,
		/// Arithmetic error when doing points conversions
		ArithmeticPointsError,
		/// The substrate address does not correspond to the EVM address in the mapping
		InvalidSubstrateAddress,
		/// The pool is not open
		PoolNotOpen,
		/// The bond amount is lower than the currency minimum allowed to bond
		BondAmoundTooLow,
		/// The bond amount would make the currency go past the maximum allowed in the system
		BondWouldExceedMaxForCurrency,
		/// The pool is full of members
		PoolMemberLimitReached,
		/// User has not balance record for the currency
		NoCurrencyBalanceForUser,
		/// The user has not enough balance of the specified currency
		NotEnoughCurrencyBalanceForUser,
		/// User is not a member of the pool
		UserNotMemberOfPool,
		/// User has no more points to unbond
		NoActivePointsToUnbond,
		/// The amount to unbond is invalid
		InvalidUnbondAmount,
		/// Unbonding this amount will make the remaining below minimum
		AmountWillGoBelowMinimum,
		/// User has too much unbonding chunks
		MaxUnbondingChunksExceeded,
		/// No funds are available to withdraw
		NoFundsToWithdraw,
		/// Caller is not authorized for this operation
		NotAuthorized,
		/// No rewards were found for the era
		NoRewardsForEra,
		/// The exposure is not founds
		ExposureNotFound,
		/// The user was not found in the exposure
		UserNotFoundInExposure,
		/// Rewards were already claimed for this user / era
		AlreadyClaimed,
		/// A user tried to unbond another user but it's only allowed if the pool is destroying
		PoolIsNotDestroying,
		/// Action is not allowed as the pool is destroying
		PoolIsDestroying,
		/// The pool is not ready to get cleaned from the storage
		PoolCannotBeCleaned,
		/// To handle compounding and easy bouding and unbonding, avail currency has no minimum
		NoMinAmountForAvailCurrency,
		/// There is no controller address to withdraw to
		NoControllerAddressForUser,
		/// If you're active points are below minimum, you cannot set compound to true
		CannotSetCompoudingWithLessThanMinimum,
		/// The state cannot be set to open if the pool is not nominating
		PoolIsNotNominating,
		/// The controller of the slash destination can only be set with the correct extrinsic
		CannotSetControllerForSlashDestination,
		/// There are too many simultaneous slashes
		TooManySlashes,
		/// Invalid slash index
		InvalidSlashIndex,
		/// Invalid slash pool id
		InvalidSlashPoolId,
		/// A user tried to claim but the account is empty, can try again later
		NotEnoughClaimableBalanceInPool,
		/// The maximum TVL was reached
		MaxTVLReached,
		/// No valid validators was provided in the targets
		NoValidValidators,
		/// Era duration was not recorded properly so we cannot retry
		EraDurationNotFound,
		/// TODO Temp, we'll see when bridge com is done
		CannotDepositAvailCurrency,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// TODO - Dummy extrinsic to add currency without bridge, to be removed
		#[pallet::call_index(99)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn deposit_currency_dummy(
			origin: OriginFor<T>,
			evm_address: EvmAddress,
			currency_id: CurrencyId,
			amount: FusionCurrencyBalance,
		) -> DispatchResult {
			ensure_signed(origin)?;
			Self::do_deposit_currency(evm_address, currency_id, amount)?;
			Ok(())
		}

		/// TODO - Dummy extrinsic to simulate an on_slash, to be removed
		#[pallet::call_index(98)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn dummy_slash(
			origin: OriginFor<T>,
			who: T::AccountId,
			bonded_amount: BalanceOf<T>,
			slashed_amount: BalanceOf<T>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			Self::do_dummy_slash(who, bonded_amount, slashed_amount)?;
			Ok(())
		}

		/// Creates a new currency
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn create_currency(
			origin: OriginFor<T>,
			currency_id: CurrencyId,
			name: BoundedVec<u8, T::MaxCurrencyName>,
			nb_decimals: u8,
			max_amount: FusionCurrencyBalance,
			min_amount: FusionCurrencyBalance,
			initial_conversion_rate: BalanceOf<T>,
		) -> DispatchResult {
			ensure_root(origin)?;

			ensure!(
				!FusionCurrencies::<T>::contains_key(currency_id),
				Error::<T>::CurrencyAlreadyExists
			);

			if currency_id == 0 {
				ensure!(min_amount == 0, Error::<T>::NoMinAmountForAvailCurrency);
			}

			let new_currency = FusionCurrency::<T> {
				currency_id,
				name: name.clone(),
				nb_decimals,
				total_staked_native: 0,
				total_slashed_native: 0,
				total_unbonding_native: 0,
				max_amount,
				min_amount,
				is_destroyed: false,
			};

			FusionCurrencies::<T>::insert(currency_id, new_currency);
			FusionCurrencyRates::<T>::insert(
				currency_id,
				T::StakingFusionDataProvider::current_era(),
				initial_conversion_rate,
			);
			FusionCurrencyRateChanges::<T>::insert(currency_id, initial_conversion_rate);

			Self::deposit_event(Event::CurrencyCreated {
				currency_id,
				name,
				nb_decimals,
				min_amount,
				max_amount,
				initial_conversion_rate,
			});

			Ok(())
		}

		/// Updates an existing currency
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn set_currency(
			origin: OriginFor<T>,
			currency_id: CurrencyId,
			name: Option<BoundedVec<u8, T::MaxCurrencyName>>,
			max_amount: Option<FusionCurrencyBalance>,
			min_amount: Option<FusionCurrencyBalance>,
		) -> DispatchResult {
			ensure_root(origin)?;

			FusionCurrencies::<T>::try_mutate(currency_id, |currency_opt| {
				let currency = currency_opt.as_mut().ok_or(Error::<T>::CurrencyNotFound)?;

				ensure!(!currency.is_destroyed, Error::<T>::CurrencyDestroyed);

				if let Some(name) = name.clone() {
					currency.name = name;
				}

				if let Some(max_amount) = max_amount {
					let total_staked_and_unbonding = currency
						.total_staked_native
						.saturating_add(currency.total_unbonding_native);
					ensure!(
						max_amount >= total_staked_and_unbonding,
						Error::<T>::InvalidMaxAmount
					);
					currency.max_amount = max_amount;
				}

				if let Some(min_amount) = min_amount {
					ensure!(
						min_amount <= currency.max_amount,
						Error::<T>::InvalidMinAmount
					);
					ensure!(
						currency_id != 0 || min_amount == 0,
						Error::<T>::NoMinAmountForAvailCurrency
					);
					currency.min_amount = min_amount;
				}

				Self::deposit_event(Event::CurrencySet {
					currency_id,
					name,
					min_amount,
					max_amount,
				});

				Ok(())
			})
		}

		/// Deletes a currency
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn destroy_currency(origin: OriginFor<T>, currency_id: CurrencyId) -> DispatchResult {
			ensure_root(origin)?;

			let pool_exists =
				FusionPools::<T>::iter().any(|(_, pool)| pool.currency_id == currency_id);
			ensure!(!pool_exists, Error::<T>::PoolExistsForCurrency);

			FusionCurrencies::<T>::try_mutate(currency_id, |currency_opt| {
				let currency = currency_opt.as_mut().ok_or(Error::<T>::CurrencyNotFound)?;

				ensure!(!currency.is_destroyed, Error::<T>::CurrencyDestroyed);

				currency.is_destroyed = true;

				let depth = T::HistoryDepth::get();
				let _ = FusionCurrencyRates::<T>::clear_prefix(currency_id, depth, None);
				FusionCurrencyRateChanges::<T>::remove(currency_id);

				Self::deposit_event(Event::CurrencyDeleted { currency_id });

				Ok(())
			})
		}

		/// Sets the conversion rate for a currency for the next era
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn set_currency_conversion_rate(
			origin: OriginFor<T>,
			currency_id: CurrencyId,
			conversion_rate: BalanceOf<T>,
		) -> DispatchResult {
			ensure_root(origin)?;

			FusionCurrencies::<T>::try_get(currency_id)
				.map_err(|_| Error::<T>::CurrencyNotFound)
				.and_then(|currency| {
					ensure!(!currency.is_destroyed, Error::<T>::CurrencyDestroyed);
					Ok(())
				})?;

			FusionCurrencyRateChanges::<T>::insert(currency_id, conversion_rate);

			Self::deposit_event(Event::CurrencyConversionRateSet {
				currency_id,
				conversion_rate,
			});

			Ok(())
		}

		/// Creates a new fusion pool
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn create_pool(
			origin: OriginFor<T>,
			pool_id: PoolId,
			currency_id: CurrencyId,
			apy: Perbill,
			nominator: Option<T::AccountId>,
		) -> DispatchResult {
			ensure_root(origin)?;

			ensure!(
				!FusionPools::<T>::contains_key(pool_id),
				Error::<T>::PoolAlreadyExists
			);

			let currency =
				FusionCurrencies::<T>::get(currency_id).ok_or(Error::<T>::CurrencyNotFound)?;
			ensure!(!currency.is_destroyed, Error::<T>::CurrencyDestroyed);

			let funds_account = Self::get_pool_funds_account(pool_id);
			let claimable_account = Self::get_pool_claimable_account(pool_id);

			let new_pool = FusionPool::<T> {
				pool_id,
				currency_id,
				apy,
				funds_account: funds_account.clone(),
				claimable_account: claimable_account.clone(),
				state: FusionPoolState::Paused,
				nominator: nominator.clone(),
				members: BoundedVec::default(),
				targets: BoundedVec::default(),
				total_staked_native: 0,
				total_staked_points: 0,
				total_slashed_native: 0,
				total_unbonding_native: 0,
			};

			FusionPoolsAccountToId::<T>::insert(&new_pool.funds_account, pool_id);
			FusionPools::<T>::insert(pool_id, new_pool);

			Self::deposit_event(Event::PoolCreated {
				pool_id,
				currency_id,
				apy,
				state: FusionPoolState::Paused,
				nominator,
				funds_account,
				claimable_account,
			});

			Ok(())
		}

		/// Updates an existing fusion pool
		/// If some rewards were missed due to low balance in account,
		/// retry_rewards_for_era can be used to generate those missing rewards.
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn set_pool(
			origin: OriginFor<T>,
			pool_id: PoolId,
			apy: Option<Perbill>,
			state: Option<FusionPoolState>,
			nominator: Option<Option<T::AccountId>>,
			retry_rewards_for_eras: Option<BoundedVec<EraIndex, T::HistoryDepth>>,
		) -> DispatchResult {
			ensure_root(origin)?;

			let mut pool_is_active = false;

			FusionPools::<T>::try_mutate(pool_id, |maybe_pool| -> DispatchResult {
				let pool = maybe_pool.as_mut().ok_or(Error::<T>::PoolNotFound)?;

				ensure!(
					pool.state != FusionPoolState::Destroying,
					Error::<T>::PoolIsDestroying
				);

				pool.apy = apy.unwrap_or(pool.apy);

				if let Some(state) = state {
					ensure!(
						state != FusionPoolState::Destroying,
						Error::<T>::CannotSetPoolToDestroying
					);
					if state == FusionPoolState::Open || state == FusionPoolState::Blocked {
						ensure!(pool.targets.len() > 0, Error::<T>::PoolIsNotNominating);
						let currency = FusionCurrencies::<T>::get(pool.currency_id)
							.ok_or(Error::<T>::CurrencyNotFound)?;
						ensure!(!currency.is_destroyed, Error::<T>::CurrencyDestroyed);
					}

					pool.state = state;
				}

				if let Some(nominator) = nominator.clone() {
					pool.nominator = nominator;
				}

				if pool.is_active() {
					pool_is_active = true;
				}

				Ok(())
			})?;

			if let Some(retry_rewards_for_eras) = retry_rewards_for_eras {
				if pool_is_active {
					retry_rewards_for_eras
						.into_iter()
						.try_for_each(|era| -> DispatchResult {
							let era_duration = EraDurations::<T>::get(era)
								.ok_or(Error::<T>::EraDurationNotFound)?;
							Self::compute_era_rewards(era, era_duration, Some(pool_id));
							Ok(())
						})?;
				}
			}

			// Emit an event for pool update
			Self::deposit_event(Event::PoolSet {
				pool_id,
				apy,
				state,
				nominator,
			});

			Ok(())
		}

		/// Deletes a pool
		/// Called once to set the pool to destroying
		/// Called a second time when everything is cleaned to actually destroy it
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn destroy_pool(origin: OriginFor<T>, pool_id: PoolId) -> DispatchResult {
			ensure_root(origin)?;

			FusionPools::<T>::try_mutate(pool_id, |maybe_pool| -> DispatchResult {
				let pool = maybe_pool.as_mut().ok_or(Error::<T>::PoolNotFound)?;

				if pool.state != FusionPoolState::Destroying {
					pool.state = FusionPoolState::Destroying;
					Self::deposit_event(Event::PoolDestroying { pool_id });
				} else {
					Self::check_and_cleanup_pool(pool)?;
				}

				Ok(())
			})
		}

		/// Fills the funds account with the specified amount of funds.
		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn fill_pool_account(
			origin: OriginFor<T>,
			pool_id: PoolId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let pool = FusionPools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;

			ensure!(!pool.is_destroying(), Error::<T>::PoolIsDestroying);

			let funds_account = Self::get_pool_funds_account(pool_id);

			T::Currency::transfer(
				&who,
				&funds_account,
				amount,
				ExistenceRequirement::KeepAlive,
			)?;

			Self::deposit_event(Event::FundsAccountFilled {
				sender: who,
				amount,
			});

			Ok(())
		}

		/// Nominates a list of validators for a given pool.
		#[pallet::call_index(8)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn nominate(
			origin: OriginFor<T>,
			pool_id: PoolId,
			targets: BoundedVec<T::AccountId, T::MaxTargets>,
		) -> DispatchResult {
			// Check if the origin is root, if not, check if it's a signed origin.
			let is_root = ensure_root(origin.clone()).is_ok();
			let who = if is_root {
				None
			} else {
				Some(ensure_signed(origin)?)
			};

			// Fetch the pool and ensure it exists
			FusionPools::<T>::try_mutate(pool_id, |pool_opt| -> DispatchResult {
				let pool = pool_opt.as_mut().ok_or(Error::<T>::PoolNotFound)?;

				// If the caller is not root, ensure it's the nominator of the pool
				if let Some(caller) = who {
					ensure!(
						Some(&caller) == pool.nominator.as_ref(),
						Error::<T>::NotAuthorized
					);
				}

				// We cannot change nominations if the pool is destroying
				ensure!(
					pool.state != FusionPoolState::Destroying,
					Error::<T>::PoolIsDestroying
				);

				// Check that targets contains at least one validator
				ensure!(
					targets
						.iter()
						.all(|target| T::StakingFusionDataProvider::is_valid_validator(&target)),
					Error::<T>::NoValidValidators
				);

				// Update the targets of the pool
				pool.targets = targets.clone();

				// Emit event for nomination
				Self::deposit_event(Event::Nominated { pool_id, targets });

				Ok(())
			})
		}

		/// Admin extrinsic to kick a user from the system.
		/// The user is immediately removed from all pools and given back all their assets and rewards.
		#[pallet::call_index(9)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn kick_user(origin: OriginFor<T>, evm_address: EvmAddress) -> DispatchResult {
			ensure_root(origin)?;

			// Retrieve all memberships of the user
			let memberships: Vec<(PoolId, FusionMembership<T>)> =
				FusionMemberships::<T>::iter_prefix(evm_address).collect();

			// Iterate through each membership and process them
			for (pool_id, membership) in memberships {
				// Fetch pool and currency details
				let mut pool = FusionPools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
				let mut currency = FusionCurrencies::<T>::get(pool.currency_id)
					.ok_or(Error::<T>::CurrencyNotFound)?;

				// Convert user's active points to currency and add to idle balance
				let currency_value =
					pool.points_to_currency(membership.active_points, Some(&currency))?;

				// Instantly return all unbonding chunks
				let total_unbonding = membership
					.unbonding_chunks
					.iter()
					.fold(0 as FusionCurrencyBalance, |acc, (_, amount)| {
						acc.saturating_add(*amount)
					});

				Self::add_to_currency_balance(
					evm_address,
					currency.currency_id,
					currency_value.saturating_add(total_unbonding),
				)?;

				// Update pool and currency totals
				pool.members.retain(|(address, _)| *address != evm_address);
				pool.total_staked_points = pool
					.total_staked_points
					.saturating_sub(membership.active_points);
				pool.total_staked_native = pool.total_staked_native.saturating_sub(currency_value);
				pool.total_unbonding_native =
					pool.total_unbonding_native.saturating_sub(total_unbonding);

				currency.total_staked_native =
					currency.total_staked_native.saturating_sub(currency_value);
				currency.total_unbonding_native = currency
					.total_unbonding_native
					.saturating_sub(total_unbonding);

				// Update TVL
				Self::sub_from_tvl(&currency, currency_value)?;

				// Save the updated pool and currency data back to storage
				FusionPools::<T>::insert(pool_id, &pool);
				FusionCurrencies::<T>::insert(currency.currency_id, &currency);

				// Remove user's membership from storage
				FusionMemberships::<T>::remove(evm_address, pool_id);
			}

			Ok(())
		}

		/// Change the Substrate controller address.
		#[pallet::call_index(10)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn set_controller_address(
			origin: OriginFor<T>,
			evm_address: EvmAddress,
			new_controller_address: Option<T::AccountId>,
		) -> DispatchResult {
			let is_root = ensure_root(origin.clone()).is_ok();
			if !is_root {
				ensure_signed(origin)?;
				// TODO - commented for tests only
				// let who = ensure_signed(origin)?;
				// Self::ensure_valid_fusion_origin(who, evm_address)?;
			}

			let slash_destination = SlashDestination::<T>::get();
			if let Some(slash_address) = slash_destination {
				ensure!(
					evm_address != slash_address,
					Error::<T>::CannotSetControllerForSlashDestination
				);
			}

			Self::do_set_controller_address(evm_address, new_controller_address)?;
			Ok(())
		}

		/// Change the Slash destination evm address.
		#[pallet::call_index(11)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn set_slash_destination(
			origin: OriginFor<T>,
			evm_address: Option<EvmAddress>,
			controller_address: Option<T::AccountId>,
		) -> DispatchResult {
			ensure_root(origin)?;

			if let Some(evm_address) = evm_address {
				SlashDestination::<T>::put(evm_address);
				Self::do_set_controller_address(evm_address, controller_address.clone())?;
			} else {
				if let Some(current_address) = SlashDestination::<T>::get() {
					Self::do_set_controller_address(current_address, None)?;
				}
				SlashDestination::<T>::kill();
			}

			Self::deposit_event(Event::SlashDestinationSet {
				evm_address,
				controller_address,
			});

			Ok(())
		}

		/// Cancel a slash given its index.
		#[pallet::call_index(12)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn cancel_slash(
			origin: OriginFor<T>,
			slash_index: u32,
			pool_id: PoolId,
		) -> DispatchResult {
			ensure_root(origin)?;

			let slash_index = slash_index as usize;
			PendingSlashes::<T>::try_mutate(|slashes| -> DispatchResult {
				ensure!(slash_index < slashes.len(), Error::<T>::InvalidSlashIndex);

				let slash = slashes
					.get(slash_index)
					.ok_or(Error::<T>::InvalidSlashIndex)?;

				ensure!(slash.pool_id == pool_id, Error::<T>::InvalidSlashPoolId);

				let removed_slash = slashes.remove(slash_index);

				FusionPools::<T>::mutate(removed_slash.pool_id, |maybe_pool| -> DispatchResult {
					let pool = maybe_pool.as_mut().ok_or(Error::<T>::PoolNotFound)?;
					FusionCurrencies::<T>::mutate(
						removed_slash.currency_id,
						|maybe_currency| -> DispatchResult {
							let currency = maybe_currency
								.as_mut()
								.ok_or(Error::<T>::CurrencyNotFound)?;

							pool.total_staked_native = pool
								.total_staked_native
								.saturating_add(removed_slash.slash_amount);
							pool.total_slashed_native = pool
								.total_slashed_native
								.saturating_sub(removed_slash.slash_amount);

							currency.total_staked_native = currency
								.total_staked_native
								.saturating_add(removed_slash.slash_amount);
							currency.total_slashed_native = currency
								.total_slashed_native
								.saturating_sub(removed_slash.slash_amount);

							// Update TVL
							Self::add_to_tvl(&currency, removed_slash.slash_amount)?;

							Ok(())
						},
					)?;
					Ok(())
				})?;

				Self::deposit_event(Event::<T>::SlashCanceled {
					slash: removed_slash,
				});

				Ok(())
			})
		}

		/// Direcly apply a slash given its index.
		#[pallet::call_index(13)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn apply_slash(
			origin: OriginFor<T>,
			slash_index: u32,
			pool_id: PoolId,
		) -> DispatchResult {
			ensure_root(origin)?;

			let slash_index = slash_index as usize;
			PendingSlashes::<T>::try_mutate(|slashes| -> DispatchResult {
				ensure!(slash_index < slashes.len(), Error::<T>::InvalidSlashIndex);

				let slash = slashes
					.get(slash_index)
					.ok_or(Error::<T>::InvalidSlashIndex)?;

				ensure!(slash.pool_id == pool_id, Error::<T>::InvalidSlashPoolId);

				let removed_slash = slashes.remove(slash_index);

				Self::do_apply_slash(removed_slash)?;

				Ok(())
			})
		}

		/// Updates the maximum TVL authorized in the Fusion pallet.
		#[pallet::call_index(14)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn update_max_tvl(origin: OriginFor<T>, new_max_tvl: BalanceOf<T>) -> DispatchResult {
			ensure_root(origin)?;
			let mut tvl_data = <TotalValueLockedData<T>>::get();
			ensure!(
				new_max_tvl >= tvl_data.total_value_locked,
				Error::<T>::MaxTVLReached
			);
			tvl_data.max_total_value_locked = new_max_tvl;
			<TotalValueLockedData<T>>::put(tvl_data);
			Self::deposit_event(Event::MaxTVLUpdated(new_max_tvl));
			Ok(())
		}

		/// Set the destination of the reward for the user.
		#[pallet::call_index(15)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn set_compounding(
			origin: OriginFor<T>,
			evm_address: EvmAddress,
			pool_id: PoolId,
			compound: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_valid_fusion_origin(who, evm_address)?;
			Self::do_set_compounding(evm_address, pool_id, compound)?;
			Ok(())
		}

		/// Stake currency into a pool, either by joining or bonding extra.
		#[pallet::call_index(16)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn stake(
			origin: OriginFor<T>,
			evm_address: EvmAddress,
			pool_id: PoolId,
			amount: FusionCurrencyBalance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_valid_fusion_origin(who, evm_address)?;
			Self::do_stake(evm_address, pool_id, amount, false)?;
			Ok(())
		}

		/// Claims the rewards for an evm address for a specific era and pool.
		#[pallet::call_index(17)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn claim_rewards(
			origin: OriginFor<T>,
			era: EraIndex,
			pool_id: PoolId,
			evm_address: EvmAddress,
		) -> DispatchResult {
			ensure_signed(origin)?;
			Self::do_claim_rewards(era, pool_id, evm_address)
		}

		/// Unbonds an amount of currency from a pool
		#[pallet::call_index(18)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn unbond_currency(
			origin: OriginFor<T>,
			evm_address: EvmAddress,
			pool_id: PoolId,
			unbond_amount: FusionCurrencyBalance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_valid_fusion_origin(who, evm_address)?;
			Self::do_unbond(evm_address, pool_id, unbond_amount, false)?;
			Ok(())
		}

		/// Withdraws unbonded currency after the bonding duration has passed.
		#[pallet::call_index(19)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn withdraw_unbonded_currency(
			origin: OriginFor<T>,
			evm_address: EvmAddress,
			pool_id: PoolId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_valid_fusion_origin(who, evm_address)?;
			Self::do_withdraw_unbonded_currency(evm_address, pool_id, false)?;
			Ok(())
		}

		/// Unbonds an amount of currency from a pool on behalf on another user
		/// Only works if the pool is destroying
		#[pallet::call_index(20)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn unbond_currency_other(
			origin: OriginFor<T>,
			evm_address: EvmAddress,
			pool_id: PoolId,
			unbond_amount: FusionCurrencyBalance,
		) -> DispatchResult {
			ensure_signed(origin)?;
			Self::do_unbond(evm_address, pool_id, unbond_amount, true)?;
			Ok(())
		}

		/// Withdraws unbonded currency after the bonding duration has passed.
		#[pallet::call_index(21)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn withdraw_unbonded_currency_other(
			origin: OriginFor<T>,
			evm_address: EvmAddress,
			pool_id: PoolId,
		) -> DispatchResult {
			ensure_signed(origin)?;
			Self::do_withdraw_unbonded_currency(evm_address, pool_id, true)?;
			Ok(())
		}

		/// Withdraws unbonded Avail Fusion Currency to the controller account.
		/// Only works for avail pool
		#[pallet::call_index(22)]
		#[pallet::weight(T::WeightInfo::create_currency())]
		pub fn withdraw_avail_to_controller(
			origin: OriginFor<T>,
			evm_address: EvmAddress,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_valid_fusion_origin(who, evm_address)?;
			Self::do_withdraw_avail_to_controller(evm_address)?;
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Avail fusion currency account holds the native avail corresponding to the equivalent in Avail Fusion currency
	pub fn avail_account() -> T::AccountId {
		T::PalletId::get().into_sub_account_truncating(FusionAccountType::AvailCurrency)
	}

	/// Helper to convert u128 to U256
	pub fn u256(value: u128) -> U256 {
		U256::from(value)
	}

	/// Helper to convert U256 to balance
	pub fn balance(value: U256) -> BalanceOf<T> {
		let value: u128 = value.try_into().unwrap_or(u128::max_value());
		value.try_into().unwrap_or(BalanceOf::<T>::max_value())
	}

	/// Helper to convert U256 to fusion currency
	pub fn fusion_currency(value: U256) -> FusionCurrencyBalance {
		value.try_into().unwrap_or(u128::max_value())
	}

	/// Helper to convert U256 to points
	pub fn points(value: U256) -> Points {
		value.try_into().unwrap_or(u128::max_value())
	}

	/// Ensures the origin is signed and that the provided EVM address maps to the correct Substrate account.
	pub fn ensure_valid_fusion_origin(
		who: T::AccountId,
		evm_address: EvmAddress,
	) -> DispatchResult {
		let mapped_address = FusionEVMToSubstrateAddress::<T>::get(evm_address)
			.ok_or(Error::<T>::InvalidSubstrateAddress)?;
		ensure!(who == mapped_address, Error::<T>::InvalidSubstrateAddress);
		Ok(())
	}

	/// Adds the fusion currency amount to the user's idle balance for a specific currency.
	fn add_to_currency_balance(
		evm_address: EvmAddress,
		currency_id: CurrencyId,
		amount: FusionCurrencyBalance,
	) -> DispatchResult {
		let _ = FusionCurrencies::<T>::get(currency_id).ok_or(Error::<T>::CurrencyNotFound)?;
		FusionMemberCurrencyBalances::<T>::mutate(evm_address, currency_id, |balance_opt| {
			if let Some(balance) = balance_opt {
				balance.amount = balance.amount.saturating_add(amount);
			} else {
				*balance_opt = Some(FusionMemberCurrencyBalance {
					evm_address,
					currency_id,
					amount,
				});
			}
		});

		Ok(())
	}

	/// Withdraw the fusion currency amount from the user's idle balance for a specific currency.
	fn withdraw_from_currency_balance(
		evm_address: EvmAddress,
		currency_id: CurrencyId,
		amount: FusionCurrencyBalance,
	) -> DispatchResult {
		FusionMemberCurrencyBalances::<T>::try_mutate(
			evm_address,
			currency_id,
			|balance_opt| -> DispatchResult {
				let balance = balance_opt
					.as_mut()
					.ok_or(Error::<T>::NoCurrencyBalanceForUser)?;
				ensure!(
					balance.amount >= amount,
					Error::<T>::NotEnoughCurrencyBalanceForUser
				);

				balance.amount = balance.amount.saturating_sub(amount);

				if balance.amount == 0 {
					*balance_opt = None;
				}

				Ok(())
			},
		)
	}

	/// Function to check if a pool should be removed and perform cleanup if necessary
	fn check_and_cleanup_pool(pool: &FusionPool<T>) -> DispatchResult {
		let pool_id = pool.pool_id;
		let has_no_members = pool.members.is_empty();
		let has_no_points = pool.total_staked_points == 0;
		let has_no_staked_native = pool.total_staked_native == 0;
		let has_no_unbonding_native = pool.total_unbonding_native == 0;

		ensure!(
			has_no_members && has_no_points && has_no_staked_native && has_no_unbonding_native,
			Error::<T>::PoolCannotBeCleaned
		);

		for key in FusionEraRewards::<T>::iter_keys() {
			if &key.1 == &pool_id {
				FusionEraRewards::<T>::remove(key.0, key.1);
			}
		}
		for key in FusionExposures::<T>::iter_keys() {
			if &key.1 == &pool_id {
				FusionExposures::<T>::remove(key.0, key.1);
			}
		}
		FusionPoolsAccountToId::<T>::remove(&pool.funds_account);
		FusionPools::<T>::remove(pool_id);
		Self::deposit_event(Event::PoolDeleted { pool_id });

		Ok(())
	}

	/// Setup the fusion currency rates for the new era
	fn setup_currency_rates(era: EraIndex) -> DispatchResult {
		for (currency_id, currency) in FusionCurrencies::<T>::iter() {
			// Skip if the currency is destroyed
			if currency.is_destroyed {
				continue;
			}

			// Try to get the new rate from the rate changes storage
			let new_rate = FusionCurrencyRateChanges::<T>::get(currency_id).or_else(|| {
				// Fallback to the current era's rate
				FusionCurrencyRates::<T>::get(currency_id, era)
			});

			// If neither a new rate nor a current rate is found, trigger an error
			let rate = new_rate.ok_or(Error::<T>::CurrencyRateNotFound)?;

			// Insert the rate for the next era
			FusionCurrencyRates::<T>::insert(currency_id, era + 1, rate);
		}
		Ok(())
	}

	/// Clean history depth storages and send old pending rewards to 'RewardRemainder'
	fn clean_history_depth_storages(era: EraIndex) -> DispatchResult {
		let history_depth = T::HistoryDepth::get();

		let Some(era_to_clear) = era.checked_sub(history_depth) else {
			return Ok(());
		};

		// Clean fusion exposures and FusionPoolsFromValidator - u32::MAX is safe knowing the maximum number of pools is low
		let _ = FusionExposures::<T>::clear_prefix(era_to_clear, u32::MAX, None);
		let _ = FusionPoolsFromValidator::<T>::clear_prefix(era_to_clear, u32::MAX, None);

		// Clean old era durations
		EraDurations::<T>::remove(era);

		// Clean currency rates
		FusionCurrencyRates::<T>::iter_keys().for_each(|(currency_id, era)| {
			if era == era_to_clear {
				FusionCurrencyRates::<T>::remove(currency_id, era);
			}
		});

		// Clean claimed rewards
		ClaimedRewards::<T>::iter_keys().for_each(|(evm_address, pool_id, era)| {
			if era == era_to_clear {
				ClaimedRewards::<T>::remove((evm_address, pool_id, era));
			}
		});

		// Clean fusion era rewards and compute remaining rewards
		let existential_deposit = T::Currency::minimum_balance();
		let mut total_remaining = BalanceOf::<T>::zero();
		FusionEraRewards::<T>::drain_prefix(era).for_each(|(pool_id, rewards)| {
			let remaining_rewards = rewards.rewards.saturating_sub(rewards.claimed_rewards);
			if remaining_rewards > BalanceOf::<T>::zero() {
				let claimable_account = Self::get_pool_claimable_account(pool_id);
				let claimable_balance = T::Currency::free_balance(&claimable_account)
					.saturating_sub(existential_deposit);
				if claimable_balance > remaining_rewards {
					let imbalance = T::Currency::withdraw(
						&claimable_account,
						remaining_rewards,
						WithdrawReasons::all(),
						ExistenceRequirement::KeepAlive,
					);
					if let Ok(imbalance) = imbalance {
						T::RewardRemainder::on_unbalanced(imbalance);
						total_remaining = total_remaining.saturating_add(remaining_rewards);
					}
				}
			}
		});
		if total_remaining > BalanceOf::<T>::zero() {
			Self::deposit_event(Event::RewardRemainderSent {
				amount: total_remaining,
			});
		}

		Ok(())
	}

	/// Compute rewards for each pool and set them in storage
	/// Reward computatation is done at the end of era N for era N
	fn compute_era_rewards(era: EraIndex, era_duration: u64, maybe_pool_id: Option<PoolId>) -> () {
		let mut total_rewarded = BalanceOf::<T>::zero();
		let mut rewarded_pools: Vec<PoolId> = vec![];
		let mut paused_pools: Vec<PoolId> = vec![];
		let mut paused_pools_missed_rewards: Vec<BalanceOf<T>> = vec![];
		let existential_deposit = T::Currency::minimum_balance();

		let exposures_iter =
			FusionExposures::<T>::iter_prefix(era).filter(|(pool_id, _)| match maybe_pool_id {
				Some(ref id) => pool_id == id,
				None => true,
			});

		for (pool_id, fusion_exposure) in exposures_iter {
			let Some(mut pool) = FusionPools::<T>::get(pool_id) else {
				log::error!(
					"🚨 Pool with PoolId {:?} not found for Era {:?}. Reward could not have been set. 🚨",
					pool_id,
					era
				);
				continue;
			};

			if fusion_exposure.total_avail.is_zero()
				|| fusion_exposure.user_points.is_empty()
				|| fusion_exposure.targets.is_empty()
				|| Perbill::is_zero(&fusion_exposure.apy)
				|| FusionEraRewards::<T>::get(era, pool_id).is_some()
			{
				// No need to pause the pool cause it's just not supposed to get rewards.
				continue;
			}

			// Era reward computation for a pool
			let apy = fusion_exposure.apy;
			let fraction_of_year = Perbill::from_rational(era_duration, MILLISECONDS_PER_YEAR);
			let total_avail = fusion_exposure.total_avail;
			let pool_era_reward = fraction_of_year * apy * total_avail;

			// Check that the pool actually backed a validator and that this validator has earned points during the era
			let mut should_earn_rewards = false;
			if let Some(native_exposure_data) = fusion_exposure.native_exposure_data {
				let validators_backed: Vec<T::AccountId> = native_exposure_data
					.into_iter()
					.map(|(account_id, _balance)| account_id)
					.collect();
				should_earn_rewards =
					T::StakingFusionDataProvider::has_earned_era_points(era, &validators_backed);
			}

			if !should_earn_rewards {
				Self::pause_pool(
					pool_id,
					&mut pool,
					&"Fusion pool selected validators have not earned rewards.",
					&mut paused_pools,
					&mut paused_pools_missed_rewards,
					pool_era_reward,
				);
				continue;
			}

			// Get the pool funds balances
			let pool_funds_balance = T::Currency::free_balance(&pool.funds_account);

			// In case of insufficient balance in pool account, we pause the pool
			// This means the reward won't get paid for this era.
			if pool_era_reward > pool_funds_balance.saturating_sub(existential_deposit) {
				Self::pause_pool(
					pool_id,
					&mut pool,
					&"Insufficient funds in fusion pool account.",
					&mut paused_pools,
					&mut paused_pools_missed_rewards,
					pool_era_reward,
				);
				continue;
			}

			if let Err(e) = T::Currency::transfer(
				&pool.funds_account,
				&pool.claimable_account,
				pool_era_reward,
				ExistenceRequirement::KeepAlive,
			) {
				Self::pause_pool(
					pool_id,
					&mut pool,
					&"An error has occured during transfer",
					&mut paused_pools,
					&mut paused_pools_missed_rewards,
					pool_era_reward,
				);
				log::error!("Error detail: {e:?}");
				continue;
			}

			total_rewarded = total_rewarded.saturating_add(pool_era_reward);

			FusionEraRewards::<T>::insert(
				era,
				pool_id,
				EraReward {
					rewards: pool_era_reward,
					claimed_rewards: BalanceOf::<T>::default(),
				},
			);

			rewarded_pools.push(pool_id);
		}

		// Recrod Era duration in case we need it later, eg. for a retry
		EraDurations::<T>::insert(era, era_duration);

		if !rewarded_pools.is_empty() || !paused_pools.is_empty() {
			Self::deposit_event(Event::RewardSet {
				era,
				rewarded_pools,
				paused_pools,
				total_rewarded,
				paused_pools_missed_rewards,
				retry: maybe_pool_id.is_some(),
			});
		}
	}

	fn pause_pool(
		pool_id: PoolId,
		pool: &mut FusionPool<T>,
		reason: &str,
		paused_pools: &mut Vec<PoolId>,
		paused_pools_missed_rewards: &mut Vec<BalanceOf<T>>,
		pool_era_reward: BalanceOf<T>,
	) {
		log::error!("Pausing pool {:?}: {}.", pool_id, reason);
		pool.state = FusionPoolState::Paused;
		FusionPools::<T>::insert(pool_id, pool);
		paused_pools.push(pool_id);
		paused_pools_missed_rewards.push(pool_era_reward);
	}

	fn add_slash(slash: FusionSlash) -> DispatchResult {
		PendingSlashes::<T>::try_mutate(|slashes| {
			ensure!(
				slashes.len() < T::MaxSlashes::get() as usize,
				Error::<T>::TooManySlashes
			);

			let position = slashes
				.binary_search_by_key(&slash.slash_apply, |s| s.slash_apply)
				.unwrap_or_else(|pos| pos);

			slashes
				.try_insert(position, slash)
				.map_err(|_| Error::<T>::TooManySlashes)?;

			Ok(())
		})
	}

	fn apply_expired_pending_slashes(era: EraIndex) -> DispatchResult {
		PendingSlashes::<T>::try_mutate(|slashes| {
			while let Some(first_slash) = slashes.first() {
				if first_slash.slash_apply > era {
					break;
				}
				let slash = slashes.remove(0);
				Self::do_apply_slash(slash)?;
			}

			Ok(())
		})
	}

	fn do_apply_slash(slash: FusionSlash) -> DispatchResult {
		// If we don't have a slash destination setup, the funds will get burned
		if let Some(slash_dest_evm) = SlashDestination::<T>::get() {
			Self::add_to_currency_balance(slash_dest_evm, slash.currency_id, slash.slash_amount)?;
		}

		Self::deposit_event(Event::SlashApplied { slash });

		Ok(())
	}

	/// Increase total value locked in avail
	fn add_to_tvl(currency: &FusionCurrency<T>, value: FusionCurrencyBalance) -> DispatchResult {
		let mut tvl_data = TotalValueLockedData::<T>::get();
		let avail_value = currency.currency_to_avail(value, None, None)?;
		tvl_data.add(avail_value)?;
		TotalValueLockedData::<T>::put(tvl_data);
		Ok(())
	}

	/// Decrease total value locked in avail
	fn sub_from_tvl(currency: &FusionCurrency<T>, value: FusionCurrencyBalance) -> DispatchResult {
		let mut tvl_data = TotalValueLockedData::<T>::get();
		let avail_value = currency.currency_to_avail(value, None, None)?;
		tvl_data.sub(avail_value);
		TotalValueLockedData::<T>::put(tvl_data);
		Ok(())
	}

	// #[cfg(test)] // TODO Remove
	/// Simulate a slashing event for tests
	fn do_dummy_slash(
		who: T::AccountId,
		bonded_amount: BalanceOf<T>,
		slashed_amount: BalanceOf<T>,
	) -> DispatchResult {
		Self::on_slash(
			&who,
			bonded_amount - slashed_amount,
			&Default::default(),
			slashed_amount,
		);
		Ok(())
	}

	/// Deposits a specified amount of currency for a given EVM address and currency ID.
	fn do_deposit_currency(
		evm_address: EvmAddress,
		currency_id: CurrencyId,
		amount: FusionCurrencyBalance,
	) -> DispatchResult {
		// TODO - in case we're adding avail, the Avail currency should come from somewhere and put in avail holdings of the pallet, for now we just prevent it
		ensure!(
			currency_id != AVAIL_CURRENCY_ID,
			Error::<T>::CannotDepositAvailCurrency
		);

		Self::add_to_currency_balance(evm_address, currency_id, amount)?;

		Self::deposit_event(Event::CurrencyDeposited {
			evm_address,
			currency_id,
			amount,
		});

		Ok(())
	}

	/// Sets or unsets a controller address for a specific EVM address.
	fn do_set_controller_address(
		evm_address: EvmAddress,
		new_controller_address: Option<T::AccountId>,
	) -> DispatchResult {
		if let Some(ref new_controller_address) = new_controller_address {
			FusionEVMToSubstrateAddress::<T>::insert(evm_address, new_controller_address);
		} else {
			FusionEVMToSubstrateAddress::<T>::remove(evm_address);
		}

		Self::deposit_event(Event::ControllerAddressSet {
			evm_address,
			new_controller_address,
		});

		Ok(())
	}

	/// Configures whether the specified EVM address should compound rewards in a given pool.
	fn do_set_compounding(
		evm_address: EvmAddress,
		pool_id: PoolId,
		compound: bool,
	) -> DispatchResult {
		FusionMemberships::<T>::try_mutate(evm_address, pool_id, |membership_opt| {
			let membership = membership_opt
				.as_mut()
				.ok_or(Error::<T>::UserNotMemberOfPool)?;
			let pool = FusionPools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let currency =
				FusionCurrencies::<T>::get(pool.currency_id).ok_or(Error::<T>::CurrencyNotFound)?;
			let active_currency_balance =
				pool.points_to_currency(membership.active_points, Some(&currency))?;
			if compound {
				ensure!(
					active_currency_balance >= currency.min_amount,
					Error::<T>::CannotSetCompoudingWithLessThanMinimum
				)
			}

			membership.is_compounding = compound;

			Self::deposit_event(Event::<T>::CompoundingSet {
				evm_address,
				pool_id,
				compound,
			});
			Ok::<(), Error<T>>(())
		})?;

		Ok(())
	}

	/// Stakes a specified amount of currency into a pool for a given EVM address.
	/// If `skip_checks` is true, some checks (like pool state or pallet balance) may be skipped.
	fn do_stake(
		evm_address: EvmAddress,
		pool_id: PoolId,
		amount: FusionCurrencyBalance,
		skip_checks: bool,
	) -> DispatchResult {
		// Fetch pool and currency
		let mut pool = FusionPools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
		let mut currency =
			FusionCurrencies::<T>::get(pool.currency_id).ok_or(Error::<T>::CurrencyNotFound)?;

		let maybe_membership = FusionMemberships::<T>::get(evm_address, pool_id);

		if !skip_checks {
			// Ensure they are open or allowed to compound
			ensure!(
				pool.state == FusionPoolState::Open
					|| (pool.state == FusionPoolState::Blocked && maybe_membership.is_some()),
				Error::<T>::PoolNotOpen
			);
			ensure!(!currency.is_destroyed, Error::<T>::CurrencyDestroyed);

			// Ensure the total staked does not exceed the max allowable amount
			ensure!(
				currency.total_staked_native.saturating_add(amount) <= currency.max_amount,
				Error::<T>::BondWouldExceedMaxForCurrency
			);
		}

		// Fetch and ensure user has enough currency balance
		Self::withdraw_from_currency_balance(evm_address, currency.currency_id, amount)?;

		// Convert currency amount to points
		let points = pool.currency_to_points(amount, Some(&currency))?;

		// Common logic to update currency and pool data
		currency.total_staked_native = currency.total_staked_native.saturating_add(amount);
		pool.total_staked_native = pool.total_staked_native.saturating_add(amount);
		pool.total_staked_points = pool.total_staked_points.saturating_add(points);

		// Save updated currency data
		FusionCurrencies::<T>::insert(pool.currency_id, &currency);

		// Update TVL
		Self::add_to_tvl(&currency, amount)?;

		// Check if the user is already a member of the pool
		if let Some(mut membership) = maybe_membership {
			// Update user's active points and save membership
			membership.active_points = membership.active_points.saturating_add(points);

			// Update the pool's member points
			if let Some(member) = pool
				.members
				.iter_mut()
				.find(|(address, _)| *address == evm_address)
			{
				member.1 = member.1.saturating_add(points);
			}

			FusionMemberships::<T>::insert(evm_address, pool_id, membership);
			FusionPools::<T>::insert(pool_id, &pool);

			// Emit event for extra bond
			Self::deposit_event(Event::PoolBondExtra {
				evm_address,
				pool_id,
				currency_id: pool.currency_id,
				amount,
				points,
			});
		} else {
			// Ensure the amount meets the minimum staking requirement
			ensure!(amount >= currency.min_amount, Error::<T>::BondAmoundTooLow);

			// Update pool members
			pool.members
				.try_push((evm_address, points))
				.map_err(|_| Error::<T>::PoolMemberLimitReached)?;

			// Insert new membership for user
			let new_membership = FusionMembership::<T> {
				evm_address,
				pool_id,
				active_points: points,
				unbonding_chunks: BoundedVec::default(),
				is_compounding: true,
			};
			FusionMemberships::<T>::insert(evm_address, pool_id, new_membership);

			// Emit event for pool join
			Self::deposit_event(Event::PoolJoined {
				evm_address,
				pool_id,
				currency_id: pool.currency_id,
				amount,
				points,
			});
		}

		// Save updated pool data
		FusionPools::<T>::insert(pool_id, &pool);

		Ok(())
	}

	/// Claims rewards for a specified era and pool for a given EVM address.
	fn do_claim_rewards(era: EraIndex, pool_id: PoolId, evm_address: EvmAddress) -> DispatchResult {
		// Get the fusion exposure for the pool and era
		let exposure =
			FusionExposures::<T>::get(era, pool_id).ok_or(Error::<T>::ExposureNotFound)?;

		FusionEraRewards::<T>::try_mutate(era, pool_id, |maybe_reward| -> DispatchResult {
			// Ensure rewards are available for the given era and pool
			let era_rewards = maybe_reward.as_mut().ok_or(Error::<T>::NoRewardsForEra)?;

			// Find the user's points in this era for the pool
			let user_points = exposure
				.user_points
				.iter()
				.find(|(user, _)| *user == evm_address)
				.map(|(_, points)| points)
				.ok_or(Error::<T>::UserNotFoundInExposure)?;

			// Ensure the user has not already claimed the reward for this era and pool
			ensure!(
				!ClaimedRewards::<T>::contains_key((evm_address, pool_id, era)),
				Error::<T>::AlreadyClaimed
			);

			// Calculate the reward ratio
			let user_share = Self::u256(*user_points);
			let total_points = Self::u256(exposure.total_points);
			let rewards_u128: u128 = era_rewards
				.rewards
				.try_into()
				.map_err(|_| Error::<T>::ArithmeticError)?;
			let rewards = Self::u256(rewards_u128);

			let user_reward = rewards
				.saturating_mul(user_share)
				.checked_div(total_points)
				.ok_or(Error::<T>::ArithmeticError)?;

			let user_reward_balance = Self::balance(user_reward);

			// Update the claimed rewards field by adding the user's reward
			era_rewards.claimed_rewards = era_rewards
				.claimed_rewards
				.saturating_add(user_reward_balance);

			// Mark rewards as claimed
			ClaimedRewards::<T>::insert((evm_address, pool_id, era), user_reward_balance);

			// Fetch avail currency
			let avail_currency = FusionCurrencies::<T>::get(AVAIL_CURRENCY_ID)
				.ok_or(Error::<T>::CurrencyNotFound)?;

			// Convert the avail reward to avail currency
			let avail_in_currency =
				avail_currency.avail_to_currency(user_reward_balance, Some(era))?;

			// Transfer claimable avail to avail fusion currency account for holding
			let pool_claimable_account = Self::get_pool_funds_account(pool_id);

			// Check that it has enough funds
			let pool_claimable_balance = T::Currency::free_balance(&pool_claimable_account);
			let existential_deposit = T::Currency::minimum_balance();
			ensure!(
				user_reward_balance <= pool_claimable_balance.saturating_sub(existential_deposit),
				Error::<T>::NotEnoughClaimableBalanceInPool
			);

			// Send the funds to the avail holdings account
			T::Currency::transfer(
				&pool_claimable_account,
				&Self::avail_account(),
				user_reward_balance,
				ExistenceRequirement::AllowDeath,
			)?;

			// We can now add the equivalent in fusion currency
			Self::add_to_currency_balance(evm_address, AVAIL_CURRENCY_ID, avail_in_currency)?;

			Self::deposit_event(Event::RewardClaimed {
				evm_address,
				pool_id,
				era,
				reward: user_reward_balance,
			});

			// Handle compounding or adding to the user's idle balance
			FusionMemberships::<T>::try_mutate(
				evm_address,
				pool_id,
				|membership_opt| -> DispatchResult {
					let Some(membership) = membership_opt.as_mut() else {
						return Ok(());
					};
					// Fetch avail pool
					let avail_pool =
						FusionPools::<T>::get(AVAIL_POOL_ID).ok_or(Error::<T>::PoolNotFound)?;

					if membership.is_compounding
						&& (avail_pool.state == FusionPoolState::Open
							|| (avail_pool.state == FusionPoolState::Blocked
								&& FusionMemberships::<T>::get(evm_address, AVAIL_POOL_ID)
									.is_some())) && !avail_currency.is_destroyed
						&& avail_currency
							.total_staked_native
							.saturating_add(avail_in_currency)
							<= avail_currency.max_amount
					{
						// At this point this should never fail except in case of arithmetic errors which is ok
						Self::do_stake(evm_address, AVAIL_POOL_ID, avail_in_currency, true)?;
					}
					Ok(())
				},
			)?;
			Ok(())
		})?;

		Ok(())
	}

	/// Unbonds a specified amount of currency from a pool for a given EVM address.
	/// If `other` is true, the unbonding is performed on behalf of another user.
	fn do_unbond(
		evm_address: EvmAddress,
		pool_id: PoolId,
		unbond_amount: FusionCurrencyBalance,
		other: bool,
	) -> DispatchResult {
		// Retrieve the user's membership in the pool
		let mut membership = FusionMemberships::<T>::get(evm_address, pool_id)
			.ok_or(Error::<T>::UserNotMemberOfPool)?;

		// Ensure the user has active points to unbond
		ensure!(
			membership.active_points > 0,
			Error::<T>::NoActivePointsToUnbond
		);

		// Fetch pool and currency details
		let mut pool = FusionPools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
		let mut currency =
			FusionCurrencies::<T>::get(pool.currency_id).ok_or(Error::<T>::CurrencyNotFound)?;

		// Allow permissionless only if pool is destroying
		ensure!(
			!other || pool.state == FusionPoolState::Destroying,
			Error::<T>::PoolIsNotDestroying
		);

		// Convert points to currency to determine how much to unbond
		let currency_value = pool.points_to_currency(membership.active_points, Some(&currency))?;

		// Ensure user has enough points to unbond the requested amount
		let requested_points = pool.currency_to_points(unbond_amount, Some(&currency))?;
		ensure!(
			membership.active_points >= requested_points,
			Error::<T>::InvalidUnbondAmount
		);

		let is_full_unbond = requested_points == membership.active_points;

		// Ensure it's full unbond or valid partial unbond
		ensure!(
			is_full_unbond || currency_value.saturating_sub(unbond_amount) >= currency.min_amount,
			Error::<T>::AmountWillGoBelowMinimum
		);

		// Get current era
		let current_era = T::StakingFusionDataProvider::current_era();

		// Update membership with unbonding chunk
		membership.active_points = membership.active_points.saturating_sub(requested_points);
		membership
			.unbonding_chunks
			.try_push((current_era, unbond_amount))
			.map_err(|_| Error::<T>::MaxUnbondingChunksExceeded)?;

		// If it is a full unbond, we set compounding to false as user probably want to leave the pool and he'll receive some rewards after
		if is_full_unbond {
			membership.is_compounding = false;
		}

		// Update the pool's member points
		if let Some(member_index) = pool
			.members
			.iter()
			.position(|(address, _)| *address == evm_address)
		{
			// Subtract the user's points from the member entry
			if let Some((_, member_points)) = pool.members.get_mut(member_index) {
				*member_points = member_points.saturating_sub(membership.active_points);

				// If the user's points are now zero, remove the user from the members array
				if *member_points == 0 {
					pool.members.remove(member_index);
				}
			}
		}

		// Update pool totals
		pool.total_staked_points = pool.total_staked_points.saturating_sub(requested_points);
		pool.total_staked_native = pool.total_staked_native.saturating_sub(unbond_amount);
		pool.total_unbonding_native = pool.total_unbonding_native.saturating_add(unbond_amount);

		// Update currency totals
		currency.total_staked_native = currency.total_staked_native.saturating_sub(unbond_amount);
		currency.total_unbonding_native = currency
			.total_unbonding_native
			.saturating_add(unbond_amount);

		// Update TVL
		Self::sub_from_tvl(&currency, unbond_amount)?;

		// Save the updated state back to storage
		FusionMemberships::<T>::insert(evm_address, pool_id, membership);
		FusionPools::<T>::insert(pool_id, &pool);
		FusionCurrencies::<T>::insert(currency.currency_id, &currency);

		// Emit event
		Self::deposit_event(Event::CurrencyUnbonded {
			evm_address,
			pool_id,
			currency_id: currency.currency_id,
			unbonded_amount: unbond_amount,
			points: requested_points,
			era: current_era,
		});

		Ok(())
	}

	/// Withdraws unbonded currency for a given EVM address after the bonding duration has passed.
	/// If `other` is true, the withdrawal is performed on behalf of another user.
	fn do_withdraw_unbonded_currency(
		evm_address: EvmAddress,
		pool_id: PoolId,
		other: bool,
	) -> DispatchResult {
		// Ensure user is a member of the pool
		let mut membership = FusionMemberships::<T>::get(evm_address, pool_id)
			.ok_or(Error::<T>::UserNotMemberOfPool)?;

		// Fetch pool and currency data
		let mut pool = FusionPools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
		let mut currency =
			FusionCurrencies::<T>::get(pool.currency_id).ok_or(Error::<T>::CurrencyNotFound)?;

		// Allow permissionless only if pool is destroying
		ensure!(
			!other || pool.state == FusionPoolState::Destroying,
			Error::<T>::PoolIsNotDestroying
		);

		// Get current era
		let current_era = T::StakingFusionDataProvider::current_era();

		// Check if there are any unbonded chunks that are now withdrawable
		let mut total_withdrawable: FusionCurrencyBalance = 0;
		let mut remaining_unbonding_chunks = BoundedVec::default();

		for &(era, amount) in membership.unbonding_chunks.iter() {
			if era + T::BondingDuration::get() <= current_era {
				// This chunk is now withdrawable
				total_withdrawable = total_withdrawable.saturating_add(amount);
			} else {
				// Keep this chunk as it's not withdrawable yet
				remaining_unbonding_chunks
					.try_push((era, amount))
					.map_err(|_| Error::<T>::MaxUnbondingChunksExceeded)?; // This error will never get triggered
			}
		}

		// Ensure there is something to withdraw
		ensure!(total_withdrawable > 0, Error::<T>::NoFundsToWithdraw);

		// Update the user's membership by removing processed unbonding chunks
		membership.unbonding_chunks = remaining_unbonding_chunks;

		// Update pool and currency data
		pool.total_unbonding_native = pool
			.total_unbonding_native
			.saturating_sub(total_withdrawable);
		currency.total_unbonding_native = currency
			.total_unbonding_native
			.saturating_sub(total_withdrawable);
		FusionPools::<T>::insert(pool_id, &pool);
		FusionCurrencies::<T>::insert(pool.currency_id, &currency);

		// Update the user's currency balance
		Self::add_to_currency_balance(evm_address, pool.currency_id, total_withdrawable)?;

		// Check if the user should be removed from the pool membership
		if membership.unbonding_chunks.is_empty() && membership.active_points == 0 {
			// Remove the user's membership from the pool
			FusionMemberships::<T>::remove(evm_address, pool_id);

			// Emit event for removing pool membership
			Self::deposit_event(Event::PoolMembershipRemoved {
				evm_address,
				pool_id,
			});
		} else {
			// If there are remaining unbonding chunks or active points, update the membership
			FusionMemberships::<T>::insert(evm_address, pool_id, membership);
		}

		// Emit event for successful withdrawal
		Self::deposit_event(Event::CurrencyWithdrawn {
			evm_address,
			pool_id,
			currency_id: pool.currency_id,
			amount: total_withdrawable,
		});

		Ok(())
	}

	/// Withdraws AVAIL currency to the controller account for a given EVM address.
	fn do_withdraw_avail_to_controller(evm_address: EvmAddress) -> DispatchResult {
		// Get the currency
		let currency =
			FusionCurrencies::<T>::get(AVAIL_CURRENCY_ID).ok_or(Error::<T>::CurrencyNotFound)?;

		// Get the controller account
		let controller_account = FusionEVMToSubstrateAddress::<T>::get(evm_address)
			.ok_or(Error::<T>::NoControllerAddressForUser)?;

		// Retrieve the user's balance of AVAIL currency
		let balance = FusionMemberCurrencyBalances::<T>::get(evm_address, AVAIL_CURRENCY_ID)
			.ok_or(Error::<T>::NoCurrencyBalanceForUser)?
			.amount;

		// Ensure the balance is greater than 0
		ensure!(balance > 0, Error::<T>::NoFundsToWithdraw);

		// Fusion currency in avail
		let balance_avail = currency.currency_to_avail(balance, None, None)?;

		T::Currency::transfer(
			&Self::avail_account(),
			&controller_account,
			balance_avail,
			ExistenceRequirement::KeepAlive,
		)?;

		// Remove the user's AVAIL currency balance after minting
		FusionMemberCurrencyBalances::<T>::remove(evm_address, AVAIL_CURRENCY_ID);

		// Emit an event indicating successful withdrawal
		Self::deposit_event(Event::AvailWithdrawnToController {
			evm_address,
			controller: controller_account,
			amount: balance_avail,
		});

		Ok(())
	}

	/// Return the pool funds account
	fn get_pool_funds_account(id: PoolId) -> T::AccountId {
		T::PalletId::get().into_sub_account_truncating((FusionAccountType::PoolFundsAccount, id))
	}

	/// Return the pool claimable account
	fn get_pool_claimable_account(id: PoolId) -> T::AccountId {
		T::PalletId::get()
			.into_sub_account_truncating((FusionAccountType::PoolClaimableAccount, id))
	}
}

impl<T: Config> FusionExt<T::AccountId, BalanceOf<T>> for Pallet<T> {
	fn set_fusion_exposures() -> () {
		let era = T::StakingFusionDataProvider::current_era();
		let mut at_least_one = false;
		// Iterate over all pools
		for (pool_id, pool) in FusionPools::<T>::iter() {
			// Check if the pool is open, has members, and has targets
			if pool.is_active()
				&& !pool.members.is_empty()
				&& !pool.targets.is_empty()
				&& pool.total_staked_points > 0
			{
				// Get total amount in avail
				let total_avail_result =
					pool.points_to_avail(pool.total_staked_points, None, Some(era));

				let Ok(total_avail) = total_avail_result else {
					log::error!(
						"Error while setting exposure for era {:?} and pool {:?} - Could not compute avail amount from pool points. - Details: {:?}",
						era,
						pool_id,
						total_avail_result
					);
					continue;
				};

				// We set the exposure for era + 1
				// The data must be available for the snapshot and next elections
				let fusion_exposure = FusionExposure::<T> {
					pool_id,
					era,
					total_avail,
					total_points: pool.total_staked_points,
					user_points: pool.members.clone(),
					targets: pool.targets.clone(),
					apy: pool.apy,
					native_exposure_data: None,
				};
				FusionExposures::<T>::insert(era, pool_id, fusion_exposure);
				at_least_one = true;
			}
		}
		if at_least_one {
			Self::deposit_event(Event::<T>::ExposuresSet { era });
		}
	}

	fn handle_end_era(era_duration: u64) -> () {
		let era = T::StakingFusionDataProvider::current_era();

		fn log_if_error<T>(
			result: Result<T, DispatchError>,
			function_name: &str,
			era: EraIndex,
		) -> Result<T, DispatchError> {
			if let Err(ref err) = result {
				log::error!("Error in {} for era {:?}: {:?}", function_name, era, err);
			}
			result
		}
		Self::compute_era_rewards(era, era_duration, None);

		let _ = log_if_error(Self::setup_currency_rates(era), "setup_currency_rates", era);
		let _ = log_if_error(
			Self::apply_expired_pending_slashes(era),
			"apply_expired_pending_slashes",
			era,
		);
		let _ = log_if_error(
			Self::clean_history_depth_storages(era),
			"clean_history_depth_storages",
			era,
		);
	}

	fn get_fusion_voters() -> Vec<(T::AccountId, u64, Vec<T::AccountId>)> {
		let era = T::StakingFusionDataProvider::current_era();
		let exposure_iterator = FusionExposures::<T>::iter_prefix(era);
		let mut fusion_voters =
			Vec::<(T::AccountId, u64, Vec<T::AccountId>)>::with_capacity(exposure_iterator.count());

		let total_issuance = T::Currency::total_issuance();

		for (pool_id, exposure) in FusionExposures::<T>::iter_prefix(era) {
			if exposure.targets.is_empty() || exposure.total_avail.is_zero() {
				continue;
			}
			let account = Self::get_pool_funds_account(pool_id);
			let targets = exposure.targets;
			let stake = exposure.total_avail;
			let fusion_pool_weight = T::CurrencyToVote::to_vote(stake, total_issuance);
			fusion_voters.push((account, fusion_pool_weight, targets.to_vec()));
		}

		fusion_voters
	}

	fn get_active_pool_count() -> usize {
		FusionExposures::<T>::iter_prefix(T::StakingFusionDataProvider::current_era()).count()
	}

	fn get_pool_id_from_funds_account(account: &T::AccountId) -> Option<PoolId> {
		FusionPoolsAccountToId::<T>::get(account)
	}

	fn update_pool_exposure(
		maybe_pool_account: &T::AccountId,
		validator: &T::AccountId,
		value: BalanceOf<T>,
	) -> () {
		let Some(pool_id) = Self::get_pool_id_from_funds_account(maybe_pool_account) else {
			return;
		};

		let era = T::StakingFusionDataProvider::current_era();
		let _ =
			FusionExposures::<T>::try_mutate(era, pool_id, |maybe_exposure| -> DispatchResult {
				// Ensure rewards are available for the given era and pool
				let Some(ref mut exposure) = maybe_exposure else {
					return Ok(());
				};

				let mut native_exposure_data = match exposure.native_exposure_data.clone() {
					Some(x) => x,
					None => BoundedVec::default(),
				};

				if let Err(_) = native_exposure_data.try_push((validator.clone(), value)) {
					log::error!(
					"Could not update fusion exposure for pool {:?} - native_exposure_data limit reached",
					pool_id
				);
				};

				let _ = FusionPoolsFromValidator::<T>::try_mutate(
					era,
					validator,
					|pool_ids| -> DispatchResult {
						if let Err(_) = pool_ids.try_push(pool_id) {
							log::error!(
							"Could not fusion pools from validator for pool {:?} and validator {:?}",
							pool_id, validator
						);
						}
						Ok(())
					},
				);

				exposure.native_exposure_data = Some(native_exposure_data);

				Ok(())
			});
	}
}

impl<T: Config> OnStakingUpdate<T::AccountId, BalanceOf<T>> for Pallet<T> {
	fn on_slash(
		who: &T::AccountId,
		slashed_active: BalanceOf<T>,
		slashed_unlocking: &BTreeMap<EraIndex, BalanceOf<T>>,
		slashed_total: BalanceOf<T>,
	) {
		log::info!("ON SLASH TRIGGERED IN FUSION PALLET");
		log::info!("WHO {who:?} -  slashed_active {slashed_active:?} -  slashed_unlocking {slashed_unlocking:?} -  slashed_total {slashed_total:?}");
		// TODO Change the logic here
		let current_era = T::StakingFusionDataProvider::current_era();
		for (pool_id, exposure) in FusionExposures::<T>::iter_prefix(current_era) {
			if exposure.targets.contains(who) {
				// TODO Change this to check for targets really nominated by the pool
				FusionPools::<T>::mutate(pool_id, |maybe_pool| {
					let pool = match maybe_pool {
						Some(ref mut pool) => pool,
						None => return,
					};

					FusionCurrencies::<T>::mutate(pool.currency_id, |maybe_currency| {
						let currency = match maybe_currency {
							Some(ref mut currency) => currency,
							None => return,
						};
						let slash_portion = Perbill::from_rational(
							slashed_total,
							slashed_total.saturating_add(slashed_active),
						);
						let slash_fusion_amount = slash_portion * pool.total_staked_native;

						currency.total_staked_native = currency
							.total_staked_native
							.saturating_sub(slash_fusion_amount);
						currency.total_slashed_native = currency
							.total_slashed_native
							.saturating_add(slash_fusion_amount);

						// Update TVL
						if let Err(e) = Self::sub_from_tvl(&currency, slash_fusion_amount) {
							log::error!(
								"Error while substracting slash from TVL: {:?} - Amount: {:?}",
								e,
								slash_fusion_amount
							);
						}

						pool.total_staked_native =
							pool.total_staked_native.saturating_sub(slash_fusion_amount);
						pool.total_slashed_native = pool
							.total_slashed_native
							.saturating_add(slash_fusion_amount);

						let new_slash = FusionSlash {
							pool_id,
							currency_id: pool.currency_id,
							slash_era: current_era,
							slash_apply: current_era + T::SlashDeferDuration::get(),
							slash_amount: slash_fusion_amount,
						};

						if let Err(e) = Self::add_slash(new_slash.clone()) {
							log::error!("Error while adding slash: {:?}", e);
						} else {
							Self::deposit_event(Event::SlashCreated { slash: new_slash });
						}
					});
				});
			}
		}
	}
}
