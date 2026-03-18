// This file is part of Substrate.

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

use std::{fmt::Debug, str::FromStr, sync::Arc, time::Duration};

use parity_scale_codec::Encode;
use sc_cli::Result;
use sc_executor::sp_wasm_interface::HostFunctions;
use serde::de::DeserializeOwned;
use sp_core::H256;
use sp_runtime::traits::NumberFor;
use tokio::sync::Mutex;

use crate::{
    common::{
        empty_block::{inherents::providers::ProviderVariant, production::mine_block},
        state::{build_executor, state_machine_call_with_proof, RuntimeChecks, State},
    },
    BlockT, SharedParams,
};

/// Configuration for [`run`].
#[derive(Debug, Clone, clap::Parser)]
pub struct Command {
    /// How many empty blocks should be processed.
    #[arg(long)]
    pub n_blocks: u64,

    /// The chain blocktime in milliseconds.
    #[arg(long)]
    pub blocktime: u64,

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

    /// Whether to run pending migrations before fast-forwarding.
    #[arg(long, default_value = "true")]
    pub run_migrations: bool,

    /// The state type to use.
    #[command(subcommand)]
    pub state: State,
}

pub async fn run<Block, HostFns>(shared: SharedParams, command: Command) -> Result<()>
where
    Block: BlockT<Hash = H256> + DeserializeOwned,
    Block::Header: DeserializeOwned,
    <Block::Hash as FromStr>::Err: Debug,
    NumberFor<Block>: FromStr,
    <NumberFor<Block> as FromStr>::Err: Debug,
    HostFns: HostFunctions,
{
    let executor = build_executor::<HostFns>(&shared);
    let runtime_checks = RuntimeChecks {
        name_matches: !shared.disable_spec_name_check,
        version_increases: false,
        try_runtime_feature_enabled: true,
    };
    let ext = command
        .state
        .to_ext::<Block, HostFns>(&shared, &executor, None, runtime_checks)
        .await?;

    if command.run_migrations {
        log::info!("Running migrations...");
        state_machine_call_with_proof::<Block, HostFns>(
            &ext,
            &mut Default::default(),
            &executor,
            "TryRuntime_on_runtime_upgrade",
            command.try_state.encode().as_ref(),
            Default::default(), // we don't really need any extensions here.
            None,
        )?;
    }

    log::info!("Fast forwarding {} blocks...", command.n_blocks);

    let inner_ext = Arc::new(Mutex::new(ext.inner_ext));
    let mut parent_header = ext.header.clone();
    let mut parent_block_building_info = None;
    let provider_variant = ProviderVariant::Smart(Duration::from_millis(command.blocktime));

    for _ in 1..=command.n_blocks {
        let (next_block_building_info, next_header, _) = mine_block::<Block, HostFns>(
            inner_ext.clone(),
            &executor,
            parent_block_building_info,
            parent_header.clone(),
            provider_variant,
            command.try_state.clone(),
        )
        .await?;

        parent_block_building_info = Some(next_block_building_info);
        parent_header = next_header;
    }

    Ok(())
}
