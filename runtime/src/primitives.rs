use crate::{extensions, AllPalletsWithSystem, Runtime, RuntimeCall};
use avail_core::header::Header as DaHeader;
use codec::{Decode, Encode};
use scale_info::prelude::{format, string::String};

use sp_runtime::{
	generic, impl_opaque_keys,
	traits::{BlakeTwo256, IdentifyAccount, Verify},
	MultiAddress, MultiSignature, OpaqueExtrinsic,
};
use sp_std::vec::Vec;

// the extrinsic version which we currently support & hence decodable from opaque bytes
const EXTRINSIC_FORMAT_VERSION: u8 = 4;

/// An index to a block.
pub type BlockNumber = u32;
/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;
/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;
/// Index of a transaction in the chain.
pub type Index = u32;
/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;
/// Digest item type.
pub type DigestItem = generic::DigestItem;
/// Time type
pub type Moment = u64;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, AccountIndex>;
/// Block header type as expected by this runtime.
pub type Header = DaHeader<BlockNumber, BlakeTwo256>;
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// DA Block type as expected by this runtime.
pub type Block = avail_core::DaBlock<Header, UncheckedExtrinsic>;
/// Block type for the node
pub type NodeBlock = generic::Block<Header, OpaqueExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// VRF Seed type.
pub type Seed = [u8; 32];

/// The SignedExtension to the basic transaction logic.
///
/// When you change this, you **MUST** modify [`sign`] in `bin/node/testing/src/keyring.rs`!
///
/// [`sign`]: <../../testing/src/keyring.rs.html>
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
	extensions::check_batch_transactions::CheckBatchTransactions<Runtime>,
);

/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
>;

/// ID type for named reserves.
pub type ReserveIdentifier = [u8; 8];

impl_opaque_keys! {
	pub struct SessionKeys {
		pub babe: crate::Babe,
		pub grandpa: crate::Grandpa,
		pub im_online: crate::ImOnline,
		pub authority_discovery: crate::AuthorityDiscovery,
	}
}

/// Return inner bytes (block-body wrapper is `Compact(len) || inner_bytes`).
fn opaque_inner_bytes(opaque: &OpaqueExtrinsic) -> Result<Vec<u8>, String> {
	let mut cursor: &[u8] = &opaque.encode();
	Vec::<u8>::decode(&mut cursor).map_err(|e| format!("opaque wrapper decode failed: {:?}", e))
}

/// Decode runtime `UncheckedExtrinsic` from `OpaqueExtrinsic` following the
/// `version_byte || [signature_tuple?] || RuntimeCall` layout (no vec prefix).
pub fn opaque_to_unchecked(opaque: &OpaqueExtrinsic) -> Result<UncheckedExtrinsic, String> {
	let inner = opaque_inner_bytes(opaque)?;
	if inner.is_empty() {
		return Err("opaque inner empty".into());
	}

	let first = inner[0];
	let is_signed = (first & 0x80) != 0;
	let version = first & 0x7f;
	if version != EXTRINSIC_FORMAT_VERSION {
		return Err(format!(
			"extrinsic version mismatch: expected {}, got {}",
			EXTRINSIC_FORMAT_VERSION, version
		));
	}

	// decode from bytes AFTER version byte (matches decode_no_vec_prefix)
	let mut input: &[u8] = &inner[1..];

	let signature = if is_signed {
		<(Address, Signature, SignedExtra)>::decode(&mut input)
			.map(Some)
			.map_err(|e| format!("signature tuple decode failed: {:?}", e))?
	} else {
		None
	};

	let function = RuntimeCall::decode(&mut input)
		.map_err(|e| format!("RuntimeCall decode failed: {:?}", e))?;

	Ok(UncheckedExtrinsic {
		signature,
		function,
	})
}

/// return caller if the xt is signed with MultiAddress::Id
pub fn unchecked_get_caller(xt: &UncheckedExtrinsic) -> Option<&sp_runtime::AccountId32> {
	xt.signature.as_ref().and_then(|(addr, _, _)| match addr {
		Address::Id(acc) => Some(acc),
		_ => None,
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use codec::Compact;
	use da_control::Call as DACall;
	use sp_core::crypto::AccountId32;
	use sp_runtime::generic::UncheckedExtrinsic as GenericUxt;
	use sp_runtime::BoundedVec;
	use sp_runtime::MultiSignature;
	use sp_runtime::OpaqueExtrinsic;
	use sp_std::convert::TryInto;

	#[test]
	fn roundtrip_show_inner_bytes_and_report() {
		// This test uses your runtime types and DA call to ensure the special decode path works.
		let v = [1u8; 8].to_vec();
		let data: BoundedVec<u8, _> = v.try_into().expect("bounded");

		// Build the UncheckedExtrinsic value (for comparison later).
		let uxt: UncheckedExtrinsic = UncheckedExtrinsic {
			signature: None,
			function: DACall::submit_data {
				app_id: avail_core::AppId(2),
				data,
			}
			.into(),
		};

		// Instead of relying on `uxt.encode()` to be the exact inner bytes layout,
		// compose the inner bytes manually: version_byte || (signature tuple if signed) || function bytes.
		let mut inner_bytes: Vec<u8> = Vec::new();

		// version byte: signed flag (0x80) | version (lower 7 bits). Unsigned -> no sign bit.
		inner_bytes.push(EXTRINSIC_FORMAT_VERSION);

		// signature: we have none (unsigned), so nothing to append here.
		// append the function encoding (RuntimeCall)
		inner_bytes.extend_from_slice(&uxt.function.encode());

		// Build the wrapper that block-body stores: Compact(len) || inner_bytes
		let mut wrapper: Vec<u8> = Vec::with_capacity(inner_bytes.len() + 8);
		Compact::<u32>(inner_bytes.len() as u32).encode_to(&mut wrapper);
		wrapper.extend_from_slice(&inner_bytes);

		// Create `OpaqueExtrinsic` from the wrapper bytes
		let opaque = OpaqueExtrinsic::from_bytes(wrapper.as_slice()).expect("opaque from bytes ok");

		// Diagnostic: show inner length and first byte
		let mut wrapper_input: &[u8] = &opaque.encode();
		let inner: Vec<u8> =
			Vec::<u8>::decode(&mut wrapper_input).expect("decode wrapper -> inner Vec<u8>");
		println!(
			"inner_len={}, first_byte={}",
			inner.len(),
			inner.get(0).copied().unwrap_or(0)
		);

		// Use your helper (which consumes the version byte and decodes function)
		let decoded: UncheckedExtrinsic =
			opaque_to_unchecked(&opaque).expect("decode must succeed");

		// Function (call) should match
		assert_eq!(decoded.function, uxt.function);
	}
}
