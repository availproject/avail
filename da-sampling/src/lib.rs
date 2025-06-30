#![allow(dead_code)]
mod schema;

use avail_core::{header::HeaderExtension, OpaqueExtrinsic};
use da_runtime::{
	apis::{DataAvailApi, KateApi},
	Header as DaHeader,
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
use sp_runtime::{
	testing::H256,
	traits::{Block as BlockT, Header, PhantomData},
};
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
const MAX_RETRIES: u32 = 3;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const PEER_RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);

static PP: std::sync::OnceLock<ArkPublicParams> = std::sync::OnceLock::new();

/// Block verification status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockVerificationStatus {
	/// Verification not started yet
	Pending,
	/// Verification in progress
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
}

/// Shared state for block's DA verification
#[derive(Debug, Clone)]
pub struct VerificationTracker {
	blocks: Arc<RwLock<HashMap<H256, BlockVerificationState>>>,
	shutdown: Arc<AtomicBool>,
}

impl VerificationTracker {
	pub fn new() -> Self {
		Self {
			blocks: Arc::new(RwLock::new(HashMap::new())),
			shutdown: Arc::new(AtomicBool::new(false)),
		}
	}

	pub fn set_status(&self, block_hash: H256, status: BlockVerificationStatus) {
		let mut blocks = self.blocks.write();
		blocks.entry(block_hash).and_modify(|state| {
			state.status = status;
			if status == BlockVerificationStatus::InProgress {
				state.last_attempt = Some(Instant::now());
			}
		});
	}

	pub fn get_status(&self, block_hash: &H256) -> Option<BlockVerificationStatus> {
		self.blocks.read().get(block_hash).map(|s| s.status)
	}

	pub fn record_failed_cells(&self, block_hash: H256, cells: Vec<CellCoordinate>) {
		let mut blocks = self.blocks.write();
		blocks.entry(block_hash).and_modify(|state| {
			state.failed_cells.extend(cells);
		});
	}

	pub fn increment_retry(&self, block_hash: H256) {
		let mut blocks = self.blocks.write();
		blocks.entry(block_hash).and_modify(|state| {
			state.retry_count += 1;
			state.last_attempt = Some(Instant::now());
		});
	}

	pub fn should_shutdown(&self) -> bool {
		self.shutdown.load(Ordering::SeqCst)
	}

	pub fn shutdown(&self) {
		self.shutdown.store(true, Ordering::SeqCst);
	}
}

/// DA samples request handler (server side)
pub struct DaSamplesRequestHandler<B, Client> {
	client: Arc<Client>,
	request_receiver: async_channel::Receiver<IncomingRequest>,
	block: PhantomData<B>,
}

impl<B, Client> DaSamplesRequestHandler<B, Client>
where
	B: BlockT<Header = DaHeader, Hash = H256, Extrinsic = OpaqueExtrinsic>,
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
			request_timeout: Duration::from_secs(30),
			inbound_queue: Some(tx),
		};

		(
			Self {
				client,
				request_receiver,
				block: PhantomData,
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

			let response = self.handle_message(&peer, &payload);

			let result = match response {
				Ok(data) => Ok(data),
				Err(e) => {
					error!(target: LOG_TARGET, "Failed to handle request from {peer}: {e}");
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
		let block_hash: B::Hash = H256::from_slice(&req.block_hash).into();

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
		let proofs = self
			.client
			.runtime_api()
			.proof(block_hash, number, extrinsics, block_len, cells)
			.map_err(SamplingError::Api)?
			.map_err(|e| SamplingError::RequestFailure(format!("Proof error: {e}")))?;

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
		Ok(encoded)
	}
}

/// DA samples downloader (client side)
pub struct DaSamplesDownloader<B: BlockT> {
	protocol_name: ProtocolName,
	network: Arc<NetworkService<B, B::Hash>>,
	verification_tracker: VerificationTracker,
}

impl<B> DaSamplesDownloader<B>
where
	B: BlockT<Header = DaHeader, Hash = H256>,
{
	pub fn new(protocol_name: ProtocolName, network: Arc<NetworkService<B, B::Hash>>) -> Self {
		Self {
			protocol_name,
			network,
			verification_tracker: VerificationTracker::new(),
		}
	}

	pub async fn run(self, import_stream: BoxStream<'static, BlockImportNotification<B>>) {
		debug!(target: LOG_TARGET, "Starting DA sample downloader");

		tokio::pin!(import_stream);

		while let Some(notification) = import_stream.next().await {
			if self.verification_tracker.should_shutdown() {
				break;
			}

			let block_hash = notification.hash;

			let header: DaHeader = notification.header;
			if header.extension.app_lookup().is_empty() {
				trace!(target: LOG_TARGET, "Block does not have any DA txs, skipping DA verification for {:?}", block_hash);
				continue;
			}
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

	async fn verify_block(&self, block_hash: H256, header: DaHeader) -> Result<(), SamplingError> {
		let dimensions = Dimensions::new(header.extension.rows(), header.extension.cols())
			.ok_or_else(|| {
				error!(target: LOG_TARGET, "Invalid dimensions");
				SamplingError::RequestFailure(format!("Invalid dimensions fro {block_hash:?}"))
			})?;

		let cells = generate_random_cells(dimensions, CELL_COUNT);

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
			return Err(SamplingError::NoPeersAvailable);
		}

		let request = SamplingRequest {
			cells,
			block_hash: block_hash.as_ref().to_vec(),
		};

		let mut retry_count = 0;
		let mut verification_success = false;
		let mut failed_cells = Vec::new();

		while retry_count < MAX_RETRIES && !verification_success {
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

			// Process results
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
				failed_cells = temp_failed_cells;
				self.verification_tracker
					.record_failed_cells(block_hash, failed_cells.clone());
				retry_count += 1;
				self.verification_tracker.increment_retry(block_hash);
				sleep(Duration::from_secs(1)).await;
			}
		}
		Ok(())
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

		debug!(target: LOG_TARGET, "Sent request to {peer}");

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
				return Err(SamplingError::Timeout);
			},
		};

		let decoded = SamplingResponse::decode(&*response.0)?;
		trace!(target: LOG_TARGET, "Received response from {peer}: {:?}", decoded);

		if decoded.proofs.len() != request.cells.len() {
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
