// This file is part of Substrate.
// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
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

use da_runtime::Block;
use sc_cli::{ChainSpec, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;

use crate::{
	chain_spec,
	cli::{Cli, Subcommand},
	service::{self, ExecutorDispatch},
};

impl SubstrateCli for Cli {
	fn impl_name() -> String { "Polygon Avail Node".into() }

	fn impl_version() -> String { env!("SUBSTRATE_CLI_IMPL_VERSION").into() }

	fn description() -> String { env!("CARGO_PKG_DESCRIPTION").into() }

	fn author() -> String { env!("CARGO_PKG_AUTHORS").into() }

	fn support_url() -> String { "support.anonymous.an".into() }

	fn copyright_start_year() -> i32 { 2017 }

	fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
		let spec = match id {
			"" => {
				return Err(
					"Please specify which chain you want to run, e.g. --dev or --chain=local"
						.into(),
				)
			},
			"dev" => Box::new(chain_spec::development_config()),
			"testnet" => Box::new(chain_spec::testnet_config()),
			//"staging" => Box::new(chain_spec::public_testnet_config()),
			path => Box::new(chain_spec::ChainSpec::from_json_file(
				std::path::PathBuf::from(path),
			)?),
		};
		Ok(spec)
	}

	fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		&da_runtime::VERSION
	}
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		None => {
			let runner = cli.create_runner(&cli.run)?;
			runner.run_node_until_exit(|config| async move {
				service::new_full(config).map_err(sc_cli::Error::Service)
			})
		},
		/*Some(Subcommand::Inspect(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| cmd.run::<Block, RuntimeApi, ExecutorDispatch>(config))
		},*/
		Some(Subcommand::Benchmark(cmd)) => {
			if cfg!(feature = "runtime-benchmarks") {
				let runner = cli.create_runner(cmd)?;

				runner.sync_run(|config| cmd.run::<Block, ExecutorDispatch>(config))
			} else {
				Err("Benchmarking wasn't enabled when building the node. \
				You can enable it with `--features runtime-benchmarks`."
					.into())
			}
		},

		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		// Some(Subcommand::Sign(cmd)) => cmd.run(),
		// Some(Subcommand::Verify(cmd)) => cmd.run(),
		// Some(Subcommand::Vanity(cmd)) => cmd.run(),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client,
					task_manager,
					import_queue,
					..
				} = service::new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client,
					task_manager,
					..
				} = service::new_partial(&config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client,
					task_manager,
					..
				} = service::new_partial(&config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client,
					task_manager,
					import_queue,
					..
				} = service::new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client,
					task_manager,
					backend,
					..
				} = service::new_partial(&config)?;
				Ok((cmd.run(client, backend), task_manager))
			})
		},
	}
}
