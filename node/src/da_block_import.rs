/// # Data Avail Protocol
///
/// This `BlockImport` ensures that any block follows the Data Availability Protocol before send it
/// to Babe and Grandpa.
/// It double-checks the **extension header** which contains the `Kate Commitment` and `Data
/// Root`.
use std::sync::Arc;

use avail_base::metrics::avail::ImportBlockMetrics;
use avail_core::{BlockLengthColumns, BlockLengthRows, OpaqueExtrinsic, BLOCK_CHUNK_SIZE};
// use codec::Decode;
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
// use sp_runtime::DigestItem;

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
		// if should_verify {
		// let no_extrinsics = vec![];
		// let extrinsics = block.body.as_ref().unwrap_or(&no_extrinsics);
		// let best_hash = self.client.info().best_hash;

		// // Extract Other type of digest from the logs
		// let other_digest = {
		// 	let digest = &block.header.digest;
		// 	let other_data: Vec<&[u8]> = digest
		// 		.logs
		// 		.iter()
		// 		.filter_map(|item| {
		// 			if let DigestItem::Other(data) = item {
		// 				Some(data.as_slice())
		// 			} else {
		// 				None
		// 			}
		// 		})
		// 		.collect();

		// 	match other_data.len() {
		// 		1 => Ok(other_data[0]),
		// 		0 => Err("No DigestItem::Other found in the digest"),
		// 		_ => Err("Multiple DigestItem::Other entries found in the digest"),
		// 	}
		// 	.map_err(|e| ConsensusError::Other(e.into()))?
		// };
		// let success_indices: Vec<u32> =
		// 	Decode::decode(&mut &other_digest[..]).unwrap_or_default();
		// log::info!(target: "DA_IMPORT_BLOCK", "success_indices: {:?}", success_indices);
		// let successful_extrinsics = success_indices
		// 	.iter()
		// 	.filter_map(|&i| extrinsics.get(i as usize).cloned())
		// 	.collect();
		// let data_root = self
		// 	.client
		// 	.runtime_api()
		// 	.build_data_root(best_hash, extrinsics)
		// 	.map_err(|e| {
		// 		ConsensusError::ClientImport(format!("Data root cannot be calculated: {e:?}"))
		// 	})?;

		// let extension = &block.header.extension;
		// let block_len = BlockLength::with_normal_ratio(
		// 	BlockLengthRows(extension.rows() as u32),
		// 	BlockLengthColumns(extension.cols() as u32),
		// 	BLOCK_CHUNK_SIZE,
		// 	sp_runtime::Perbill::from_percent(90),
		// )
		// .expect("Valid BlockLength at genesis .qed");

		// let generated_ext = self
		// 	.client
		// 	.runtime_api()
		// 	.build_extension(
		// 		best_hash,
		// 		extrinsics.clone(),
		// 		data_root,
		// 		block_len,
		// 		block.header.number,
		// 	)
		// 	.map_err(|e| {
		// 		ConsensusError::ClientImport(format!("Build extension fails due to: {e:?}"))
		// 	})?;

		// ensure!(
		// 	extension == &generated_ext,
		// 	ConsensusError::ClientImport(
		// 		format!("DA Extension does NOT match\nExpected: {extension:#?}\nGenerated:{generated_ext:#?}"))
		// );
		// }

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
			let success_indices = self
				.client
				.runtime_api()
				.successfull_exrinsic_indices(import_block_hash)
				.map_err(|_e| {
					ConsensusError::ClientImport(format!("Failed to fetch the successful indices"))
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
					ConsensusError::ClientImport(format!("Data root cannot be calculated: {e:?}"))
				})?;

			log::info!(target: "DA_IMPORT_BLOCK", "data_root computation done, building extension");
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
