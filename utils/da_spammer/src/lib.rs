use avail_rust::H256;
use da_commitment::build_da_commitments::build_da_commitments;
use sp_crypto_hashing::keccak_256;

/// Default grid (tune to your runtime)
pub const DEFAULT_ROWS: usize = 1024;
pub const DEFAULT_COLS: usize = 4096;

/// Build a blob filled with `byte` (length `len_bytes`), its keccak256 hash,
/// and DA commitments using KZG (rows/cols + Seed::default()).
pub fn build_blob_and_commitments(byte: u8, len_bytes: usize) -> (Vec<u8>, H256, Vec<u8>) {
	let blob = vec![byte; len_bytes];
	let blob_hash = H256::from(keccak_256(&blob));
	let commitments =
		build_da_commitments(blob.clone(), DEFAULT_ROWS, DEFAULT_COLS, Default::default());
	(blob, blob_hash, commitments)
}
