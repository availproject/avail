use avail_rust_client::{avail_rust_core::rpc::blob::submit_blob, prelude::*};
use codec::{Decode, Encode};
use da_commitment::build_da_commitments::{build_da_commitments, build_extended_commitments};
use kate::Seed;
use sp_core::keccak_256;
use sp_std::iter::repeat;
use std::{
	fs,
	path::{Path, PathBuf},
};

const COUNT: usize = 50;
const LEN: usize = 32 * 1024 * 1024;

#[derive(Encode, Decode)]
struct Bundle {
	blob: Vec<u8>,
	hash: H256,
	commitments: Vec<u8>,
	extended_commitments: Vec<u8>,
}

fn bundle_path(data_dir: &Path, mode: u8) -> PathBuf {
	data_dir.join(format!("mode{mode}.bin"))
}

fn write_bundles(data_dir: &Path, mode: u8, bundles: &[Bundle]) {
	let path = bundle_path(data_dir, mode);
	fs::create_dir_all(data_dir).expect("cannot create data dir");
	fs::write(&path, bundles.encode()).expect("failed to write bundles");
	println!("Recorded {} bundles → {}", bundles.len(), path.display());
}

fn read_bundles(data_dir: &Path, mode: u8) -> Vec<Bundle> {
	let path = bundle_path(data_dir, mode);
	let bytes =
		fs::read(&path).unwrap_or_else(|_| panic!("Missing bundles file: {}", path.display()));
	<Vec<Bundle>>::decode(&mut &bytes[..]).expect("failed to decode bundles")
}

fn build_bundles_for_mode(mode: u8) -> Vec<Bundle> {
	let byte = if mode == 1 {
		b'A'
	} else if mode == 2 {
		b'B'
	} else {
		b'C'
	};
	let mut out = Vec::with_capacity(COUNT);

	println!("---------- START Commitment generation (mode {mode}) ----------");
	for i in 0..COUNT {
		println!("-- gen {i} --");
		let blob: Vec<u8> = repeat(byte).take(LEN - i).collect::<Vec<u8>>();
		let blob_hash = H256::from(keccak_256(&blob));
		let commitments = build_da_commitments(blob.clone(), 1024, 4096, Seed::default());
		let extended_commitments = build_extended_commitments(commitments.clone()).unwrap();

		println!("blob len = {}", blob.len());
		println!("blob_hash = {:?}", blob_hash);
		println!("commitments len = {}", commitments.len());
		println!("extended_commitments len = {}", extended_commitments.len());

		out.push(Bundle {
			blob,
			hash: blob_hash,
			commitments,
			extended_commitments,
		});
	}

	out
}

pub async fn run() -> Result<(), ClientError> {
	let record = false;
	let data_dir = PathBuf::from("./e2e_data");
	let mode = 3;

	// Record commitments into a file
	if record {
		println!("---------- START Creating commitments ---------- ");
		for mode in 1u8..=3 {
			let bundles = build_bundles_for_mode(mode);
			write_bundles(&data_dir, mode, &bundles);
		}
		println!("Recording done.");
	}

	// Submit data
	println!("---------- START Submission ---------- ");
	let local_endpoint: &str = if mode == 1 {
		"http://127.0.0.1:9944"
	} else if mode == 2 {
		"http://127.0.0.1:9945"
	} else {
		"http://127.0.0.1:9946"
	};
	let client = Client::new(local_endpoint).await?;
	let signer = if mode == 1 {
		alice()
	} else if mode == 2 {
		bob()
	} else {
		charlie()
	};

	let nonce = client.nonce(&signer.account_id()).await?;
	println!("Nonce: {nonce}");

	let bundles = read_bundles(&data_dir, mode);

	for (i, b) in bundles.into_iter().enumerate() {
		println!(
			"[mode {mode}] Submit {i}: blob_len={}, hash={:?}, commitments_len={}, extended_len={}",
			b.blob.len(),
			b.hash,
			b.commitments.len(),
			b.extended_commitments.len()
		);

		let options = Options::new()
			.app_id((i % 5) as u32)
			.nonce(nonce + i as u32);
		let unsigned_tx = client.tx().data_availability().submit_blob_metadata(
			b.hash,
			b.blob.len() as u64,
			b.commitments,
			b.extended_commitments,
		);

		let tx = unsigned_tx.sign(&signer, options).await.unwrap().0.encode();

		if let Err(e) = submit_blob(&client.rpc_client, tx, b.blob).await {
			eprintln!("[mode {mode}] Error: {e}");
		}
		println!("---------- END Submission {i} ---------- ");
	}

	Ok(())
}
