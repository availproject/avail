use da_control::CheckAppId;
use da_primitives::asdr::AppExtrinsic;
use frame_system::submitted_data::extrinsics_root;
use hex_literal::hex;
use pallet_transaction_payment::ChargeTransactionPayment;
use sp_core::{sr25519::Signature, H256};
use sp_runtime::{generic::Era, AccountId32, MultiAddress};
use test_case::test_case;

use super::*;

fn submit_call<A: Into<AppId>>(app_id: A) -> AppExtrinsic {
	let data = hex!("ed018400d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01be06880f2f6203365b508b4226fd697d3d79d3a50a5617aad714466d40ef47067225e823135b32121aa0f6f56e696f5f71107a6d44768c2fefe38cb209f7f28224000000041d014054657374207375626d69742064617461").to_vec();
	AppExtrinsic {
		app_id: app_id.into(),
		data,
	}
}
fn submit_call_expected() -> H256 {
	// hex!("ddf368647a902a6f6ab9f53b32245be28edc99e92f43f0004bbc2cb359814b2a").into()
	// hex!("9c6cf805b377632c6a224e1ca035f8f6975932529a5e492e73742e4f861ba89d").into()
	hex!("66dde8b32cbd3e6c3ae02f570a23202413d67870b15354c17cc12c4c49894c55").into()
}

#[test]
fn decode_submit_call() {
	let encoded_call = submit_call(1).data;

	let call = super::UncheckedExtrinsic::decode(&mut encoded_call.as_slice()).unwrap();

	let account = hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");
	let expected_signature = MultiSignature::Sr25519(Signature(hex!("be06880f2f6203365b508b4226fd697d3d79d3a50a5617aad714466d40ef47067225e823135b32121aa0f6f56e696f5f71107a6d44768c2fefe38cb209f7f282")));
	let expected_call = AppUncheckedExtrinsic {
		function: Call::DataAvailability(da_control::Call::submit_data {
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

#[test_case( submit_call(0) => submit_call_expected(); "Submit data 0")]
#[test_case( submit_call(1) => submit_call_expected(); "Submit data 1")]
fn data_root_filter(extrinsic: AppExtrinsic) -> H256 {
	extrinsics_root::<Runtime, _>(vec![extrinsic].into_iter())
}
