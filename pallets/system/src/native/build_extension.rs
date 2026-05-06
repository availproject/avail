#![cfg(feature = "std")]

use avail_base::header_extension::SubmittedData;
use avail_core::header::extension::fri_v1::{
	FriBlobCommitment, HeaderExtension as V1HeaderExtension,
};
use avail_core::{header::HeaderExtension, FriParamsVersion};
use sp_core::H256;
use std::vec::Vec;

#[cfg(feature = "testing-environment")]
use avail_base::testing_env::*;

/// Build a FRI V1 header extension from submitted blobs.
///
/// - We expect `submitted[i].commitment` to contain exactly one 32-byte Fri commitment
///   (Merkle root of the RS codewords). If any entry has len != 32, we log and return a *faulty* header.
pub fn build_extension(
	submitted: Vec<SubmittedData>,
	data_root: H256,
	params_version: FriParamsVersion,
) -> HeaderExtension {
	if submitted.is_empty() {
		return HeaderExtension::get_empty_header(data_root);
	}

	// Just do some sanitary check, as we cant actually check teh commitments here
	let mut blobs: Vec<FriBlobCommitment> = Vec::with_capacity(submitted.len());

	for (idx, s) in submitted.into_iter().enumerate() {
		if s.commitment.len() != 32 {
			log::error!(
				"Fri header: expected 32-byte commitment for blob #{idx}, got {} bytes",
				s.commitment.len()
			);
			return HeaderExtension::get_faulty_header(data_root);
		}

		blobs.push(FriBlobCommitment {
			blob_hash: s.hash,
			size_bytes: s.size_bytes,
			commitment: s.commitment,
		});
	}

	HeaderExtension::V1(V1HeaderExtension {
		blobs,
		data_root,
		params_version,
	})
}
