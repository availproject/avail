#![allow(dead_code)]

use avail_rust::{avail_rust_core::rpc::blob::submit_blob, prelude::*};
use da_commitment::build_da_commitments::build_da_commitments;
use kate::Seed;
use sp_crypto_hashing::keccak_256;
use sp_std::iter::repeat;

pub async fn run() -> Result<(), Error> {
	println!("---------- START Submission ---------- ");
	let len = 31 * 1024 * 1024;
	let mode = 1;

	let local_endpoint: &str = if mode == 1 {
		"http://127.0.0.1:9944"
	} else if mode == 2 {
		"http://127.0.0.1:9945"
	} else if mode == 3 {
		"http://127.0.0.1:9946"
	} else {
		"http://127.0.0.1:9947"
	};

	let client = Client::new(local_endpoint).await?;
	let signer = if mode == 1 {
		alice()
	} else if mode == 2 {
		bob()
	} else if mode == 3 {
		charlie()
	} else {
		dave()
	};
	let byte = if mode == 1 {
		b'A'
	} else if mode == 2 {
		b'B'
	} else if mode == 3 {
		b'C'
	} else {
		b'D'
	};

	let nonce = client.chain().account_nonce(signer.account_id()).await?;
	println!("Nonce: {nonce}");

	let mut blobs: Vec<(Vec<u8>, H256, Vec<u8>)> = Vec::new();
	println!("---------- START Commitments generation ---------- ");
	for i in 0..50 {
		println!("---------- START Commitment generation {i} ---------- ");
		let blob: Vec<u8> = repeat(byte).take(len - i).collect::<Vec<u8>>();
		let blob_hash = H256::from(keccak_256(&blob));
		let commitments = build_da_commitments(&blob, 1024, 4096, Seed::default());
		println!("blob len = {:?}", blob.len());
		println!("blob_hash = {:?}", blob_hash);
		println!("commitments len = {:?}", commitments.len());
		blobs.push((blob, blob_hash, commitments));
	}
	for (i, (blob, hash, commitments)) in blobs.into_iter().enumerate() {
		println!("---------- START Submission {i} ---------- ");
		let options = Options::default().nonce(nonce + i as u32);
		let unsigned_tx = client.tx().data_availability().submit_blob_metadata(
			5,
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

	Ok(())
}
