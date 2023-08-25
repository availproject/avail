/// # Data Avail Protocol
///
/// This `BlockImport` ensures that any block follows the Data Availability Protocol before send it
/// to Babe and Grandpa.
/// It double-checks the **extension header** which contains the `Kate Commitment` and `Data
/// Root`.
use std::{collections::HashMap, sync::Arc};

use da_primitives::{BlockLengthColumns, BlockLengthRows, OpaqueExtrinsic, BLOCK_CHUNK_SIZE};
use da_runtime::{
	apis::{DataAvailApi, ExtensionBuilder},
	Header as DaHeader,
};
use frame_support::ensure;
use frame_system::limits::BlockLength;
use sc_consensus::{
	block_import::{BlockCheckParams, BlockImport as BlockImportT, BlockImportParams},
	ImportResult,
};
use sc_network::config::SyncMode;
use sc_service::Configuration;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_consensus::{CacheKeyId, Error as ConsensusError};
use sp_runtime::{generic::BlockId, traits::Block as BlockT};

pub struct BlockImport<C, I> {
	pub client: Arc<C>,
	pub inner: I,
	pub disable_kate_check: bool,
}

impl<C, I> BlockImport<C, I> {
	pub fn new(client: Arc<C>, config: &Configuration, inner: I) -> Self {
		let disable_kate_check = match config.network.sync_mode {
			SyncMode::Fast { .. } | SyncMode::Warp => true,
			_ => false,
		};

		Self {
			client,
			inner,
			disable_kate_check,
		}
	}
}

impl<C, I: Clone> Clone for BlockImport<C, I> {
	fn clone(&self) -> Self {
		Self {
			client: self.client.clone(),
			inner: self.inner.clone(),
			disable_kate_check: self.disable_kate_check,
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
	async fn import_block(
		&mut self,
		block: BlockImportParams<B, Self::Transaction>,
		new_cache: HashMap<CacheKeyId, Vec<u8>>,
	) -> Result<ImportResult, Self::Error> {
		if !self.disable_kate_check {
			self.check_header_extension(&block)?;
		}

		self.inner
			.import_block(block, new_cache)
			.await
			.map_err(Into::into)
	}

	/// # TODO
	/// - Check that `Runtime::System::BlockLenght` was not changed inside the block;
	async fn check_block(
		&mut self,
		block: BlockCheckParams<B>,
	) -> Result<ImportResult, Self::Error> {
		self.inner.check_block(block).await.map_err(Into::into)
	}
}

trait CheckHeaderExtension<B: BlockT> {
	type Transaction: Send + 'static;

	fn check_header_extension(
		&self,
		block: &BlockImportParams<B, Self::Transaction>,
	) -> Result<(), ConsensusError>;
}

impl<B, C, I> CheckHeaderExtension<B> for BlockImport<C, I>
where
	B: BlockT<Extrinsic = OpaqueExtrinsic, Header = DaHeader>,
	I: BlockImportT<B> + Clone + Send + Sync,
	C: ProvideRuntimeApi<B> + HeaderBackend<B> + Send + Sync,
	C::Api: DataAvailApi<B>,
	C::Api: ExtensionBuilder<B>,
{
	type Transaction = <I as BlockImportT<B>>::Transaction;

	// It verifies that header extension (Kate commitment & data root) is properly calculated.
	fn check_header_extension(
		&self,
		block: &BlockImportParams<B, Self::Transaction>,
	) -> Result<(), ConsensusError> {
		let no_extrinsics = vec![];
		let extrinsics = block.body.as_ref().unwrap_or(&no_extrinsics);
		let best_hash = self.client.info().best_hash;
		let block_id = BlockId::Hash(best_hash);

		let data_root = self
			.client
			.runtime_api()
			.build_data_root(&block_id, extrinsics.clone())
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
				&block_id,
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

		Ok(())
	}
}
