use crate::primitives;
use subxt::{
	blocks::BlocksClient,
	config::substrate::BlakeTwo256,
	tx::{TxClient, TxInBlock},
	utils::{AccountId32, MultiAddress, MultiSignature, H256},
	Config, OnlineClient,
};

pub type AccountId = AccountId32;
pub type AccountIndex = u32;
pub type Address = MultiAddress<AccountId, AccountIndex>;
pub type Signature = MultiSignature;

pub type Api = OnlineClient<AvailConfig>;
pub type AvailBlocksClient = BlocksClient<AvailConfig, Api>;
pub type TxApi = TxClient<AvailConfig, Api>;
pub type TransactionInBlock = TxInBlock<AvailConfig, Api>;

#[derive(Clone, Copy, Default)]
pub struct AppId(pub avail_core::AppId);

#[derive(Clone, Debug, Default)]
pub struct AvailConfig;

impl Config for AvailConfig {
	type AccountId = AccountId;
	type Address = Address;
	type ExtrinsicParams = primitives::AvailExtrinsicParams<Self>;
	type Hash = H256;
	type Hasher = BlakeTwo256;
	type Header = primitives::Header;
	type Signature = Signature;
	type AssetId = u32;
}
