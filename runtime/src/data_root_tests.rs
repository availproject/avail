use avail_core::asdr::AppUncheckedExtrinsic;
use avail_core::OpaqueExtrinsic;
use codec::Decode;
use da_control::{Call as DaCall, CheckAppId};
use frame_election_provider_support::BoundedVec;
use frame_support::traits::DefensiveTruncateFrom;
use frame_system::submitted_data::{Message, MessageType};
use frame_system::{
	submitted_data::extrinsics_root_v2, CheckEra, CheckGenesis, CheckNonZeroSender, CheckNonce,
	CheckSpecVersion, CheckTxVersion, CheckWeight,
};
use hex_literal::hex;
use pallet_transaction_payment::ChargeTransactionPayment;
use sp_core::{sr25519::Signature, H256};
use sp_io::hashing::keccak_256;
use sp_runtime::{generic::Era, AccountId32, MultiAddress};
use test_case::test_case;

use super::*;

fn submit_blob_call() -> Vec<u8> {
	hex!("ed018400d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01be06880f2f6203365b508b4226fd697d3d79d3a50a5617aad714466d40ef47067225e823135b32121aa0f6f56e696f5f71107a6d44768c2fefe38cb209f7f28224000000041d014054657374207375626d69742064617461").to_vec()
}

fn send_message() -> Vec<u8> {
	hex!("fd028400d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0178ece77c33dbecbf1eff04371af093ab55f7a7b102b0d38760f1506468e78556761e1a1fc9ca57dcfad69f551306fc86d16191a094d9d1ce61ee70aa421339884400040000270301000000000000000000000000000000000000000000000000000000000000000108010100000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000").to_vec()
}

fn expected_blob_root() -> H256 {
	// data = "Test submit data"
	// leaf is keccak256(data) -> keccak256(root)
	let blob_root = keccak_256(
		hex!("db45128913020d152dbee4d00a1dffebdb703425c44adbd7d7dfc7ae93d836bc").as_slice(),
	);

	H256(blob_root)
}

fn submit_blob_call_expected() -> H256 {
	let mut concat = vec![];
	// bridge_root = 0x0..0
	// keccak_256(blob_root, bridge_root)
	concat.extend_from_slice(expected_blob_root().as_bytes());
	concat.extend_from_slice(H256::zero().as_bytes());
	H256(keccak_256(concat.as_slice()))
}

fn expected_bridge_root() -> H256 {
	let data = hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001");
	let encoded_data = BoundedVec::defensive_truncate_from(data.to_vec());

	let message = Message {
		message_type: MessageType::FungibleToken,
		from: H256(hex!(
			"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
		)),
		to: H256(hex!(
			"0000000000000000000000000000000000000000000000000000000000000001"
		)),
		origin_domain: 1,
		destination_domain: 2,
		data: encoded_data,
		id: 1,
	};

	let encoded = message.abi_encode();
	let leaf = keccak_256(encoded.as_slice());
	let expected_bridge_root = leaf.as_slice();

	H256::from_slice(expected_bridge_root)
}

fn send_message_expected() -> H256 {
	let mut concat = vec![];
	concat.extend_from_slice(H256::zero().as_bytes());
	concat.extend_from_slice(expected_bridge_root().as_bytes());

	H256(keccak_256(concat.as_slice()))
}

fn expect_sending_blob_and_bridge_extrinsic() -> H256 {
	let mut concat = vec![];
	concat.extend_from_slice(expected_blob_root().as_bytes());
	concat.extend_from_slice(expected_bridge_root().as_bytes());

	H256(keccak_256(concat.as_slice()))
}

#[test]
fn decode_submit_blob_call() {
	let encoded_call = submit_blob_call();

	let call = super::UncheckedExtrinsic::decode(&mut encoded_call.as_slice()).unwrap();

	let account = hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");
	let expected_signature = sp_runtime::MultiSignature::Sr25519(Signature(hex!("be06880f2f6203365b508b4226fd697d3d79d3a50a5617aad714466d40ef47067225e823135b32121aa0f6f56e696f5f71107a6d44768c2fefe38cb209f7f282")));
	let expected_call = AppUncheckedExtrinsic {
		function: RuntimeCall::DataAvailability(DaCall::submit_data {
			data: hex!("54657374207375626d69742064617461")
				.to_vec()
				.try_into()
				.unwrap(),
		}),

		// signature: Option<(Address, Signature, Extra)>,
		signature: Some((
			MultiAddress::Id(AccountId32::new(account)),
			expected_signature.clone(),
			// super::SignedExtra::default()
			(
				CheckNonZeroSender::<Runtime>::new(),
				CheckSpecVersion::<Runtime>::new(),
				CheckTxVersion::<Runtime>::new(),
				CheckGenesis::<Runtime>::new(),
				CheckEra::<Runtime>::from(Era::Mortal(32, 2)),
				CheckNonce::<Runtime>::from(0),
				CheckWeight::<Runtime>::new(),
				ChargeTransactionPayment::<Runtime>::from(0),
				CheckAppId::<Runtime>::from(AppId(1)),
			),
		)),
	};

	if let Some(ref signature) = call.signature {
		assert_eq!(signature.1, expected_signature);
	}
	assert_eq!(call, expected_call);
}

#[test_case([submit_blob_call()].into() => submit_blob_call_expected(); "Test submit blob extrinsic")]
#[test_case([send_message()].into() => send_message_expected(); "Test submit bridge extrinsic")]
#[test_case([send_message(), submit_blob_call()].into() => expect_sending_blob_and_bridge_extrinsic(); "Test send message and bridge extrinsic")]
fn data_root_filter(extrinsics: Vec<Vec<u8>>) -> H256 {
	let mut opaque = vec![];

	for extrinsic in extrinsics {
		let o = OpaqueExtrinsic::decode(&mut extrinsic.as_slice()).unwrap();
		opaque.push(o)
	}

	extrinsics_root_v2::<Runtime, _>(opaque.iter(), 0u64).0
}
