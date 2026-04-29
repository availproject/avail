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

use std::{fmt::Debug, path::PathBuf, str::FromStr};

use frame_remote_externalities::{
	Builder, Mode, OfflineConfig, OnlineConfig, RemoteExternalities, SnapshotConfig, Transport,
};
use parity_scale_codec::Decode;
use sc_cli::{execution_method_from_cli, RuntimeVersion};
use sc_executor::{
	sp_wasm_interface::HostFunctions, HeapAllocStrategy, WasmExecutor, DEFAULT_HEAP_ALLOC_STRATEGY,
};
use sp_api::{CallContext, StorageProof};
use sp_core::{
	hexdisplay::HexDisplay, storage::well_known_keys, traits::ReadRuntimeVersion, twox_128, Hasher,
};
use sp_externalities::Extensions;
use sp_runtime::{
	traits::{BlakeTwo256, Block as BlockT, HashingFor, Header as HeaderT},
	DeserializeOwned,
};
use sp_state_machine::{OverlayedChanges, StateMachine, TestExternalities, TrieBackendBuilder};
use substrate_rpc_client::{ws_client, ChainApi};

use crate::{
	common::{
		parse,
		shared_parameters::{Runtime, SharedParams},
	},
	hash_of, rpc_err_handler, LOG_TARGET,
};

/// A `Live` variant for [`State`]
#[derive(Debug, Clone, clap::Args)]
pub struct LiveState {
	/// The url(s) to connect to. Can be provided multiple times for parallel state download.
	#[arg(
		short,
		long,
		value_parser = parse::url,
		num_args = 1..,
		required = true,
	)]
	pub uri: Vec<String>,

	/// The block hash at which to fetch the state.
	///
	/// If non provided, then the latest finalized head is used.
	#[arg(
		short,
		long,
		value_parser = parse::hash,
	)]
	pub at: Option<String>,

	/// A pallet to scrape. Can be provided multiple times. If empty, entire chain state will
	/// be scraped.
	///
	/// This is equivalent to passing `xx_hash_64(pallet)` to `--hashed_prefixes`.
	#[arg(short, long, num_args = 1..)]
	pub pallet: Vec<String>,

	/// Storage entry key prefixes to scrape and inject into the test externalities. Pass as 0x
	/// prefixed hex strings. By default, all keys are scraped and included.
	#[arg(long = "prefix", value_parser = parse::hash, num_args = 1..)]
	pub hashed_prefixes: Vec<String>,

	/// Fetch the child-keys as well.
	///
	/// Default is `false`, if specific `--pallets` are specified, `true` otherwise. In other
	/// words, if you scrape the whole state the child tree data is included out of the box.
	/// Otherwise, it must be enabled explicitly using this flag.
	#[arg(long)]
	pub child_tree: bool,
}

impl LiveState {
	/// Return the `at` block hash as a `Hash`, if it exists.
	pub fn at<Block: BlockT>(&self) -> sc_cli::Result<Option<<Block>::Hash>>
	where
		<Block::Hash as FromStr>::Err: Debug,
	{
		self.at
			.clone()
			.map(|s| hash_of::<Block>(s.as_str()))
			.transpose()
	}

	/// Converts this `LiveState` into a `LiveState` for the previous block.
	///
	/// Useful for opertations like when you want to execute a block, but also need the state of the
	/// block *before* it.
	pub async fn to_prev_block_live_state<Block: BlockT>(self) -> sc_cli::Result<LiveState>
	where
		<Block::Hash as FromStr>::Err: Debug,
	{
		// We want to execute the block `at`, therefore need the state of the block *before* it.
		let at = self.at::<Block>()?;

		// Get the block number requested by the user, or the current block number if they
		// didn't specify one.
		let rpc = ws_client(&self.uri[0]).await?;
		let previous_hash = ChainApi::<(), Block::Hash, Block::Header, ()>::header(&rpc, at)
			.await
			.map_err(rpc_err_handler)
			.and_then(|maybe_header| {
				maybe_header
					.ok_or("header_not_found")
					.map(|h| *h.parent_hash())
			})?;

		Ok(LiveState {
			at: Some(hex::encode(previous_hash)),
			..self
		})
	}
}

