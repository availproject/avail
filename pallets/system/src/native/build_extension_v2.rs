// !!!!
// If the logic is changed in this file it will break Turing/Mainnet. Do not change it.
// If the logic is changed in avail-core it will break Turing/Mainnet as well. Do no change it.
// !!!!
#![cfg(feature = "std")]

use crate::limits::BlockLength;
use avail_base::header_extension::SubmittedData;
use avail_core::{
	header::{extension as he, HeaderExtension},
	kate::COMMITMENT_SIZE,
	kate_commitment as kc, AppId, DataLookup, HeaderVersion,
};
use sp_core::H256;
use std::vec::Vec;

#[cfg(feature = "testing-environment")]
use avail_base::testing_env::*;

#[allow(unused_mut)]
pub fn build_extension_v4(
	mut submitted: Vec<SubmittedData>,
	data_root: H256,
	block_length: BlockLength,
	version: HeaderVersion,
) -> HeaderExtension {
	// Blocks with non-DA extrinsics will have empty commitments
	if submitted.is_empty() {
		return HeaderExtension::get_empty_header(data_root, version);
	}

	let max_columns = block_length.cols.0 as usize;
	if max_columns == 0 {
		// Blocks with 0 columns will have empty commitments, ideally we should never reach here
		return HeaderExtension::get_empty_header(data_root, version);
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
		Err(_) => return HeaderExtension::get_faulty_header(data_root, version),
	};

	let original_rows = app_lookup.len();
	let padded_rows = original_rows.next_power_of_two();

	// We can reduce the header size further letting the verification clients to do this padding since anyway they're extending the commitments
	if padded_rows > original_rows {
		let (_, padded_row_commitment) =
			match kate::gridgen::core::get_pregenerated_row_and_commitment(max_columns) {
				Ok(result) => result,
				Err(e) => {
					log::error!("NODE_CRITICAL_ERROR_003 - A critical error has occured: {e:?}.");
					log::error!("NODE_CRITICAL_ERROR_003 - If you see this, please warn Avail team and raise an issue.");
					return HeaderExtension::get_faulty_header(data_root, version);
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

	he::v4::HeaderExtension {
		app_lookup,
		commitment,
	}
	.into()
}
