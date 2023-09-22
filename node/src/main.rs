//! Substrate Node Template CLI library.
#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod benchmarking;
#[macro_use]
mod service;
mod cli;
mod command;
mod da_block_import;
mod rpc;

fn main() -> sc_cli::Result<()> {
	command::run()
}
