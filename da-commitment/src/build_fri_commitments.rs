#![cfg(feature = "std")]

use anyhow::Result;
use avail_fri::{
	core::{FriBiniusPCS, B128},
	encoding::BytesEncoder,
};
use log;
use thiserror_no_std::Error;
// re-export for convineince
pub use avail_fri::core::FriParamsVersion;

/// Single Fri commitment for a DA blob (32-byte)
pub type FriDaCommitment = Vec<u8>;

#[derive(Error, Debug)]
pub enum FriDaCommitmentError {
	#[error("Bytes → packed MLE encoding failed: {0}")]
	EncodingFailed(String),
	#[error("FRI context initialization failed: {0}")]
	ContextInitFailed(String),
	#[error("PCS commitment failed: {0}")]
	CommitFailed(String),
}

fn build_fri_commitment_internal(
	data: &[u8],
	params_version: FriParamsVersion,
) -> Result<FriDaCommitment, FriDaCommitmentError> {
	// Encode bytes → multilinear extension over B128
	let encoder = BytesEncoder::<B128>::new();
	let packed = encoder
		.bytes_to_packed_mle(data)
		.map_err(|e| FriDaCommitmentError::EncodingFailed(e.to_string()))?;

	let n_vars = packed.total_n_vars;

	// Map version + n_vars → concrete FriParamsConfig
	let cfg = params_version.to_config(n_vars);

	// Build PCS + FRI context
	let pcs = FriBiniusPCS::new(cfg);
	let ctx = pcs
		.initialize_fri_context::<B128>(packed.packed_mle.log_len())
		.map_err(|e| FriDaCommitmentError::ContextInitFailed(e.to_string()))?;

	// Commit to the blob MLE: returns a 32-byte digest in `commitment`
	let commit_output = pcs
		.commit(&packed.packed_mle, &ctx)
		.map_err(|e| FriDaCommitmentError::CommitFailed(e.to_string()))?;

	Ok(commit_output.commitment)
}

/// Build commitment using Fri PCS with given version configuration
pub fn build_fri_da_commitment(data: &[u8], params_version: FriParamsVersion) -> FriDaCommitment {
	match build_fri_commitment_internal(data, params_version) {
		Ok(c) => c,
		Err(e) => {
			log::error!("Fri DA commitment generation failed: {:?}", e);
			FriDaCommitment::new()
		},
	}
}
