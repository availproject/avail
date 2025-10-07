use crate::traits::NonceCacheApiT;
use sp_runtime::AccountId32;
use std::sync::Mutex;
use ttl_cache::TtlCache;

use crate::{BLOB_FUTURE_ACCOUNTS_IN_CACHE, BLOB_FUTURE_NONCE_CACHE_TTL, BLOB_FUTURE_NONCE_DEPTH};

pub struct NonceCache {
	inner: Mutex<TtlCache<AccountId32, u32>>,
}

impl NonceCache {
	pub fn new() -> Self {
		Self {
			inner: Mutex::new(TtlCache::new(BLOB_FUTURE_ACCOUNTS_IN_CACHE as usize)),
		}
	}
}

impl NonceCacheApiT for NonceCache {
	fn check(&self, who: &AccountId32, onchain_nonce: u32, tx_nonce: u32) -> Result<(), String> {
		let max_allowed = onchain_nonce.saturating_add(BLOB_FUTURE_NONCE_DEPTH);
		if tx_nonce > max_allowed {
			return Err(format!(
				"nonce too far in the future (tx={}, onchain={}, depth={})",
				tx_nonce, onchain_nonce, BLOB_FUTURE_NONCE_DEPTH
			));
		}

		let cache = self.inner.lock().unwrap();
		let accept = match cache.get(who) {
			None => tx_nonce == onchain_nonce,
			Some(last) => tx_nonce == last.saturating_add(1),
		};

		if accept {
			Ok(())
		} else {
			Err(format!(
				"unexpected nonce (tx={}, onchain={}, cached={:?})",
				tx_nonce,
				onchain_nonce,
				cache.get(who).copied()
			))
		}
	}

	fn commit(&self, who: &AccountId32, tx_nonce: u32) {
		match self.inner.lock() {
			Ok(mut cache) => {
				cache.insert(who.clone(), tx_nonce, BLOB_FUTURE_NONCE_CACHE_TTL);
			},
			Err(e) => {
				log::warn!(
					"NonceCache: failed to lock cache for account={} nonce={} (error={})",
					who,
					tx_nonce,
					e
				);
			},
		}
	}
}
