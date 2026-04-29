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

//! # Try-runtime
//!
//! Substrate's programmatic testing framework.
//!
//! > As the name suggests, `try-runtime` is a detailed testing framework that gives you a lot of
//! > control over what is being executed in which environment. It is recommended that user's first
//! > familiarize themselves with substrate in depth, particularly the execution model. It is
//! > critical
//! > to deeply understand how the wasm/client/runtime interactions, and the runtime apis work in
//! > the
//! > substrate runtime, before commencing to working with `try-runtime`.
//!
//! #### Resources
//!
//! Some resources about the above:
//!
//! 1. <https://www.crowdcast.io/e/substrate-seminar/41>
//! 2. <https://docs.substrate.io/fundamentals/runtime-development/>
//! 3. <https://www.youtube.com/watch?v=a_u3KMG-n-I>
//!
//! ---
//!
//! ## Background Knowledge
//!
//! The basis of all try-runtime commands is the same: connect to a live node, scrape its *state*
//! and put it inside a [`TestExternalities`], then call into a *specific runtime-api* using the
//! given state and some *runtime*. Multiple node URIs can be provided to enable parallel state
//! downloads for improved performance.
//!
//! Alternatively, the state could come from a snapshot file.
//!
//! All of the variables in the above statement are made *italic*. Let's look at each of them:
//!
//! 1. **State** is the key-value pairs of data that comprise the canonical information that any
//!    blockchain is keeping. A state can be full (all key-value pairs), or be partial (only pairs
//!    related to some pallets/prefixes). Moreover, some keys are special and are not related to
//!    specific pallets, known as [`well_known_keys`] in substrate. The most important of these is
//!    the `:CODE:` key, which contains the code used for execution, when wasm execution is chosen.
//!
//! 2. *A runtime-api* call is a call into a function defined in the runtime, *on top of a given
//!    state*. Each subcommand of `try-runtime` utilizes a specific *runtime-api*.
//!
//! 3. Finally, the **runtime** is the actual code that is used to execute the aforementioned
//!    runtime-api. Everything in this crate assumes wasm execution, which means the runtime that
//!    you use is the one stored onchain, namely under the `:CODE:` key.
//!
//! To recap, a typical try-runtime command does the following:
//!
//! 1. Download the state of a live chain, and write to an `externalities`.
//! 2. Overwrite the `:CODE:` with a given wasm blob
//! 3. Test some functionality via calling a runtime-api.
//!
//! ## Installation

