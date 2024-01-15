/// # Data Avail Protocol
///
/// This `BlockImport` ensures that any block follows the Data Availability Protocol before send it
/// to Babe and Grandpa.
/// It double-checks the **extension header** which contains the `Kate Commitment` and `Data
/// Root`.
use std::sync::Arc;

use avail_base::metrics::avail::ImportBlockMetrics;
use avail_core::{
	header::HeaderExtension, BlockLengthColumns, BlockLengthRows, OpaqueExtrinsic, BLOCK_CHUNK_SIZE,
};
use da_runtime::{
	apis::{DataAvailApi, ExtensionBuilder},
	Header as DaHeader,
};
use derive_more::Constructor;
use frame_support::ensure;
use frame_system::limits::BlockLength;
use frame_system::HeaderVersion;
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

		let extrinsics = block.body.as_ref().unwrap_or(&vec![]).clone();

		let best_hash = self.client.info().best_hash;
		let extension = &block.header.extension.clone();

		let (block_number, import_block_hash) = (block.header.number, block.post_hash());

		let import_block_res = self.inner.import_block(block).await.map_err(Into::into)?;

		// Nothing to verify. Let's do an early return.
		if !should_verify {
			// Metrics
			ImportBlockMetrics::observe_total_execution_time(import_block_start.elapsed());
			return Ok(import_block_res);
		}

		let success = build_data_root_and_extension(
			self,
			best_hash,
			import_block_hash,
			extrinsics,
			extension,
			block_number,
		);

		let mut result = Ok(import_block_res);
		if let Err(err) = success {
			log::error!(
				target: "DA_IMPORT_BLOCK",
				"Error during da extension validation: {:?}, attempting to revert the block import", err
			);

			// Revert the import operation
			let _ = self.backend.revert(block_number, false);
			result = Err(err);
		}

		// Metrics
		ImportBlockMetrics::observe_total_execution_time(import_block_start.elapsed());

		result
	}

	async fn check_block(
		&mut self,
		block: BlockCheckParams<B>,
	) -> Result<ImportResult, Self::Error> {
		self.inner.check_block(block).await.map_err(Into::into)
	}
}

fn build_data_root_and_extension<B, BE, C, I>(
	block_import: &BlockImport<BE, C, I>,
	best_hash: <B as BlockT>::Hash,
	import_block_hash: <B as BlockT>::Hash,
	extrinsics: Vec<OpaqueExtrinsic>,
	extension: &avail_core::header::HeaderExtension,
	block_number: u32,
) -> Result<(), ConsensusError>
where
	B: BlockT<Extrinsic = OpaqueExtrinsic, Header = DaHeader>,
	C: ProvideRuntimeApi<B> + HeaderBackend<B> + Send + Sync,
	C::Api: DataAvailApi<B>,
	C::Api: ExtensionBuilder<B>,
{
	use ConsensusError::ClientImport;

	let header_version = match extension {
		HeaderExtension::V1(_) => HeaderVersion::V1,
		HeaderExtension::V2(_) => HeaderVersion::V2,
	};

	let block_length = BlockLength::with_normal_ratio(
		BlockLengthRows(extension.rows() as u32),
		BlockLengthColumns(extension.cols() as u32),
		BLOCK_CHUNK_SIZE,
		sp_runtime::Perbill::from_percent(90),
	)
	.expect("Valid BlockLength at genesis .qed");

	let generated_ext = match header_version {
		HeaderVersion::V1 => {
			log::debug!(target: "DA_IMPORT_BLOCK", "V1 validation..");
			let data_root = block_import
				.client
				.runtime_api()
				.build_data_root(best_hash, extrinsics.clone())
				.map_err(|e| ClientImport(format!("Data root cannot be calculated: {e:?}")))?;

			block_import
				.client
				.runtime_api()
				.build_extension(best_hash, extrinsics, data_root, block_length, block_number)
				.map_err(|e| ClientImport(format!("Build extension fails due to: {e:?}")))?
		},
		_ => {
			log::debug!(target: "DA_IMPORT_BLOCK", "V2 validation..");
			let success_indices: Vec<u32> = block_import
				.client
				.runtime_api()
				.successful_extrinsic_indices(import_block_hash)
				.map_err(|_e| ClientImport("Failed to fetch the successful indices".into()))?;

			log::debug!(
				target: "DA_IMPORT_BLOCK",
				"success_indices: {:?} at: {}",
				success_indices,
				import_block_hash
			);

			let successful_extrinsics: Vec<_> = success_indices
				.iter()
				.filter_map(|&i| extrinsics.get(i as usize).cloned())
				.collect();

			let data_root = block_import
				.client
				.runtime_api()
				.build_data_root_v2(best_hash, successful_extrinsics.clone())
				.map_err(|e| ClientImport(format!("Data root cannot be calculated: {e:?}")))?;

			block_import
				.client
				.runtime_api()
				.build_versioned_extension(
					best_hash,
					successful_extrinsics,
					data_root,
					block_length,
					block_number,
					header_version,
				)
				.map_err(|e| ClientImport(format!("Build extension fails due to: {e:?}")))?
		},
	};

	ensure!(
		extension == &generated_ext,
		ClientImport(format!(
			"DA Extension does NOT match\nExpected: {extension:#?}\nGenerated:{generated_ext:#?}"
		))
	);

	Ok(())
}
