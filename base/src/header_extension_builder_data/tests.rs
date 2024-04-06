use super::*;
use crate::{calls_proof, CallsProof};
use avail_core::AppId;

use avail_core::data_proof::{AddressedMessage, Message, SubTrie};
use binary_merkle_tree::MerkleProof;
use codec::{Decode, Encode};
use frame_support::traits::{DefensiveTruncateFrom, ExtrinsicCall};
use hex_literal::hex;
use sp_core::{keccak_256, H256};
use sp_runtime::{traits::Extrinsic, AccountId32, BoundedVec};
use std::vec;

const ACC: AccountId32 = AccountId32::new([0u8; 32]);

#[derive(Encode, Decode)]
struct TExt {
	caller: AccountId32,
	call: String,
	app_id: AppId,
}

impl TExt {
	pub fn new(caller: AccountId32, call: String) -> Self {
		let app_id = AppId::default();
		Self {
			caller,
			call,
			app_id,
		}
	}
}

impl Extrinsic for TExt {
	type Call = String;
	type SignaturePayload = ();

	// Provided methods
	fn is_signed(&self) -> Option<bool> {
		Some(false)
	}
	fn new(_call: Self::Call, _signed_data: Option<Self::SignaturePayload>) -> Option<Self> {
		None
	}
}

impl ExtrinsicCall for TExt {
	fn call(&self) -> &Self::Call {
		&self.call
	}
}

impl MaybeCaller<AccountId32> for TExt {
	fn caller(&self) -> Option<&AccountId32> {
		Some(&self.caller)
	}
}

impl GetAppId for TExt {}

// dummy filter implementation that skips empty strings in vector
impl TxDataFilter<AccountId32, String> for String {
	fn filter<'a, 'b>(
		_: Option<&AccountId32>,
		s: &String,
		app: AppId,
		_: u32,
		tx_idx: usize,
		_: &mut Metrics,
	) -> Option<HeaderExtensionBuilderData> {
		if s.is_empty() {
			return None;
		}

		let tx_idx = u32::try_from(tx_idx).ok()?;
		let data = SubmittedData::new(app, tx_idx, s.as_bytes().to_vec());
		Some(data.into())
	}
}

fn calls_proof_adaptor<I>(calls: I, tx_index: u32) -> Option<(MerkleProof<H256, Vec<u8>>, H256)>
where
	I: IntoIterator<Item = (AccountId32, String)>,
{
	// Create calls with Tx Index: `(acc, tx_idx, data)`
	let extrinsics = calls
		.into_iter()
		.enumerate()
		.filter_map(|(idx, (acc, s))| (!s.is_empty()).then(|| (idx, TExt::new(acc, s).encode())))
		.collect::<Vec<(usize, Vec<_>)>>();

	// Map `tx_index` to leaf index
	let leaf_idx = extrinsics
		.iter()
		.position(|(idx, _)| *idx == tx_index as usize)?;

	let CallsProof {
		proof,
		root,
		message: _,
	} = calls_proof::<String, TExt, _, _>(
		0,
		extrinsics.iter().map(|(_, e)| e),
		leaf_idx,
		SubTrie::DataSubmit,
	)?;

	Some((proof, root))
}

#[test]
fn test_left_data_proof_with_one_tx() {
	let exp_da_root = hex!("40105d5bc10105c17fd72b93a8f73369e2ee6eee4d4714b7bf7bf3c2f156e601");
	let calls = vec![(ACC, "0".to_string())];
	// leaf 0 keccak256(044852b2a670ade5407e78fb2863c51de9fcb96542a07186fe3aeda6bb8a116d)
	//                  40105d5bc10105c17fd72b93a8f73369e2ee6eee4d4714b7bf7bf3c2f156e601
	assert_eq!(
		keccak_256("0".as_bytes()),
		hex!("044852b2a670ade5407e78fb2863c51de9fcb96542a07186fe3aeda6bb8a116d")
	);
	assert_eq!(
		keccak_256(&keccak_256("0".as_bytes())),
		hex!("40105d5bc10105c17fd72b93a8f73369e2ee6eee4d4714b7bf7bf3c2f156e601")
	);

	let (da_proof, root) =
		calls_proof_adaptor(calls, 0).expect("Proof not generated for the transaction index 0!");

	assert_eq!(root, H256::zero());
	assert_eq!(da_proof.leaf_index, 0);
	assert_eq!(H256(exp_da_root), da_proof.root);
	assert_eq!(da_proof.proof.len(), 0);
	assert_eq!(da_proof.number_of_leaves, 1);
}

