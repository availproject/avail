#![cfg_attr(not(feature = "std"), no_std)]
#![warn(unused_extern_crates)]

pub mod mem_tmp_storage;
pub use mem_tmp_storage::{MemoryTemporaryStorage, StorageMap};

mod post_inherents;
pub use post_inherents::*;

pub mod header_extension;
pub use header_extension::{HeaderExtensionBuilderData, HeaderExtensionDataFilter};

pub mod testing_env;

/// Maximum number of blob transactions allowed in a single authored block
pub const MAX_BLOB_TXS_PER_BLOCK: usize = 128;
