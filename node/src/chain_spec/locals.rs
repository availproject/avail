use da_primitives::currency::AVL;
use da_runtime::{AccountId, Balance, GenesisConfig};
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
fn dev_endowed_accounts() -> Vec<AccountId> {
	DEFAULT_ENDOWED_SEEDS
		.iter()
		.map(|seed| get_account_id_from_seed::<Public>(seed))
		.collect::<Vec<_>>()
}

/// `Alice` is the sudo key in `dev`.
fn dev_sudo() -> AccountId { get_account_id_from_seed::<Public>("Alice") }

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

pub mod solo;
pub mod tri;
