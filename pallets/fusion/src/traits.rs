use crate::*;
use sp_staking::EraIndex;

// A trait that provides the current era.
pub trait EraProvider {
	/// Returns the current era.
	fn current_era() -> EraIndex;
}

// A trait for Fusion operations with a generic `AccountId`.
pub trait FusionExt<AccountId> {
	/// Deposits a specified amount of currency for a given EVM address and currency ID.
	fn do_deposit_currency(
		evm_address: EvmAddress,
		currency_id: CurrencyId,
		amount: FusionCurrencyBalance,
	) -> DispatchResult;

	/// Sets or unsets a controller address for a specific EVM address.
	fn do_set_controller_address(
		evm_address: EvmAddress,
		new_controller_address: Option<AccountId>,
	) -> DispatchResult;

	/// Configures whether the specified EVM address should compound rewards in a given pool.
	fn do_set_compounding(
		evm_address: EvmAddress,
		pool_id: PoolId,
		compound: bool,
	) -> DispatchResult;

	/// Stakes a specified amount of currency into a pool for a given EVM address.
	/// If `skip_checks` is true, some checks (like pool state or pallet balance) may be skipped.
	fn do_stake(
		evm_address: EvmAddress,
		pool_id: PoolId,
		amount: FusionCurrencyBalance,
		skip_checks: bool,
	) -> DispatchResult;

	/// Claims rewards for a specified era and pool for a given EVM address.
	fn do_claim_rewards(era: EraIndex, pool_id: PoolId, evm_address: EvmAddress) -> DispatchResult;

	/// Unbonds a specified amount of currency from a pool for a given EVM address.
	/// If `other` is true, the unbonding is performed on behalf of another user.
	fn do_unbond(
		evm_address: EvmAddress,
		pool_id: PoolId,
		unbond_amount: FusionCurrencyBalance,
		other: bool,
	) -> DispatchResult;

	/// Withdraws unbonded currency for a given EVM address after the bonding duration has passed.
	/// If `other` is true, the withdrawal is performed on behalf of another user.
	fn do_withdraw_unbonded_currency(
		evm_address: EvmAddress,
		pool_id: PoolId,
		other: bool,
	) -> DispatchResult;

	/// Withdraws AVAIL currency to the controller account for a given EVM address.
	fn do_withdraw_avail_to_controller(evm_address: EvmAddress) -> DispatchResult;

	/// Return the pool account for phragmen algorithm
	fn get_pool_account(id: PoolId) -> AccountId;

	/// Handles the change of an era, which includes operations like distributing rewards and cleaning up old data.
	fn handle_era_change(era_duration: u64) -> DispatchResult;

	/// Set the exposure for each pool for reward computation
	/// Exposure is set at the end of the era N for era N
	fn set_fusion_exposures(era: EraIndex) -> DispatchResult;
}
