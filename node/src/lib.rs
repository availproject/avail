pub mod chains;

pub mod da_block_import;
pub use da_block_import::BlockImport;

pub mod cli;
pub mod rpc;
pub mod service;

pub const NODE_VERSION: &str = "2.2.1";
