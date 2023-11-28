use ark_std::iterable::Iterable;
use patricia_merkle_trie::{EIP1186Layout, StorageProof};
use primitive_types::{H160, H256};
use rlp::Rlp;
use scale_info::prelude::vec::Vec;
use sp_io::hashing::keccak_256;
use trie_db::{DBValue, Trie, TrieDBBuilder};

use crate::Config;

// struct Message {
//     uint8 version;
//     uint64 nonce;
//     uint32 sourceChainId;
//     address sourceAddress;
//     uint32 destinationChainId;
//     bytes32 destinationAddress;
//     bytes data;
// }

// #[derive(Debug, PartialEq, RlpDecodable)]
// pub struct Account {
//     pub nonce: U256,
//     pub balance: U256,
//     pub storage_root: H256,
//     pub code_hash: H256,
// }

#[derive(Debug)]
pub struct Message {
	pub version: u8,
	pub nonce: u64,
	pub source_chain_id: u32,
	pub source_address: H160,
	pub destination_chain_id: u32,
	pub destination_address: H256,
	// arbitrary data that we want to pass
	pub data: Vec<u8>,
}

pub fn decode_message(message: Vec<u8>) -> Message {
	let version: u8;
	let nonce: u64;
	let source_chain_id: u32;
	let destination_chain_id: u32;
	let source_address: H160;
	let destination_address: H256;

	version = message[0];
	let mut buf = [0u8; 8];
	buf[..8].copy_from_slice(&message[1..9]);
	nonce = u64::from_be_bytes(buf);

	let mut buf_source_chain = [0u8; 4];
	buf_source_chain[..4].copy_from_slice(&message[9..13]);
	source_chain_id = u32::from_be_bytes(buf_source_chain);

	let mut buf_source_address = [0u8; 20];
	buf_source_address[..20].copy_from_slice(&message[13..33]);
	source_address = H160(buf_source_address);

	let mut buf_dest_chain = [0u8; 4];
	buf_dest_chain[..4].copy_from_slice(&message[33..37]);
	destination_chain_id = u32::from_be_bytes(buf_dest_chain);

	let mut buf_dest_address = [0u8; 32];
	buf_dest_address[..32].copy_from_slice(&message[37..69]);
	destination_address = H256(buf_dest_address);

	let data = message[69..].to_vec();

	return Message {
		version,
		nonce,
		source_chain_id,
		source_address,
		destination_chain_id,
		destination_address,
		data,
	};
}

#[derive(Debug)]
pub enum StorageError {
	StorageError,
}

pub fn get_storage_value(
	slot_hash: H256,
	storage_root: H256,
	proof: Vec<Vec<u8>>,
) -> Result<H256, StorageError> {
	let db = StorageProof::new(proof).into_memory_db::<keccak256::KeccakHasher>();
	let trie =
		TrieDBBuilder::<EIP1186Layout<keccak256::KeccakHasher>>::new(&db, &storage_root).build();

	if let Some(storage_root) = trie
		.get(&slot_hash.as_bytes())
		.map_err(|_| StorageError::StorageError)?
	{
		let r = Rlp::new(storage_root.as_slice());
		// ensure!(r.data().len() > 0, Error::<T>::CannotDecodeRlpItems);
		let storage_value = r.data().map_err(|_| StorageError::StorageError)?;
		Ok(H256::from_slice(storage_value))
	} else {
		Err(StorageError::StorageError)
	}
}

pub fn get_storage_root(
	proof: Vec<Vec<u8>>,
	address: H160,
	state_root: H256,
) -> Result<H256, StorageError> {
	let key = keccak_256(address.as_bytes());
	let db = StorageProof::new(proof).into_memory_db::<keccak256::KeccakHasher>();
	let trie =
		TrieDBBuilder::<EIP1186Layout<keccak256::KeccakHasher>>::new(&db, &state_root).build();

	let result: DBValue = trie.get(&key.as_slice()).unwrap().unwrap();
	let byte_slice = result.as_slice();
	let r = Rlp::new(byte_slice);

	let item_count = r.item_count().map_err(|_| StorageError::StorageError)?;

	// ensure!(item_count == 4, Error::<T>::AccountNotFound);

	let item = r
		.at(2)
		.map_err(|_| StorageError::StorageError)?
		.data()
		.map_err(|_| StorageError::StorageError)?;

	let storage_root = H256::from_slice(item);

	Ok(storage_root)
}

pub mod keccak256 {
	use hash256_std_hasher::Hash256StdHasher;
	use sp_io::hashing::keccak_256;

	use super::*;

	/// Concrete implementation of Hasher using Keccak 256-bit hashes
	#[derive(Debug)]
	pub struct KeccakHasher;

	impl hash_db::Hasher for KeccakHasher {
		type Out = H256;
		type StdHasher = Hash256StdHasher;
		const LENGTH: usize = 32;

		fn hash(x: &[u8]) -> Self::Out {
			keccak_256(x).into()
		}
	}
}

#[cfg(test)]
mod test {
	use ark_std::vec;
	use hex_literal::hex;
	use patricia_merkle_trie::{EIP1186Layout, StorageProof};
	use primitive_types::{H160, H256, U256};
	use rlp::{Decodable, Rlp};
	use sp_core::keccak_256;
	use trie_db::Trie;
	use trie_db::{DBValue, TrieDBBuilder};

	use crate::target_amb::keccak256::KeccakHasher;
	use crate::target_amb::{decode_message, get_storage_root, get_storage_value};

	#[test]
	fn test_message_decoding() {
		let message_encoded = hex!("01000000000000007b00000005e2b19845fe2b7bb353f377d12dd51af012fbba20000000640000000000000000000000000000000000000000000000000000000000bc614e6789");

		let message_decoded = decode_message(message_encoded.to_vec());
		assert_eq!(123, message_decoded.nonce);
		assert_eq!(1, message_decoded.version);
		assert_eq!(5, message_decoded.source_chain_id);
		assert_eq!(
			H160(hex!("e2B19845Fe2B7Bb353f377d12dD51af012fbba20")),
			message_decoded.source_address
		);
		assert_eq!(100, message_decoded.destination_chain_id);
		assert_eq!(
			U256::from(12345678u64),
			U256::from(message_decoded.destination_address.as_bytes())
		);
		assert_eq!(vec![103, 137], message_decoded.data);

		println!("{:?}", message_decoded.nonce)
	}

