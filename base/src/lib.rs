#![cfg_attr(not(feature = "std"), no_std)]
#![warn(unused_extern_crates)]

#[cfg(feature = "std")]
pub mod metrics;

pub mod mem_tmp_storage;
pub use mem_tmp_storage::{
	mts_clear, mts_get, mts_insert, mts_remove, mts_storage, mts_take, mts_update, StorageMap,
};

mod post_inherents;
pub use post_inherents::*;
