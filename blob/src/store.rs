use anyhow::{anyhow, Result};
use codec::{Decode, Encode};
use kvdb::DBTransaction;
use kvdb_rocksdb::Database;
use std::path::Path;
use tempfile::TempDir;

use crate::{
	types::{BlobHash, BlobMetadata, Shard},
	LOG_TARGET,
};

pub trait ShardStore: Send + Sync + 'static {
	// Blob metadata
	fn insert_blob_metadata(&self, hash: &BlobHash, blob_metadata: &BlobMetadata) -> Result<()>;
	fn get_blob_metadata(&self, hash: &BlobHash) -> Result<Option<BlobMetadata>>;
	fn remove_blob_metadata(&self, hash: &BlobHash) -> Result<()>;

	// Blob read error retry count
	fn insert_blob_retry(&self, hash: &BlobHash, count: u16) -> Result<()>;
	fn get_blob_retry(&self, hash: &BlobHash) -> Result<u16>;
	fn remove_blob_retry(&self, hash: &BlobHash) -> Result<()>;

	// Shards
	fn insert_shards(&self, shards: &Vec<Shard>) -> Result<()>;
	fn get_shard(&self, hash: &BlobHash, shard_id: u16) -> Result<Option<Shard>>;
	fn remove_shards(&self, shards: &Vec<(BlobHash, u16)>) -> Result<()>;

	// Testing stuff
	fn log_all_entries(&self) -> Result<()>;
}

pub struct RocksdbShardStore {
	db: Database,
}

impl RocksdbShardStore {
	/// Open (or create) a new DB at `path`, with a single column.
	pub fn open(path: impl AsRef<Path>) -> Result<Self> {
		let db = Database::open(&Default::default(), path.as_ref())?;
		Ok(RocksdbShardStore { db })
	}

	/// blob key = b"blob:" || hash_bytes
	fn blob_key(hash: &BlobHash) -> Vec<u8> {
		let mut k = b"blob:".to_vec();
		k.extend_from_slice(hash.as_bytes());
		k
	}

	/// blob key = b"blob:" || hash_bytes
	fn blob_count_key(hash: &BlobHash) -> Vec<u8> {
		let mut k = b"count:".to_vec();
		k.extend_from_slice(hash.as_bytes());
		k
	}

	/// shard key = b"shard:" || hash_bytes || shard_id_be
	fn shard_key(hash: &BlobHash, shard_id: u16) -> Vec<u8> {
		let mut k = b"shard:".to_vec();
		k.extend_from_slice(hash.as_bytes());
		k.extend_from_slice(&shard_id.to_be_bytes());
		k
	}
}

impl Default for RocksdbShardStore {
	fn default() -> Self {
		let temp_dir = TempDir::new().expect("failed to create temp dir for RocksdbShardStore");
		let db_path = temp_dir.path().join("blob_store");
		let db = Database::open(&Default::default(), db_path)
			.expect("opening RocksDB blob store failed");
		RocksdbShardStore { db }
	}
}

impl ShardStore for RocksdbShardStore {
	fn insert_blob_metadata(&self, hash: &BlobHash, blob_metadata: &BlobMetadata) -> Result<()> {
		let mut tx = DBTransaction::new();
		tx.put(0, &Self::blob_key(hash), &blob_metadata.encode());
		self.db.write(tx)?;
		// let _ = self.log_all_entries();
		Ok(())
	}

	fn get_blob_metadata(&self, hash: &BlobHash) -> Result<Option<BlobMetadata>> {
		self.db
			.get(0, &Self::blob_key(hash))?
			.map(|bytes| {
				let mut slice = bytes.as_slice();
				BlobMetadata::decode(&mut slice)
					.map_err(|_| anyhow!("failed to decode blob metadata from the store"))
			})
			.transpose()
	}

	fn remove_blob_metadata(&self, hash: &BlobHash) -> Result<()> {
		let mut tx = DBTransaction::new();
		tx.delete(0, &Self::blob_key(hash));
		self.db.write(tx)?;
		Ok(())
	}

