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
// use da_commitment::build_kzg_commitments::build_da_commitments;
// use da_commitment::build_fri_commitments::build_fri_da_commitment;
// use kate::Seed;
use sp_crypto_hashing::keccak_256;
use sp_std::iter::repeat;

const EXTRA_QUERY_DOMAIN_SEP: &[u8] = b"fri-extra-query-v1";

fn derive_extra_query_index(blob_hash: H256, commitment: &[u8], leaf_count: usize) -> usize {
	let mut preimage = Vec::with_capacity(32 + commitment.len() + EXTRA_QUERY_DOMAIN_SEP.len());
	preimage.extend_from_slice(blob_hash.as_bytes());
	preimage.extend_from_slice(commitment);
	preimage.extend_from_slice(EXTRA_QUERY_DOMAIN_SEP);
	let hash = keccak_256(&preimage);
	let mut arr = [0u8; 8];
	arr.copy_from_slice(&hash[..8]);
	(u64::from_le_bytes(arr) as usize) % leaf_count
}

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
	let mortality =
		MortalityOption::Full(Mortality::new(32, finalized_hash, finalized_header.number));

	let mut blobs: Vec<(
		Vec<u8>,
		H256,
		Vec<u8>,
		Option<[u8; 32]>,
		Option<[u8; 16]>,
		Option<Vec<u8>>,
	)> = Vec::new();
	println!("---------- START Commitments generation ---------- ");
	for i in 0..1 {
		println!("---------- START Commitment generation {i} ---------- ");
		let blob: Vec<u8> = repeat(byte).take(len - i).collect::<Vec<u8>>();
		let blob_hash = H256::from(keccak_256(&blob));
		// let commitments = build_da_commitments(&blob, 1024, 4096, Seed::default());
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
		let (terminate_codeword, query_prover, proof) = pcs
			.prove_with_openings::<B128>(
				packed.packed_mle.clone(),
				&ctx,
				&commit_output,
				&eval_point,
			)
			.expect("Failed to generate FRI openings");
		let log_batch_size = ctx.fri_params.log_batch_size();
		let leaf_count = 1usize
			<< (ctx
				.fri_params
				.rs_code()
				.log_len()
				.saturating_sub(log_batch_size));
		let extra_index = derive_extra_query_index(blob_hash, &commitments, leaf_count);
		let eval_proof = pcs
			.build_eval_proof_bundle(&proof, &terminate_codeword, &query_prover, extra_index)
			.expect("Failed to build eval proof bundle")
			.encode();
		println!("blob len = {:?}", blob.len());
		println!("blob_hash = {:?}", blob_hash);
		println!("commitments len = {:?}", commitments.len());
		println!("eval_proof len = {:?}", eval_proof.len());
		blobs.push((
			blob,
			blob_hash,
			commitments,
			Some(eval_point_seed),
			Some(eval_cliam_bytes),
			Some(eval_proof),
		));
	}
	for (i, (blob, hash, commitments, eval_point_seed, eval_claim, eval_proof)) in
		blobs.into_iter().enumerate()
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
			eval_proof,
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
