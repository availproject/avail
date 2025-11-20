/// # Data Avail Protocol
///
/// This `BlockImport` ensures that any block follows the Data Availability Protocol before send it
/// to Babe and Grandpa.
/// It double-checks the **extension header** which contains the `Kate Commitment` and `Data
/// Root`.
use avail_base::HeaderExtensionBuilderData;
use avail_blob::{
	store::StorageApiT,
	types::{BlobInfo, OwnershipEntry},
};
use avail_core::{
	ensure,
	header::{extension as he, HeaderExtension},
	kate::COMMITMENT_SIZE,
	kate_commitment as kc, AppId, BlockLengthColumns, BlockLengthRows, DataLookup, HeaderVersion,
	OpaqueExtrinsic, BLOCK_CHUNK_SIZE,
};
use avail_observability::metrics::avail::{MetricObserver, ObserveKind};
use da_control::BlobTxSummaryRuntime;
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
use std::{marker::PhantomData, sync::Arc, time::Instant};

type RTExtractor = <Runtime as frame_system::Config>::HeaderExtensionDataFilter;

pub struct BlockImport<B, C, I> {
	client: Arc<C>,
	inner: I,
	// If true, it skips the DA block import check during sync only.
	unsafe_da_sync: bool,
	// External blob DB handle:
	blob_store: Arc<dyn StorageApiT>,
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
	pub fn new(
		client: Arc<C>,
		inner: I,
		unsafe_da_sync: bool,
		blob_store: Arc<dyn StorageApiT>,
	) -> Self {
		Self {
			client,
			inner,
			unsafe_da_sync,
			_block: PhantomData,
			blob_store,
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

		let Ok(found) = api.check_if_extrinsic_is_vector_post_inherent(parent_hash, last_extrinsic)
		else {
			return Err(err);
		};

		ensure!(found, err);

		Ok(())
	}

	// Now this ALWAYS runs and returns the decoded summaries (if any)
	// by calling the new runtime API that both checks and decodes the
	// DA post-inherent.
	fn ensure_before_last_extrinsic_is_blob_summary_tx(
		&self,
		block: &BlockImportParams<B>,
	) -> Result<Vec<BlobTxSummaryRuntime>, ConsensusError> {
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

		let Ok(extracted) = api.extract_post_inherent_summaries(parent_hash, da_summary_extrinsic)
		else {
			return Err(err);
		};

		ensure!(extracted.is_some(), err);

		Ok(extracted.expect("Checked above; qed"))
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
			// Since V3 has AppExtrinsics which is derived from the AppId SignedExtension, We cant support it GOING FORWARD
			HeaderVersion::V3 => todo!(),
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
		let _metric_observer = MetricObserver::new(ObserveKind::ImportBlockTotalExecutionTime);

		// We only want to check for blocks that are not from "Own"
		let is_own = matches!(block.origin, BlockOrigin::Own);

		// We skip checks if we're syncing and unsafe_da_sync is true
		let is_sync = matches!(
			block.origin,
			BlockOrigin::NetworkInitialSync | BlockOrigin::File
		);
		let skip_sync = self.unsafe_da_sync && is_sync;

		// Always extract blob summaries (if any) from DA post-inherent extrinsic.
		// we know that it will add small overheasd but simplifies the code flow.
		let pre_extracted_summaries =
			self.ensure_before_last_extrinsic_is_blob_summary_tx(&block)?;

		if !is_own && !skip_sync && !block.with_state() {
			self.ensure_last_extrinsic_is_failed_send_message_txs(&block)?;
			self.ensure_valid_header_extension(&block)?;
		}

		let candidate_block_number: u32 = block.header.number;
		let candidate_block_hash = block.post_hash();

		// Next import block stage & metrics
		let result = self.inner.import_block(block).await;

		// On successful import of block, write to our blob indexer.
		if let Ok(ImportResult::Imported(_imported)) = &result {
			// filter out successful blobs only and collect BlobInfo entries
			let mut blob_infos: Vec<BlobInfo> = Vec::new();

			for s in pre_extracted_summaries.iter().filter(|s| s.success) {
				let ownership_entries: Vec<OwnershipEntry> = s
					.ownership
					.iter()
					.map(|(a, b, c, d)| OwnershipEntry {
						address: a.clone(),
						babe_key: b.clone(),
						encoded_peer_id: c.clone(),
						signature: d.clone(),
					})
					.collect();

				let blob_info = BlobInfo {
					hash: s.hash,
					block_hash: candidate_block_hash,
					block_number: candidate_block_number,
					ownership: ownership_entries,
				};

				blob_infos.push(blob_info);
			}

			// If there are none, skip DB work and logs
			if blob_infos.is_empty() {
				log::debug!(
					"No successful blob summaries to write for block #{}/{}",
					candidate_block_number,
					candidate_block_hash
				);
			} else {
				// Batch insert per-block history (blob_by_hash_block + blob_by_block)
				let write_start = std::time::Instant::now();
				let mut written_history = 0usize;
				if let Err(e) = self
					.blob_store
					.insert_blob_infos_by_block_batch(&blob_infos)
				{
					log::warn!(
						"Failed batch insert_blob_infos_by_block_batch for block #{}/{}: {}",
						candidate_block_number,
						candidate_block_hash,
						e
					);
				} else {
					written_history = blob_infos.len();
				}
				let history_ns = write_start.elapsed().as_nanos();

				// Append pending pending_by_block
				let pending_start = std::time::Instant::now();
				let mut written_pending = 0usize;
				if let Err(e) = self
					.blob_store
					.append_pending_blob_infos_batch(&candidate_block_hash, &blob_infos)
				{
					log::warn!(
						"Failed append_pending_blob_infos_batch for block #{}/{}: {}",
						candidate_block_number,
						candidate_block_hash,
						e
					);
				} else {
					written_pending = blob_infos.len();
				}
				let pending_ns = pending_start.elapsed().as_nanos();

				// Logging aggregated stats
				if written_history > 0 || written_pending > 0 {
					let total_written = std::cmp::max(written_history, written_pending);
					let total_ns = history_ns + pending_ns;
					let avg_us = (total_ns as f64 / total_written as f64) / 1_000.0_f64;
					log::info!(
						"⏱️ Persisted {} BlobInfo entries for block #{}/{} (history={}, pending={}), total_time = {} ms, avg = {:.3} µs",
						total_written,
						candidate_block_number,
						candidate_block_hash,
						written_history,
						written_pending,
						(total_ns as f64) / 1_000_000.0_f64,
						avg_us
					);
				} else {
					log::warn!(
						"Attempted writes for blob_info in block #{}/{} took total {} ms (all failed)",
						candidate_block_number,
						candidate_block_hash,
						((history_ns + pending_ns) as f64) / 1_000_000.0_f64
					);
				}
			}
		}

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
			blob_store: self.blob_store.clone(),
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
	let timer_total = Instant::now();
	let timer_app_ext = Instant::now();
	let app_extrinsics = HeaderExtensionBuilderData::from_opaque_extrinsics::<RTExtractor>(
		block_number,
		&extrinsics,
		block_length.cols.0,
		block_length.rows.0,
	)
	.data_submissions;
	log::info!(
		"⏱️ Extracting app extrinsics took {:?}",
		timer_app_ext.elapsed()
	);
	log::info!("Ext length: {}", extrinsics.len());

	// Blocks with non-DA extrinsics will have empty commitments
	if app_extrinsics.is_empty() {
		log::info!(
			"✅ No DA extrinsics, returning empty header. Total time: {:?}",
			timer_total.elapsed()
		);
		return Ok(HeaderExtension::get_empty_header(data_root, version));
	}

	let max_columns = block_length.cols.0 as usize;
	if max_columns == 0 {
		log::info!(
			"⚠️ Max columns = 0, returning empty header. Total time: {:?}",
			timer_total.elapsed()
		);
		return Ok(HeaderExtension::get_empty_header(data_root, version));
	}

	let timer_commitment_prep = Instant::now();
	let total_commitments_len: usize = app_extrinsics
		.iter()
		.map(|da_call| da_call.commitments.len())
		.sum();
	let mut commitment = Vec::with_capacity(total_commitments_len);

	let mut app_rows: Vec<(AppId, usize)> = Vec::with_capacity(app_extrinsics.len());

	for da_call in app_extrinsics.iter() {
		// Commitments from blob submission where checked
		// Commitments from regular submit data are computed by the node
		commitment.extend(da_call.commitments.clone());
		let rows_taken = da_call.commitments.len() / COMMITMENT_SIZE;

		// Update app_rows
		app_rows.push((da_call.id, rows_taken));
	}
	log::info!(
		"⏱️ Collecting commitments + app_rows took {:?}",
		timer_commitment_prep.elapsed()
	);

	let timer_lookup = Instant::now();
	let app_lookup = DataLookup::from_id_and_len_iter(app_rows.clone().into_iter())
		.map_err(|_| data_lookup_failed())?;
	log::info!("⏱️ Building DataLookup took {:?}", timer_lookup.elapsed());

	let timer_padding = Instant::now();
	let original_rows = app_lookup.len();
	let padded_rows = original_rows.next_power_of_two();
	if padded_rows > original_rows {
		let (_, padded_row_commitment) =
			kate::gridgen::core::get_pregenerated_row_and_commitment(max_columns)
				.map_err(|_| pregenerated_comms_failed())?;
		commitment = commitment
			.into_iter()
			.chain(
				std::iter::repeat_n(
					padded_row_commitment,
					(padded_rows - original_rows) as usize,
				)
				.flatten(),
			)
			.collect();
	}
	log::info!("⏱️ Padding commitments took {:?}", timer_padding.elapsed());

	let timer_kate = Instant::now();
	let commitment = kc::v3::KateCommitment::new(
		padded_rows.try_into().unwrap_or_default(),
		max_columns.try_into().unwrap_or_default(),
		data_root,
		commitment,
	);
	log::info!("⏱️ Building KateCommitment took {:?}", timer_kate.elapsed());

	log::info!(
		"✅ Finished build_extension_with_comms in {:?}",
		timer_total.elapsed()
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
	let msg = "Failed to get pregenerated rows & commitments.".to_string();
	ConsensusError::ClientImport(msg)
}

fn data_lookup_failed() -> ConsensusError {
	let msg = "Failed to construct DataLookup.".to_string();
	ConsensusError::ClientImport(msg)
}

fn data_root_fail(e: ApiError) -> ConsensusError {
	let msg = format!("Data root cannot be calculated: {e:?}");
	ConsensusError::ClientImport(msg)
}

// fn build_ext_fail(e: ApiError) -> ConsensusError {
// 	let msg = format!("Build extension fails due to: {e:?}");
// 	ConsensusError::ClientImport(msg)
// }

fn block_doesnt_contain_vector_post_inherent() -> ConsensusError {
	let msg = "Block does not contain vector post inherent".to_string();
	ConsensusError::ClientImport(msg)
}

fn block_doesnt_contain_da_post_inherent() -> ConsensusError {
	let msg = "Block does not contain da post inherent".to_string();
	ConsensusError::ClientImport(msg)
}
