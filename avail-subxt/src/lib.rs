use anyhow::Result;
use structopt::StructOpt;
use subxt::{
	ext::{
		sp_core::H256,
		sp_runtime::{traits::BlakeTwo256, AccountId32, MultiAddress, MultiSignature},
	},
	Config, OnlineClient,
};

pub mod primitives;
use primitives::{AppUncheckedExtrinsic, AvailExtrinsicParams, Header};

#[derive(Clone, Debug, Default)]
pub struct AvailConfig;

pub type Signature = MultiSignature;
pub type AccountId = AccountId32;
pub type AccountIndex = u32;
pub type Address = MultiAddress<AccountId, AccountIndex>;
pub type Call = api::runtime_types::da_runtime::Call;
pub type SignaturePayload = (Address, Signature, AvailExtrinsicParams);

/// Avail Blockchain configuration
impl Config for AvailConfig {
	type AccountId = AccountId;
	type Address = Address;
	type BlockNumber = u32;
	// type Extrinsic = AvailExtrinsic;
	// type Extrinsic = Vec<u8>;
	type Extrinsic = AppUncheckedExtrinsic;
	type ExtrinsicParams = AvailExtrinsicParams;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Index = u32;
	type Signature = Signature;
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

#[cfg(test)]
mod test {
	use codec::{Decode as _, Error};
	use hex_literal::hex;
	use test_case::test_case;

	use super::{
		api::runtime_types::pallet_timestamp::pallet::Call as TimestampCall, AppUncheckedExtrinsic,
		Call,
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
