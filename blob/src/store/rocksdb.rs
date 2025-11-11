use crate::{store::*, types::CompressedBlob};
use anyhow::{anyhow, Result};
use codec::{Decode, Encode};
use kvdb::{DBTransaction, KeyValueDB};
use kvdb_rocksdb::{Database, DatabaseConfig};
use std::{path::Path, sync::Mutex};
use tempfile::TempDir;
use ttl_cache::TtlCache;

use crate::{
	types::{Blob, BlobHash, BlobInfo, BlobMetadata, BlockHash, OwnershipEntry},
	BLOB_CACHE_DURATION, LOG_TARGET, MAX_BLOBS_IN_CACHE,
};

// First implementation, single rocks db
pub struct RocksdbBlobStore {
	db: Database,
	cache: Mutex<TtlCache<BlobHash, Vec<u8>>>,
}
impl RocksdbBlobStore {
	pub const COL_BLOB_METADATA: u32 = 0;
	pub const COL_BLOB_RETRY: u32 = 1;
	pub const COL_BLOB: u32 = 2;
	pub const COL_BLOB_OWNERSHIP: u32 = 3;
	pub const COL_BLOB_OWNERSHIP_EXPIRY: u32 = 4;
	// canonical blob info storage: BlobHash -> BlobInfo
	pub const COL_BLOB_INFO: u32 = 5;
	// temporary blob_info by blob_hash & block_hash: BlobHash || BlockHash -> BlobInfo
	pub const COL_BLOB_BY_HASH_BLOCK: u32 = 6;
	// temporary blobs info by block_hash & blob_hash : BlockHash || BlobHash -> BlobInfo
	pub const COL_BLOB_BY_BLOCK: u32 = 7;
	// pending blobs by block: BlockHash -> Vec<BlobInfo>
	pub const COL_BLOB_PENDING_BY_BLOCK: u32 = 8;

