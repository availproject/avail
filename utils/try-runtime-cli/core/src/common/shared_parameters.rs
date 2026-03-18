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

use std::{path::PathBuf, str::FromStr};

use sc_cli::{
    WasmExecutionMethod, WasmtimeInstantiationStrategy, DEFAULT_WASMTIME_INSTANTIATION_STRATEGY,
    DEFAULT_WASM_EXECUTION_METHOD,
};
use sp_runtime::StateVersion;

use crate::common::parse;

/// Shared parameters of the `try-runtime` commands
#[derive(Debug, Clone, clap::Parser)]
#[group(skip)]
pub struct SharedParams {
    /// The runtime to use.
    ///
    /// Must be a path to a wasm blob, compiled with `try-runtime` feature flag.
    ///
    /// Or, `existing`, indicating that you don't want to overwrite the runtime. This will use
    /// whatever comes from the remote node, or the snapshot file. This will most likely not work
    /// against a remote node, as no (sane) blockchain should compile its onchain wasm with
    /// `try-runtime` feature.
    #[arg(long, default_value = "existing")]
    pub runtime: Runtime,

    /// Whether to disable enforcing the new runtime `spec_name` matches the existing `spec_name`.
    #[clap(long, default_value = "false", default_missing_value = "true")]
    pub disable_spec_name_check: bool,

    /// Type of wasm execution used.
    #[arg(
		long = "wasm-execution",
		value_name = "METHOD",
		value_enum,
		ignore_case = true,
		default_value_t = DEFAULT_WASM_EXECUTION_METHOD,
	)]
    pub wasm_method: WasmExecutionMethod,

    /// The WASM instantiation method to use.
    ///
    /// Only has an effect when `wasm-execution` is set to `compiled`.
    #[arg(
		long = "wasm-instantiation-strategy",
		value_name = "STRATEGY",
		default_value_t = DEFAULT_WASMTIME_INSTANTIATION_STRATEGY,
		value_enum,
	)]
    pub wasmtime_instantiation_strategy: WasmtimeInstantiationStrategy,

    /// The number of 64KB pages to allocate for Wasm execution.
    ///
    /// Defaults to `sc_service::Configuration::default_heap_pages`.
    #[arg(long)]
    pub heap_pages: Option<u64>,

    /// Path to a file to export the storage proof into (as a JSON).
    /// If several blocks are executed, the path is interpreted as a folder
    /// where one file per block will be written (named `{block_number}-{block_hash}`).
    #[clap(long)]
    pub export_proof: Option<PathBuf>,

    /// Overwrite the `state_version`.
    ///
    /// Otherwise `remote-externalities` will automatically set the correct state version.
    #[arg(long, value_parser = parse::state_version)]
    pub overwrite_state_version: Option<StateVersion>,
}

#[derive(Debug, Clone)]
pub enum Runtime {
    /// Use the given path to the wasm binary file.
    ///
    /// It must have been compiled with `try-runtime`.
    Path(PathBuf),

    /// Use the code of the remote node, or the snapshot.
    ///
    /// In almost all cases, this is not what you want, because the code in the remote node does
    /// not have any of the try-runtime custom runtime APIs.
    Existing,
}

impl FromStr for Runtime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_ref() {
            "existing" => Runtime::Existing,
            x => Runtime::Path(x.into()),
        })
    }
}
