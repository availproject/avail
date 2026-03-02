use crate::traits::CommitmentQueueApiT;
use crate::{
	traits::{NonceCacheApiT, RuntimeApiT},
	utils::{extract_signer_and_nonce, CommitmentQueueMessage},
};
use avail_fri::{
	core::{FriBiniusPCS, B128},
	encoding::{mle_dims_from_blob_size, BytesEncoder},
	eval_utils::{derive_evaluation_point, eval_claim_from_bytes},
	transcript_from_bytes, FriEvalProofBundle, FriParamsVersion,
};
use avail_observability::metrics::BlobMetrics;
use codec::{Decode, Encode};
// use da_commitment::build_fri_commitments::build_fri_da_commitment;
use da_control::Call;
use da_runtime::RuntimeCall;
use da_runtime::UncheckedExtrinsic;
use kate::gridgen::core::PolynomialGrid;
use sp_core::keccak_256;
use sp_core::H256;
use sp_runtime::transaction_validity::TransactionSource;
use std::sync::Arc;

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

pub fn initial_validation(
	max_blob_size: usize,
	blob: &[u8],
	metadata: &[u8],
) -> Result<
	(
		avail_core::AppId,
		H256,
		Vec<u8>,
		Option<[u8; 32]>,
		Option<[u8; 16]>,
	),
	String,
> {
	if blob.len() > max_blob_size {
		return Err("blob is too big".into());
	}

	let mut metadata = metadata;
	let encoded_metadata_signed_transaction: UncheckedExtrinsic = Decode::decode(&mut metadata)
		.map_err(|_| String::from("failed to decode concrete metadata call"))?;
	let (
		app_id,
		provided_size,
		provided_blob_hash,
		provided_commitment,
		eval_pont_seed,
		eval_claim,
	) = match encoded_metadata_signed_transaction.function {
		RuntimeCall::DataAvailability(Call::submit_blob_metadata {
			app_id,
			size,
			blob_hash,
			commitment,
			eval_point_seed,
			eval_claim,
		}) => (
			app_id,
			size as usize,
			blob_hash,
			commitment,
			eval_point_seed,
			eval_claim,
		),
		_ => return Err("metadata extrinsic must be dataAvailability.submitBlobMetadata".into()),
	};

	// TODO: do basic check like if the current commitment scheme is Fri, eval_point_seed and eval_claim must be present

	// Check size
	if provided_size != blob.len() {
		return Err(std::format!(
			"submit data length ({}) != blob length ({})",
			provided_size,
			blob.len()
		));
	}

	let blob_hash = H256::from(keccak_256(&blob));
	if provided_blob_hash != blob_hash {
		return Err(std::format!("submitted blob: {provided_blob_hash:?} does not correspond to generated blob {blob_hash:?}"));
	}

	Ok((
		app_id,
		blob_hash,
		provided_commitment,
		eval_pont_seed,
		eval_claim,
	))
}

pub fn tx_validation(
	at: H256,
	metadata: &[u8],
	min_transaction_validity: u64,
	max_transaction_validity: u64,
	runtime_client: &Arc<dyn RuntimeApiT>,
	nonce_cache: &Arc<dyn NonceCacheApiT>,
) -> Result<UncheckedExtrinsic, String> {
	let mut metadata = metadata;

	// --- a. Decode the opaque extrinsic ---------------------------------
	let opaque_tx: UncheckedExtrinsic = codec::Decode::decode(&mut metadata)
		.map_err(|_| String::from("failed to decode metadata extrinsic"))?;
	// --- b. Let the runtime validate it (signature, nonce, weight) ------
	let opaque_tx_clone = opaque_tx.clone();

	let validity =
		runtime_client.validate_transaction(at, TransactionSource::External, opaque_tx_clone, at);
	let validity = validity.map_err(|e| std::format!("runtime validate_transaction error: {e}"))?;
	let validity =
		validity.map_err(|e| format!("Metadata extrinsic rejected by runtime: {:?}", e))?;

	// --- c. Check also that transaction lifetime is above minimum tx lifetime so it does not expire. If validity is not correct, we reject the tx
	if validity.longevity < min_transaction_validity {
		return Err("signed transaction does not live for enough time".into());
	}
	if validity.longevity > max_transaction_validity {
		return Err("signed transaction lifetime is too long".into());
	}

	// --- d. Check the nonce in case it's a future transaction
	let Some((who, tx_nonce)) = extract_signer_and_nonce(&opaque_tx) else {
		return Err("signature payload not found (unsigned extrinsic)".into());
	};
	let onchain_nonce: u32 = runtime_client
		.account_nonce(at, who.clone())
		.map_err(|e| std::format!("failed to read on-chain nonce: {e:?}"))?;

	if let Err(reason) = nonce_cache.check(&who, onchain_nonce, tx_nonce) {
		return Err(std::format!("nonce check failed: {}", reason));
	}

	Ok(opaque_tx)
}

