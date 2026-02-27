use std::collections::HashSet;
use std::{str::FromStr, sync::Arc};

use crate::{
	response_to_samplingproofs, types::SamplingError, DaSamplingRequest, DaSamplingResponse,
};
use avail_blob::p2p::BlobHandle;
use avail_core::{header::extension::fri_v1::FriBlobCommitment, traits::extended_header::ExtendedHeader};
use da_runtime::Header as DaHeader;
use futures::channel::oneshot;
use log::{debug, error, info, trace, warn};
use prost::Message;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use sc_network::{IfDisconnected, NetworkRequest, PeerId, ProtocolName};
use sp_core::H256;
use sp_runtime::traits::Block as BlockT;

use avail_fri::{
	core::{FriBiniusPCS, B128},
	encoding::mle_dims_from_blob_size,
	FriCommitment, FriParamsVersion,
};

const LOG_TARGET: &str = "da-sampling::client";

pub struct DaSamplingDownloader<B>
where
	B: BlockT<Header = DaHeader>,
{
	blob_handle: Arc<BlobHandle<B>>,
	protocol: ProtocolName,
	config: DaSamplingConfig,
}

#[derive(Debug, Clone, Copy)]
pub struct DaSamplingConfig {
	pub samples_per_blob: u32,
	pub with_ev_proof: bool,
	pub app_id: Option<u32>,
}

impl Default for DaSamplingConfig {
	fn default() -> Self {
		Self {
			samples_per_blob: 16,
			with_ev_proof: false,
			app_id: None,
		}
	}
}

