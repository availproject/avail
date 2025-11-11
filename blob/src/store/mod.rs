use crate::types::{
	Blob, BlobHash, BlobInfo, BlobMetadata, BlockHash, CompressedBlob, OwnershipEntry,
};
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

	/// Inserts a single BlobInfo entry for a specific (blob_hash, block_hash) pair.
	fn insert_blob_info_by_block(&self, blob_info: &BlobInfo) -> Result<()>;

	/// Inserts multiple BlobInfo entries (batch version of `insert_blob_info_by_block`).
	fn insert_blob_infos_by_block_batch(&self, items: &[BlobInfo]) -> Result<()>;

	/// Fetches a BlobInfo for a given blob_hash and block_hash pair.
	fn get_blob_info_by_block(
		&self,
		blob_hash: &BlobHash,
		block_hash: &BlockHash,
	) -> Result<Option<BlobInfo>>;

	/// Lists all BlobInfo entries across blocks for a given blob_hash.
	fn list_blob_infos_by_hash(&self, blob_hash: &BlobHash) -> Result<Vec<BlobInfo>>;

	/// Lists all BlobInfo entries included in a specific block.
	fn list_blob_infos_by_block(&self, block_hash: &BlockHash) -> Result<Vec<BlobInfo>>;

	/// Appends a single BlobInfo entry to the pending set for a given block.
	fn append_pending_blob_info(&self, block_hash: &BlockHash, blob_info: &BlobInfo) -> Result<()>;

	/// Appends multiple BlobInfo entries to the pending set for a given block (batch insert).
	fn append_pending_blob_infos_batch(
		&self,
		block_hash: &BlockHash,
		items: &[BlobInfo],
	) -> Result<()>;

	/// Takes and removes all pending BlobInfo entries for a given block (used after finalization).
	fn take_pending_blob_infos(&self, block_hash: &BlockHash) -> Result<Vec<BlobInfo>>;

	/// Returns the list of blocks that currently have pending BlobInfo entries.
	fn get_pending_block_hashes(&self) -> Result<Vec<BlockHash>>;

	/// Inserts a canonical BlobInfo (finalized block only, replaces older data if needed).
	///
	/// Used by the finality promoter to record finalized blob-block mappings.
	fn insert_blob_info(&self, blob_info: BlobInfo) -> Result<()>;

	/// Fetches a canonical BlobInfo for a given blob hash (from finalized block only).
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

#[inline]
pub(crate) fn blob_by_hash_block_key(hash: &BlobHash, block_hash: &BlockHash) -> Vec<u8> {
	let mut k = Vec::with_capacity(64);
	k.extend_from_slice(&hash.0);
	k.extend_from_slice(&block_hash.0);
	k
}

#[inline]
pub(crate) fn blob_by_block_key(block_hash: &BlockHash, hash: &BlobHash) -> Vec<u8> {
	let mut k = Vec::with_capacity(64);
	k.extend_from_slice(&block_hash.0);
	k.extend_from_slice(&hash.0);
	k
}

#[inline]
pub(crate) fn block_key(hash: &BlockHash) -> Vec<u8> {
	hash.0.to_vec()
}
