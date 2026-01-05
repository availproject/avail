use crate::{CellProof, DaSamplingRequest, DaSamplingResponse};
use avail_blob::p2p::BlobHandle;
use prost::Message;
use sc_network::request_responses::{IncomingRequest, OutgoingResponse};
use sp_core::H256;
use sp_runtime::traits::Block as BlockT;

use futures::StreamExt;
use log::{debug, error};
use std::sync::Arc;

use avail_fri::{
	core::{FriBiniusPCS, B128},
	encoding::BytesEncoder,
	transcript_to_bytes, FriParamsVersion,
};

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
		while let Some(req) = self.request_rx.next().await {
			let IncomingRequest {
				peer,
				payload,
				pending_response,
			} = req;

			let result = self.handle_request(&payload);

			let outgoing = match result {
				Ok(bytes) => OutgoingResponse {
					result: Ok(bytes),
					reputation_changes: Vec::new(),
					sent_feedback: None,
				},
				Err(e) => {
					error!("DA sampling request failed from {peer:?}: {e}");
					OutgoingResponse {
						result: Err(()),
						reputation_changes: Vec::new(),
						sent_feedback: None,
					}
				},
			};

			let _ = pending_response.send(outgoing);
		}
	}

	fn handle_request(&self, payload: &[u8]) -> Result<Vec<u8>, String> {
		let req = DaSamplingRequest::decode(payload).map_err(|e| format!("Decode failed: {e}"))?;

		debug!(
			"DA sampling request block={}, cells={}",
			hex::encode(&req.block_hash),
			req.cell_indices.len()
		);

		let blob = self
			.blob_handle
			.blob_database
			.get_blob(&H256::from_slice(req.blob_hash.as_slice()))
			.map_err(|e| e.to_string())?;

		if blob.is_none() {
			return Err(format!(
				"blob does not exist in blob_database: {:?}",
				req.blob_hash
			));
		}
		let encoder = BytesEncoder::<B128>::new();
		let packed = encoder
			.bytes_to_packed_mle(&blob.expect("checked above").data)
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

		let proofs = req
			.cell_indices
			.into_iter()
			.map(|idx| {
				let value = commit_output.codeword[idx as usize];
				let transcript = pcs
					.inclusion_proof::<B128>(&commit_output.committed, idx as usize)
					.map_err(|e| e.to_string())?;

				Ok(CellProof {
					index: idx,
					cell: value.val().to_le_bytes().to_vec(),
					proof: transcript_to_bytes(&transcript),
				})
			})
			.collect::<Result<Vec<_>, String>>()?;

		let resp = DaSamplingResponse { proofs };

		let mut out = Vec::with_capacity(resp.encoded_len());
		resp.encode(&mut out)
			.map_err(|e| format!("Encode failed: {e}"))?;

		Ok(out)
	}
}
