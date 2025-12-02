// !!!!
// If the logic is changed in this file it will break Turing/Mainnet. Do not change it.
// If the logic is changed in avail-core it will break Turing/Mainnet as well. Do no change it.
// !!!!
#![cfg(feature = "std")]

use crate::limits::BlockLength;
use avail_base::header_extension::SubmittedData;
use avail_core::FriParamsVersion;
use avail_core::{
	header::extension as he, kate::COMMITMENT_SIZE, kate_commitment as kc, AppId, DataLookup,
};
use he::fri::{FriHeader, FriHeaderVersion};
use he::fri_v1::{FriBlobCommitment, HeaderExtension as FriV1HeaderExtension};
use he::{
	kzg::{KzgHeader, KzgHeaderVersion},
	HeaderExtension,
};
use sp_core::H256;
use std::vec::Vec;

#[cfg(feature = "testing-environment")]
use avail_base::testing_env::*;

/// Build a KZG v4 header extension
pub fn build_kzg_extension(
	submitted: Vec<SubmittedData>,
	data_root: H256,
	block_length: BlockLength,
	kzg_version: KzgHeaderVersion,
) -> HeaderExtension {
	// Blocks with non-DA extrinsics will have empty commitments
	if submitted.is_empty() {
		return HeaderExtension::get_empty_kzg(data_root, kzg_version);
	}

	let max_columns = block_length.cols.0 as usize;
	if max_columns == 0 {
		// Blocks with 0 columns will have empty commitments, ideally we should never reach here
		return HeaderExtension::get_empty_kzg(data_root, kzg_version);
	}

	let total_commitments: usize = submitted
		.iter()
		.map(|da_call| da_call.commitments.len())
		.sum();
	let mut commitment = Vec::with_capacity(total_commitments);

	let mut app_rows: Vec<(AppId, usize)> = Vec::with_capacity(submitted.len());

	for da_call in submitted.iter() {
		commitment.extend(da_call.commitments.clone());
		// As we have already correctness of commitments against data, we can safely assume that the commitments are correct
		let rows_taken = da_call.commitments.len() / COMMITMENT_SIZE;

		// Update app_rows
		app_rows.push((da_call.id, rows_taken));
	}

	let app_lookup = match DataLookup::from_id_and_len_iter(app_rows.into_iter()) {
		Ok(lookup) => lookup,
		Err(_) => return HeaderExtension::get_faulty_kzg(data_root, kzg_version),
	};

	let original_rows = app_lookup.len();
	let padded_rows = original_rows.next_power_of_two();

	// We can reduce the header size further letting the verification clients do this padding
	// since anyway they're extending the commitments.
	if padded_rows > original_rows {
		let (_, padded_row_commitment) =
			match kate::gridgen::core::get_pregenerated_row_and_commitment(max_columns) {
				Ok(result) => result,
				Err(e) => {
					log::error!("NODE_CRITICAL_ERROR_003 - A critical error has occured: {e:?}.");
					log::error!(
                        "NODE_CRITICAL_ERROR_003 - If you see this, please warn Avail team and raise an issue."
                    );
					return HeaderExtension::get_faulty_kzg(data_root, kzg_version);
				},
			};
		commitment = commitment
			.into_iter()
			.chain(
				std::iter::repeat(padded_row_commitment)
					.take((padded_rows - original_rows) as usize)
					.flatten(),
			)
			.collect();
	}

	let commitment = kc::v3::KateCommitment::new(
		padded_rows.try_into().unwrap_or_default(),
		max_columns.try_into().unwrap_or_default(),
		data_root,
		commitment,
	);

	// Build the v4 KZG header extension
	let v4_ext = he::v4::HeaderExtension {
		app_lookup,
		commitment,
	};

	HeaderExtension::Kzg(KzgHeader::from(v4_ext))
}

/// Build a Fri header extension (V1) from submitted blobs.
///
/// - We expect `submitted[i].commitments` to contain exactly one 32-byte Fri commitment
///   (Merkle root of the RS codewords). If any entry has len != 32, we log and return a *faulty* header.
pub fn build_fri_extension(
	submitted: Vec<SubmittedData>,
	data_root: H256,
	params_version: FriParamsVersion,
	fri_version: FriHeaderVersion,
) -> HeaderExtension {
	if submitted.is_empty() {
		return HeaderExtension::get_empty_fri(data_root, fri_version);
	}

	// Just do some sanitary check, as we cant actually check teh commitments here
	let fri_v1 = match fri_version {
		FriHeaderVersion::V1 => {
			let mut blobs: Vec<FriBlobCommitment> = Vec::with_capacity(submitted.len());

			for (idx, s) in submitted.into_iter().enumerate() {
				if s.commitments.len() != 32 {
					log::error!(
						"Fri header: expected 32-byte commitment for blob #{idx}, got {} bytes",
						s.commitments.len()
					);
					return HeaderExtension::get_faulty_fri(data_root, fri_version);
				}

				blobs.push(FriBlobCommitment {
					size_bytes: s.size_bytes,
					commitment: s.commitments,
				});
			}

			FriV1HeaderExtension {
				blobs,
				data_root,
				params_version,
			}
		},
	};

	HeaderExtension::Fri(FriHeader::V1(fri_v1))
}
