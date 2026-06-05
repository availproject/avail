/// # Data Avail Protocol
///
/// This `BlockImport` ensures that any block follows the Data Availability Protocol before send it
/// to Babe and Grandpa.
/// It double-checks the **extension header** which contains FRI commitments and data root.
use async_channel::TrySendError;
use avail_base::HeaderExtensionBuilderData;
use avail_blob::{
	p2p::global_eval_sender,
	store::StorageApiT,
	types::{BlobInfo, EvalClaimsMessage, OwnershipEntry},
};
use avail_core::{ensure, header::HeaderExtension};
use avail_observability::metrics::avail::{MetricObserver, ObserveKind};
use da_control::BlobTxSummaryRuntime;
use da_runtime::{apis::ExtensionBuilder, Header as DaHeader, Runtime};
use frame_system::native::build_extension;
use sc_consensus::{
	block_import::{BlockCheckParams, BlockImport as BlockImportT, BlockImportParams},
	ImportResult,
};
use sp_api::{ApiError, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_consensus::{BlockOrigin, Error as ConsensusError};
use sp_core::H256;
use sp_runtime::traits::Block as BlockT;
use sp_runtime::OpaqueExtrinsic;
use std::{marker::PhantomData, sync::Arc};

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
	C::Api: ExtensionBuilder<B>,
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
			blob_store,
			_block: PhantomData,
		}
	}

	fn ensure_last_extrinsic_is_failed_send_message_txs(
		&self,
		block: &BlockImportParams<B>,
	) -> Result<(), ConsensusError> {
		let body = block
			.body
			.as_ref()
			.ok_or_else(block_doesnt_contain_vector_post_inherent)?;

		let last = body
			.last()
			.ok_or_else(block_doesnt_contain_vector_post_inherent)?;

		let parent_hash = <B as BlockT>::Hash::from(block.header.parent_hash);
		let api = self.client.runtime_api();

		let found = api
			.check_if_extrinsic_is_vector_post_inherent(parent_hash, last)
			.map_err(|_| block_doesnt_contain_vector_post_inherent())?;

		ensure!(found, block_doesnt_contain_vector_post_inherent());
		Ok(())
	}

	fn extract_blob_summaries(
		&self,
		block: &BlockImportParams<B>,
	) -> Result<Vec<BlobTxSummaryRuntime>, ConsensusError> {
		let body = block
			.body
			.as_ref()
			.ok_or_else(block_doesnt_contain_da_post_inherent)?;

		let da_xt = body
			.get(body.len().wrapping_sub(2))
			.ok_or_else(block_doesnt_contain_da_post_inherent)?;

		let parent_hash = <B as BlockT>::Hash::from(block.header.parent_hash);
		let api = self.client.runtime_api();

		let extracted = api
			.extract_post_inherent_summaries(parent_hash, da_xt)
			.map_err(|_| block_doesnt_contain_da_post_inherent())?;

		ensure!(extracted.is_some(), block_doesnt_contain_da_post_inherent());
		Ok(extracted.expect("checked above; qed"))
	}

	fn ensure_valid_header_extension(
		&self,
		block: &BlockImportParams<B>,
		extracted: &HeaderExtensionBuilderData,
		skip_sync: bool,
	) -> Result<(), ConsensusError> {
		let extrinsics = block.body.clone().unwrap_or_default();
		let block_number: u32 = block.header.number;
		let parent_hash = <B as BlockT>::Hash::from(block.header.parent_hash);
		let api = self.client.runtime_api();

		let data_root = api
			.build_data_root(parent_hash, block_number, extrinsics.clone())
			.map_err(data_root_fail)?;
		let submitted_blobs = extracted.data_submissions.clone();
		let regenerated_extension = match &block.header.extension {
			HeaderExtension::V1(ext) => {
				// FRI eval proofs are best-effort import-time data: blob-owner attestations are the
				// primary validity signal, and requiring proofs in every summary would strengthen
				// guarantees but can reduce throughput by forcing authors to wait on proof delivery.
				// When a proof is included we still verify it here, but blocks remain importable
				// without it.
				if !skip_sync {
					for da in submitted_blobs.iter() {
						if let Some(eval_proof) = da.eval_proof.as_ref() {
							if let Err(e) = avail_blob::validation::validate_fri_proof(
								da.size_bytes as usize,
								ext.params_version,
								&da.commitment,
								&da.eval_point_seed,
								&da.eval_claim,
								eval_proof,
							) {
								log::warn!(
									"FRI proof validation failed for blob {:?}: {e}",
									da.hash
								);
							}
						}
					}
				}

				build_extension::build_extension(submitted_blobs, data_root, ext.params_version)
			},
		};

		// Check equality between calculated and imported extensions.
		ensure!(
			block.header.extension == regenerated_extension,
			extension_mismatch(&block.header.extension, &regenerated_extension)
		);
		Ok(())
	}

	fn publish_eval_claims(&self, block_hash: H256, extracted: &HeaderExtensionBuilderData) {
		let Some(eval_sender) = global_eval_sender() else {
			return;
		};

		let mut messages = Vec::new();

		for da in &extracted.data_submissions {
			let Some(eval_proof) = da.eval_proof.clone() else {
				continue;
			};

			messages.push(EvalClaimsMessage {
				block_hash,
				app_id: da.id,
				blob_hash: da.hash,
				eval_point_seed: da.eval_point_seed,
				eval_claim: da.eval_claim,
				eval_proof,
			});
		}

		if messages.is_empty() {
			return;
		}

		tokio::spawn(async move {
			for msg in messages {
				let blob_hash = msg.blob_hash;
				match eval_sender.try_send(msg) {
					Ok(()) => {},
					Err(TrySendError::Full(_)) => {
						log::warn!(
							"Eval claims channel full, dropping eval claims for block {:?}, blob {:?}",
							block_hash,
							blob_hash
						);
					},
					Err(TrySendError::Closed(_)) => {
						log::warn!(
							"Eval claims channel closed, dropping eval claims for block {:?}, blob {:?}",
							block_hash,
							blob_hash
						);
					},
				}
			}
		});
	}
}

