use super::*;
use crate::{Runtime, SignedExtra, UncheckedExtrinsic};

use avail_base::HeaderExtensionBuilderData;
use avail_core::data_proof::{BoundedData, Message, TxDataRoots};
use da_control::{AppDataFor, Call as DaCall, CheckAppId};
use frame_system::{
	CheckEra, CheckGenesis, CheckNonZeroSender, CheckNonce, CheckSpecVersion, CheckTxVersion,
	CheckWeight,
};
use pallet_vector::Call as VectorCall;

use codec::Encode;
use hex_literal::hex;
use pallet_transaction_payment::ChargeTransactionPayment;
use sp_core::H256;
use sp_keyring::AccountKeyring::Alice;
use sp_runtime::{
	generic::Era,
	traits::{SignedExtension, Verify as _},
	MultiSignature,
};
use test_case::test_case;

fn extra() -> SignedExtra {
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
	)
}
fn additional_signed() -> <SignedExtra as SignedExtension>::AdditionalSigned {
	let spec_ver = VERSION.spec_version;
	let tx_ver = VERSION.transaction_version;
	let genesis = H256::default();
	let era = H256::repeat_byte(1);

	((), spec_ver, tx_ver, genesis, era, (), (), (), ())
}

fn signed_extrinsic(function: RuntimeCall) -> Vec<u8> {
	let extra = extra();
	let additional = additional_signed();
	let alice = Alice.to_account_id();

	let payload = SignedPayload::from_raw(function.clone(), extra.clone(), additional).encode();
	let signature: MultiSignature = Alice.sign(&payload).into();

	assert!(signature.verify(&*payload, &alice));
	UncheckedExtrinsic::new_signed(function, alice.into(), signature, extra).encode()
}

fn submit_data(data: Vec<u8>) -> Vec<u8> {
	let data = AppDataFor::<Runtime>::truncate_from(data);
	let function = DaCall::submit_data { data }.into();

	signed_extrinsic(function)
}

fn bridge_msg(data: Vec<u8>) -> Vec<u8> {
	let message = Message::ArbitraryMessage(BoundedData::truncate_from(data));
	let to = H256::repeat_byte(0x01);
	let function = VectorCall::send_message {
		message,
		to,
		domain: 2,
	}
	.into();

	signed_extrinsic(function)
}

fn bridge_fungible_msg(asset_id: H256, amount: u128) -> Vec<u8> {
	let message = Message::FungibleToken { asset_id, amount };
	let to = H256::repeat_byte(0x01);
	let function = VectorCall::send_message {
		message,
		to,
		domain: 2,
	}
	.into();

	signed_extrinsic(function)
}

fn empty_root() -> H256 {
	let root = TxDataRoots::new(H256::zero(), H256::zero()).data_root;
	let exp_root = hex!("ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5");
	assert_eq!(root.0, exp_root);
	root
}

#[test_case(&[submit_data(hex!("abcd").to_vec())] => H256(hex!("f1f399f7e0d8c8ed712df0c21b4ec78f3b8533f1c3d0215e4023e1b7c80bfd91")); "submitted")]
#[test_case(&[submit_data(vec![])] => empty_root(); "empty submitted")]
#[test_case(&[] => empty_root(); "empty submitted 2")]
#[test_case(&[bridge_msg(hex!("47").to_vec())] => H256(hex!("df93f65f9f5adf3ac0d46e5a08432b96ef362bf229e1737939051884c5506e02")); "bridged data")]
#[test_case(&[bridge_fungible_msg(H256::repeat_byte(1), 1_000_000)] => H256(hex!("e93394eeaedb2158a154a29b9333fe06451fbe82c9cff5b961a6d701782450bc")) ; "bridged fungible")]
#[test_case(&[submit_data(hex!("abcd").to_vec()), bridge_msg(hex!("47").to_vec())] => H256(hex!("c925bfccfc86f15523c5b40b2bd6d8a66fc51f3d41176d77be7928cb9e3831a7")); "submitted and bridged")]
fn data_root_filter(extrinsics: &[Vec<u8>]) -> H256 {
	HeaderExtensionBuilderData::from_raw_extrinsics::<Runtime>(0, extrinsics).data_root()
}
