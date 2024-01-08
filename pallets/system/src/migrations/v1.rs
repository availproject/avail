use codec::Decode;
use frame_support::weights::Weight;
use sp_core::Get;

/// # V1 Migrations
/// - `BlockLength` migration to `DynamicBlockLength`.
/// - `AllExtrinsicLen` from single `u32` into `ExtrinsicLen` type.
use crate::{limits::BlockLength, AllExtrinsicsLen, Config, DynamicBlockLength, ExtrinsicLen};

pub const BLOCK_LENGTH: &[u8] = b":block_length:";

pub fn migrate<T: Config>() -> Weight {
	let mut weight = Weight::zero();

	// 1. Raw storage ":block_length:" into `System::DynamicBlockLength`.
	let encoded_block_len = sp_io::storage::get(BLOCK_LENGTH).unwrap_or_default();
	let block_len = BlockLength::decode(&mut &encoded_block_len[..]).unwrap_or_default();
	DynamicBlockLength::<T>::put(block_len);
	weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

	// 2. Storage `AllExtrinsicsLen` from `u32` to `ExtrinsicLen`.
	// As it is called before `on_initialize`, it should be 0.
	let _ = <AllExtrinsicsLen<T>>::translate(|maybe_len: Option<u32>| -> Option<ExtrinsicLen> {
		maybe_len.map(|_| ExtrinsicLen::default())
	});
	weight.saturating_add(T::DbWeight::get().reads_writes(1, 1))
}
