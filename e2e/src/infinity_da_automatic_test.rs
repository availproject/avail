#![allow(dead_code)]

use avail_rust::{
	avail_rust_core::rpc::blob::submit_blob,
	block_api::{BlockExtOptionsExpanded, BlockWithRawExt},
	prelude::*,
};
use da_commitment::build_da_commitments::build_da_commitments;
use kate::Seed;
use sp_crypto_hashing::keccak_256;
use sp_std::iter::repeat;

pub async fn run() -> Result<(), Error> {
	println!("---------- START Submission ---------- ");
	let len = 1 * 1024 * 1024;
	let client = Client::new(LOCAL_ENDPOINT).await?;
	let signer = alice();
	let byte = b'A';
	let nonce = client.chain().account_nonce(signer.account_id()).await?;

	let mut blobs: Vec<(Vec<u8>, H256, Vec<u8>)> = Vec::new();
	let nb_tx: u32 = 10;
	println!("---------- START Commitments generation ---------- ");
	for i in 0..(nb_tx as usize) {
		println!("---------- START Commitment generation {i} ---------- ");
		let blob: Vec<u8> = repeat(byte).take(len - i).collect::<Vec<u8>>();
		let blob_hash = H256::from(keccak_256(&blob));
		let commitments = build_da_commitments(&blob, 1024, 4096, Seed::default());
		println!("blob len = {:?}", blob.len());
		println!("blob_hash = {:?}", blob_hash);
		println!("commitments len = {:?}", commitments.len());
		blobs.push((blob, blob_hash, commitments));
	}

	let block_height_before = client.finalized().block_header().await?.number;
	let mut sub = Sub::new(client.clone());
	sub.use_best_block(true);
	sub.set_block_height(block_height_before);

	for (i, (blob, hash, commitments)) in blobs.into_iter().enumerate() {
		println!("---------- START Submission {i} ---------- ");
		let options = Options::new((i % 5) as u32).nonce(nonce + i as u32);
		let unsigned_tx = client.tx().data_availability().submit_blob_metadata(
			hash,
			blob.len() as u64,
			commitments,
		);

		let tx = unsigned_tx.sign(&signer, options).await.unwrap().encode();

		if let Err(e) = submit_blob(&client.rpc_client, &tx, &blob).await {
			println!("An error has occured: {e}");
		}
		println!("---------- END Submission {i} ---------- ");
	}

	let mut found_blob_count = 0;
	let mut block_searched = 0;

	loop {
		let block_ref = sub.next().await?;
		let block = BlockWithRawExt::new(client.clone(), block_ref.height);
		let regular_count = block
			.count(BlockExtOptionsExpanded::default())
			.await
			.unwrap();
		let count = (regular_count - 3).max(0);
		println!(
			"Searched in block {}, found {} blobs",
			block_ref.height, count
		);
		found_blob_count += count;
		block_searched += 1;
		if found_blob_count >= nb_tx as usize {
			println!("Successfully found all blobs");
			break;
		}
		if block_searched > 10 {
			println!("Failed to find blobs, stopped at {found_blob_count} blob(s)");
			break;
		}
	}

	Ok(())
}
