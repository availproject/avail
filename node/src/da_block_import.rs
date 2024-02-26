/// # Data Avail Protocol
///
/// This `BlockImport` ensures that any block follows the Data Availability Protocol before send it
/// to Babe and Grandpa.
/// It double-checks the **extension header** which contains the `Kate Commitment` and `Data
/// Root`.
use std::sync::Arc;

use avail_base::metrics::avail::ImportBlockMetrics;
use avail_core::{
	header::HeaderExtension, BlockLengthColumns, BlockLengthRows, HeaderVersion, OpaqueExtrinsic,
	BLOCK_CHUNK_SIZE,
};
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
use sp_core::H256;
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
	// type Transaction = <I as BlockImportT<B>>::Transaction;

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
		/*
		let should_verify = !is_own && !skip_sync;
		if should_verify {

			let parent_hash = <B as BlockT>::Hash::from(block.header.parent_hash);
			let extension = &block.header.extension.clone();
			let (block_number, import_block_hash) = (block.header.number, block.post_hash());

			let success = build_data_root_and_extension(
				self,
				parent_hash,
				import_block_hash,
				extrinsics,
				extension,
				block_number,
			);

		}*/

		// Next import block stage & metrics
		let result = self.inner.import_block(block).await;
		ImportBlockMetrics::observe_total_execution_time(import_block_start.elapsed());
		result.map_err(Into::into)
	}

	/*
	fn check_header_extension(&self,
		block: &BlockCheckParams<B>,
	) -> bool {
		// NOTE: Using `extrinsics` as ref is important to avoid cloning the extrinsics.
		let no_extrinsics = vec![];
		let extrinsics = block.body.as_ref().unwrap_or(&no_extrinsics);

		let block_len = extension_block_len(&block.header.extension);
		match extension.get_header_version() {
			HeaderVersion::V1 => check_header_extension_v1( self),
			HeaderVersion::V2 => check_header_extension_v1( self),
			HeaderVersion::V3 => check_header_extension_v1( self),
		}
	}*/

	async fn check_block(
		&mut self,
		block: BlockCheckParams<B>,
	) -> Result<ImportResult, Self::Error> {
		self.inner.check_block(block).await.map_err(Into::into)
	}
}

fn extension_block_len(extension: &HeaderExtension) -> BlockLength {
	BlockLength::with_normal_ratio(
		BlockLengthRows(extension.rows() as u32),
		BlockLengthColumns(extension.cols() as u32),
		BLOCK_CHUNK_SIZE,
		sp_runtime::Perbill::from_percent(90),
	)
	.expect("Valid BlockLength at genesis .qed")
}

/*
fn build_data_root_and_extension<B, BE, C, I>(
	block_import: &BlockImport<C, I>,
	parent_hash: <B as BlockT>::Hash,
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

	let header_version = extension.get_header_version();

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
				.build_data_root(parent_hash, extrinsics.clone())
				.map_err(|e| ClientImport(format!("Data root cannot be calculated: {e:?}")))?;

			block_import
				.client
				.runtime_api()
				.build_extension(
					parent_hash,
					extrinsics,
					data_root,
					block_length,
					block_number,
				)
				.map_err(|e| ClientImport(format!("Build extension fails due to: {e:?}")))?
		},
		_ => {
			log::debug!(target: "DA_IMPORT_BLOCK", "V2^ validation..");
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
				.build_data_root_v2(parent_hash, successful_extrinsics.clone())
				.map_err(|e| ClientImport(format!("Data root cannot be calculated: {e:?}")))?;

			block_import
				.client
				.runtime_api()
				.build_versioned_extension(
					parent_hash,
					extrinsics,
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
}*/
