// This file is part of try-runtime-cli.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{fmt::Debug, str::FromStr};

use parity_scale_codec::Encode;
use sc_executor::sp_wasm_interface::HostFunctions;
use sp_runtime::traits::{Block as BlockT, NumberFor};
use substrate_rpc_client::{ws_client, ChainApi};

use crate::{
    common::{
        parse,
        state::{build_executor, state_machine_call, LiveState, RuntimeChecks, State},
    },
    full_extensions, rpc_err_handler, SharedParams, LOG_TARGET,
};

/// Configuration for [`run`].
#[derive(Debug, Clone, clap::Parser)]
pub struct Command {
    /// The ws uri from which to fetch the header.
    ///
    /// If the `live` state type is being used, then this can be omitted, and is equal to whatever
    /// the `state::uri` is. Only use this (with care) when combined with a snapshot.
    #[arg(
		long,
		value_parser = parse::url
	)]
    pub header_ws_uri: Option<String>,

    /// The state type to use.
    #[command(subcommand)]
    pub state: State,
}

impl Command {
    fn header_ws_uri(&self) -> String {
        match (&self.header_ws_uri, &self.state) {
            (Some(header_ws_uri), State::Snap { .. }) => header_ws_uri.to_owned(),
            (Some(header_ws_uri), State::Live { .. }) => {
                log::error!(target: LOG_TARGET, "--header-uri is provided while state type is live, this will most likely lead to a nonsensical result.");
                header_ws_uri.to_owned()
            }
            (None, State::Live(LiveState { uri, .. })) => uri[0].clone(),
            (None, State::Snap { .. }) => {
                panic!("either `--header-uri` must be provided, or state must be `live`");
            }
        }
    }
}

// Runs the `offchain_worker` command.
pub async fn run<Block, HostFns>(shared: SharedParams, command: Command) -> sc_cli::Result<()>
where
    Block: BlockT + serde::de::DeserializeOwned,
    Block::Header: serde::de::DeserializeOwned,
    <Block::Hash as FromStr>::Err: Debug,
    NumberFor<Block>: FromStr,
    <NumberFor<Block> as FromStr>::Err: Debug,
    HostFns: HostFunctions,
{
    let executor = build_executor(&shared);
    let block_ws_uri = command.header_ws_uri();
    let rpc = ws_client(&block_ws_uri).await?;

    let live_state = match command.state {
        State::Live(live_state) => live_state,
        _ => {
            unreachable!("execute block currently only supports Live state")
        }
    };

    // The block we want to *execute* at is the block passed by the user
    let execute_at = live_state.at::<Block>()?;

    // Get state for the prev block
    let prev_block_live_state = live_state.to_prev_block_live_state::<Block>().await?;
    let runtime_checks = RuntimeChecks {
        name_matches: !shared.disable_spec_name_check,
        version_increases: false,
        try_runtime_feature_enabled: true,
    };
    let ext = State::Live(prev_block_live_state)
        .to_ext::<Block, HostFns>(&shared, &executor, None, runtime_checks)
        .await?;

    let header = ChainApi::<(), Block::Hash, Block::Header, ()>::header(&rpc, execute_at)
        .await
        .map_err(rpc_err_handler)
        .map(|maybe_header| maybe_header.ok_or("Header does not exist"))??;
    let payload = header.encode();

    let _ = state_machine_call::<Block, HostFns>(
        &ext,
        &executor,
        "OffchainWorkerApi_offchain_worker",
        &payload,
        full_extensions(executor.clone()),
    )?;

    log::info!(target: LOG_TARGET, "finished execution");
    Ok(())
}
