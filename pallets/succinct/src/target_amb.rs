use patricia_merkle_trie::{EIP1186Layout, StorageProof};
use rlp::Rlp;
use scale_info::prelude::vec::Vec;
use sp_core::{H160, H256};
use trie_db::{Trie, TrieDBBuilder};
use sp_io::hashing::sha2_256;

use crate::target_amb::keccak256::KeccakHasher;

#[derive(Debug, PartialEq)]
pub enum AMBError {
    InvalidTargetSlot,
    BranchIndexNotValid,
    CannotCastBranchLength,
    CannotGetTrieValue,
    CannotReadLog,
    CannotReadEventEmitter,
    CannotReadEventSignature,
    CannotReadTopicData,
    CannotConvertIndex,
    EventWasNotEmittedByClaimedEmitter,
    EventSignatureDoesNotMatch,

    NotFoundTrieValue,
    NotSupportedTransaction,
    InvalidReceiptLog,
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

    Message {
        version,
        nonce,
        source_chain_id,
        source_address,
        destination_chain_id,
        destination_address,
        data,
    }
}


fn restore_merkle_root(leaf: H256, mut index: u64, branch: Vec<H256>) -> Result<H256, AMBError> {
    let branch_len = u32::try_from(branch.len()).map_err(|_| AMBError::CannotCastBranchLength)?;
    if 2u64.pow(branch_len + 1) <= index {
        return Err(AMBError::BranchIndexNotValid);
    }

    let mut value = leaf;
    let mut i = 0;

    while index != 1 {
        if index % 2 == 1 {
            let mut result = [0; 64];
            result[32..].copy_from_slice(&value.as_bytes());
            result[..32].copy_from_slice(&branch[i].as_bytes());

            value = H256(sha2_256(result.as_slice()));
        } else {
            let mut result = [0; 64];
            result[32..].copy_from_slice(&branch[i].as_bytes());
            result[..32].copy_from_slice(&value.as_bytes());
            value = H256(sha2_256(result.as_slice()));
        }

        index /= 2;
        i += 1;
    }

    Ok(value)
}

pub fn get_event_topic(
    proof: Vec<Vec<u8>>,
    key: Vec<u8>,
    root: H256,
    log_index: u64,
    claimed_emitter: H160,
    event_signature_input: H256,
    topic_index: u32,
) -> Result<H256, AMBError> {
    let db = StorageProof::new(proof)
        .into_memory_db::<KeccakHasher>();
    let trie = TrieDBBuilder::<EIP1186Layout<KeccakHasher>>::new(&db, &root)
        .build();

    let result = trie.get(&key.as_slice())
        .map_err(|_| AMBError::CannotGetTrieValue)?;

    let value = result.ok_or(AMBError::NotFoundTrieValue)?;

    // Currently, there are three possible transaction types on Ethereum. Receipts either come
    // in the form "TransactionType | ReceiptPayload" or "ReceiptPayload". The currently
    // supported set of transaction types are 0x01 and 0x02. In this case, we must truncate
    // the first byte to access the payload. To detect the other case, we can use the fact
    // that the first byte of a RLP-encoded list will always be greater than 0xc0.
    // Reference 1: https://eips.ethereum.org/EIPS/eip-2718
    // Reference 2: https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp
    let tx_type_of_first_byte = value[0];

    let offset: usize;
    if tx_type_of_first_byte == 1 || tx_type_of_first_byte == 2 {
        offset = 1;
    } else if tx_type_of_first_byte >= 192 {
        offset = 0;
    } else {
        return Err(AMBError::NotSupportedTransaction);
    }

    let byte_slice = value.as_slice();

    let slice = &byte_slice[offset..];
    let rlp_value = Rlp::new(slice.as_ref());

    let values = rlp_value.item_count()
        .map_err(|_| AMBError::CannotReadLog)?;
    if values != 4 {
        return Err(AMBError::InvalidReceiptLog);
    }

    let logs = rlp_value.at(3)
        .map_err(|_| AMBError::CannotReadLog)?;


    let log_idx = usize::try_from(log_index)
        .map_err(|_| AMBError::CannotConvertIndex)?;

    let relevant_log = logs.at(log_idx)
        .map_err(|_| AMBError::CannotReadLog)?;
    let number_of_logs = relevant_log.item_count()
        .map_err(|_| AMBError::CannotReadLog)?;

    if number_of_logs != 3 {
        return Err(AMBError::InvalidReceiptLog);
    }

    let event_emitter = relevant_log.at(0)
        .map_err(|_| AMBError::CannotReadLog)?
        .data()
        .map_err(|_| AMBError::CannotReadEventEmitter)?;


    let event_emitter_address = H160::from_slice(event_emitter);
    if event_emitter_address != claimed_emitter {
        return Err(AMBError::EventWasNotEmittedByClaimedEmitter);
    }


    let event_signature = relevant_log.at(1)
        .map_err(|_| AMBError::CannotReadLog)?
        .at(0).map_err(|_| AMBError::CannotReadLog)?
        .data().map_err(|_| AMBError::CannotReadEventSignature)?;

    let event_signature = H256::from_slice(event_signature);
    if event_signature != event_signature_input {
        return Err(AMBError::EventSignatureDoesNotMatch);
    }

    let topic_idx = usize::try_from(topic_index)
        .map_err(|_| AMBError::CannotConvertIndex)?;
    let topic_data = relevant_log.at(1)
        .map_err(|_| AMBError::CannotReadLog)?
        .at(topic_idx).map_err(|_| AMBError::CannotReadLog)?
        .data().map_err(|_| AMBError::CannotReadTopicData)?;

    let data = H256::from_slice(topic_data);

    Ok(data)
}