	pub fn open(path: impl AsRef<Path>) -> Result<Self> {
		let num_columns = 9;
		let db_config = DatabaseConfig::with_columns(num_columns);
		let db = Database::open(&db_config, path.as_ref())?;
		Ok(RocksdbBlobStore {
			db,
			cache: Mutex::new(TtlCache::new(MAX_BLOBS_IN_CACHE as usize)),
		})
	}
}
impl Default for RocksdbBlobStore {
	fn default() -> Self {
		let temp_dir = TempDir::new().expect("failed to create temp dir for RocksdbBlobStore");
		let db_path = temp_dir.path().join("blob_database");
		let store = Self::open(db_path).expect("opening RocksDB blob store failed");
		store
	}
}
impl StorageApiT for RocksdbBlobStore {
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
		self.db.write(tx)?;
		Ok(())
	}

	fn get_blob_metadata(&self, hash: &BlobHash) -> Result<Option<BlobMetadata>> {
		self.db
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
			.db
			.get(Self::COL_BLOB_METADATA, &blob_meta_key(hash))?
			.is_some())
	}

	fn insert_blob_retry(&self, hash: &BlobHash, count: u16) -> Result<()> {
		let mut tx = DBTransaction::new();
		tx.put(Self::COL_BLOB_RETRY, &blob_count_key(hash), &count.encode());
		self.db.write(tx)?;
		Ok(())
	}
	fn get_blob_retry(&self, hash: &BlobHash) -> Result<u16> {
		self.db
			.get(Self::COL_BLOB_RETRY, &blob_count_key(hash))?
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
		tx.put(Self::COL_BLOB, &blob_key(&blob_hash), &blob_encoded);
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

				let cached = match CompressedBlob::decode(cached) {
					Ok(x) => x,
					Err(_) => return Err(anyhow!("Failed to decode compressed data.")),
				};
				return Ok(Some(cached));
			}
		}

		// Fallback to db
		let data = self.db.get(Self::COL_BLOB, &blob_key(hash))?;

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

		let data = match CompressedBlob::decode(data) {
			Ok(x) => x,
			Err(_) => return Err(anyhow!("Failed to decode compressed data.")),
		};
		Ok(Some(data))
	}

	fn insert_blob_ownership(&self, hash: &BlobHash, o: &OwnershipEntry) -> Result<()> {
		let mut tx = DBTransaction::new();
		tx.put(
			Self::COL_BLOB_OWNERSHIP,
			&blob_ownership_key(hash, &o.address.encode()),
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
		let key = blob_ownership_key(hash, owner);
		self.db
			.get(Self::COL_BLOB_OWNERSHIP, &key)?
			.map(|bytes| {
				let mut slice = bytes.as_slice();
				OwnershipEntry::decode(&mut slice)
					.map_err(|_| anyhow!("failed to decode blob from the store"))
			})
			.transpose()
	}

	fn get_blob_ownerships(&self, hash: &BlobHash) -> Result<Vec<OwnershipEntry>> {
		let prefix = blob_ownership_key_prefix(hash);
		let iter = self.db.iter_with_prefix(Self::COL_BLOB_OWNERSHIP, &prefix);
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
			Self::COL_BLOB_OWNERSHIP_EXPIRY,
			&blob_ownership_expiry_key(hash),
			&expires_at.encode(),
		);
		self.db.write(tx)?;
		Ok(())
	}

	fn get_blob_ownership_expiry(&self, hash: &BlobHash) -> Result<Option<u64>> {
		self.db
			.get(
				Self::COL_BLOB_OWNERSHIP_EXPIRY,
				&blob_ownership_expiry_key(hash),
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
			Self::COL_BLOB_OWNERSHIP_EXPIRY,
			&blob_ownership_expiry_key(hash),
		);

		self.db.write(tx)?;
		Ok(())
	}

	fn clean_blobs_info(&self, hashes: &Vec<BlobHash>) -> Result<()> {
		let mut tx = DBTransaction::new();

		for hash in hashes {
			// remove blob metadata
			tx.delete(Self::COL_BLOB_METADATA, &blob_meta_key(hash));

			// remove blob retry
			tx.delete(Self::COL_BLOB_RETRY, &blob_count_key(hash));

			// remove blob
			tx.delete(Self::COL_BLOB, &blob_key(hash));

			// remove blob ownership
			tx.delete_prefix(Self::COL_BLOB_OWNERSHIP, &blob_ownership_key_prefix(hash));

			// remove blob ownership expiry
			tx.delete(
				Self::COL_BLOB_OWNERSHIP_EXPIRY,
				&blob_ownership_expiry_key(hash),
			);
		}

		self.db.write(tx)?;
		Ok(())
	}

	fn clean_expired_ownerships_without_metadata(&self, hashes: &Vec<BlobHash>) -> Result<()> {
		let mut tx = DBTransaction::new();

		for hash in hashes {
			// remove blob ownership
			tx.delete_prefix(Self::COL_BLOB_OWNERSHIP, &blob_ownership_key_prefix(hash));

			// remove blob ownership expiry
			tx.delete(
				Self::COL_BLOB_OWNERSHIP_EXPIRY,
				&blob_ownership_expiry_key(hash),
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
		for (key, value) in self.db.iter(Self::COL_BLOB_METADATA).filter_map(Result::ok) {
			if let Ok(blob_metadata) = BlobMetadata::decode(&mut value.as_slice()) {
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
			.iter(Self::COL_BLOB_OWNERSHIP_EXPIRY)
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
		for (_key, value) in self.db.iter(Self::COL_BLOB_METADATA).filter_map(Result::ok) {
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
			}
		}

		// Log Blob Retries
		log::info!(target: LOG_TARGET, "--- Blob Ownerships ---");
		for (key, value) in self
			.db
			.iter(Self::COL_BLOB_OWNERSHIP)
			.filter_map(Result::ok)
		{
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
		for (key, value) in self.db.iter(Self::COL_BLOB_RETRY).filter_map(Result::ok) {
			if let Ok(count) = u16::decode(&mut value.as_slice()) {
				let hash = BlobHash::from_slice(&key);
				log::info!(target: LOG_TARGET, "Blob Retry: hash={:?}, count={}", hash, count);
			}
		}

		log::info!(target: LOG_TARGET, "--- End of blob store log ---");
		Ok(())
	}

	fn insert_blob_info(&self, blob_info: BlobInfo) -> Result<()> {
		let mut tx = DBTransaction::new();
		tx.put(
			Self::COL_BLOB_INFO,
			&blob_key(&blob_info.hash),
			&blob_info.encode(),
		);
		self.db.write(tx)?;
		Ok(())
	}

	fn get_blob_info(&self, hash: &BlobHash) -> Result<Option<BlobInfo>> {
		let key = blob_key(hash);
		self.db
			.get(Self::COL_BLOB_INFO, &key)?
			.map(|bytes| {
				let mut slice = bytes.as_slice();
				BlobInfo::decode(&mut slice)
					.map_err(|_| anyhow!("failed to decode blob info from the store"))
			})
			.transpose()
	}

	fn insert_blob_info_by_block(&self, blob_info: &BlobInfo) -> Result<()> {
		let key1 = blob_by_hash_block_key(&blob_info.hash, &blob_info.block_hash);
		let key2 = blob_by_block_key(&blob_info.block_hash, &blob_info.hash);
		let value = blob_info.encode();

		let mut tx = DBTransaction::new();
		tx.put(Self::COL_BLOB_BY_HASH_BLOCK, &key1, &value);
		tx.put(Self::COL_BLOB_BY_BLOCK, &key2, &value);
		self.db.write(tx)?;
		Ok(())
	}

	fn insert_blob_infos_by_block_batch(&self, items: &[BlobInfo]) -> Result<()> {
		if items.is_empty() {
			return Ok(());
		}
		let mut tx = DBTransaction::new();
		for info in items.iter() {
			let key1 = blob_by_hash_block_key(&info.hash, &info.block_hash);
			let key2 = blob_by_block_key(&info.block_hash, &info.hash);
			let value = info.encode();
			tx.put(Self::COL_BLOB_BY_HASH_BLOCK, &key1, &value);
			tx.put(Self::COL_BLOB_BY_BLOCK, &key2, &value);
		}
		self.db.write(tx)?;
		Ok(())
	}

	fn get_blob_info_by_block(
		&self,
		blob_hash: &BlobHash,
		block_hash: &BlockHash,
	) -> Result<Option<BlobInfo>> {
		let key = blob_by_hash_block_key(blob_hash, block_hash);
		match self.db.get(Self::COL_BLOB_BY_HASH_BLOCK, &key)? {
			Some(bytes) => {
				let mut s = bytes.as_slice();
				let info =
					BlobInfo::decode(&mut s).map_err(|_| anyhow!("failed to decode BlobInfo"))?;
				Ok(Some(info))
			},
			None => Ok(None),
		}
	}

	fn list_blob_infos_by_hash(&self, blob_hash: &BlobHash) -> Result<Vec<BlobInfo>> {
		let prefix = blob_hash.0.to_vec();
		let iter = self
			.db
			.iter_with_prefix(Self::COL_BLOB_BY_HASH_BLOCK, &prefix);
		let mut out = Vec::new();
		for kv in iter {
			let (_k, v) = kv?;
			let mut s = v.as_slice();
			let info =
				BlobInfo::decode(&mut s).map_err(|_| anyhow!("failed to decode BlobInfo"))?;
			out.push(info);
		}
		Ok(out)
	}

	fn list_blob_infos_by_block(&self, block_hash: &BlockHash) -> Result<Vec<BlobInfo>> {
		let prefix = block_hash.0.to_vec();
		let iter = self.db.iter_with_prefix(Self::COL_BLOB_BY_BLOCK, &prefix);
		let mut out = Vec::new();
		for kv in iter {
			let (_k, v) = kv?;
			let mut s = v.as_slice();
			let info =
				BlobInfo::decode(&mut s).map_err(|_| anyhow!("failed to decode BlobInfo"))?;
			out.push(info);
		}
		Ok(out)
	}

	fn append_pending_blob_info(&self, block_hash: &BlockHash, blob_info: &BlobInfo) -> Result<()> {
		let key = block_key(block_hash);
		let existing = self.db.get(Self::COL_BLOB_PENDING_BY_BLOCK, &key)?;
		let mut vec: Vec<BlobInfo> = if let Some(bytes) = existing {
			let mut s = bytes.as_slice();
			Vec::<BlobInfo>::decode(&mut s)
				.map_err(|_| anyhow!("failed to decode pending BlobInfo vec"))?
		} else {
			Vec::new()
		};

		vec.push(blob_info.clone());
		let encoded = vec.encode();

		let mut tx = DBTransaction::new();
		tx.put(Self::COL_BLOB_PENDING_BY_BLOCK, &key, &encoded);
		self.db.write(tx)?;
		Ok(())
	}

	fn append_pending_blob_infos_batch(
		&self,
		block_hash: &BlockHash,
		items: &[BlobInfo],
	) -> Result<()> {
		let key = block_key(block_hash);
		// read existing
		let existing = self.db.get(Self::COL_BLOB_PENDING_BY_BLOCK, &key)?;
		let mut vec: Vec<BlobInfo> = if let Some(bytes) = existing {
			let mut s = bytes.as_slice();
			Vec::<BlobInfo>::decode(&mut s)
				.map_err(|_| anyhow!("failed to decode pending BlobInfo vec"))?
		} else {
			Vec::new()
		};

		vec.extend_from_slice(items);
		let encoded = vec.encode();

		let mut tx = DBTransaction::new();
		tx.put(Self::COL_BLOB_PENDING_BY_BLOCK, &key, &encoded);
		self.db.write(tx)?;
		Ok(())
	}

	fn take_pending_blob_infos(&self, block_hash: &BlockHash) -> Result<Vec<BlobInfo>> {
		let key = block_key(block_hash);
		let existing = self.db.get(Self::COL_BLOB_PENDING_BY_BLOCK, &key)?;
		let mut tx = DBTransaction::new();
		tx.delete(Self::COL_BLOB_PENDING_BY_BLOCK, &key);
		self.db.write(tx)?;
		if let Some(bytes) = existing {
			let mut s = bytes.as_slice();
			let vec = Vec::<BlobInfo>::decode(&mut s)
				.map_err(|_| anyhow!("failed to decode pending BlobInfo vec"))?;
			Ok(vec)
		} else {
			Ok(Vec::new())
		}
	}

	fn get_pending_block_hashes(&self) -> Result<Vec<BlockHash>> {
		let mut out = Vec::new();
		for kv in self
			.db
			.iter(Self::COL_BLOB_PENDING_BY_BLOCK)
			.filter_map(Result::ok)
		{
			let (k, _) = kv;
			if k.len() == 32 {
				out.push(BlockHash::from_slice(&k));
			}
		}
		Ok(out)
	}
}
