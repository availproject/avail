use da_control::{Call as DaCall, CheckAppId};
use frame_system::{
	submitted_data::extrinsics_root, CheckEra, CheckGenesis, CheckNonZeroSender, CheckNonce,
	CheckSpecVersion, CheckTxVersion, CheckWeight,
};
use hex_literal::hex;
use pallet_transaction_payment::ChargeTransactionPayment;
use sp_core::{sr25519::Signature, H256};
use sp_runtime::{generic::Era, AccountId32, MultiAddress};
use test_case::test_case;

use super::*;

fn submit_call() -> Vec<u8> {
	hex!("ed018400d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01be06880f2f6203365b508b4226fd697d3d79d3a50a5617aad714466d40ef47067225e823135b32121aa0f6f56e696f5f71107a6d44768c2fefe38cb209f7f28224000000041d014054657374207375626d69742064617461").to_vec()
}
fn submit_call_expected() -> H256 {
	// hex!("ddf368647a902a6f6ab9f53b32245be28edc99e92f43f0004bbc2cb359814b2a").into()
	// hex!("9c6cf805b377632c6a224e1ca035f8f6975932529a5e492e73742e4f861ba89d").into()
	hex!("db45128913020d152dbee4d00a1dffebdb703425c44adbd7d7dfc7ae93d836bc").into()
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
				CheckAppId::<Runtime>::from(1.into()),
			),
		)),
	};

	if let Some(ref signature) = call.signature {
		assert_eq!(signature.1, expected_signature);
	}
	assert_eq!(call, expected_call);
}

#[test_case( submit_call() => submit_call_expected(); "Submit data 0")]
fn data_root_filter(extrinsic: Vec<u8>) -> H256 {
	let opaque = OpaqueExtrinsic::decode(&mut extrinsic.as_slice()).unwrap();
	extrinsics_root::<Runtime, _>([opaque].iter())
}
