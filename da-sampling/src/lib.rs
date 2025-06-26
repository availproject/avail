#![allow(dead_code)]
mod schema;
use futures::stream::BoxStream;
use futures::{channel::oneshot, stream::StreamExt};
use libp2p_identity::PeerId;
use log::{debug, error, trace};
use prost::Message;
use sc_chain_spec::ChainSpec;
use sc_client_api::BlockBackend;
use sc_client_api::BlockImportNotification;
use sc_network::NetworkRequest;
use sc_network::{
	request_responses::{IfDisconnected, IncomingRequest, OutgoingResponse, ProtocolConfig},
	types::ProtocolName,
	NetworkService,
};
use schema::v1::da_sampling::*;
use sp_runtime::traits::Block as BlockT;
use std::{io, sync::Arc, time::Duration};

const LOG_TARGET: &str = "da-sampling";

// Match Substrate protocol max
const MAX_PACKET_SIZE: u64 = 16 * 1024 * 1024;
const MAX_REQUEST_QUEUE: usize = 100;
const NAME: &'static str = "/da-sampling/1";

/// Return the protocol name for the DA sampling protocol based on the genesis hash and chain spec.
pub fn get_protocol_name<Hash: AsRef<[u8]>>(
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

/// DA sampling protocol error.
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
}

/// DA samples request handler (server side).
pub struct DaSamplesRequestHandler<B> {
	client: Arc<dyn BlockBackend<B> + Send + Sync>,
	request_receiver: async_channel::Receiver<IncomingRequest>,
}

impl<B: BlockT> DaSamplesRequestHandler<B> {
	pub fn new<Hash: AsRef<[u8]>>(
		client: Arc<dyn BlockBackend<B> + Send + Sync>,
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

		let handler = Self {
			client,
			request_receiver,
		};

		(handler, config)
	}

	pub async fn run(mut self) {
		while let Some(request) = self.request_receiver.next().await {
			let IncomingRequest {
				peer,
				payload,
				pending_response,
			} = request;

			let response = self.handle_message(&peer, &payload);
			match response {
				Ok(data) => {
					let _ = pending_response.send(OutgoingResponse {
						result: Ok(data),
						sent_feedback: None,
						reputation_changes: Vec::new(), // Update this later
					});
				},
				Err(e) => {
					error!(target: LOG_TARGET, "Failed to handle request from {peer}: {e}");
					let _ = pending_response.send(OutgoingResponse {
						result: Err(()),
						reputation_changes: Vec::new(),
						sent_feedback: None,
					});
				},
			}
		}
	}

	fn handle_message(
		&mut self,
		peer: &PeerId,
		payload: &Vec<u8>,
	) -> Result<Vec<u8>, SamplingError> {
		debug!(target: LOG_TARGET, "Handling da-sampling request from {peer}");

		// TODO: Replace with actual schema decoding and proof generation
		let _req = SamplingRequest::decode(&payload[..])?;

		let proof = vec![1, 2, 3, 4]; // placeholder
		let resp = SamplingResponse {
			proofs: vec![CellProof {
				cell: Some(CellCoordinate { row: 0, col: 0 }),
				data: proof.clone(),
				proof,
			}],
		};
		let mut encoded = Vec::with_capacity(resp.encoded_len());
		resp.encode(&mut encoded)?;
		Ok(encoded)
	}
}

/// DA samples downloader (client side).
pub struct DaSamplesDownloader<B: BlockT> {
	protocol_name: ProtocolName,
	import_stream: BoxStream<'static, BlockImportNotification<B>>,
	network: Arc<NetworkService<B, B::Hash>>,
}

impl<B: BlockT> DaSamplesDownloader<B> {
	pub fn new(
		protocol_name: ProtocolName,
		import_stream: BoxStream<'static, BlockImportNotification<B>>,
		network: Arc<NetworkService<B, B::Hash>>,
	) -> Self {
		Self {
			protocol_name,
			import_stream,
			network,
		}
	}

	pub async fn run(mut self) {
		while let Some(notification) = self.import_stream.next().await {
			trace!(target: LOG_TARGET, "Received block import notification for: {:?}", notification.hash);
			// for time being, lets use the reserved_peers as supernodes
			let peers = self.network.reserved_peers().await.unwrap();

			let request = SamplingRequest {
				cells: vec![CellCoordinate { row: 0, col: 0 }],
				block_hash: notification.hash.as_ref().to_vec(),
			};

			for peer in peers {
				let _ = self.request_cell_proof(peer.clone(), request.clone()).await;
			}
		}
	}

	pub async fn request_cell_proof(
		&mut self,
		peer: PeerId,
		request: SamplingRequest,
	) -> Result<(), SamplingError> {
		let mut buf = Vec::with_capacity(request.encoded_len());
		request
			.encode(&mut buf)
			.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

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
			.expect("Lets see");

		let decoded = SamplingResponse::decode(&*response.0)?;
		debug!(target: LOG_TARGET, "Received response from {peer}: {:?}", decoded);
		Ok(())
	}
}
