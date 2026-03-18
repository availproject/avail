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

use sc_executor::sp_wasm_interface::HostFunctions;
use sp_core::H256;
use sp_runtime::{
    traits::{Block as BlockT, NumberFor},
    DeserializeOwned,
};

use crate::common::shared_parameters::SharedParams;

pub mod create_snapshot;
pub mod execute_block;
pub mod fast_forward;
pub mod follow_chain;
pub mod offchain_worker;
pub mod on_runtime_upgrade;

/// Ready to use, vanilla command combining common actions.
#[derive(Debug, Clone, clap::Parser)]
#[command(author, version, about)]
pub struct TryRuntime {
    #[clap(flatten)]
    pub shared: SharedParams,

    #[command(subcommand)]
    pub action: Action,
}

impl TryRuntime {
    pub async fn run<Block, HostFns>(&self) -> sc_cli::Result<()>
    where
        Block: BlockT<Hash = H256> + DeserializeOwned,
        Block::Header: DeserializeOwned,
        Block::Hash: FromStr,
        <Block::Hash as FromStr>::Err: Debug,
        <NumberFor<Block> as FromStr>::Err: Debug,
        <NumberFor<Block> as TryInto<u64>>::Error: Debug,
        NumberFor<Block>: FromStr,
        HostFns: HostFunctions,
    {
        self.action.run::<Block, HostFns>(&self.shared).await
    }
}

/// Possible actions of `try-runtime`.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum Action {
    /// Execute the migrations of the given runtime
    ///
    /// This uses a custom runtime api call, namely "TryRuntime_on_runtime_upgrade". The code path
    /// only triggers all of the `on_runtime_upgrade` hooks in the runtime, and optionally
    /// `try_state`.
    ///
    /// See [`TryRuntime`] and [`on_runtime_upgrade::Command`] for more information.
    OnRuntimeUpgrade(on_runtime_upgrade::Command),

    /// Executes the given block against some state.
    ///
    /// This uses a custom runtime api call, namely "TryRuntime_execute_block". Some checks, such
    /// as state-root and signature checks are always disabled, and additional checks like
    /// `try-state` can be enabled.
    ///
    /// See [`TryRuntime`] and [`execute_block::Command`] for more information.
    ExecuteBlock(execute_block::Command),

    /// Executes *the offchain worker hooks* of a given block against some state.
    ///
    /// This executes the same runtime api as normal block import, namely
    /// `OffchainWorkerApi_offchain_worker`.
    ///
    /// See [`frame_try_runtime::TryRuntime`] and [`offchain_worker::Command`]
    /// for more information.
    OffchainWorker(offchain_worker::Command),

    /// Follow the given chain's finalized blocks and apply all of its extrinsics.
    ///
    /// This is essentially repeated calls to [`Action::ExecuteBlock`].
    ///
    /// This allows the behavior of a new runtime to be inspected over a long period of time, with
    /// realistic transactions coming as input.
    ///
    /// NOTE: this does NOT execute the offchain worker hooks of mirrored blocks. This might be
    /// added in the future.
    ///
    /// This does not support snapshot states, and can only work with a remote chain. Upon first
    /// connections, starts listening for finalized block events. Upon first block notification, it
    /// initializes the state from the remote node, and starts applying that block, plus all the
    /// blocks that follow, to the same growing state.
    ///
    /// This can only work if the block format between the remote chain and the new runtime being
    /// tested has remained the same, otherwise block decoding might fail.
    FollowChain(follow_chain::Command),

    /// Create snapshot files.
    ///
    /// The `create-snapshot` subcommand facilitates the creation of a snapshot from a node's
    /// state. This snapshot can be loaded rapidly into memory from disk, providing an
    /// efficient alternative to downloading state from the node for every new command
    /// execution.
    ///
    /// **Usage**:
    ///
    /// 1. Create a snapshot from a remote node:
    ///
    /// try-runtime create-snapshot --uri ws://remote-node-uri my_state.snap
    ///
    /// 2. Utilize the snapshot with `on-runtime-upgrade`:
    ///
    /// try-runtime --runtime ./path/to/runtime.wasm on-runtime-upgrade snap --path my_state.snap
    CreateSnapshot(create_snapshot::Command),

    /// Executes a runtime upgrade (optional), then mines a number of blocks while performing
    /// try-state checks.
    ///
    /// The try-state checks are performed using the `TryRuntime_execute_block` runtime api.
    ///
    /// See [`TryRuntime`] and [`fast_forward::Command`] for more information.
    FastForward(fast_forward::Command),
}

impl Action {
    pub async fn run<Block, HostFns>(&self, shared: &SharedParams) -> sc_cli::Result<()>
    where
        Block: BlockT<Hash = H256> + DeserializeOwned,
        Block::Header: DeserializeOwned,
        Block::Hash: FromStr,
        <Block::Hash as FromStr>::Err: Debug,
        <NumberFor<Block> as FromStr>::Err: Debug,
        <NumberFor<Block> as TryInto<u64>>::Error: Debug,
        NumberFor<Block>: FromStr,
        HostFns: HostFunctions,
    {
        match &self {
            Action::OnRuntimeUpgrade(ref cmd) => {
                on_runtime_upgrade::CheckOnRuntimeUpgrade::<Block, HostFns> {
                    shared: shared.clone(),
                    command: cmd.clone(),
                    _phantom: Default::default(),
                }
                .run()
                .await
            }
            Action::ExecuteBlock(cmd) => {
                execute_block::run::<Block, HostFns>(shared.clone(), cmd.clone()).await
            }
            Action::OffchainWorker(cmd) => {
                offchain_worker::run::<Block, HostFns>(shared.clone(), cmd.clone()).await
            }
            Action::FollowChain(cmd) => {
                follow_chain::run::<Block, HostFns>(shared.clone(), cmd.clone()).await
            }
            Action::CreateSnapshot(cmd) => {
                create_snapshot::run::<Block, HostFns>(shared.clone(), cmd.clone()).await
            }
            Action::FastForward(cmd) => {
                fast_forward::run::<Block, HostFns>(shared.clone(), cmd.clone()).await
            }
        }
    }
}
