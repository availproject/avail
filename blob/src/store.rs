use anyhow::Result;
use log::info;

use crate::types::{BlobHash, Shard};

pub trait ShardStore: Send + Sync + 'static {
	fn insert_blob(&self, hash: &BlobHash, blob: &[u8]) -> Result<()>;
	fn insert_shard(&self, shard: &Shard) -> Result<()>;
	fn get_shard(&self, hash: &BlobHash, shard_id: u16) -> Option<Vec<u8>>;
	fn list_shards(&self, hash: &BlobHash) -> Result<Vec<Shard>>;
}

#[derive(Clone, Default)]
pub struct MockShardStore;

impl ShardStore for MockShardStore {
	// For the rpc to temporarily keep the blob
	fn insert_blob(&self, hash: &BlobHash, _blob: &[u8]) -> Result<()> {
		info!("MockShardStore::insert_blob hash={}", hash);
		Ok(())
	}

	// For the validator for long storage (N days)
	fn insert_shard(&self, shard: &Shard) -> Result<()> {
		info!("MockShardStore::insert_shard hash={}", shard.hash,);
		Ok(())
	}

	fn get_shard(&self, hash: &BlobHash, shard_id: u16) -> Option<Vec<u8>> {
		info!(
			"MockShardStore::get_shard hash={}, shard_id={}  → None",
			hash, shard_id
		);
		None
	}

	fn list_shards(&self, hash: &BlobHash) -> Result<Vec<Shard>> {
		info!("MockShardStore::list_shards hash={}  → []", hash);
		Ok(Vec::new())
	}
}
