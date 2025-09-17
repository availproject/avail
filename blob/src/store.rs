use anyhow::{anyhow, Result};
use codec::{Decode, Encode};
use kvdb::DBTransaction;
use kvdb::KeyValueDB;
use kvdb_rocksdb::{Database, DatabaseConfig};
use sp_runtime::traits::Block as BlockT;
use std::sync::Mutex;
use std::{marker::PhantomData, path::Path};
use tempfile::TempDir;
use ttl_cache::TtlCache;
use crate::types::CompressedBlob;

use crate::{
	types::{Blob, BlobHash, BlobMetadata, OwnershipEntry},
	BLOB_CACHE_DURATION, LOG_TARGET, MAX_BLOBS_IN_CACHE,
};

const COL_BLOB_METADATA: u32 = 0;
const COL_BLOB_RETRY: u32 = 1;
const COL_BLOB: u32 = 2;
const COL_BLOB_OWNERSHIP: u32 = 3;
const COL_BLOB_OWNERSHIP_EXPIRY: u32 = 4;

pub trait BlobStore<Block: BlockT>: Send + Sync + 'static {
	// Blob metadata
	fn insert_blob_metadata(&self, blob_metadata: &BlobMetadata<Block>) -> Result<()>;
	fn get_blob_metadata(&self, hash: &BlobHash) -> Result<Option<BlobMetadata<Block>>>;
	fn blob_metadata_exists(&self, hash: &BlobHash) -> Result<bool>;

	// Blobs
	fn insert_blob(&self, blob_hash: &BlobHash, blob: &CompressedBlob) -> Result<()>;
	fn get_blob(&self, hash: &BlobHash) -> Result<Option<Blob>>;
	fn get_raw_blob(&self, hash: &BlobHash) -> Result<Option<CompressedBlob>>;

	// Blobs ownership
	fn insert_blob_ownership(&self, hash: &BlobHash, o: &OwnershipEntry) -> Result<()>;
	fn get_blob_ownership(
		&self,
		hash: &BlobHash,
		owner: &Vec<u8>,
	) -> Result<Option<OwnershipEntry>>;
	fn get_blob_ownerships(&self, hash: &BlobHash) -> Result<Vec<OwnershipEntry>>;

	// Blob read error retry count
	fn insert_blob_retry(&self, hash: &BlobHash, count: u16) -> Result<()>;
	fn get_blob_retry(&self, hash: &BlobHash) -> Result<u16>;

	// Blob ownership expiry -> In case an ownership is recorded without ever seeing the metadata
	fn insert_blob_ownership_expiry(&self, hash: &BlobHash, expires_at: u64) -> Result<()>;
	fn get_blob_ownership_expiry(&self, hash: &BlobHash) -> Result<Option<u64>>;
	fn remove_blob_ownership_expiry(&self, hash: &BlobHash) -> Result<()>;

	// Cleaning
	fn clean_blobs_info(&self, hashes: &Vec<BlobHash>) -> Result<()>;
	fn clean_expired_ownerships_without_metadata(&self, hashes: &Vec<BlobHash>) -> Result<()>;
	fn clean_expired_blobs_info(
		&self,
		current_block: u64,
	) -> Result<(Vec<BlobHash>, Vec<BlobHash>)>;

	// Testing stuff
	fn log_all_entries(&self) -> Result<()>;
	fn clear_blob_storage(&self) -> Result<()>;
}

pub struct RocksdbBlobStore<Block: BlockT> {
	db: Database,
	_block: PhantomData<Block>,
	cache: Mutex<TtlCache<BlobHash, Vec<u8>>>,
}

impl<Block: BlockT> RocksdbBlobStore<Block> {
	/// Open (or create) a new DB at `path`, with a single column.
	pub fn open(path: impl AsRef<Path>) -> Result<Self> {
		let num_columns = 5;
		let db_config = DatabaseConfig::with_columns(num_columns);
		let db = Database::open(&db_config, path.as_ref())?;
		Ok(RocksdbBlobStore::<Block> {
			db,
			_block: PhantomData,
			cache: Mutex::new(TtlCache::new(MAX_BLOBS_IN_CACHE as usize)), // keep ~128 blobs
		})
	}

