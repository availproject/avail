use crate::traits::CommitmentQueueApiT;
use crate::types::{BlobEvalData, BlobInfo, BlobSummary, FriData};
use crate::utils::{designated_prover_index, get_babe_randomness_key, get_my_validator_id};
use crate::validation::{
	initial_validation, tx_validation, validate_fri_commitment, validate_kzg_commitment,
};
use crate::{
	nonce_cache::NonceCache,
	p2p::BlobHandle,
	send_blob_query_request,
	store::StorageApiT,
	traits::{
		BackendApiT, BackendClient, ExternalitiesT, NonceCacheApiT, RealExternalities, RuntimeApiT,
		RuntimeClient, TransactionPoolApiT, TransactionPoolClient,
	},
	types::{Blob, BlobMetadata, BlobNotification, BlobReceived, CompressedBlob, OwnershipEntry},
	utils::{
		build_signature_payload, extract_signer_and_nonce, generate_base_index,
		get_dynamic_blocklength_key, get_my_validator_public_account, get_validator_per_blob_inner,
		sign_blob_data, validators_for_blob, B64Param, CommitmentQueue,
	},
	MAX_RPC_RETRIES,
};
use avail_base::HeaderExtensionBuilderData;
use avail_core::header::extension::CommitmentScheme;
use avail_core::{AppId, DataProof};
use avail_fri::eval_utils::derive_seed_from_inputs;
use avail_fri::{
	transcript_to_bytes, BytesEncoder, FriBiniusPCS, FriParamsVersion, SamplingProof, B128,
};
use avail_observability::metrics::BlobMetrics;
use codec::{Decode, Encode};
use da_commitment::build_kzg_commitments::build_polynomial_grid;
use da_control::{BlobRuntimeParameters, Call};
use da_runtime::apis::{BlobApi as _, KateApi};
use da_runtime::{Runtime, RuntimeCall, UncheckedExtrinsic};
use frame_system::limits::BlockLength;
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::ErrorObject,
};
use parking_lot::Mutex;
use sc_client_api::{BlockBackend, HeaderBackend, StateBackend};
use sc_network::NetworkStateInfo;
use sc_network::PeerId;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_core::H256;
use sp_runtime::{
	traits::{Block as BlockT, HashingFor, Header as HeaderT},
	transaction_validity::TransactionSource,
	AccountId32, SaturatedConversion,
};
use std::collections::HashMap;
use std::{
	marker::{PhantomData, Sync},
	str::FromStr,
	sync::Arc,
};
use tokio::task;
use tracing::Instrument;

/// Cached FRI state for a blob at a given block
#[derive(Clone)]
struct FriSamplingCacheEntry {
	commit_output: Arc<avail_fri::FriCommitOutput<B128>>,
	pcs: Arc<avail_fri::FriBiniusPCS>,
	log_batch_size: usize,
	leaf_count: usize,
}

// block_hash, blob_hash
type FriSamplingCacheKey = (H256, H256);

type RTExtractor = <Runtime as frame_system::Config>::HeaderExtensionDataFilter;

pub enum Error {
	BlobError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::BlobError => 1,
		}
	}
}

macro_rules! internal_err {
    ($($arg:tt)*) => {{
        ErrorObject::owned(
            Error::BlobError.into(),
            format!($($arg)*),
            None::<()>
        )
    }}
}

