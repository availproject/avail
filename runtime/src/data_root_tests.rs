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
use sp_core::bytes::to_hex;
use sp_core::{sr25519::Signature, H256};
use sp_io::hashing::keccak_256;
use sp_runtime::{generic::Era, AccountId32, MultiAddress};
use test_case::test_case;

use super::*;

fn submit_call() -> Vec<u8> {
	hex!("ed018400d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01be06880f2f6203365b508b4226fd697d3d79d3a50a5617aad714466d40ef47067225e823135b32121aa0f6f56e696f5f71107a6d44768c2fefe38cb209f7f28224000000041d014054657374207375626d69742064617461").to_vec()
}

fn send_message() -> Vec<u8> {
	hex!("fd028400d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01386fbc5afe4337f34e0684eccff3cb3be2c510f593f46bc31242df817b9f213336076016c20c602cc6b831c320d79e854e8756c6aaf21dc34a14cd3b78fd1c86e401080000270501000000000000000000000000000000000000000000000000000000000000000108010100000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000").to_vec()
}

fn submit_call_expected() -> H256 {
	// data = "Test submit data"
	// leaf is keccak256(data) -> root
	let blob_root = hex!("db45128913020d152dbee4d00a1dffebdb703425c44adbd7d7dfc7ae93d836bc");

	let mut concat = vec![];
	// bridge_root = 0x0..0
	// keccak_256(blob_root, bridge_root)
	concat.extend_from_slice(blob_root.as_slice());
	concat.extend_from_slice(H256::zero().as_bytes());
	H256(keccak_256(concat.as_slice()))
}

fn send_message_expected() -> H256 {
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

	println!(
		"{:?}",
		to_hex(message.clone().abi_encode().as_slice(), false)
	);

	let encoded = message.abi_encode();
	let leaf = keccak_256(encoded.as_slice());
	let expected_bridge_root = keccak_256(leaf.as_slice());
	println!("expected_bridge_root {:?}", H256(expected_bridge_root));
	let mut concat = vec![];
	concat.extend_from_slice(H256::zero().as_bytes());
	concat.extend_from_slice(expected_bridge_root.as_slice());
	H256(keccak_256(concat.as_slice()))
}

#[test]
fn decode_submit_call() {
	let encoded_call = submit_call();

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

#[test_case([submit_call()].into() => submit_call_expected(); "Test submit data")]
#[test_case([send_message()].into() => send_message_expected(); "Test send message")]
fn data_root_filter(extrinsics: Vec<Vec<u8>>) -> H256 {
	let mut opaque = vec![];

	for extrinsic in extrinsics {
		let o = OpaqueExtrinsic::decode(&mut extrinsic.as_slice()).unwrap();
		opaque.push(o)
	}
	extrinsics_root_v2::<Runtime, _>(opaque.iter(), 0u64).0
}