pub async fn validate_kzg_commitment(
	hash: H256,
	provided_commitment: &Vec<u8>,
	grid: PolynomialGrid,
	queue: &Arc<dyn CommitmentQueueApiT>,
) -> Result<(), String> {
	// Metrics
	let queue_capacity = queue.capacity();
	BlobMetrics::set_queue_capacity(queue_capacity as u64);
	crate::telemetry::BlobSubmission::queue_capacity(hash, queue_capacity);

	let (message, rx_comm) = CommitmentQueueMessage::new(hash, grid);
	if !queue.send(message) {
		// Need better error handling
		return Err("Commitment queue is full".into());
	}
	let commitment = match rx_comm.await {
		Ok(x) => x,
		Err(_) => {
			return Err("Cannot compute commitment. :(  Channel is down".into());
		},
	};

	// Check comitment
	if !provided_commitment.eq(&commitment) {
		return Err("submitted blob commitment mismatch".into());
	}

	Ok(())
}

/// Validate FRI commitment for the given blob and verify the evaluation proof for the given evaluation point and claim.
pub fn validate_fri_commitment(
	blob_hash: H256,
	blob: &[u8],
	provided_commitment: &[u8],
	params_version: FriParamsVersion,
	eval_point_seed: &[u8; 32],
	eval_claim: &[u8; 16],
) -> Result<Vec<u8>, String> {
	const FRI_COMMITMENT_SIZE: usize = 32;

	if provided_commitment.len() != FRI_COMMITMENT_SIZE {
		return Err(format!(
			"Fri commitment must be {} bytes, got {}",
			FRI_COMMITMENT_SIZE,
			provided_commitment.len()
		));
	}

	// let expected = build_fri_da_commitment(blob, params_version);
	// Encode bytes → multilinear extension over B128
	let encoder = BytesEncoder::<B128>::new();
	let packed = encoder
		.bytes_to_packed_mle(blob)
		.map_err(|e| e.to_string())?;

	let n_vars = packed.total_n_vars;

	// Map version + n_vars → concrete FriParamsConfig
	let cfg = params_version.to_config(n_vars);

	// Build PCS + FRI context
	let pcs = FriBiniusPCS::new(cfg);
	let ctx = pcs
		.initialize_fri_context::<B128>(packed.packed_mle.log_len())
		.map_err(|e| e.to_string())?;

	// Commit to the blob MLE: returns a 32-byte digest in `commitment`
	let commit_output = pcs
		.commit(&packed.packed_mle, &ctx)
		.map_err(|e| e.to_string())?;

	if commit_output.commitment.as_slice() != provided_commitment {
		return Err(format!("Fri commitment mismatch for blob {blob_hash:?}"));
	}

	let eval_point = derive_evaluation_point(*eval_point_seed, n_vars);
	let eval_claim = eval_claim_from_bytes(eval_claim)
		.map_err(|e| format!("Failed to deserialize evaluation claim: {}", e))?;

	let (terminate_codeword, query_prover, proof) = pcs
		.prove_with_openings::<B128>(packed.packed_mle.clone(), &ctx, &commit_output, &eval_point)
		.map_err(|e| e.to_string())?;
	let log_batch_size = ctx.fri_params.log_batch_size();
	let leaf_count = 1usize
		<< (ctx
			.fri_params
			.rs_code()
			.log_len()
			.saturating_sub(log_batch_size));
	let extra_index = derive_extra_query_index(blob_hash, provided_commitment, leaf_count);
	let bundle = pcs
		.build_eval_proof_bundle(&proof, &terminate_codeword, &query_prover, extra_index)
		.map_err(|e| e.to_string())?;

	pcs.verify_eval_proof_bundle(&bundle, eval_claim, &eval_point, &ctx)
		.map_err(|e| e.to_string())?;
	let proof_bytes = bundle.encode();
	Ok(proof_bytes)
}

pub fn validate_fri_proof(
	blob_size: usize,
	params_version: FriParamsVersion,
	expected_commitment: &[u8],
	eval_point_seed: &[u8; 32],
	eval_claim: &[u8; 16],
	eval_proof: &[u8],
) -> Result<(), String> {
	const FRI_COMMITMENT_SIZE: usize = 32;

	if expected_commitment.len() != FRI_COMMITMENT_SIZE {
		return Err(format!(
			"Fri commitment must be {} bytes, got {}",
			FRI_COMMITMENT_SIZE,
			expected_commitment.len()
		));
	}

	let bundle = FriEvalProofBundle::decode(&mut &eval_proof[..]).map_err(|e| e.to_string())?;
	let mut transcript = transcript_from_bytes(bundle.eval_proof.clone());
	let retrieved_commitment: [u8; 32] = transcript
		.message()
		.read()
		.map_err(|e| format!("Failed to read commitment from transcript: {}", e))?;

	if retrieved_commitment.as_slice() != expected_commitment {
		return Err("FRI proof commitment does not match blob commitment".into());
	}

	let (log_len, n_vars) = mle_dims_from_blob_size(blob_size);
	let cfg = params_version.to_config(n_vars);
	let pcs = FriBiniusPCS::new(cfg);
	let ctx = pcs
		.initialize_fri_context::<B128>(log_len)
		.map_err(|e| e.to_string())?;

	let eval_point = derive_evaluation_point(*eval_point_seed, n_vars);
	let eval_claim = eval_claim_from_bytes(eval_claim)
		.map_err(|e| format!("Failed to deserialize evaluation claim: {}", e))?;
	pcs.verify_eval_proof_bundle(&bundle, eval_claim, &eval_point, &ctx)
		.map_err(|e| e.to_string())
}
