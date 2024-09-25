use frame_support::pallet_prelude::*;
use sp_core::H160;
use sp_runtime::Perbill;
use sp_staking::EraIndex;

use crate::*;

/// Type representing an EVM address
pub type EvmAddress = H160;

/// Type representing a balance for external currency
pub type FusionCurrencyBalance = u128;

/// Type to represent points
pub type Points = u128;

/// Type of the currency id
pub type CurrencyId = u32;

/// Type of the pool id
pub type PoolId = u32;

/// Prefix used for storing accounts
pub const FUNDS_ACCOUNT_PREFIX: &str = "funds";
pub const CLAIMABLE_ACCOUNT_PREFIX: &str = "claimable";
pub const AVAIL_CURRENCY_ACCOUNT_PREFIX: &str = "avail";
pub const POOL_ACCOUNT_PREFIX: &str = "pool_acc_";
pub const POOL_REWARD_ACCOUNT_PREFIX: &str = "reward_acc_";

/// State of the pool
#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum FusionPoolState {
	/// Anyone can join, the pool is earning rewards
	Open,
	/// Nobody can join, the pool is earning rewards
	Blocked,
	/// The pool is paused, nobody can join, the pool is not earning rewards
	Paused,
	/// Pool is getting deleted, nobody can join, the pool is not earning rewards
	Destroying,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct EraReward<T: Config> {
	/// The total rewards
	pub rewards: BalanceOf<T>,
	/// The actual amount of reward claimed
	pub claimed_rewards: BalanceOf<T>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct PalletAccounts<T: Config> {
	pub funds_reward_account: T::AccountId,
	pub claimable_reward_account: T::AccountId,
	pub avail_currency_account: T::AccountId,
}
impl<T: Config> Default for PalletAccounts<T> {
	fn default() -> Self {
		PalletAccounts {
			funds_reward_account: T::PalletId::get()
				.into_sub_account_truncating(FUNDS_ACCOUNT_PREFIX.as_bytes()),
			claimable_reward_account: T::PalletId::get()
				.into_sub_account_truncating(CLAIMABLE_ACCOUNT_PREFIX.as_bytes()),
			avail_currency_account: T::PalletId::get()
				.into_sub_account_truncating(AVAIL_CURRENCY_ACCOUNT_PREFIX.as_bytes()),
		}
	}
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct FusionCurrency<T: Config> {
	/// Id of the fusion currency
	pub currency_id: CurrencyId,
	/// Name of the currency (e.g., "AVAIL", "ETH", "wBTC")
	pub name: BoundedVec<u8, T::MaxCurrencyName>,
	/// Number of decimals to represent 1 unit of the currency (e.g., 8 for wBTC, 18 for ETH)
	pub nb_decimals: u8,
	/// The amount staked in native form
	pub total_staked_native: FusionCurrencyBalance,
	/// The amount slashed in native form
	pub total_slashed_native: FusionCurrencyBalance,
	/// The amount unbonding in native form
	pub total_unbonding_native: FusionCurrencyBalance,
	/// Maximum allowable stake for this currency
	pub max_amount: FusionCurrencyBalance,
	/// Minimum amount to join a pool of this currency
	pub min_amount: FusionCurrencyBalance,
	/// State of the currency
	pub is_destroyed: bool,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct FusionPool<T: Config> {
	/// Id of the fusion pool
	pub pool_id: PoolId,
	/// Id of the currency this pool uses
	pub currency_id: CurrencyId,
	/// Percentage representing annual yield for this pool
	pub apy: Perbill,
	/// The account used during snapshot and for Phragmen
	pub pool_account: T::AccountId,
	/// Optional nominator of the pool, mandate can always manage
	pub nominator: Option<T::AccountId>,
	/// Optional, if not managed by avail, a pool should have a keyless reward account
	pub reward_account: Option<T::AccountId>,
	/// The evm addresses of members of the pool
	pub members: BoundedVec<(EvmAddress, Points), T::MaxMembersPerPool>,
	/// The target validators to be nominated by this pool
	pub targets: BoundedVec<T::AccountId, T::MaxTargets>,
	/// The amount staked in native form
	pub total_staked_native: FusionCurrencyBalance,
	/// The amount staked in points
	pub total_staked_points: Points,
	/// The amount slashed in this pool
	pub total_slashed_native: FusionCurrencyBalance,
	/// The total amount unbonding in this pool, conversion happens at unbonding
	pub total_unbonding_native: FusionCurrencyBalance,
	/// State of the pool
	pub state: FusionPoolState,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct FusionMembership<T: Config> {
	/// Evm address of the user
	pub evm_address: EvmAddress,
	/// Id of the pool the user selected, users can join multiple pools
	pub pool_id: PoolId,
	/// The stake of the user represented by points
	pub active_points: Points,
	/// Amounts and eras where the user unbonded, handles partial unbonds
	pub unbonding_chunks: BoundedVec<(EraIndex, FusionCurrencyBalance), T::MaxUnbonding>,
	/// If true, rewards will go to the AVAIL pool
	pub is_compounding: bool,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct FusionMemberCurrencyBalance {
	/// Evm address of the user
	pub evm_address: EvmAddress,
	/// Id of the idle currency this pool uses
	pub currency_id: CurrencyId,
	/// Amount of currency available, for AVAIL, it's the amount you can compound
	pub amount: FusionCurrencyBalance,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct FusionExposure<T: Config> {
	/// Id of the pool the user selected, users can join multiple pools
	pub pool_id: PoolId,
	/// Era of the exposure to compute rewards
	pub era: EraIndex,
	/// The total in native currency
	pub total_native: FusionCurrencyBalance,
	/// The total in avail
	pub total_avail: BalanceOf<T>,
	/// The total points in the pool
	pub total_points: Points,
	/// The users points in the pool
	pub user_points: BoundedVec<(EvmAddress, Points), T::MaxMembersPerPool>,
	/// The nominations of the pool at the time of setting the exposure
	pub targets: BoundedVec<T::AccountId, T::MaxTargets>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct FusionSlash {
	/// Id of the pool that got slashed
	pub pool_id: PoolId,
	/// Id of the currency that got slashed
	pub currency_id: CurrencyId,
	/// Era where the slash happen
	pub slash_era: EraIndex,
	/// Era where the slash need to get applied
	pub slash_apply: EraIndex,
	/// Slashed amoun
	pub slash_amount: FusionCurrencyBalance,
}

impl<T: Config> FusionCurrency<T> {
	/// Converts a given amount of this external currency to its equivalent in AVAIL.
	pub fn currency_to_avail(
		&self,
		amount: FusionCurrencyBalance,
		era: Option<EraIndex>,
	) -> Result<BalanceOf<T>, Error<T>> {
		let era = era.unwrap_or_else(T::EraProvider::current_era);
		let rate = FusionCurrencyRates::<T>::get(self.currency_id, era)
			.ok_or(Error::<T>::CurrencyRateNotFound)?;

		let rate = Pallet::<T>::u256(rate.try_into().map_err(|_| Error::<T>::ArithmeticError)?);
		let amount = Pallet::<T>::u256(amount);
		let divisor = Pallet::<T>::u256(10u128.pow(self.nb_decimals as u32));

		let avail_value = rate
			.saturating_mul(amount)
			.checked_div(divisor)
			.ok_or(Error::<T>::ArithmeticError)?;

		Ok(Pallet::<T>::balance(avail_value))
	}

	/// Converts a given amount of AVAIL to its equivalent in this external currency.
	pub fn avail_to_currency(
		&self,
		avail_amount: BalanceOf<T>,
		era: Option<EraIndex>,
	) -> Result<FusionCurrencyBalance, Error<T>> {
		let era = era.unwrap_or_else(T::EraProvider::current_era);

		let rate = FusionCurrencyRates::<T>::get(self.currency_id, era)
			.ok_or(Error::<T>::CurrencyRateNotFound)?;

		let rate = Pallet::<T>::u256(rate.try_into().map_err(|_| Error::<T>::ArithmeticError)?);
		let avail_amount = Pallet::<T>::u256(
			avail_amount
				.try_into()
				.map_err(|_| Error::<T>::ArithmeticError)?,
		);
		let multiplier = Pallet::<T>::u256(10u128.pow(self.nb_decimals as u32));

		let currency_value = avail_amount
			.saturating_mul(multiplier)
			.checked_div(rate)
			.ok_or(Error::<T>::ArithmeticError)?;

		Ok(Pallet::<T>::fusion_currency(currency_value))
	}
}

impl<T: Config> FusionPool<T> {
	/// Checks if the pool is paused
	/// If it uses a custom account, we only check the pool status
	/// If it uses the global account, we check for the pool status and the global status
	pub fn is_paused(&self) -> bool {
		self.state == FusionPoolState::Paused
			|| (self.reward_account.is_none() && Pallet::<T>::ensure_pallet_not_paused().is_err())
	}

	/// Checks if the pool is destroying
	pub fn is_destroying(&self) -> bool {
		self.state == FusionPoolState::Destroying
	}

	/// Converts a given amount of points to its equivalent in external currency.
	pub fn points_to_currency(
		&self,
		points: Points,
		currency: Option<&FusionCurrency<T>>,
	) -> Result<FusionCurrencyBalance, Error<T>> {
		if self.total_staked_native == 0 && self.total_staked_points == 0 {
			let currency_decimals = if let Some(c) = currency {
				c.nb_decimals
			} else {
				let stored_currency = FusionCurrencies::<T>::get(self.currency_id)
					.ok_or(Error::<T>::CurrencyNotFound)?;
				stored_currency.nb_decimals
			};

			let divisor = Pallet::<T>::u256(10u128.pow(18 - currency_decimals as u32));
			let points = Pallet::<T>::u256(points);

			let currency_value = points
				.checked_div(divisor)
				.ok_or(Error::<T>::ArithmeticPointsError)?;

			Ok(Pallet::<T>::fusion_currency(currency_value))
		} else {
			ensure!(
				self.total_staked_points > 0,
				Error::<T>::ArithmeticPointsError
			);

			let points = Pallet::<T>::u256(points);
			let total_staked_native = Pallet::<T>::u256(self.total_staked_native);
			let total_staked_points = Pallet::<T>::u256(self.total_staked_points);

			let currency_value = points
				.saturating_mul(total_staked_native)
				.checked_div(total_staked_points)
				.ok_or(Error::<T>::ArithmeticPointsError)?;

			Ok(Pallet::<T>::fusion_currency(currency_value))
		}
	}

	/// Converts a given amount of external currency to its equivalent in points.
	pub fn currency_to_points(
		&self,
		currency_amount: FusionCurrencyBalance,
		currency: Option<&FusionCurrency<T>>,
	) -> Result<Points, Error<T>> {
		if self.total_staked_native == 0 && self.total_staked_points == 0 {
			let currency_decimals = if let Some(c) = currency {
				c.nb_decimals
			} else {
				let currency = FusionCurrencies::<T>::get(self.currency_id)
					.ok_or(Error::<T>::CurrencyNotFound)?;
				currency.nb_decimals
			};
			let multiplier = Pallet::<T>::u256(10u128.pow(18 - currency_decimals as u32));
			let currency_amount = Pallet::<T>::u256(currency_amount);

			let points = currency_amount.saturating_mul(multiplier);

			Ok(Pallet::<T>::points(points))
		} else {
			ensure!(
				self.total_staked_native > 0,
				Error::<T>::ArithmeticPointsError
			);

			let currency_amount = Pallet::<T>::u256(currency_amount);
			let total_staked_native = Pallet::<T>::u256(self.total_staked_native);
			let total_staked_points = Pallet::<T>::u256(self.total_staked_points);

			let points = currency_amount
				.saturating_mul(total_staked_points)
				.checked_div(total_staked_native)
				.ok_or(Error::<T>::ArithmeticPointsError)?;

			Ok(Pallet::<T>::points(points))
		}
	}

	/// Converts a given amount of points to its equivalent in AVAIL.
	pub fn points_to_avail(
		&self,
		points: Points,
		currency: Option<&FusionCurrency<T>>,
		era: Option<EraIndex>,
	) -> Result<BalanceOf<T>, Error<T>> {
		let currency_value = self.points_to_currency(points, currency)?;

		let avail_value = if let Some(currency) = currency {
			currency.currency_to_avail(currency_value, era)?
		} else {
			let currency =
				FusionCurrencies::<T>::get(self.currency_id).ok_or(Error::<T>::CurrencyNotFound)?;
			currency.currency_to_avail(currency_value, era)?
		};

		Ok(avail_value)
	}

	/// Converts a given amount of AVAIL to its equivalent in points.
	pub fn avail_to_points(
		&self,
		avail_amount: BalanceOf<T>,
		currency: Option<&FusionCurrency<T>>,
		era: Option<EraIndex>,
	) -> Result<Points, Error<T>> {
		let currency_value = if let Some(currency) = currency {
			currency.avail_to_currency(avail_amount, era)?
		} else {
			let currency =
				FusionCurrencies::<T>::get(self.currency_id).ok_or(Error::<T>::CurrencyNotFound)?;
			currency.avail_to_currency(avail_amount, era)?
		};

		let points = self.currency_to_points(currency_value, currency)?;
		Ok(points)
	}
}
