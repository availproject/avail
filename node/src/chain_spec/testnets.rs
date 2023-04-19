use da_primitives::currency::AVL;
use da_runtime::{AccountId, Balance};

use crate::chain_spec::{AuthorityKeys, GenesisConfig};

pub mod ada;

const ENDOWMENT: Balance = 1_000 * AVL;
const MIN_VALIDATOR_BOND: Balance = 10 * AVL;
const MIN_NOMINATOR_BOND: Balance = 1 * AVL;

fn make_genesis(
	sudo: AccountId,
	authorities: Vec<AuthorityKeys>,
	tech_committee_members: Vec<AccountId>,
	endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
	crate::chain_spec::make_genesis(
		sudo,
		authorities,
		tech_committee_members,
		endowed_accounts,
		ENDOWMENT,
		MIN_VALIDATOR_BOND,
		MIN_NOMINATOR_BOND,
	)
}
