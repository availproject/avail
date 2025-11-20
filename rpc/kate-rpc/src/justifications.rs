use codec::Decode;
use da_runtime::Header;
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::ErrorObject,
};
use sc_client_api::BlockBackend;
use sp_core::H256;
use sp_runtime::{traits::Block as BlockT, AccountId32};
use std::{marker::PhantomData, sync::Arc};

/// GRANDPA consensus engine_id
pub const GRANDPA_ENGINE_ID: [u8; 4] = *b"FRNK";

#[rpc(client, server)]
pub trait Grandpa<Block>
where
	Block: BlockT,
{
	#[method(name = "grandpa_blockJustification")]
	async fn block_justification(&self, block_number: u32) -> RpcResult<Option<String>>;

	#[method(name = "grandpa_blockJustificationJson")]
	async fn block_justification_json(
		&self,
		block_number: u32,
	) -> RpcResult<Option<GrandpaJustification>>;
}

pub struct GrandpaJustifications<Client, Block: BlockT> {
	client: Arc<Client>,
	_block: PhantomData<Block>,
}

impl<Client, Block: BlockT> GrandpaJustifications<Client, Block> {
	pub fn new(client: Arc<Client>) -> Self {
		Self {
			client,
			_block: PhantomData,
		}
	}
}

/// Error type for this RPC API.
pub enum Error {
	/// Generic justification error.
	JustificationError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::JustificationError => 1,
		}
	}
}

macro_rules! internal_err {
	($($arg:tt)*) => {{
		ErrorObject::owned(
			Error::JustificationError.into(),
			format!($($arg)*),
			None::<()>
		)
	}}
}

#[async_trait]
impl<Client, Block> GrandpaServer<Block> for GrandpaJustifications<Client, Block>
where
	Block: BlockT,
	Client: Send + Sync + 'static,
	Client: BlockBackend<Block>,
{
	/// Returns the GRANDPA justification for the given block number, if available.
	///
	/// # Parameters
	/// - `block_number`: The number of the block for which the justification is requested.
	///
	/// # Returns
	/// - `Ok(Some(Vec<u8>))`: The encoded justification bytes for the block.
	/// - `Ok(None)`: Indicates that no justification is available for the specified block.
	/// - `Err`: If there is an error retrieving the block hash or justifications from the backend.
	///
	/// # Notes
	/// This method checks whether a justification exists in the backend for the block.
	/// If the justification exists for the `GRANDPA_ENGINE_ID`, it is returned.
	/// Otherwise, `None` is returned, indicating no justification is present.
	async fn block_justification(&self, block_number: u32) -> RpcResult<Option<String>> {
		// Fetch the block hash
		let block_hash = self
			.client
			.block_hash(block_number.into())
			.map_err(|e| internal_err!("Failed to fetch block hash: {e:?}"))?
			.ok_or_else(|| internal_err!("Block hash not found for block #{block_number}"))?;

		// Fetch the justification for the block hash
		let justification = self
			.client
			.justifications(block_hash)
			.map_err(|e| internal_err!("Failed to fetch justifications: {e:?}"))?
			.and_then(|just| just.into_justification(GRANDPA_ENGINE_ID));

		let Some(justification) = justification else {
			return Ok(None);
		};

		Ok(Some(const_hex::encode(&justification)))
	}

	/// Returns the GRANDPA justification for the given block number, if available.
	/// Same as block_justification but instead returns a decoded GrandpaJustification object
	async fn block_justification_json(
		&self,
		block_number: u32,
	) -> RpcResult<Option<GrandpaJustification>> {
		// Fetch the block hash
		let block_hash = self
			.client
			.block_hash(block_number.into())
			.map_err(|e| internal_err!("Failed to fetch block hash: {e:?}"))?
			.ok_or_else(|| internal_err!("Block hash not found for block #{block_number}"))?;

		// Fetch the justification for the block hash
		let justification = self
			.client
			.justifications(block_hash)
			.map_err(|e| internal_err!("Failed to fetch justifications: {e:?}"))?
			.and_then(|just| just.into_justification(GRANDPA_ENGINE_ID));

		let Some(justification) = justification else {
			return Ok(None);
		};

		let Ok(justification) = GrandpaJustification::decode(&mut justification.as_slice()) else {
			return Err(internal_err!("Failed to decode grandpa justification"));
		};

		Ok(Some(justification))
	}
}

#[derive(Clone, codec::Decode)]
pub struct AuthorityId(pub [u8; 32]);

impl serde::Serialize for AuthorityId {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let account_id = AccountId32::from(self.0);
		serializer.serialize_str(&account_id.to_string())
	}
}

impl<'de> serde::Deserialize<'de> for AuthorityId {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let account_id = AccountId32::deserialize(deserializer)?;
		Ok(Self(account_id.into()))
	}
}

#[derive(Clone, Copy, codec::Decode)]
pub struct Signature(pub [u8; 64]);

impl serde::Serialize for Signature {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&const_hex::encode_prefixed(self.0))
	}
}

impl<'de> serde::Deserialize<'de> for Signature {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let signature_hex =
			const_hex::decode(String::deserialize(deserializer)?.trim_start_matches("0x"))
				.map_err(|e| serde::de::Error::custom(format!("{:?}", e)))?;
		let signature: [u8; 64usize] = signature_hex
			.try_into()
			.map_err(|e| serde::de::Error::custom(format!("{:?}", e)))?;
		Ok(Self(signature))
	}
}

#[derive(Clone, codec::Decode, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export, export_to = "BlockJustification.ts"))]
pub struct Precommit {
	/// The target block's hash.
	#[cfg_attr(feature = "ts", ts(as = "String"))]
	pub target_hash: H256,
	/// The target block's number
	pub target_number: u32,
}

#[derive(Clone, codec::Decode, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export, export_to = "BlockJustification.ts"))]
pub struct SignedPrecommit {
	/// The precommit message which has been signed.
	pub precommit: Precommit,
	/// The signature on the message.
	#[cfg_attr(feature = "ts", ts(as = "String"))]
	pub signature: Signature,
	/// The Id of the signer.
	#[cfg_attr(feature = "ts", ts(as = "String"))]
	pub id: AuthorityId,
}

#[derive(Clone, codec::Decode, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export, export_to = "BlockJustification.ts"))]
pub struct Commit {
	/// The target block's hash.
	#[cfg_attr(feature = "ts", ts(as = "String"))]
	pub target_hash: H256,
	/// The target block's number.
	pub target_number: u32,
	/// Precommits for target block or any block after it that justify this commit.
	pub precommits: Vec<SignedPrecommit>,
}

#[derive(Clone, codec::Decode, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export, export_to = "BlockJustification.ts"))]
pub struct GrandpaJustification {
	pub round: u64,
	pub commit: Commit,
	#[cfg_attr(feature = "ts", ts(as = "Vec<String>"))]
	pub votes_ancestries: Vec<Header>,
}