	/// Blob metadata key
	fn blob_meta_key(hash: &BlobHash) -> Vec<u8> {
		hash.0.to_vec()
	}

	/// Blob count retry key
	fn blob_count_key(hash: &BlobHash) -> Vec<u8> {
		hash.0.to_vec()
	}

	/// Blob key
	fn blob_key(hash: &BlobHash) -> Vec<u8> {
		hash.0.to_vec()
	}

	/// Blob ownership key prefix
	fn blob_ownership_key_prefix(hash: &BlobHash) -> Vec<u8> {
		hash.0.to_vec()
	}

	/// Blob ownership expiry key prefix
	fn blob_ownership_expiry_key(hash: &BlobHash) -> Vec<u8> {
		hash.0.to_vec()
	}

	/// Blob ownership key
	fn blob_ownership_key(hash: &BlobHash, owner: &Vec<u8>) -> Vec<u8> {
		let prefix = Self::blob_ownership_key_prefix(hash);
		let mut k = Vec::with_capacity(prefix.len() + owner.len());
		k.extend_from_slice(&prefix);
		k.extend_from_slice(&owner);
		k
	}
}

impl<Block: BlockT> Default for RocksdbBlobStore<Block> {
	fn default() -> Self {
		let temp_dir = TempDir::new().expect("failed to create temp dir for RocksdbBlobStore");
		let db_path = temp_dir.path().join("blob_store");
		let num_columns = 4;
		let db_config = DatabaseConfig::with_columns(num_columns);
		let db = Database::open(&db_config, db_path).expect("opening RocksDB blob store failed");
		RocksdbBlobStore::<Block> {
			db,
			_block: PhantomData,
			cache: Mutex::new(TtlCache::new(MAX_BLOBS_IN_CACHE as usize)),
		}
	}
}

impl<Block: BlockT> BlobStore<Block> for RocksdbBlobStore<Block> {
	fn insert_blob_metadata(&self, blob_metadata: &BlobMetadata<Block>) -> Result<()> {
		if let Some(existing) = self.get_blob_metadata(&blob_metadata.hash).ok().flatten() {
			if existing.is_notified {
				return Ok(());
			}
		}

		let mut tx = DBTransaction::new();
		tx.put(
			COL_BLOB_METADATA,
			&Self::blob_meta_key(&blob_metadata.hash),
			&blob_metadata.encode(),
		);
		self.db.write(tx)?;
		Ok(())
	}

	fn get_blob_metadata(&self, hash: &BlobHash) -> Result<Option<BlobMetadata<Block>>> {
		self.db
			.get(COL_BLOB_METADATA, &Self::blob_meta_key(hash))?
			.map(|bytes| {
				let mut slice = bytes.as_slice();
				BlobMetadata::decode(&mut slice)
					.map_err(|_| anyhow!("failed to decode blob metadata from the store"))
			})
			.transpose()
	}

