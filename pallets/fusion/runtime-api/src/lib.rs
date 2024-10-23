#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use pallet_fusion::types::*;
use sp_staking::EraIndex;

sp_api::decl_runtime_apis! {
	pub trait FusionApi<AccountId, Balance>
		where
			AccountId: Codec,
			Balance: Codec,
	{
		/// Return the amount of pending rewards for the user alongside the concerned pools and eras for each pool.
		fn api_pending_rewards(who: EvmAddress, pool_id: PoolId, era: EraIndex) -> Balance;

		/// Convert an AVAIL amount to a specified external currency.
		fn api_avail_to_currency(currency_id: CurrencyId, avail_amount: Balance, era: Option<EraIndex>) -> FusionCurrencyBalance;

		/// Convert a specified external currency amount to AVAIL.
		fn api_currency_to_avail(currency_id: CurrencyId, currency_amount: FusionCurrencyBalance, era: Option<EraIndex>) -> Balance;

		/// Convert points to the equivalent amount in a specified external currency for a given pool.
		fn api_points_to_currency(pool_id: PoolId, points: Points) -> FusionCurrencyBalance;

		/// Convert a specified external currency amount to points for a given pool.
		fn api_currency_to_points(pool_id: PoolId, currency_amount: FusionCurrencyBalance) -> Points;

		/// Convert points to the equivalent AVAIL amount for a given pool.
		fn api_points_to_avail(pool_id: PoolId, points: Points, era: Option<EraIndex>) -> Balance;

		/// Convert an AVAIL amount to points for a given pool.
		fn api_avail_to_points(pool_id: PoolId, avail_amount: Balance, era: Option<EraIndex>) -> Points;
	}
}
