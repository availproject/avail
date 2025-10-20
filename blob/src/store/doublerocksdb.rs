use crate::{store::*, types::CompressedBlob};
use anyhow::{anyhow, Result};
use codec::{Decode, Encode};
use kvdb::{DBTransaction, KeyValueDB};
use kvdb_rocksdb::{Database, DatabaseConfig};
use std::{path::Path, sync::Mutex};
use tempfile::TempDir;
use ttl_cache::TtlCache;

use crate::{
	types::{Blob, BlobHash, BlobMetadata, OwnershipEntry},
	BLOB_CACHE_DURATION, LOG_TARGET, MAX_BLOBS_IN_CACHE,
};

// Second implementation, double rocks db, one for blobs
pub struct DoubleRocksdbBlobStore {
	db_meta: Database,
	db_blob: Database,
	cache: Mutex<TtlCache<BlobHash, Vec<u8>>>,
}
impl DoubleRocksdbBlobStore {
	pub const COL_BLOB_METADATA: u32 = 0;
	pub const COL_BLOB_RETRY: u32 = 1;
	pub const COL_BLOB_OWNERSHIP: u32 = 2;
	pub const COL_BLOB_OWNERSHIP_EXPIRY: u32 = 3;

	pub const COL_BLOB: u32 = 1;

	pub fn open(path: impl AsRef<Path>) -> Result<Self> {
		let base = path.as_ref();

		let meta_dir = base.join("meta");
		let num_columns = 4;
		let db_config = DatabaseConfig::with_columns(num_columns);
		let db_meta = Database::open(&db_config, meta_dir)?;

		let blob_dir = base.join("blob");
		let num_columns = 1;
		let db_config = DatabaseConfig::with_columns(num_columns);
		let db_blob = Database::open(&db_config, blob_dir)?;

		Ok(DoubleRocksdbBlobStore {
			db_meta,
			db_blob,
			cache: Mutex::new(TtlCache::new(MAX_BLOBS_IN_CACHE as usize)),
		})
	}
}
impl Default for DoubleRocksdbBlobStore {
	fn default() -> Self {
		let temp_dir =
			TempDir::new().expect("failed to create temp dir for DoubleRocksdbBlobStore");
		let db_path = temp_dir.path().join("blob_database");
		let store = Self::open(db_path).expect("opening RocksDB blob store failed");
		store
	}
}
impl StorageApiT for DoubleRocksdbBlobStore {
	fn insert_blob_metadata(&self, blob_metadata: &BlobMetadata) -> Result<()> {
		if let Some(existing) = self.get_blob_metadata(&blob_metadata.hash).ok().flatten() {
			if existing.is_notified {
				return Ok(());
			}
		}

		let mut tx = DBTransaction::new();
		tx.put(
			Self::COL_BLOB_METADATA,
			&blob_meta_key(&blob_metadata.hash),
			&blob_metadata.encode(),
		);
		self.db_meta.write(tx)?;
		Ok(())
	}

	fn get_blob_metadata(&self, hash: &BlobHash) -> Result<Option<BlobMetadata>> {
		self.db_meta
			.get(Self::COL_BLOB_METADATA, &blob_meta_key(hash))?
			.map(|bytes| {
				let mut slice = bytes.as_slice();
				BlobMetadata::decode(&mut slice)
					.map_err(|_| anyhow!("failed to decode blob metadata from the store"))
			})
			.transpose()
	}

	fn blob_metadata_exists(&self, hash: &BlobHash) -> Result<bool> {
		Ok(self
			.db_meta
			.get(Self::COL_BLOB_METADATA, &blob_meta_key(hash))?
			.is_some())
	}