#[test]
fn test_left_data_proof_with_two_tx() {
	let exp_proof_root = hex!("db0ccc7a2d6559682303cc9322d4b79a7ad619f0c87d5f94723a33015550a64e");
	let exp_proof_0 = hex!("4aeff0db81e3146828378be230d377356e57b6d599286b4b517dbf8941b3e1b2");
	let calls = vec![(ACC, "0".to_string()), (ACC, "1".to_string())];
	// leaf 0 keccak256(044852b2a670ade5407e78fb2863c51de9fcb96542a07186fe3aeda6bb8a116d)
	//                  40105d5bc10105c17fd72b93a8f73369e2ee6eee4d4714b7bf7bf3c2f156e601
	// leaf 1 keccak256(c89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc6)
	//                  4aeff0db81e3146828378be230d377356e57b6d599286b4b517dbf8941b3e1b2

	let (da_proof, root) =
		calls_proof_adaptor(calls, 0).expect("Proof not generated for the transaction index 0!");

	assert_eq!(root, H256::zero());
	assert_eq!(da_proof.leaf_index, 0);
	assert_eq!(H256(exp_proof_root), da_proof.root);
	assert_eq!(da_proof.proof.len(), 1);
	assert_eq!(da_proof.number_of_leaves, 2);
	assert_eq!(H256(exp_proof_0), da_proof.proof[0]);
}

