/// # Data Avail Protocol
///
/// This `BlockImport` ensures that any block follows the Data Availability Protocol before send it
/// to Babe and Grandpa.
/// It double-checks the **extension header** which contains the `Kate Commitment` and `Data
/// Root`.
use avail_base::{
	metrics::avail::{MetricObserver, ObserveKind},
	HeaderExtensionBuilderData,
};
use avail_core::{
	ensure,
	header::{extension as he, HeaderExtension},
	kate::COMMITMENT_SIZE,
	kate_commitment as kc, AppId, BlockLengthColumns, BlockLengthRows, DataLookup, HeaderVersion,
	OpaqueExtrinsic, BLOCK_CHUNK_SIZE,
};
use da_runtime::{
	apis::{DataAvailApi, ExtensionBuilder},
	Header as DaHeader, Runtime,
};
use frame_system::limits::BlockLength;

use sc_consensus::{
	block_import::{BlockCheckParams, BlockImport as BlockImportT, BlockImportParams},
	ImportResult,
};
use sp_api::{ApiError, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_consensus::{BlockOrigin, Error as ConsensusError};
use sp_core::H256;
use sp_runtime::traits::Block as BlockT;
use std::{marker::PhantomData, sync::Arc};

type RTExtractor = <Runtime as frame_system::Config>::HeaderExtensionDataFilter;

pub struct BlockImport<B, C, I> {
	client: Arc<C>,
	inner: I,
	// If true, it skips the DA block import check during sync only.
	unsafe_da_sync: bool,
	_block: PhantomData<B>,
}

impl<B, C, I> BlockImport<B, C, I>
where
	B: BlockT<Extrinsic = OpaqueExtrinsic, Header = DaHeader, Hash = H256>,
	I: BlockImportT<B> + Clone + Send + Sync,
	I::Error: Into<ConsensusError>,
	C: ProvideRuntimeApi<B> + HeaderBackend<B> + Send + Sync,
	C::Api: DataAvailApi<B> + ExtensionBuilder<B>,
{
	pub fn new(client: Arc<C>, inner: I, unsafe_da_sync: bool) -> Self {
		Self {
			client,
			inner,
			unsafe_da_sync,
			_block: PhantomData,
		}
	}

	fn ensure_last_extrinsic_is_failed_send_message_txs(
		&self,
		block: &BlockImportParams<B>,
	) -> Result<(), ConsensusError> {
		let err = block_doesnt_contain_vector_post_inherent();

		let maybe_body = block.body.as_ref();
		let Some(body) = maybe_body else {
			return Err(err);
		};

		let Some(last_extrinsic) = body.last() else {
			return Err(err);
		};

		let parent_hash = <B as BlockT>::Hash::from(block.header.parent_hash);
		let api = self.client.runtime_api();

		let Ok(found) = api.check_if_extrinsic_is_vector_post_inherent(parent_hash, last_extrinsic) else {
			return Err(err);
		};

		ensure!(found, err);

		Ok(())
	}

	fn ensure_before_last_extrinsic_is_blob_summary_tx(
		&self,
		block: &BlockImportParams<B>,
	) -> Result<(), ConsensusError> {
		let err = block_doesnt_contain_da_post_inherent();

		let maybe_body = block.body.as_ref();
		let Some(body) = maybe_body else {
			return Err(err);
		};

		let Some(da_summary_extrinsic) = body.get(body.len().wrapping_sub(2)) else {
			return Err(err);
		};

		let parent_hash = <B as BlockT>::Hash::from(block.header.parent_hash);
		let api = self.client.runtime_api();

		let Ok(found) = api.check_if_extrinsic_is_da_post_inherent(parent_hash, da_summary_extrinsic) else {
			return Err(err);
		};

		ensure!(found, err);

		Ok(())
	}

	fn ensure_valid_header_extension(
		&self,
		block: &BlockImportParams<B>,
	) -> Result<(), ConsensusError> {
		let block_len = extension_block_len(&block.header.extension);
		let extrinsics = || block.body.clone().unwrap_or_default();
		let block_number: u32 = block.header.number;
		let parent_hash = <B as BlockT>::Hash::from(block.header.parent_hash);
		let api = self.client.runtime_api();

		// Calculate data root and extension.
		let data_root = api
			.build_data_root(parent_hash, block_number, extrinsics())
			.map_err(data_root_fail)?;
		let version = block.header.extension.get_header_version();

		let extension = match version {
			HeaderVersion::V3 => api
				.build_extension(
					parent_hash,
					extrinsics(),
					data_root,
					block_len,
					block_number,
				)
				.map_err(build_ext_fail)?,
			HeaderVersion::V4 => build_extension_with_comms(
				extrinsics(),
				data_root,
				block_len,
				block_number,
				block.header.extension.get_header_version(),
			)?,
		};

		// Check equality between calculated and imported extensions.
		ensure!(
			block.header.extension == extension,
			extension_mismatch(&block.header.extension, &extension)
		);
		Ok(())
	}
}

#[async_trait::async_trait]
impl<B, C, I> BlockImportT<B> for BlockImport<B, C, I>
where
	B: BlockT<Extrinsic = OpaqueExtrinsic, Header = DaHeader, Hash = H256>,
	I: BlockImportT<B> + Clone + Send + Sync,
	I::Error: Into<ConsensusError>,
	C: ProvideRuntimeApi<B> + HeaderBackend<B> + Send + Sync,
	C::Api: DataAvailApi<B> + ExtensionBuilder<B>,
{
	type Error = ConsensusError;

	/// It verifies that header extension (Kate commitment & data root) is properly calculated.
	async fn import_block(
		&mut self,
		block: BlockImportParams<B>,
	) -> Result<ImportResult, Self::Error> {
		log::info!("IMPORTING BLOCK - Number: {:?} - Hash: {:?}", block.header.number, block.header.hash());

		let _metric_observer = MetricObserver::new(ObserveKind::ImportBlockTotalExecutionTime);

		// We only want to check for blocks that are not from "Own"
		let is_own = matches!(block.origin, BlockOrigin::Own);

		// We skip checks if we're syncing and unsafe_da_sync is true
		let is_sync = matches!(
			block.origin,
			BlockOrigin::NetworkInitialSync | BlockOrigin::File
		);
		let skip_sync = self.unsafe_da_sync && is_sync;
		if !is_own && !skip_sync && !block.with_state() {
			self.ensure_last_extrinsic_is_failed_send_message_txs(&block)?;
			self.ensure_before_last_extrinsic_is_blob_summary_tx(&block)?;
			self.ensure_valid_header_extension(&block)?; // TODO Blob do some kind of sampling like in block authorship ?
		}

		// Next import block stage & metrics
		let result = self.inner.import_block(block).await;
		result.map_err(Into::into)
	}

	async fn check_block(
		&mut self,
		block: BlockCheckParams<B>,
	) -> Result<ImportResult, Self::Error> {
		self.inner.check_block(block).await.map_err(Into::into)
	}
}

impl<B, C, I: Clone> Clone for BlockImport<B, C, I> {
	fn clone(&self) -> Self {
		Self {
			client: self.client.clone(),
			inner: self.inner.clone(),
			unsafe_da_sync: self.unsafe_da_sync,
			_block: PhantomData,
		}
	}
}

/// builds header extension by regenerating the commitments for DA txs
fn build_extension_with_comms(
	extrinsics: Vec<OpaqueExtrinsic>,
	data_root: H256,
	block_length: BlockLength,
	block_number: u32,
	version: HeaderVersion,
) -> Result<HeaderExtension, ConsensusError> {
	let app_extrinsics = HeaderExtensionBuilderData::from_opaque_extrinsics::<RTExtractor>(
		block_number,
		&extrinsics,
	)
	.data_submissions;

	// Blocks with non-DA extrinsics will have empty commitments
	if app_extrinsics.is_empty() {
		return Ok(HeaderExtension::get_empty_header(data_root, version));
	}

	let max_columns = block_length.cols.0 as usize;
	if max_columns == 0 {
		return Ok(HeaderExtension::get_empty_header(data_root, version));
	}
	let total_commitments_len: usize = app_extrinsics
		.iter()
		.map(|da_call| da_call.commitments.len())
		.sum();
	let mut commitment = Vec::with_capacity(total_commitments_len);

	let mut app_rows: Vec<(AppId, usize)> = Vec::with_capacity(app_extrinsics.len());

	for da_call in app_extrinsics.iter() {
		// TODO Blob - No need to check for commitments as they're checked before ?
		// For blob submission, the author has marked it as failed

		// let expected_commitments = &da_call.commitments;
		// let generated_commitments =
		// 	build_da_commitments(da_call.data.clone(), block_length.clone(), SEED);
		// // Early return if any of the DA commitments does not match.
		// ensure!(
		// 	expected_commitments == &generated_commitments,
		// 	commitments_mismatch(da_call.tx_index)
		// );
		commitment.extend(da_call.commitments.clone());
		let rows_taken = da_call.commitments.len() / COMMITMENT_SIZE;

		// Update app_rows
		app_rows.push((da_call.id, rows_taken));
	}

	let app_lookup = DataLookup::from_id_and_len_iter(app_rows.clone().into_iter())
		.map_err(|_| data_lookup_failed())?;
	let original_rows = app_lookup.len();
	let padded_rows = original_rows.next_power_of_two();
	if padded_rows > original_rows {
		let (_, padded_row_commitment) =
			kate::gridgen::core::get_pregenerated_row_and_commitment(max_columns)
				.map_err(|_| pregenerated_comms_failed())?;
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
	Ok(he::v4::HeaderExtension {
		app_lookup,
		commitment,
	}
	.into())
}

/// Calculate block length from `extension`.
fn extension_block_len(extension: &HeaderExtension) -> BlockLength {
	BlockLength::with_normal_ratio(
		BlockLengthRows(extension.rows() as u32),
		BlockLengthColumns(extension.cols() as u32),
		BLOCK_CHUNK_SIZE,
		sp_runtime::Perbill::from_percent(90),
	)
	.expect("Valid BlockLength at genesis .qed")
}

fn extension_mismatch(imported: &HeaderExtension, generated: &HeaderExtension) -> ConsensusError {
	let msg =
		format!("DA Extension does NOT match\nExpected: {imported:#?}\nGenerated:{generated:#?}");
	ConsensusError::ClientImport(msg)
}

// fn commitments_mismatch(tx_id: u32) -> ConsensusError {
// 	let msg = format!("DA Commitments does NOT match for tx_id: {tx_id}.");
// 	ConsensusError::ClientImport(msg)
// }

fn pregenerated_comms_failed() -> ConsensusError {
	let msg = format!("Failed to get pregenerated rows & commitments.");
	ConsensusError::ClientImport(msg)
}

fn data_lookup_failed() -> ConsensusError {
	let msg = format!("Failed to construct DataLookup.");
	ConsensusError::ClientImport(msg)
}

fn data_root_fail(e: ApiError) -> ConsensusError {
	let msg = format!("Data root cannot be calculated: {e:?}");
	ConsensusError::ClientImport(msg)
}

fn build_ext_fail(e: ApiError) -> ConsensusError {
	let msg = format!("Build extension fails due to: {e:?}");
	ConsensusError::ClientImport(msg)
}

fn block_doesnt_contain_vector_post_inherent() -> ConsensusError {
	let msg = "Block does not contain vector post inherent".to_string();
	ConsensusError::ClientImport(msg)
}

fn block_doesnt_contain_da_post_inherent() -> ConsensusError {
	let msg = "Block does not contain da post inherent".to_string();
	ConsensusError::ClientImport(msg)
}
