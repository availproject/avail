use anyhow::{anyhow, Result};
use codec::{Decode, Encode};
use kvdb::DBTransaction;
use kvdb_rocksdb::{Database, DatabaseConfig};
use sp_runtime::traits::Block as BlockT;
use std::{marker::PhantomData, path::Path};
use tempfile::TempDir;

use crate::{
	types::{BlobHash, BlobMetadata, Shard},
	LOG_TARGET,
};

const COL_BLOB_METADATA: u32 = 0;
const COL_BLOB_RETRY: u32 = 1;
const COL_SHARDS: u32 = 2;

pub trait ShardStore<Block: BlockT>: Send + Sync + 'static {
	// Blob metadata
	fn insert_blob_metadata(
		&self,
		hash: &BlobHash,
		blob_metadata: &BlobMetadata<Block>,
	) -> Result<()>;
	fn get_blob_metadata(&self, hash: &BlobHash) -> Result<Option<BlobMetadata<Block>>>;

	// Blob read error retry count
	fn insert_blob_retry(&self, hash: &BlobHash, count: u16) -> Result<()>;
	fn get_blob_retry(&self, hash: &BlobHash) -> Result<u16>;

	// Shards
	fn insert_shards(&self, shards: &Vec<Shard>) -> Result<()>;
	fn get_shard(&self, hash: &BlobHash, shard_id: u16) -> Result<Option<Shard>>;

	// Cleaning
	fn clean_blob_data(&self, hash: &BlobHash) -> Result<()>;
	fn clean_expired_blobs(&self, current_block: u64) -> Result<()>;

	// Testing stuff
	fn log_all_entries(&self) -> Result<()>;
}

pub struct RocksdbShardStore<Block: BlockT> {
	db: Database,
	_block: PhantomData<Block>,
}

impl<Block: BlockT> RocksdbShardStore<Block> {
	/// Open (or create) a new DB at `path`, with a single column.
	pub fn open(path: impl AsRef<Path>) -> Result<Self> {
		let num_columns = 3;
		let db_config = DatabaseConfig::with_columns(num_columns);
		let db = Database::open(&db_config, path.as_ref())?;
		Ok(RocksdbShardStore::<Block> {
			db,
			_block: PhantomData,
		})
	}

	/// blob key = b"blob:" || hash_bytes
	fn blob_key(hash: &BlobHash) -> Vec<u8> {
		hash.0.to_vec()
	}

	/// blob key = b"blob:" || hash_bytes
	fn blob_count_key(hash: &BlobHash) -> Vec<u8> {
		hash.0.to_vec()
	}

	/// shard key = b"shard:" || hash_bytes || shard_id_be
	fn shard_key(hash: &BlobHash, shard_id: u16) -> Vec<u8> {
		let mut key = hash.0.to_vec();
		key.extend_from_slice(&shard_id.to_be_bytes());
		key
	}
}

impl<Block: BlockT> Default for RocksdbShardStore<Block> {
	fn default() -> Self {
		let temp_dir = TempDir::new().expect("failed to create temp dir for RocksdbShardStore");
		let db_path = temp_dir.path().join("blob_store");
		let num_columns = 3;
		let db_config = DatabaseConfig::with_columns(num_columns);
		let db = Database::open(&db_config, db_path).expect("opening RocksDB blob store failed");
		RocksdbShardStore::<Block> {
			db,
			_block: PhantomData,
		}
	}
}

impl<Block: BlockT> ShardStore<Block> for RocksdbShardStore<Block> {
	fn insert_blob_metadata(
		&self,
		hash: &BlobHash,
		blob_metadata: &BlobMetadata<Block>,
	) -> Result<()> {
		let mut tx = DBTransaction::new();
		tx.put(
			COL_BLOB_METADATA,
			&Self::blob_key(hash),
			&blob_metadata.encode(),
		);
		self.db.write(tx)?;
		Ok(())
	}

	fn get_blob_metadata(&self, hash: &BlobHash) -> Result<Option<BlobMetadata<Block>>> {
		self.db
			.get(COL_BLOB_METADATA, &Self::blob_key(hash))?
			.map(|bytes| {
				let mut slice = bytes.as_slice();
				BlobMetadata::decode(&mut slice)
					.map_err(|_| anyhow!("failed to decode blob metadata from the store"))
			})
			.transpose()
	}

