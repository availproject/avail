pub use anyhow::Result;
use structopt::StructOpt;

// Re-export some tools from `subxt`
pub use api::runtime_types::bounded_collections::bounded_vec::BoundedVec;
pub use subxt::{config, utils};
pub use subxt::backend::rpc::RpcParams;
use subxt::{
	config::substrate::BlakeTwo256,
	utils::{AccountId32, MultiAddress, MultiSignature, H256},
	Config,
};

pub mod primitives;
pub use primitives::Cell;
pub mod avail_client;
pub use avail_client::AvailClient;

pub mod rpc;
pub mod submit;
pub mod tx;

#[cfg(feature = "api-dev")]
mod api_dev;
#[cfg(feature = "api-dev")]
pub use api_dev::api;

pub mod helpers;

#[derive(Clone, Debug, Default)]
pub struct AvailConfig;

pub type Signature = MultiSignature;
pub type AccountId = AccountId32;
pub type AccountIndex = u32;
pub type Address = MultiAddress<AccountId, AccountIndex>;
pub type Call = api::runtime_types::da_runtime::RuntimeCall;
pub type SignaturePayload = (Address, Signature, primitives::ExtrinsicParams);
pub type AppId = api::runtime_types::avail_core::AppId;

/// Avail Blockchain configuration
impl Config for AvailConfig {
	type AccountId = AccountId;
	type Address = Address;
	type ExtrinsicParams = primitives::ExtrinsicParams;
	type Hash = H256;
	type Hasher = BlakeTwo256;
	type Header = primitives::Header;
	type Signature = Signature;
	type AssetId = u32;
}

pub mod avail {
	use super::*;
	use sp_core::ConstU32;

	pub type Client = subxt::OnlineClient<AvailConfig>;
	pub type TxProgress = subxt::tx::TxProgress<AvailConfig, Client>;
	pub type TxInBlock = subxt::tx::TxInBlock<AvailConfig, Client>;
	pub type TxStatus = subxt::tx::TxStatus<AvailConfig, Client>;

	pub type RuntimeCall = api::runtime_types::da_runtime::RuntimeCall;
	pub type BlakeTwo256 = api::runtime_types::sp_runtime::traits::BlakeTwo256;
	pub type Bounded =
		api::runtime_types::frame_support::traits::preimages::Bounded<RuntimeCall, BlakeTwo256>;

	pub const AVAIL: u128 = 1_000_000_000_000_000_000;

	pub type MaxCells = ConstU32<64>;
	pub type Cells = bounded_collections::BoundedVec<Cell, MaxCells>;

	pub type MaxRows = ConstU32<64>;
	pub type Rows = bounded_collections::BoundedVec<u32, MaxRows>;
}

#[allow(dead_code)]
#[derive(Debug, StructOpt)]
pub struct Opts {
	/// The WebSocket address of the target the Avail Node,
	#[structopt(name = "ws_uri", long, default_value = "ws://127.0.0.1:9944")]
	pub ws: String,

	/// Check whether the Client you are using is aligned with the statically generated codegen.
	#[structopt(name = "validate_codege", short = "c", long)]
	pub validate_codegen: bool,
}
