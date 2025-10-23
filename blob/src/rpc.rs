use crate::traits::CommitmentQueueApiT;
use crate::validation::{commitment_validation, initial_validation, tx_validation};
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
		sign_blob_data, validators_for_blob, B64Param, CommitmentQueue, SmartStopwatch,
	},
	MAX_RPC_RETRIES,
};
use anyhow::Result;
use avail_observability::metrics::BlobMetrics;
use codec::{Decode, Encode};
use da_commitment::build_da_commitments::build_polynomial_grid;
use da_control::{pallet::BlobTxSummaryRuntime, BlobRuntimeParameters, Call};
use da_runtime::{RuntimeCall, UncheckedExtrinsic};
use frame_system::limits::BlockLength;
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::ErrorObject,
};
use sc_client_api::{BlockBackend, HeaderBackend, StateBackend};
use sc_network::PeerId;
use sc_transaction_pool_api::TransactionPool;
use sp_core::H256;
use sp_runtime::{
	traits::{Block as BlockT, HashingFor, Header as HeaderT},
	transaction_validity::TransactionSource,
	AccountId32, SaturatedConversion,
};
use std::{
	marker::{PhantomData, Sync},
	str::FromStr,
	sync::Arc,
};
use tokio::task;

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
	#[method(name = "blob_submitBlob")]
	async fn submit_blob(
		&self,
		metadata_signed_transaction: B64Param,
		blob: B64Param,
	) -> RpcResult<()>;

	#[method(name = "blob_getBlob")]
	async fn get_blob(
		&self,
		block_hash: Block::Hash,
		blob_index: u32,
		blob_hash: H256,
	) -> RpcResult<Blob>;

	#[method(name = "blob_logStuff")]
	async fn log_stuff(&self) -> RpcResult<()>;
}

pub struct BlobRpc<Pool, Block: BlockT, Backend> {
	pool: Arc<Pool>,
	backend: Arc<Backend>,
	blob_handle: Arc<BlobHandle<Block>>,
	commitment_queue: Arc<CommitmentQueue>,
	nonce_cache: Arc<NonceCache>,
	_block: PhantomData<Block>,
}

impl<Pool, Block: BlockT, Backend> BlobRpc<Pool, Block, Backend> {
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
			_block: PhantomData,
		}
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

		let now = std::time::Instant::now();
		let result = submit_blob_main_task(
			self.commitment_queue.clone(),
			metadata_signed_transaction.0,
			blob.0,
			friends,
			self.nonce_cache.clone(),
		)
		.await;
		let elapsed = now.elapsed();

		// Metrics
		BlobMetrics::inc_submissions_valid_total();
		BlobMetrics::observe_submission_rpc_duration(elapsed.as_millis() as f64 / 1000f64);

		result?;

		Ok(())
	}

	async fn get_blob(
		&self,
		block_hash: Block::Hash,
		blob_index: u32,
		blob_hash: H256,
	) -> RpcResult<Blob> {
		let Ok(Some(summaries)) =
			get_blob_tx_summaries_from_block(&self.blob_handle.client, block_hash.into()).await
		else {
			return Err(internal_err!(
				"Blob transactions summaries not found in the given block"
			));
		};

		let Some(blob_summary) = summaries.get(blob_index as usize) else {
			return Err(internal_err!(
				"Blob transaction summary not found in the given block and blob_index"
			));
		};

		if blob_summary.hash != blob_hash {
			return Err(internal_err!(
				"Blob hash mismatch - Expected: {:?} - Found: {:?}",
				blob_hash,
				blob_summary.hash
			));
		}

		if !blob_summary.success {
			return Err(internal_err!(
				"Blob update was not successful: {:?}",
				blob_summary.reason
			));
		}

		let ownership_len = blob_summary.ownership.len();
		if ownership_len == 0 {
			return Err(internal_err!("Blob ownership is empty"));
		}

		let finalized_hash = self.blob_handle.client.info().finalized_hash;

		// Take a random owner index and try to get the blob from him, retry with next ones
		let base_index =
			match generate_base_index(blob_hash, &finalized_hash.encode(), ownership_len, None) {
				Ok(i) => i,
				Err(e) => {
					return Err(internal_err!(
						"An error has occured while generating a base index: {e:?}"
					));
				},
			};

		for attempt in 0..(MAX_RPC_RETRIES as usize) {
			let index = (base_index + attempt) % ownership_len;
			let Some((_, _, encoded_peer_id, _)) = blob_summary.ownership.get(index) else {
				log::warn!(
					"Attempt {}/{}: invalid array index",
					attempt + 1,
					MAX_RPC_RETRIES
				);
				continue;
			};

			match PeerId::from_str(&encoded_peer_id) {
				Ok(peer_id) => {
					match send_blob_query_request(blob_hash, peer_id, &self.blob_handle.network)
						.await
					{
						Ok(Some(blob)) => {
							return Ok(blob);
						},
						Ok(None) => {
							log::warn!(
								"attempt {}/{}: no blob returned",
								attempt + 1,
								MAX_RPC_RETRIES
							);
						},
						Err(e) => {
							log::warn!(
								"attempt {}/{} RPC error: {:?}",
								attempt + 1,
								MAX_RPC_RETRIES,
								e
							);
						},
					}
				},
				Err(e) => {
					log::warn!("Attempt {}: invalid peer_id {:?}", attempt + 1, e);
				},
			}
		}

		Err(internal_err!(
			"All attempts to get the blob from validators failed."
		))
	}

	async fn log_stuff(&self) -> RpcResult<()> {
		let _ = self.blob_handle.blob_database.log_all_entries();
		Ok(())
	}
}

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

