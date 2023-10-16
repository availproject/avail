use ark_std::iterable::Iterable;
use ark_std::vec;
use patricia_merkle_trie::{EIP1186Layout, StorageProof};
use primitive_types::H256;
use scale_info::prelude::vec::Vec;
use trie_db::{DBValue, Trie, TrieDBBuilder};

use crate::{Config, Error, MessageStatus};

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

pub struct Message {
	pub version: u8,
	pub nonce: u64,
	pub source_chain_id: u32,
	pub source_address: H256,
	pub destination_chain_id: u32,
	pub destination_address: H256,
	pub data: Vec<u8>,
}

// TODO add message decoding

pub fn decode_message(message: Vec<u8>) -> Message {
	return Message {
		version: 0,
		nonce: 0,
		source_chain_id: 0,
		source_address: Default::default(),
		destination_chain_id: 0,
		destination_address: Default::default(),
		data: vec![],
	};
}

pub fn get_storage_proof(key: H256, root: H256, proof: Vec<Vec<u8>>) {
	let db = StorageProof::new(proof).into_memory_db::<keccak256::KeccakHasher>();
	let trie = TrieDBBuilder::<EIP1186Layout<keccak256::KeccakHasher>>::new(&db, &root).build();

	let result: DBValue = trie.get(&key.0).unwrap().unwrap();
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
	use primitive_types::H256;
	use rlp::Decodable;
	use sp_core::keccak_256;
	use trie_db::Trie;
	use trie_db::{DBValue, TrieDBBuilder};

	use crate::target_amb::keccak256::KeccakHasher;
	use crate::target_amb::Account;

	// #[test]
	// fn test_account_proof() {
	//     let key = keccak_256(&hex!("43f0222552e8114ad8F224DEA89976d3bf41659D"));
	//     let proof = vec![
	//         hex!("f90211a050da92c339db0b71cd6a8ac7893a6b8689ec5a3a46a0231b3ee2bd1baee75e1da045a3d973eb74a02b762d8b1ba683f39bca3965806276c8ceffe2d2ebc6cce233a0e88ad29ca98fa08f59f2a7f0110d63505d99a173628643290df869c4d1fa312ba00bb4cc9dc0b1de6ae0d80424b1fa992efb400a07a0e84615c91762fe734b2d0ca0a07e495d39bf2b779405790c6c7e7eb1cc3c803a88db36d1ec600fb0e555b5bba09a1c776e89c8be75d0a9ea022c05fd2ff095869d549e74a8fff7f2fb2deaf738a073b874e49e77dfd9312d7b1afd1ac10e02021a1ba2ab7c97ecaeaa0e26a34027a07e3424405c13aa33a2eb9ec6d8640aa1f67fdd8c8e9e4276334515b1cf1df65ca0246b93b2e3cc625a5e75b40165c6cb95ae8ffb9406563d34092d6359c7616aeea04d2fd8fdb1ab7d8f8fc6079400396fec828914230fade3794f13dc5ae7f6bbb8a04811b9efbfa8d495c5be91be78372b4a29140bd1e092e793db50ed9c495a6d54a02e1b3a417e8341dc8e1ade6ca527778192d33c7c827cfa63a366d007f2884e24a0845f4f33a4993d85766a14222cde1d124bd0f15523d239572883258a7bbcccd9a0ed2021cc2206fcfd9f80d592890b1b4eb615fae4f11d4e4a66d54a6767908901a07d46bf6e9dc9599eb7ca036aa976ef9cc63f02e9097252799f5d3a8792c49620a00b58d1d2cc72401c7cb978d34e15f74038ac63355e415d53b894179b8938dbb780").to_vec(),
	//         hex!("f90211a03e22056b0eefc94898d516b45ea579bd298291aa521c8665f3d5215da5619513a0b5f3f839320c3d63436a8f27a07bc47a5e7ace4f5d80e437ce2084a007e0fd37a0e666b7198d6b6023de9a698f4bc90a9595e57f7f4e70d07d0366c693d53994c5a05b14820b719fbb37f5e9ff55770d6ceb63ef90af46934377c0364ca72335852ea09c4a1a1d5b1e58e9be1c9b4ea4e943c514b4ae8a382be6dd16e53336354e0500a0058c24b25f97ed51ca2c44e016631753eb97197733b23aea23aef112a2323321a03347d79447b18678fbbedd01b48e52747a5301d32223c4be91f5681d2a69d7b2a04182f6e242615804a49f3a54399e285d84a6e7692cca41008d2b638be30fe00fa0c64a1e71e7512d73008d4cce2a2ba0981023c4ff5f821ba97fcf8059f4699bb5a0673bee8a446cac15221e9292a904ed44762ccb19dac57bbef085d76c6c5b9bb0a065d1ccec63163a4e5ea501f3951a384daaa9aaf4c9c976f963e3597b3e8ce4eca0fb4a788676b5a593e7db6c1149e3c89c774ef9915010846bcb53563736ccde70a0d5274ce6a4e744adab98139ed9d6b5846a449721f32d0f49e020061f5abb094ba0bbf7fd5e93a74f6d8ec4df6f2b0c7f6ff2b387a1a2cb2fd1f26545208c099443a0ddac5ec494b529e87a014e9f80869493008eba559e8ed9e9691fcf219bea14d0a06092b5dc5dd24f768b0c0bf74a6deb0e4e9a5fa3c474d06d52a63ace81d272c980").to_vec(),
	//         hex!("f90211a0d7b01a1c5e66b3cbb35832888bdb5c1312968a2849b942aad3433c6c21990faca067d17e56fc092306254b21e6101503c64326bbba467c714cadee8c9978aa2b57a0344929c8674281f336f38f5116469a7440bc41695916bd3faaf871716973a257a0e829cbcc6b207df95879af17d6df49a1327a63be6a2b2e8a2c1f8a8485a996e6a03a7f6e4ebd66e0377e7881a2de4361a34ac09116b0ccfe7bf2a96ab5100c4a21a0707b3b93b7aeae349737613b49037c406d411017fcf99c0877225338437fa549a0dce10f297e8bd76ff379c9ef548d51f491db677b566ceb5f83a139bd0b60ae4ea0c4f1e68723d248195d4439942c35f373ddd2889cd97a224ff1a3d379229b79aea0d1716748894608fdb98067c7daaad0e703cb42bd8bc57f39785b155f6914c2aca0c39df4d8b0242b1eaf733f3cd6237211c26b595a18d5e831c062a070ea3a4807a0b2e51fcaee45d252a96baf975e0e506dce7c7e3ddc39e30f7bb9de8955f602dba06ef05cdd0a80b246a4d91bc0dde4df927959474d567fdc9b11a586eede643191a07754dd15ffae315ed9f309f2e2722140fc1989c783fdda3f454fe8d5e7bf0e3ba06ca8810923e01ec88b0a14535fe248d6680df5de9becc5962b97a3c755bb2f84a07cdfc9857d06ba074ad5ca1769ac041c7c99e25a41331f625f16c6ce86bb1ba8a09d779a55977e48cd90d6c6b73b9b86301ad54bce224c4e1abcd7667dfa44347b80").to_vec(),
	//         hex!("f90211a0a9088ce9294db8a3f65adf5a3ceb5d1cd34c7804f8fe9a69eaf66bb860c5df91a0d6ad86f7ef958121aab83506dd9d5742f5980477e4db503c8a0eee7359d69857a00e72d2f638a2b873689a06afd5c080893e05ee6f8922b495d41b43727879cf3ea0e6f398effbe276d71947a920fc816602b255df3fb73bd59acfd3c036ae0f7996a0a84e9d20d33bb5d5db857f3ace6e32b54c93f7d148ddece8777d01aca293a9c4a0e3e7126f8ebb286919b3cfb2189a22f63fa475fc0fa7b36e79526f28993d089ea0af9c84fa15d80d5cd8462cc342072257f8eac2161a113da401536dc4b4de5ceba0df7cf975aa213b6ca4e655a99f4d074b24b5412e3056d1b39188550b49dea0f7a0c76abf47096e3b2660f935061b4e136378126aefecbcb348c895a4c678192536a0cf16074b69b96652074546708053506e6b2d5b7d6f3564f2091ddb690b701409a042330d1d46d74569cb62f900c06bb2659503d73b93be83371390904004897f11a00eefdc7fd5890c2051dd6f6326e036268613b8209b46c5f31dd4ca57e270a0eda05da0e6248c96f367e2b139c2329ca8ea2d4a9ba4c6438e2d33a9ed37f3d63104a0d43d6814ed1f765fd5d204ce91a92996adef6e65c563af59271b59bd933719eea04e6c678da69cd38894f2574d9b30d8871cd7eddd62b718e0941679a85a85b17da0300aa0769fa573f8c40bc841597e33d763ff32bc044f98aa6559e2df09b3174980").to_vec(),
	//         hex!("f90211a0bc80d8ae6ccac93ee4b2c020cdc98e960c2f840719e4eed518a28462f5c2e042a01481627b435734196f94592d10ca71c7dcb36eab7c2b39df29aa2c10ea944bc7a0627368fcc64ca6ddc311ddf7310625ef5d02a7a660a739047c4dde24d7f375aaa0ab6529dbec1ad45c32851fe70e17ca08332d8316453884c68f74e7a889ba46c0a0d73c5946469b9925e7681f45580a8d957f98a05f80a1a9bd7fe229ab79fbb7eca0406818fe909531d28e0461f93b428f6b530aa411529a53de213016e0d47693c4a0a2c63d00409e11dce2433521617080599719f65e727fb1b966d288fc5515515ba016e46c67a3b4aba26ed57a38bb0ac50d40301bd4e4482e3eec0667f2d70d4f9aa09262644352b4c7e435f2c77566b0f03b09b3109b0ba4fdb3c18f9f5b3ff83a68a006dc0a9848791e8068f25b0fca1a8f2a17c6421415f73355ee585f69e48dd9c0a0c158363b7c36d9abf2c07fac52c43ad8cbb3708af4c8375c64408da4b1c6112ea020290f03df9348a45be69b11f43ef60239aba95f31bc348439b4827c5a94ea1aa0950c0b5eb46cb26804706efb963b3e8cf3bbf0b0ce78fbfe4232f88e1cc4980fa0dfd3aa0540319f45916236f460f76831bfc526e8c0279fe798c3674ad08998eea0d68b134cb5a9433729bb46521b46e9bf737fabe2c1568185dc0d62cb2df23633a072708353bc10a239c80991deefd9a08158902b0d4ddd81857541368358e71ab280").to_vec(),
	//         hex!("f901518080a06b3861e939ffd924c512e631febac4b7573840910a230356701f5d9876d462f78080a0644b04a89b048be9044f7ddf0ddfcfdf16eb859770c59bea283be83efc0ab852a04783d2f6f95d2df8ecfe9cd176aabf0d5ce6e1a52009c0d7d8016a9c897cd996a05ebf2e95f0ce88623be1b9df655ddff6032bb68530ce80fc060914a26c983ed6a0b2cda30c80dadf34909d937dc977928bef8b702bcf64ac7cbfb14a1c55444898a0de3bef8b9dfce8c4a2d24b6ca802f5116e7e873ea2d0863f1cf72c23672f82c280a04e75b47f705d7811a0d326440a499b2dfeb0959cd151f91b71896111bfe8ae6580a05fccb9d0c6524886af03bb1f68990c9f54c098f57c664a5c51994052fd563aeca0cbab9ef5e83548e993c5cd9b688af2f34c6d9c5c632b59b687fa5a5e87b6bbf2a0fb82bb552d3eec458a68d01642f0e7df3d88d5b3040f69fa79b2e402adf412fa80").to_vec(),
	//         hex!("f851808080808080808080808080a035d937961d73f8a0eea9ae41b2f4cbb73c1d2c0666ea35f1ae05c43b5896b1098080a0e05c86fffb9aada22f0429326d6eda556e23f655917975b4f859bc258d32f67f80").to_vec(),
	//         hex!("f8669d399e1ef4313dc3558aee86cc911474c2262f1dbe387aea254422552a5fb846f8440180a0a03e10dfba89f79567f7c9a238ee7fe66ed32e711be4db6e73d7211601dec360a0356c7854fe7a483ece02a531c58b63aa2bdbab40df89c9f919f0d524b54dd494").to_vec(),
	//     ];
	//
	//     // execution state root
	//     let root = H256(hex!("cd187a0c3dddad24f1bb44211849cc55b6d2ff2713be85f727e9ab8c491c621c"));
	//
	//     let db = StorageProof::new(proof).into_memory_db::<KeccakHasher>();
	//     let trie = TrieDBBuilder::<EIP1186Layout<KeccakHasher>>::new(&db, &root).build();
	//     let result: DBValue = trie.get(&key).unwrap().unwrap();
	//
	//
	//     let byte_slice = result.as_slice();
	//
	//     let mut r = Rlp::new(byte_slice);
	//
	//     // let acc = Account::decode(&mut r).unwrap();
	//     // println!("{:?}", acc);
	//
	//
	//     println!("{:?}", r.item_count());
	//     println!("{:?}", r.at(2));
	//     println!("==========================");
	//     println!("{:?}", r.at(0).unwrap().data());
	//     println!("{:?}", r.at(1).unwrap().data());
	//     println!("{:?}", r.at(2).unwrap().data());
	//     println!("{:?}", r.at(3).unwrap().data());
	//     println!("==========================");
	//
	//
	//     // let account = Account::decode(&rlp::Rlp::new(&result)).unwrap();
	//     // println!("account {:?}", account)
	// }

	#[test]
	fn test_storage_value() {
		// 000000000000000100

		let hash = hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001");

		let key = keccak_256(keccak_256(hash.as_slice()).as_slice());
		// 0x66b32740ad8041bcc3b909c72d7e1afe60094ec55e3cde329b4b3a28501d826c
		println!("{:x?}", key);

		let proof = vec![
            hex!("f90211a0f0a16ee9b11528f3da8796229dad134b9085ed9428d868e6988f9b2473b59d6fa0f8175015d0a3df8fc451d2bd3d64a34e0836f3203129ac567e869f1157b488dfa0f9d56e943c6962cf8e2ca51b94b54307eb45424ebb84ed079b417cf03a85e298a0408af9f1c5f64ed6c517b1dbf661b75a705ef7d78bcae67b9a54c1e8052b56b2a02157d476a9a077cfc9eb00ead5ab65dcbfe363a71e993c3602a66c0fccf13e4aa00772697ebf25f2e83830918bd52bbb9600c077ae289e740ae76c7bdfd34b7ebea0a1dd0da76aacf7c82629c55e4b956b2e9ef77d7fdcee1adeb23d022f0950d554a0695cb723c857d98ad1c96a372f7983bf771556f4608674266a0698531543217ba05c0fb347305720b81c7d39be6fd5b2083af607654098a0f1418ec111a846510aa0ecd30808bffcb164a258c332a29f3050e9e85d28e988305b7f643dcad4f32c8fa0ec5ee93a7ede0a9c641dcd7515c1408ab48f86b5295cd26b3d738e8d8ac7829fa01434a5f6054456bbce0a59ba1c182eeee8e64fd6762ff365e550ca7cd8cedad0a0b4fefcb325f044a6663c9441ec9f025718d0f2d7fc1c29ec819f4a366cafbb6fa0cc26bfb18151569b0f765335474fa3840f9093385816bd14a4a3c553fae62949a06a28c02f7b649bad24b39d9a4e9fc4c8e93b1ae2b043af4f5bbcb8238e193eaba011ef889094bf6ca740810423041169453b7daea3df98b3018523f86e96bf033580").to_vec(),
            hex!("f8d1808080a0f79069170d9bd1640662027bb904b1f5655f346c231e6c82467dbdeea9aedc938080808080a0e74e7a66148fbbe731b2ee3c9983cee7aa421ce456e8e6d10cd7dcab71cf25d0a0b6d34e2a998dea02a986714f8b74cf215c5b3dd21ce40289dbd425a35fa07cfa8080a0e0a0199f06e921919a65252eec43991d50d664accc0e62f6ee3f8762d60757e3a08a8491cb5f3b10f33d9c227dbda2fa944fc5fddf49d81108d53dc56151b06569a0bc83eb0afdea380f55a8956b903999c79a67f8729ea067187431810e18b8993180").to_vec(),
            hex!("f843a0206da15a1efe671ce3bd410addbe01acf66ddcc7f403dfe6617a85c318a2e5cca1a0fa4f7548b723c0a10458af299de4e7cf3ab69189c9042e38325e4e882b82e611").to_vec(),
        ];

		// execution state root
		// let storage_root = H256([160, 62, 16, 223, 186, 137, 247, 149, 103, 247, 201, 162, 56, 238, 127, 230, 110, 211, 46, 113, 27, 228, 219, 110, 115, 215, 33, 22, 1, 222, 195, 96]);
		// println!("{}", storage_root);
		let storage_root1 = H256(hex!(
			"a03e10dfba89f79567f7c9a238ee7fe66ed32e711be4db6e73d7211601dec360"
		));

		let db = StorageProof::new(proof).into_memory_db::<KeccakHasher>();
		let trie = TrieDBBuilder::<EIP1186Layout<KeccakHasher>>::new(&db, &storage_root1).build();

		let result: DBValue = trie.get(&key).unwrap().unwrap();
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