#[rpc(client, server)]
pub trait BlobApi<Block>
where
	Block: BlockT,
{
	/// Submits a data blob and its metadata transaction to the network.
	///
	/// This RPC performs the full client-side submission flow:
	/// - validates the metadata transaction
	/// - validates the blob size and commitment
	/// - verifies (or generates) commitment-related proofs
	/// - gossips the blob to designated blob owners
	/// - submits the metadata transaction to the transaction pool
	///
	/// The blob data itself is **not** included on-chain. Only the metadata
	/// transaction is submitted to the chain.
	///
	/// ### Parameters
	/// - `metadata_signed_transaction`:
	///   A SCALE-encoded, signed metadata transaction (base64-encoded),
	///   typically a `submit_blob_metadata` call.
	/// - `blob`:
	///   The raw blob data (base64-encoded).
	///
	/// ### Returns
	/// - `()` on successful submission.
	///
	/// ### Errors
	/// - If the blob is empty or exceeds size limits.
	/// - If the metadata transaction is invalid or expired.
	/// - If the commitment or evaluation data is invalid.
	/// - If commitment validation fails.
	/// - If submission to the transaction pool fails.
	#[method(name = "blob_submitBlob")]
	async fn submit_blob(
		&self,
		metadata_signed_transaction: B64Param,
		blob: B64Param,
	) -> RpcResult<()>;

	/// Returns the full blob data for a given blob hash.
	///
	/// This RPC retrieves the blob either from local storage or
	/// from the network, depending on availability.
	///
	/// The RPC operates in **two modes**:
	///
	/// ### Mode A: Block-scoped lookup
	/// If `at` is provided:
	/// - Blob ownership is derived from the DA post-inherent
	///   in the specified block.
	/// - The node attempts to fetch the blob from the
	///   owners listed in that block.
	///
	/// ### Mode B: Storage-based lookup
	/// If `at` is omitted:
	/// - Blob ownership is derived from the local blob indexer.
	/// - The node attempts to fetch the blob from known owners.
	///
	/// In both cases:
	/// - If the blob exists locally, it is returned immediately.
	/// - Otherwise, the node queries blob owners via p2p.
	///
	/// ### Parameters
	/// - `blob_hash`: The hash of the blob to retrieve.
	/// - `at`: Optional block hash.
	///   - If provided, restricts lookup to blob ownership
	///     recorded in that block.
	///   - If omitted, uses locally indexed blob ownership.
	///
	/// ### Returns
	/// - `Blob` containing:
	///   - blob hash
	///   - blob size
	///   - raw blob data
	///
	/// ### Errors
	/// - If the blob hash is unknown.
	/// - If no owners are known for the blob.
	/// - If all attempts to fetch the blob from owners fail.
	/// - If the block specified by `at` cannot be found or decoded.
	#[method(name = "blob_getBlob")]
	async fn get_blob(&self, blob_hash: H256, at: Option<Block::Hash>) -> RpcResult<Blob>;

	/// Returns metadata and inclusion information for a blob.
	///
	/// This RPC queries the local blob indexer and returns information
	/// about the block in which the blob was included and the validators
	/// that claimed ownership of the blob.
	///
	/// This does **not** return the blob data itself.
	///
	/// ### Parameters
	/// - `blob_hash`: The hash of the blob.
	///
	/// ### Returns
	/// - `BlobInfo` containing:
	///   - blob hash
	///   - block hash and block number where the blob was included
	///   - ownership information (validators who stored the blob)
	///
	/// ### Errors
	/// - If the blob hash is unknown to the node.
	#[method(name = "blob_getBlobInfo")]
	async fn get_blob_info(&self, blob_hash: H256) -> RpcResult<BlobInfo>;

	/// Returns a proof that a blob was included in a specific block.
	///
	/// The proof allows a verifier to verify that
	/// the blob hash was included in the block's data root.
	///
	/// ### Parameters
	/// - `blob_hash`: The hash of the blob.
	/// - `at`: Optional block hash.
	///   - If provided, the proof is generated against that block.
	///   - If omitted, the node uses the local indexer to get the block where the blob
	///     was included.
	///
	/// ### Returns
	/// - `DataProof` proving inclusion of the blob in the block.
	///
	/// ### Errors
	/// - If the blob is not found in the specified block.
	/// - If the block cannot be retrieved or decoded.
	#[method(name = "blob_inclusionProof")]
	async fn inclusion_proof(
		&self,
		blob_hash: H256,
		at: Option<Block::Hash>,
	) -> RpcResult<DataProof>;

	// TODO: feature-gate this RPC only for debugging & development
	#[method(name = "blob_logStuff")]
	async fn log_stuff(&self) -> RpcResult<()>;

	/// Returns a summary of all successfully included blobs in a block.
	///
	/// The summary is derived from the DA post-inherent and does **not**
	/// include heavy evaluation data (e.g. FRI proofs).
	///
	/// This is suitable for:
	/// - Light Clients
	/// - Custom Indexers
	/// - Explorers
	///
	/// ### Parameters
	/// - `at`: Optional block hash.
	///   - If omitted, uses the node's best block.
	///
	/// ### Returns
	/// - A list of `BlobSummary`, each containing:
	///   - blob hash
	///   - transaction index
	///   - AppId
	///   - blob size (bytes)
	///
	/// ### Errors
	/// - If the block is not found.
	/// - If the block does not contain a DA post-inherent.
	#[method(name = "blob_getBlobsSummary")]
	async fn get_blobs_summary(&self, at: Option<Block::Hash>) -> RpcResult<Vec<BlobSummary>>;

	/// Returns all blob hashes associated with a given AppId in a block.
	///
	/// This RPC is a filtered view over `blob_getBlobsSummary`
	/// and is useful for application-specific indexing.
	///
	/// ### Parameters
	/// - `app_id`: The AppId to filter blobs by.
	/// - `at`: Optional block hash.
	///   - If omitted, uses the node's best block.
	///
	/// ### Returns
	/// - A list of blob hashes associated with the given AppId.
	///
	/// ### Errors
	/// - If the block is not found.
	/// - If the block does not contain a DA post-inherent.
	#[method(name = "blob_getBlobsByAppId")]
	async fn get_blobs_by_appid(
		&self,
		app_id: AppId,
		at: Option<Block::Hash>,
	) -> RpcResult<Vec<H256>>;

	/// Returns FRI evaluation data for a blob in a block.
	///
	/// This RPC exposes the data required to verify the correctness
	/// of the blob's FRI commitment:
	/// - evaluation point seed
	/// - evaluation claim
	/// - evaluation proof
	///
	/// This data is included in the block body (post-inherent) and the BlobSummary extrinsic
	/// and can be independently verified by Light Clients.
	///
	/// ### Parameters
	/// - `blob_hash`: The hash of the blob.
	/// - `at`: Optional block hash.
	///   - If omitted, uses the node's best block.
	///
	/// ### Returns
	/// - `BlobEvalData` containing:
	///   - evaluation point seed
	///   - evaluation claim
	///   - evaluation proof bytes
	///
	/// ### Errors
	/// - If the blob is not found in the block.
	/// - If the blob does not contain evaluation data
	///   (e.g. non-FRI or incomplete data).
	#[method(name = "blob_getEvalData")]
	async fn get_eval_data(
		&self,
		blob_hash: H256,
		at: Option<Block::Hash>,
	) -> RpcResult<BlobEvalData>;

	/// Returns FRI sampling (inclusion) proofs for specific cells of a blob.
	///
	/// Each sampling proof allows a verifier to check that a specific
	/// codeword cell belongs to the committed polynomial.
	///
	/// This RPC is intended for:
	/// - Light Clients performing data availability sampling
	/// - External verifiers auditing blob availability
	///
	/// ### Parameters
	/// - `cells`: A list of codeword indices (`u32`) to sample.
	/// - `blob_hash`: The hash of the blob.
	/// - `at`: Optional block hash.
	///   - If omitted, uses the node's best block.
	///
	/// ### Returns
	/// - A list of `SamplingProof`, each containing:
	///   - cell index
	///   - cell value (16 bytes)
	///   - serialized inclusion proof transcript
	///
	/// ### Errors
	/// - If the blob cannot be retrieved.
	/// - If any cell index is out of bounds.
	/// - If proof generation fails.
	#[method(name = "blob_getSamplingProof")]
	async fn get_sampling_proof(
		&self,
		cells: Vec<u32>,
		blob_hash: H256,
		at: Option<Block::Hash>,
	) -> RpcResult<Vec<SamplingProof>>;
}

pub struct BlobRpc<Pool, Block: BlockT, Backend> {
	pool: Arc<Pool>,
	backend: Arc<Backend>,
	blob_handle: Arc<BlobHandle<Block>>,
	commitment_queue: Arc<CommitmentQueue>,
	nonce_cache: Arc<NonceCache>,
	fri_sampling_cache: Arc<Mutex<HashMap<FriSamplingCacheKey, FriSamplingCacheEntry>>>,
	_block: PhantomData<Block>,
}

impl<Pool, Block: BlockT, Backend> BlobRpc<Pool, Block, Backend>
where
	H256: From<<Block as BlockT>::Hash>,
	<Block as BlockT>::Hash: From<H256>,
{
	pub fn new(
		blob_handle: Arc<BlobHandle<Block>>,
		pool: Arc<Pool>,
		backend: Arc<Backend>,
	) -> Self {
		let (queue, rx) = CommitmentQueue::new(25);
		BlobMetrics::set_queue_capacity(rx.capacity() as u64);
		CommitmentQueue::spawn_background_task(rx);

		Self {
			pool,
			backend,
			blob_handle,
			commitment_queue: Arc::new(queue),
			nonce_cache: Arc::new(NonceCache::new()),
			fri_sampling_cache: Arc::new(Mutex::new(HashMap::new())),
			_block: PhantomData,
		}
	}

	fn at_or_best(&self, at: Option<Block::Hash>) -> Block::Hash {
		at.unwrap_or_else(|| self.blob_handle.client.info().best_hash.into())
	}

	// The SubmittedData contains info about only succesfull blob from both BlobMetdata & BlobSummary post-inherent
	fn load_da_submissions(
		&self,
		at: Block::Hash,
	) -> RpcResult<Vec<avail_base::header_extension::SubmittedData>> {
		let block = self
			.blob_handle
			.client
			.block(at.into())
			.map_err(|e| internal_err!("Failed to get block: {:?}", e))?
			.ok_or_else(|| internal_err!("Block not found: {:?}", at))?
			.block;

		let extrinsics = block.extrinsics();
		if extrinsics.len() < 2 {
			return Err(internal_err!(
				"Block does not contain post-inherent summary extrinsic"
			));
		}

		Ok(
			HeaderExtensionBuilderData::from_opaque_extrinsics::<RTExtractor>(
				block.header.number,
				&extrinsics,
			)
			.data_submissions,
		)
	}
}

