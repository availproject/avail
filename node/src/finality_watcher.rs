use avail_blob::store::StorageApiT;
use log::{debug, error, info, warn};
use sp_blockchain::HeaderBackend;
use sp_core::H256;
use sp_runtime::traits::{Block as BlockT, Header};
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

pub(crate) const LOG_TARGET: &str = "finality_promoter";

/// A finality promoter task future.
///
/// Returns a Future (not spawned). Callers should spawn it with the node's spawn handle:
pub fn finality_promoter<B, C>(
	client: Arc<C>,
	blob_store: Arc<dyn StorageApiT>,
	poll_interval: Duration,
) -> impl Future<Output = ()>
where
	B: BlockT<Hash = H256> + 'static,
	C: HeaderBackend<B> + Send + Sync + 'static,
{
	async move {
		// Initialize last_finalized from client at start
		let mut last_finalized: H256 = client.info().finalized_hash;

		info!(
			target: LOG_TARGET,
			"finality_promoter started (initial finalized: {:?})",
			last_finalized
		);

		let mut interval = time::interval(poll_interval);

		loop {
			interval.tick().await;

			let current_finalized = client.info().finalized_hash;
			if current_finalized == last_finalized {
				continue;
			}

			info!(
				target: LOG_TARGET,
				"Detected finalized head change: {:?} -> {:?}",
				last_finalized,
				current_finalized
			);

			// Build vector of block hashes that were newly finalized, from oldest -> newest.
			// Approach: walk parents from `current_finalized` back until we hit last_finalized (exclusive)
			// or until some reasonable depth (safety).
			let mut to_promote_rev: Vec<H256> = Vec::new();
			let mut cursor = current_finalized;
			let mut safety_counter: u32 = 0;
			const MAX_WALK: u32 = 2400; // Assuming that finality can be stuck for max of 4 hours (6s block), after which we have other problems to solve

			loop {
				if cursor == last_finalized {
					break;
				}

				match client.header(cursor) {
					Ok(Some(header)) => {
						let parent = *header.parent_hash();
						to_promote_rev.push(cursor);
						// If parent equals last_finalized, we've reached child-of-last_finalized.
						if parent == last_finalized {
							break;
						}
						cursor = parent;
					},
					Ok(None) => {
						warn!(
							target: LOG_TARGET,
							"header not found while walking finalized chain for hash {:?}; stopping walk",
							cursor
						);
						break;
					},
					Err(e) => {
						error!(
							target: LOG_TARGET,
							"error fetching header for {:?}: {:?}; stopping walk",
							cursor, e
						);
						break;
					},
				}

				safety_counter += 1;
				if safety_counter > MAX_WALK {
					error!(
						target: LOG_TARGET,
						"finality promoter reached MAX_WALK={} while walking from {:?} to {:?}; stopping walk",
						MAX_WALK, current_finalized, last_finalized
					);
					break;
				}
			}

			// Reverse to get oldest -> newest
			to_promote_rev.reverse();
			if to_promote_rev.is_empty() {
				// Fallback: include current_finalized as best-effort
				to_promote_rev.push(current_finalized);
			}

			// Promote each block's pending blob infos
			for block_hash in to_promote_rev.iter() {
				debug!(
					target: LOG_TARGET,
					"Promoting pending blob infos for finalized block {:?}",
					block_hash
				);

				match blob_store.take_pending_blob_infos(block_hash) {
					Ok(infos) => {
						if infos.is_empty() {
							debug!(
								target: LOG_TARGET,
								"No pending blob infos for block {:?}",
								block_hash
							);
							continue;
						}

						// Insert canonical blob info entries (one by one or batch if you extend API)
						let mut promoted = 0usize;
						let mut failed = 0usize;
						let start_ns = std::time::Instant::now();
						for info in infos.into_iter() {
							let hash = info.hash;
							match blob_store.insert_blob_info(info) {
								Ok(()) => promoted += 1,
								Err(e) => {
									failed += 1;
									warn!(
										target: LOG_TARGET,
										"Failed to insert canonical blob_info for {} from block {:?}: {}",
										hash, block_hash, e
									);
								},
							}
						}
						let elapsed_ms = start_ns.elapsed().as_millis();
						info!(
							target: LOG_TARGET,
							"Promoted {} canonical blob_info(s) for {:?} (failed={}, time={} ms)",
							promoted,
							block_hash,
							failed,
							elapsed_ms
						);
					},
					Err(e) => {
						error!(
							target: LOG_TARGET,
							"Failed to take_pending_blob_infos for {:?}: {:?}",
							block_hash, e
						);
					},
				}
			}

			// Update last_finalized AFTER processing
			last_finalized = current_finalized;
		}
	}
}
