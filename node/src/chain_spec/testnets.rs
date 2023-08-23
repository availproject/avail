use std::collections::HashMap;

use avail_core::currency::AVL;
use da_runtime::{AccountId, Balance};

use crate::chain_spec::{AuthorityKeys, RuntimeGenesisConfig};

pub mod kate;

const MIN_VALIDATOR_BOND: Balance = 10 * AVL;
const MIN_NOMINATOR_BOND: Balance = 1 * AVL;

fn make_genesis(
	sudo: AccountId,
	authorities: Vec<AuthorityKeys>,
	council: Vec<AccountId>,
	tech_committee_members: Vec<AccountId>,
	endowed_accounts: HashMap<AccountId, Balance>,
) -> RuntimeGenesisConfig {
	crate::chain_spec::make_genesis(
		sudo,
		authorities,
		council,
		tech_committee_members,
		endowed_accounts,
		MIN_VALIDATOR_BOND,
		MIN_NOMINATOR_BOND,
	)
}