#[test]
fn test_left_data_proof_with_skipped_tx() {
	let calls = ["0", "", "1", "2"]
		.iter()
		.map(|str| (ACC, str.to_string()))
		.collect::<Vec<_>>();

	// leaf 0 keccak256(044852b2a670ade5407e78fb2863c51de9fcb96542a07186fe3aeda6bb8a116d)
	//                  40105d5bc10105c17fd72b93a8f73369e2ee6eee4d4714b7bf7bf3c2f156e601
	// leaf 1 keccak256(c89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc6)
	//                  4aeff0db81e3146828378be230d377356e57b6d599286b4b517dbf8941b3e1b2
	// leaf 2 keccak256(ad7c5bef027816a800da1736444fb58a807ef4c9603b7848673f7e3a68eb14a5)
	//                  1204b3dcd975ba0a68eafbf4d2ca0d13cc7b5e3709749c1dc36e6e74934270b3
	//  leaf appended in in order to have 2^n number of leaves
	// leaf 3 (appended) keccak256(0000000000000000000000000000000000000000000000000000000000000000)
	//                  290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563

	// intermediate root (leaf[0], leaf[1]) db0ccc7a2d6559682303cc9322d4b79a7ad619f0c87d5f94723a33015550a64e
	// intermediate root (leaf[2], leaf[3]) 3c86bde3a90d18efbcf23e27e9b6714012aa055263fe903a72333aa9caa37f1b
	// data_root keccak256(db0ccc7a2d6559682303cc9322d4b79a7ad619f0c87d5f94723a33015550a64e, 3c86bde3a90d18efbcf23e27e9b6714012aa055263fe903a72333aa9caa37f1b)
	//                                                       (877f9ed6aa67f160e9b9b7794bb851998d15b65d11bab3efc6ff444339a3d750)

	let (da_proof, root) = calls_proof_adaptor(calls.clone(), 0)
		.expect("Proof not generated for the transaction index 0!");

	let exp_proof_root = hex!("877f9ed6aa67f160e9b9b7794bb851998d15b65d11bab3efc6ff444339a3d750");
	let exp_proofs_0: [H256; 2] = [
		hex!("4aeff0db81e3146828378be230d377356e57b6d599286b4b517dbf8941b3e1b2").into(),
		hex!("3c86bde3a90d18efbcf23e27e9b6714012aa055263fe903a72333aa9caa37f1b").into(),
	];
	let exp_leaf_0 = hex!("044852b2a670ade5407e78fb2863c51de9fcb96542a07186fe3aeda6bb8a116d");
	assert_eq!(root, H256::zero());
	assert_eq!(da_proof.leaf_index, 0);
	assert_eq!(H256(exp_proof_root), da_proof.root);
	assert_eq!(da_proof.proof.len(), 2);
	assert_eq!(exp_proofs_0, da_proof.proof.as_slice());
	assert_eq!(exp_leaf_0, da_proof.leaf.as_slice());
	assert_eq!(da_proof.number_of_leaves, 4);

	// proof should not be generated when there is not data
	assert_eq!(None, calls_proof_adaptor(calls.clone(), 1));

	let (da_proof, root) = calls_proof_adaptor(calls.clone(), 2)
		.expect("Proof not generated for the transaction index 2!");
	let exp_proof_2: [H256; 2] = [
		hex!("40105d5bc10105c17fd72b93a8f73369e2ee6eee4d4714b7bf7bf3c2f156e601").into(),
		hex!("3c86bde3a90d18efbcf23e27e9b6714012aa055263fe903a72333aa9caa37f1b").into(),
	];
	assert_eq!(root, H256::zero());
	assert_eq!(da_proof.leaf_index, 1);
	assert_eq!(H256(exp_proof_root), da_proof.root);
	assert_eq!(da_proof.proof.len(), 2);
	assert_eq!(exp_proof_2, da_proof.proof.as_slice());
	assert_eq!(da_proof.number_of_leaves, 4);

	let (da_proof, root) = calls_proof_adaptor(calls.clone(), 3)
		.expect("Proof not generated for the transaction index 3!");

	assert_eq!(root, H256::zero());
	assert_eq!(da_proof.leaf_index, 2);

	assert_eq!(
		da_proof.root,
		hex!("877f9ed6aa67f160e9b9b7794bb851998d15b65d11bab3efc6ff444339a3d750").into()
	);

	assert_eq!(da_proof.proof.len(), 2);
	assert_eq!(
		da_proof.proof,
		[
			hex!("290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563").into(),
			hex!("db0ccc7a2d6559682303cc9322d4b79a7ad619f0c87d5f94723a33015550a64e").into()
		]
	);
	assert_eq!(da_proof.number_of_leaves, 4);

	// submit index that does not exists and proof should not be generated
	assert_eq!(None, calls_proof_adaptor(calls, 15));
}

#[test]
fn test_message_encoding() {
	let expected_encoded_message = hex!("00000000000000000000000000000000000000000000000000000000000000200200000000000000000000000000000000000000000000000000000000000000681257bed628425a28b469114dc21a7c30205cfd00000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000de0b6b3a7640000").to_vec();

	// Message(0x02, bytes32(bytes20(0x681257BED628425a28B469114Dc21A7c30205cFD)), bytes32(uint256(1)), 2, 1, abi.encode(bytes32(0), 1 ether), 0);
	let asset_id = H256::zero();
	let amount = 1_000_000_000_000_000_000u128;
	let message = AddressedMessage {
		message: Message::FungibleToken { asset_id, amount },
		from: hex!("681257BED628425a28B469114Dc21A7c30205cFD000000000000000000000000").into(),
		to: hex!("0000000000000000000000000000000000000000000000000000000000000001").into(),
		origin_domain: 2,
		destination_domain: 1,
		id: 0,
	};

	let abi_encoded = message.abi_encode();
	assert_eq!(expected_encoded_message, abi_encoded);
}