pub fn verify_receipts_root(
    receipts_root: H256,
    receipts_root_proof: Vec<H256>,
    header_root: H256,
    src_slot: u64,
    tx_slot: u64,
    source_chain_id: u32,
    slots_per_historical_root: u64,
    historical_roots_limit: u64,
) -> Result<bool, AMBError> {
    // TODO is this required, capella is all we need?
    let capella_fork_slot = get_capella_slot(source_chain_id);

    // In Bellatrix we use state.historical_roots, in Capella we use state.historical_summaries
    // We use < here because capellaForkSlot is the last slot processed using Bellatrix logic;
    // the last batch in state.historical_roots contains the 8192 slots *before* capellaForkSlot.
    let state_to_historical_g_index = if tx_slot < capella_fork_slot { 7 } else { 27 };

    // The list state.historical_summaries is empty at the beginning of Capella
    let historical_list_index = if tx_slot < capella_fork_slot {
        tx_slot / slots_per_historical_root
    } else {
        (tx_slot - capella_fork_slot) / slots_per_historical_root
    };

    let mut index: u64;
    if src_slot == tx_slot {
        index = 8 + 3;
        index = index * 2u64.pow(9) + 387;
    } else if src_slot - tx_slot <= slots_per_historical_root {
        index = 8 + 3;
        index = index * 2u64.pow(5) + 6;
        index = index * slots_per_historical_root + tx_slot % slots_per_historical_root;
        index = index * 2u64.pow(9) + 387;
    } else if tx_slot < src_slot {
        index = 8 + 3;
        index = index * 2u64.pow(5) + state_to_historical_g_index;
        index = index * 2 + 0; // +0?
        index = index * historical_roots_limit + historical_list_index;
        index = index * 2 + 1;
        index = index * slots_per_historical_root + tx_slot % slots_per_historical_root;
        index = index * 2u64.pow(9) + 387;
    } else {
        return Err(AMBError::InvalidTargetSlot);
    }

    let computed_root = restore_merkle_root(receipts_root, index, receipts_root_proof)?;
    Ok(computed_root == header_root)
}

