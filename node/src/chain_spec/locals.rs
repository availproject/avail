use std::collections::HashMap;

use avail_core::currency::AVL;
use da_runtime::{AccountId, Balance, RuntimeGenesisConfig};
use sp_core::sr25519::Public;

use crate::chain_spec::{get_account_id_from_seed, AuthorityKeys};

#[rustfmt::skip]
const DEFAULT_ENDOWED_SEEDS :[&str;12] = [ 
	"Alice", "Bob", "Charlie", "Dave", "Eve", "Ferdie", 
	"Alice//stash", "Bob//stash", "Charlie//stash", "Dave//stash", "Eve//stash", "Ferdie//stash",
];
const ENDOWMENT: Balance = 1_000 * AVL;
const MIN_VALIDATOR_BOND: Balance = 10 * AVL;
const MIN_NOMINATOR_BOND: Balance = 1 * AVL;

/// Generates a default endowed accounts.
fn dev_endowed_accounts() -> HashMap<AccountId, Balance> {
	DEFAULT_ENDOWED_SEEDS
		.iter()
		.map(|seed| (get_account_id_from_seed::<Public>(seed), ENDOWMENT))
		.collect::<HashMap<_, _>>()
}

/// `Alice` is the sudo key in `dev`.
fn dev_sudo() -> AccountId {
	get_account_id_from_seed::<Public>("Alice")
}

fn make_genesis(
	sudo: AccountId,
	authorities: Vec<AuthorityKeys>,
	endowed_accounts: HashMap<AccountId, Balance>,
) -> RuntimeGenesisConfig {
	let council = DEFAULT_ENDOWED_SEEDS[..5]
		.iter()
		.map(|seed| get_account_id_from_seed::<Public>(seed))
		.collect::<Vec<_>>();
	let tech_committee = council.clone();

	crate::chain_spec::make_genesis(
		sudo,
		authorities,
		council,
		tech_committee,
		endowed_accounts,
		MIN_VALIDATOR_BOND,
		MIN_NOMINATOR_BOND,
	)
}

pub mod solo;
pub mod tri;