	#[test]
	fn test_account_proof() {
		let key = H160::from_slice(&hex!("43f0222552e8114ad8F224DEA89976d3bf41659D").as_slice());
		let proof = vec![
            hex!("f90211a050da92c339db0b71cd6a8ac7893a6b8689ec5a3a46a0231b3ee2bd1baee75e1da045a3d973eb74a02b762d8b1ba683f39bca3965806276c8ceffe2d2ebc6cce233a0e88ad29ca98fa08f59f2a7f0110d63505d99a173628643290df869c4d1fa312ba00bb4cc9dc0b1de6ae0d80424b1fa992efb400a07a0e84615c91762fe734b2d0ca0a07e495d39bf2b779405790c6c7e7eb1cc3c803a88db36d1ec600fb0e555b5bba09a1c776e89c8be75d0a9ea022c05fd2ff095869d549e74a8fff7f2fb2deaf738a073b874e49e77dfd9312d7b1afd1ac10e02021a1ba2ab7c97ecaeaa0e26a34027a07e3424405c13aa33a2eb9ec6d8640aa1f67fdd8c8e9e4276334515b1cf1df65ca0246b93b2e3cc625a5e75b40165c6cb95ae8ffb9406563d34092d6359c7616aeea04d2fd8fdb1ab7d8f8fc6079400396fec828914230fade3794f13dc5ae7f6bbb8a04811b9efbfa8d495c5be91be78372b4a29140bd1e092e793db50ed9c495a6d54a02e1b3a417e8341dc8e1ade6ca527778192d33c7c827cfa63a366d007f2884e24a0845f4f33a4993d85766a14222cde1d124bd0f15523d239572883258a7bbcccd9a0ed2021cc2206fcfd9f80d592890b1b4eb615fae4f11d4e4a66d54a6767908901a07d46bf6e9dc9599eb7ca036aa976ef9cc63f02e9097252799f5d3a8792c49620a00b58d1d2cc72401c7cb978d34e15f74038ac63355e415d53b894179b8938dbb780").to_vec(),
            hex!("f90211a03e22056b0eefc94898d516b45ea579bd298291aa521c8665f3d5215da5619513a0b5f3f839320c3d63436a8f27a07bc47a5e7ace4f5d80e437ce2084a007e0fd37a0e666b7198d6b6023de9a698f4bc90a9595e57f7f4e70d07d0366c693d53994c5a05b14820b719fbb37f5e9ff55770d6ceb63ef90af46934377c0364ca72335852ea09c4a1a1d5b1e58e9be1c9b4ea4e943c514b4ae8a382be6dd16e53336354e0500a0058c24b25f97ed51ca2c44e016631753eb97197733b23aea23aef112a2323321a03347d79447b18678fbbedd01b48e52747a5301d32223c4be91f5681d2a69d7b2a04182f6e242615804a49f3a54399e285d84a6e7692cca41008d2b638be30fe00fa0c64a1e71e7512d73008d4cce2a2ba0981023c4ff5f821ba97fcf8059f4699bb5a0673bee8a446cac15221e9292a904ed44762ccb19dac57bbef085d76c6c5b9bb0a065d1ccec63163a4e5ea501f3951a384daaa9aaf4c9c976f963e3597b3e8ce4eca0fb4a788676b5a593e7db6c1149e3c89c774ef9915010846bcb53563736ccde70a0d5274ce6a4e744adab98139ed9d6b5846a449721f32d0f49e020061f5abb094ba0bbf7fd5e93a74f6d8ec4df6f2b0c7f6ff2b387a1a2cb2fd1f26545208c099443a0ddac5ec494b529e87a014e9f80869493008eba559e8ed9e9691fcf219bea14d0a06092b5dc5dd24f768b0c0bf74a6deb0e4e9a5fa3c474d06d52a63ace81d272c980").to_vec(),
            hex!("f90211a0d7b01a1c5e66b3cbb35832888bdb5c1312968a2849b942aad3433c6c21990faca067d17e56fc092306254b21e6101503c64326bbba467c714cadee8c9978aa2b57a0344929c8674281f336f38f5116469a7440bc41695916bd3faaf871716973a257a0e829cbcc6b207df95879af17d6df49a1327a63be6a2b2e8a2c1f8a8485a996e6a03a7f6e4ebd66e0377e7881a2de4361a34ac09116b0ccfe7bf2a96ab5100c4a21a0707b3b93b7aeae349737613b49037c406d411017fcf99c0877225338437fa549a0dce10f297e8bd76ff379c9ef548d51f491db677b566ceb5f83a139bd0b60ae4ea0c4f1e68723d248195d4439942c35f373ddd2889cd97a224ff1a3d379229b79aea0d1716748894608fdb98067c7daaad0e703cb42bd8bc57f39785b155f6914c2aca0c39df4d8b0242b1eaf733f3cd6237211c26b595a18d5e831c062a070ea3a4807a0b2e51fcaee45d252a96baf975e0e506dce7c7e3ddc39e30f7bb9de8955f602dba06ef05cdd0a80b246a4d91bc0dde4df927959474d567fdc9b11a586eede643191a07754dd15ffae315ed9f309f2e2722140fc1989c783fdda3f454fe8d5e7bf0e3ba06ca8810923e01ec88b0a14535fe248d6680df5de9becc5962b97a3c755bb2f84a07cdfc9857d06ba074ad5ca1769ac041c7c99e25a41331f625f16c6ce86bb1ba8a09d779a55977e48cd90d6c6b73b9b86301ad54bce224c4e1abcd7667dfa44347b80").to_vec(),
            hex!("f90211a0a9088ce9294db8a3f65adf5a3ceb5d1cd34c7804f8fe9a69eaf66bb860c5df91a0d6ad86f7ef958121aab83506dd9d5742f5980477e4db503c8a0eee7359d69857a00e72d2f638a2b873689a06afd5c080893e05ee6f8922b495d41b43727879cf3ea0e6f398effbe276d71947a920fc816602b255df3fb73bd59acfd3c036ae0f7996a0a84e9d20d33bb5d5db857f3ace6e32b54c93f7d148ddece8777d01aca293a9c4a0e3e7126f8ebb286919b3cfb2189a22f63fa475fc0fa7b36e79526f28993d089ea0af9c84fa15d80d5cd8462cc342072257f8eac2161a113da401536dc4b4de5ceba0df7cf975aa213b6ca4e655a99f4d074b24b5412e3056d1b39188550b49dea0f7a0c76abf47096e3b2660f935061b4e136378126aefecbcb348c895a4c678192536a0cf16074b69b96652074546708053506e6b2d5b7d6f3564f2091ddb690b701409a042330d1d46d74569cb62f900c06bb2659503d73b93be83371390904004897f11a00eefdc7fd5890c2051dd6f6326e036268613b8209b46c5f31dd4ca57e270a0eda05da0e6248c96f367e2b139c2329ca8ea2d4a9ba4c6438e2d33a9ed37f3d63104a0d43d6814ed1f765fd5d204ce91a92996adef6e65c563af59271b59bd933719eea04e6c678da69cd38894f2574d9b30d8871cd7eddd62b718e0941679a85a85b17da0300aa0769fa573f8c40bc841597e33d763ff32bc044f98aa6559e2df09b3174980").to_vec(),
            hex!("f90211a0bc80d8ae6ccac93ee4b2c020cdc98e960c2f840719e4eed518a28462f5c2e042a01481627b435734196f94592d10ca71c7dcb36eab7c2b39df29aa2c10ea944bc7a0627368fcc64ca6ddc311ddf7310625ef5d02a7a660a739047c4dde24d7f375aaa0ab6529dbec1ad45c32851fe70e17ca08332d8316453884c68f74e7a889ba46c0a0d73c5946469b9925e7681f45580a8d957f98a05f80a1a9bd7fe229ab79fbb7eca0406818fe909531d28e0461f93b428f6b530aa411529a53de213016e0d47693c4a0a2c63d00409e11dce2433521617080599719f65e727fb1b966d288fc5515515ba016e46c67a3b4aba26ed57a38bb0ac50d40301bd4e4482e3eec0667f2d70d4f9aa09262644352b4c7e435f2c77566b0f03b09b3109b0ba4fdb3c18f9f5b3ff83a68a006dc0a9848791e8068f25b0fca1a8f2a17c6421415f73355ee585f69e48dd9c0a0c158363b7c36d9abf2c07fac52c43ad8cbb3708af4c8375c64408da4b1c6112ea020290f03df9348a45be69b11f43ef60239aba95f31bc348439b4827c5a94ea1aa0950c0b5eb46cb26804706efb963b3e8cf3bbf0b0ce78fbfe4232f88e1cc4980fa0dfd3aa0540319f45916236f460f76831bfc526e8c0279fe798c3674ad08998eea0d68b134cb5a9433729bb46521b46e9bf737fabe2c1568185dc0d62cb2df23633a072708353bc10a239c80991deefd9a08158902b0d4ddd81857541368358e71ab280").to_vec(),
            hex!("f901518080a06b3861e939ffd924c512e631febac4b7573840910a230356701f5d9876d462f78080a0644b04a89b048be9044f7ddf0ddfcfdf16eb859770c59bea283be83efc0ab852a04783d2f6f95d2df8ecfe9cd176aabf0d5ce6e1a52009c0d7d8016a9c897cd996a05ebf2e95f0ce88623be1b9df655ddff6032bb68530ce80fc060914a26c983ed6a0b2cda30c80dadf34909d937dc977928bef8b702bcf64ac7cbfb14a1c55444898a0de3bef8b9dfce8c4a2d24b6ca802f5116e7e873ea2d0863f1cf72c23672f82c280a04e75b47f705d7811a0d326440a499b2dfeb0959cd151f91b71896111bfe8ae6580a05fccb9d0c6524886af03bb1f68990c9f54c098f57c664a5c51994052fd563aeca0cbab9ef5e83548e993c5cd9b688af2f34c6d9c5c632b59b687fa5a5e87b6bbf2a0fb82bb552d3eec458a68d01642f0e7df3d88d5b3040f69fa79b2e402adf412fa80").to_vec(),
            hex!("f851808080808080808080808080a035d937961d73f8a0eea9ae41b2f4cbb73c1d2c0666ea35f1ae05c43b5896b1098080a0e05c86fffb9aada22f0429326d6eda556e23f655917975b4f859bc258d32f67f80").to_vec(),
            hex!("f8669d399e1ef4313dc3558aee86cc911474c2262f1dbe387aea254422552a5fb846f8440180a0a03e10dfba89f79567f7c9a238ee7fe66ed32e711be4db6e73d7211601dec360a0356c7854fe7a483ece02a531c58b63aa2bdbab40df89c9f919f0d524b54dd494").to_vec(),
        ];

		// execution state root
		let root = H256(hex!(
			"cd187a0c3dddad24f1bb44211849cc55b6d2ff2713be85f727e9ab8c491c621c"
		));

		let expected_storage_root = H256(hex!(
			"a03e10dfba89f79567f7c9a238ee7fe66ed32e711be4db6e73d7211601dec360"
		));

		let storage_root_result = get_storage_root(proof, key, root);

		// assert_ok!(storage_root_result);
		assert_eq!(expected_storage_root, storage_root_result.unwrap());

		//
		// let mut rlp_value = Rlp::new(byte_slice);
		//
		// // let acc = Account::decode(&mut r).unwrap();
		// // println!("{:?}", acc);
		//
		// let expected_value =
		//     hex!("a03e10dfba89f79567f7c9a238ee7fe66ed32e711be4db6e73d7211601dec360");
		// assert_eq!(4, rlp_value.item_count().unwrap());
		// assert_eq!(expected_value, rlp_value.at(2).unwrap().data().unwrap());
		// // assert_eq!(4, r.item_count());
		// // assert_eq!(4, r.item_count());
		// // TODO cleanup this
		// println!("{:?}", rlp_value.item_count());
		// println!("{:02x?}", rlp_value.at(2).unwrap().data());
		// println!("==========================");
		// println!("to sting {:?}", rlp_value.to_string());
		//
		// println!("{:?}", rlp_value.at(0).unwrap().data());
		// println!("{:?}", rlp_value.at(1).unwrap().data());
		// println!("{:?}", rlp_value.at(2).unwrap().data());
		// println!("{:?}", rlp_value.at(3).unwrap().data());
		// println!("==========================");

		// let account = Account::decode(&rlp::Rlp::new(&result)).unwrap();
		// println!("account {:?}", account)
	}

