mod common;
pub mod definitions;
pub mod dev;
pub mod dev_tri;

use common::*;
pub use definitions::*;

pub mod biryani {
	use super::ChainSpec;

	pub fn chain_spec() -> Result<ChainSpec, String> {
		ChainSpec::from_json_bytes(&include_bytes!("./../../../misc/genesis/biryani.raw.json",)[..])
	}
}

pub mod dymension {
	use super::ChainSpec;

	pub fn chain_spec() -> Result<ChainSpec, String> {
		ChainSpec::from_json_bytes(
			&include_bytes!("./../../../misc/genesis/dymension.raw.json",)[..],
		)
	}
}

pub mod kate {
	use super::ChainSpec;

	pub fn chain_spec() -> Result<ChainSpec, String> {
		ChainSpec::from_json_bytes(
			&include_bytes!("./../../../misc/genesis/testnet.kate.chain.spec.raw.json",)[..],
		)
	}
}
