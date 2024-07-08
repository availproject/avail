/*
use crate::config::AccountId;
use subxt::utils::FromSs58Error;

fn from_ss58check(s: &str) -> Result<AccountId, FromSs58Error> {
	const CHECKSUM_LEN: usize = 2;
	let body_len = 32;

	use base58::FromBase58;
	let data = s.from_base58().map_err(|_| FromSs58Error::BadBase58)?;
	if data.len() < 2 {
		return Err(FromSs58Error::BadLength);
	}
	let prefix_len = match data[0] {
		0..=63 => 1,
		64..=127 => 2,
		_ => return Err(FromSs58Error::InvalidPrefix),
	};
	if data.len() != prefix_len + body_len + CHECKSUM_LEN {
		return Err(FromSs58Error::BadLength);
	}
	let hash = ss58hash(&data[0..body_len + prefix_len]);
	let checksum = &hash[0..CHECKSUM_LEN];
	if data[body_len + prefix_len..body_len + prefix_len + CHECKSUM_LEN] != *checksum {
		// Invalid checksum.
		return Err(FromSs58Error::InvalidChecksum);
	}

	let result = data[prefix_len..body_len + prefix_len]
		.try_into()
		.map_err(|_| FromSs58Error::BadLength)?;
	Ok(AccountId(result))
}
*/
