use anyhow::Result;
use structopt::StructOpt;
// Re-export some tools from `subxt`
pub use api::runtime_types::bounded_collections::bounded_vec::BoundedVec;
pub use subxt::{config, rpc, utils};
use subxt::{
	config::substrate::BlakeTwo256,
	utils::{AccountId32, MultiAddress, MultiSignature, H256},
	Config, OnlineClient,
};

pub mod primitives;
use primitives::{AvailExtrinsicParams, Header};

#[derive(Clone, Debug, Default)]
pub struct AvailConfig;

pub type Signature = MultiSignature;
pub type AccountId = AccountId32;
pub type AccountIndex = u32;
pub type Address = MultiAddress<AccountId, AccountIndex>;
pub type Call = api::runtime_types::da_runtime::RuntimeCall;
pub type SignaturePayload = (Address, Signature, AvailExtrinsicParams);
pub type AppId = api::runtime_types::avail_core::AppId;

/// Avail Blockchain configuration
impl Config for AvailConfig {
	type AccountId = AccountId;
	type Address = Address;
	type ExtrinsicParams = AvailExtrinsicParams;
	type Hash = H256;
	type Hasher = BlakeTwo256;
	type Header = Header;
	type Index = u32;
	type Signature = Signature;
}

pub mod avail {
	use super::{api, AvailConfig};

	pub type Client = subxt::OnlineClient<AvailConfig>;
	pub type TxProgress = subxt::tx::TxProgress<AvailConfig, Client>;
	pub type TxInBlock = subxt::tx::TxInBlock<AvailConfig, Client>;
	pub type AppUncheckedExtrinsic =
		crate::primitives::app_unchecked_extrinsic::AppUncheckedExtrinsic;
	pub type Pair = sp_core::sr25519::Pair;
	pub type PairSigner = subxt::tx::PairSigner<AvailConfig, Pair>;

	pub type RuntimeCall = api::runtime_types::da_runtime::RuntimeCall;
	pub type Bounded = api::runtime_types::frame_support::traits::preimages::Bounded<RuntimeCall>;

	pub const AVL: u128 = 1_000_000_000_000_000_000;
}

#[cfg(feature = "api-dev")]
mod api_dev;
#[cfg(feature = "api-dev")]
pub use api_dev::api;

pub mod helpers;

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

/// Creates a client and validate the code generation if `validate_codegen == true`.
pub async fn build_client<U: AsRef<str>>(
	url: U,
	validate_codegen: bool,
) -> Result<OnlineClient<AvailConfig>> {
	let api = OnlineClient::<AvailConfig>::from_url(url).await?;
	if validate_codegen {
		api::validate_codegen(&api)?;
	}
	Ok(api)
}

pub async fn submit_data<ID, D>(
	client: &OnlineClient<AvailConfig>,
	signer: &avail::PairSigner,
	data: D,
	app_id: ID,
) -> Result<avail::TxProgress>
where
	ID: Into<AppId>,
	D: Into<Vec<u8>>,
{
	let data = BoundedVec(data.into());
	let call = api::tx().data_availability().submit_data(data);
	let tx = tx_async_send!(client, &call, signer, app_id);
	Ok(tx)
}

pub async fn submit_data_in_block<ID, D>(
	client: &OnlineClient<AvailConfig>,
	signer: &avail::PairSigner,
	data: D,
	app_id: ID,
) -> Result<avail::TxInBlock>
where
	ID: Into<AppId>,
	D: Into<Vec<u8>>,
{
	let tx_in_block = submit_data(client, signer, data, app_id)
		.await?
		.wait_for_in_block()
		.await?;
	Ok(tx_in_block)
}

pub async fn submit_data_finalized<ID, D>(
	client: &OnlineClient<AvailConfig>,
	signer: &avail::PairSigner,
	data: D,
	app_id: ID,
) -> Result<avail::TxInBlock>
where
	ID: Into<AppId>,
	D: Into<Vec<u8>>,
{
	let tx_in_block = submit_data(client, signer, data, app_id)
		.await?
		.wait_for_finalized()
		.await?;
	Ok(tx_in_block)
}

#[macro_export]
macro_rules! tx_async_send {
	($client: expr, $call: expr, $signer: expr, $app_id: expr) => {{
		let ext_params = $crate::primitives::AvailExtrinsicParams::new_with_app_id($app_id.into());
		$client
			.tx()
			.sign_and_submit_then_watch($call, $signer, ext_params)
			.await?
	}};
	($client: expr, $call: expr, $signer: expr) => {
		$client
			.tx()
			.sign_and_submit_then_watch_default($call, $signer)
			.await?
	};
}

#[macro_export]
macro_rules! tx_send_in_block {
	($client: expr, $call: expr, $signer: expr, $app_id: expr) => {
		$crate::tx_async_send!($client, $call, $signer, $app_id)
			.wait_for_in_block()
			.await?
	};
	($client: expr, $call: expr, $signer: expr) => {
		$crate::tx_async_send!($client, $call, $signer)
			.wait_for_in_block()
			.await?
	};
}

#[macro_export]
macro_rules! tx_send_in_finalized {
	($client: expr, $call: expr, $signer: expr, $app_id: expr) => {
		$crate::tx_async_send!($client, $call, $signer, $app_id).wait_for_finalized_success()
	};
	($client: expr, $call: expr, $signer: expr) => {
		$crate::tx_async_send!($client, $call, $signer).wait_for_finalized_success()
	};
}

#[cfg(test)]
mod test {
	use codec::{Decode as _, Error};
	use hex_literal::hex;
	use test_case::test_case;

	use super::{
		api::runtime_types::pallet_timestamp::pallet::Call as TimestampCall,
		primitives::AppUncheckedExtrinsic, Call,
	};

	const TIMESTAMP_1: &[u8] = &hex!("280403000b804aa9518401");
	fn timestamp_1_call() -> Result<Call, Error> {
		Ok(Call::Timestamp(TimestampCall::set {
			now: 1_667_817_360_000,
		}))
	}

	#[test_case( TIMESTAMP_1.to_vec() => timestamp_1_call(); "Timestamp 16678173600000" )]
	fn decode_extrinsic(encoded_ext: Vec<u8>) -> Result<Call, Error> {
		<AppUncheckedExtrinsic>::decode(&mut encoded_ext.as_slice()).map(|ext| ext.function)
	}
}
