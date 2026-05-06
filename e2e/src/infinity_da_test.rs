#![allow(dead_code)]

use avail_rust::{
	avail_rust_core::rpc::{blob::submit_blob, chain},
	prelude::*,
	transaction_options::{Mortality, MortalityOption},
};
// use avail_core::FriParamsVersion;
use avail_fri::{
	core::{FriBiniusPCS, B128},
	encoding::BytesEncoder,
	eval_utils::{derive_evaluation_point, derive_seed_from_inputs, eval_claim_to_bytes},
	FriParamsVersion,
};
use codec::Encode;
// use da_commitment::build_fri_commitments::build_fri_da_commitment;
use sp_crypto_hashing::keccak_256;
use sp_std::iter::repeat;

pub struct BabeRandomness;
impl StorageValue for BabeRandomness {
	type VALUE = [u8; 32];

	const PALLET_NAME: &str = "Babe";
	const STORAGE_NAME: &str = "Randomness";
}

pub async fn run() -> Result<(), Error> {
	println!("---------- START Submission ---------- ");
	let len = 31 * 1024;
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

	let client = Client::connect(local_endpoint).await?;
	println!("RPC endpoint: {local_endpoint}");
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

	let signer_account_id = signer.public_key().to_account_id();
	let nonce = client
		.chain()
		.account_nonce(signer_account_id.clone())
		.await?;
	println!("Nonce: {nonce}");
	let finalized_hash = chain::get_finalized_head(&client.rpc_client).await?;
	let finalized_header = chain::get_header(&client.rpc_client, Some(finalized_hash))
		.await?
		.expect("finalized header should be available");
	let mortality = MortalityOption::Full(Mortality::new(
		32,
		finalized_hash,
		finalized_header.number,
	));

	let mut blobs: Vec<(Vec<u8>, H256, Vec<u8>, Option<[u8; 32]>, Option<[u8; 16]>)> = Vec::new();
	println!("---------- START Commitments generation ---------- ");
	for i in 0..1 {
		println!("---------- START Commitment generation {i} ---------- ");
		let blob: Vec<u8> = repeat(byte).take(len - i).collect::<Vec<u8>>();
		let blob_hash = H256::from(keccak_256(&blob));
		// let commitments = build_fri_da_commitment(&blob, FriParamsVersion::V0);
		let params_version = FriParamsVersion::V0;
		// Encode bytes → multilinear extension over B128
		let encoder = BytesEncoder::<B128>::new();
		let packed = encoder
			.bytes_to_packed_mle(&blob)
			.expect("Failed to encode blob to packed MLE");

		let n_vars = packed.total_n_vars;

		// Map version + n_vars → concrete FriParamsConfig
		let cfg = params_version.to_config(n_vars);

		// Build PCS + FRI context
		let pcs = FriBiniusPCS::new(cfg);
		let ctx = pcs
			.initialize_fri_context::<B128>(packed.packed_mle.log_len())
			.expect("Failed to initialize FRI context");

		// Commit to the blob MLE: returns a 32-byte digest in `commitment`
		let commit_output = pcs
			.commit(&packed.packed_mle, &ctx)
			.expect("Failed to commit to blob MLE");
		let commitments = commit_output.commitment.to_vec();
		// fetch current epoch randomness from the chain & use it to derive eval point seed
		let rpc_client = &client.rpc_client;
		let babe_randomness = BabeRandomness::fetch(&rpc_client, None)
			.await?
			.expect("Babe Randomness should be available for every epoch except genesis era");
		let eval_point_seed = derive_seed_from_inputs(&babe_randomness, &blob_hash.0);
		let eval_point = derive_evaluation_point(eval_point_seed, n_vars);
		let eval_claim = pcs
			.calculate_evaluation_claim(&packed.packed_values, &eval_point)
			.expect("Failed to calculate evaluation claim");
		let eval_cliam_bytes = eval_claim_to_bytes(eval_claim);
		println!("blob len = {:?}", blob.len());
		println!("blob_hash = {:?}", blob_hash);
		println!("commitments len = {:?}", commitments.len());
		blobs.push((
			blob,
			blob_hash,
			commitments,
			Some(eval_point_seed),
			Some(eval_cliam_bytes),
		));
	}
	for (i, (blob, hash, commitments, eval_point_seed, eval_claim)) in blobs.into_iter().enumerate()
	{
		println!("---------- START Submission {i} ---------- ");
		let options = Options::default()
			.nonce(nonce + i as u32)
			.mortality(mortality);
		let unsigned_tx = client.tx().data_availability().submit_blob_metadata(
			5,
			hash,
			blob.len() as u64,
			commitments,
			eval_point_seed,
			eval_claim,
		);

		let signed = match unsigned_tx.sign(&signer, options).await {
			Ok(v) => v,
			Err(e) => {
				eprintln!("sign() failed: {e:?}");
				return Err(e);
			},
		};
		let tx = signed.encode();

		if let Err(e) = submit_blob(&client.rpc_client, &tx, &blob).await {
			println!("An error has occured: {e}");
		}
		println!("---------- END Submission {i} ---------- ");
	}

	Ok(())
}
