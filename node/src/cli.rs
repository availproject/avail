// This file is part of Substrate.

// Copyright (C) 2018-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

/// An overarching CLI command definition.
#[derive(Debug, clap::Parser)]
pub struct Cli {
	/// Possible subcommand with parameters.
	#[command(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[allow(missing_docs)]
	#[clap(flatten)]
	pub run: sc_cli::RunCmd,

	/// Disable automatic hardware benchmarks.
	///
	/// By default these benchmarks are automatically ran at startup and measure
	/// the CPU speed, the memory bandwidth and the disk speed.
	///
	/// The results are then printed out in the logs, and also sent as part of
	/// telemetry, if telemetry is enabled.
	#[arg(long)]
	pub no_hardware_benchmarks: bool,

	/// Disable checking commitment on imported block during sync
	#[arg(long, conflicts_with_all = &["validator"])]
	pub unsafe_da_sync: bool,

	/// Provides storage monitoring options on the node
	#[clap(flatten)]
	pub storage_monitor: sc_storage_monitor::StorageMonitorParams,

	/// Enable Kate RPC
	#[clap(long = "enable-kate-rpc", default_value_t = false)]
	pub kate_rpc_enabled: bool,

	/// Enable Kate RPC Metrics
	#[clap(long = "enable-kate-rpc-metrics", default_value_t = false)]
	pub kate_rpc_metrics_enabled: bool,

	/// The maximum number of cells that can be requested in one go.
	///
	/// Max size cannot exceed 10_000
	#[arg(long, default_value_t = 64, value_parser=kate_max_cells_size_upper_bound)]
	pub kate_max_cells_size: usize,

	/// The name of the network.
	///
	/// This parameter can be used to update the network name and id of the `dev` and `dev_tri` chains.
	#[arg(long)]
	pub network_name: Option<String>,

	/// Enable Transaction State RPC. This allows querying the transaction state (success or failure)
	/// using only a transaction hash.
	#[clap(long = "enable-tx-state-rpc", default_value_t = false)]
	pub tx_state_rpc_enabled: bool,

	/// The maximum number of results the transaction state RPC will return for a transaction hash.
	/// If a transaction hash appears in multiple blocks, the RPC will return only the top `X` transaction states.  
	/// In most cases, the transaction hash is unique, so this parameter is usually irrelevant.
	#[clap(long, default_value_t = 10)]
	pub tx_state_rpc_max_search_results: usize,

	/// The maximum number of blocks preserved and stored in the transaction state RPC database.
	///
	/// The default is 7 days' worth of blocks.
	#[clap(long, default_value_t = 30240, value_parser=more_or_euqal_than_10)]
	pub tx_state_rpc_max_stored_block_count: usize,

	/// Logging interval for transaction state, in milliseconds.
	/// A lower value results in more frequent log updates.
	///
	/// The default is 300_000 milliseconds (300 seconds).
	#[clap(long, default_value_t = 300_000)]
	pub tx_state_logging_interval: u64,

	/// If enabled, will use Vector instead of a HashMap for Transaction State RPC Database.
	/// This will decrease the memory footprint at the cost of lookup performance.
	#[clap(long, default_value_t = false)]
	pub tx_state_rpc_use_vector: bool,
}

fn more_or_euqal_than_10(s: &str) -> Result<usize, String> {
	let Ok(value) = s.parse::<usize>() else {
		return Err(String::from("is not a valid number"));
	};

	if value < 10 {
		return Err(String::from("cannot be less than 10"));
	}

	Ok(value)
}

fn kate_max_cells_size_upper_bound(s: &str) -> Result<usize, String> {
	clap_num::number_range(s, 0, 10_000)
}

/// Possible subcommands of the main binary.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
	/*
	/// The custom inspect subcommand for decoding blocks and extrinsics.
	#[command(
		name = "inspect",
		about = "Decode given block or extrinsic using current native runtime."
	)]
	Inspect(node_inspect::cli::InspectCmd),
	*/
	/// Sub-commands concerned with benchmarking.
	/// The pallet benchmarking moved to the `pallet` sub-command.
	#[command(subcommand)]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// Try some command against runtime state.
	#[cfg(feature = "try-runtime")]
	TryRuntime(try_runtime_cli::TryRuntimeCmd),

	/// Try some command against runtime state. Note: `try-runtime` feature must be enabled.
	#[cfg(not(feature = "try-runtime"))]
	TryRuntime,

	/// Key management cli utilities
	#[command(subcommand)]
	Key(sc_cli::KeySubcommand),

	/// Verify a signature for a message, provided on STDIN, with a given (public or secret) key.
	Verify(sc_cli::VerifyCmd),

	/// Generate a seed that provides a vanity address.
	Vanity(sc_cli::VanityCmd),

	/// Sign a message, with a given (secret) key.
	Sign(sc_cli::SignCmd),

	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(sc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// Db meta columns information.
	ChainInfo(sc_cli::ChainInfoCmd),
}
