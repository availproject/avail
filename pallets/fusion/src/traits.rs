use crate::*;
use sp_staking::EraIndex;

// A trait that provides data from the staking pallet.
pub trait StakingFusionDataProvider<AccountId> {
	/// Returns the active era.
	fn active_era() -> EraIndex;
	/// Returns the currently planned era.
	fn current_era() -> EraIndex;
	/// Checks if an account is a validator.
	fn is_valid_validator(account: &AccountId) -> bool;
	/// Checks if a validator has earned era points for an era (meaning he'll get rewards).
	fn has_earned_era_points(era: EraIndex, accounts: &Vec<AccountId>) -> bool;
}
impl<AccountId> StakingFusionDataProvider<AccountId> for () {
	fn active_era() -> EraIndex {
		0
	}
	fn current_era() -> EraIndex {
		0
	}
	fn is_valid_validator(_account: &AccountId) -> bool {
		false
	}
	fn has_earned_era_points(_era: EraIndex, _accounts: &Vec<AccountId>) -> bool {
		false
	}
}