#[test]
fn test_message_encoding_from_avail_with_hash() {
	// Message is : Message {
	// message_type: FungibleToken,
	// from: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d,
	// to: 0x0000000000000000000000000000000000000000000000000000000000000001,
	// origin_domain: 1,
	// destination_domain: 2,
	// data: BoundedVec([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 102400),
	// id: 1 }

	let asset_id = H256::zero();
	let amount = 1u128;
	let message = AddressedMessage {
		message: Message::FungibleToken { asset_id, amount },
		from: hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").into(),
		to: hex!("0000000000000000000000000000000000000000000000000000000000000001").into(),
		origin_domain: 1,
		destination_domain: 2,
		id: 1,
	};

	let encoded = message.abi_encode();
	let leaf_hash = hex!("ccd6cb2b400270449e283f0f9e4fdf1dbfeb44fa5d86468272d6834d2be7574f");
	assert_eq!(leaf_hash, keccak_256(encoded.as_slice()));
}

#[test]
fn test_message_encoding_from_avail_with_hash1() {
	let asset_id = H256::zero();
	let amount = 1_000_000_000_000_000_000u128;
	let message = AddressedMessage {
		message: Message::FungibleToken { asset_id, amount },
		from: hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").into(),
		to: hex!("0000000000000000000000000000000000000000000000000000000000000001").into(),
		origin_domain: 1,
		destination_domain: 2,
		id: 1,
	};

	let encoded = message.abi_encode();
	let leaf_hash = hex!("94491650baa28a6f0db3c5e9495e12e43b7f1b2726fa5c5dabed2619514bd7b5");
	assert_eq!(leaf_hash, keccak_256(encoded.as_slice()));
}

#[test]
fn test_amb_message_encoding() {
	let expected_encoding = hex!("000000000000000000000000000000000000000000000000000000000000002001000000000000000000000000000000000000000000000000000000000000008f8d47bf15953e26c622f36f3366e43e26b9b78b000000000000000000000000c437b127628aa7984af0f001dc7ac023eee266f0df6356ef9243f340af8842360000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000e0000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000002b67");
	let expected_hash = hex!("ad79a34ee43ea39301b1f190558ea279122328ebd66b342a49a131ee5befd3b5");

	let data = hex!("0000000000000000000000000000000000000000000000000000000000002b67");
	let data = BoundedVec::defensive_truncate_from(data.to_vec());

	let message = AddressedMessage {
		message: Message::ArbitraryMessage(data),
		from: H256(hex!(
			"8f8d47bf15953e26c622f36f3366e43e26b9b78b000000000000000000000000"
		)),
		to: H256(hex!(
			"c437b127628aa7984af0f001dc7ac023eee266f0df6356ef9243f340af884236"
		)),
		origin_domain: 2,
		destination_domain: 1,
		id: 1,
	};
	let encoded_message = message.abi_encode();
	assert_eq!(&expected_encoding, encoded_message.as_slice());
	assert_eq!(expected_hash, keccak_256(encoded_message.as_slice()));
}

#[test]
fn test_amb_message_encoding_with_hash_check() {
	let expected_encoding = hex!("00000000000000000000000000000000000000000000000000000000000000200100000000000000000000000000000000000000000000000000000000000000681257bed628425a28b469114dc21a7c30205cfd0000000000000000000000003547517355657647456b6f7847444a5044576251694b4478714b6d675a3570470000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000d48656c6c6f2c20576f726c642100000000000000000000000000000000000000");
	let expected_hash = hex!("5774ba3f9618e2da3885b0e2853e4005c3e836625e8be0f69bf3d93f51fac58d");
	let data = BoundedVec::defensive_truncate_from("Hello, World!".as_bytes().to_vec());

	let message = AddressedMessage {
		message: Message::ArbitraryMessage(data),
		from: hex!("681257BED628425a28B469114Dc21A7c30205cFD000000000000000000000000").into(),
		to: hex!("3547517355657647456b6f7847444a5044576251694b4478714b6d675a357047").into(),
		origin_domain: 2,
		destination_domain: 1,
		id: 0,
	};

	let encoded_message = message.abi_encode();
	assert_eq!(&expected_encoding, encoded_message.as_slice());
	assert_eq!(expected_hash, keccak_256(encoded_message.as_slice()));
}
