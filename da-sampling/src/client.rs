use std::{str::FromStr, sync::Arc};

use crate::{
	response_to_samplingproofs, types::SamplingError, DaSamplingRequest, DaSamplingResponse,
	LOG_TARGET,
};
use avail_blob::p2p::BlobHandle;
use avail_core::{
	header::{
		extension::{fri::FriHeader, fri_v1::FriBlobCommitment},
		HeaderExtension,
	},
	traits::extended_header::ExtendedHeader,
};
use da_runtime::Header as DaHeader;
use futures::channel::oneshot;
use log::{info, warn};
use prost::Message;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use sc_network::{IfDisconnected, NetworkRequest, PeerId, ProtocolName};
use sp_core::H256;
use sp_runtime::traits::Block as BlockT;

use avail_fri::{
	core::{FriBiniusPCS, B128},
	encoding::mle_dims_from_blob_size,
	FriCommitment, FriParamsVersion,
};

pub struct DaSamplingDownloader<B>
where
	B: BlockT<Header = DaHeader>,
{
	blob_handle: Arc<BlobHandle<B>>,
	protocol: ProtocolName,
}

impl<B> DaSamplingDownloader<B>
where
	B: BlockT<Header = DaHeader, Hash = H256>,
{
	pub fn new(blob_handle: Arc<BlobHandle<B>>, protocol: ProtocolName) -> Self {
		Self {
			blob_handle,
			protocol,
		}
	}
	pub async fn on_finalized(&self, header: B::Header) {
		info!("Finalised block :{:?}", header.hash());
		if header.extension.is_kzg() || !header.extension().has_da_commitments() {
			return;
		}

		let extension = match &header.extension {
			HeaderExtension::Fri(ext) => ext,
			_ => return,
		};

		let blobs = match extension {
			FriHeader::V1(ext) => &ext.blobs,
		};

		if blobs.is_empty() {
			return;
		}

		let mut rng = StdRng::from_entropy();
		let blob = &blobs[rng.gen_range(0..blobs.len())];

		// TODO: temporary, we should use blob owners from local state
		let peers = self.blob_handle.network.reserved_peers().await.unwrap();
		// let owners = match self
		// 	.blob_handle
		// 	.blob_database
		// 	.get_blob_ownerships(&blob.blob_hash)
		// {
		// 	Ok(o) if !o.is_empty() => o,
		// 	Ok(_) => {
		// 		log::error!(
		// 			target: LOG_TARGET,
		// 			"No owners found for blob {:?} in block {:?}, skipping sampling",
		// 			blob.blob_hash,
		// 			header.hash()
		// 		);
		// 		return;
		// 	},
		// 	Err(e) => {
		// 		log::error!(
		// 			target: LOG_TARGET,
		// 			"Failed to fetch ownership for blob {:?}: {e}",
		// 			blob.blob_hash
		// 		);
		// 		return;
		// 	},
		// };

		// for owner in owners {
		// 	let peer_id = match PeerId::from_str(&owner.encoded_peer_id) {
		// 		Ok(p) => p,
		// 		Err(_) => return,
		// 	};
		// 	let _ = self.request_and_verify(peer_id, header.hash(), blob).await;
		// }
		for peer in peers {
			let _ = self.request_and_verify(peer, header.hash(), blob).await;
		}
	}

	fn sample_cells(&self, max: u32) -> Vec<u32> {
		let mut rng = StdRng::from_entropy();
		let count = 16.min(max);
		(0..count).map(|_| rng.gen_range(0..max)).collect()
	}

	async fn request_and_verify(
		&self,
		peer: PeerId,
		block_hash: B::Hash,
		blob: &FriBlobCommitment,
	) -> Result<(), SamplingError> {
		let (log_len, n_vars) = mle_dims_from_blob_size(blob.size_bytes as usize);
		let cells = self.sample_cells(log_len as u32);

		let req = DaSamplingRequest {
			block_hash: block_hash.as_ref().to_vec(),
			blob_hash: blob.blob_hash.0.to_vec(),
			cell_indices: cells.clone(),
		};

		let mut buf = Vec::new();
		req.encode(&mut buf)?;

		let (tx, rx) = oneshot::channel();

		self.blob_handle.network.start_request(
			peer,
			self.protocol.clone(),
			buf,
			None,
			tx,
			IfDisconnected::TryConnect,
		);

		let resp = match rx.await {
			Ok(Ok(resp)) => resp,
			Ok(Err(e)) => {
				warn!(target: LOG_TARGET, "Request to {peer} failed: {e}");
				return Err(SamplingError::RequestFailure(e.to_string()));
			},
			Err(_) => {
				return Err(SamplingError::RequestFailure("channel closed".into()));
			},
		};

		let resp = DaSamplingResponse::decode(&*resp.0)?;

		let params_version = FriParamsVersion(0);
		let cfg = params_version.to_config(n_vars);
		let pcs = FriBiniusPCS::new(cfg);
		let ctx = pcs
			.initialize_fri_context::<B128>(log_len)
			.map_err(|_| SamplingError::VerificationFailed)?;

		let sampling_proofs = response_to_samplingproofs(resp.proofs)?;
		let digest: [u8; 32] = blob
			.commitment
			.as_slice()
			.try_into()
			.map_err(|_| SamplingError::VerificationFailed)?;

		let commitment = FriCommitment { digest };

		for proof in sampling_proofs {
			proof
				.verify_b128(&pcs, &ctx, &commitment)
				.map_err(|_| SamplingError::VerificationFailed)?;
		}

		info!(target: LOG_TARGET, "DA sampling verified for {:?}", block_hash);
		Ok(())
	}
}
