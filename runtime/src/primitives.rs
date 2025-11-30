use crate::{extensions, AllPalletsWithSystem, Runtime, RuntimeCall};
use avail_core::header::Header as DaHeader;
use codec::{Decode, Encode};
use scale_info::prelude::{format, string::String};

use sp_runtime::{
	generic::{self, Preamble},
	impl_opaque_keys,
	traits::{BlakeTwo256, IdentifyAccount, Verify},
	MultiAddress, MultiSignature, OpaqueExtrinsic,
};
use sp_std::vec::Vec;

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
	sp_runtime::generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
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


/// Decode runtime `UncheckedExtrinsic` from `OpaqueExtrinsic` following the
/// `Compact(len) || inner_bytes` layout.
pub fn opaque_to_unchecked(opaque: &OpaqueExtrinsic) -> Result<UncheckedExtrinsic, String> {
	// 1) Decode the wrapper: Compact<u32>(len) + inner bytes.
	let inner = opaque.encode();

	if inner.is_empty() {
		return Err("opaque inner extrinsic bytes empty".into());
	}

	// 2) Decode the runtime UncheckedExtrinsic from the inner bytes.
	let mut inner_input: &[u8] = &inner;
	UncheckedExtrinsic::decode(&mut inner_input)
		.map_err(|e| format!("failed to decode UncheckedExtrinsic: {e:?}"))
}

/// return caller if the xt is signed with MultiAddress::Id
pub fn unchecked_get_caller(xt: &UncheckedExtrinsic) -> Option<AccountId> {
	match &xt.preamble {
		Preamble::Signed(address, _signature, _) => match address {
			Address::Id(account) => Some(account.clone()),
			_ => None,
		},
		_ => None,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::extensions::check_batch_transactions::CheckBatchTransactions;
	use crate::primitives::generic::Era;
	use codec::Encode;
	use da_control::Call as DACall;
	use frame_system::{
		CheckEra, CheckGenesis, CheckNonZeroSender, CheckNonce, CheckSpecVersion, CheckTxVersion,
		CheckWeight,
	};
	use pallet_transaction_payment::ChargeTransactionPayment;
	use sp_io::TestExternalities;
	use sp_keyring::Sr25519Keyring::Alice;
	use sp_runtime::BoundedVec;
	use sp_runtime::BuildStorage;
	use sp_runtime::OpaqueExtrinsic;
	use sp_std::convert::TryInto;

	fn extra() -> SignedExtra {
		(
			CheckNonZeroSender::<Runtime>::new(),
			CheckSpecVersion::<Runtime>::new(),
			CheckTxVersion::<Runtime>::new(),
			CheckGenesis::<Runtime>::new(),
			// For tests we don't care about mortality; Immortal keeps Era checks simple.
			CheckEra::<Runtime>::from(Era::Immortal),
			CheckNonce::<Runtime>::from(0),
			CheckWeight::<Runtime>::new(),
			ChargeTransactionPayment::<Runtime>::from(0),
			CheckBatchTransactions::<Runtime>::new(),
		)
	}

	#[test]
	fn roundtrip_unsigned_ext() {
		// This test uses your runtime types and DA call to ensure the decode path works.
		let v = [1u8; 8].to_vec();
		let data: BoundedVec<u8, _> = v.try_into().expect("bounded");

		// Build the call.
		let call: RuntimeCall = DACall::submit_data {
			app_id: avail_core::AppId(2),
			data,
		}
		.into();

		// Build the UncheckedExtrinsic as unsigned using the proper constructor.
		let uxt: UncheckedExtrinsic = UncheckedExtrinsic::new_unsigned(call.clone());

		// Raw bytes: SCALE encoding of UncheckedExtrinsic (no extra wrapper).
		let raw_bytes: Vec<u8> = uxt.encode();

		// OpaqueExtrinsic is assumed to hold exactly these raw bytes.
		let opaque =
			OpaqueExtrinsic::from_bytes(raw_bytes.as_slice()).expect("opaque from bytes ok");

		let decoded: UncheckedExtrinsic =
			opaque_to_unchecked(&opaque).expect("decode must succeed");

		// Function (call) should match.
		assert_eq!(decoded.function, call);
	}

	#[test]
	fn roundtrip_signed_ext() {
		// Minimal storage / externalities for the Runtime so `SignedPayload::new`
		// can run the TransactionExtensions (CheckGenesis, etc).
		let storage = <frame_system::GenesisConfig<Runtime> as BuildStorage>::build_storage(
			&frame_system::GenesisConfig::default(),
		)
		.expect("frame_system genesis storage builds");
		let mut ext = TestExternalities::new(storage);

		ext.execute_with(|| {
			let v = [1u8; 8].to_vec();
			let data: BoundedVec<u8, _> = v.try_into().expect("bounded");
			let extra = extra();
			let alice = Alice.to_account_id();

			// Build the call.
			let call: RuntimeCall = DACall::submit_data {
				app_id: avail_core::AppId(2),
				data,
			}
			.into();

			// NOTE: requires externalities (CheckGenesis, etc.).
			let payload = SignedPayload::new(call.clone(), extra.clone())
				.expect("SignedPayload::new should work with ext")
				.encode();

			let signature: MultiSignature = Alice.sign(&payload).into();
			assert!(signature.verify(&*payload, &alice));

			let uxt: UncheckedExtrinsic =
				UncheckedExtrinsic::new_signed(call.clone(), alice.clone().into(), signature, extra);

			// Raw bytes: SCALE encoding of UncheckedExtrinsic.
			let raw_bytes: Vec<u8> = uxt.encode();

			// OpaqueExtrinsic holds exactly these bytes.
			let opaque =
				OpaqueExtrinsic::from_bytes(raw_bytes.as_slice()).expect("opaque from bytes ok");

			let decoded: UncheckedExtrinsic =
				opaque_to_unchecked(&opaque).expect("decode must succeed");

			// Function (call) should match.
			assert_eq!(decoded.function, call);

			// And caller should be recoverable as Alice.
			let caller = unchecked_get_caller(&decoded).expect("must be signed with Id");
			assert_eq!(caller, alice);
		});
	}
}