/// The source of runtime *state* to use.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum State {
	/// Use a state snapshot as the source of runtime state.
	Snap {
		#[clap(short = 'p', long = "path", alias = "snapshot-path")]
		path: Option<PathBuf>,
	},

	/// Use a live chain as the source of runtime state.
	Live(LiveState),
}

/// Checks to perform on the given runtime, compared to the existing runtime.
#[derive(Debug, Clone, Copy)]
pub struct RuntimeChecks {
	/// Enforce the `spec_name`s match
	pub name_matches: bool,
	/// Enforce the `spec_version` of the given is greater or equal to the existing
	/// runtime.
	pub version_increases: bool,
	/// Enforce that the given runtime is compiled with the try-runtime feature.
	pub try_runtime_feature_enabled: bool,
}

impl State {
	/// Create the [`RemoteExternalities`].
	///
	/// This will override the code as it sees fit based on [`Runtime`]. It will also check the
	/// spec-version and name.
	pub async fn to_ext<Block: BlockT + DeserializeOwned, HostFns: HostFunctions>(
		&self,
		shared: &SharedParams,
		executor: &WasmExecutor<HostFns>,
		state_snapshot: Option<SnapshotConfig>,
		runtime_checks: RuntimeChecks,
	) -> sc_cli::Result<RemoteExternalities<Block>>
	where
		Block::Header: DeserializeOwned,
		<Block::Hash as FromStr>::Err: Debug,
	{
		let builder = match self {
			State::Snap { path } => {
				let path = path
					.as_ref()
					.ok_or_else(|| "no snapshot path provided".to_string())?;

				Builder::<Block>::new().mode(Mode::Offline(OfflineConfig {
					state_snapshot: SnapshotConfig::new(path),
				}))
			},
			State::Live(LiveState {
				pallet,
				uri,
				at,
				child_tree,
				hashed_prefixes,
			}) => {
				let at = match at {
					Some(at_str) => Some(hash_of::<Block>(at_str)?),
					None => None,
				};
				let hashed_prefixes = hashed_prefixes
                    .iter()
                    .map(|p_str| {
                        hex::decode(p_str).map_err(|e| {
                            format!(
                                "Error decoding `hashed_prefixes` hex string entry '{:?}' to bytes: {:?}",
                                p_str, e
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
				if uri.len() > 1 {
					log::warn!(
						target: LOG_TARGET,
						"multiple RPC URIs were provided, but this frame-remote-externalities revision only supports a single transport; using the first URI"
					);
				}
				Builder::<Block>::new().mode(Mode::Online(OnlineConfig {
					at,
					transport: Transport::from(uri[0].clone()),
					state_snapshot,
					pallets: pallet.clone(),
					child_trie: *child_tree,
					hashed_keys: vec![
						// we always download the code, but we almost always won't use it, based on
						// `Runtime`.
						well_known_keys::CODE.to_vec(),
						// we will always download this key, since it helps detect if we should do
						// runtime migration or not.
						[twox_128(b"System"), twox_128(b"LastRuntimeUpgrade")].concat(),
						[twox_128(b"System"), twox_128(b"Number")].concat(),
					],
					hashed_prefixes,
				}))
			},
		};

		// possibly overwrite the state version, should hardly be needed.
		let builder = if let Some(state_version) = shared.overwrite_state_version {
			log::warn!(
				target: LOG_TARGET,
				"overwriting state version to {:?}, you better know what you are doing.",
				state_version
			);
			builder.overwrite_state_version(state_version)
		} else {
			builder
		};

		// then, we prepare to replace the code based on what the CLI wishes.
		let maybe_code_to_overwrite = match shared.runtime {
			Runtime::Path(ref path) => Some(std::fs::read(path).map_err(|e| {
				format!("error while reading runtime file from {:?}: {:?}", path, e)
			})?),
			Runtime::Existing => None,
		};

		// build the main ext.
		let mut ext = builder.build().await?;

		// actually replace the code if needed.
		if let Some(new_code) = maybe_code_to_overwrite {
			let original_code = ext
				.execute_with(|| sp_io::storage::get(well_known_keys::CODE))
				.expect("':CODE:' is always downloaded in try-runtime-cli; qed");

			// NOTE: see the impl notes of `read_runtime_version`, the ext is almost not used here,
			// only as a backup.
			ext.insert(well_known_keys::CODE.to_vec(), new_code.clone());
			let old_version = <RuntimeVersion as Decode>::decode(
				&mut &*executor
					.read_runtime_version(&original_code, &mut ext.ext())
					.unwrap(),
			)
			.unwrap();
			let old_code_hash =
				HexDisplay::from(BlakeTwo256::hash(&original_code).as_fixed_bytes()).to_string();
			log::info!(
				target: LOG_TARGET,
				"Original runtime [Name: {:?}] [Version: {:?}] [Code hash: 0x{}...{}]",
				old_version.spec_name,
				old_version.spec_version,
				&old_code_hash[0..4],
				&old_code_hash[old_code_hash.len() - 4..],
			);
			log::debug!(
				target: LOG_TARGET,
				"Original runtime full code hash: 0x{:?}",
				old_code_hash,
			);
			let new_version = <RuntimeVersion as Decode>::decode(
				&mut &*executor
					.read_runtime_version(&new_code, &mut ext.ext())
					.unwrap(),
			)
			.unwrap();
			let new_code_hash =
				HexDisplay::from(BlakeTwo256::hash(&new_code).as_fixed_bytes()).to_string();
			log::info!(
				target: LOG_TARGET,
				"New runtime      [Name: {:?}] [Version: {:?}] [Code hash: 0x{}...{}]",
				new_version.spec_name,
				new_version.spec_version,
				&new_code_hash[0..4],
				&new_code_hash[new_code_hash.len() - 4..],
			);
			log::debug!(
				target: LOG_TARGET,
				"New runtime code hash: 0x{:?}",
				new_code_hash
			);

			if runtime_checks.name_matches && new_version.spec_name != old_version.spec_name {
				return Err(
					"Spec names must match. Use `--disable-spec-name-check` to disable this check."
						.into(),
				);
			}

			if runtime_checks.version_increases
				&& new_version.spec_version <= old_version.spec_version
			{
				return Err(format!("New runtime spec version must be greater than the on-chain runtime spec version: {} <= {}. Use `--disable-spec-version-check` to disable this check.", new_version.spec_version, old_version.spec_version).into());
			}
		}

		if runtime_checks.try_runtime_feature_enabled
			&& !ensure_try_runtime::<Block, HostFns>(executor, &mut ext)
		{
			return Err("Given runtime is not compiled with the try-runtime feature.".into());
		}

		Ok(ext)
	}
}

/// Build wasm executor by default config.
pub(crate) fn build_executor<H: HostFunctions>(shared: &SharedParams) -> WasmExecutor<H> {
	let heap_pages =
		shared
			.heap_pages
			.map_or(DEFAULT_HEAP_ALLOC_STRATEGY, |p| HeapAllocStrategy::Static {
				extra_pages: p as _,
			});

	WasmExecutor::builder()
		.with_execution_method(execution_method_from_cli(
			shared.wasm_method,
			shared.wasmtime_instantiation_strategy,
		))
		.with_onchain_heap_alloc_strategy(heap_pages)
		.with_offchain_heap_alloc_strategy(heap_pages)
		// There is not that much we can do if someone is using unknown host functions.
		// They would need to fork the `cli` to add their custom host functions.
		.with_allow_missing_host_functions(true)
		.build()
}

/// Ensure that the given `ext` is compiled with `try-runtime`
fn ensure_try_runtime<Block: BlockT, HostFns: HostFunctions>(
	executor: &WasmExecutor<HostFns>,
	ext: &mut TestExternalities<HashingFor<Block>>,
) -> bool {
	use sp_api::RuntimeApiInfo;
	let final_code = ext
		.execute_with(|| sp_io::storage::get(well_known_keys::CODE))
		.expect("':CODE:' is always downloaded in try-runtime-cli; qed");
	let final_version = <RuntimeVersion as Decode>::decode(
		&mut &*executor
			.read_runtime_version(&final_code, &mut ext.ext())
			.unwrap(),
	)
	.unwrap();
	final_version
		.api_version(&<dyn frame_try_runtime::TryRuntime<Block>>::ID)
		.is_some()
}

/// Execute the given `method` and `data` on top of `ext`, returning the results (encoded) and the
/// state `changes`.
pub(crate) fn state_machine_call<Block: BlockT, HostFns: HostFunctions>(
	ext: &TestExternalities<HashingFor<Block>>,
	executor: &WasmExecutor<HostFns>,
	method: &'static str,
	data: &[u8],
	mut extensions: Extensions,
) -> sc_cli::Result<(OverlayedChanges<HashingFor<Block>>, Vec<u8>)> {
	let mut changes = Default::default();
	let encoded_result = StateMachine::new(
		&ext.backend,
		&mut changes,
		executor,
		method,
		data,
		&mut extensions,
		&sp_state_machine::backend::BackendRuntimeCode::new(&ext.backend).runtime_code()?,
		CallContext::Offchain,
	)
	.execute()
	.map_err(|e| format!("failed to execute '{}': {}", method, e))
	.map_err::<sc_cli::Error, _>(Into::into)?;

	Ok((changes, encoded_result))
}

/// Same as [`state_machine_call`], but it also computes and returns the storage proof and ref time
/// information.
///
/// Make sure [`LOG_TARGET`] is enabled in logging.
pub(crate) fn state_machine_call_with_proof<Block: BlockT, HostFns: HostFunctions>(
	ext: &TestExternalities<HashingFor<Block>>,
	storage_overlay: &mut OverlayedChanges<HashingFor<Block>>,
	executor: &WasmExecutor<HostFns>,
	method: &'static str,
	data: &[u8],
	mut extensions: Extensions,
	maybe_export_proof: Option<PathBuf>,
) -> sc_cli::Result<(StorageProof, Vec<u8>)> {
	let runtime_code_backend = sp_state_machine::backend::BackendRuntimeCode::new(&ext.backend);
	let proving_backend = TrieBackendBuilder::wrap(&ext.backend)
		.with_recorder(Default::default())
		.build();
	let runtime_code = runtime_code_backend.runtime_code()?;

	let encoded_result = StateMachine::new(
		&proving_backend,
		storage_overlay,
		executor,
		method,
		data,
		&mut extensions,
		&runtime_code,
		CallContext::Offchain,
	)
	.execute()
	.map_err(|e| format!("failed to execute {}: {}", method, e))
	.map_err::<sc_cli::Error, _>(Into::into)?;

	let proof = proving_backend
		.extract_proof()
		.expect("A recorder was set and thus, a storage proof can be extracted; qed");

	if let Some(path) = maybe_export_proof {
		let mut file = std::fs::File::create(&path).map_err(|e| {
			log::error!(
				target: LOG_TARGET,
				"Failed to create file {}: {:?}",
				path.to_string_lossy(),
				e
			);
			e
		})?;

		log::info!(target: LOG_TARGET, "Writing storage proof to {}", path.to_string_lossy());

		use std::io::Write as _;
		file.write_all(storage_proof_to_raw_json(&proof).as_bytes())
			.map_err(|e| {
				log::error!(
					target: LOG_TARGET,
					"Failed to write storage proof to {}: {:?}",
					path.to_string_lossy(),
					e
				);
				e
			})?;
	}

	Ok((proof, encoded_result))
}

/// Converts a [`sp_state_machine::StorageProof`] into a JSON string.
fn storage_proof_to_raw_json(storage_proof: &sp_state_machine::StorageProof) -> String {
	serde_json::Value::Object(
		storage_proof
			.to_memory_db::<sp_runtime::traits::BlakeTwo256>()
			.drain()
			.iter()
			.map(|(key, (value, _n))| {
				(
					format!("0x{}", hex::encode(key.as_bytes())),
					serde_json::Value::String(format!("0x{}", hex::encode(value))),
				)
			})
			.collect(),
	)
	.to_string()
}
