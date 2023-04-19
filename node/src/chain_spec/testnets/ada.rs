use da_runtime::{AccountId, GenesisConfig};
use hex_literal::hex;
use sc_service::ChainType;
use sp_core::crypto::UncheckedInto;

use crate::chain_spec::{
	chain_properties, testnets::make_genesis, AuthorityKeys, ChainSpec, FORK_ID, PROTOCOL_ID,
};

#[rustfmt::skip]
fn authorities() -> Vec<AuthorityKeys> {
	vec![
		// Validator 1.
		AuthorityKeys::from_accounts(
			hex!("b2dad89ebcbf90f48021d8f18ad72339da8eb3de16fcefac84402c7615dce932").into(),
			hex!("9fa3481874ee8d02456438fff97273f320a9e11ad45f52cbc8391b9165d7aa70").unchecked_into(),
		),
		// Validator 2.
		AuthorityKeys::from_accounts(
			hex!("101020d67afe1df8b2811d30e8a6d57dc74bc2c1b76d4f8c4b20da21b581f710").into(),
			hex!("3f190c60714a0f233bcc6a8941e3802b3a1eeca6dbef157727c859a13c6bca1e").unchecked_into(),
		),
		// Validator 3.
		AuthorityKeys::from_accounts(
			hex!("e099cabb7e96b3e5033fcc9eac120f15877b03881aafe811c98fe48e67047e06").into(),
			hex!("2d12c83a0bd991a8888620a215c1e214d954790cb4c42a7adaf7e49d877f099f").unchecked_into(),
		),
	]
}

pub fn technical_committee() -> Vec<AccountId> {
	vec![
		hex!("d2fba4e644431142a62d320c4ca1590a1b493af416d165a0e502b08376babc4a").into(), // TC 1
		hex!("a0777497f1d3b4c163c8cc265a201eb7c9fc0eaea2313641260ab622deb8882a").into(), // TC 2
		hex!("cac01e6ea3ecd574c8f91800669ee2fcd164f88b0a51b83bfb1fa5ed986d5023").into(), // TC 3
	]
}

#[rustfmt::skip]
const SUDO: AccountId = AccountId::new(hex!("f8ce86d05d54a80ca05e8879bdaeecfc56169ec41f3a9bebf58c07dcaa5b0423"));

fn user_accounts() -> Vec<AccountId> {
	[
		SUDO,
		// Faucet bridge.
		hex!("166efe5750d52f473dee8ef21a3b31779e09b2140753db34f9e9aa6cab6c9000").into(),
	]
	.to_vec()
}

pub fn chain_spec() -> ChainSpec {
	ChainSpec::from_genesis(
		"Avail Ada Testnet",
		"Avail Testnet",
		ChainType::Live,
		config_genesis,
		vec![],
		None,
		PROTOCOL_ID,
		FORK_ID,
		chain_properties(),
		Default::default(),
	)
}

fn config_genesis() -> GenesisConfig {
	let authorities = authorities();
	let tc_members = technical_committee();
	let endowed_accs = user_accounts();

	make_genesis(SUDO, authorities, tc_members, endowed_accs)
}