#[async_trait]
impl<Pool, Block, Backend> BlobApiServer<Block> for BlobRpc<Pool, Block, Backend>
where
	Block: BlockT,
	Pool: TransactionPool<Block = Block> + 'static,
	Backend: sc_client_api::Backend<Block> + Send + Sync + 'static,
	Backend::State: StateBackend<HashingFor<Block>>,
	H256: From<<Block as BlockT>::Hash>,
	<Block as BlockT>::Hash: From<H256>,
	u32: From<<<Block as BlockT>::Header as HeaderT>::Number>,
	<Block as BlockT>::Extrinsic: From<UncheckedExtrinsic>,
	H256: From<<Pool as sc_transaction_pool_api::TransactionPool>::Hash>,
{
	#[tracing::instrument(name = "blob.submit", skip_all)]
	async fn submit_blob(
		&self,
		metadata_signed_transaction: B64Param,
		blob: B64Param,
	) -> RpcResult<()> {
		// Metrics
		BlobMetrics::inc_submissions_total();

		// --- 0. Quick checks -------------------------------------------------
		if blob.0.is_empty() {
			return Err(internal_err!("blob cannot be empty"));
		}
		if metadata_signed_transaction.0.is_empty() {
			return Err(internal_err!("metadata tx cannot be empty"));
		}

		let friends = Friends {
			externalities: Arc::new(RealExternalities::new(self.blob_handle.clone())),
			runtime_client: Arc::new(RuntimeClient::new(self.blob_handle.client.clone())),
			backend_client: Arc::new(BackendClient::new(self.backend.clone())),
			tx_pool_client: Arc::new(TransactionPoolClient::new(self.pool.clone())),
			database: self.blob_handle.blob_database.clone(),
		};

		let result = submit_blob_main_task(
			self.commitment_queue.clone(),
			metadata_signed_transaction.0,
			blob.0,
			friends,
			self.nonce_cache.clone(),
		)
		.await;

		// Metrics
		BlobMetrics::inc_submissions_valid_total();

		result?;

		Ok(())
	}

	#[tracing::instrument(name = "get_blob", skip_all)]
	async fn get_blob(&self, blob_hash: H256, at: Option<Block::Hash>) -> RpcResult<Blob> {
		// get the blob owners peer_id's
		let peer_ids: Vec<String> = if let Some(at_hash) = at {
			// Mode A: read the block directly from our client and decode the summary extrinsic.
			let block = self
				.blob_handle
				.client
				.block(at_hash.into())
				.map_err(|e| internal_err!("Failed to get block: {:?}", e))?
				.ok_or_else(|| internal_err!("Block not found: {:?}", at_hash))?
				.block;

			let extrinsics = block.extrinsics();
			if extrinsics.len() < 2 {
				return Err(internal_err!(
					"Block does not contain post-inherent summary extrinsic"
				));
			}

			// summary is second last extrinsic
			let summary_encoded = extrinsics[extrinsics.len().wrapping_sub(2)].encode();
			let summary_xt: UncheckedExtrinsic = Decode::decode(&mut &summary_encoded[..])
				.map_err(|_| internal_err!("Failed to decode summary extrinsic"))?;

			if let RuntimeCall::DataAvailability(Call::submit_blob_txs_summary {
				blob_txs_summary,
				..
			}) = summary_xt.function
			{
				// find the summary for this blob_hash
				let summary = blob_txs_summary
					.into_iter()
					.find(|s| s.hash == blob_hash)
					.ok_or_else(|| {
						internal_err!(
						"Blob transaction summary not found for blob {:?} in provided block {:?}",
						blob_hash,
						at_hash
					)
					})?;

				if !summary.success {
					return Err(internal_err!(
						"Blob update not successful at provided block {:?}: {:?}",
						at_hash,
						summary.reason
					));
				}
				if summary.ownership.is_empty() {
					return Err(internal_err!(
						"Blob ownership empty in provided block {:?} for blob {:?}",
						at_hash,
						blob_hash
					));
				}

				summary
					.ownership
					.into_iter()
					.map(|(_addr, _babe_key, encoded_peer_id, _sig)| encoded_peer_id)
					.collect()
			} else {
				return Err(internal_err!(
					"Expected DataAvailability::submit_blob_txs_summary in block"
				));
			}
		} else {
			// Mode B: read local BlobInfo (indexer).
			let blob_info = self
				.blob_handle
				.blob_database
				.get_blob_info(&blob_hash)
				.map_err(|e| internal_err!("Failed to get blob info: {:?}", e))?
				.ok_or_else(|| internal_err!("Blob info not found for hash: {:?}", blob_hash))?;

			if blob_info.ownership.is_empty() {
				return Err(internal_err!(
					"Blob ownership empty in local BlobInfo for {:?}",
					blob_hash
				));
			}

			blob_info
				.ownership
				.into_iter()
				.map(|entry| entry.encoded_peer_id)
				.collect()
		};

		// If blob exists locally, return immediately
		if let Ok(Some(blob)) = self.blob_handle.blob_database.get_blob(&blob_hash) {
			tracing::info!("Blob found in local storage: {:?}", blob_hash);
			return Ok(blob);
		}

		// Ensure we have peers to try
		if peer_ids.is_empty() {
			return Err(internal_err!(
				"No owners/peers known for blob {:?}",
				blob_hash
			));
		}

		let my_peer_id = self.blob_handle.network.local_peer_id();
		// Deterministic start index (seeded by finalized_hash for stability)
		let seed_bytes = self.blob_handle.client.info().finalized_hash.encode();
		let base_index = generate_base_index(blob_hash, &seed_bytes, peer_ids.len(), None)
			.map_err(|e| internal_err!("Failed to generate base index: {e:?}"))?;

		// Try peers round-robin
		for attempt in 0..(MAX_RPC_RETRIES as usize) {
			let index = (base_index + attempt) % peer_ids.len();
			let encoded_peer_id = &peer_ids[index];

			match PeerId::from_str(encoded_peer_id) {
				Ok(peer_id) => {
					if peer_id == my_peer_id {
						tracing::warn!(
							"Attempt {}/{}: skipping self peer_id {} we've already tried locally",
							attempt + 1,
							MAX_RPC_RETRIES,
							encoded_peer_id
						);
						continue;
					}
					match send_blob_query_request::<Block>(
						blob_hash,
						peer_id,
						&self.blob_handle.network,
					)
					.await
					{
						Ok(Some(blob)) => return Ok(blob),
						Ok(None) => {
							tracing::warn!(
								"Attempt {}/{}: owner {} returned no blob",
								attempt + 1,
								MAX_RPC_RETRIES,
								encoded_peer_id
							);
						},
						Err(e) => {
							tracing::warn!(
								"Attempt {}/{}: RPC error from {}: {:?}",
								attempt + 1,
								MAX_RPC_RETRIES,
								encoded_peer_id,
								e
							);
						},
					}
				},
				Err(e) => {
					tracing::warn!(
						"Attempt {}/{}: invalid peer_id '{}' : {:?}",
						attempt + 1,
						MAX_RPC_RETRIES,
						encoded_peer_id,
						e
					);
				},
			}
		}

		Err(internal_err!(
			"All attempts to fetch blob {:?} from its owners failed.",
			blob_hash
		))
	}

	#[tracing::instrument(name = "get_blob_info", skip_all)]
	async fn get_blob_info(&self, blob_hash: H256) -> RpcResult<BlobInfo> {
		self.blob_handle
			.blob_database
			.get_blob_info(&blob_hash)
			.map_err(|e| internal_err!("Failed to get blob info: {:?}", e))?
			.ok_or_else(|| internal_err!("Blob info not found for hash: {:?}", blob_hash))
	}

	#[tracing::instrument(name = "inclusion_proof", skip_all)]
	async fn inclusion_proof(
		&self,
		blob_hash: H256,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<DataProof> {
		// if block_hash is supplied, use it, otherwise try using blob_info to find the latest finalised block it was included in
		let block_hash = if let Some(h) = at {
			h
		} else {
			let blob_info = self
				.blob_handle
				.blob_database
				.get_blob_info(&blob_hash)
				.map_err(|e| internal_err!("Failed to get blob info: {:?}", e))?
				.ok_or_else(|| {
					internal_err!("Blob info not found for blob_hash: {:?}", blob_hash)
				})?;
			blob_info.block_hash.into()
		};

		let block = self
			.blob_handle
			.client
			.block(block_hash.into())
			.map_err(|e| {
				internal_err!(
					"Failed to get block for generating inclusion proof: {:?}",
					e
				)
			})?
			.ok_or_else(|| internal_err!("Block not found for block_hash: {:?}", block_hash))?
			.block;

		// We can restrict generating inclusion_proof only for finalised blocks, although if block_hash is not provided, we use blob_info from finalised blocks only
		let (header, extrinsics) = block.deconstruct();
		self.blob_handle
			.client
			.runtime_api()
			.inclusion_proof(header.hash(), extrinsics, blob_hash)
			.map_err(|e| internal_err!("KateApi::inclusion_proof failed: {e:?}"))?
			.ok_or_else(|| {
				internal_err!(
					"Cannot fetch tx data by blob_hash {blob_hash:?} at block {:?}",
					block_hash
				)
			})
	}

	#[tracing::instrument(name = "log_stuff", skip_all)]
	async fn log_stuff(&self) -> RpcResult<()> {
		let _ = self.blob_handle.blob_database.log_all_entries();
		Ok(())
	}

	#[tracing::instrument(name = "get_blobs_summary", skip_all)]
	async fn get_blobs_summary(&self, at: Option<Block::Hash>) -> RpcResult<Vec<BlobSummary>> {
		let at = self.at_or_best(at);
		let submissions = self.load_da_submissions(at)?;

		Ok(submissions
			.iter()
			.map(|d| BlobSummary::new(d.hash, d.tx_index, d.id, d.size_bytes))
			.collect())
	}

	#[tracing::instrument(name = "get_blobs_by_appid", skip_all)]
	async fn get_blobs_by_appid(
		&self,
		app_id: AppId,
		at: Option<Block::Hash>,
	) -> RpcResult<Vec<H256>> {
		let at = self.at_or_best(at);
		let submissions = self.load_da_submissions(at)?;

		Ok(submissions
			.iter()
			.filter(|d| d.id == app_id)
			.map(|d| d.hash)
			.collect())
	}

	#[tracing::instrument(name = "get_eval_data", skip_all)]
	async fn get_eval_data(
		&self,
		blob_hash: H256,
		at: Option<Block::Hash>,
	) -> RpcResult<BlobEvalData> {
		let at = self.at_or_best(at);
		let submissions = self.load_da_submissions(at)?;

		let d = submissions
			.iter()
			.find(|d| d.hash == blob_hash)
			.ok_or_else(|| {
				internal_err!(
					"Blob submission data not found for blob {:?} in block {:?}",
					blob_hash,
					at
				)
			})?;

		match (&d.eval_point_seed, &d.eval_claim, &d.eval_proof) {
			(Some(seed), Some(claim), Some(proof)) => {
				Ok(BlobEvalData::new(*seed, *claim, proof.clone()))
			},
			_ => Err(internal_err!(
				"Blob {:?} does not contain eval data in block {:?}",
				blob_hash,
				at
			)),
		}
	}

	#[tracing::instrument(name = "get_sampling_proof", skip_all)]
	async fn get_sampling_proof(
		&self,
		cells: Vec<u32>,
		blob_hash: H256,
		at: Option<Block::Hash>,
	) -> RpcResult<Vec<SamplingProof>> {
		let at = self.at_or_best(at);
		let cache_key = (at.into(), blob_hash);
		let submissions = self.load_da_submissions(at)?;
		let expected_commitment = submissions
			.iter()
			.find(|d| d.hash == blob_hash)
			.map(|d| d.commitments.clone())
			.ok_or_else(|| {
				internal_err!(
					"Blob submission data not found for blob {:?} in block {:?}",
					blob_hash,
					at
				)
			})?;

		if let Some(entry) = {
			let cache = self.fri_sampling_cache.lock();
			cache.get(&cache_key).cloned()
		} {
			return build_sampling_proofs(entry, cells, &expected_commitment);
		}

		let blob = self.get_blob(blob_hash, Some(at)).await?;

		let encoder = BytesEncoder::<B128>::new();
		let packed = encoder
			.bytes_to_packed_mle(&blob.data)
			.map_err(|e| internal_err!("bytes_to_packed_mle failed: {e}"))?;

		let params_version = self
			.blob_handle
			.client
			.runtime_api()
			.get_fri_params_version(at.into())
			.map_err(|e| internal_err!("failed to get FRI params version from runtime: {e:?}"))?;
		let cfg = params_version.to_config(packed.total_n_vars);
		let pcs = Arc::new(FriBiniusPCS::new(cfg));

		let ctx = pcs
			.initialize_fri_context::<B128>(packed.packed_mle.log_len())
			.map_err(|e| internal_err!("FRI ctx init failed: {e}"))?;

		let commit_output = Arc::new(
			pcs.commit(&packed.packed_mle, &ctx)
				.map_err(|e| internal_err!("FRI commit failed: {e}"))?,
		);
		let log_batch_size = ctx.fri_params.log_batch_size();
		let leaf_count = 1usize
			<< (ctx
				.fri_params
				.rs_code()
				.log_len()
				.saturating_sub(log_batch_size));

		let entry = FriSamplingCacheEntry {
			pcs: pcs.clone(),
			commit_output: commit_output.clone(),
			log_batch_size,
			leaf_count,
		};

		{
			let mut cache = self.fri_sampling_cache.lock();
			cache.insert(cache_key, entry.clone());
		}

		build_sampling_proofs(entry, cells, &expected_commitment)
	}
}

#[tracing::instrument(name = "blob.sampling_proofs", skip_all)]
fn build_sampling_proofs(
	entry: FriSamplingCacheEntry,
	cells: Vec<u32>,
	expected_commitment: &[u8],
) -> RpcResult<Vec<SamplingProof>> {
	if entry.commit_output.commitment.as_slice() != expected_commitment {
		return Err(internal_err!(
			"Blob commitment mismatch between block submission and local blob data"
		));
	}

	if cells.iter().any(|&c| (c as usize) >= entry.leaf_count) {
		return Err(internal_err!("One or more cell indices out of bounds"));
	}

	let mut proofs = Vec::with_capacity(cells.len());

	for &cell in &cells {
		let idx = cell as usize;
		let sampled_values = entry
			.commit_output
			.codeword
			.to_ref()
			.chunk(entry.log_batch_size, idx)
			.iter_scalars()
			.collect::<Vec<_>>();

		let transcript = entry
			.pcs
			.inclusion_proof::<B128>(&entry.commit_output.committed, idx)
			.map_err(|e| internal_err!("Sampling proof failed: {e}"))?;

		let mut cell_bytes = Vec::with_capacity(sampled_values.len() * 16);
		for value in sampled_values {
			cell_bytes.extend_from_slice(&value.val().to_le_bytes());
		}

		proofs.push(SamplingProof::new(
			cell,
			cell_bytes,
			transcript_to_bytes(&transcript),
		));
	}

	Ok(proofs)
}

#[tracing::instrument(name = "check_rpc_store_blob", skip_all)]
async fn check_rpc_store_blob(
	blob_metadata: &BlobMetadata,
	my_encoded_peer_id: String,
	finalized_block_hash: H256,
	externalities: &Arc<dyn ExternalitiesT>,
	runtime_client: &Arc<dyn RuntimeApiT>,
	storing_validators: &Vec<AccountId32>,
) -> std::result::Result<Option<OwnershipEntry>, String> {
	let role = externalities.role();
	if !role.is_authority() {
		// RPC node (me) is not an authority, so I don't have to store blobs
		return Ok(None);
	}

	let keystore = externalities.keystore();
	let Some((authority_id, key_type_id)) = get_my_validator_public_account(keystore) else {
		return Ok(None);
	};

	let Ok(owner_opt) = runtime_client.get_validator_from_key(
		finalized_block_hash,
		key_type_id,
		authority_id.encode(),
	) else {
		return Ok(None);
	};

	let Some(my_validator_id) = owner_opt else {
		return Ok(None);
	};

	let should_store_blob = storing_validators.contains(&my_validator_id);
	if !should_store_blob {
		return Ok(None);
	}

	let signature_payload = build_signature_payload(
		blob_metadata.hash,
		[my_validator_id.encode(), b"stored".to_vec()].concat(),
	);
	let signature = match sign_blob_data(keystore, signature_payload) {
		Ok(s) => s.signature,
		Err(e) => {
			return Err(std::format!(
				"An error has occured while trying to sign data, exiting the function: {e}"
			));
		},
	};

	Ok(Some(OwnershipEntry {
		address: my_validator_id,
		babe_key: authority_id,
		encoded_peer_id: my_encoded_peer_id,
		signature,
	}))
}

#[tracing::instrument(name = "get_babe_randomness", skip_all)]
fn get_babe_randomness(
	backend_client: &Arc<dyn BackendApiT>,
	finalized_block_hash: H256,
) -> RpcResult<[u8; 32]> {
	let storage_key = get_babe_randomness_key();
	let maybe_raw = backend_client
		.storage(finalized_block_hash, &storage_key.0)
		.map_err(|e| internal_err!("Storage query error: {e:?}"))?;
	let raw = maybe_raw.ok_or(internal_err!("Randomness not found"))?;
	let randomness =
		<[u8; 32]>::decode(&mut &raw[..]).map_err(|e| internal_err!("Decode error: {e:?}"))?;

	Ok(randomness)
}

fn get_dynamic_block_length(
	backend_client: &Arc<dyn BackendApiT>,
	finalized_block_hash: H256,
) -> RpcResult<(usize, usize)> {
	let storage_key = get_dynamic_blocklength_key();
	let maybe_raw = backend_client
		.storage(finalized_block_hash, &storage_key.0)
		.map_err(|e| internal_err!("Storage query error: {e:?}"))?;
	let raw = maybe_raw.ok_or(internal_err!("DynamicBlockLength not found"))?;
	let block_length =
		BlockLength::decode(&mut &raw[..]).map_err(|e| internal_err!("Decode error: {e:?}"))?;
	let cols = block_length.cols.0 as usize;
	let rows = block_length.rows.0 as usize;

	Ok((cols, rows))
}

#[tracing::instrument(name = "blob.submit.main_task", skip_all)]
pub async fn submit_blob_main_task(
	commitment_queue: Arc<dyn CommitmentQueueApiT>,
	metadata_signed_transaction: Vec<u8>,
	blob: Vec<u8>,
	friends: Friends,
	nonce_cache: Arc<dyn NonceCacheApiT>,
) -> RpcResult<tokio::task::JoinHandle<()>> {
	let runtime_client = friends.runtime_client.clone();

	// Get client info
	let client_info = friends.externalities.client_info();
	let best_hash = client_info.best_hash;
	let finalized_block_hash = client_info.finalized_hash;

	let commitment_scheme = match runtime_client.commitment_scheme(best_hash) {
		Ok(scheme) => scheme,
		Err(e) => {
			tracing::error!(
				"Could not get commitment scheme from runtime at {:?}: {e:?}. Falling back to Fri.",
				best_hash
			);
			CommitmentScheme::Fri
		},
	};

	let blob_params = match runtime_client.get_blob_runtime_parameters(finalized_block_hash) {
		Ok(p) => p,
		Err(e) => {
			tracing::error!("Could not get blob_params: {e:?}");
			BlobRuntimeParameters::default()
		},
	};
	let fri_params_version = match runtime_client.get_fri_params_version(finalized_block_hash) {
		Ok(v) => v,
		Err(e) => {
			tracing::error!(
				"Could not get FRI params version from runtime at {:?}: {e:?}. Falling back to V0.",
				finalized_block_hash
			);
			FriParamsVersion::V0
		},
	};
	let max_blob_size = blob_params.max_blob_size as usize;

	let (app_id, blob_hash, provided_commitment, eval_point_seed, eval_claim) =
		initial_validation(max_blob_size, &blob, &metadata_signed_transaction)
			.map_err(|e| internal_err!("{}", e))?;

	tracing::info!(block_hash = ?blob_hash, blob_size = blob.len(), "Blob passed initial validation");

	let opaque_tx = tx_validation(
		best_hash,
		&metadata_signed_transaction,
		blob_params.min_transaction_validity,
		blob_params.max_transaction_validity,
		&runtime_client,
		&nonce_cache,
		true,
	)
	.map_err(|e| internal_err!("{}", e))?;

	if let Some((who, nonce)) = extract_signer_and_nonce(&opaque_tx) {
		nonce_cache.commit(&who, nonce);
	}

	let parent = tracing::Span::current();
	match commitment_scheme {
		CommitmentScheme::Kzg => {
			let (cols, rows) =
				get_dynamic_block_length(&friends.backend_client, finalized_block_hash).map_err(
					|e| {
						clear_reserved_nonce(&nonce_cache, &opaque_tx);
						e
					},
				)?;
			let blob = Arc::new(blob);

			// ideally eval_point_seed and eval_claim should be None here for KZG, but we can let it pass for now
			Ok(tokio::spawn(async move {
				let result = handle_kzg_submission(
					commitment_queue,
					metadata_signed_transaction,
					opaque_tx,
					blob_hash,
					blob,
					blob_params,
					provided_commitment,
					friends,
					nonce_cache,
					runtime_client,
					cols,
					rows,
				)
				.instrument(parent)
				.await;
				if let Err(e) = result {
					tracing::error!(error = ?e, "handle_fri_submission error.");
				}
			}))
		},

		CommitmentScheme::Fri => {
			// Check if the eval_point_seed and eval_claim are present for Fri
			if eval_point_seed.is_none() || eval_claim.is_none() {
				clear_reserved_nonce(&nonce_cache, &opaque_tx);
				return Err(internal_err!(
					"eval_point_seed and eval_claim must be present for Fri commitment scheme"
				));
			}

			let eval_point_seed = eval_point_seed.expect("checked above; qed");
			let eval_claim = eval_claim.expect("checked above; qed");
			let babe_randomness =
				get_babe_randomness(&friends.backend_client, finalized_block_hash).map_err(
					|e| {
						clear_reserved_nonce(&nonce_cache, &opaque_tx);
						e
					},
				)?;
			let derived_eval_seed = derive_seed_from_inputs(&babe_randomness, &blob_hash.0);
			if eval_point_seed != derived_eval_seed {
				clear_reserved_nonce(&nonce_cache, &opaque_tx);
				return Err(internal_err!(
					"eval_point_seed does not match derived seed!"
				));
			}

			Ok(tokio::spawn(async move {
				let result = handle_fri_submission(
					metadata_signed_transaction,
					opaque_tx,
					app_id,
					blob_hash,
					blob,
					blob_params,
					provided_commitment,
					friends,
					nonce_cache,
					runtime_client,
					fri_params_version,
					eval_point_seed,
					eval_claim,
					derived_eval_seed,
				)
				.instrument(parent)
				.await;

				if let Err(e) = result {
					tracing::error!(error = ?e, "handle_fri_submission error.");
				}
			}))
		},
	}
}

#[tracing::instrument(name = "handle_kzg_submission", skip_all)]
async fn handle_kzg_submission(
	commitment_queue: Arc<dyn CommitmentQueueApiT>,
	metadata_signed_transaction: Vec<u8>,
	opaque_tx: UncheckedExtrinsic,
	blob_hash: H256,
	blob: Arc<Vec<u8>>,
	blob_params: BlobRuntimeParameters,
	provided_commitment: Vec<u8>,
	friends: Friends,
	nonce_cache: Arc<dyn NonceCacheApiT>,
	runtime_client: Arc<dyn RuntimeApiT>,
	cols: usize,
	rows: usize,
) -> anyhow::Result<()> {
	tracing::info!(block_hash = ?blob_hash, blob_size = blob.len(), "Blob handle kzg submission");

	let blob_for_grid = blob.clone();

	let parent = tracing::Span::current();
	let grid_span = tracing::info_span!(
		parent: &parent,
		"build_polynomial_grid_blocking"
	);
	let grid = task::spawn_blocking(move || {
		let _enter = grid_span.enter();
		build_polynomial_grid(blob_for_grid.as_slice(), cols, rows, Default::default())
	})
	.await
	.map_err(|e| {
		clear_reserved_nonce(&nonce_cache, &opaque_tx);
		internal_err!(
			"KZG polynomial grid generation task failed for blob {:?}: {}",
			blob_hash,
			e
		)
	})?;

	validate_kzg_commitment(blob_hash, &provided_commitment, grid, &commitment_queue)
		.await
		.map_err(|e| {
			clear_reserved_nonce(&nonce_cache, &opaque_tx);
			internal_err!("{}", e)
		})?;

	// After potentially long work, re-validate tx
	let client_info = friends.externalities.client_info();
	let best_hash = client_info.best_hash;

	let _ = tx_validation(
		best_hash,
		&metadata_signed_transaction,
		blob_params.min_transaction_validity,
		blob_params.max_transaction_validity,
		&runtime_client,
		&nonce_cache,
		false,
	)
	.map_err(|e| {
		clear_reserved_nonce(&nonce_cache, &opaque_tx);
		internal_err!("{}", e)
	})?;

	submit_blob_background_task(
		opaque_tx,
		blob_hash,
		blob,
		blob_params,
		provided_commitment,
		None,
		friends,
		nonce_cache,
	)
	.await;

	Ok(())
}

#[tracing::instrument(name = "handle_fri_submission", skip_all)]
async fn handle_fri_submission(
	metadata_signed_transaction: Vec<u8>,
	opaque_tx: UncheckedExtrinsic,
	app_id: AppId,
	blob_hash: H256,
	blob: Vec<u8>,
	blob_params: BlobRuntimeParameters,
	provided_commitment: Vec<u8>,
	friends: Friends,
	nonce_cache: Arc<dyn NonceCacheApiT>,
	runtime_client: Arc<dyn RuntimeApiT>,
	fri_params_version: FriParamsVersion,
	eval_point_seed: [u8; 32],
	eval_claim: [u8; 16],
	derived_eval_seed: [u8; 32],
) -> anyhow::Result<()> {
	tracing::info!(block_hash = ?blob_hash, blob_size = blob.len(), "Blob handle fri submission");

	let blob = Arc::new(blob);
	let blob_for_validation = blob.clone();
	let commitment_for_validation = provided_commitment.clone();

	let parent = tracing::Span::current();
	let validation_span = tracing::info_span!(
		parent: &parent,
		"validate_fri_commitment_blocking"
	);
	let fri_eval_proof = task::spawn_blocking(move || {
		let _enter = validation_span.enter();

		validate_fri_commitment(
			blob_hash,
			blob_for_validation.as_slice(),
			&commitment_for_validation,
			fri_params_version,
			&derived_eval_seed,
			&eval_claim,
		)
	})
	.await
	.expect("Fri commitment validation task panicked")
	.map_err(|e| {
		clear_reserved_nonce(&nonce_cache, &opaque_tx);
		internal_err!("{}", e)
	})?;

	let client_info = friends.externalities.client_info();
	let best_hash = client_info.best_hash;

	let _ = tx_validation(
		best_hash,
		&metadata_signed_transaction,
		blob_params.min_transaction_validity,
		blob_params.max_transaction_validity,
		&runtime_client,
		&nonce_cache,
		false,
	)
	.map_err(|e| {
		clear_reserved_nonce(&nonce_cache, &opaque_tx);
		internal_err!("{}", e)
	})?;

	let fri_data = FriData {
		app_id,
		eval_point_seed,
		eval_claim,
		fri_eval_proof: Some(fri_eval_proof),
	};
	submit_blob_background_task(
		opaque_tx,
		blob_hash,
		blob,
		blob_params,
		provided_commitment,
		Some(fri_data),
		friends,
		nonce_cache,
	)
	.await;

	Ok(())
}

#[tracing::instrument(name = "blob.submit.background_task", skip_all)]
async fn submit_blob_background_task(
	opaque_tx: UncheckedExtrinsic,
	blob_hash: H256,
	blob: Arc<Vec<u8>>,
	blob_params: BlobRuntimeParameters,
	commitment: Vec<u8>,
	fri_data: Option<FriData>,
	friends: Friends,
	nonce_cache: Arc<dyn NonceCacheApiT>,
) {
	tracing::info!(block_hash = ?blob_hash, blob_size = blob.len(), "Submit blob background task started.");

	let blob_len = blob.len();
	let signer = extract_signer_and_nonce(&opaque_tx);

	let stored =
		store_and_gossip_blob(blob_hash, blob, blob_params, commitment, fri_data, &friends).await;
	if stored.is_err() {
		if let Some((who, _)) = signer.as_ref() {
			nonce_cache.clear(who);
		}
		return;
	}

	// Push the clean extrinsic to the tx pool ---------------------
	// Get the best hash once more, to submit the tx
	let best_hash = friends.externalities.client_info().best_hash;
	if let Err(e) = friends
		.tx_pool_client
		.submit_one(best_hash, TransactionSource::External, opaque_tx)
		.await
	{
		if let Some((who, _)) = signer.as_ref() {
			nonce_cache.clear(who);
		}
		tracing::error!("tx-pool error: {e}")
	}

	// Metrics and Telemetry
	BlobMetrics::inc_submissions_added_to_pool_total();
	BlobMetrics::inc_submissions_blob_size_pool_total(blob_len as u64);
	crate::telemetry::BlobSubmission::added_to_pool(blob_hash);
}

fn clear_reserved_nonce(nonce_cache: &Arc<dyn NonceCacheApiT>, opaque_tx: &UncheckedExtrinsic) {
	if let Some((who, _)) = extract_signer_and_nonce(opaque_tx) {
		nonce_cache.clear(&who);
	}
}

#[tracing::instrument(name = "blob.submit.store_and_gossip", skip_all)]
pub async fn store_and_gossip_blob(
	blob_hash: H256,
	blob: Arc<Vec<u8>>,
	blob_params: BlobRuntimeParameters,
	commitment: Vec<u8>,
	fri_data: Option<FriData>,
	friends: &Friends,
) -> Result<(), ()> {
	let client_info = friends.externalities.client_info();
	let finalized_block_hash = client_info.finalized_hash;
	let finalized_block_number = client_info.finalized_height as u64;

	// Get my own peer id data
	let my_peer_id = friends.externalities.local_peer_id();
	let my_peer_id_base58 = my_peer_id.to_base58();

	// Setup blob metadata and blob and check first in case we already received this exact blob before
	let maybe_blob_metadata = match friends.database.get_blob_metadata(&blob_hash) {
		Ok(m) => m,
		Err(e) => {
			tracing::error!("Failed to get data from blob storage: {e}");
			return Err(());
		},
	};

	let commitment_scheme = match friends
		.runtime_client
		.commitment_scheme(finalized_block_hash)
	{
		Ok(scheme) => scheme,
		Err(e) => {
			tracing::error!(
				"Could not get commitment scheme from runtime at {:?}: {e:?}. Falling back to Fri.",
				finalized_block_hash
			);
			CommitmentScheme::Fri
		},
	};
	let mut blob_metadata = maybe_blob_metadata.unwrap_or_else(|| {
		let blob_len = blob.len();

		BlobMetadata {
			hash: blob_hash,
			size: blob_len.saturated_into(),
			commitment,
			is_notified: true,
			expires_at: 0,
			finalized_block_hash: Default::default(),
			finalized_block_number: 0,
			nb_validators_per_blob: 0,
			nb_validators_per_blob_threshold: 0,
			storing_validator_list: Default::default(),
			eval_point_seed: None,
			eval_claim: None,
			fri_eval_proof: None,
			fri_eval_prover_index: None,
		}
	});

	// It might be a new blob or an old one being resubmitted, we still update most of the values
	let validators = match friends
		.runtime_client
		.get_active_validators(finalized_block_hash)
	{
		Ok(validators) if validators.is_empty() => return Err(()),
		Ok(validators) => validators,
		Err(e) => {
			let err = std::format!(
				"Failed to fetch active validators at {:?}: {:?}",
				finalized_block_hash,
				e
			);
			tracing::error!("{}", err);
			return Err(());
		},
	};

	let (nb_validators_per_blob, threshold) =
		get_validator_per_blob_inner(blob_params.clone(), validators.len() as u32);
	let storing_validators = match validators_for_blob(
		blob_hash,
		&validators,
		&finalized_block_hash.encode(),
		nb_validators_per_blob,
	) {
		Ok(st) => st,
		Err(e) => {
			let err = std::format!(
				"Failed to fetch storing validators at {:?}: {:?}",
				finalized_block_hash,
				e
			);
			tracing::error!("{}", err);
			return Err(());
		},
	};

	if commitment_scheme == CommitmentScheme::Fri {
		if fri_data.is_none() {
			tracing::error!("Fri data must be available for Fri commitment scheme");
			return Err(());
		}
		let fri_data = fri_data.expect("checked above; qed");
		let prover_index =
			designated_prover_index(&blob_hash, &finalized_block_hash, nb_validators_per_blob);

		if let Ok((my_validator_id, _babe_key)) = get_my_validator_id(
			&friends.externalities.keystore(),
			friends.runtime_client.as_ref(),
			finalized_block_hash,
		) {
			if storing_validators[prover_index as usize] == my_validator_id {
				tracing::info!(
					"I am the designated prover for blob {:?} including eval_proof? {}",
					blob_hash,
					fri_data.fri_eval_proof.is_some()
				);
				blob_metadata.fri_eval_proof = fri_data.fri_eval_proof;
				blob_metadata.fri_eval_prover_index = Some(prover_index);
			}
		}
		blob_metadata.eval_point_seed = Some(fri_data.eval_point_seed);
		blob_metadata.eval_claim = Some(fri_data.eval_claim);
	}

	blob_metadata.is_notified = true;
	blob_metadata.expires_at = finalized_block_number.saturating_add(blob_params.temp_blob_ttl);
	blob_metadata.finalized_block_hash = finalized_block_hash.into();
	blob_metadata.finalized_block_number = finalized_block_number;
	blob_metadata.nb_validators_per_blob = nb_validators_per_blob;
	blob_metadata.nb_validators_per_blob_threshold = threshold;

	let maybe_ownership: Option<OwnershipEntry> = match check_rpc_store_blob(
		&blob_metadata,
		my_peer_id_base58.clone(),
		finalized_block_hash,
		&friends.externalities,
		&friends.runtime_client,
		&storing_validators,
	)
	.await
	{
		Ok(o) => o,
		Err(e) => {
			tracing::error!("could not check if rpc should store blob: {e}");
			return Err(());
		},
	};

	blob_metadata.storing_validator_list = storing_validators;

	let (b_hash, b_size, b_commitment) = (
		blob_metadata.hash,
		blob_metadata.size,
		blob_metadata.commitment.clone(),
	);

	if maybe_ownership.is_some() {
		blob_metadata.expires_at =
			finalized_block_number.saturating_add(blob_params.blob_ttl) as u64;
	}

	store_new_blob(
		blob_hash,
		blob,
		&blob_metadata,
		&friends.database,
		&maybe_ownership,
	);

	// Announce the blob to the network -------------------
	let blob_received_notification: BlobNotification =
		BlobNotification::BlobReceived(BlobReceived {
			hash: b_hash,
			size: b_size,
			commitment: b_commitment,
			ownership: maybe_ownership,
			original_peer_id: my_peer_id_base58.clone(),
			finalized_block_hash: finalized_block_hash.into(),
			finalized_block_number,
			eval_point_seed: blob_metadata.eval_point_seed,
			eval_claim: blob_metadata.eval_claim.clone(),
			fri_eval_proof: blob_metadata.fri_eval_proof.clone(),
			fri_eval_prover_index: blob_metadata.fri_eval_prover_index,
		});

	let gossip_cmd_sender = friends.externalities.gossip_cmd_sender();

	if let Err(e) = gossip_cmd_sender.send(blob_received_notification).await {
		tracing::error!("internal channel closed: {e}");
		return Err(());
	}

	tracing::info!(
		"BLOB - RPC submit_blob - bg:task - After gossiping blob notif - {:?}",
		blob_hash,
	);

	Ok(())
}

#[tracing::instrument(name = "blob.store", skip_all)]
fn store_new_blob(
	blob_hash: H256,
	blob: Arc<Vec<u8>>,
	blob_metadata: &BlobMetadata,
	database: &Arc<dyn StorageApiT>,
	maybe_ownership: &Option<OwnershipEntry>,
) {
	// Arc::unwrap_or_clone will correctly unwrap as this is the only instance
	let blob = Blob {
		blob_hash,
		size: blob.len().saturated_into(),
		data: Arc::unwrap_or_clone(blob),
	};

	if let Some(o) = maybe_ownership {
		tracing::info!(
			"BLOB - RPC submit_blob - bg:task - I Should store - {:?}",
			blob_hash,
		);
		if let Err(e) = database.insert_blob_ownership(&blob_hash, o) {
			tracing::error!("failed to insert blob ownership into store: {e}");
		}
	}

	// Store the blob in the store -------------------
	if let Err(e) = database.insert_blob_metadata(blob_metadata) {
		tracing::error!("failed to insert blob metadata into store: {e}");
	}
	tracing::info!(
		"BLOB - RPC submit_blob - bg:task - After inserting metadata - {:?}",
		blob_hash,
	);

	let compressed_blob = CompressedBlob::new_zstd_compress_with_fallback(&blob.data);

	let compresion_rate = blob.data.len() as f32 / compressed_blob.raw_data().len() as f32;
	tracing::info!(compresion_rate);

	if let Err(e) = database.insert_blob(&blob.blob_hash, &compressed_blob) {
		tracing::error!("failed to insert blob into store: {e}");
	}
	tracing::info!(
		"BLOB - RPC submit_blob - bg:task - After inserting blob - {:?}",
		blob_hash,
	);
}

/*
	I'll be there for you
	(When the rain starts to pour)
	I'll be there for you
	(Like I've been there before)
	I'll be there for you
	('Cause you're there for me too)
*/
#[derive(Clone)]
pub struct Friends {
	pub externalities: Arc<dyn ExternalitiesT>,
	pub runtime_client: Arc<dyn RuntimeApiT>,
	pub backend_client: Arc<dyn BackendApiT>,
	pub tx_pool_client: Arc<dyn TransactionPoolApiT>,
	pub database: Arc<dyn StorageApiT>,
}