async fn get_blob_tx_summaries_from_block<Client, Block>(
	client: &Arc<Client>,
	block_hash: Block::Hash,
) -> RpcResult<Option<Vec<BlobTxSummaryRuntime>>>
where
	Block: BlockT,
	<Block as BlockT>::Extrinsic: Encode,
	Client: HeaderBackend<Block> + BlockBackend<Block> + Send + Sync + 'static,
{
	let block = client
		.block(block_hash)
		.map_err(|e| internal_err!("Failed to get block: {e}"))?
		.ok_or_else(|| internal_err!("Block not found"))?;

	let extrinsics = block.block.extrinsics();
	if extrinsics.len() < 2 {
		return Ok(None);
	}

	let summary_extrinsic_encoded = extrinsics[extrinsics.len() - 2].encode();
	let summary_extrinsic: UncheckedExtrinsic = Decode::decode(&mut &summary_extrinsic_encoded[..])
		.map_err(|_| internal_err!("Failed to decode summary extrinsic"))?;

	if let RuntimeCall::DataAvailability(Call::submit_blob_txs_summary {
		blob_txs_summary, ..
	}) = summary_extrinsic.function
	{
		Ok(Some(blob_txs_summary))
	} else {
		Ok(None)
	}
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

pub async fn submit_blob_main_task(
	commitment_queue: Arc<dyn CommitmentQueueApiT>,
	metadata_signed_transaction: Vec<u8>,
	blob: Vec<u8>,
	friends: Friends,
	nonce_cache: Arc<dyn NonceCacheApiT>,
) -> RpcResult<tokio::task::JoinHandle<()>> {
	let mut stop_watch = SmartStopwatch::new("😍 Submit Blob Main Task");

	let runtime_client = friends.runtime_client.clone();

	// Get client info
	let client_info = friends.externalities.client_info();
	let best_hash = client_info.best_hash;
	let finalized_block_hash = client_info.finalized_hash;

	let blob_params = match runtime_client.get_blob_runtime_parameters(finalized_block_hash) {
		Ok(p) => p,
		Err(e) => {
			log::error!("Could not get blob_params: {e:?}");
			BlobRuntimeParameters::default()
		},
	};
	let max_blob_size = blob_params.max_blob_size as usize;

	stop_watch.start("Initial Validation");
	let (blob_hash, provided_commitment) =
		initial_validation(max_blob_size as usize, &blob, &metadata_signed_transaction)
			.map_err(|e| internal_err!("{}", e))?;
	stop_watch.stop("Initial Validation");
	stop_watch.add_extra_information(std::format!("Blob Hash: {:?}", blob_hash));

	// Telemetry
	crate::telemetry::blob_received(blob.len(), blob_hash);

	stop_watch.start("TX validation");
	let opaque_tx = tx_validation(
		best_hash,
		&metadata_signed_transaction,
		blob_params.min_transaction_validity,
		blob_params.max_transaction_validity,
		&runtime_client,
		&nonce_cache,
	)
	.map_err(|e| internal_err!("{}", e))?;
	stop_watch.stop("TX validation");

	// Commitment Validation can take a long time.
	stop_watch.start("Commitments (Total)");
	let (cols, rows) = get_dynamic_block_length(&friends.backend_client, finalized_block_hash)?;
	let blob = Arc::new(blob);

	let start = crate::utils::get_current_timestamp_ms();
	stop_watch.start("Polynominal Grid Gen.");
	let grid = build_polynomial_grid(&*blob, cols, rows, Default::default());
	stop_watch.stop("Polynominal Grid Gen.");
	let end = crate::utils::get_current_timestamp_ms();
	// Telemetry
	crate::telemetry::blob_poly_grid(blob_hash, start, end);

	stop_watch.start("Commitment Validation");
	commitment_validation(blob_hash, &provided_commitment, grid, &commitment_queue)
		.await
		.map_err(|e| internal_err!("{}", e))?;
	stop_watch.stop("Commitment Validation");

	stop_watch.stop("Commitments (Total)");

	// Because Commitment Validation can take a long time
	// the moment it is done minutes can pass.
	// Let's check once more to see if the transactions is still valid
	//
	// TODO Blob we might remove this
	let client_info = friends.externalities.client_info();
	let best_hash = client_info.best_hash;

	let _ = tx_validation(
		best_hash,
		&metadata_signed_transaction,
		blob_params.min_transaction_validity,
		blob_params.max_transaction_validity,
		&runtime_client,
		&nonce_cache,
	)
	.map_err(|e| internal_err!("{}", e))?;

	// From this point, the transaction should not fail as the user has done everything correctly
	// We will spawn a task to finish the work and instantly return to the user.
	let handle = task::spawn(async move {
		submit_blob_background_task(
			opaque_tx,
			blob_hash,
			blob,
			blob_params,
			provided_commitment,
			friends,
			nonce_cache,
		)
		.await
	});

	Ok(handle)
}

async fn submit_blob_background_task(
	opaque_tx: UncheckedExtrinsic,
	blob_hash: H256,
	blob: Arc<Vec<u8>>,
	blob_params: BlobRuntimeParameters,
	commitment: Vec<u8>,
	friends: Friends,
	nonce_cache: Arc<dyn NonceCacheApiT>,
) {
	let blob_len = blob.len();

	if let Some((who, nonce)) = extract_signer_and_nonce(&opaque_tx) {
		nonce_cache.commit(&who, nonce);
	}

	let stored = store_and_gossip_blob(blob_hash, blob, blob_params, commitment, &friends).await;
	if stored.is_err() {
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
		log::error!("tx-pool error: {e}")
	}

	log::info!(
		"BLOB - RPC submit_blob - bg:task - After Submitting to pool - {:?}",
		blob_hash,
	);

	// Metrics and Telemetry
	BlobMetrics::inc_submissions_added_to_pool_total();
	BlobMetrics::inc_submissions_blob_size_pool_total(blob_len as u64);
	crate::telemetry::blob_added_to_pool(blob_len, blob_hash);
}

pub async fn store_and_gossip_blob(
	blob_hash: H256,
	blob: Arc<Vec<u8>>,
	blob_params: BlobRuntimeParameters,
	commitment: Vec<u8>,
	friends: &Friends,
) -> Result<(), ()> {
	let mut stop_watch = SmartStopwatch::new("😍😍 STORE AND GOSSIP BLOB");
	stop_watch.add_extra_information(std::format!("Blob Hash: {:?}", blob_hash));

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
			log::error!("Failed to get data from blob storage: {e}");
			return Err(());
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
			log::error!("{}", err);
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
			log::error!("{}", err);
			return Err(());
		},
	};
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
			log::error!("could not check if rpc should store blob: {e}");
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
		&mut stop_watch,
	);

	stop_watch.start("Gossiping");

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
		});

	let gossip_cmd_sender = friends.externalities.gossip_cmd_sender();

	if let Err(e) = gossip_cmd_sender.send(blob_received_notification).await {
		log::error!("internal channel closed: {e}");
		return Err(());
	}
	log::info!(
		"BLOB - RPC submit_blob - bg:task - After gossiping blob notif - {:?}",
		blob_hash,
	);
	stop_watch.stop("Gossiping");

	Ok(())
}