	fn insert_blob(&self, blob_hash: &BlobHash, blob: &CompressedBlob) -> Result<()> {
		let blob_encoded = blob.encode();

		let mut tx = DBTransaction::new();
		tx.put(Self::COL_BLOB, &blob_key(blob_hash), &blob_encoded);
		self.db_blob.write(tx)?;

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
					.map_err(|_| anyhow!("failed to decode blob from the blob store"))?;
				log::info!(
					"GET_BLOB[Double] - Decoding took - {:?} - hash: {:?}",
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

		// Try cache first
		if let Ok(cache) = self.cache.lock() {
			if let Some(cached) = cache.get(hash).cloned() {
				log::info!(
					"GET_RAW_BLOB[Double] - CACHE HIT - {:?} - hash: {:?}",
					timer.elapsed(),
					hash
				);
				let cached = CompressedBlob::decode(cached)
					.map_err(|_| anyhow!("Failed to decode compressed data (cache)."))?;
				return Ok(Some(cached));
			}
		}

		// Fallback to blob DB
		let data = self.db_blob.get(Self::COL_BLOB, &blob_key(hash))?;

		// Populate cache if present
		if let Some(ref v) = data {
			if let Ok(mut cache) = self.cache.lock() {
				cache.insert(hash.clone(), v.clone(), BLOB_CACHE_DURATION);
			}
		}

		log::info!(
			"GET_RAW_BLOB[Double] - CACHE MISS - {:?} - hash: {:?}",
			timer.elapsed(),
			hash
		);

		let Some(data) = data else {
			return Ok(None);
		};

		let data = CompressedBlob::decode(data)
			.map_err(|_| anyhow!("Failed to decode compressed data (blob db)."))?;
		Ok(Some(data))
	}

	fn insert_blob_ownership(&self, hash: &BlobHash, o: &OwnershipEntry) -> Result<()> {
		let mut tx = DBTransaction::new();
		tx.put(
			Self::COL_BLOB_OWNERSHIP,
			&blob_ownership_key(hash, &o.address.encode()),
			&o.encode(),
		);
		self.db_meta.write(tx)?;
		Ok(())
	}

	fn get_blob_ownership(
		&self,
		hash: &BlobHash,
		owner: &Vec<u8>,
	) -> Result<Option<OwnershipEntry>> {
		let key = blob_ownership_key(hash, owner);
		self.db_meta
			.get(Self::COL_BLOB_OWNERSHIP, &key)?
			.map(|bytes| {
				let mut slice = bytes.as_slice();
				OwnershipEntry::decode(&mut slice)
					.map_err(|_| anyhow!("failed to decode ownership entry from the meta store"))
			})
			.transpose()
	}

	fn get_blob_ownerships(&self, hash: &BlobHash) -> Result<Vec<OwnershipEntry>> {
		let prefix = blob_ownership_key_prefix(hash);
		let iter = self
			.db_meta
			.iter_with_prefix(Self::COL_BLOB_OWNERSHIP, &prefix);
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

	fn insert_blob_retry(&self, hash: &BlobHash, count: u16) -> Result<()> {
		let mut tx = DBTransaction::new();
		tx.put(Self::COL_BLOB_RETRY, &blob_count_key(hash), &count.encode());
		self.db_meta.write(tx)?;
		Ok(())
	}

	fn get_blob_retry(&self, hash: &BlobHash) -> Result<u16> {
		self.db_meta
			.get(Self::COL_BLOB_RETRY, &blob_count_key(hash))?
			.map(|bytes| {
				let mut slice = bytes.as_slice();
				u16::decode(&mut slice)
					.map_err(|_| anyhow!("failed to decode blob retry value from the meta store"))
			})
			.transpose()
			.map(|opt| opt.unwrap_or(0))
	}

	fn insert_blob_ownership_expiry(&self, hash: &BlobHash, expires_at: u64) -> Result<()> {
		let mut tx = DBTransaction::new();
		tx.put(
			Self::COL_BLOB_OWNERSHIP_EXPIRY,
			&blob_ownership_expiry_key(hash),
			&expires_at.encode(),
		);
		self.db_meta.write(tx)?;
		Ok(())
	}

	fn get_blob_ownership_expiry(&self, hash: &BlobHash) -> Result<Option<u64>> {
		self.db_meta
			.get(
				Self::COL_BLOB_OWNERSHIP_EXPIRY,
				&blob_ownership_expiry_key(hash),
			)?
			.map(|bytes| {
				let mut slice = bytes.as_slice();
				u64::decode(&mut slice).map_err(|_| {
					anyhow!("failed to decode blob ownership expiry value from the meta store")
				})
			})
			.transpose()
	}

	fn remove_blob_ownership_expiry(&self, hash: &BlobHash) -> Result<()> {
		let mut tx = DBTransaction::new();
		tx.delete(
			Self::COL_BLOB_OWNERSHIP_EXPIRY,
			&blob_ownership_expiry_key(hash),
		);
		self.db_meta.write(tx)?;
		Ok(())
	}

	fn clean_blobs_info(&self, hashes: &Vec<BlobHash>) -> Result<()> {
		let mut tx_meta = DBTransaction::new();
		let mut tx_blob = DBTransaction::new();

		for hash in hashes {
			// meta side
			tx_meta.delete(Self::COL_BLOB_METADATA, &blob_meta_key(hash));
			tx_meta.delete(Self::COL_BLOB_RETRY, &blob_count_key(hash));
			tx_meta.delete_prefix(Self::COL_BLOB_OWNERSHIP, &blob_ownership_key_prefix(hash));
			tx_meta.delete(
				Self::COL_BLOB_OWNERSHIP_EXPIRY,
				&blob_ownership_expiry_key(hash),
			);

			// blob side
			tx_blob.delete(Self::COL_BLOB, &blob_key(hash));
		}

		self.db_meta.write(tx_meta)?;
		self.db_blob.write(tx_blob)?;
		Ok(())
	}

	fn clean_expired_ownerships_without_metadata(&self, hashes: &Vec<BlobHash>) -> Result<()> {
		let mut tx_meta = DBTransaction::new();

		for hash in hashes {
			tx_meta.delete_prefix(Self::COL_BLOB_OWNERSHIP, &blob_ownership_key_prefix(hash));
			tx_meta.delete(
				Self::COL_BLOB_OWNERSHIP_EXPIRY,
				&blob_ownership_expiry_key(hash),
			);
		}

		self.db_meta.write(tx_meta)?;
		Ok(())
	}

	fn clean_expired_blobs_info(
		&self,
		current_block: u64,
	) -> Result<(Vec<BlobHash>, Vec<BlobHash>)> {
		// Expired metadata → remove full blob info (both DBs)
		let mut expired_blobs = Vec::new();
		for (key, value) in self
			.db_meta
			.iter(Self::COL_BLOB_METADATA)
			.filter_map(Result::ok)
		{
			if let Ok(blob_metadata) = BlobMetadata::decode(&mut value.as_slice()) {
				if blob_metadata.expires_at <= current_block {
					let hash = BlobHash::from_slice(&key);
					expired_blobs.push(hash);
				}
			} else {
				log::warn!(target: LOG_TARGET, "[Double] Failed to decode blob metadata for key {:?}", key);
			}
		}

		if !expired_blobs.is_empty() {
			self.clean_blobs_info(&expired_blobs)?;
		}

		// Expired ownership-expiry entries where metadata never arrived
		let mut expired_ownerships = Vec::new();
		for (key, value) in self
			.db_meta
			.iter(Self::COL_BLOB_OWNERSHIP_EXPIRY)
			.filter_map(Result::ok)
		{
			if let Ok(expiry) = u64::decode(&mut value.as_slice()) {
				if expiry <= current_block {
					let hash = BlobHash::from_slice(&key);
					expired_ownerships.push(hash);
				}
			} else {
				log::warn!(target: LOG_TARGET, "[Double] Failed to decode blob ownership expiry for key {:?}", key);
			}
		}
		if !expired_ownerships.is_empty() {
			self.clean_expired_ownerships_without_metadata(&expired_ownerships)?;
		}

		Ok((expired_blobs, expired_ownerships))
	}

	fn log_all_entries(&self) -> Result<()> {
		log::info!(target: LOG_TARGET, "--- [Double] Logging all entries ---");

		// Blob Metadatas (meta db)
		log::info!(target: LOG_TARGET, "--- [Double] Blob Metadatas ---");
		for (_key, value) in self
			.db_meta
			.iter(Self::COL_BLOB_METADATA)
			.filter_map(Result::ok)
		{
			if let Ok(blob_metadata) = BlobMetadata::decode(&mut value.as_slice()) {
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
			} else {
				log::warn!(target: LOG_TARGET, "[Double] Failed to decode a blob metadata entry");
			}
		}

		// Blob Ownerships (meta db)
		log::info!(target: LOG_TARGET, "--- [Double] Blob Ownerships ---");
		for (key, value) in self
			.db_meta
			.iter(Self::COL_BLOB_OWNERSHIP)
			.filter_map(Result::ok)
		{
			if let Ok(o) = OwnershipEntry::decode(&mut value.as_slice()) {
				const BLOB_HASH_LEN: usize = 32;
				if key.len() >= BLOB_HASH_LEN {
					let hash = BlobHash::from_slice(&key[..BLOB_HASH_LEN]);
					log::info!(target: LOG_TARGET, "Blob Ownership: hash={:?}, ownership={:?}", hash, o.address);
				}
			} else {
				log::warn!(target: LOG_TARGET, "[Double] Failed to decode an ownership entry");
			}
		}

		// Blob Retries (meta db)
		log::info!(target: LOG_TARGET, "--- [Double] Blob Retries ---");
		for (key, value) in self
			.db_meta
			.iter(Self::COL_BLOB_RETRY)
			.filter_map(Result::ok)
		{
			if let Ok(count) = u16::decode(&mut value.as_slice()) {
				let hash = BlobHash::from_slice(&key);
				log::info!(target: LOG_TARGET, "Blob Retry: hash={:?}, count={}", hash, count);
			} else {
				log::warn!(target: LOG_TARGET, "[Double] Failed to decode a retry entry");
			}
		}

		// Raw Blobs (blob db) — list presence by key
		log::info!(target: LOG_TARGET, "--- [Double] Blob Keys ---");
		for (key, _value) in self.db_blob.iter(Self::COL_BLOB).filter_map(Result::ok) {
			let hash = BlobHash::from_slice(&key);
			log::info!(target: LOG_TARGET, "Blob present: hash={:?}", hash);
		}

		log::info!(target: LOG_TARGET, "--- [Double] End of blob store log ---");
		Ok(())
	}
}
