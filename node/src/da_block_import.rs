/// # Data Avail Protocol
///
/// This `BlockImport` ensures that any block follows the Data Availability Protocol before send it
/// to Babe and Grandpa.
/// It double-checks the **extension header** which contains the `Kate Commitment` and `Data
/// Root`.
use std::sync::Arc;

use avail_base::metrics::avail::ImportBlockMetrics;
use avail_core::{BlockLengthColumns, BlockLengthRows, OpaqueExtrinsic, BLOCK_CHUNK_SIZE};
use da_runtime::{
	apis::{DataAvailApi, ExtensionBuilder},
	Header as DaHeader,
};
use derive_more::Constructor;
use frame_support::ensure;
use frame_system::limits::BlockLength;
use sc_client_api::Backend;
use sc_consensus::{
	block_import::{BlockCheckParams, BlockImport as BlockImportT, BlockImportParams},
	ImportResult,
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_consensus::{BlockOrigin, Error as ConsensusError};
use sp_runtime::traits::Block as BlockT;

#[derive(Constructor)]
pub struct BlockImport<BE, C, I> {
	pub backend: Arc<BE>,
	pub client: Arc<C>,
	pub inner: I,
	// If true, it skips the DA block import check during sync only.
	pub unsafe_da_sync: bool,
}

impl<BE, C, I: Clone> Clone for BlockImport<BE, C, I> {
	fn clone(&self) -> Self {
		Self {
			backend: self.backend.clone(),
			client: self.client.clone(),
			inner: self.inner.clone(),
			unsafe_da_sync: self.unsafe_da_sync,
		}
	}
}

#[async_trait::async_trait]
impl<B, BE, C, I> BlockImportT<B> for BlockImport<BE, C, I>
where
	B: BlockT<Extrinsic = OpaqueExtrinsic, Header = DaHeader>,
	BE: Backend<B>,
	I: BlockImportT<B> + Clone + Send + Sync,
	I::Error: Into<ConsensusError>,
	C: ProvideRuntimeApi<B> + HeaderBackend<B> + Send + Sync,
	C::Api: DataAvailApi<B>,
	C::Api: ExtensionBuilder<B>,
{
	type Error = ConsensusError;
	type Transaction = <I as BlockImportT<B>>::Transaction;

	/// It verifies that header extension (Kate commitment & data root) is properly calculated.
	// TODO: Optimise this fn after testing the PoC
	async fn import_block(
		&mut self,
		block: BlockImportParams<B, Self::Transaction>,
	) -> Result<ImportResult, Self::Error> {
		let import_block_start = std::time::Instant::now();

		// We only want to check for blocks that are not from "Own"
		let is_own = matches!(block.origin, BlockOrigin::Own);

		// We skip checks if we're syncing and unsafe_da_sync is true
		let is_sync = matches!(
			block.origin,
			BlockOrigin::NetworkInitialSync | BlockOrigin::File
		);
		let skip_sync = self.unsafe_da_sync && is_sync;

		let should_verify = !is_own && !skip_sync;
		let no_extrinsics = vec![];
		let extrinsics = block.body.as_ref().unwrap_or(&no_extrinsics).clone();
		let best_hash = self.client.info().best_hash;
		let extension = &block.header.extension.clone();
		let block_number = block.header.number;
		// hash of the block being imported
		let import_block_hash = block.post_hash();
		// If there is any error in importing the block, No need to validate extension & return the error
		let import_block_res = self.inner.import_block(block).await.map_err(Into::into)?;
		if should_verify {
			let da_validation_result = (|| -> Result<(), ConsensusError> {
				let success_indices = self
					.client
					.runtime_api()
					.successfull_exrinsic_indices(import_block_hash)
					.map_err(|_e| {
						ConsensusError::ClientImport(format!(
							"Failed to fetch the successful indices"
						))
					})?;
				log::info!(target: "DA_IMPORT_BLOCK", "success_indices: {:?} at: {}", success_indices, import_block_hash);
				let successful_extrinsics: Vec<_> = success_indices
					.iter()
					.filter_map(|&i| extrinsics.get(i as usize).cloned())
					.collect();
				let data_root = self
					.client
					.runtime_api()
					.build_data_root(best_hash, successful_extrinsics.clone())
					.map_err(|e| {
						ConsensusError::ClientImport(format!(
							"Data root cannot be calculated: {e:?}"
						))
					})?;

				let block_len = BlockLength::with_normal_ratio(
					BlockLengthRows(extension.rows() as u32),
					BlockLengthColumns(extension.cols() as u32),
					BLOCK_CHUNK_SIZE,
					sp_runtime::Perbill::from_percent(90),
				)
				.expect("Valid BlockLength at genesis .qed");

				let generated_ext = self
					.client
					.runtime_api()
					.build_extension(
						best_hash,
						successful_extrinsics,
						data_root,
						block_len,
						block_number,
					)
					.map_err(|e| {
						ConsensusError::ClientImport(format!("Build extension fails due to: {e:?}"))
					})?;

				ensure!(
				extension == &generated_ext,
				ConsensusError::ClientImport(
					format!("DA Extension does NOT match\nExpected: {extension:#?}\nGenerated:{generated_ext:#?}"))
			);
				Ok(())
			})();
			if let Err(err) = da_validation_result {
				log::error!(
					target: "DA_IMPORT_BLOCK",
					"Error during da extension validation: {:?}, attempting to revert the block import", err
				);
				// Revert the import operation
				let _ = self.backend.revert(block_number, false);
				return Err(err.into());
			}
		}
		// Metrics
		ImportBlockMetrics::observe_total_execution_time(import_block_start.elapsed());

		Ok(import_block_res)
	}

	async fn check_block(
		&mut self,
		block: BlockCheckParams<B>,
	) -> Result<ImportResult, Self::Error> {
		self.inner.check_block(block).await.map_err(Into::into)
	}
}
