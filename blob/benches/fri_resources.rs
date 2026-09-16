//! Native resource measurements for FRI and blob-summary operations.
//!
//! These measurements are intentionally not wired into runtime weights. Commitment and proof
//! work currently runs in the client. The summary decode measurement is provided to calibrate the
//! conservative per-byte runtime weight used by `submit_blob_txs_summary`.
//!
//! Run with, for example:
//! `FILE=/path/to/blob cargo bench -p avail-blob --bench fri_resources`

use avail_fri::{
	core::{FriBiniusPCS, FriCommitOutput, FriContext, B128},
	encoding::{BytesEncoder, PackedMLE},
	eval_utils::{derive_evaluation_point, eval_claim_to_bytes},
	FriParamsVersion,
};
use codec::{Decode, Encode};
use da_control::{BlobTxSummaryRuntime, BoundedBlobTxSummaries, BoundedEvalProof};
use divan::{black_box, Bencher};
use sp_core::H256;
use sp_io::hashing::keccak_256;

const PARAMS_VERSION: FriParamsVersion = FriParamsVersion::V0;
const EVAL_POINT_SEED: [u8; 32] = [7; 32];
const EXTRA_QUERY_DOMAIN_SEP: &[u8] = b"fri-extra-query-v1";

fn main() {
	divan::main();
}

fn read_blob() -> Vec<u8> {
	let path = std::env::var("FILE").expect(
		"set FILE to a blob path, e.g. FILE=/path/to/blob cargo bench -p avail-blob --bench fri_resources",
	);
	std::fs::read(path).expect("benchmark blob should be readable")
}

struct PreparedFri {
	blob_hash: H256,
	packed: PackedMLE<B128>,
	pcs: FriBiniusPCS,
	ctx: FriContext,
	commit_output: FriCommitOutput<B128>,
	eval_point: Vec<B128>,
	eval_claim: [u8; 16],
}

fn prepare(blob: &[u8]) -> PreparedFri {
	let encoder = BytesEncoder::<B128>::new();
	let packed = encoder
		.bytes_to_packed_mle(blob)
		.expect("blob should encode as a packed MLE");
	let pcs = FriBiniusPCS::new(PARAMS_VERSION.to_config(packed.total_n_vars));
	let ctx = pcs
		.initialize_fri_context::<B128>(packed.packed_mle.log_len())
		.expect("FRI context should initialize");
	let commit_output = pcs
		.commit(&packed.packed_mle, &ctx)
		.expect("commitment generation should succeed");
	let eval_point = derive_evaluation_point(EVAL_POINT_SEED, packed.total_n_vars);
	let eval_claim = pcs
		.calculate_evaluation_claim(&packed.packed_values, &eval_point)
		.map(eval_claim_to_bytes)
		.expect("evaluation claim generation should succeed");

	PreparedFri {
		blob_hash: H256::from(keccak_256(blob)),
		packed,
		pcs,
		ctx,
		commit_output,
		eval_point,
		eval_claim,
	}
}

fn extra_query_index(prepared: &PreparedFri, leaf_count: usize) -> usize {
	let mut preimage = Vec::with_capacity(32 + 32 + EXTRA_QUERY_DOMAIN_SEP.len());
	preimage.extend_from_slice(prepared.blob_hash.as_bytes());
	preimage.extend_from_slice(prepared.commit_output.commitment.as_slice());
	preimage.extend_from_slice(EXTRA_QUERY_DOMAIN_SEP);
	let hash = keccak_256(&preimage);
	let mut index = [0u8; 8];
	index.copy_from_slice(&hash[..8]);
	(u64::from_le_bytes(index) as usize) % leaf_count
}

fn generate_proof(prepared: &PreparedFri) -> Vec<u8> {
	let (terminate_codeword, query_prover, proof) = prepared
		.pcs
		.prove_with_openings::<B128>(
			prepared.packed.packed_mle.clone(),
			&prepared.ctx,
			&prepared.commit_output,
			&prepared.eval_point,
		)
		.expect("proof generation should succeed");
	let log_batch_size = prepared.ctx.fri_params.log_batch_size();
	let leaf_count = 1usize
		<< prepared
			.ctx
			.fri_params
			.rs_code()
			.log_len()
			.saturating_sub(log_batch_size);
	prepared
		.pcs
		.build_eval_proof_bundle(
			&proof,
			&terminate_codeword,
			&query_prover,
			extra_query_index(prepared, leaf_count),
		)
		.expect("proof bundle construction should succeed")
		.encode()
}

#[divan::bench(max_time = 3)]
fn commitment_generation(bencher: Bencher) {
	let blob = read_blob();
	bencher.bench_local(|| {
		black_box(
			da_commitment::build_fri_commitments::build_fri_da_commitment(
				black_box(&blob),
				PARAMS_VERSION,
			),
		)
	});
}

#[divan::bench(max_time = 3, sample_count = 10)]
fn proof_generation(bencher: Bencher) {
	let prepared = prepare(&read_blob());
	bencher.bench_local(|| black_box(generate_proof(black_box(&prepared))));
}

#[divan::bench(max_time = 3)]
fn proof_verification(bencher: Bencher) {
	let blob = read_blob();
	let prepared = prepare(&blob);
	let proof = generate_proof(&prepared);
	bencher.bench_local(|| {
		avail_blob::validation::validate_fri_proof(
			black_box(blob.len()),
			PARAMS_VERSION,
			black_box(prepared.commit_output.commitment.as_slice()),
			&EVAL_POINT_SEED,
			black_box(&prepared.eval_claim),
			black_box(&proof),
		)
		.expect("proof verification should succeed")
	});
}

#[divan::bench(max_time = 3)]
fn post_inherent_summary_scale_decode(bencher: Bencher) {
	let proof = vec![0u8; read_blob().len()];
	let summaries: BoundedBlobTxSummaries = vec![BlobTxSummaryRuntime {
		hash: H256::repeat_byte(1),
		tx_index: 0,
		success: true,
		reason: None,
		ownership: Vec::new().try_into().unwrap(),
		eval_proof: Some(BoundedEvalProof::try_from(proof).expect("proof exceeds runtime bound")),
	}]
	.try_into()
	.expect("single summary is bounded");
	let encoded = summaries.encode();

	bencher.bench_local(|| {
		let decoded = BoundedBlobTxSummaries::decode(&mut black_box(encoded.as_slice()))
			.expect("summary should decode");
		black_box(decoded)
	});
}