	fn insert_blob_retry(&self, hash: &BlobHash, count: u16) -> Result<()> {
		let mut tx = DBTransaction::new();
		tx.put(0, &Self::blob_count_key(hash), &count.encode());
		self.db.write(tx)?;
		Ok(())
	}
	fn get_blob_retry(&self, hash: &BlobHash) -> Result<u16> {
		self.db
			.get(0, &Self::blob_count_key(hash))?
			.map(|bytes| {
				let mut slice = bytes.as_slice();
				u16::decode(&mut slice)
					.map_err(|_| anyhow!("failed to decode blob retry value from the store"))
			})
			.transpose()
			.map(|opt| opt.unwrap_or(0))
	}

	fn remove_blob_retry(&self, hash: &BlobHash) -> Result<()> {
		let mut tx = DBTransaction::new();
		tx.delete(0, &Self::blob_count_key(hash));
		self.db.write(tx)?;
		Ok(())
	}

	fn insert_shards(&self, shards: &Vec<Shard>) -> Result<()> {
		let mut tx = DBTransaction::new();
		for shard in shards {
			tx.put(
				0,
				&Self::shard_key(&shard.blob_hash, shard.shard_id),
				&shard.encode(),
			);
		}
		self.db.write(tx)?;
		// let _ = self.log_all_entries();
		Ok(())
	}

	fn get_shard(&self, hash: &BlobHash, shard_id: u16) -> Result<Option<Shard>> {
		self.db
			.get(0, &Self::shard_key(hash, shard_id))?
			.map(|bytes| {
				let mut slice = bytes.as_slice();
				Shard::decode(&mut slice)
					.map_err(|_| anyhow!("failed to decode shard from the store"))
			})
			.transpose()
	}

	fn remove_shards(&self, shards: &Vec<(BlobHash, u16)>) -> Result<()> {
		let mut tx = DBTransaction::new();
		for (blob_hash, shard_index) in shards {
			tx.delete(0, &&Self::shard_key(blob_hash, *shard_index));
		}
		self.db.write(tx)?;
		Ok(())
	}

	fn log_all_entries(&self) -> Result<()> {
		let mut iter = self.db.iter(0);
		log::info!(target: LOG_TARGET, "📦 Logging all entries in RocksdbShardStore...");

		while let Some(entry) = iter.next() {
			match entry {
				Ok((key, value)) => {
					if key.starts_with(b"blob:") {
						let hash_hex = hex::encode(&key[5..]);
						match BlobMetadata::decode(&mut &value[..]) {
							Ok(metadata) => {
								log::info!(
									target: LOG_TARGET,
									"🔹 BlobMetadata [hash: 0x{}, blob.len = {}, nb_shards = {}, ownership = {:#?}]",
									hash_hex,
									metadata.size,
									metadata.nb_shards,
									metadata.ownership,
								);
							},
							Err(e) => {
								log::error!(
									target: LOG_TARGET,
									"❌ Failed to decode BlobMetadata for key 0x{}: {:?}",
									hash_hex,
									e
								);
							},
						}
					} else if key.starts_with(b"shard:") {
						let hash_hex = hex::encode(&key[6..38]); // 32 bytes after "shard:"
						let shard_id = u16::from_be_bytes([key[38], key[39]]);
						match Shard::decode(&mut &value[..]) {
							Ok(shard) => {
								log::info!(
									target: LOG_TARGET,
									"🧩 Shard [hash: 0x{}, id: {}]: data.len = {}, blob_hash = {}",
									hash_hex,
									shard.shard_id,
									shard.size,
									shard.blob_hash,
								);
							},
							Err(e) => {
								log::error!(
									target: LOG_TARGET,
									"❌ Failed to decode Shard [hash: 0x{}, id: {}]: {:?}",
									hash_hex,
									shard_id,
									e
								);
							},
						}
					} else {
						log::info!(
							target: LOG_TARGET,
							"❓ Unknown entry: key = {}, value.len = {}",
							hex::encode(&key),
							value.len()
						);
					}
				},
				Err(e) => {
					log::error!(target: LOG_TARGET,"❌ Failed to read from DB iterator: {:?}", e);
				},
			}
		}
		log::info!(target: LOG_TARGET,"✅ Finished logging RocksDB entries.");
		Ok(())
	}
}