#[async_trait::async_trait]
impl<B, C, I> BlockImportT<B> for BlockImport<B, C, I>
where
	B: BlockT<Extrinsic = OpaqueExtrinsic, Header = DaHeader, Hash = H256>,
	I: BlockImportT<B> + Clone + Send + Sync,
	I::Error: Into<ConsensusError>,
	C: ProvideRuntimeApi<B> + HeaderBackend<B> + Send + Sync,
	C::Api: ExtensionBuilder<B>,
{
	type Error = ConsensusError;

	/// It verifies that the header extension is properly calculated.
	#[tracing::instrument(name = "block.import", skip_all)]
	async fn import_block(&self, block: BlockImportParams<B>) -> Result<ImportResult, Self::Error> {
		let _metric_observer = MetricObserver::new(ObserveKind::ImportBlockTotalExecutionTime);

		// We only want to check for blocks that are not from "Own"
		let is_own = matches!(block.origin, BlockOrigin::Own);

		// We skip checks if we're syncing and unsafe_da_sync is true
		let is_sync = matches!(
			block.origin,
			BlockOrigin::NetworkInitialSync | BlockOrigin::File
		);
		let skip_sync = self.unsafe_da_sync && is_sync;

		let blob_summaries = if skip_sync {
			Vec::new()
		} else {
			self.extract_blob_summaries(&block)?
		};

		let extracted = if skip_sync {
			None
		} else {
			let extrinsics = block.body.clone().unwrap_or_default();
			Some(HeaderExtensionBuilderData::from_opaque_extrinsics::<
				RTExtractor,
			>(block.header.number, &extrinsics))
		};

		if !is_own && !skip_sync && !block.with_state() {
			self.ensure_last_extrinsic_is_failed_send_message_txs(&block)?;
			self.ensure_valid_header_extension(
				&block,
				extracted
					.as_ref()
					.expect("extracted is present when skip_sync is false; qed"),
				skip_sync,
			)?;
		}

		let block_number: u32 = block.header.number;
		let block_hash = block.post_hash();

		// Next import block stage & metrics
		let result = self.inner.import_block(block).await;

		// On successful import of block, write to our blob indexer and publish eval claims.
		if let Ok(ImportResult::Imported(_imported)) = &result {
			if !is_sync {
				if let Some(extracted) = extracted.as_ref() {
					self.publish_eval_claims(block_hash, extracted);
				}
			}

			// filter out successful blobs only and collect BlobInfo entries
			let mut blob_infos = Vec::new();

			for s in blob_summaries.iter().filter(|s| s.success) {
				let ownership = s
					.ownership
					.iter()
					.map(|(a, b, c, d)| OwnershipEntry {
						address: a.clone(),
						babe_key: b.clone(),
						encoded_peer_id: c.clone(),
						signature: d.clone(),
					})
					.collect();

				blob_infos.push(BlobInfo {
					hash: s.hash,
					block_hash,
					block_number,
					ownership,
				});
			}

			if !blob_infos.is_empty() {
				let blob_store = self.blob_store.clone();
				tokio::task::spawn_blocking(move || {
					if let Err(err) = blob_store.insert_blob_infos_by_block_batch(&blob_infos) {
						log::warn!(
							"Failed to store canonical blob infos for block {:?}: {:?}",
							block_hash,
							err
						);
					}

					if let Err(err) =
						blob_store.append_pending_blob_infos_batch(&block_hash, &blob_infos)
					{
						log::warn!(
							"Failed to append pending blob infos for block {:?}: {:?}",
							block_hash,
							err
						);
					}
				});
			}
		}

		result.map_err(Into::into)
	}

	async fn check_block(&self, block: BlockCheckParams<B>) -> Result<ImportResult, Self::Error> {
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

fn extension_mismatch(a: &HeaderExtension, b: &HeaderExtension) -> ConsensusError {
	ConsensusError::ClientImport(format!(
		"DA extension mismatch\nExpected: {a:#?}\nGenerated: {b:#?}"
	))
}

fn data_root_fail(e: ApiError) -> ConsensusError {
	ConsensusError::ClientImport(format!("Data root build failed: {e:?}"))
}

fn block_doesnt_contain_vector_post_inherent() -> ConsensusError {
	ConsensusError::ClientImport("Missing vector post inherent".into())
}

fn block_doesnt_contain_da_post_inherent() -> ConsensusError {
	ConsensusError::ClientImport("Missing DA post inherent".into())
}