//!```bash
//! # Install latest version (recommended for local development)
//! cargo install --git https://github.com/paritytech/try-runtime-cli --locked
//! # Install a specific version (recommended for tools like CI)
//! cargo install --git https://github.com/paritytech/try-runtime-cli --tag vX.Y.Z --locked
//! try-runtime --help
//! try-runtime on-runtime-upgrade --help
//! ```
//!
//! ## Usage
//!
//! To use any of the provided commands, [`SharedParams`] must be provided. The most important of
//! which being [`SharedParams::runtime`], which specifies which runtime to use. Furthermore,
//! [`SharedParams::overwrite_state_version`] can be used to alter the state-version (see
//! <https://forum.polkadot.network/t/state-trie-migration/852> for more info).
//!
//! Then, the specific command has to be specified. See [`Action`] for more information about each
//! command's specific customization flags, and assumptions regarding the runtime being used.
//!
//! Briefly, this CLI is capable of executing:
//!
//! * [`Action::OnRuntimeUpgrade`]: execute all the [`OnRuntimeUpgrade`] hooks.
//! * [`Action::ExecuteBlock`]: re-execute the given block.
//! * [`Action::FastForward`]: execute [`OnRuntimeUpgrade`] hooks, then fast-forward the chain a
//!   given number of blocks while checking try-state invarients.
//! * [`Action::OffchainWorker`]: re-execute the given block's offchain worker code path.
//! * [`Action::FollowChain`]: continuously execute the blocks of a remote chain on top of a given
//!   runtime.
//! * [`Action::CreateSnapshot`]: Create a snapshot file from a remote node.
//!
//! Finally, to make sure there are no errors regarding this, always run any `try-runtime` command
//! with `executor=trace` logging targets, which will specify which runtime is being used per api
//! call. Moreover, `remote-ext`, `try-runtime` and `runtime` logs targets will also be useful.
//!
//! ## Spec name check
//!
//! A common pitfall is that you might be running some test on top of the state of chain `x`, with
//! the runtime of chain `y`. To avoid this all commands do a spec-name check before executing
//! anything by default. This will check the, if any alterations are being made to the `:CODE:`,
//! then the spec names match. The spec versions are warned, but are not mandated to match.
//!
//! > If anything, in most cases, we expect spec-versions to NOT match, because try-runtime is all
//! > about testing unreleased runtimes.
//!
//! ## Note on signature and state-root checks
//!
//! All of the commands calling into `TryRuntime_execute_block` ([`Action::ExecuteBlock`] and
//! [`Action::FollowChain`]) disable both state root and signature checks. This is because in 99%
//! of the cases, the runtime that is being tested is different from the one that is stored in the
//! canonical chain state. This implies:
//!
//! 1. the state root will NEVER match, because `:CODE:` is different between the two.
//! 2. replaying all transactions will fail, because the spec-version is part of the transaction
//!    signature.
//!
//! ## Best Practices
//!
//! Try-runtime is all about battle-testing unreleased runtimes. The following list of suggestions
//! help developers maximize their testing coverage and make the best use of `try-runtime` features.
//!
//! ### Testing Runtime Upgrades
//!
//! One of the most powerful abilities of `try-runtime` is using the
//! [`OnRuntimeUpgrade::pre_upgrade`] and [`OnRuntimeUpgrade::post_upgrade`] hooks to test runtime
//! upgrades implemented with [`OnRuntimeUpgrade`]. [`OnRuntimeUpgrade`] can be implemented inside
//! the pallet, or standalone in a runtime to define a migration to execute next runtime upgrade. In
//! both cases, these methods can be added:
//!
//! ```ignore
//! #[cfg(feature = "try-runtime")]
//! fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {}
//!
//! #[cfg(feature = "try-runtime")]
//! fn post_upgrade(state: Vec<u8>) -> Result<(), TryRuntimeError> {}
//! ```
//!
//! (The pallet macro syntax will support this simply as a part of `#[pallet::hooks]`).
//!
//! These hooks will be called when you execute the [`Action::OnRuntimeUpgrade`] command, before and
//! after the migration. [`OnRuntimeUpgrade::pre_upgrade`] returns a [`Vec<u8>`] that can contain
//! arbitrary encoded data (usually some pre-upgrade state) which will be passed to
//! [`OnRuntimeUpgrade::pre_upgrade`] after upgrading and used for post checking.
//!
//! **Note on Multi-Block Migrations (MBM):** If the runtime uses MBMs, the standard
//! `pre_upgrade` and `post_upgrade` checks might be skipped by the executive. To
//! force these hooks to run synchronously for testing, use the `--disable-mbm-checks` flag.
//!
//! ### [`VersionedMigration`]
//!
//! It is strongly suggested to use [`VersionedMigration`] when writing custom migrations for
//! pallets.
//!
//! ### State Consistency
//!
//! Similarly, each pallet can expose a function in `#[pallet::hooks]` section as follows:
//!
//! ```ignore
//! #[cfg(feature = "try-runtime")]
//! fn try_state(_: BlockNumber) -> Result<(), TryRuntimeError> {}
//! ```
//!
//! which is called on numerous code paths in the try-runtime tool. These checks should ensure that
//! the state of the pallet is consistent and correct. See [`TryState`] for more info.
//!
//! ### Logging
//!
//! It is super helpful to make sure your migration code uses logging (always with a `runtime` log
//! target prefix, e.g. `runtime::balance`) and state exactly at which stage it is, and what it is
//! doing.
//!
//! ## Examples
//!
//! For the following examples, we assume the existence of the following:
//!
//! 1. a substrate node compiled with `--features try-runtime`, called `substrate`. This will be the
//!    running node that you connect to, and provide a wasm blob that has try-runtime functionality
//!    enabled.
//! 2. the `try-runtime` CLI binary on your path.
//!
//! ```bash
//! # this is like your running deployed node.
//! cargo build --features try-runtime --release && cp target/release/substrate .
//! ```
//!
//! > The above example is with `substrate`'s `kitchensink-runtime`, but is applicable to any
//! > substrate-based chain.
//!
//! * Run the migrations of a given runtime on top of a live state.
//!
//! ```bash
//! # Assuming local nodes are running (e.g., `./substrate --dev --tmp --ws-port 9999`).
//! # Multiple --uri flags can be provided for parallel state download.
//! try-runtime \
//!     --runtime /path-to-substrate/target/release/wbuild/my-runtime.wasm \
//!     on-runtime-upgrade \
//!     --disable-mbm-checks \
//!     live \
//!     --uri ws://localhost:9999 \
//!     --uri ws://localhost:9998
//!     ...
//! ```
//!
//! To speed up state download, you can provide multiple URIs for parallel fetching:
//!
//! ```bash
//! # assuming multiple substrate nodes running on ports 9999, 9998, 9997
//! try-runtime \
//!     --runtime /path-to-substrate/target/release/wbuild/my-runtime.wasm \
//!     on-runtime-upgrade \
//!     live \
//!     --uri ws://localhost:9999 \
//!     --uri ws://localhost:9998 \
//!     --uri ws://localhost:9997
//! ```
//!
//! * Same as the previous example, but run it at specific block number's state and using the live
//!   polkadot network. This means that this block hash's state should not yet have been pruned by
//!   the node running at `rpc.polkadot.io`. Multiple `--uri` flags can be provided for parallel
//!   state download.
//!
//! ```bash
//! try-runtime \
//!     --runtime /path-to-polkadot-runtimes/target/release/wbuild/polkadot-runtime/polkadot-runtime.wasm \
//!     on-runtime-upgrade \
//!     live --uri wss://rpc.polkadot.io:443 \
//!     # replace with your desired block hash!
//!     --at 0xa1b16c1efd889a9f17375ec4dd5c1b4351a2be17fa069564fced10d23b9b3836
//! ```
//!
//! * Now, let's use a snapshot file. First, we create the snapshot:
//!
//! ```bash
//! try-runtime --runtime existing create-snapshot --uri ws://localhost:9999 -- my-snapshot.snap
//! 2022-12-13 10:28:17.516  INFO                 main remote-ext: since no at is provided, setting it to latest finalized head, 0xe7d0b614dfe89af65b33577aae46a6f958c974bf52f8a5e865a0f4faeb578d22
//! 2022-12-13 10:28:17.516  INFO                 main remote-ext: since no prefix is filtered, the data for all pallets will be downloaded
//! 2022-12-13 10:28:17.550  INFO                 main remote-ext: writing snapshot of 1611464 bytes to "node-268@latest.snap"
//! 2022-12-13 10:28:17.551  INFO                 main remote-ext: initialized state externalities with storage root 0x925e4e95de4c08474fb7f976c4472fa9b8a1091619cd7820a793bf796ee6d932 and state_version V1
//! ```
//!
//! For faster snapshot creation with large state, use multiple RPC endpoints:
//!
//! ```bash
//! try-runtime --runtime existing create-snapshot \
//!     --uri ws://localhost:9999 \
//!     --uri ws://localhost:9998 \
//!     -- my-snapshot.snap
//! ```
//!
//! > Note that the snapshot contains the `existing` runtime, which does not have the correct
//! > `try-runtime` feature. In the following commands, we still need to overwrite the runtime.
//!
//! Then, we can use it to have the same command as before, `on-runtime-upgrade`
//!
//! ```bash
//! try-runtime \
//!     --runtime /path-to-substrate/target/release/wbuild/my-runtime.wasm \
//!     on-runtime-upgrade \
//!     snap -p my-snapshot.snap
//! ```
//!
//! * Execute the latest finalized block with the given runtime.
//!
//! ```bash
//! try-runtime \
//!     --runtime /path-to-substrate/target/release/wbuild/my-runtime.wasm \
//!     execute-block live \
//!     --uri ws://localhost:9999
//! ```
//!
//! This can still be customized at a given block with `--at`. If you want to use a snapshot, you
//! can still use `--block-ws-uri` to provide a node form which the block data can be fetched.
//!
//! Moreover, this runs the [`TryState`] hooks as well. The hooks to run can be customized with the
//! `--try-state`. For example:
//!
//! ```bash
//! try-runtime \
//!     --runtime /path-to-substrate/target/release/wbuild/my-runtime.wasm \
//!    execute-block \
//!    --try-state System,Staking \
//!    live \
//!    --uri ws://localhost:9999 \
//!    --pallet System Staking
//! ```
//!
//! Will only run the `try-state` of the two given pallets. When running `try-state` against
//! some real chain data it can take a long time for the command to execute since it has to
//! query all the key-value pairs. In scenarios like above where we only want to run the
//! `try-state` for some specific pallets, we can use the `--pallet` option to specify from
//! which pallets we want to query the state. This will greatly decrease the execution time.
//!
//! See [`TryStateSelect`] for more information.
//!
//! * Follow our live chain's blocks using `follow-chain`, whilst running the try-state of 3 pallets
//!   in a round robin fashion
//!
//! ```bash
//! try-runtime \
//!     --runtime /path-to-substrate/target/release/wbuild/my-runtime.wasm \
//!     follow-chain \
//!     --uri ws://localhost:9999 \
//!     --try-state rr-3
//! ```
//!
//! [`VersionedMigration`]: frame_support::migrations::VersionedMigration
//! [`OnRuntimeUpgrade`]: frame_support::traits::OnRuntimeUpgrade
//! [`OnRuntimeUpgrade::pre_upgrade`]: frame_support::traits::OnRuntimeUpgrade::pre_upgrade
//! [`OnRuntimeUpgrade::post_upgrade`]: frame_support::traits::OnRuntimeUpgrade::post_upgrade
//! [`TryStateSelect`]: frame_support::traits::TryStateSelect
//! [`TryState`]: frame_support::traits::TryState
//! [`TestExternalities`]: sp_state_machine::TestExternalities
//! [`well_known_keys`]: sp_storage::well_known_keys
//! [`Action`]: try_runtime_core::commands::Action
//! [`Action::FollowChain`]: try_runtime_core::commands::Action::FollowChain
//! [`Action::OnRuntimeUpgrade`]: try_runtime_core::commands::Action::OnRuntimeUpgrade
//! [`Action::ExecuteBlock`]: try_runtime_core::commands::Action::ExecuteBlock
//! [`Action::OffchainWorker`]: try_runtime_core::commands::Action::OffchainWorker
//! [`Action::CreateSnapshot`]: try_runtime_core::commands::Action::CreateSnapshot
//! [`Action::FastForward`]: try_runtime_core::commands::Action::FastForward
//! [`SharedParams`]: try_runtime_core::common::shared_parameters::SharedParams
//! [`SharedParams::runtime`]: try_runtime_core::common::shared_parameters::SharedParams::runtime
//! [`SharedParams::overwrite_state_version`]: try_runtime_core::common::shared_parameters::SharedParams::overwrite_state_version

use std::env;

use avail_blob::types::HostFunctions;
use clap::Parser;
use da_runtime::NodeBlock;
use try_runtime_core::commands::TryRuntime;

fn init_env() {
	if env::var(env_logger::DEFAULT_FILTER_ENV).is_err() {
		env::set_var(env_logger::DEFAULT_FILTER_ENV, "info");
	}
	env_logger::init();
}

#[tokio::main]
async fn main() {
	init_env();

	let cmd = TryRuntime::parse();
	cmd.run::<NodeBlock, HostFunctions>().await.unwrap();
}
