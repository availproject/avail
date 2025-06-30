#![allow(dead_code)]
mod schema;

use std::{collections::HashSet, io, sync::Arc, time::Duration};
use avail_core::{header::HeaderExtension, OpaqueExtrinsic};
use da_runtime::{apis::{DataAvailApi, KateApi}, Header as DaHeader};
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
use log::{debug, error, trace};
use prost::Message;
use rand::{thread_rng, Rng};
use sc_chain_spec::ChainSpec;
use sc_client_api::{BlockBackend, BlockImportNotification};
use sc_network::{
	NetworkRequest,
	NetworkService,
	request_responses::{IfDisconnected, IncomingRequest, OutgoingResponse, ProtocolConfig},
	types::ProtocolName,
};
use schema::v1::da_sampling::*;
use sp_api::ProvideRuntimeApi;
use sp_runtime::{
	testing::H256,
	traits::{Block as BlockT, Header, PhantomData},
};
use tokio::time::sleep;

const LOG_TARGET: &str = "da-sampling";
const MAX_PACKET_SIZE: u64 = 16 * 1024 * 1024; // Match Substrate protocol max
const MAX_REQUEST_QUEUE: usize = 100;
const NAME: &str = "/da-sampling/1";
const CELL_COUNT: u32 = 14; // Number of cells required for 99.99% confidence

static PP: std::sync::OnceLock<ArkPublicParams> = std::sync::OnceLock::new();

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
}

/// DA samples request handler (server side).
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

			let _ = pending_response.send(OutgoingResponse {
				result,
				sent_feedback: None,
				reputation_changes: Vec::new(),
			});
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

/// DA samples downloader (client side).
pub struct DaSamplesDownloader<B: BlockT> {
	protocol_name: ProtocolName,
	network: Arc<NetworkService<B, B::Hash>>,
}

impl<B> DaSamplesDownloader<B>
where
	B: BlockT<Header = DaHeader>,
{
	pub fn new(protocol_name: ProtocolName, network: Arc<NetworkService<B, B::Hash>>) -> Self {
		Self {
			protocol_name,
			network,
		}
	}

	pub async fn run(self, mut import_stream: BoxStream<'static, BlockImportNotification<B>>) {
		debug!(target: LOG_TARGET, "Starting DA sample downloader");

		while let Some(notification) = import_stream.next().await {
			trace!(target: LOG_TARGET, "Received block import notification for: {:?}", notification.hash);

			let header: DaHeader = notification.header;
			if header.extension.app_lookup().is_empty() {
				trace!(target: LOG_TARGET, "Empty app lookup, skipping sampling");
				continue;
			}

			let dimensions = Dimensions::new(header.extension.rows(), header.extension.cols())
				.expect("Valid dimensions");
			let cells = generate_random_cells(dimensions, CELL_COUNT);

			let commitments = match header.extension {
				HeaderExtension::V3(ext) => ext.commitment.commitment,
				HeaderExtension::V4(ext) => ext.commitment.commitment,
			};

			let original_commitments: Vec<ArkCommitment> = commitments
				.chunks_exact(48)
				.map(|chunk| ArkCommitment::from_bytes(chunk.try_into().expect("48 bytes")))
				.collect::<Result<_, _>>()
				.expect("Valid commitments");

			let extended_commitments = ArkCommitment::extend_commitments(
				&original_commitments,
				original_commitments.len() * 2,
			)
			.expect("Valid extension");

			let commitments: Vec<_> = extended_commitments
				.into_iter()
				.map(|c| c.to_bytes().expect("Valid commitment"))
				.collect();

			let peers = self.network.reserved_peers().await.unwrap();
			let request = SamplingRequest {
				cells,
				block_hash: notification.hash.as_ref().to_vec(),
			};

			// Temp delay to ensure the super has time to process the block
			sleep(Duration::from_secs(3)).await;

			// Process peers in parallel
			let futures = peers.into_iter().map(|peer| {
				self.request_cell_proofs(peer, request.clone(), commitments.clone(), dimensions)
			});

			futures::future::join_all(futures).await;
		}
	}

	async fn request_cell_proofs(
		&self,
		peer: PeerId,
		request: SamplingRequest,
		commitments: Vec<[u8; 48]>,
		dimensions: Dimensions,
	) -> Result<(), SamplingError> {
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
		let response = rx
			.await
			.map_err(|_| SamplingError::RequestFailure("Channel closed".into()))?
			.map_err(|_| SamplingError::RequestFailure("Request failed".into()))?;

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
		for cell in cells {
			if let Err(e) = verify_v2(
				pp,
				dimensions,
				&commitments[cell.position.row as usize],
				&cell,
			) {
				error!(target: LOG_TARGET, "Cell proof verification failed: {e}");
			}
		}

		Ok(())
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
