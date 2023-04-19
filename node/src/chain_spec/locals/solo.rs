use da_runtime::GenesisConfig;
use sp_core::sr25519::Public;

use crate::chain_spec::{
	chain_properties, get_account_id_from_seed,
	locals::{dev_endowed_accounts, make_genesis},
	tech_committee_from_authorities, AuthorityKeys, ChainSpec, ChainType, FORK_ID, PROTOCOL_ID,
};

pub fn chain_spec() -> ChainSpec {
	let empty_boot_nodes = vec![];
	ChainSpec::from_genesis(
		"Avail Local Solo",
		"Avail Local",
		ChainType::Development,
		config_genesis,
		empty_boot_nodes,
		None,
		PROTOCOL_ID,
		FORK_ID,
		chain_properties(),
		Default::default(),
	)
}

/// Local Solo Chain genesis configuration.
///
/// It defines:
/// - `Alice` as unique initial authority.
/// - `Alice` as `sudo` account.
/// - All devs account are endowed.
///
fn config_genesis() -> GenesisConfig {
	let sudo = get_account_id_from_seed::<Public>("Alice");
	let authorities = vec![AuthorityKeys::from_seed("Alice")];
	let tc_members = tech_committee_from_authorities(&authorities);
	let endowed_accs = dev_endowed_accounts();

	make_genesis(sudo, authorities, tc_members, endowed_accs)
}