fn get_capella_slot(source_chain_id: u32) -> u64 {
    // Returns CAPELLA_FORK_EPOCH * SLOTS_PER_EPOCH for the corresponding beacon chain.
    match source_chain_id {
        // https://github.com/ethereum/consensus-specs/blob/dev/specs/capella/fork.md?plain=1#L30
        1 => 6209536,
        // https://blog.ethereum.org/2023/03/08/goerli-shapella-announcement
        //  https://github.com/eth-clients/goerli/blob/main/prater/config.yaml#L43
        5 => 5193728,
        _ => u64::MAX, // Return max uint256 for unknown sourceChainId
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use std::fs::File;
    use ark_std::vec;
    use hex_literal::hex;
    use primitive_types::{H160, H256, U256};
    use rlp::{Decodable};
    use sp_core::hexdisplay::ascii_format;
    use sp_io::hashing::keccak_256;

    use crate::target_amb::{decode_message, get_capella_slot, get_event_topic, verify_receipts_root};


    #[test]
    fn test_get_event_topic() {
        let tx_index_rlp_encoded = hex!("0a");
        let proof = vec![
            hex!("f891a013dac50a1c27f32bbe246112e1863ba3cffc4c80c1be0d25be2a8dcb4109b5cba001a811ec34dd545e7258e9b50b598fab8c5e661b7d0663f5f698a4e0778dfd2da00a1fd26a431028cbb11f2d5921b91de14af0d6b89f04eb90e977858521dd196e8080808080a09d322dc045913233638b59eea7bb855fe3719425bfaa3c369bbf46380ca87e748080808080808080").to_vec(),
            hex!("f901f180a08587e2dfd86ad2b26104473202069457b22903a686713501553b415817b39b29a073bbedceecc959735e1dbf16bb6dcabc25caf9bb4b10b33455f2cbc3ef2fa0dfa047c1728a392ef6e6e99dce5ff632a7125d61dbc76ee071a39abb015707ee4381a01397db6b22073d5bd649d04f8767db1a2e72a5c929aa0dc3a3b741f41a68b4cca0a85f8fab0f4fa23552272d67b6a12cf426863be110a3325411214cd9e524f70fa0985c0c311c107c272ad680d953b35ea6d5f155199b475d796202d982ac5554f9a040073d8d804c72ca07595a92b1205b5e93104243a6c20bb980b3f6f01b3faac1a00ba0ce6a6632d2519ff2f0bdfa99a7d87cefaa31956c675b218e510dd60c2b0aa0e715ded2e4268d7e9c17da9a94b666160a6b2fafab7d5c384c842d8dc7fa6345a0669a429fae3ca4703466c52ca9d05131c5c449c7625811175c5d30825eb5994ca0a3a7b504375df07869c580b61429312fdcf2de36f36e244782dfd4e04e36bd12a0033eb57f873d03aa38e6d5afa6578076e8450b484e44b2173707fae2fd4d6b5fa0398cbd1ef59a41d225101d82f92a7de87b03c73f384b0983ea3222089ccdfd71a09ffec1af26d3a5413ced31e6c27800f18f88e54982c0999ceea54dcc38043b51a0795d4e0884a7dd7626ec815607949aae540549519f0fb6edf3fac85744b8084780").to_vec(),
            hex!("f9027220b9026e02f9026a01833db22cb9010000000000000000000000000000000000000000000400000000000000000000000002000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000040000002000000000000000000000020000000000000000000800000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000020020000000000000000000000000000000000000000000000000000000000000000000f9015ff9015c942284a1b214d800748159237464de4d236c050377f863a0e5944a34d67c652e0ebf2304b48432aae0b55e40f79ba8a21a4d7054c169ffaca00000000000000000000000000000000000000000000000000000000000000000a0fe0891475496de5e014b0ec90de4411fcf5da2cc6962defa209a159dc2191791b8e00000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000008502000000000000000000000005ded0000e32f8f40414d3ab3a830f735a3553e18e00000064000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000064000000000000000000000000000000000000000000000000000000").to_vec(),
        ];
        let root = H256(hex!(
		"fde7d8248cf1bb7c63e4de2341d337b499ab4ee72de6da29ff4341004c2eb31c"
	));
        let log_index = 0;
        let broadcaster = H160(hex!("2284a1b214d800748159237464de4d236c050377"));
        let event_signature = H256(keccak_256("SentMessage(uint64,bytes32,bytes)".as_bytes()));
        let result = get_event_topic(
            proof,
            tx_index_rlp_encoded.to_vec(),
            root,
            log_index,
            broadcaster,
            event_signature,
            2,
        );

        assert_eq!(true, result.is_ok());
        assert_eq!(H256(hex!("fe0891475496de5e014b0ec90de4411fcf5da2cc6962defa209a159dc2191791")), result.unwrap());
    }

    #[test]
    fn test_capella_slots() {
        let source_chain_eth = 1;
        assert_eq!(6209536, get_capella_slot(source_chain_eth));

        let source_chain_goerli = 5;
        assert_eq!(5193728, get_capella_slot(source_chain_goerli));

        let source_chain_unknown = 5541;
        assert_eq!(u64::MAX, get_capella_slot(source_chain_unknown));
    }


    // TODO make test generic and add more test cases
    #[test]
    fn test_verify_receipts_root() {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct RootProof {
            header_root: String,
            tx_slot: u64,
            src_slot: u64,
            receipts_root_proof: Vec<String>,
            receipts_root: String,
            gindex: u64,
            source_chain: u32,
        }

        for i in 65u8..74u8 { // ascii A-J
            for j in 1u8..5u8 { // 1-5
                let mut path = format!("src/test/capella/receiptsRootProof_ {}{}.json", ascii_format(i.to_be_bytes().as_slice()), j);
                let file = File::open("src/test/capella/receiptsRootProof_A1.json").unwrap();
                let json: RootProof =
                    serde_json::from_reader(file).expect("JSON was not well-formatted");

                let mut proofs: Vec<H256> = vec![];
                for p in json.receipts_root_proof {
                    proofs.push(p.parse::<H256>().unwrap())
                }

                let is_valid = verify_receipts_root(
                    json.receipts_root.parse::<H256>().unwrap(),
                    proofs,
                    json.header_root.parse::<H256>().unwrap(),
                    json.src_slot,
                    json.tx_slot,
                    json.source_chain,
                    8192,
                    16777216,
                );

                assert_eq!(true, is_valid.unwrap());
            }
        }
    }

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
    }
}
