use std::{collections::HashMap, iter::once};

use da_runtime::{constants::elections::InitialMemberBond, AccountId, Balance, GenesisConfig, AVL};
use hex_literal::hex;
use sc_service::ChainType;
use sp_core::crypto::UncheckedInto;

use crate::chain_spec::{
	chain_properties, testnets::make_genesis, AuthorityKeys, ChainSpec, FORK_ID, PROTOCOL_ID,
};

fn authorities() -> Vec<AuthorityKeys> {
	vec![
		// Validator 1 5CDA2QG5ir4EUuWM5Mpve328P1YXtUH4M5orPwvkPoboCJzD
		AuthorityKeys::from_accounts(
			hex!("06720d4c1c934b28cce1c830714d587d0b5ddc43968fcbce51dd13f017be8d46").into(),
			hex!("995d40330b598da1ad50e38aa7d72550103bdb5878ffca945fe5a49e3014154b")
				.unchecked_into(),
		),
		// Validator 2 5FsQBbxBeUvkPv8vJbrhs2j5WCcRQb9jTEa3M41t7iVZhhKT
		AuthorityKeys::from_accounts(
			hex!("a851bd6bb41d8614645d6a12ce2431ab30819af0483cfb7870038ce3aef55c38").into(),
			hex!("e3f90d187c983c99f6648b8035cd63ec49cc6889592953edf4feaa8247022834")
				.unchecked_into(),
		),
		// Validator 3 5Ey2QhfwiQBHoQ1MU9xYkLzoQRRfXnhQaJPbAtFZ5pHhkRuT
		AuthorityKeys::from_accounts(
			hex!("805fae3795a8f479bdafd34753957dcf26ce7207adbedfde51ce6a1872a1827b").into(),
			hex!("eb236122afff271e82ebc489e1cea12e6352668b518ccca8604aacb5b9b38695")
				.unchecked_into(),
		),
		// Validator 4 5Gj58EEcyXhVu16dcmLBZVxNXq4vvwxEnaKbyhmen15TxDu4
		AuthorityKeys::from_accounts(
			hex!("ce33dac6783b876d7d6b5012a0e25b5fe93a46824fbaa2fef6f229f43deb8a28").into(),
			hex!("acd803975a0e70942a507f20f4ad0572bc17bb1321dc204054b7dc6af82eaae5")
				.unchecked_into(),
		),
		// Validator 5 5ETyro1PqeQJrYXots7e4m2donhrede251s8CQNZPCNbpV9K
		AuthorityKeys::from_accounts(
			hex!("6a38fab2076c1634b743aeeac004e55c6ab560a66f24f1e6baa134f2a2fbf24e").into(),
			hex!("379a8e9cea41c74405b617c8e4ddbd6455664ecd4878a01584b32c6d4391d20a")
				.unchecked_into(),
		),
		// Validator 6 5ECEHdvgq79WJGuR55PR2zjQNpregui6B8KHgM1tEoT1bYDg
		AuthorityKeys::from_accounts(
			hex!("5e359f775013228eb4ecc54ca3af7d2c14c5fc50f1f3eafed775515ca7c8af79").into(),
			hex!("31920433c10117862ee3f3b52f102b15f81cf7fcd0a0a7c2be9f0e417b953181")
				.unchecked_into(),
		),
		// Validator 7 5CSt9Pn7NrTHpHSDTC5Yd4YHronw64HAPr9h6K5YR6UMYV8y
		AuthorityKeys::from_accounts(
			hex!("10ea08584efaa8bf8c9563333b47bec02ecd3046feb53a1491e23e794104f34f").into(),
			hex!("8d41197aa896bd924ad41662702fcd5477e59370cb226183664fca120bfc2cc5")
				.unchecked_into(),
		),
	]
}

pub fn council_members() -> Vec<AccountId> {
	[
		// 5CiSXC1o4L257Gq7vd244jgGfMsWQadSqVwwLEz8KvF9ME36
		hex!("1cc7b34b29b7166de967d6bc7fae39430a4787d92b9bac9e53e3dc462a0a9b7a").into(),
		// 5DnsSWZRC7MiPKcpfYdbj67FSUvSMMNMMF8Uw3ZdAqYd4rPZ
		hex!("4c64cd21d679f0fb0c10f3320c70f61e142c53d915cd4188bb909810506a7835").into(),
		// 5FH2QRq2k2uA6HHwwMggDbjGHjJJE9R3SXac9aBPxcK6rn8x
		hex!("8e1a14fb549c3c9e96f6a31bad454fe16f40c22e67a3e536e99443a7e8143f7a").into(),
		// 5GmUKWK1aoqyndcJcmDfJScF26LUwyj1woVN2E2AhCdZw4Mk
		hex!("d0086cc14858ddc8edb7adc5c45e2ae881738bc2fe43e88be3e2239268b59f27").into(),
	]
	.to_vec()
}

pub fn technical_committee() -> Vec<AccountId> {
	[
		// 5CSgF4xbSZ5EN2Z52tbdu63CwYnPsvM6HahhKjoXumV8AJom
		hex!("10c1f76625e973060cf4fd26ae9fc7a844455f0ace85ea6c542ecaab18d79228").into(),
		// 5CcnpV9rsL98Vv8CxvSDXNXXi4vTZLc2pzv1KKJcSvvQuXGh
		hex!("18788ec9bf57fe0c0ac41d2559b24e9ee9ff2324f81ce0577e5d8937c993fd09").into(),
		// 5EHdVc8aX4KGQrL1MxNBgrkGJDNBe6Xg1PcGQTyvKLxhrfiV
		hex!("6253f8324cfa92fb324e2b3c469256ea7ec3ae98611071be69524bd24f10b64c").into(),
	]
	.to_vec()
}

#[rustfmt::skip]
const SUDO: AccountId = AccountId::new(
	// 5C7vaq3yC94X4mbv4cgrrNod1G5HLhaeiBP8aeSBCy4iZyUp
	hex!("0274942c0845b86feae8dc7ed7448d79f2781b2440f83135b9e5094087ca0829"));

// TODO --- Review below items

fn user_accounts(council: &[AccountId]) -> HashMap<AccountId, Balance> {
	council
		.iter()
		.map(|acc| (acc.clone(), InitialMemberBond::get()))
		.chain(once((SUDO, 100 * AVL)))
		.collect()
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
	let council = council_members();
	let endowed_accs = user_accounts(&council);

	make_genesis(SUDO, authorities, council, tc_members, endowed_accs)
}
