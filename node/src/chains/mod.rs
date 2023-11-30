mod common;
mod definitions;

use common::*;
pub use definitions::*;

#[cfg(test)]
use sp_runtime::BuildStorage;

pub mod goldberg {
	use super::*;

	pub fn chain_spec() -> Result<ChainSpec, String> {
		ChainSpec::from_json_bytes(
			&include_bytes!("./../../../misc/genesis/testnet.goldberg.chain.raw.json",)[..],
		)
	}

	#[test]
	fn test_chain_spec_creation() {
		chain_spec().unwrap().build_storage().unwrap();
	}
}

pub mod dev {
	use super::*;
	use da_runtime::RuntimeGenesisConfig;
	use sc_chain_spec::ChainType;

	pub fn chain_spec() -> ChainSpec {
		ChainSpec::from_genesis(
			"Avail Development Network",
			"avail_development_network",
			ChainType::Development,
			genesis_constructor,
			vec![],
			Some(super::to_telemetry_endpoint(TELEMETRY_URL.into())),
			PROTOCOL_ID,
			None,
			chain_properties(),
			Default::default(),
		)
	}

	pub fn genesis_constructor() -> RuntimeGenesisConfig {
		let alice = AuthorityKeys::from_seed("Alice");
		let sudo = alice.controller.clone();

		runtime_genesis_config(sudo.clone(), vec![sudo], vec![alice])
	}

	#[test]
	fn test_chain_spec_creation() {
		chain_spec().build_storage().unwrap();
	}
}

pub mod dev_tri {
	use super::*;
	use da_runtime::RuntimeGenesisConfig;
	use sc_chain_spec::ChainType;

	pub fn chain_spec() -> ChainSpec {
		ChainSpec::from_genesis(
			"Avail Tri Development Network",
			"avail_tri_development_network",
			ChainType::Development,
			genesis_constructor,
			vec![],
			Some(super::to_telemetry_endpoint(TELEMETRY_URL.into())),
			PROTOCOL_ID,
			None,
			chain_properties(),
			Default::default(),
		)
	}

	pub fn genesis_constructor() -> RuntimeGenesisConfig {
		let alice = AuthorityKeys::from_seed("Alice");
		let bob = AuthorityKeys::from_seed("Bob");
		let charlie = AuthorityKeys::from_seed("Charlie");
		let sudo = alice.controller.clone();

		runtime_genesis_config(sudo.clone(), vec![sudo], vec![alice, bob, charlie])
	}

	#[test]
	fn test_chain_spec_creation() {
		chain_spec().build_storage().unwrap();
	}
}

pub mod devnet0 {
	use super::*;

	pub fn chain_spec() -> Result<ChainSpec, String> {
		ChainSpec::from_json_bytes(
			&include_bytes!("./../../../misc/genesis/devnet.chain.raw.json",)[..],
		)
	}

	#[test]
	fn test_chain_spec_creation() {
		chain_spec().unwrap().build_storage().unwrap();
	}
}
