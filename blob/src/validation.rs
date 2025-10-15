use crate::telemetry::TelemetryOperator;
use crate::traits::CommitmentQueueApiT;
use crate::{
	traits::{NonceCacheApiT, RuntimeApiT},
	utils::{extract_signer_and_nonce, CommitmentQueueMessage},
};
use codec::Decode;
use da_control::Call;
use da_runtime::RuntimeCall;
use da_runtime::UncheckedExtrinsic;
use kate::gridgen::core::PolynomialGrid;
use sp_core::keccak_256;
use sp_core::H256;
use sp_runtime::transaction_validity::TransactionSource;
use std::sync::Arc;

pub fn initial_validation(
	max_blob_size: usize,
	blob: &[u8],
	metadata: &[u8],
) -> Result<(H256, Vec<u8>), String> {
	if blob.len() > max_blob_size {
		return Err("blob is too big".into());
	}

	let mut metadata = metadata;
	let encoded_metadata_signed_transaction: UncheckedExtrinsic = Decode::decode(&mut metadata)
		.map_err(|_| String::from("failed to decode concrete metadata call"))?;
	let (provided_size, provided_blob_hash, provided_commitment) =
		match encoded_metadata_signed_transaction.function {
			RuntimeCall::DataAvailability(Call::submit_blob_metadata {
				size,
				blob_hash,
				commitment,
			}) => (size as usize, blob_hash, commitment),
			_ => {
				return Err("metadata extrinsic must be dataAvailability.submitBlobMetadata".into())
			},
		};

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

	Ok((blob_hash, provided_commitment))
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

pub async fn commitment_validation(
	hash: H256,
	size: usize,
	provided_commitment: &Vec<u8>,
	grid: PolynomialGrid,
	queue: &Arc<dyn CommitmentQueueApiT>,
	telemetry_operator: &TelemetryOperator,
) -> Result<(), String> {
	let (message, rx_comm) = CommitmentQueueMessage::new(hash, size, grid);
	if !queue.send(message) {
		// Need better error handling
		return Err("Commitment queue is full".into());
	}
	let commitment = match rx_comm.await {
		Ok(x) => x,
		Err(_) => {
			telemetry_operator.blob_dropped(Some(hash), true);
			return Err("Cannot compute commitment. :(  Channel is down".into());
		},
	};

	// Check comitment
	if !provided_commitment.eq(&commitment) {
		return Err("submitted blob commitment mismatch".into());
	}

	Ok(())
}