	#[test]
	fn test_storage_value() {
		let message_bytes = hex!("01000000000000005400000005e2b19845fe2b7bb353f377d12dd51af012fbba2000000064000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000064").to_vec();
		let message_bytes1 = hex!("01000000000000005400000005e2b19845fe2b7bb353f377d12dd51af012fbba2000000064000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000064").to_vec();
		let message = decode_message(message_bytes);
		println!("{}", message.nonce);

		// 841
		let abi_encoded = hex!("00000000000000000000000000000000000000000000000000000000000000540000000000000000000000000000000000000000000000000000000000000001").as_slice();
		let binding = keccak_256(abi_encoded);
		let key = binding.as_slice();
		// 0x66b32740ad8041bcc3b909c72d7e1afe60094ec55e3cde329b4b3a28501d826c
		// println!("{:x?}", key);

		// let k = hex!("95eea00c49d14a895954837cd876ffa8cfad96cbaacc40fc31d6df2c902528a8");

		let proof = vec![
            hex!("f90211a0f0a16ee9b11528f3da8796229dad134b9085ed9428d868e6988f9b2473b59d6fa0f8175015d0a3df8fc451d2bd3d64a34e0836f3203129ac567e869f1157b488dfa0f9d56e943c6962cf8e2ca51b94b54307eb45424ebb84ed079b417cf03a85e298a0408af9f1c5f64ed6c517b1dbf661b75a705ef7d78bcae67b9a54c1e8052b56b2a02157d476a9a077cfc9eb00ead5ab65dcbfe363a71e993c3602a66c0fccf13e4aa00772697ebf25f2e83830918bd52bbb9600c077ae289e740ae76c7bdfd34b7ebea0a1dd0da76aacf7c82629c55e4b956b2e9ef77d7fdcee1adeb23d022f0950d554a0695cb723c857d98ad1c96a372f7983bf771556f4608674266a0698531543217ba05c0fb347305720b81c7d39be6fd5b2083af607654098a0f1418ec111a846510aa0ecd30808bffcb164a258c332a29f3050e9e85d28e988305b7f643dcad4f32c8fa0ec5ee93a7ede0a9c641dcd7515c1408ab48f86b5295cd26b3d738e8d8ac7829fa01434a5f6054456bbce0a59ba1c182eeee8e64fd6762ff365e550ca7cd8cedad0a0b4fefcb325f044a6663c9441ec9f025718d0f2d7fc1c29ec819f4a366cafbb6fa0cc26bfb18151569b0f765335474fa3840f9093385816bd14a4a3c553fae62949a06a28c02f7b649bad24b39d9a4e9fc4c8e93b1ae2b043af4f5bbcb8238e193eaba011ef889094bf6ca740810423041169453b7daea3df98b3018523f86e96bf033580").to_vec(),
            hex!("f8d180808080a0053a80e0ec0645b0acdddd1650b28104de2a51e7144bc5c7f7f69d44c544587a80a0bb2d4c2215259ba0a7fba5e750be34f510fb4494a19b4fbabc8b419f6a35346e808080a01a9817fbc2f3624eb22a44d5b6643c370eac51c77ff3a8d59f42b1d9fe5ea925a09c851efdcfd1d623fd4a3e5ef7f041b1f59b6ae7d60740291cc2e25bccc0a9b38080a0ddf637c0efd4778239f93a609faa694809faf5420e462488de85b0a2ba5bcf66a0fc31bff1855e70288e2c52383e1841cebc68bbcc08da7507c6112f2d2007231680").to_vec(),
            hex!("f843a0204effc936259a57c56ffc97bf601a6f6ee129ac5cd39809a889df1a8ad3fdc1a1a03617643cdff88aaf66c6d09fd11c1a73ce69dd905086afd692a62c4ba800fdd4").to_vec(),
        ];

		let storage_root1 = H256(hex!(
			"a03e10dfba89f79567f7c9a238ee7fe66ed32e711be4db6e73d7211601dec360"
		));

		let value = get_storage_value(H256(keccak_256(binding.as_slice())), storage_root1, proof);
		let expected_value = keccak_256(message_bytes1.as_slice());
		println!("{:?}", value);
		assert_eq!(H256(expected_value), value.unwrap())
	}

