use crate::traits::{ExternalitiesT, RealExternalities};
use crate::types::CompressedBlob;
use crate::utils::B64Param;
use crate::{
	p2p::BlobHandle,
	send_blob_query_request,
	store::BlobStore,
	types::{Blob, BlobMetadata, BlobNotification, BlobReceived, Deps, OwnershipEntry},
	utils::{
		build_signature_payload, check_store_blob, generate_base_index,
		get_dynamic_blocklength_key, get_my_validator_id, get_validator_per_blob_inner,
		sign_blob_data_inner, CommitmentQueue, CommitmentQueueMessage, SmartStopwatch,
	},
	MAX_RPC_RETRIES,
};
use anyhow::{anyhow, Result};
use codec::{Decode, Encode};
use da_commitment::build_da_commitments::build_polynomal_grid;
use da_control::{pallet::BlobTxSummaryRuntime, BlobRuntimeParameters, Call};
use da_runtime::{apis::BlobApi, RuntimeCall, UncheckedExtrinsic};
use frame_system::limits::BlockLength;
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::ErrorObject,
};
use kate::Seed;
use sc_client_api::{BlockBackend, HeaderBackend, StateBackend};
use sc_network::PeerId;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ApiError;
use sp_api::ProvideRuntimeApi;
use sp_core::{keccak_256, H256};
use sp_runtime::transaction_validity::TransactionValidityError;
use sp_runtime::transaction_validity::ValidTransaction;
use sp_runtime::{
	traits::{Block as BlockT, HashingFor, Header as HeaderT},
	SaturatedConversion,
};
use sp_runtime::{transaction_validity::TransactionSource, AccountId32};
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
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

pub struct BlobRpc<Client, Pool, Block: BlockT, Backend> {
	client: Arc<Client>,
	pool: Arc<Pool>,
	blob_handle: Arc<BlobHandle<Block>>,
	backend: Arc<Backend>,
	commitment_queue: Arc<CommitmentQueue>,
	_block: PhantomData<Block>,
}

impl<Client, Pool, Block: BlockT, Backend> BlobRpc<Client, Pool, Block, Backend> {
	pub fn new(
		client: Arc<Client>,
		pool: Arc<Pool>,
		deps: Deps<Block>,
		backend: Arc<Backend>,
	) -> Self {
		Self {
			client,
			pool,
			blob_handle: deps.blob_handle,
			backend,
			commitment_queue: Arc::new(CommitmentQueue::new(100)),
			_block: PhantomData,
		}
	}
}

#[async_trait]
impl<Client, Pool, Block, Backend> BlobApiServer<Block> for BlobRpc<Client, Pool, Block, Backend>
where
	Block: BlockT,
	Client: HeaderBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	Client::Api: TaggedTransactionQueue<Block> + BlobApi<Block>,
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
		let _stop_watch = SmartStopwatch::new("😍 SUBMIT BLOB RPC");

		// --- 0. Quick checks -------------------------------------------------
		if blob.0.is_empty() {
			return Err(internal_err!("blob cannot be empty"));
		}
		if metadata_signed_transaction.0.is_empty() {
			return Err(internal_err!("metadata tx cannot be empty"));
		}

		let externalities = RealExternalities::new(
			self.client.clone(),
			self.pool.clone(),
			self.blob_handle.clone(),
			self.backend.clone(),
		);

		submit_blob_inner(
			self.client.clone(),
			self.blob_handle.clone(),
			self.commitment_queue.clone(),
			metadata_signed_transaction.0,
			blob.0,
			Arc::new(externalities),
		)
		.await
	}

	async fn get_blob(
		&self,
		block_hash: Block::Hash,
		blob_index: u32,
		blob_hash: H256,
	) -> RpcResult<Blob> {
		let network = self
			.blob_handle
			.network
			.get()
			.ok_or_else(|| internal_err!("Could not get network to get my peer id"))
			.map_err(|e| e)?;

		let Ok(Some(summaries)) = get_blob_tx_summaries_from_block(&self.client, block_hash).await
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

		let finalized_hash = self.client.info().finalized_hash;

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
				Ok(peer_id) => match send_blob_query_request(blob_hash, peer_id, &network).await {
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
		let _ = self.blob_handle.blob_store.log_all_entries();
		Ok(())
	}
}

