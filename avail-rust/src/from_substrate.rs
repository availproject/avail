use crate::{BlockHash, BlockNumber};
use serde::{Deserialize, Deserializer};

/// Network Peer information
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerInfo {
	/// Peer ID
	pub peer_id: String,
	/// Roles
	pub roles: String,
	/// Peer best block hash
	pub best_hash: BlockHash,
	/// Peer best block number
	pub best_number: BlockNumber,
}

/// The role the node is running as
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum NodeRole {
	/// The node is a full node
	Full,
	/// The node is an authority
	Authority,
}

/// The state of the syncing of the node.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncState {
	/// Height of the block at which syncing started.
	pub starting_block: BlockNumber,
	/// Height of the current best block of the node.
	pub current_block: BlockNumber,
	/// Height of the highest block in the network.
	pub highest_block: BlockNumber,
}

/// The base fee and adjusted weight and length fees constitute the _inclusion fee_.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InclusionFee {
	/// This is the minimum amount a user pays for a transaction. It is declared
	/// as a base _weight_ in the runtime and converted to a fee using `WeightToFee`.
	#[serde(deserialize_with = "number_from_hex")]
	pub base_fee: u128,
	/// The length fee, the amount paid for the encoded length (in bytes) of the transaction.
	#[serde(deserialize_with = "number_from_hex")]
	pub len_fee: u128,
	///
	/// - `targeted_fee_adjustment`: This is a multiplier that can tune the final fee based on the
	///   congestion of the network.
	/// - `weight_fee`: This amount is computed based on the weight of the transaction. Weight
	/// accounts for the execution time of a transaction.
	///
	/// adjusted_weight_fee = targeted_fee_adjustment * weight_fee
	#[serde(deserialize_with = "number_from_hex")]
	pub adjusted_weight_fee: u128,
}

impl InclusionFee {
	/// Returns the total of inclusion fee.
	///
	/// ```ignore
	/// inclusion_fee = base_fee + len_fee + adjusted_weight_fee
	/// ```
	pub fn inclusion_fee(&self) -> u128 {
		self.base_fee
			.saturating_add(self.len_fee)
			.saturating_add(self.adjusted_weight_fee)
	}
}

/// The `FeeDetails` is composed of:
///   - (Optional) `inclusion_fee`: Only the `Pays::Yes` transaction can have the inclusion fee.
///   - `tip`: If included in the transaction, the tip will be added on top. Only signed
///     transactions can have a tip.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeDetails {
	/// The minimum fee for a transaction to be included in a block.
	pub inclusion_fee: Option<InclusionFee>,
	// Do not serialize and deserialize `tip` as we actually can not pass any tip to the RPC.
	#[serde(skip)]
	pub tip: u128,
}

impl FeeDetails {
	/// Returns the final fee.
	///
	/// ```ignore
	/// final_fee = inclusion_fee + tip;
	/// ```
	pub fn final_fee(&self) -> u128 {
		self.inclusion_fee
			.as_ref()
			.map(|i| i.inclusion_fee())
			.unwrap_or_else(|| 0u128)
			.saturating_add(self.tip)
	}
}

fn number_from_hex<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
	D: Deserializer<'de>,
{
	let buf = String::deserialize(deserializer)?;
	let without_prefix = buf.trim_start_matches("0x");
	Ok(u128::from_str_radix(without_prefix, 16).unwrap())
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDispatchInfo {
	/// Weight of this dispatch.
	pub weight: Weight,
	/// Class of this dispatch.
	pub class: DispatchClass,
	/// The inclusion fee of this dispatch.
	///
	/// This does not include a tip or anything else that
	/// depends on the signature (i.e. depends on a `SignedExtension`).
	#[serde(deserialize_with = "number_from_hex_2")]
	pub partial_fee: u128,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DispatchClass {
	/// A normal dispatch.
	Normal,
	/// An operational dispatch.
	Operational,
	/// A mandatory dispatch. These kinds of dispatch are always included regardless of their
	/// weight, therefore it is critical that they are separately validated to ensure that a
	/// malicious validator cannot craft a valid but impossibly heavy block. Usually this just
	/// means ensuring that the extrinsic can only be included once and that it is always very
	/// light.
	///
	/// Do *NOT* use it for extrinsics that can be heavy.
	///
	/// The only real use case for this is inherent extrinsics that are required to execute in a
	/// block for the block to be valid, and it solves the issue in the case that the block
	/// initialization is sufficiently heavy to mean that those inherents do not fit into the
	/// block. Essentially, we assume that in these exceptional circumstances, it is better to
	/// allow an overweight block to be created than to not allow any block at all to be created.
	Mandatory,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Weight {
	/// The weight of computational time used based on some reference hardware.
	ref_time: u64,
	/// The weight of storage space used by proof of validity.
	proof_size: u64,
}

fn number_from_hex_2<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
	D: Deserializer<'de>,
{
	let buf = String::deserialize(deserializer)?;
	Ok(u128::from_str_radix(&buf, 10).unwrap())
}
