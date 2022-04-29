use codec::Decode;
use frame_support::traits::{Get, StorageVersion};

use crate::{Account, AccountInfo, Config, Pallet, RefCount, UpgradedToTripleRefCount, Weight};

pub fn migrate<T: Config>() -> Weight {
	// Original pallet migrations
	let mut weight = if !UpgradedToTripleRefCount::<T>::get() {
		UpgradedToTripleRefCount::<T>::put(true);
		migrate_to_triple_ref_count::<T>()
	} else {
		0
	};

	// Polygon versions.
	let curr_version = StorageVersion::get::<Pallet<T>>();
	if curr_version < 1 {
		weight = weight.saturating_add(v1::migrate::<T>());
	}

	// Update the pallet version.
	StorageVersion::new(1).put::<Pallet<T>>();
	weight.saturating_add(T::DbWeight::get().reads_writes(0, 1))
}

#[allow(dead_code)]
/// Migrate from unique `u8` reference counting to triple `u32` reference counting.
pub fn migrate_all<T: Config>() -> frame_support::weights::Weight {
	Account::<T>::translate::<(T::Index, u8, T::AccountData), _>(|_key, (nonce, rc, data)| {
		Some(AccountInfo {
			nonce,
			consumers: rc as RefCount,
			providers: 1,
			sufficients: 0,
			data,
		})
	});
	T::BlockWeights::get().max_block
}

#[allow(dead_code)]
/// Migrate from unique `u32` reference counting to triple `u32` reference counting.
pub fn migrate_to_dual_ref_count<T: Config>() -> frame_support::weights::Weight {
	Account::<T>::translate::<(T::Index, RefCount, T::AccountData), _>(
		|_key, (nonce, consumers, data)| {
			Some(AccountInfo {
				nonce,
				consumers,
				providers: 1,
				sufficients: 0,
				data,
			})
		},
	);
	T::BlockWeights::get().max_block
}

/// Migrate from dual `u32` reference counting to triple `u32` reference counting.
pub fn migrate_to_triple_ref_count<T: Config>() -> frame_support::weights::Weight {
	Account::<T>::translate::<(T::Index, RefCount, RefCount, T::AccountData), _>(
		|_key, (nonce, consumers, providers, data)| {
			Some(AccountInfo {
				nonce,
				consumers,
				providers,
				sufficients: 0,
				data,
			})
		},
	);
	T::BlockWeights::get().max_block
}

/// V1 Migrations
///  -
mod v1 {
	use super::*;
	use crate::{limits::BlockLength, AllExtrinsicsLen, DynamicBlockLength, ExtrinsicLen};

	pub const BLOCK_LENGTH: &[u8] = b":block_length:";

	pub fn migrate<T: Config>() -> Weight {
		let mut weight: Weight = 0;

		// 1. Raw storage ":block_length:" into `System::DynamicBlockLength`.
		let encoded_block_len = sp_io::storage::get(BLOCK_LENGTH).unwrap_or_default();
		let block_len = BlockLength::decode(&mut &encoded_block_len[..]).unwrap_or_default();
		DynamicBlockLength::<T>::put(block_len);
		weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

		// 2. Storage `AllExtrinsicsLen` from `u32` to `ExtrinsicLen`.
		// As it is called before `on_initialize`, it should be 0.
		let _ =
			<AllExtrinsicsLen<T>>::translate(|maybe_len: Option<u32>| -> Option<ExtrinsicLen> {
				maybe_len.map(|_| ExtrinsicLen::default())
			});
		weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

		weight
	}
}
