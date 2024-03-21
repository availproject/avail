use avail_core::{
	data_proof::{tx_uid, AddressedMessage, BoundedData, Message, SubTrie},
	AppId, Keccak256,
};
use avail_subxt::{
	api, avail_client::RpcMethods, rpc::KateRpcClient as _, tx, AccountId, AvailClient, Opts,
};

use binary_merkle_tree::verify_proof;
use derive_more::Constructor;
use futures::stream::{FuturesOrdered, TryStreamExt as _};
use hex_literal::hex;
use std::collections::HashSet;
use structopt::StructOpt;
use subxt::{backend::BlockRef, error::RpcError, utils::H256, Error};
use subxt_signer::sr25519::dev;

const DATA: &[u8] = b"Test 42";
const DOMAIN: u32 = 2;

fn messages() -> Vec<Message> {
	let asset_id = hex!("4554480000000000000000000000000000000000000000000000000000000000");

	vec![
		Message::ArbitraryMessage(BoundedData::truncate_from(DATA.to_vec())),
		Message::FungibleToken {
			asset_id: H256(asset_id),
			amount: 42_000_000_000_000_000_000u128,
		},
	]
}

/// Send messages and wait until they are in the same block.
/// It tries up to 5 times to archive the same block.
async fn send_messages_in_same_block(client: &AvailClient) -> Result<(H256, Vec<u32>), Error> {
	let alice = dev::alice();
	let to_bob = H256(AccountId::from(dev::bob().public_key()).0);

	for _ in 0..5 {
		let nonce = tx::nonce(&client, &alice).await?;
		let calls = messages()
			.into_iter()
			.map(|m| api::tx().vector().send_message(m.into(), to_bob, DOMAIN))
			.collect::<Vec<_>>();

		// Send messages all messages.
		let send_progress_list = calls
			.iter()
			.enumerate()
			.map(|(idx, call)| {
				tx::send_with_nonce(&client, call, &alice, AppId(0), nonce + idx as u64)
			})
			.collect::<FuturesOrdered<_>>()
			.try_collect::<Vec<_>>()
			.await?;
		println!(
			"Messages sent (len={}) to the network",
			send_progress_list.len()
		);

		// Wait until all messages are finalized.
		let in_block_list = send_progress_list
			.into_iter()
			.map(tx::in_finalized)
			.collect::<FuturesOrdered<_>>()
			.try_collect::<Vec<_>>()
			.await?;
		let hashes = in_block_list
			.iter()
			.map(|p| p.block_hash())
			.collect::<HashSet<_>>();
		println!("Messages in blocks: {hashes:?}");

		// Ensure all messages are in the same block.
		if hashes.len() == 1 {
			let hash = hashes.into_iter().next().unwrap();

			// Extract extrinsic indexes.
			let tx_indexes = in_block_list
				.iter()
				.map(|b| b.wait_for_success())
				.collect::<FuturesOrdered<_>>()
				.try_collect::<Vec<_>>()
				.await?
				.into_iter()
				.map(|event| event.extrinsic_index())
				.collect::<Vec<_>>();

			return Ok((hash, tx_indexes));
		}
	}

	Err(Error::Other("Messages not in the same block".to_string()))
}

#[derive(Constructor)]
struct Leaf {
	leaf_idx: usize,
	tx_idx: u32,
	leaf: Vec<u8>,
}

fn messages_to_leaves(block_number: u32, tx_indexes: Vec<u32>) -> Vec<Leaf> {
	let from: AccountId = dev::alice().public_key().into();
	let to: AccountId = dev::bob().public_key().into();
	messages()
		.into_iter()
		.zip(tx_indexes)
		.enumerate()
		.map(|(leaf_idx, (m, tx_idx))| {
			let id = tx_uid(block_number, tx_idx);
			let addr_msg = AddressedMessage::new(m, H256(from.0), H256(to.0), 1, DOMAIN, id);
			let leaf = addr_msg.abi_encode();
			Leaf::new(leaf_idx, tx_idx, leaf)
		})
		.collect::<Vec<_>>()
}

async fn check_query_data_proof_rpc(
	rpc_methods: &RpcMethods,
	block_hash: H256,
	leaves: &[Leaf],
) -> Result<(), Error> {
	let indexed_leafs_len = leaves.len();
	for indexed_leaf in leaves {
		let Leaf {
			leaf_idx,
			tx_idx,
			leaf,
		} = indexed_leaf;

		let rpc_proof = rpc_methods
			.query_data_proof(*tx_idx, block_hash)
			.await
			.map_err(|je| RpcError::ClientError(Box::new(je)))?;
		let bridge_root = rpc_proof.data_proof.roots.bridge_root;
		let proof = rpc_proof
			.data_proof
			.as_sub_merkle_proof(SubTrie::Bridge, leaf.clone())
			.proof;
		let verified = verify_proof::<Keccak256, _, _>(
			&bridge_root,
			proof,
			indexed_leafs_len,
			*leaf_idx,
			leaf,
		);
		println!("Proof for leaf {leaf_idx:?} verified: {verified:?}");
		assert!(verified);
	}

	Ok(())
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
	let args = Opts::from_args();
	let client = AvailClient::new(args.ws).await?;

	// 0. Send messages and get the block.
	let (block_hash, tx_indexes) = send_messages_in_same_block(&client).await?;
	println!("Messages in block {block_hash:?} at index: {tx_indexes:?}");
	let block = client.blocks().at(BlockRef::from_hash(block_hash)).await?;
	let block_number = block.number();

	// 1. Generate merkle leafs for bridged messages.
	let indexed_leaves = messages_to_leaves(block_number, tx_indexes);

	// 2. Use Kate to get the proof and double-check it.
	check_query_data_proof_rpc(&client.rpc_methods(), block_hash, &indexed_leaves).await?;

	// 3. Test query_block len RPC.
	let block_len = client.rpc_methods().query_block_length(block_hash).await?;
	println!(
		"Test query_block_length RPC: cols={}, rows={}",
		block_len.cols.0, block_len.rows.0
	);
	assert_eq!(block_len.cols.0, 256);
	assert_eq!(block_len.rows.0, 256);

	Ok(())
}
