mod common;
mod definitions;

use common::*;
pub use definitions::*;

use serde_json::Value;
#[cfg(test)]
use sp_runtime::BuildStorage;

pub mod dev {
	use super::*;
	use da_runtime::wasm_binary_unwrap;
	use sc_chain_spec::ChainType;

	pub fn chain_spec() -> ChainSpec {
		ChainSpec::builder(wasm_binary_unwrap(), Default::default())
			.with_name("Avail Development Network")
			.with_id("avail_development_network")
			.with_chain_type(ChainType::Development)
			.with_genesis_config_patch(genesis_constructor())
			.with_telemetry_endpoints(super::to_telemetry_endpoint(TELEMETRY_URL.into()))
			.with_protocol_id(PROTOCOL_ID)
			.with_properties(chain_properties())
			.with_boot_nodes(vec![])
			.build()
	}

	pub fn genesis_constructor() -> Value {
		let alice = AuthorityKeys::from_seed("Alice");
		let sudo = alice.controller.clone();

		runtime_genesis_config(sudo.clone(), vec![sudo.clone()], vec![sudo], vec![alice])
	}

	#[test]
	fn test_chain_spec_creation() {
		chain_spec().build_storage().unwrap();
	}
}

pub mod dev_tri {
	use super::*;
	use da_runtime::wasm_binary_unwrap;
	use sc_chain_spec::ChainType;

	pub fn chain_spec() -> ChainSpec {
		ChainSpec::builder(wasm_binary_unwrap(), Default::default())
			.with_name("Avail Tri Development Network")
			.with_id("avail_tri_development_network")
			.with_chain_type(ChainType::Development)
			.with_genesis_config_patch(genesis_constructor())
			.with_telemetry_endpoints(super::to_telemetry_endpoint(TELEMETRY_URL.into()))
			.with_protocol_id(PROTOCOL_ID)
			.with_properties(chain_properties())
			.with_boot_nodes(vec![])
			.build()
	}

	pub fn genesis_constructor() -> Value {
		let alice = AuthorityKeys::from_seed("Alice");
		let bob = AuthorityKeys::from_seed("Bob");
		let charlie = AuthorityKeys::from_seed("Charlie");
		let sudo = alice.controller.clone();

		runtime_genesis_config(
			sudo.clone(),
			vec![sudo.clone()],
			vec![sudo],
			vec![alice, bob, charlie],
		)
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

pub mod mainnet {
	use super::*;

	pub fn chain_spec() -> Result<ChainSpec, String> {
		ChainSpec::from_json_bytes(
			&include_bytes!("./../../../misc/genesis/mainnet.chain.spec.raw.json",)[..],
		)
	}

	#[test]
	fn test_chain_spec_creation() {
		chain_spec().unwrap().build_storage().unwrap();
	}
}

pub mod new {
	use super::*;

	pub fn chain_spec() -> Result<ChainSpec, String> {
		ChainSpec::from_json_bytes(
			&include_bytes!("./../../../misc/genesis/testnet.new.chain.spec.raw.json",)[..],
		)
	}

	#[test]
	fn test_chain_spec_creation() {
		chain_spec().unwrap().build_storage().unwrap();
	}
}