async fn check_rpc_store_blob<Client, Block>(
	client: &Arc<Client>,
	blob_handle: &BlobHandle<Block>,
	blob_metadata: &BlobMetadata<Block>,
	my_encoded_peer_id: String,
	nb_validators_per_blob: u32,
	validators: &Vec<AccountId32>,
	finalized_block_hash: &Block::Hash,
) -> Result<Option<OwnershipEntry>>
where
	Block: BlockT,
	Client: HeaderBackend<Block> + ProvideRuntimeApi<Block>,
	Client::Api: BlobApi<Block>,
{
	let role = &blob_handle.role;
	if !role.is_authority() {
		// RPC node (me) is not an authority, so I don't have to store blobs
		return Ok(None);
	}

	let keystore = blob_handle.keystore.get();
	let Some(keystore) = keystore else {
		return Err(anyhow!("failed to get keystore"));
	};

	let Some((my_validator_id, babe_key)) =
		get_my_validator_id(keystore, client, &finalized_block_hash.encode())
	else {
		return Ok(None);
	};

	let should_store_blob = match check_store_blob(
		blob_metadata.hash,
		validators,
		&my_validator_id,
		&blob_metadata.finalized_block_hash.encode(),
		nb_validators_per_blob,
	) {
		Ok(s) => s,
		Err(e) => {
			return Err(anyhow!("failed to check if I should hold a blob: {e}"));
		},
	};

	if !should_store_blob {
		return Ok(None);
	}

	let signature_payload = build_signature_payload(
		blob_metadata.hash,
		[my_validator_id.encode(), b"stored".to_vec()].concat(),
	);
	let signature = match sign_blob_data_inner(keystore, signature_payload) {
		Ok(s) => s.signature,
		Err(e) => {
			return Err(anyhow!(
				"An error has occured while trying to sign data, exiting the function: {e}"
			));
		},
	};

	Ok(Some(OwnershipEntry {
		address: my_validator_id,
		babe_key,
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
	externalities: &Arc<dyn ExternalitiesT>,
	finalized_block_hash: H256,
) -> RpcResult<(usize, usize)> {
	let storage_key = get_dynamic_blocklength_key();
	let maybe_raw = externalities
		.storage(finalized_block_hash, &storage_key.0)
		.map_err(|e| internal_err!("Storage query error: {e:?}"))?;
	let raw = maybe_raw.ok_or(internal_err!("DynamicBlockLength not found"))?;
	let block_length =
		BlockLength::decode(&mut &raw[..]).map_err(|e| internal_err!("Decode error: {e:?}"))?;
	let cols = block_length.cols.0 as usize;
	let rows = block_length.rows.0 as usize;

	Ok((cols, rows))
}

fn initial_validation(
	max_blob_size: usize,
	blob: &[u8],
	metadata: &[u8],
) -> RpcResult<(H256, Vec<u8>)> {
	if blob.len() > max_blob_size {
		return Err(internal_err!("blob is too big"));
	}

	let mut metadata = metadata;
	let encoded_metadata_signed_transaction: UncheckedExtrinsic = Decode::decode(&mut metadata)
		.map_err(|_| internal_err!("failed to decode concrete metadata call"))?;
	let (provided_size, provided_blob_hash, provided_commitment) =
		match encoded_metadata_signed_transaction.function {
			RuntimeCall::DataAvailability(Call::submit_blob_metadata {
				size,
				blob_hash,
				commitment,
			}) => (size as usize, blob_hash, commitment),
			_ => {
				return Err(internal_err!(
					"metadata extrinsic must be dataAvailability.submitBlobMetadata"
				))
			},
		};

	// Check size
	if provided_size != blob.len() {
		return Err(internal_err!(
			"submit data length ({}) != blob length ({})",
			provided_size,
			blob.len()
		));
	}

	let blob_hash = H256::from(keccak_256(&blob));
	if provided_blob_hash != blob_hash {
		return Err(internal_err!("submitted blob: {provided_blob_hash:?} does not correspond to generated blob {blob_hash:?}"));
	}

	Ok((blob_hash, provided_commitment))
}

fn tx_validation<Block: BlockT>(
	metadata: &[u8],
	min_transaction_validity: u64,
	max_transaction_validity: u64,
	validity_op: impl Fn(
		UncheckedExtrinsic,
	) -> Result<Result<ValidTransaction, TransactionValidityError>, ApiError>,
) -> RpcResult<UncheckedExtrinsic> {
	let mut metadata = metadata;

	// --- a. Decode the opaque extrinsic ---------------------------------
	let opaque_tx: UncheckedExtrinsic = codec::Decode::decode(&mut metadata)
		.map_err(|_| internal_err!("failed to decode metadata extrinsic"))?;
	// --- b. Let the runtime validate it (signature, nonce, weight) ------
	let opaque_tx_clone = opaque_tx.clone();
	let validity = validity_op(opaque_tx_clone)
		.map_err(|e| internal_err!("runtime validate_transaction error: {e}"))?;
	let Ok(validity) = validity else {
		return Err(internal_err!("metadata extrinsic rejected by runtime"));
	};
	// --- c. Check also that transaction lifetime is above minimum tx lifetime so it does not expire. If validity is not correct, we reject the tx
	if validity.longevity < min_transaction_validity {
		return Err(internal_err!(
			"signed transaction does not live for enough time"
		));
	}
	if validity.longevity > max_transaction_validity {
		return Err(internal_err!("signed transaction lifetime is too long"));
	}

	Ok(opaque_tx)
}

async fn commitment_validation(
	provided_commitment: &Vec<u8>,
	blob: Arc<Vec<u8>>,
	cols: usize,
	rows: usize,
	queue: &CommitmentQueue,
	stop_watch: &mut SmartStopwatch,
) -> RpcResult<()> {
	stop_watch.start_tracking("Build Poly Grid");
	let grid = build_polynomal_grid(&*blob, cols, rows, Seed::default());
	stop_watch.stop_tracking("Build Poly Grid", "");

	let (message, rx_comm) = CommitmentQueueMessage::new(grid);
	if !queue.send(message) {
		// Need better error handling
		return Err(internal_err!("Commitment queue is full"));
	}
	let commitment = match rx_comm.await {
		Ok(x) => x,
		Err(_) => {
			return Err(internal_err!(
				"Cannot compute commitment. :(  Channel is down"
			));
		},
	};

	// Check comitment
	if !provided_commitment.eq(&commitment) {
		return Err(internal_err!("submitted blob commitment mismatch"));
	}

	Ok(())
}

pub async fn store_and_gossip_blob<Block, Client>(
	client: Arc<Client>,
	opaque_tx: UncheckedExtrinsic,
	blob_hash: H256,
	blob_handle: Arc<BlobHandle<Block>>,
	blob: Arc<Vec<u8>>,
	blob_params: BlobRuntimeParameters,
	commitment: Vec<u8>,
	externalities: Arc<dyn ExternalitiesT>,
) where
	Client: HeaderBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	Client::Api: TaggedTransactionQueue<Block> + BlobApi<Block>,
	Block: BlockT,
	H256: From<<Block as BlockT>::Hash>,
	<Block as BlockT>::Hash: From<H256>,
	<Block as BlockT>::Extrinsic: From<UncheckedExtrinsic>,
{
	const GOSSIPING: &str = "GOSSIPING";
	let mut stop_watch = SmartStopwatch::new("😍😍 STORE AND GOSSIP BLOB");
	stop_watch.add_extra_information(std::format!("Blob Hash: {:?}", blob_hash));

	let client_info = externalities.client_info();
	let finalized_block_hash = client_info.finalized_hash;
	let finalized_block_number = client_info.finalized_height as u64;

	// Get my own peer id data
	let Ok(my_peer_id) = externalities.local_peer_id() else {
		log::error!("submit_blob(bg): network not initialized");
		return;
	};

	let my_peer_id_base58 = my_peer_id.to_base58();

	// Setup blob metadata and blob and check first in case we already received this exact blob before
	let maybe_blob_metadata = match blob_handle.blob_store.get_blob_metadata(&blob_hash) {
		Ok(m) => m,
		Err(e) => {
			log::error!("Failed to get data from blob storage: {e}");
			return;
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
			finalized_block_hash: Block::Hash::default(),
			finalized_block_number: 0,
			nb_validators_per_blob: 0,
			nb_validators_per_blob_threshold: 0,
		}
	});

	// It might be a new blob or an old one being resubmitted, we still update most of the values
	let validators = match externalities.get_active_validators(finalized_block_hash) {
		Ok(validators) => validators,
		Err(e) => {
			log::error!(
				"Failed to fetch active validators at {:?}: {:?}",
				finalized_block_hash,
				e
			);
			Vec::new()
		},
	};

	if validators.is_empty() {
		return;
	}
	let (nb_validators_per_blob, threshold) =
		get_validator_per_blob_inner(blob_params.clone(), validators.len() as u32);
	blob_metadata.is_notified = true;
	blob_metadata.expires_at = finalized_block_number.saturating_add(blob_params.temp_blob_ttl);
	blob_metadata.finalized_block_hash = finalized_block_hash.into();
	blob_metadata.finalized_block_number = finalized_block_number;
	blob_metadata.nb_validators_per_blob = nb_validators_per_blob;
	blob_metadata.nb_validators_per_blob_threshold = threshold;
	let block_hash = finalized_block_hash.into();
	let maybe_ownership: Option<OwnershipEntry> = match check_rpc_store_blob::<Client, Block>(
		&client,
		&blob_handle,
		&blob_metadata,
		my_peer_id_base58.clone(),
		nb_validators_per_blob,
		&validators,
		&block_hash,
	)
	.await
	{
		Ok(o) => o,
		Err(e) => {
			log::error!("could not check if rpc should store blob: {e}");
			return;
		},
	};

	stop_watch.start_tracking("STORAGE");

	// Arc::unwrap_or_clone will correctly unwrap as this is the only instance
	let blob = Blob {
		blob_hash,
		size: blob.len().saturated_into(),
		data: Arc::unwrap_or_clone(blob),
	};

	if let Some(o) = &maybe_ownership {
		log::info!(
			"BLOB - RPC submit_blob - bg:task - I Should store - {:?}",
			blob_hash,
		);
		if let Err(e) = blob_handle.blob_store.insert_blob_ownership(&blob_hash, o) {
			log::error!("failed to insert blob ownership into store: {e}");
		}
		blob_metadata.expires_at = finalized_block_number.saturating_add(blob_params.blob_ttl);
	}

	// Store the blob in the store -------------------
	if let Err(e) = blob_handle.blob_store.insert_blob_metadata(&blob_metadata) {
		log::error!("failed to insert blob metadata into store: {e}");
	}
	log::info!(
		"BLOB - RPC submit_blob - bg:task - After inserting metadata - {:?}",
		blob_hash,
	);

	let compressed_blob = CompressedBlob::new_zstd_compress_with_fallback(&blob.data);

	if let Err(e) = blob_handle
		.blob_data_store
		.insert_blob(&blob.blob_hash, &compressed_blob)
	{
		log::error!("failed to insert blob into store: {e}");
	}
	log::info!(
		"BLOB - RPC submit_blob - bg:task - After inserting blob - {:?}",
		blob_hash,
	);

	stop_watch.stop_tracking("STORAGE", "");
	stop_watch.start_tracking(GOSSIPING);

	// Announce the blob to the network -------------------
	let blob_received_notification: BlobNotification<Block> =
		BlobNotification::BlobReceived(BlobReceived {
			hash: blob_metadata.hash,
			size: blob_metadata.size,
			commitment: blob_metadata.commitment.clone(),
			ownership: maybe_ownership,
			original_peer_id: my_peer_id_base58.clone(),
			finalized_block_hash: finalized_block_hash.into(),
			finalized_block_number,
		});

	let Some(gossip_cmd_sender) = blob_handle.gossip_cmd_sender.get() else {
		log::error!("gossip_cmd_sender was not initialized");
		return;
	};

	if let Err(e) = gossip_cmd_sender.send(blob_received_notification).await {
		log::error!("internal channel closed: {e}");
		return;
	}
	log::info!(
		"BLOB - RPC submit_blob - bg:task - After gossiping blob notif - {:?}",
		blob_hash,
	);
	stop_watch.stop_tracking(GOSSIPING, "");

	// Push the clean extrinsic to the tx pool ---------------------
	// Get the best hash once more, to submit the tx
	stop_watch.start_tracking("Submit One");
	let best_hash = externalities.client_info().best_hash;
	if let Err(e) = externalities
		.submit_one(best_hash, TransactionSource::External, opaque_tx)
		.await
	{
		log::error!("tx-pool error: {e}")
	}
	log::info!(
		"BLOB - RPC submit_blob - bg:task - After Submitting to pool - {:?}",
		blob_hash,
	);
	stop_watch.stop_tracking("Submit One", "");
}

pub async fn submit_blob_inner<Client, Block>(
	client: Arc<Client>,
	blob_handle: Arc<BlobHandle<Block>>,
	commitment_queue: Arc<CommitmentQueue>,
	metadata_signed_transaction: Vec<u8>,
	blob: Vec<u8>,
	externalities: Arc<dyn ExternalitiesT>,
) -> RpcResult<()>
where
	Block: BlockT,
	Client: HeaderBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	Client::Api: TaggedTransactionQueue<Block> + BlobApi<Block>,
	H256: From<<Block as BlockT>::Hash>,
	<Block as BlockT>::Hash: From<H256>,
	<Block as BlockT>::Extrinsic: From<UncheckedExtrinsic>,
{
	let mut stop_watch = SmartStopwatch::new("😍 SUBMIT BLOB Inner");

	// Get client info
	let client_info = externalities.client_info();
	let best_hash = client_info.best_hash;
	let finalized_block_hash = client_info.finalized_hash;

	let blob_params = match externalities.get_blob_runtime_parameters(finalized_block_hash) {
		Ok(p) => p,
		Err(e) => {
			log::error!("Could not get blob_params: {e:?}");
			BlobRuntimeParameters::default()
		},
	};
	let max_blob_size = blob_params.max_blob_size as usize;

	stop_watch.start_tracking("Initial Validation");
	let (blob_hash, provided_commitment) =
		initial_validation(max_blob_size as usize, &blob, &metadata_signed_transaction)?;
	stop_watch.stop_tracking("Initial Validation", "");
	stop_watch.add_extra_information(std::format!("Blob Hash: {:?}", blob_hash));

	let e = externalities.clone();
	let validity_op = move |uxt: UncheckedExtrinsic| {
		e.validate_transaction(best_hash, TransactionSource::External, uxt, best_hash)
	};

	stop_watch.start_tracking("TX validation");
	let opaque_tx = tx_validation::<Block>(
		&metadata_signed_transaction,
		blob_params.min_transaction_validity,
		blob_params.max_transaction_validity,
		validity_op,
	)?;
	stop_watch.stop_tracking("TX validation", "");

	// Commitment Validation can take a long time.
	stop_watch.start_tracking("Commitment Validation");
	let (cols, rows) = get_dynamic_block_length(&externalities, finalized_block_hash)?;
	let blob = Arc::new(blob);
	commitment_validation(
		&provided_commitment,
		blob.clone(),
		cols,
		rows,
		&commitment_queue,
		&mut stop_watch,
	)
	.await?;
	stop_watch.stop_tracking("Commitment Validation", "");

	// Because Commitment Validation can take a long time
	// the moment it is done minutes can pass.
	// Let's check once more to see if the transactions is still valid
	//
	// TODO we might remove this
	let client_info = externalities.client_info();
	let best_hash = client_info.best_hash;
	let e = externalities.clone();
	let validity_op = move |uxt: UncheckedExtrinsic| {
		e.validate_transaction(best_hash, TransactionSource::External, uxt, best_hash)
	};
	let _ = tx_validation::<Block>(
		&metadata_signed_transaction,
		blob_params.min_transaction_validity,
		blob_params.max_transaction_validity,
		validity_op,
	)?;

	// From this point, the transaction should not fail as the user has done everything correctly
	// We will spawn a task to finish the work and instantly return to the user.
	{
		task::spawn(async move {
			store_and_gossip_blob(
				client,
				opaque_tx,
				blob_hash,
				blob_handle,
				blob,
				blob_params,
				provided_commitment,
				externalities,
			)
			.await
		});
	}

	Ok(())
}