	fn blob_metadata_exists(&self, hash: &BlobHash) -> Result<bool> {
		Ok(self
			.db
			.get(COL_BLOB_METADATA, &Self::blob_meta_key(hash))?
			.is_some())
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

	fn insert_blob(&self, blob_hash: &BlobHash, blob: &CompressedBlob) -> Result<()> {
		let blob_encoded = blob.encode();
		
		// Write to db
		let mut tx = DBTransaction::new();
		tx.put(COL_BLOB, &Self::blob_key(&blob_hash), &blob_encoded);
		self.db.write(tx)?;

		// Write to cache
		if let Ok(mut cache) = self.cache.lock() {
			cache.insert(blob_hash.clone(), blob_encoded, BLOB_CACHE_DURATION);
		}

		Ok(())
	}

	fn get_blob(&self, hash: &BlobHash) -> Result<Option<Blob>> {
		match self.get_raw_blob(hash)? {
			Some(compressed_blob) => {
				let timer = std::time::Instant::now();
				let data = compressed_blob.data()?;
				let mut slice = data.as_slice();
				let decoded = Blob::decode(&mut slice)
					.map_err(|_| anyhow!("failed to decode blob from the store"))?;
				log::info!(
					"GET_BLOB - Decoding took - {:?} - hash: {:?}",
					timer.elapsed(),
					hash
				);
				Ok(Some(decoded))
			},
			None => Ok(None),
		}
	}

	fn get_raw_blob(&self, hash: &BlobHash) -> Result<Option<CompressedBlob>> {
		let timer = std::time::Instant::now();
		// Try to read from cache
		if let Ok(cache) = self.cache.lock() {
			if let Some(cached) = cache.get(hash).cloned() {
				log::info!(
					"GET_RAW_BLOB - CACHE HIT - {:?} - hash: {:?}",
					timer.elapsed(),
					hash
				);

				let cached = CompressedBlob::decode(cached).unwrap();
				return Ok(Some(cached));
			}
		}

		// Fallback to db
		let data = self.db.get(COL_BLOB, &Self::blob_key(hash))?;

		// Write to cache
		if let Some(ref v) = data {
			if let Ok(mut cache) = self.cache.lock() {
				cache.insert(hash.clone(), v.clone(), BLOB_CACHE_DURATION);
			}
		}

		log::info!(
			"GET_RAW_BLOB - CACHE MISS - {:?} - hash: {:?}",
			timer.elapsed(),
			hash
		);

		let Some(data) = data else {
			return Ok(None);
		};
		
		let data = CompressedBlob::decode(data).unwrap();
		Ok(Some(data))
	}

	fn insert_blob_ownership(&self, hash: &BlobHash, o: &OwnershipEntry) -> Result<()> {
		let mut tx = DBTransaction::new();
		tx.put(
			COL_BLOB_OWNERSHIP,
			&Self::blob_ownership_key(hash, &o.address.encode()),
			&o.encode(),
		);
		self.db.write(tx)?;
		Ok(())
	}

	fn get_blob_ownership(
		&self,
		hash: &BlobHash,
		owner: &Vec<u8>,
	) -> Result<Option<OwnershipEntry>> {
		let key = Self::blob_ownership_key(hash, owner);
		self.db
			.get(COL_BLOB_OWNERSHIP, &key)?
			.map(|bytes| {
				let mut slice = bytes.as_slice();
				OwnershipEntry::decode(&mut slice)
					.map_err(|_| anyhow!("failed to decode blob from the store"))
			})
			.transpose()
	}

	fn get_blob_ownerships(&self, hash: &BlobHash) -> Result<Vec<OwnershipEntry>> {
		let prefix = Self::blob_ownership_key_prefix(hash);
		let iter = self.db.iter_with_prefix(COL_BLOB_OWNERSHIP, &prefix);
		let mut out = Vec::new();
		for kv in iter {
			let (_k, v) = kv?;
			let mut s = v.as_slice();
			let entry = OwnershipEntry::decode(&mut s)
				.map_err(|_| anyhow!("failed to decode ownership entry"))?;
			out.push(entry);
		}
		Ok(out)
	}

	fn insert_blob_ownership_expiry(&self, hash: &BlobHash, expires_at: u64) -> Result<()> {
		let mut tx = DBTransaction::new();
		tx.put(
			COL_BLOB_OWNERSHIP_EXPIRY,
			&Self::blob_ownership_expiry_key(hash),
			&expires_at.encode(),
		);
		self.db.write(tx)?;
		Ok(())
	}

	fn get_blob_ownership_expiry(&self, hash: &BlobHash) -> Result<Option<u64>> {
		self.db
			.get(
				COL_BLOB_OWNERSHIP_EXPIRY,
				&Self::blob_ownership_expiry_key(hash),
			)?
			.map(|bytes| {
				let mut slice = bytes.as_slice();
				u64::decode(&mut slice).map_err(|_| {
					anyhow!("failed to decode blob ownership expiry value from the store")
				})
			})
			.transpose()
	}

	fn remove_blob_ownership_expiry(&self, hash: &BlobHash) -> Result<()> {
		let mut tx = DBTransaction::new();

		tx.delete(
			COL_BLOB_OWNERSHIP_EXPIRY,
			&Self::blob_ownership_expiry_key(hash),
		);

		self.db.write(tx)?;
		Ok(())
	}

	fn clean_blobs_info(&self, hashes: &Vec<BlobHash>) -> Result<()> {
		let mut tx = DBTransaction::new();

		for hash in hashes {
			// remove blob metadata
			tx.delete(COL_BLOB_METADATA, &Self::blob_meta_key(hash));

			// remove blob retry
			tx.delete(COL_BLOB_RETRY, &Self::blob_count_key(hash));

			// remove blob
			tx.delete(COL_BLOB, &Self::blob_key(hash));

			// remove blob ownership
			tx.delete_prefix(COL_BLOB_OWNERSHIP, &Self::blob_ownership_key_prefix(hash));

			// remove blob ownership expiry
			tx.delete(
				COL_BLOB_OWNERSHIP_EXPIRY,
				&Self::blob_ownership_expiry_key(hash),
			);
		}

		self.db.write(tx)?;
		Ok(())
	}

	fn clean_expired_ownerships_without_metadata(&self, hashes: &Vec<BlobHash>) -> Result<()> {
		let mut tx = DBTransaction::new();

		for hash in hashes {
			// remove blob ownership
			tx.delete_prefix(COL_BLOB_OWNERSHIP, &Self::blob_ownership_key_prefix(hash));

			// remove blob ownership expiry
			tx.delete(
				COL_BLOB_OWNERSHIP_EXPIRY,
				&Self::blob_ownership_expiry_key(hash),
			);
		}

		self.db.write(tx)?;
		Ok(())
	}

	fn clean_expired_blobs_info(
		&self,
		current_block: u64,
	) -> Result<(Vec<BlobHash>, Vec<BlobHash>)> {
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

		if !expired_blobs.is_empty() {
			self.clean_blobs_info(&expired_blobs)?;
		}

		let mut expired_ownerships = Vec::new();
		for (key, value) in self
			.db
			.iter(COL_BLOB_OWNERSHIP_EXPIRY)
			.filter_map(Result::ok)
		{
			if let Ok(expiry) = u64::decode(&mut value.as_slice()) {
				if expiry <= current_block {
					let hash = BlobHash::from_slice(&key);
					expired_ownerships.push(hash);
				}
			} else {
				log::warn!(
					target: LOG_TARGET,
					"Failed to decode blob ownership expiry for key {:?}",
					key
				);
			}
		}
		if !expired_ownerships.is_empty() {
			self.clean_expired_ownerships_without_metadata(&expired_ownerships)?;
		}

		Ok((expired_blobs, expired_ownerships))
	}

	fn log_all_entries(&self) -> Result<()> {
		log::info!(target: LOG_TARGET, "--- Logging all entries in the blob store ---");

		// Log Blob Metadata
		log::info!(target: LOG_TARGET, "--- Blob Metadatas ---");
		for (_key, value) in self.db.iter(COL_BLOB_METADATA).filter_map(Result::ok) {
			if let Ok(blob_metadata) = BlobMetadata::<Block>::decode(&mut value.as_slice()) {
				log::info!(
					target: LOG_TARGET,
					"Blob: hash={:?}, size={}, commitments_len={}, is_notified={}, nb_val_per_blob={}, expires_at={}",
					blob_metadata.hash,
					blob_metadata.size,
					blob_metadata.commitment.len(),
					blob_metadata.is_notified,
					blob_metadata.nb_validators_per_blob,
					blob_metadata.expires_at,
				);
			}
		}

		// Log Blob Retries
		log::info!(target: LOG_TARGET, "--- Blob Ownerships ---");
		for (key, value) in self.db.iter(COL_BLOB_OWNERSHIP).filter_map(Result::ok) {
			if let Ok(o) = OwnershipEntry::decode(&mut value.as_slice()) {
				const BLOB_HASH_LEN: usize = 32;
				if key.len() < BLOB_HASH_LEN {
					continue;
				}
				let hash = BlobHash::from_slice(&key[..BLOB_HASH_LEN]);
				log::info!(target: LOG_TARGET, "Blob Ownership: hash={:?}, ownership={:?}", hash, o.address);
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

	fn clear_blob_storage(&self) -> Result<()> {
		let mut tx = DBTransaction::new();

		for col in 0..self.db.num_columns() {
			tx.delete_prefix(col, &[]);
		}

		self.db.write(tx)?;
		Ok(())
	}
}
