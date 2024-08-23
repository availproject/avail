use crate::{DefaultExtrinsicParams, DefaultExtrinsicParamsBuilder, AvailHeader};
use subxt::{
	backend::legacy::rpc_methods::{Block as BlockRPC, BlockDetails as BlockDetailsRPC},
	blocks::BlocksClient,
	config::substrate::BlakeTwo256,
	tx::{TxClient, TxInBlock},
	utils::{AccountId32, MultiAddress, MultiSignature, H256},
	Config, OnlineClient,
};

/// Chain Primitives
pub type AccountId = AccountId32;
pub type AccountIndex = u32;
pub type Address = MultiAddress<AccountId, AccountIndex>;
pub type Signature = MultiSignature;
pub type BlockNumber = u32;
pub type BlockHash = H256;

/// Clients
pub type Api = OnlineClient<AvailConfig>;
pub type AvailBlocksClient = BlocksClient<AvailConfig, Api>;
pub type TxApi = TxClient<AvailConfig, Api>;

/// TX status
pub type TransactionInBlock = TxInBlock<AvailConfig, Api>;

/// Used only when chain_getBlock RPC is called. This is part of legacy baggage.
pub type AvailBlockDetailsRPC = BlockDetailsRPC<AvailConfig>;
pub type AvailBlockRPC = BlockRPC<AvailConfig>;

/// A struct representing the signed extra and additional parameters required
/// to construct a transaction for a avail node.
pub type AvailExtrinsicParams<T> = DefaultExtrinsicParams<T>;

/// A builder which leads to [`PolkadotExtrinsicParams`] being constructed.
/// This is what you provide to methods like `sign_and_submit()`.
pub type AvailExtrinsicParamsBuilder<T> = DefaultExtrinsicParamsBuilder<T>;

#[derive(Clone, Copy, Default, Debug)]
pub struct AppId(pub avail_core::AppId);

impl From<avail_core::AppId> for AppId {
	fn from(value: avail_core::AppId) -> Self {
		Self(value)
	}
}

#[derive(Clone, Debug, Default)]
pub struct AvailConfig;

impl Config for AvailConfig {
	type AccountId = AccountId;
	type Address = Address;
	type ExtrinsicParams = AvailExtrinsicParams<Self>;
	type Hash = BlockHash;
	type Hasher = BlakeTwo256;
	type Header = AvailHeader;
	type Signature = Signature;
	type AssetId = u32;
}
