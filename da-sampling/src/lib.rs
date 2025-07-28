#![allow(dead_code)]
pub mod da_voting_rule;
mod schema;

use avail_core::{header::HeaderExtension, OpaqueExtrinsic};
use da_runtime::{
	apis::{DataAvailApi, KateApi},
	Hash, Header as DaHeader,
};
use futures::{
	channel::oneshot,
	stream::{BoxStream, StreamExt},
};
use kate::{couscous::multiproof_params, gridgen::core::AsBytes};
use kate_recovery::{
	commons::{ArkCommitment, ArkPublicParams},
	data::SingleCell,
	matrix::{Dimensions, Position},
	proof::verify_v2,
};
use libp2p_identity::PeerId;
use log::{debug, error, trace, warn};
use lru::LruCache;
use parking_lot::RwLock;
use prost::Message;
use rand::{thread_rng, Rng};
use sc_chain_spec::ChainSpec;
use sc_client_api::{BlockBackend, BlockImportNotification};
use sc_network::{
	request_responses::{IfDisconnected, IncomingRequest, OutgoingResponse, ProtocolConfig},
	types::ProtocolName,
	NetworkRequest, NetworkService,
};
use schema::v1::da_sampling::*;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_consensus::BlockOrigin;
use sp_runtime::traits::{Block as BlockT, Header, PhantomData};
use std::sync::Mutex;
use std::{
	collections::{HashMap, HashSet},
	io,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
	time::{Duration, Instant},
};
use tokio::time::{sleep, timeout};

const LOG_TARGET: &str = "da-sampling";
const MAX_PACKET_SIZE: u64 = 16 * 1024 * 1024; // Match Substrate protocol max
const MAX_REQUEST_QUEUE: usize = 100;
const NAME: &str = "/da-sampling/1";
const CELL_COUNT: u32 = 14; // Number of cells required for 99.99% confidence
const PEER_RESPONSE_TIMEOUT: Duration = Duration::from_secs(120);

static PP: std::sync::OnceLock<ArkPublicParams> = std::sync::OnceLock::new();

/// Block verification status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockVerificationStatus {
	/// Downloader has seen the block but verification not started yet
	Pending,
	/// Downloader has created a request & verification in progress
	InProgress,
	/// Verification succeeded
	Verified,
	/// Verification failed
	Failed,
	/// Verification timed out
	TimedOut,
}

