use crate::primitives;
use subxt::{
	config::substrate::BlakeTwo256,
	utils::{AccountId32, MultiAddress, MultiSignature, H256},
	Config,
};

pub type AccountId = AccountId32;
pub type AccountIndex = u32;
pub type Address = MultiAddress<AccountId, AccountIndex>;
pub type Signature = MultiSignature;

#[derive(Clone, Copy, Default)]
pub struct AppId(pub avail_core::AppId);

#[derive(Clone, Debug, Default)]
pub struct AvailConfig;

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
