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
use sp_runtime::traits::{Block as BlockT, NumberFor};
use substrate_rpc_client::{ws_client, StateApi};

use crate::{
    common::{
        shared_parameters,
        state::{build_executor, LiveState, RuntimeChecks, State},
    },
    SharedParams, LOG_TARGET,
};

/// Configurations for [`run`].
#[derive(Debug, Clone, clap::Parser)]
pub struct Command {
    /// The source of the snapshot. Must be a remote node.
    #[clap(flatten)]
    pub from: LiveState,

    /// The snapshot path to write to.
    ///
    /// If not provided `<spec-name>-<spec-version>@<block-hash>.snap` will be used.
    #[clap(index = 1)]
    pub snapshot_path: Option<String>,
}

/// Runs the `create_snapshot` command.
pub async fn run<Block, HostFns>(shared: SharedParams, command: Command) -> sc_cli::Result<()>
where
    Block: BlockT + serde::de::DeserializeOwned,
    Block::Hash: serde::de::DeserializeOwned,
    Block::Header: serde::de::DeserializeOwned,
    <Block::Hash as FromStr>::Err: Debug,
    NumberFor<Block>: FromStr,
    <NumberFor<Block> as FromStr>::Err: Debug,
    HostFns: HostFunctions,
{
    let snapshot_path = command.snapshot_path;
    if !matches!(shared.runtime, shared_parameters::Runtime::Existing) {
        return Err("creating a snapshot is only possible with --runtime existing.".into());
    }

    let path = match snapshot_path {
        Some(path) => path,
        None => {
            let rpc = ws_client(&command.from.uri[0]).await.unwrap();
            let remote_spec = StateApi::<Block::Hash>::runtime_version(&rpc, None)
                .await
                .unwrap();
            let path_str = format!(
                "{}-{}@{}.snap",
                remote_spec.spec_name.to_lowercase(),
                remote_spec.spec_version,
                command.from.at.clone().unwrap_or("latest".to_owned())
            );
            log::info!(target: LOG_TARGET, "snapshot path not provided (-s), using '{}'", path_str);
            path_str
        }
    };

    let executor = build_executor::<HostFns>(&shared);
    let runtime_checks = RuntimeChecks {
        name_matches: false,
        version_increases: false,
        try_runtime_feature_enabled: false,
    };
    let _ = State::Live(command.from)
        .to_ext::<Block, HostFns>(&shared, &executor, Some(path.into()), runtime_checks)
        .await?;

    Ok(())
}