/// DA sampling protocol errors
#[derive(Debug, thiserror::Error)]
pub enum SamplingError {
	#[error(transparent)]
	Client(#[from] sp_blockchain::Error),

	#[error("Failed to send response.")]
	SendResponse,

	#[error("Request failed: {0}")]
	RequestFailure(String),

	#[error("Response decode error: {0}")]
	ResponseDecode(#[from] prost::DecodeError),

	#[error(transparent)]
	Encode(#[from] prost::EncodeError),

	#[error(transparent)]
	Io(#[from] io::Error),

	#[error(transparent)]
	Api(#[from] sp_api::ApiError),

	#[error("Verification failed for block")]
	VerificationFailed,

	#[error("Operation timed out")]
	Timeout,

	#[error("No peers available")]
	NoPeersAvailable,
}

/// Verification state for a block
#[derive(Debug, Clone)]
struct BlockVerificationState {
	status: BlockVerificationStatus,
	last_attempt: Option<Instant>,
	retry_count: u32,
	failed_cells: Vec<CellCoordinate>,
	cells: Option<Vec<CellCoordinate>>,
}

/// Shared state for block's DA verification
#[derive(Debug, Clone)]
pub struct VerificationTracker {
	blocks: Arc<RwLock<HashMap<Hash, BlockVerificationState>>>,
	shutdown: Arc<AtomicBool>,
}

impl VerificationTracker {
	pub fn new() -> Self {
		Self {
			blocks: Arc::new(RwLock::new(HashMap::new())),
			shutdown: Arc::new(AtomicBool::new(false)),
		}
	}

	pub fn set_status(&self, block_hash: Hash, status: BlockVerificationStatus) {
		let mut blocks = self.blocks.write();
		blocks
			.entry(block_hash)
			.and_modify(|state| {
				state.status = status;
				if status == BlockVerificationStatus::InProgress {
					state.last_attempt = Some(Instant::now());
				}
			})
			.or_insert_with(|| BlockVerificationState {
				status,
				last_attempt: if status == BlockVerificationStatus::InProgress {
					Some(Instant::now())
				} else {
					None
				},
				retry_count: 0,
				failed_cells: Vec::new(),
				cells: None,
			});
	}

	pub fn get_status(&self, block_hash: &Hash) -> Option<BlockVerificationStatus> {
		self.blocks.read().get(block_hash).map(|s| s.status)
	}

	pub fn record_failed_cells(&self, block_hash: Hash, cells: Vec<CellCoordinate>) {
		let mut blocks = self.blocks.write();
		blocks.entry(block_hash).and_modify(|state| {
			state.failed_cells.extend(cells);
		});
	}

	pub fn has_failed_cells(&self, block_hash: &Hash) -> bool {
		self.blocks
			.read()
			.get(block_hash)
			.map_or(false, |state| !state.failed_cells.is_empty())
	}

	pub fn increment_retry(&self, block_hash: Hash) {
		let mut blocks = self.blocks.write();
		blocks.entry(block_hash).and_modify(|state| {
			state.retry_count += 1;
			state.last_attempt = Some(Instant::now());
		});
	}

	pub fn set_cells(&self, block_hash: Hash, cells: Vec<CellCoordinate>) {
		let mut blocks = self.blocks.write();
		blocks
			.entry(block_hash)
			.and_modify(|state| {
				state.cells = Some(cells.clone());
			})
			.or_insert_with(|| BlockVerificationState {
				status: BlockVerificationStatus::Pending,
				last_attempt: None,
				retry_count: 0,
				failed_cells: Vec::new(),
				cells: Some(cells),
			});
	}

	pub fn get_cells(&self, block_hash: &Hash) -> Option<Vec<CellCoordinate>> {
		self.blocks
			.read()
			.get(block_hash)
			.and_then(|s| s.cells.clone())
	}

	pub fn should_shutdown(&self) -> bool {
		self.shutdown.load(Ordering::SeqCst)
	}

	pub fn shutdown(&self) {
		self.shutdown.store(true, Ordering::SeqCst);
	}
}

/// Key for caching sampling proofs
#[derive(Clone, PartialEq, Eq, Hash)]
struct SamplingCacheKey {
	block_hash: Hash,
	cells: Vec<CellCoordinate>,
}

impl SamplingCacheKey {
	fn new(block_hash: Hash, cells: &[CellCoordinate]) -> Self {
		let mut cells = cells.to_vec();
		cells.sort_unstable_by(|a, b| (a.row, a.col).cmp(&(b.row, b.col)));
		Self { block_hash, cells }
	}
}

/// DA samples request handler (server side)
pub struct DaSamplesRequestHandler<B, Client> {
	client: Arc<Client>,
	request_receiver: async_channel::Receiver<IncomingRequest>,
	block: PhantomData<B>,
	cache: Mutex<LruCache<SamplingCacheKey, Vec<u8>>>,
}

impl<B, Client> DaSamplesRequestHandler<B, Client>
where
	B: BlockT<Header = DaHeader, Hash = Hash, Extrinsic = OpaqueExtrinsic>,
	Client: Send + Sync + 'static + ProvideRuntimeApi<B> + BlockBackend<B>,
	Client::Api: DataAvailApi<B> + KateApi<B>,
{
	pub fn new<Hash: AsRef<[u8]>>(
		client: Arc<Client>,
		genesis_hash: Hash,
		chain_spec: &Box<dyn ChainSpec>,
	) -> (Self, ProtocolConfig) {
		let (tx, request_receiver) = async_channel::bounded(MAX_REQUEST_QUEUE);
		let protocol_name = get_protocol_name(genesis_hash, chain_spec);

		let config = ProtocolConfig {
			name: protocol_name.clone(),
			fallback_names: vec![],
			max_request_size: MAX_PACKET_SIZE,
			max_response_size: MAX_PACKET_SIZE,
			request_timeout: PEER_RESPONSE_TIMEOUT,
			inbound_queue: Some(tx),
		};

		let cache = Mutex::new(LruCache::new(100));

		(
			Self {
				client,
				request_receiver,
				block: PhantomData,
				cache,
			},
			config,
		)
	}

	pub async fn run(mut self) {
		debug!(target: LOG_TARGET, "Starting DA sampling request handler");

		while let Some(request) = self.request_receiver.next().await {
			let IncomingRequest {
				peer,
				payload,
				pending_response,
			} = request;
			let start_time = Instant::now();
			let response = self.handle_message(&peer, &payload);
			let duration = start_time.elapsed();
			let result = match response {
				Ok(data) => {
					debug!(
						target: LOG_TARGET,
						"Handled request from {peer} in {:?}",
						duration
					);
					Ok(data)
				},
				Err(e) => {
					error!(
						target: LOG_TARGET,
						"Failed to handle request from {peer}: {e} (took {:?})",
						duration
					);
					Err(())
				},
			};

			if let Err(e) = pending_response.send(OutgoingResponse {
				result,
				sent_feedback: None,
				reputation_changes: Vec::new(),
			}) {
				error!(target: LOG_TARGET, "Failed to send response to {peer}: {e:?}");
			}
		}
	}

	fn handle_message(&self, peer: &PeerId, payload: &[u8]) -> Result<Vec<u8>, SamplingError> {
		debug!(target: LOG_TARGET, "Handling da-sampling request from {peer}");

		let req = SamplingRequest::decode(payload)?;
		let block_hash: B::Hash = Hash::from_slice(&req.block_hash).into();

		let cache_key = SamplingCacheKey::new(block_hash, &req.cells);

		// LRU cache access must be locked
		if let Some(cached) = self.cache.lock().unwrap().get(&cache_key).cloned() {
			debug!(target: LOG_TARGET, "Cache hit for block {:?} cells {:?}", block_hash, req.cells);
			return Ok(cached);
		}

		let block = self.client.block(block_hash)?.ok_or_else(|| {
			SamplingError::RequestFailure(format!("Block not found: {:?}", block_hash))
		})?;

		let (header, extrinsics) = block.block.deconstruct();
		let number: u32 = (*header.number()).try_into().map_err(|_| {
			SamplingError::RequestFailure(format!("Invalid block number for {block_hash:?}"))
		})?;

		let block_len = self
			.client
			.runtime_api()
			.block_length(block_hash)
			.map_err(|_| {
				SamplingError::RequestFailure(format!(
					"Failed to get BlockLength for {block_hash:?}"
				))
			})?;

		let cells: Vec<_> = req.cells.iter().map(|c| (c.row, c.col)).collect();
		let start_time = Instant::now();
		let proofs = self
			.client
			.runtime_api()
			.proof(block_hash, number, extrinsics, block_len, cells)
			.map_err(SamplingError::Api)?
			.map_err(|e| SamplingError::RequestFailure(format!("Proof error: {e}")))?;
		debug!(target: LOG_TARGET, "Proof generation took: {:?}", start_time.elapsed());
		let cell_proofs = proofs
			.into_iter()
			.map(|(scalar, proof)| {
				let mut data_bytes = [0u8; 32];
				scalar.to_big_endian(&mut data_bytes);

				CellProof {
					data: data_bytes.to_vec(),
					proof: proof.to_vec(),
				}
			})
			.collect();

		let resp = SamplingResponse {
			proofs: cell_proofs,
		};
		let mut encoded = Vec::with_capacity(resp.encoded_len());
		resp.encode(&mut encoded)?;

		self.cache.lock().unwrap().put(cache_key, encoded.clone());

		Ok(encoded)
	}
}

/// DA samples downloader (client side)
#[derive(Clone)]
pub struct DaSamplesDownloader<B: BlockT, Client> {
	client: Arc<Client>,
	protocol_name: ProtocolName,
	network: Arc<NetworkService<B, B::Hash>>,
	pub verification_tracker: Arc<VerificationTracker>,
}

impl<B, Client> DaSamplesDownloader<B, Client>
where
	B: BlockT<Header = DaHeader, Hash = Hash>,
	Client: Send + Sync + 'static + ProvideRuntimeApi<B> + HeaderBackend<B>,
	Client::Api: DataAvailApi<B> + KateApi<B>,
{
	pub fn new(
		protocol_name: ProtocolName,
		client: Arc<Client>,
		network: Arc<NetworkService<B, B::Hash>>,
		tracker: Arc<VerificationTracker>,
	) -> Self {
		Self {
			protocol_name,
			client,
			network,
			verification_tracker: tracker,
		}
	}

	pub async fn run(self, import_stream: BoxStream<'static, BlockImportNotification<B>>) {
		debug!(target: LOG_TARGET, "Starting DA sample downloader");

		let tracker = self.verification_tracker.clone();
		let network = self.network.clone();
		let protocol_name = self.protocol_name.clone();

		let client = self.client.clone();

		tokio::spawn(async move {
			use std::time::Duration;
			use tokio::time::interval;

			let mut retry_interval = interval(Duration::from_secs(10));

			loop {
				retry_interval.tick().await;

				let finalized = client.info().finalized_number;

				let blocks = tracker.blocks.read().clone();
				for (hash, state) in blocks {
					let eligible = match state.status {
						BlockVerificationStatus::Verified | BlockVerificationStatus::Failed => {
							false
						},
						_ => true,
					};

					if !eligible {
						continue;
					}
					trace!(target: LOG_TARGET, "Checking block {hash:?} for retry with status {:?}", state.status);
					if let Ok(Some(header)) = client.header(hash) {
						let number = *header.number();
						if number <= finalized {
							continue;
						}

						if header.extension.app_lookup().is_empty() {
							trace!(target: LOG_TARGET, "Block does not have any DA txs, marking as verified {:?}", hash);
							tracker.set_status(hash, BlockVerificationStatus::Verified);
							continue;
						}

						debug!(
							target: LOG_TARGET,
							"Retrying DA verification for block {hash} (attempt {})",
							state.retry_count + 1
						);

						let header = header.clone();
						let tracker = tracker.clone();
						let downloader = DaSamplesDownloader {
							protocol_name: protocol_name.clone(),
							network: network.clone(),
							verification_tracker: tracker.clone(),
							client: client.clone(),
						};

						tokio::spawn(async move {
							if let Err(e) = downloader.verify_block(hash, header).await {
								error!(target: LOG_TARGET, "Retry DA verification failed for {hash}: {e}");
								tracker.set_status(hash, BlockVerificationStatus::TimedOut);
							}
						});
					}
				}
			}
		});
		tokio::pin!(import_stream);

		while let Some(notification) = import_stream.next().await {
			if self.verification_tracker.should_shutdown() {
				break;
			}

			let block_hash = notification.hash;

			let header: DaHeader = notification.header;
			if header.extension.app_lookup().is_empty() {
				trace!(target: LOG_TARGET, "Block does not have any DA txs, marking as verified {:?}", block_hash);
				self.verification_tracker
					.set_status(block_hash, BlockVerificationStatus::Verified);
				continue;
			}
			let is_own = matches!(notification.origin, BlockOrigin::Own);
			if is_own {
				trace!(target: LOG_TARGET, "Skipping DA verification of own block {:?}", block_hash);
				self.verification_tracker
					.set_status(block_hash, BlockVerificationStatus::Verified);
				continue;
			}
			sleep(Duration::from_secs(5)).await;
			trace!(target: LOG_TARGET, "Processing DA samples for block {:?}", block_hash);

			match self.verify_block(block_hash, header).await {
				Ok(_) => {
					debug!(target: LOG_TARGET, "Verified DA samples for block {:?}", block_hash)
				},
				Err(e) => {
					error!(target: LOG_TARGET, "Failed to verify DA samples for block {:?}: {}", block_hash, e)
				},
			}
		}
	}

	async fn verify_block(&self, block_hash: Hash, header: DaHeader) -> Result<(), SamplingError> {
		self.verification_tracker
			.set_status(block_hash, BlockVerificationStatus::Pending);

		let dimensions = Dimensions::new(header.extension.rows(), header.extension.cols())
			.ok_or_else(|| {
				error!(target: LOG_TARGET, "Invalid dimensions");
				SamplingError::RequestFailure(format!("Invalid dimensions for {block_hash:?}"))
			})?;

		let cells = if let Some(cells) = self.verification_tracker.get_cells(&block_hash) {
			cells
		} else {
			let cells = generate_random_cells(dimensions, CELL_COUNT);
			self.verification_tracker
				.set_cells(block_hash, cells.clone());
			cells
		};
		trace!(target: LOG_TARGET, "Using random cells for block {:?}: {:?}", block_hash, cells);

		let commitments = match header.extension {
			HeaderExtension::V3(ext) => ext.commitment.commitment,
			HeaderExtension::V4(ext) => ext.commitment.commitment,
		};

		let original_commitments: Vec<ArkCommitment> = commitments
			.chunks_exact(48)
			.map(|chunk| {
				ArkCommitment::from_bytes(chunk.try_into().expect("48 bytes")).map_err(|e| {
					error!(target: LOG_TARGET, "Invalid commitment: {e}");
					SamplingError::RequestFailure(format!("Invalid commitment: {e}"))
				})
			})
			.collect::<Result<_, _>>()?;

		let extended_commitments = ArkCommitment::extend_commitments(
			&original_commitments,
			original_commitments.len() * 2,
		)
		.map_err(|e| {
			error!(target: LOG_TARGET, "Commitment extension failed: {e}");
			SamplingError::RequestFailure(format!("Commitment extension failed: {e}"))
		})?;

		let commitments: Vec<_> = extended_commitments
			.into_iter()
			.map(|c| {
				c.to_bytes().map_err(|e| {
					error!(target: LOG_TARGET, "Invalid commitment bytes: {e}");
					SamplingError::RequestFailure(format!("Invalid commitment bytes: {e}"))
				})
			})
			.collect::<Result<_, _>>()?;

		let peers = self
			.network
			.reserved_peers()
			.await
			.map_err(|_| SamplingError::NoPeersAvailable)?;

		if peers.is_empty() {
			error!(target: LOG_TARGET, "No supernode peers available to provide DA sampling cell proof for block {:?}", block_hash);
			return Err(SamplingError::NoPeersAvailable);
		}

		let request = SamplingRequest {
			cells: cells.clone(),
			block_hash: block_hash.as_ref().to_vec(),
		};

		let mut retry_count = 0;
		let mut verification_success = false;

		let base_backoff_secs = std::cmp::max((commitments.len() / 512) as u64, 1);
		let cliff = 5;
		let max_backoff_secs = 1800; // 30 minutes

		self.verification_tracker
			.set_status(block_hash, BlockVerificationStatus::InProgress);

		while !verification_success {
			if self.verification_tracker.should_shutdown() {
				break;
			}

			debug!(target: LOG_TARGET, "Attempt {} for block {:?}", retry_count + 1, block_hash);

			let futures = peers.iter().map(|peer| {
				self.request_and_verify_cells(
					*peer,
					request.clone(),
					commitments.clone(),
					dimensions,
				)
			});

			let results = futures::future::join_all(futures).await;

			let mut temp_failed_cells = Vec::new();

			for result in results {
				match result {
					Ok(failed) => {
						if failed.is_empty() {
							verification_success = true;
						} else {
							temp_failed_cells.extend(failed);
						}
					},
					Err(e) => {
						warn!(target: LOG_TARGET, "Peer verification failed: {e}");
					},
				}
			}

			if !verification_success {
				self.verification_tracker
					.record_failed_cells(block_hash, temp_failed_cells.clone());
				self.verification_tracker.increment_retry(block_hash);
				retry_count += 1;

				let delay_secs = if retry_count <= cliff {
					10
				} else {
					let cliff_backoff = base_backoff_secs * cliff;
					let exp_delay =
						cliff_backoff.saturating_mul(2_u64.pow((retry_count - cliff) as u32));
					std::cmp::min(exp_delay, max_backoff_secs)
				};
				debug!(target: LOG_TARGET, "Retrying in {} seconds for block {:?}", delay_secs, block_hash);
				sleep(Duration::from_secs(delay_secs)).await;
			}
		}

		if self.verification_tracker.has_failed_cells(&block_hash) {
			warn!(target: LOG_TARGET, "Block {:?} has failed cells after retries", block_hash);
			self.verification_tracker
				.set_status(block_hash, BlockVerificationStatus::Failed);
			Err(SamplingError::VerificationFailed)
		} else if verification_success {
			self.verification_tracker
				.set_status(block_hash, BlockVerificationStatus::Verified);
			Ok(())
		} else {
			// Unexpected fallback — should retry later
			self.verification_tracker
				.set_status(block_hash, BlockVerificationStatus::Pending);
			Ok(())
		}
	}
	async fn request_and_verify_cells(
		&self,
		peer: PeerId,
		request: SamplingRequest,
		commitments: Vec<[u8; 48]>,
		dimensions: Dimensions,
	) -> Result<Vec<CellCoordinate>, SamplingError> {
		let mut buf = Vec::with_capacity(request.encoded_len());
		request
			.encode(&mut buf)
			.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

		let (tx, rx) = oneshot::channel();
		self.network.start_request(
			peer,
			self.protocol_name.clone(),
			buf,
			None,
			tx,
			IfDisconnected::ImmediateError,
		);

		debug!(target: LOG_TARGET, "Sent DA samples request to {peer} for block {:?}", request.block_hash);

		let response = match timeout(PEER_RESPONSE_TIMEOUT, rx).await {
			Ok(Ok(Ok(response))) => response,
			Ok(Ok(Err(e))) => {
				warn!(target: LOG_TARGET, "Request to {peer} failed: {e}");
				return Err(SamplingError::RequestFailure(format!(
					"Request to {peer} failed: {e}"
				)));
			},
			Ok(Err(_)) => {
				warn!(target: LOG_TARGET, "Channel closed for peer {peer}");
				return Err(SamplingError::RequestFailure("Channel closed".into()));
			},
			Err(_) => {
				warn!(target: LOG_TARGET, "Request to {peer} timed out");
				self.verification_tracker.set_status(
					Hash::from_slice(&request.block_hash).into(),
					BlockVerificationStatus::TimedOut,
				);
				return Err(SamplingError::Timeout);
			},
		};

		let decoded = SamplingResponse::decode(&*response.0)?;
		trace!(target: LOG_TARGET, "Received response from {peer}: for {:?}", request.block_hash);

		if decoded.proofs.len() != request.cells.len() {
			trace!(target: LOG_TARGET, "Proof count mismatch for block {:?}: expected {}, got {}", request.block_hash, request.cells.len(), decoded.proofs.len());
			return Err(SamplingError::RequestFailure(format!(
				"Proof count mismatch: expected {}, got {}",
				request.cells.len(),
				decoded.proofs.len()
			)));
		}

		let cells = request
			.cells
			.iter()
			.zip(decoded.proofs.iter())
			.map(|(coord, proof)| {
				let mut data_proof = [0u8; 80];
				data_proof[..48].copy_from_slice(&proof.proof);
				data_proof[48..].copy_from_slice(&proof.data);

				SingleCell::new(
					Position {
						row: coord.row,
						col: coord.col as u16,
					},
					data_proof,
				)
			})
			.collect::<Vec<_>>();

		let pp = PP.get_or_init(multiproof_params);
		let mut failed_cells = Vec::new();

		for (cell, coord) in cells.into_iter().zip(request.cells.iter()) {
			if let Err(e) = verify_v2(
				pp,
				dimensions,
				&commitments[cell.position.row as usize],
				&cell,
			) {
				warn!(
					target: LOG_TARGET,
					"Cell proof verification failed for {:?}: {e}", coord
				);
				failed_cells.push(coord.clone());
			}
		}

		if !failed_cells.is_empty() {
			Ok(failed_cells)
		} else {
			Ok(Vec::new())
		}
	}
}

fn get_protocol_name<Hash: AsRef<[u8]>>(
	genesis_hash: Hash,
	chain_spec: &Box<dyn ChainSpec>,
) -> ProtocolName {
	let genesis_hash = genesis_hash.as_ref();
	let chain_prefix = match chain_spec.fork_id() {
		Some(fork_id) => format!("/{}/{}", array_bytes::bytes2hex("", genesis_hash), fork_id),
		None => format!("/{}", array_bytes::bytes2hex("", genesis_hash)),
	};
	format!("{}{}", chain_prefix, NAME).into()
}

fn generate_random_cells(dimensions: Dimensions, cell_count: u32) -> Vec<CellCoordinate> {
	let (max_cells, row_limit) = (dimensions.extended_size(), dimensions.extended_rows());
	let count = max_cells.min(cell_count);

	if max_cells < cell_count {
		debug!("Max cells {max_cells} < requested {cell_count}");
	}

	let mut rng = thread_rng();
	let mut indices = HashSet::with_capacity(count as usize);

	while indices.len() < count as usize {
		indices.insert(CellCoordinate {
			row: rng.gen_range(0..row_limit),
			col: rng.gen_range(0..dimensions.cols().into()) as u32,
		});
	}

	indices.into_iter().collect()
}
