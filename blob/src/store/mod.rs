use crate::types::{Blob, BlobHash, BlobInfo, BlobMetadata, CompressedBlob, OwnershipEntry};
use anyhow::Result;

mod doublerocksdb;
mod rocksdb;

pub use doublerocksdb::DoubleRocksdbBlobStore;
pub use rocksdb::RocksdbBlobStore;

/// Blob Storage interface
pub trait StorageApiT: Send + Sync {
	// Blob metadata
	fn insert_blob_metadata(&self, blob_metadata: &BlobMetadata) -> Result<()>;
	fn get_blob_metadata(&self, hash: &BlobHash) -> Result<Option<BlobMetadata>>;
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

	// Blob info is a lightweight Blob indexer, which keeps track of blobs included in blocks, Wont store blobs themselves
	fn insert_blob_info(&self, blob_info: BlobInfo) -> Result<()>;
	fn get_blob_info(&self, hash: &BlobHash) -> Result<Option<BlobInfo>>;

	// Testing / diagnostics
	fn log_all_entries(&self) -> Result<()>;
}

#[inline]
pub(crate) fn blob_meta_key(hash: &BlobHash) -> Vec<u8> {
	hash.0.to_vec()
}

#[inline]
pub(crate) fn blob_count_key(hash: &BlobHash) -> Vec<u8> {
	hash.0.to_vec()
}

#[inline]
pub(crate) fn blob_key(hash: &BlobHash) -> Vec<u8> {
	hash.0.to_vec()
}

#[inline]
pub(crate) fn blob_ownership_key_prefix(hash: &BlobHash) -> Vec<u8> {
	hash.0.to_vec()
}

#[inline]
pub(crate) fn blob_ownership_expiry_key(hash: &BlobHash) -> Vec<u8> {
	hash.0.to_vec()
}

#[inline]
pub(crate) fn blob_ownership_key(hash: &BlobHash, owner: &Vec<u8>) -> Vec<u8> {
	let prefix = blob_ownership_key_prefix(hash);
	let mut k = Vec::with_capacity(prefix.len() + owner.len());
	k.extend_from_slice(&prefix);
	k.extend_from_slice(owner);
	k
}
