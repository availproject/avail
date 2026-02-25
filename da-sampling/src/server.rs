use crate::{CellProof, DaSamplingRequest, DaSamplingResponse};
use avail_blob::p2p::BlobHandle;
use prost::Message;
use sc_network::{
	request_responses::{IncomingRequest, OutgoingResponse},
	PeerId,
};
use sp_core::H256;
use sp_runtime::traits::Block as BlockT;

use futures::StreamExt;
use log::{debug, error, info, trace, warn};
use std::sync::Arc;

use avail_fri::{
	core::{FriBiniusPCS, B128},
	encoding::BytesEncoder,
	transcript_to_bytes, FriParamsVersion,
};

const LOG_TARGET: &str = "da-sampling::server";

pub struct DaSamplingRequestHandler<B: BlockT> {
	blob_handle: Arc<BlobHandle<B>>,
	request_rx: async_channel::Receiver<IncomingRequest>,
	_marker: std::marker::PhantomData<B>,
}

impl<B> DaSamplingRequestHandler<B>
where
	B: BlockT,
{
	pub fn new(
		blob_handle: Arc<BlobHandle<B>>,
		request_rx: async_channel::Receiver<IncomingRequest>,
	) -> Self {
		Self {
			blob_handle,
			request_rx,
			_marker: Default::default(),
		}
	}

	pub async fn run(mut self) {
		info!(
			target: LOG_TARGET,
			"🚀 DA sampling request handler started"
		);

		while let Some(req) = self.request_rx.next().await {
			let IncomingRequest {
				peer,
				payload,
				pending_response,
			} = req;

			debug!(
				target: LOG_TARGET,
				"📥 Incoming DA sampling request from peer {:?} ({} bytes)",
				peer,
				payload.len()
			);

			let result = self.handle_request(&peer, &payload);

			let outgoing = match result {
				Ok(bytes) => {
					info!(
						target: LOG_TARGET,
						"📤 Responding to peer {:?} with {} bytes",
						peer,
						bytes.len()
					);
					OutgoingResponse {
						result: Ok(bytes),
						reputation_changes: Vec::new(),
						sent_feedback: None,
					}
				},
				Err(e) => {
					error!(
						target: LOG_TARGET,
						"❌ DA sampling request FAILED from peer {:?}: {}",
						peer,
						e
					);
					OutgoingResponse {
						result: Err(()),
						reputation_changes: Vec::new(),
						sent_feedback: None,
					}
				},
			};

			if let Err(e) = pending_response.send(outgoing) {
				warn!(
					target: LOG_TARGET,
					"⚠️ Failed to send DA sampling response to peer {:?}: {:?}",
					peer,
					e
				);
			}
		}

		info!(
			target: LOG_TARGET,
			"🛑 DA sampling request handler stopped"
		);
	}

	fn handle_request(&self, peer: &PeerId, payload: &[u8]) -> Result<Vec<u8>, String> {
		let req = DaSamplingRequest::decode(payload).map_err(|e| format!("Decode failed: {e}"))?;

		debug!(
			target: LOG_TARGET,
			"📦 Decoded request from {:?}: block={}, blob={}, cells={}",
			peer,
			hex::encode(&req.block_hash),
			hex::encode(&req.blob_hash),
			req.cell_indices.len()
		);

		let blob_hash = H256::from_slice(req.blob_hash.as_slice());

		trace!(
			target: LOG_TARGET,
			"🗄️ Looking up blob {:?} in local blob database",
			blob_hash
		);

		let blob = self
			.blob_handle
			.blob_database
			.get_blob(&blob_hash)
			.map_err(|e| e.to_string())?;

		let blob =
			blob.ok_or_else(|| format!("blob not found in local database: {:?}", blob_hash))?;

		debug!(
			target: LOG_TARGET,
			"🗄️ Blob {:?} found locally ({} bytes)",
			blob_hash,
			blob.data.len()
		);

		let encoder = BytesEncoder::<B128>::new();
		let packed = encoder
			.bytes_to_packed_mle(&blob.data)
			.map_err(|e| e.to_string())?;

		let cfg = FriParamsVersion(0).to_config(packed.total_n_vars);
		let pcs = Arc::new(FriBiniusPCS::new(cfg));

		let ctx = pcs
			.initialize_fri_context::<B128>(packed.packed_mle.log_len())
			.map_err(|e| e.to_string())?;

		let commit_output = Arc::new(
			pcs.commit(&packed.packed_mle, &ctx)
				.map_err(|e| e.to_string())?,
		);

		let log_batch_size = ctx.fri_params.log_batch_size();
		let leaf_count = 1usize
			<< (ctx
				.fri_params
				.rs_code()
				.log_len()
				.saturating_sub(log_batch_size));

		for &idx in &req.cell_indices {
			if idx as usize >= leaf_count {
				return Err(format!(
					"invalid sampling request: index {} >= leaf_count {}",
					idx, leaf_count
				));
			}
		}

		debug!(
			target: LOG_TARGET,
			"🧪 Generating {} inclusion proofs for blob {:?}",
			req.cell_indices.len(),
			blob_hash
		);

		let proofs = req
			.cell_indices
			.into_iter()
			.map(|idx| {
				trace!(
					target: LOG_TARGET,
					"🧪 Generating inclusion proof for cell index {}",
					idx
				);

				let sampled_values = commit_output
					.codeword
					.to_ref()
					.chunk(log_batch_size, idx as usize)
					.iter_scalars()
					.collect::<Vec<_>>();

				let mut cell_bytes = Vec::with_capacity(sampled_values.len() * 16);
				for value in sampled_values {
					cell_bytes.extend_from_slice(&value.val().to_le_bytes());
				}

				let transcript = pcs
					.inclusion_proof::<B128>(&commit_output.committed, idx as usize)
					.map_err(|e| e.to_string())?;

				Ok(CellProof {
					index: idx,
					cell: cell_bytes,
					proof: transcript_to_bytes(&transcript),
				})
			})
			.collect::<Result<Vec<_>, String>>()?;

		let resp = DaSamplingResponse { proofs };

		let mut out = Vec::with_capacity(resp.encoded_len());
		resp.encode(&mut out)
			.map_err(|e| format!("Encode failed: {e}"))?;

		info!(
			target: LOG_TARGET,
			"✅ DA sampling response ready for peer {:?} ({} proofs)",
			peer,
			resp.proofs.len()
		);

		Ok(out)
	}
}