	fn insert_blob_retry(&self, hash: &BlobHash, count: u16) -> Result<()> {
		let mut tx = DBTransaction::new();
		tx.put(COL_BLOB_RETRY, &Self::blob_count_key(hash), &count.encode());
		self.db.write(tx)?;
		Ok(())
	}
	fn get_blob_retry(&self, hash: &BlobHash) -> Result<u16> {
		self.db
			.get(COL_BLOB_RETRY, &Self::blob_count_key(hash))?
			.map(|bytes| {
				let mut slice = bytes.as_slice();
				u16::decode(&mut slice)
					.map_err(|_| anyhow!("failed to decode blob retry value from the store"))
			})
			.transpose()
			.map(|opt| opt.unwrap_or(0))
	}

	fn insert_shards(&self, shards: &Vec<Shard>) -> Result<()> {
		let mut tx = DBTransaction::new();
		for shard in shards {
			tx.put(
				COL_SHARDS,
				&Self::shard_key(&shard.blob_hash, shard.shard_id),
				&shard.encode(),
			);
		}
		self.db.write(tx)?;
		Ok(())
	}

	fn get_shard(&self, hash: &BlobHash, shard_id: u16) -> Result<Option<Shard>> {
		self.db
			.get(COL_SHARDS, &Self::shard_key(hash, shard_id))?
			.map(|bytes| {
				let mut slice = bytes.as_slice();
				Shard::decode(&mut slice)
					.map_err(|_| anyhow!("failed to decode shard from the store"))
			})
			.transpose()
	}

	fn clean_blob_data(&self, hash: &BlobHash) -> Result<()> {
		if let Some(blob_metadata) = self.get_blob_metadata(hash)? {
			let mut tx = DBTransaction::new();

			// remove blob metadata
			tx.delete(COL_BLOB_METADATA, &Self::blob_key(hash));

			// remove blob retry
			tx.delete(COL_BLOB_RETRY, &Self::blob_count_key(hash));

			// remove all shards
			for shard_id in 0..blob_metadata.nb_shards {
				tx.delete(COL_SHARDS, &Self::shard_key(hash, shard_id));
			}

			self.db.write(tx)?;
		}
		Ok(())
	}

	fn clean_expired_blobs(&self, current_block: u64) -> Result<()> {
		let mut expired_blobs = Vec::new();
		for (key, value) in self.db.iter(COL_BLOB_METADATA).filter_map(Result::ok) {
			if let Ok(blob_metadata) = BlobMetadata::<Block>::decode(&mut value.as_slice()) {
				if blob_metadata.expires_at <= current_block {
					let hash = BlobHash::from_slice(&key);
					expired_blobs.push(hash);
				}
			} else {
				log::warn!(
					target: LOG_TARGET,
					"Failed to decode blob metadata for key {:?}",
					key
				);
			}
		}

		for hash in expired_blobs {
			self.clean_blob_data(&hash)?;
		}

		Ok(())
	}

	fn log_all_entries(&self) -> Result<()> {
		log::info!(target: LOG_TARGET, "--- Logging all entries in the blob store ---");

		// Log Blob Metadata
		log::info!(target: LOG_TARGET, "--- Blob Metadatas ---");
		for (_key, value) in self.db.iter(COL_BLOB_METADATA).filter_map(Result::ok) {
			if let Ok(blob_metadata) = BlobMetadata::<Block>::decode(&mut value.as_slice()) {
				log::info!(
					target: LOG_TARGET,
					"Blob: hash={:?}, size={}, nb_shards={}, commitments_len={}, ownership={:?}, is_notified={}, expires_at={}",
					blob_metadata.hash,
					blob_metadata.size,
					blob_metadata.nb_shards,
					blob_metadata.commitments.len(),
					blob_metadata.ownership,
					blob_metadata.is_notified,
					blob_metadata.expires_at
				);
			}
		}

		// Log Shards
		log::info!(target: LOG_TARGET, "--- Shards ---");
		for (_key, value) in self.db.iter(COL_SHARDS).filter_map(Result::ok) {
			if let Ok(shard) = Shard::decode(&mut value.as_slice()) {
				log::info!(
					target: LOG_TARGET,
					"Shard: hash={:?}, shard_id={}, size={}",
					shard.blob_hash,
					shard.shard_id,
					shard.size
				);
			}
		}

		// Log Blob Retries
		log::info!(target: LOG_TARGET, "--- Blob Retries ---");
		for (key, value) in self.db.iter(COL_BLOB_RETRY).filter_map(Result::ok) {
			if let Ok(count) = u16::decode(&mut value.as_slice()) {
				let hash = BlobHash::from_slice(&key);
				log::info!(target: LOG_TARGET, "Blob Retry: hash={:?}, count={}", hash, count);
			}
		}

		log::info!(target: LOG_TARGET, "--- End of blob store log ---");
		Ok(())
	}
}