impl<B> DaSamplingDownloader<B>
where
	B: BlockT<Header = DaHeader, Hash = H256>,
{
	pub fn new(
		blob_handle: Arc<BlobHandle<B>>,
		protocol: ProtocolName,
		config: DaSamplingConfig,
	) -> Self {
		info!(
			target: LOG_TARGET,
			"Initializing DA sampling downloader with protocol {:?}",
			protocol
		);
		if let Some(app_id) = config.app_id {
			info!(
				target: LOG_TARGET,
				"⚙️  DA sampling is enabled for app_id {} only",
				app_id
			);
		}

		if config.with_ev_proof {
			info!(
				target: LOG_TARGET,
				"⚙️  Evaluation proof verification is enabled",
			);
		}
		Self {
			blob_handle,
			protocol,
			config,
		}
	}

	pub async fn on_finalized(&self, header: B::Header) {
		debug!(
			target: LOG_TARGET,
			"🔍 Finalized block received: hash={:?}, number={}",
			header.hash(),
			header.number
		);

		if header.extension.is_kzg() {
			trace!(target: LOG_TARGET, "⏭️ Skipping DA sampling: KZG block");
			return;
		}

		if !header.extension().has_da_commitments() {
			trace!(target: LOG_TARGET, "⏭️ Skipping DA sampling: no DA commitments");
			return;
		}

		let block_hash = header.hash();
		let blobs = self.get_fri_blobs_for_block(block_hash);

		if blobs.is_empty() {
			debug!(
				target: LOG_TARGET,
				"⏭️ Block {:?} has no FRI blobs recorded in sidecar",
				block_hash
			);
			return;
		}

		let blob_indices = if let Some(app_id) = self.config.app_id {
			let indices = self.get_app_id_blob_indices(block_hash, &blobs, app_id);
			if indices.is_empty() {
				debug!(
					target: LOG_TARGET,
					"⏭️ No blobs matched app_id={} in block {:?}",
					app_id,
					block_hash
				);
				return;
			}
			indices
		} else {
			(0..blobs.len()).collect()
		};

		// Randomly select ONE blob per block to sample from
		let mut rng = StdRng::from_entropy();
		let selected_index = rng.gen_range(0..blob_indices.len());
		let blob_index = blob_indices[selected_index];
		let blob = &blobs[blob_index];

		info!(
			target: LOG_TARGET,
			"🎯 Selected blob_hash={:?} for sampling in block: {:?}",
			blob.blob_hash,
			block_hash,
		);

		let peers = self.get_blob_owners_or_peers(blob.blob_hash).await;
		for (i, peer) in peers.iter().copied().enumerate() {
			info!(
				target: LOG_TARGET,
				"🔁 Sampling attempt {} for block {:?} via peer {:?}",
				i + 1,
				block_hash,
				peer
			);

			match self.request_and_verify(peer, block_hash, blob).await {
				Ok(_) => {
					info!(
						target: LOG_TARGET,
						"✅ DA sampling SUCCESS for block {:?} via peer {:?}",
						block_hash,
						peer
					);
					return;
				},
				Err(e) => {
					warn!(
						target: LOG_TARGET,
						"⚠️ DA sampling failed via peer {:?}: {:?}",
						peer,
						e
					);
				},
			}
		}

		error!(
			target: LOG_TARGET,
			"❌ DA sampling FAILED for block {:?}: all peers exhausted",
			block_hash
		);
	}

	// We use eval-sidecar data to filter blobs by app_id (if configured).
	fn get_app_id_blob_indices(
		&self,
		block_hash: H256,
		blobs: &[FriBlobCommitment],
		app_id: u32,
	) -> Vec<usize> {
		blobs
			.iter()
			.enumerate()
			.filter_map(|(idx, blob)| {
				self.blob_handle
					.get_eval_data_for_blob(block_hash, blob.blob_hash)
					.filter(|eval_data| eval_data.app_id.0 == app_id)
					.map(|_| idx)
			})
			.collect()
	}

	fn get_fri_blobs_for_block(&self, block_hash: H256) -> Vec<FriBlobCommitment> {
		let infos = match self
			.blob_handle
			.blob_database
			.list_blob_infos_by_block(&block_hash)
		{
			Ok(infos) => infos,
			Err(e) => {
				error!(
					target: LOG_TARGET,
					"❌ Failed to list BlobInfo entries for block {:?}: {e}",
					block_hash
				);
				return Vec::new();
			},
		};

		if infos.is_empty() {
			return Vec::new();
		}

		let mut blobs = Vec::with_capacity(infos.len());

		for info in infos {
			let meta = match self
				.blob_handle
				.blob_database
				.get_blob_metadata(&info.hash)
			{
				Ok(Some(meta)) => meta,
				Ok(None) => {
					warn!(
						target: LOG_TARGET,
						"⚠️ Missing BlobMetadata for blob {:?} in block {:?}, skipping",
						info.hash,
						block_hash
					);
					continue;
				},
				Err(e) => {
					error!(
						target: LOG_TARGET,
						"❌ Failed to fetch BlobMetadata for blob {:?}: {e}",
						info.hash
					);
					continue;
				},
			};

			blobs.push(FriBlobCommitment {
				blob_hash: info.hash,
				size_bytes: meta.size,
				commitment: meta.commitment.clone(),
			});
		}

		blobs
	}

	// Returns peer IDs to try for DA sampling:
	/// 1. Blob owners from local DB (preferred)
	/// 2. Reserved peers as fallback
	async fn get_blob_owners_or_peers(&self, blob_hash: H256) -> Vec<PeerId> {
		// Try getting blob_owners first
		match self
			.blob_handle
			.blob_database
			.get_blob_ownerships(&blob_hash)
		{
			Ok(owners) if !owners.is_empty() => {
				let mut peer_ids = Vec::with_capacity(owners.len());

				for owner in owners {
					match PeerId::from_str(&owner.encoded_peer_id) {
						Ok(peer_id) => peer_ids.push(peer_id),
						Err(e) => warn!(
							target: LOG_TARGET,
							"⚠️ Invalid peer_id '{}' for blob {:?}: {e}",
							owner.encoded_peer_id,
							blob_hash
						),
					}
				}

				if !peer_ids.is_empty() {
					debug!(
						target: LOG_TARGET,
						"📦 Using {} blob owners for blob {:?}",
						peer_ids.len(),
						blob_hash
					);
					return peer_ids;
				}

				warn!(
					target: LOG_TARGET,
					"⚠️ Blob {:?} has owners but none had valid PeerIds, falling back",
					blob_hash
				);
			},

			Ok(_) => {
				warn!(
					target: LOG_TARGET,
					"⚠️ No owners recorded for blob {:?}, falling back to reserved peers",
					blob_hash
				);
			},

			Err(e) => {
				error!(
					target: LOG_TARGET,
					"❌ Failed to fetch ownership for blob {:?}: {e}",
					blob_hash
				);
			},
		}

		// Fallback to reserved peers
		match self.blob_handle.network.reserved_peers().await {
			Ok(peers) if !peers.is_empty() => {
				debug!(
					target: LOG_TARGET,
					"🔁 Using {} reserved peers as fallback for blob {:?}",
					peers.len(),
					blob_hash
				);
				peers
			},
			_ => {
				error!(
					target: LOG_TARGET,
					"❌ No peers available for DA sampling for blob {:?}",
					blob_hash
				);
				Vec::new()
			},
		}
	}

	fn sample_cells(&self, max: u32) -> Vec<u32> {
		let mut rng = StdRng::from_entropy();
		let target = (self.config.samples_per_blob as usize).min(max as usize);

		let mut indices = HashSet::with_capacity(target);

		while indices.len() < target {
			indices.insert(rng.gen_range(0..max));
		}

		indices.into_iter().collect()
	}

	async fn request_and_verify(
		&self,
		peer: PeerId,
		block_hash: B::Hash,
		blob: &FriBlobCommitment,
	) -> Result<(), SamplingError> {
		debug!(
			target: LOG_TARGET,
			"Preparing DA sampling request: block={:?}, blob={:?}",
			block_hash,
			blob.blob_hash
		);

		let (log_len, n_vars) = mle_dims_from_blob_size(blob.size_bytes as usize);

		trace!(
			target: LOG_TARGET,
			"Derived FRI dimensions: log_len={}, n_vars={}",
			log_len,
			n_vars
		);

		let cells = self.sample_cells(log_len as u32);

		let req = DaSamplingRequest {
			block_hash: block_hash.as_ref().to_vec(),
			blob_hash: blob.blob_hash.0.to_vec(),
			cell_indices: cells.clone(),
		};

		let mut buf = Vec::with_capacity(req.encoded_len());
		req.encode(&mut buf)?;

		let (tx, rx) = oneshot::channel();

		trace!(
			target: LOG_TARGET,
			"📤 Sending DA sampling request to peer {:?} ({} cells)",
			peer,
			cells.len()
		);

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
				warn!(
					target: LOG_TARGET,
					"⚠️ Peer {:?} returned error response: {e}",
					peer
				);
				return Err(SamplingError::RequestFailure {
					reason: e.to_string(),
				});
			},
			Err(_) => {
				warn!(
					target: LOG_TARGET,
					"⚠️ DA sampling channel closed by peer {:?}",
					peer
				);
				return Err(SamplingError::RequestFailure {
					reason: "channel closed".into(),
				});
			},
		};

		let resp = DaSamplingResponse::decode(&*resp.0)?;
		// do sanity check of responded cells
		let requested: HashSet<u32> = cells.iter().copied().collect();
		let returned: HashSet<u32> = resp.proofs.iter().map(|p| p.index).collect();

		if requested != returned {
			error!(
				target: LOG_TARGET,
				"❌ Sampling response mismatch from peer {peer}: requested={:?}, returned={:?}",
				requested,
				returned
			);
			return Err(SamplingError::RequestFailure {
				reason: "server returned mismatched sampling indices".into(),
			});
		}

		debug!(
			target: LOG_TARGET,
			"📤 Received {} sampling proofs from peer {:?}",
			resp.proofs.len(),
			peer
		);

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
			trace!(
				target: LOG_TARGET,
				"🧪 Verifying inclusion proof at index {}",
				proof.index
			);

			proof
				.verify_b128(&pcs, &ctx, &commitment)
				.map_err(|_| SamplingError::VerificationFailed)?;
		}

		if self.config.with_ev_proof {
			if let Some(eval_data) = self
				.blob_handle
				.get_eval_data_for_blob(block_hash, blob.blob_hash)
			{
				let app_match = self
					.config
					.app_id
					.map_or(true, |id| eval_data.app_id.0 == id);
				if !app_match {
					debug!(
						target: LOG_TARGET,
						"Skipping eval-proof verification for blob {:?}: app_id mismatch (configured {:?}, received {})",
						blob.blob_hash,
						self.config.app_id,
						eval_data.app_id.0
					);
				} else {
					match avail_blob::validation::validate_fri_proof(
						blob.size_bytes as usize,
						&eval_data.eval_point_seed,
						&eval_data.eval_claim,
						&eval_data.eval_proof,
					) {
						Ok(()) => {
							info!(
								target: LOG_TARGET,
								"✅ Proof verification PASSED for blob {:?} (eval data from topic)",
								blob.blob_hash
							);
						},
						Err(e) => {
							warn!(
								target: LOG_TARGET,
								"❌ Proof verification failed for blob {:?}: {e}",
								blob.blob_hash
							);
						},
					}
				}
			} else {
				warn!(
					target: LOG_TARGET,
					"⚠️ No eval data available for blob {:?}, skipping FRI proof verification",
					blob.blob_hash
				);
			}
		}

		info!(
			target: LOG_TARGET,
			"✅ DA sampling verification PASSED for blob {:?} via peer {:?}",
			blob.blob_hash,
			peer
		);

		Ok(())
	}
}
