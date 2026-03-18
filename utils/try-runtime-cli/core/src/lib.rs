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

#![allow(clippy::result_large_err)]

use std::{fmt::Debug, str::FromStr, time::Duration};

use common::shared_parameters::SharedParams;
use parity_scale_codec::DecodeAll;
use sc_executor::{sp_wasm_interface::HostFunctions, WasmExecutor};
use sp_core::{
    offchain::{
        testing::{TestOffchainExt, TestTransactionPoolExt},
        OffchainDbExt, OffchainWorkerExt, TransactionPoolExt,
    },
    traits::ReadRuntimeVersionExt,
};
use sp_externalities::Extensions;
use sp_keystore::{testing::MemoryKeystore, KeystoreExt};
use sp_runtime::traits::Block as BlockT;
use sp_weights::Weight;

pub mod commands;
pub mod common;

pub(crate) const LOG_TARGET: &str = "try-runtime::cli";

/// Get the hash type of the generic `Block` from a `hash_str`.
pub(crate) fn hash_of<Block: BlockT>(hash_str: &str) -> sc_cli::Result<Block::Hash>
where
    <Block::Hash as FromStr>::Err: Debug,
{
    hash_str
        .parse::<<Block as BlockT>::Hash>()
        .map_err(|e| format!("Could not parse block hash: {:?}", e).into())
}

pub struct RefTimeInfo {
    pub used: Duration,
    pub max: Duration,
}

impl TryFrom<Vec<u8>> for RefTimeInfo {
    type Error = String;

    /// try_from Vec encoded as (Weight, Weight) tuple
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let (weight_used, weight_max) = <(Weight, Weight)>::decode_all(&mut &*value)
            .map_err(|e| format!("failed to decode weight: {:?}", e))?;

        Ok(RefTimeInfo {
            // 1000 picoseconds == 1 nanosecond
            used: Duration::from_nanos(weight_used.ref_time() / 1000),
            max: Duration::from_nanos(weight_max.ref_time() / 1000),
        })
    }
}

/// Build all extensions that we typically use.
pub(crate) fn full_extensions<H: HostFunctions>(wasm_executor: WasmExecutor<H>) -> Extensions {
    let mut extensions = Extensions::default();
    let (offchain, _offchain_state) = TestOffchainExt::new();
    let (pool, _pool_state) = TestTransactionPoolExt::new();
    let keystore = MemoryKeystore::new();
    extensions.register(OffchainDbExt::new(offchain.clone()));
    extensions.register(OffchainWorkerExt::new(offchain));
    extensions.register(KeystoreExt::new(keystore));
    extensions.register(TransactionPoolExt::new(pool));
    extensions.register(ReadRuntimeVersionExt::new(wasm_executor));

    extensions
}

pub(crate) fn rpc_err_handler(error: impl Debug) -> &'static str {
    log::error!(target: LOG_TARGET, "rpc error: {:?}", error);
    "rpc error."
}
