use anyhow::anyhow;
use sp_core::H256;
use sp_runtime::{AccountId32, SaturatedConversion};
use sp_std::vec::Vec;

/// Return the list of storing validators for a blob
pub fn validators_for_blob(
	blob_hash: H256,
	validators: &Vec<AccountId32>,
	block_hash_bytes: &[u8],
	nb_validators_per_blob: u32,
) -> anyhow::Result<Vec<AccountId32>> {
	let n = validators.len() as u32;
	if n == 0 || nb_validators_per_blob == 0 {
		return Ok(Vec::new());
	}
	let base = generate_base_index(blob_hash, block_hash_bytes, n as usize, None)? as u32;
	let k = nb_validators_per_blob.min(n);
	Ok((0..k)
		.map(|i| validators[((base + i) % n) as usize].clone())
		.collect())
}

/// Generate pseudo deterministic index based on given values
pub fn generate_base_index(
	blob_hash: H256,
	block_hash_bytes: &[u8],
	ring_size: usize,
	additional: Option<Vec<u8>>,
) -> anyhow::Result<usize> {
	let ring_size: u64 = ring_size.saturated_into();

	let hash_bytes = blob_hash.as_bytes();
	let truncated = hash_bytes
		.get(..8)
		.ok_or(anyhow!("Blob hash is too short, expected at least 8 bytes"))?;
	let array: [u8; 8] = truncated
		.try_into()
		.map_err(|_| anyhow!("Failed to convert first 8 bytes of blob hash"))?;
	let blob_seed = u64::from_le_bytes(array);
	let blob_index = blob_seed % ring_size;

	let truncated = block_hash_bytes.get(..8).ok_or(anyhow!(
		"Block hash is too short, expected at least 8 bytes"
	))?;
	let array: [u8; 8] = truncated
		.try_into()
		.map_err(|_| anyhow!("Failed to convert first 8 bytes of block hash"))?;
	let block_seed = u64::from_le_bytes(array);
	let block_index = block_seed % ring_size;

	let additional_index = match additional {
		Some(additional) => {
			let truncated = additional.get(..8).ok_or(anyhow!(
				"Additional hash is too short, expected at least 8 bytes"
			))?;
			let array: [u8; 8] = truncated
				.try_into()
				.map_err(|_| anyhow!("Failed to convert first 8 bytes of blob hash"))?;
			let additional_seed = u64::from_le_bytes(array);
			let additional_index = additional_seed % ring_size;
			additional_index
		},
		None => 0,
	};

	let index = blob_index
		.wrapping_add(block_index)
		.wrapping_add(additional_index)
		% ring_size;

	Ok(index as usize)
}
