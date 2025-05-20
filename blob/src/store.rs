use anyhow::Result;
use codec::{Decode, Encode};
use kvdb::DBTransaction;
use kvdb_rocksdb::Database;
use std::path::Path;
use tempfile::TempDir;

use crate::types::{BlobHash, BlobMetadata, Shard};

pub trait ShardStore: Send + Sync + 'static {
	fn insert_blob_metadata(&self, hash: &BlobHash, blob_metadata: &BlobMetadata) -> Result<()>;
	fn remove_blob_metadata(&self, hash: &BlobHash) -> Result<()>;
	fn insert_shards(&self, shards: &Vec<Shard>) -> Result<()>;
	fn remove_shards(&self, shards: &Vec<(BlobHash, u16)>) -> Result<()>;
	fn has_shard(&self, hash: &BlobHash, shard_id: u16) -> Result<bool>;
	fn get_shard(&self, hash: &BlobHash, shard_id: u16) -> Result<Option<Vec<u8>>>;
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
		Ok(())
	}

	fn remove_blob_metadata(&self, hash: &BlobHash) -> Result<()> {
		let mut tx = DBTransaction::new();
		tx.delete(0, &Self::blob_key(hash));
		self.db.write(tx)?;
		Ok(())
	}

	fn insert_shards(&self, shards: &Vec<Shard>) -> Result<()> {
		let mut tx = DBTransaction::new();
		for shard in shards {
			tx.put(
				0,
				&&Self::shard_key(&shard.hash, shard.shard_id),
				&shard.encode(),
			);
		}
		self.db.write(tx)?;
		Ok(())
	}

	fn remove_shards(&self, shards: &Vec<(BlobHash, u16)>) -> Result<()> {
		let mut tx = DBTransaction::new();
		for (blob_hash, shard_index) in shards {
			tx.delete(0, &&Self::shard_key(blob_hash, *shard_index));
		}
		self.db.write(tx)?;
		Ok(())
	}

	fn has_shard(&self, hash: &BlobHash, shard_id: u16) -> Result<bool> {
		Ok(self
			.db
			.get(0, &Self::shard_key(hash, shard_id))
			.map(|opt| opt.is_some())?)
	}

	fn get_shard(&self, hash: &BlobHash, shard_id: u16) -> Result<Option<Vec<u8>>> {
		Ok(self.db.get(0, &Self::shard_key(hash, shard_id))?)
	}

	fn log_all_entries(&self) -> Result<()> {
		let mut iter = self.db.iter(0);
		log::info!("📦 Logging all entries in RocksdbShardStore...");

		while let Some(entry) = iter.next() {
			match entry {
				Ok((key, value)) => {
					if key.starts_with(b"blob:") {
						let hash_hex = hex::encode(&key[5..]); // after "blob:"
						match BlobMetadata::decode(&mut &value[..]) {
							Ok(metadata) => {
								log::info!(
									"🔹 BlobMetadata [hash: 0x{}]: {:#?}",
									hash_hex,
									metadata
								);
							},
							Err(e) => {
								log::error!(
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
									"🧩 Shard [hash: 0x{}, id: {}]: data.len = {}, ttl = {}",
									hash_hex,
									shard.shard_id,
									shard.data.len(),
									shard.block_ttl
								);
							},
							Err(e) => {
								log::error!(
									"❌ Failed to decode Shard [hash: 0x{}, id: {}]: {:?}",
									hash_hex,
									shard_id,
									e
								);
							},
						}
					} else {
						log::info!(
							"❓ Unknown entry: key = {}, value.len = {}",
							hex::encode(&key),
							value.len()
						);
					}
				},
				Err(e) => {
					log::error!("❌ Failed to read from DB iterator: {:?}", e);
				},
			}
		}
		log::info!("✅ Finished logging RocksDB entries.");
		Ok(())
	}
}
