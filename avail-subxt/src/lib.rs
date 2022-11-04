use anyhow::Result;
use structopt::StructOpt;
use subxt::{
	ext::{
		sp_core::H256,
		sp_runtime::{
			traits::BlakeTwo256, AccountId32, MultiAddress, MultiSignature, OpaqueExtrinsic,
		},
	},
	Config, OnlineClient,
};

pub mod primitives;
use primitives::{AvailExtrinsicParams, Header};

#[derive(Clone, Debug, Default)]
pub struct AvailConfig;

/// Avail Blockchain configuration
impl Config for AvailConfig {
	type AccountId = AccountId32;
	type Address = MultiAddress<Self::AccountId, u32>;
	type BlockNumber = u32;
	// type Extrinsic = AvailExtrinsic;
	type Extrinsic = OpaqueExtrinsic;
	type ExtrinsicParams = AvailExtrinsicParams;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Index = u32;
	type Signature = MultiSignature;
}

#[cfg(feature = "api-dev")]
mod api_dev;
#[cfg(feature = "api-dev")]
pub use api_dev::api;

#[allow(dead_code)]
#[derive(Debug, StructOpt)]
pub struct Opts {
	/// The WebSocket address of the target the Avail Node,
	#[structopt(name = "ws_uri", long, default_value = "ws://127.0.0.1:9944")]
	pub ws: String,
}

/// Creates a client and validate the code generation.
pub async fn build_client<U: AsRef<str>>(url: U) -> Result<OnlineClient<AvailConfig>> {
	let api = OnlineClient::<AvailConfig>::from_url(url).await?;
	api::validate_codegen(&api)?;
	Ok(api)
}