	#[test]
	fn test_storage_value_test() {
		let key = keccak_256(&hex!(
			"95eea00c49d14a895954837cd876ffa8cfad96cbaacc40fc31d6df2c902528a8"
		));

		println!("{:?}", H256(key));

		let proof = vec![
            hex!("f90211a03697534056039e03300557bd69fe16e18ce4a6ccd5522db4dfa97dfe1fad3d3aa0b1bf1f230b98b9034738d599177ae817c08143b9395a47f300636b0dd2fb3c5ea0aa04a4966751d4c50063fe13a96a6c7924f665819733f556849b5eb9fa1d6839a0e162e080d1c12c59dc984fb2246d8ad61209264bee40d3fdd07c4ea4ff411b6aa0e5c3f2dde71bf303423f34674748567dcdf8379129653b8213f698468738d492a068a3e3059b6e7115a055a7874f81c5a1e84ddc1967527973f8c78cd86a1c9f8fa0d734bd63b7be8e8471091b792f5bbcbc7b0ce582f6d985b7a15a3c0155242c56a00143c06f57a65c8485dbae750aa51df5dff1bf7bdf28060129a20de9e51364eda07b416f79b3f4e39d0159efff351009d44002d9e83530fb5a5778eb55f5f4432ca036706b52196fa0b73feb2e7ff8f1379c7176d427dd44ad63c7b65e66693904a1a0fd6c8b815e2769ce379a20eaccdba1f145fb11f77c280553f15ee4f1ee135375a02f5233009f082177e5ed2bfa6e180bf1a7310e6bc3c079cb85a4ac6fee4ae379a03f07f1bb33fa26ebd772fa874914dc7a08581095e5159fdcf9221be6cbeb6648a097557eec1ac08c3bfe45ce8e34cd329164a33928ac83fef1009656536ef6907fa028196bfb31aa7f14a0a8000b00b0aa5d09450c32d537e45eebee70b14313ff1ca0126ce265ca7bbb0e0b01f068d1edef1544cbeb2f048c99829713c18d7abc049a80").to_vec(),
            hex!("f90211a01f7c019858f447dbff8ed8e4329e88600bab8f17fec9594c664e25acc95da3dba00916866833439bc250b5e08edc2b7c041634ca3b33a013f79eb299a3e33a056fa0a8fae921061f3bc81154b5d2c149d3860a3cdef00d02173ee3b5837de6b26f56a097583621a54e74994619a97cf82f823005a35ad1bf4795047726619eccd11ad4a0be39789c4abdb2a185cce40f2c77575eee2a1eab38d3168395560da15307614aa0c46a7fcea5501656d70508178f0731460edcce1c01e32ed08c1468f2593db277a0460f030038b09b8461d834ded79d77fd3ed25c4e248775752bf1c830a530ef2ba03d887abec623c6b4d93be75d1608dcacb291bf30406fbbc944a94aa4203fb1eba0475eeb07d471044af74313093cfbb0201e17a405d7af13a7ae6b245e18915515a0ea794035230f90e14f4f601d0e9f04217ed02710df363417bc3dd7dda5a84608a08aa9f0c44e5a9e359b65d4f0e40937662f07af10642a626e19ca7bc56c329706a05677c9b342c1cbcfd491cd491800d44423d1bdc08ab00eb59376a7118f7c4e23a0b761f22d67328e6c90caf8be65affb701b045f4f1f581472fc5f724e0c61328da0b6727240643009e59bba78249918a83ca8faeaf1d5b47fe0b41c356e8ae300dda0d7cbeb12faf439126bdb86f94b6262c3a70e88e4d8a47b49735f0b9d632a5df8a0428fe930556ce5ea94bcc4d092fe0f05a2a9b360175857711729ab3a7092a81780").to_vec(),
            hex!("f90211a0a81ff74945250ba9925753e379aa8815a3fe77926aad2d02db4d78a3da7ecb48a0e795c64d2b738b34ebb77a3307df800c9f1fe324b442cd71ffa5fd268ec12ffca020a5f968a1c8292d08135cb451daed999a44eb0cdd04526a89c38e753398c50ca0cc7165f6b984f2f2569d101d70f72af94eb79b18c126895da2d3cf557b5aca51a00a10f4dc5851e71f195cca5bd7a2a62a6c3ca03d95e91c7dc55e55f4cb726903a09ecaaf877b18a55ca4001fe46cc10389c94104abc2994cdfe5843f28814b119ca099f59b2b52ee9d9b44ab7123a86775dcfe6c50301dd7cf9c6fc6ea968c1d2a01a0fccda5c1489dd3268fcf2471f6a372fc3afe5eebecc0ba9fc3c023d85da26aeda0d730ea7aabdad2e5451e826726e53c86e6e805e46220bc7edd80bf3f7e467f96a074f3d767a84557aa9559b08b2d1f93d8205827c042d1ac616b8cf37e22de2beca03ca1fe479ea1e64c3bfeae9c603ef4f55c270de0bb4c79d045f67d3481ca2852a0bdcbc3d154db40b5faa1f6875f46d485b96bdfffb4513da631f9b302768faae7a096923f4f559c7b7f912587292cc865aba5934e9e75c9aa88f20de73e928c6c51a0f52ca9710977327dd407ba9d9f00b0075d7e312ee19686f1b53579585ff50132a076adb4cf98f9af261d1b2a147b9b2cbace1647eab537ca1a55e8d40d35775b74a0723fa2f72d6a983939bc9bf9ee22e05fc142437726450e5707c60155bc34ec0580").to_vec(),
            hex!("f90211a039a64fd9b31f3ea3bf9a991ca828eadb67cf9ae0ebbcdd195d297454699580e8a0c95d85a63beb9a02b56d032116d86fdf9a64065dd5d44e33acef674ae3ceb6f9a0d83ef07a99302abccdce1289d353a20494713f45d8edbd3c2e87f08788a878d9a063a19aee41fec40a98edcf4d60c759c509b512bfc5d9feae7de50c8c2d00eee7a029ce5c8c1ac9939cbf481274b8e6f5f24a430136dc5aab7deb489a9ce7db5a95a03b45c53d2e4f54e49a53eb298aee828f4803581ee60ca52940533b77c3e3fadfa082b8084dfc0337a49fd5d53ce107fa0bdcdf25ac7bbac017d7ac250b77c8764da0d4dab90004fd3bb36b2fa5f6e914a7c90820d119e5ba3ba8439270c6cfbd0a18a0bd9286c9248ae8a953d00aa9906f06b6f0364d0bb9fb615de04c9f8d5b5fd346a00955eccaa41a17fafb0ed66272b5183b3a30a973af7a43e0f25f0b640fd5df0ca0a7ba773602ace05991211770fc5555dd9c55e2518bcb005d1db022584a89132da0450b066588b44701992ab4a926d5dd185dd465abf1e51a3f60ab4a9f924cf85aa05eda0687636512339d0db99eade61c0d44358bb8027185296ad59b4cedd2935aa0cd18604dee296e5443b598e470a04efde04fbb0213c0fda8cc3af20dae2a1c34a01c0474c1bf15e4732d1c22a1303573c7ec8643f711354e853ab26038b2eebe25a081a0b2d329a9519375f71384a7130faafc3a1379db59bfb7db7135c9d27d9cc080").to_vec(),
            hex!("f901f1a0e14d9caa85464966f0371f427250e7bf2d86f6d41535b08ea79044391d6f0fe7a08bea233c92e4c1d05bd03d62cd93974cc29f6df54e6fa2574bf8dfb3af936a85a0dd4f4af9ab72d1bbbf59c286a731614f48bf3cdcef572cd819426fc2a30ae5d9a058465ea8e97b2f3a873646f9504aff111ab967abaea8ebb2fe91bf058ea162f3a06466c41eb770bae5c07c26cd692540e7f9af70fee2bc164e12f29083ebd2cca1a0fe0925da033ffb967ca1e1d6505c2ca740553916c3b9a205131d719b7921fb00a0973ecee958d1305f1b6f8159a9732e47f5fbd4121f60254ea44a8f028308150780a0f81717c7f8702ed39d89a34fc86827aab31db26b22d22ee399667e4a44081c95a0e276ec24ee74ee61bdbe92e8afa57813d59f26e0ad34f24d71f0627719f8a11ea00a8ec2f6922480890f9e67c62c1654dcccf1710a01a378f614e02a3785d42d7ea08c43cc0c6c690dd87cf42691e9ead799c5ddbcfc9458ebd054a46acecedbe9f7a0d3727ed077c60ed6ff1d9d5d20fdf2912a513942d0efee9ec2d97d33ab9b7f56a03f293b0c0f25b9aa2735c4e42b132008d8af2ecfaaefdc940dc94cf312f5a1dda04860bc5f19829bb277554927c5b6dbc0af93025aca49aed3be020a3080996a26a0e47475bf4f62364d8a0dd87f2c21d0b64ef4d9aaf5fc46d9b1e3af5b83f56d4580").to_vec(),
            hex!("f89180a0dd3524693059f48ce47c5dd82d0462953a5141c7abc5a63981637b71e0100bcf80a033ca98826ea65c1c4be16e1df12e78142d992bf1fc0189401632ceff3fcbbe7880a0e5daf803b0890d164e287fa43b51332286cddeb9b62280426037fc02dc2b4d5f8080a0bfa907c2e30720a07347d23cfa573dec8c9b3afa51a4a0e0783a481da0fcb1b38080808080808080").to_vec(),
            hex!("e79e208246cec5810061f4ff7efe1dcd6cb407d59abc3478830df04484584c868786d647b234389e").to_vec(),
        ];

		let storage_root1 = H256(hex!(
			"3310913fe74cfb66dbde8fe8557b48e8e65617a17c2375a581c32d49f812cde4"
		));

		let db = StorageProof::new(proof).into_memory_db::<KeccakHasher>();
		let trie = TrieDBBuilder::<EIP1186Layout<KeccakHasher>>::new(&db, &storage_root1).build();

		let result: DBValue = trie.get(&key).unwrap().unwrap();

		let r = Rlp::new(result.as_slice());

		println!("{:?}", r.to_string());

		assert_eq!(r.data().unwrap(), hex!("d647b234389e"))
	}

