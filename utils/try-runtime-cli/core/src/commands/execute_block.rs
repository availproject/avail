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
use sp_runtime::{
    generic::SignedBlock,
    traits::{Block as BlockT, Header as HeaderT, NumberFor},
};
use substrate_rpc_client::{ws_client, ChainApi};

use crate::{
    common::state::{
        build_executor, state_machine_call_with_proof, LiveState, RuntimeChecks, State,
    },
    full_extensions, rpc_err_handler, SharedParams, LOG_TARGET,
};

/// Configurations for [`run`].
///
/// This will always call into `TryRuntime_execute_block`, which can optionally skip the state-root
/// check (useful for trying a unreleased runtime), and can execute runtime sanity checks as well.
#[derive(Debug, Clone, clap::Parser)]
pub struct Command {
    /// Which try-state targets to execute when running this command.
    ///
    /// Expected values:
    /// - `all`
    /// - `none`
    /// - A comma separated list of pallets, as per pallet names in `construct_runtime!()` (e.g.
    ///   `Staking, System`).
    /// - `rr-[x]` where `[x]` is a number. Then, the given number of pallets are checked in a
    ///   round-robin fashion.
    #[arg(long, default_value = "all")]
    pub try_state: frame_try_runtime::TryStateSelect,

    /// The ws uri from which to fetch the block.
    ///
    /// This will always fetch the next block of whatever `state` is referring to, because this is
    /// the only sensible combination. In other words, if you have the state of block `n`, you
    /// should execute block `n+1` on top of it.
    ///
    /// If `state` is `Live`, this can be ignored and the same uri is used for both.
    #[arg(
		long,
		value_parser = crate::common::parse::url
	)]
    pub block_ws_uri: Option<String>,

    /// The state type to use.
    #[command(subcommand)]
    pub state: State,
}

impl Command {
    fn block_ws_uri(&self) -> String {
        match (&self.block_ws_uri, &self.state) {
            (Some(block_ws_uri), State::Snap { .. }) => block_ws_uri.to_owned(),
            (Some(block_ws_uri), State::Live { .. }) => {
                log::error!(target: LOG_TARGET, "--block-uri is provided while state type is live, Are you sure you know what you are doing?");
                block_ws_uri.to_owned()
            }
            (None, State::Live(LiveState { uri, .. })) => uri[0].clone(),
            (None, State::Snap { .. }) => {
                panic!("either `--block-uri` must be provided, or state must be `live`");
            }
        }
    }
}

// Runs the `execute_block` command.
pub async fn run<Block, HostFns>(shared: SharedParams, command: Command) -> sc_cli::Result<()>
where
    Block: BlockT + serde::de::DeserializeOwned,
    <Block::Hash as FromStr>::Err: Debug,
    Block::Hash: serde::de::DeserializeOwned,
    Block::Header: serde::de::DeserializeOwned,
    <NumberFor<Block> as TryInto<u64>>::Error: Debug,
    HostFns: HostFunctions,
{
    let executor = build_executor::<HostFns>(&shared);
    let block_ws_uri = command.block_ws_uri();
    let rpc = ws_client(&block_ws_uri).await?;

    let live_state = match command.state {
        State::Live(live_state) => {
            // If no --at is provided, get the latest block to replay
            if live_state.at.is_some() {
                live_state
            } else {
                let header =
                    ChainApi::<(), Block::Hash, Block::Header, SignedBlock<Block>>::header(
                        &rpc, None,
                    )
                    .await
                    .map_err(rpc_err_handler)?
                    .expect("header exists, block should also exist; qed");
                LiveState {
                    uri: vec![block_ws_uri],
                    at: Some(hex::encode(header.hash().encode())),
                    pallet: Default::default(),
                    hashed_prefixes: Default::default(),
                    child_tree: Default::default(),
                }
            }
        }
        _ => {
            unreachable!("execute block currently only supports Live state")
        }
    };

    // The block we want to *execute* at is the block passed by the user
    let execute_at = live_state.at::<Block>()?;

    let prev_block_live_state = live_state.to_prev_block_live_state::<Block>().await?;

    // Get state for the prev block
    let runtime_checks = RuntimeChecks {
        name_matches: !shared.disable_spec_name_check,
        version_increases: false,
        try_runtime_feature_enabled: true,
    };
    let ext = State::Live(prev_block_live_state)
        .to_ext::<Block, HostFns>(&shared, &executor, None, runtime_checks)
        .await?;

    // Execute the desired block on top of it
    let block =
        ChainApi::<(), Block::Hash, Block::Header, SignedBlock<Block>>::block(&rpc, execute_at)
            .await
            .map_err(rpc_err_handler)?
            .expect("header exists, block should also exist; qed")
            .block;

    // A digest item gets added when the runtime is processing the block, so we need to pop
    // the last one to be consistent with what a gossiped block would contain.
    let (mut header, extrinsics) = block.deconstruct();
    header.digest_mut().pop();
    let block = Block::new(header, extrinsics);

    // for now, hardcoded for the sake of simplicity. We might customize them one day.
    let state_root_check = false;
    let signature_check = false;
    let payload = (
        block.clone(),
        state_root_check,
        signature_check,
        command.try_state,
    )
        .encode();

    let _ = state_machine_call_with_proof::<Block, HostFns>(
        &ext,
        &mut Default::default(),
        &executor,
        "TryRuntime_execute_block",
        &payload,
        full_extensions(executor.clone()),
        shared.export_proof,
    )?;

    Ok(())
}