fn store_new_blob(
	blob_hash: H256,
	blob: Arc<Vec<u8>>,
	blob_metadata: &BlobMetadata,
	database: &Arc<dyn StorageApiT>,
	maybe_ownership: &Option<OwnershipEntry>,
	stop_watch: &mut SmartStopwatch,
) {
	stop_watch.start("Storing Blob");

	// Arc::unwrap_or_clone will correctly unwrap as this is the only instance
	let blob = Blob {
		blob_hash,
		size: blob.len().saturated_into(),
		data: Arc::unwrap_or_clone(blob),
	};

	if let Some(o) = maybe_ownership {
		log::info!(
			"BLOB - RPC submit_blob - bg:task - I Should store - {:?}",
			blob_hash,
		);
		if let Err(e) = database.insert_blob_ownership(&blob_hash, o) {
			log::error!("failed to insert blob ownership into store: {e}");
		}
	}

	// Store the blob in the store -------------------
	if let Err(e) = database.insert_blob_metadata(blob_metadata) {
		log::error!("failed to insert blob metadata into store: {e}");
	}
	log::info!(
		"BLOB - RPC submit_blob - bg:task - After inserting metadata - {:?}",
		blob_hash,
	);

	stop_watch.start("Compression");
	let compressed_blob = CompressedBlob::new_zstd_compress_with_fallback(&blob.data);
	let duration = stop_watch.stop("Compression");

	// Telemetry
	crate::telemetry::blob_compression(
		blob.data.len(),
		compressed_blob.raw_data().len(),
		blob_hash,
		duration,
	);

	stop_watch.add_extra_information(std::format!(
		"Compresion rate: {}",
		blob.data.len() as f32 / compressed_blob.raw_data().len() as f32
	));

	if let Err(e) = database.insert_blob(&blob.blob_hash, &compressed_blob) {
		log::error!("failed to insert blob into store: {e}");
	}
	log::info!(
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
