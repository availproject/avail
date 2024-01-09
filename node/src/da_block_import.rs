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
use sc_consensus::{
	block_import::{BlockCheckParams, BlockImport as BlockImportT, BlockImportParams},
	ImportResult,
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_consensus::{BlockOrigin, Error as ConsensusError};
use sp_runtime::traits::Block as BlockT;

#[derive(Constructor)]
pub struct BlockImport<C, I> {
	pub client: Arc<C>,
	pub inner: I,
	// If true, it skips the DA block import check during sync only.
	pub unsafe_da_sync: bool,
}

impl<C, I: Clone> Clone for BlockImport<C, I> {
	fn clone(&self) -> Self {
		Self {
			client: self.client.clone(),
			inner: self.inner.clone(),
			unsafe_da_sync: self.unsafe_da_sync,
		}
	}
}

#[async_trait::async_trait]
impl<B, C, I> BlockImportT<B> for BlockImport<C, I>
where
	B: BlockT<Extrinsic = OpaqueExtrinsic, Header = DaHeader>,
	I: BlockImportT<B> + Clone + Send + Sync,
	I::Error: Into<ConsensusError>,
	C: ProvideRuntimeApi<B> + HeaderBackend<B> + Send + Sync,
	C::Api: DataAvailApi<B>,
	C::Api: ExtensionBuilder<B>,
{
	type Error = ConsensusError;

	/// It verifies that header extension (Kate commitment & data root) is properly calculated.
	async fn import_block(
		&mut self,
		block: BlockImportParams<B>,
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
		if should_verify {
			let no_extrinsics = vec![];
			let extrinsics = block.body.as_ref().unwrap_or(&no_extrinsics);
			let best_hash = self.client.info().best_hash;

			let data_root = self
				.client
				.runtime_api()
				.build_data_root(best_hash, extrinsics.clone())
				.map_err(|e| {
					ConsensusError::ClientImport(format!("Data root cannot be calculated: {e:?}"))
				})?;

			let extension = &block.header.extension;
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
					extrinsics.clone(),
					data_root,
					block_len,
					block.header.number,
				)
				.map_err(|e| {
					ConsensusError::ClientImport(format!("Build extension fails due to: {e:?}"))
				})?;

			ensure!(
				extension == &generated_ext,
				ConsensusError::ClientImport(
					format!("DA Extension does NOT match\nExpected: {extension:#?}\nGenerated:{generated_ext:#?}"))
			);
		}

		let import_block_res = self.inner.import_block(block).await.map_err(Into::into);

		// Metrics
		ImportBlockMetrics::observe_total_execution_time(import_block_start.elapsed());

		import_block_res
	}

	async fn check_block(
		&mut self,
		block: BlockCheckParams<B>,
	) -> Result<ImportResult, Self::Error> {
		self.inner.check_block(block).await.map_err(Into::into)
	}
}