	//
	// fn validate_message(message: Vec<u8>) {}
	//
	//
	//
	// #[cfg(test)]
	// mod test {
	//     // extern crate hashdb;
	//     // extern crate keccak_hasher;
	//     // extern crate memorydb;
	//     //
	//     // use ethereum_types::H256;
	//     // use hashdb::*;
	//     // use keccak_hasher::KeccakHasher;
	//     //
	//     // use patricia_trie::{DBValue, TrieDBMut};
	//     // use memorydb::MemoryDB;
	//     //
	//     // extern crate patricia_trie as trie;
	//     // extern crate patricia_trie_ethereum as ethtrie;
	//     // extern crate ethereum_types;
	//     //
	//     // use patricia_trie::TrieDB;
	//
	//
	//     // use hash256_std_hasher::Hash256StdHasher;
	//     // use hash_db::Hasher;
	//     // use tiny_keccak::{Hasher as _, Keccak};
	//     //
	//     // /// The `Keccak` hash output type.
	//     // pub type KeccakHash2 = [u8; 32];
	//     //
	//     // /// Concrete `Hasher` impl for the Keccak-256 hash
	//     // #[derive(Default, Debug, Clone, PartialEq)]
	//     // pub struct KeccakHasher2;
	//     //
	//     // impl Hasher for KeccakHasher2{
	//     //     type Out = KeccakHash2;
	//     //
	//     //     type StdHasher = Hash256StdHasher;
	//     //
	//     //     const LENGTH: usize = 32;
	//     //
	//     //     fn hash(x: &[u8]) -> Self::Out {
	//     //         let mut keccak = Keccak::v256();
	//     //         keccak.update(x);
	//     //         let mut out = [0u8; 32];
	//     //         keccak.finalize(&mut out);
	//     //         out
	//     //     }
	//     // }
	//
	//
	//     // use ethereum_types::H256;
	//     use hex_literal::hex;
	//     use primitive_types::H256;
	//     use sp_core::{keccak_256};
	//     use trie_db::{DBValue, TrieDBBuilder};
	//     use patricia_merkle_trie::{EIP1186Layout, StorageProof};
	//     use rlp::Rlp;
	//     // #[cfg(feature = "std")]
	//     use patricia_merkle_trie::keccak::KeccakHasher;
	//     use trie_db::proof::verify_proof;
	//     use trie_db::Trie;
	//
	//     #[test]
	//     fn test_insert() {
	//         // contract address
	//         let key = keccak_256(&hex!("43f0222552e8114ad8F224DEA89976d3bf41659D"));
	//         let proof = vec![
	//             hex!("f90211a050da92c339db0b71cd6a8ac7893a6b8689ec5a3a46a0231b3ee2bd1baee75e1da045a3d973eb74a02b762d8b1ba683f39bca3965806276c8ceffe2d2ebc6cce233a0e88ad29ca98fa08f59f2a7f0110d63505d99a173628643290df869c4d1fa312ba00bb4cc9dc0b1de6ae0d80424b1fa992efb400a07a0e84615c91762fe734b2d0ca0a07e495d39bf2b779405790c6c7e7eb1cc3c803a88db36d1ec600fb0e555b5bba09a1c776e89c8be75d0a9ea022c05fd2ff095869d549e74a8fff7f2fb2deaf738a073b874e49e77dfd9312d7b1afd1ac10e02021a1ba2ab7c97ecaeaa0e26a34027a07e3424405c13aa33a2eb9ec6d8640aa1f67fdd8c8e9e4276334515b1cf1df65ca0246b93b2e3cc625a5e75b40165c6cb95ae8ffb9406563d34092d6359c7616aeea04d2fd8fdb1ab7d8f8fc6079400396fec828914230fade3794f13dc5ae7f6bbb8a04811b9efbfa8d495c5be91be78372b4a29140bd1e092e793db50ed9c495a6d54a02e1b3a417e8341dc8e1ade6ca527778192d33c7c827cfa63a366d007f2884e24a0845f4f33a4993d85766a14222cde1d124bd0f15523d239572883258a7bbcccd9a0ed2021cc2206fcfd9f80d592890b1b4eb615fae4f11d4e4a66d54a6767908901a07d46bf6e9dc9599eb7ca036aa976ef9cc63f02e9097252799f5d3a8792c49620a00b58d1d2cc72401c7cb978d34e15f74038ac63355e415d53b894179b8938dbb780").to_vec(),
	//             hex!("f90211a03e22056b0eefc94898d516b45ea579bd298291aa521c8665f3d5215da5619513a0b5f3f839320c3d63436a8f27a07bc47a5e7ace4f5d80e437ce2084a007e0fd37a0e666b7198d6b6023de9a698f4bc90a9595e57f7f4e70d07d0366c693d53994c5a05b14820b719fbb37f5e9ff55770d6ceb63ef90af46934377c0364ca72335852ea09c4a1a1d5b1e58e9be1c9b4ea4e943c514b4ae8a382be6dd16e53336354e0500a0058c24b25f97ed51ca2c44e016631753eb97197733b23aea23aef112a2323321a03347d79447b18678fbbedd01b48e52747a5301d32223c4be91f5681d2a69d7b2a04182f6e242615804a49f3a54399e285d84a6e7692cca41008d2b638be30fe00fa0c64a1e71e7512d73008d4cce2a2ba0981023c4ff5f821ba97fcf8059f4699bb5a0673bee8a446cac15221e9292a904ed44762ccb19dac57bbef085d76c6c5b9bb0a065d1ccec63163a4e5ea501f3951a384daaa9aaf4c9c976f963e3597b3e8ce4eca0fb4a788676b5a593e7db6c1149e3c89c774ef9915010846bcb53563736ccde70a0d5274ce6a4e744adab98139ed9d6b5846a449721f32d0f49e020061f5abb094ba0bbf7fd5e93a74f6d8ec4df6f2b0c7f6ff2b387a1a2cb2fd1f26545208c099443a0ddac5ec494b529e87a014e9f80869493008eba559e8ed9e9691fcf219bea14d0a06092b5dc5dd24f768b0c0bf74a6deb0e4e9a5fa3c474d06d52a63ace81d272c980").to_vec(),
	//             hex!("f90211a0d7b01a1c5e66b3cbb35832888bdb5c1312968a2849b942aad3433c6c21990faca067d17e56fc092306254b21e6101503c64326bbba467c714cadee8c9978aa2b57a0344929c8674281f336f38f5116469a7440bc41695916bd3faaf871716973a257a0e829cbcc6b207df95879af17d6df49a1327a63be6a2b2e8a2c1f8a8485a996e6a03a7f6e4ebd66e0377e7881a2de4361a34ac09116b0ccfe7bf2a96ab5100c4a21a0707b3b93b7aeae349737613b49037c406d411017fcf99c0877225338437fa549a0dce10f297e8bd76ff379c9ef548d51f491db677b566ceb5f83a139bd0b60ae4ea0c4f1e68723d248195d4439942c35f373ddd2889cd97a224ff1a3d379229b79aea0d1716748894608fdb98067c7daaad0e703cb42bd8bc57f39785b155f6914c2aca0c39df4d8b0242b1eaf733f3cd6237211c26b595a18d5e831c062a070ea3a4807a0b2e51fcaee45d252a96baf975e0e506dce7c7e3ddc39e30f7bb9de8955f602dba06ef05cdd0a80b246a4d91bc0dde4df927959474d567fdc9b11a586eede643191a07754dd15ffae315ed9f309f2e2722140fc1989c783fdda3f454fe8d5e7bf0e3ba06ca8810923e01ec88b0a14535fe248d6680df5de9becc5962b97a3c755bb2f84a07cdfc9857d06ba074ad5ca1769ac041c7c99e25a41331f625f16c6ce86bb1ba8a09d779a55977e48cd90d6c6b73b9b86301ad54bce224c4e1abcd7667dfa44347b80").to_vec(),
	//             hex!("f90211a0a9088ce9294db8a3f65adf5a3ceb5d1cd34c7804f8fe9a69eaf66bb860c5df91a0d6ad86f7ef958121aab83506dd9d5742f5980477e4db503c8a0eee7359d69857a00e72d2f638a2b873689a06afd5c080893e05ee6f8922b495d41b43727879cf3ea0e6f398effbe276d71947a920fc816602b255df3fb73bd59acfd3c036ae0f7996a0a84e9d20d33bb5d5db857f3ace6e32b54c93f7d148ddece8777d01aca293a9c4a0e3e7126f8ebb286919b3cfb2189a22f63fa475fc0fa7b36e79526f28993d089ea0af9c84fa15d80d5cd8462cc342072257f8eac2161a113da401536dc4b4de5ceba0df7cf975aa213b6ca4e655a99f4d074b24b5412e3056d1b39188550b49dea0f7a0c76abf47096e3b2660f935061b4e136378126aefecbcb348c895a4c678192536a0cf16074b69b96652074546708053506e6b2d5b7d6f3564f2091ddb690b701409a042330d1d46d74569cb62f900c06bb2659503d73b93be83371390904004897f11a00eefdc7fd5890c2051dd6f6326e036268613b8209b46c5f31dd4ca57e270a0eda05da0e6248c96f367e2b139c2329ca8ea2d4a9ba4c6438e2d33a9ed37f3d63104a0d43d6814ed1f765fd5d204ce91a92996adef6e65c563af59271b59bd933719eea04e6c678da69cd38894f2574d9b30d8871cd7eddd62b718e0941679a85a85b17da0300aa0769fa573f8c40bc841597e33d763ff32bc044f98aa6559e2df09b3174980").to_vec(),
	//             hex!("f90211a0bc80d8ae6ccac93ee4b2c020cdc98e960c2f840719e4eed518a28462f5c2e042a01481627b435734196f94592d10ca71c7dcb36eab7c2b39df29aa2c10ea944bc7a0627368fcc64ca6ddc311ddf7310625ef5d02a7a660a739047c4dde24d7f375aaa0ab6529dbec1ad45c32851fe70e17ca08332d8316453884c68f74e7a889ba46c0a0d73c5946469b9925e7681f45580a8d957f98a05f80a1a9bd7fe229ab79fbb7eca0406818fe909531d28e0461f93b428f6b530aa411529a53de213016e0d47693c4a0a2c63d00409e11dce2433521617080599719f65e727fb1b966d288fc5515515ba016e46c67a3b4aba26ed57a38bb0ac50d40301bd4e4482e3eec0667f2d70d4f9aa09262644352b4c7e435f2c77566b0f03b09b3109b0ba4fdb3c18f9f5b3ff83a68a006dc0a9848791e8068f25b0fca1a8f2a17c6421415f73355ee585f69e48dd9c0a0c158363b7c36d9abf2c07fac52c43ad8cbb3708af4c8375c64408da4b1c6112ea020290f03df9348a45be69b11f43ef60239aba95f31bc348439b4827c5a94ea1aa0950c0b5eb46cb26804706efb963b3e8cf3bbf0b0ce78fbfe4232f88e1cc4980fa0dfd3aa0540319f45916236f460f76831bfc526e8c0279fe798c3674ad08998eea0d68b134cb5a9433729bb46521b46e9bf737fabe2c1568185dc0d62cb2df23633a072708353bc10a239c80991deefd9a08158902b0d4ddd81857541368358e71ab280").to_vec(),
	//             hex!("f901518080a06b3861e939ffd924c512e631febac4b7573840910a230356701f5d9876d462f78080a0644b04a89b048be9044f7ddf0ddfcfdf16eb859770c59bea283be83efc0ab852a04783d2f6f95d2df8ecfe9cd176aabf0d5ce6e1a52009c0d7d8016a9c897cd996a05ebf2e95f0ce88623be1b9df655ddff6032bb68530ce80fc060914a26c983ed6a0b2cda30c80dadf34909d937dc977928bef8b702bcf64ac7cbfb14a1c55444898a0de3bef8b9dfce8c4a2d24b6ca802f5116e7e873ea2d0863f1cf72c23672f82c280a04e75b47f705d7811a0d326440a499b2dfeb0959cd151f91b71896111bfe8ae6580a05fccb9d0c6524886af03bb1f68990c9f54c098f57c664a5c51994052fd563aeca0cbab9ef5e83548e993c5cd9b688af2f34c6d9c5c632b59b687fa5a5e87b6bbf2a0fb82bb552d3eec458a68d01642f0e7df3d88d5b3040f69fa79b2e402adf412fa80").to_vec(),
	//             hex!("f851808080808080808080808080a035d937961d73f8a0eea9ae41b2f4cbb73c1d2c0666ea35f1ae05c43b5896b1098080a0e05c86fffb9aada22f0429326d6eda556e23f655917975b4f859bc258d32f67f80").to_vec(),
	//             hex!("f8669d399e1ef4313dc3558aee86cc911474c2262f1dbe387aea254422552a5fb846f8440180a0a03e10dfba89f79567f7c9a238ee7fe66ed32e711be4db6e73d7211601dec360a0356c7854fe7a483ece02a531c58b63aa2bdbab40df89c9f919f0d524b54dd494").to_vec(),
	//         ];
	//
	//         // execution state root
	//         let root = H256(hex!("cd187a0c3dddad24f1bb44211849cc55b6d2ff2713be85f727e9ab8c491c621c"));
	//
	//         let db = StorageProof::new(proof).into_memory_db::<KeccakHasher>();
	//         let trie = TrieDBBuilder::<EIP1186Layout<KeccakHasher>>::new(&db, &root).build();
	//
	//
	//         verify_proof(&root, &proof, );
	//
	//
	//         let result: DBValue = trie.get(&key).unwrap().unwrap();
	//
	//
	//
	//         println!("{:02x?}", result);
	//     }
	//
	//     #[test]
	//     fn test_insert_storage() {
	//         // contract address
	//         let key = keccak_256(&hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001")); // slot key
	//         let proof = vec![
	//             hex!("f90211a0f0a16ee9b11528f3da8796229dad134b9085ed9428d868e6988f9b2473b59d6fa0f8175015d0a3df8fc451d2bd3d64a34e0836f3203129ac567e869f1157b488dfa0f9d56e943c6962cf8e2ca51b94b54307eb45424ebb84ed079b417cf03a85e298a0408af9f1c5f64ed6c517b1dbf661b75a705ef7d78bcae67b9a54c1e8052b56b2a02157d476a9a077cfc9eb00ead5ab65dcbfe363a71e993c3602a66c0fccf13e4aa00772697ebf25f2e83830918bd52bbb9600c077ae289e740ae76c7bdfd34b7ebea0a1dd0da76aacf7c82629c55e4b956b2e9ef77d7fdcee1adeb23d022f0950d554a0695cb723c857d98ad1c96a372f7983bf771556f4608674266a0698531543217ba05c0fb347305720b81c7d39be6fd5b2083af607654098a0f1418ec111a846510aa0ecd30808bffcb164a258c332a29f3050e9e85d28e988305b7f643dcad4f32c8fa0ec5ee93a7ede0a9c641dcd7515c1408ab48f86b5295cd26b3d738e8d8ac7829fa01434a5f6054456bbce0a59ba1c182eeee8e64fd6762ff365e550ca7cd8cedad0a0b4fefcb325f044a6663c9441ec9f025718d0f2d7fc1c29ec819f4a366cafbb6fa0cc26bfb18151569b0f765335474fa3840f9093385816bd14a4a3c553fae62949a06a28c02f7b649bad24b39d9a4e9fc4c8e93b1ae2b043af4f5bbcb8238e193eaba011ef889094bf6ca740810423041169453b7daea3df98b3018523f86e96bf033580").to_vec(),
	//             hex!("f8d180808080a0053a80e0ec0645b0acdddd1650b28104de2a51e7144bc5c7f7f69d44c544587a80a0bb2d4c2215259ba0a7fba5e750be34f510fb4494a19b4fbabc8b419f6a35346e808080a01a9817fbc2f3624eb22a44d5b6643c370eac51c77ff3a8d59f42b1d9fe5ea925a09c851efdcfd1d623fd4a3e5ef7f041b1f59b6ae7d60740291cc2e25bccc0a9b38080a0ddf637c0efd4778239f93a609faa694809faf5420e462488de85b0a2ba5bcf66a0fc31bff1855e70288e2c52383e1841cebc68bbcc08da7507c6112f2d2007231680").to_vec(),
	//             hex!("f843a0204effc936259a57c56ffc97bf601a6f6ee129ac5cd39809a889df1a8ad3fdc1a1a03617643cdff88aaf66c6d09fd11c1a73ce69dd905086afd692a62c4ba800fdd4").to_vec(),
	//         ];
	//
	//         // [f8, 44, 01, 80, a0, a0, 3e, 10, df, ba, 89, f7, 95, 67, f7, c9, a2, 38, ee, 7f, e6, 6e, d3, 2e, 71, 1b, e4, db, 6e, 73, d7, 21, 16, 01, de, c3, 60, a0, 35, 6c, 78, 54, fe, 7a, 48, 3e, ce, 02, a5, 31, c5, 8b, 63, aa, 2b, db, ab, 40, df, 89, c9, f9, 19, f0, d5, 24, b5, 4d, d4, 94]
	//
	//         // execution state root
	//         let storage_root = H256(hex!("cd187a0c3dddad24f1bb44211849cc55b6d2ff2713be85f727e9ab8c491c621c"));
	//
	//         let db = StorageProof::new(proof).into_memory_db::<KeccakHasher>();
	//         let trie = TrieDBBuilder::<EIP1186Layout<KeccakHasher>>::new(&db, &storage_root).build();
	//
	//         println!("{}", trie.root());
	//
	//         let result: DBValue = trie.get(&key).unwrap().unwrap();
	//         println!("{:?}", result);
	//     }
	//
	//
	//     //
	//     // use hash_db::{Hasher, HashDB, EMPTY_PREFIX};
	//     // use keccak_hasher::KeccakHasher;
	//     // use memory_db::{MemoryDB, HashKey};
	//     // #[test]
	//     // fn test_mp_trie() {
	//     //
	//     //       let mut m = MemoryDB::<KeccakHasher, HashKey<_>, Vec<u8>>::default();
	//     //
	//     //     println!("{}", 15);
	//     // }
	// }
}
