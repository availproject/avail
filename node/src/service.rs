// This file is part of Substrate.

// Copyright (C) 2018-2022 Parity Technologies (UK) Ltd.
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

//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.
#![allow(dead_code)]

use crate::finality_watcher::finality_promoter;
use crate::{cli::Cli, rpc as node_rpc};
use avail_blob::p2p::{get_blob_p2p_config, BlobHandle};
use avail_blob::rpc::{BlobApiServer, BlobRpc};
use avail_blob::store::{RocksdbBlobStore, StorageApiT};
use avail_blob::types::FullClient;
use codec::Encode;
use da_runtime::extensions::check_batch_transactions::CheckBatchTransactions;
use da_runtime::{apis::RuntimeApi, BlockNumber, NodeBlock as Block, Runtime};
use frame_benchmarking_cli::SUBSTRATE_REFERENCE_HARDWARE;
use frame_system_rpc_runtime_api::AccountNonceApi;
use futures::prelude::*;
use jsonrpsee::RpcModule;
use pallet_transaction_payment::ChargeTransactionPayment;
use sc_client_api::{Backend, BlockBackend};
use sc_consensus_babe::{self, SlotProportion};
use sc_network::{service::traits::NetworkService, Event, NetworkBackend, NetworkEventStream};
use sc_network_sync::SyncingService;
use sc_network_sync::WarpSyncConfig;
use sc_service::{error::Error as ServiceError, Configuration, RpcHandlers, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_api::ProvideRuntimeApi;
use sp_consensus_babe::inherents::BabeCreateInherentDataProviders;
use sp_core::crypto::Pair;
use sp_core::traits::SpawnNamed;
use sp_runtime::{generic::Era, traits::Block as BlockT, SaturatedConversion};
use std::time::Duration;
use std::{path::Path, sync::Arc};
use substrate_prometheus_endpoint::{PrometheusError, Registry};

pub const LOG_TARGET: &str = "avail::node::service";

pub type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
type FullGrandpaBlockImport =
	sc_consensus_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>;

/// The transaction pool type definition.
pub type TransactionPool = sc_transaction_pool::TransactionPoolHandle<Block, FullClient>;

type Backoff = sc_consensus_slots::BackoffAuthoringOnFinalizedHeadLagging<BlockNumber>;

pub type BlockImport = crate::da_block_import::BlockImport<
	Block,
	FullClient,
	sc_consensus_babe::BabeBlockImport<
		Block,
		FullClient,
		FullGrandpaBlockImport,
		BabeCreateInherentDataProviders<Block>,
		FullSelectChain,
	>,
>;

/// Fetch the nonce of the given `account` from the chain state.
///
/// Note: Should only be used for tests.
pub fn fetch_nonce(client: &FullClient, account: sp_core::sr25519::Pair) -> u32 {
	let best_hash = client.chain_info().best_hash;
	client
		.runtime_api()
		.account_nonce(best_hash, account.public().into())
		.expect("Fetching account nonce works; qed")
}

/// Create a transaction using the given `call`.
///
/// The transaction will be signed by `sender`. If `nonce` is `None` it will be fetched from the
/// state of the best block.
///
/// Note: Should only be used for tests.
pub fn create_extrinsic(
	client: &FullClient,
	sender: sp_core::sr25519::Pair,
	function: impl Into<da_runtime::RuntimeCall>,
	nonce: Option<u32>,
) -> da_runtime::UncheckedExtrinsic {
	let function = function.into();
	let genesis_hash = client
		.block_hash(0)
		.ok()
		.flatten()
		.expect("Genesis block exists; qed");
	let best_hash = client.chain_info().best_hash;
	let best_block = client.chain_info().best_number;
	let nonce = nonce.unwrap_or_else(|| fetch_nonce(client, sender.clone()));

	let period = da_runtime::BlockHashCount::get()
		.checked_next_power_of_two()
		.map(|c| c / 2)
		.unwrap_or(2) as u64;
	let tip = 0;
	let extra = (
		frame_system::CheckNonZeroSender::<Runtime>::new(),
		frame_system::CheckSpecVersion::<Runtime>::new(),
		frame_system::CheckTxVersion::<Runtime>::new(),
		frame_system::CheckGenesis::<Runtime>::new(),
		frame_system::CheckEra::<Runtime>::from(Era::mortal(period, best_block.saturated_into())),
		frame_system::CheckNonce::<Runtime>::from(nonce),
		frame_system::CheckWeight::<Runtime>::new(),
		ChargeTransactionPayment::<Runtime>::from(tip),
		CheckBatchTransactions::<Runtime>::new(),
	);

	let raw_payload = da_runtime::SignedPayload::from_raw(
		function.clone(),
		extra.clone(),
		(
			(),
			da_runtime::VERSION.spec_version,
			da_runtime::VERSION.transaction_version,
			genesis_hash,
			best_hash,
			(),
			(),
			(),
			(),
		),
	);
	let signature = raw_payload.using_encoded(|e| sender.sign(e));

	da_runtime::UncheckedExtrinsic::new_signed(
		function,
		sp_runtime::AccountId32::from(sender.public()).into(),
		da_runtime::Signature::Sr25519(signature),
		extra,
	)
}

/// Creates a new partial node.
#[allow(clippy::redundant_clone)]
#[allow(clippy::type_complexity)]
pub fn new_partial(
	config: &Configuration,
	unsafe_da_sync: bool,
	kate_rpc_deps: kate_rpc::Deps,
	grandpa_justification_period: u32,
) -> Result<
	sc_service::PartialComponents<
		FullClient,
		FullBackend,
		FullSelectChain,
		sc_consensus::DefaultImportQueue<Block>,
		sc_transaction_pool::TransactionPoolHandle<Block, FullClient>,
		(
			impl Fn(
				sc_rpc::SubscriptionTaskExecutor,
			) -> Result<jsonrpsee::RpcModule<()>, sc_service::Error>,
			(
				BlockImport,
				sc_consensus_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
				sc_consensus_babe::BabeLink<Block>,
			),
			sc_consensus_grandpa::SharedVoterState,
			Option<Telemetry>,
			Arc<dyn StorageApiT>,
		),
	>,
	ServiceError,
> {
	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	let executor = sc_service::new_native_or_wasm_executor(&config);

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, _>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;
	let client = Arc::new(client);

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager
			.spawn_handle()
			.spawn("telemetry", None, worker.run());
		telemetry
	});

	let telemetry_handle = telemetry.as_ref().map(|t| t.handle());
	let (telemetry_worker, telemetry_channel) =
		avail_observability::telemetry::Worker::new(telemetry_handle.clone());
	_ = avail_observability::telemetry::AVAIL_TELEMETRY.set(telemetry_channel);

	telemetry_worker.spawn_background_task();

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let transaction_pool = Arc::from(
		sc_transaction_pool::Builder::new(
			task_manager.spawn_essential_handle(),
			client.clone(),
			config.role.is_authority().into(),
		)
		.with_options(config.transaction_pool.clone())
		.with_prometheus(config.prometheus_registry())
		.build(),
	);

	let (grandpa_block_import, grandpa_link) = sc_consensus_grandpa::block_import(
		client.clone(),
		grandpa_justification_period,
		&(client.clone() as Arc<_>),
		select_chain.clone(),
		telemetry.as_ref().map(|x| x.handle()),
	)?;
	let justification_import = grandpa_block_import.clone();
	let babe_config = sc_consensus_babe::configuration(&*client)?;
	let slot_duration = babe_config.slot_duration();
	let (block_import, babe_link) = sc_consensus_babe::block_import(
		sc_consensus_babe::configuration(&*client)?,
		grandpa_block_import,
		client.clone(),
		Arc::new(move |_, _| async move {
			let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
			let slot =
			sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
				*timestamp,
				slot_duration,
			);
			Ok((slot, timestamp))
		}) as BabeCreateInherentDataProviders<Block>,
		select_chain.clone(),
		OffchainTransactionPoolFactory::new(transaction_pool.clone()),
	)?;

	// Initialize the Blob database
	let blob_db_path = config.base_path.path().join("blob_database");

	let blob_database: Arc<dyn StorageApiT> = Arc::new(
		RocksdbBlobStore::open(&blob_db_path)
			.map_err(|e| ServiceError::Other(format!("open blob_database: {e}")))?,
	);

	let da_block_import = BlockImport::new(
		client.clone(),
		block_import.clone(),
		unsafe_da_sync,
		blob_database.clone(),
	);

	let slot_duration = babe_link.config().slot_duration();
	let (import_queue, babe_worker_handle) =
		sc_consensus_babe::import_queue(sc_consensus_babe::ImportQueueParams {
			link: babe_link.clone(),
			block_import: block_import.clone(),
			justification_import: Some(Box::new(justification_import)),
			client: client.clone(),
			slot_duration,
			spawner: &task_manager.spawn_essential_handle(),
			registry: config.prometheus_registry(),
			telemetry: telemetry.as_ref().map(|x| x.handle()),
		})?;

	let import_setup = (da_block_import, grandpa_link, babe_link);

	let (rpc_extensions_builder, rpc_setup) = {
		let (_, grandpa_link, _) = &import_setup;

		let justification_stream = grandpa_link.justification_stream();
		let shared_authority_set = grandpa_link.shared_authority_set().clone();
		let shared_voter_state = sc_consensus_grandpa::SharedVoterState::empty();
		let shared_voter_state2 = shared_voter_state.clone();

		let finality_proof_provider = sc_consensus_grandpa::FinalityProofProvider::new_for_service(
			backend.clone(),
			Some(shared_authority_set.clone()),
		);

		let client = client.clone();
		let pool = transaction_pool.clone();
		let select_chain = select_chain.clone();
		let keystore = keystore_container.keystore();
		let chain_spec = config.chain_spec.cloned_box();

		let rpc_backend = backend.clone();
		let rpc_extensions_builder = move |subscription_executor| {
			let deps = node_rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				select_chain: select_chain.clone(),
				chain_spec: chain_spec.cloned_box(),
				babe: node_rpc::BabeDeps {
					keystore: keystore.clone(),
					babe_worker_handle: babe_worker_handle.clone(),
				},
				grandpa: node_rpc::GrandpaDeps {
					shared_voter_state: shared_voter_state.clone(),
					shared_authority_set: shared_authority_set.clone(),
					justification_stream: justification_stream.clone(),
					subscription_executor,
					finality_provider: finality_proof_provider.clone(),
				},
				kate_rpc_deps: kate_rpc_deps.clone(),
			};
			node_rpc::create_full(deps, rpc_backend.clone()).map_err(Into::into)
		};

		(rpc_extensions_builder, shared_voter_state2)
	};

	Ok(sc_service::PartialComponents {
		client,
		backend,
		task_manager,
		keystore_container,
		select_chain,
		import_queue,
		transaction_pool,
		other: (
			rpc_extensions_builder,
			import_setup,
			rpc_setup,
			telemetry,
			blob_database,
		),
	})
}

/// Result of [`new_full_base`].
pub struct NewFullBase {
	/// The task manager of the node.
	pub task_manager: TaskManager,
	/// The client instance of the node.
	pub client: Arc<FullClient>,
	/// The networking service of the node.
	pub network: Arc<dyn NetworkService>,
	/// The syncing service of the node.
	pub sync: Arc<SyncingService<Block>>,
	/// The transaction pool of the node.
	pub transaction_pool: Arc<TransactionPool>,
	/// The rpc handlers of the node.
	pub rpc_handlers: RpcHandlers,
}

/// Creates a full service from the configuration.
#[allow(clippy::too_many_arguments)]
pub fn new_full_base<N: NetworkBackend<Block, <Block as BlockT>::Hash>>(
	config: Configuration,
	disable_hardware_benchmarks: bool,
	with_startup_data: impl FnOnce(&BlockImport, &sc_consensus_babe::BabeLink<Block>),
	unsafe_da_sync: bool,
	kate_rpc_deps: kate_rpc::Deps,
	grandpa_justification_period: u32,
) -> Result<NewFullBase, ServiceError> {
	// let is_offchain_indexing_enabled = config.offchain_worker.indexing_enabled;
	let role = config.role;
	let force_authoring = config.force_authoring;
	let name = config.network.node_name.clone();
	let enable_grandpa = !config.disable_grandpa;
	let prometheus_registry = config.prometheus_registry().cloned();
	let enable_offchain_worker = config.offchain_worker.enabled;

	let hwbench = (!disable_hardware_benchmarks)
		.then(|| {
			config.database.path().map(|database_path| {
				let _ = std::fs::create_dir_all(&database_path);
				sc_sysinfo::gather_hwbench(Some(database_path), &SUBSTRATE_REFERENCE_HARDWARE)
			})
		})
		.flatten();

	let sc_service::PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (rpc_builder, import_setup, rpc_setup, mut telemetry, blob_database),
	} = new_partial(
		&config,
		unsafe_da_sync,
		kate_rpc_deps,
		grandpa_justification_period,
	)?;

	let metrics = N::register_notification_metrics(
		config.prometheus_config.as_ref().map(|cfg| &cfg.registry),
	);
	let shared_voter_state = rpc_setup;
	let auth_disc_publish_non_global_ips = config.network.allow_non_globals_in_dht;
	let auth_disc_public_addresses = config.network.public_addresses.clone();

	let mut net_config = sc_network::config::FullNetworkConfiguration::<_, _, N>::new(
		&config.network,
		config
			.prometheus_config
			.as_ref()
			.map(|cfg| cfg.registry.clone()),
	);

	let genesis_hash = client
		.block_hash(0)
		.ok()
		.flatten()
		.expect("Genesis block exists; qed");
	let peer_store_handle = net_config.peer_store_handle();

	let (blob_req_res_cfg, blob_req_receiver, blob_gossip_cfg, blob_gossip_service) =
		get_blob_p2p_config::<Block, N>(metrics.clone(), Arc::clone(&peer_store_handle));

	let grandpa_protocol_name =
		sc_consensus_grandpa::protocol_standard_name(&genesis_hash, &config.chain_spec);
	let (grandpa_protocol_config, grandpa_notification_service) =
		sc_consensus_grandpa::grandpa_peers_set_config::<Block, N>(
			grandpa_protocol_name.clone(),
			metrics.clone(),
			Arc::clone(&peer_store_handle),
		);
	net_config.add_notification_protocol(grandpa_protocol_config);

	// Add our blob protocols to the network config
	net_config.add_request_response_protocol(blob_req_res_cfg);
	net_config.add_notification_protocol(blob_gossip_cfg);

	let warp_sync = Arc::new(sc_consensus_grandpa::warp_proof::NetworkProvider::new(
		backend.clone(),
		import_setup.1.shared_authority_set().clone(),
		Vec::default(),
	));

	let (network, system_rpc_tx, tx_handler_controller, sync_service) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			net_config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync_config: Some(WarpSyncConfig::WithProvider(warp_sync)),
			block_relay: None,
			metrics,
		})?;

	// === Initialize blob module ===
	let blob_handle = BlobHandle::new(
		config.role.clone(),
		blob_database.clone(),
		blob_gossip_service,
		blob_req_receiver,
		network.clone(),
		client.clone(),
		keystore_container.local_keystore(),
		sync_service.clone(),
		task_manager.spawn_handle(),
		transaction_pool.clone(),
	);

	let basic_authorship_db = blob_handle.blob_database.clone();
	let rpc_transaction_pool = transaction_pool.clone();
	let rpc_backend = backend.clone();
	let blob_handle_for_rpc = blob_handle.clone();

	let overloaded_rpc_builder =
		move |spawn_handle: Arc<dyn SpawnNamed>| -> Result<RpcModule<()>, sc_service::Error> {
			let mut io = (rpc_builder)(spawn_handle)?;
			io.merge(BlobApiServer::into_rpc(BlobRpc::<_, Block, _>::new(
				blob_handle_for_rpc.clone(),
				rpc_transaction_pool.clone(),
				rpc_backend.clone(),
			)))
			.map_err(|e| sc_service::Error::Other(format!("failed to merge Blob RPC: {e}")))?;

			Ok(io)
		};
	// === END Initialize blob module ===

	task_manager.spawn_handle().spawn(
		"finality_promoter",
		Some("blob_indexing"),
		finality_promoter(client.clone(), blob_database, Duration::from_secs(3)),
	);

	if let Some(reg) = prometheus_registry.as_ref() {
		extend_metrics(reg)?;
	}

	let net_config_path = config.network.net_config_path.clone();
	let rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		config,
		backend: backend.clone(),
		client: client.clone(),
		keystore: keystore_container.keystore(),
		network: network.clone(),
		rpc_builder: Box::new(overloaded_rpc_builder),
		transaction_pool: transaction_pool.clone(),
		task_manager: &mut task_manager,
		system_rpc_tx,
		tx_handler_controller,
		sync_service: sync_service.clone(),
		telemetry: match telemetry.as_mut() {
			Some(t) => Some(t),
			None => None,
		},
	})?;

	if let Some(hwbench) = hwbench {
		sc_sysinfo::print_hwbench(&hwbench);
		match SUBSTRATE_REFERENCE_HARDWARE.check_hardware(&hwbench, false) {
			Err(err) if role.is_authority() => {
				log::warn!(
					"⚠️  The hardware does not meet the minimal requirements {} for role 'Authority'.",
					err
				);
			},
			_ => {},
		}

		if let Some(ref mut telemetry) = telemetry {
			let telemetry_handle = telemetry.handle();
			task_manager.spawn_handle().spawn(
				"telemetry_hwbench",
				None,
				sc_sysinfo::initialize_hwbench_telemetry(telemetry_handle, hwbench),
			);
		}
	}

	let (block_import, grandpa_link, babe_link) = import_setup;

	(with_startup_data)(&block_import, &babe_link);

	if let sc_service::config::Role::Authority = &role {
		let proposer = sc_basic_authorship::ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool.clone(),
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|x| x.handle()),
			basic_authorship_db,
		);

		let client_clone = client.clone();
		let slot_duration = babe_link.config().slot_duration();
		let backoff_authoring_blocks =
			Some(sc_consensus_slots::BackoffAuthoringOnFinalizedHeadLagging::default());

		let babe_config = sc_consensus_babe::BabeParams {
			keystore: keystore_container.keystore(),
			client: client.clone(),
			select_chain,
			env: proposer,
			block_import,
			sync_oracle: sync_service.clone(),
			justification_sync_link: sync_service.clone(),
			create_inherent_data_providers: move |parent, ()| {
				let client_clone = client_clone.clone();
				async move {
					let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

					let slot =
                        sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
                            *timestamp,
                            slot_duration,
                        );

					let storage_proof =
						sp_transaction_storage_proof::registration::new_data_provider(
							&*client_clone,
							&parent,
						)?;

					Ok((slot, timestamp, storage_proof))
				}
			},
			force_authoring,
			backoff_authoring_blocks,
			babe_link,
			block_proposal_slot_portion: SlotProportion::new(0.5),
			max_block_proposal_slot_portion: None,
			telemetry: telemetry.as_ref().map(|x| x.handle()),
		};

		let babe = sc_consensus_babe::start_babe(babe_config)?;
		task_manager.spawn_essential_handle().spawn_blocking(
			"babe-proposer",
			Some("block-authoring"),
			babe,
		);
	}

	// Spawn authority discovery module.
	if role.is_authority() {
		let authority_discovery_role =
			sc_authority_discovery::Role::PublishAndDiscover(keystore_container.keystore());
		let dht_event_stream =
			network
				.event_stream("authority-discovery")
				.filter_map(|e| async move {
					match e {
						Event::Dht(e) => Some(e),
						_ => None,
					}
				});
		let (authority_discovery_worker, _service) =
			sc_authority_discovery::new_worker_and_service_with_config(
				sc_authority_discovery::WorkerConfig {
					publish_non_global_ips: auth_disc_publish_non_global_ips,
					public_addresses: auth_disc_public_addresses,
					persisted_cache_directory: net_config_path,
					..Default::default()
				},
				client.clone(),
				Arc::new(network.clone()),
				Box::pin(dht_event_stream),
				authority_discovery_role,
				prometheus_registry.clone(),
				task_manager.spawn_handle(),
			);

		task_manager.spawn_handle().spawn(
			"authority-discovery-worker",
			Some("networking"),
			authority_discovery_worker.run(),
		);
	}

	// if the node isn't actively participating in consensus then it doesn't
	// need a keystore, regardless of which protocol we use below.
	let keystore = if role.is_authority() {
		Some(keystore_container.keystore())
	} else {
		None
	};

	let grandpa_config = sc_consensus_grandpa::Config {
		gossip_duration: std::time::Duration::from_millis(333),
		justification_generation_period: grandpa_justification_period,
		name: Some(name),
		observer_enabled: false,
		keystore,
		local_role: role.clone(),
		telemetry: telemetry.as_ref().map(|x| x.handle()),
		protocol_name: grandpa_protocol_name,
	};

	if enable_grandpa {
		let grandpa_config = sc_consensus_grandpa::GrandpaParams {
			config: grandpa_config,
			link: grandpa_link,
			network: network.clone(),
			notification_service: grandpa_notification_service,
			sync: Arc::new(sync_service.clone()),
			telemetry: telemetry.as_ref().map(|x| x.handle()),
			voting_rule: sc_consensus_grandpa::VotingRulesBuilder::default().build(),
			prometheus_registry,
			shared_voter_state,
			offchain_tx_pool_factory: OffchainTransactionPoolFactory::new(transaction_pool.clone()),
		};

		task_manager.spawn_essential_handle().spawn_blocking(
			"grandpa-voter",
			None,
			sc_consensus_grandpa::run_grandpa_voter(grandpa_config)?,
		);
	}

	if enable_offchain_worker {
		let offchain_workers =
			sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
				runtime_api_provider: client.clone(),
				keystore: Some(keystore_container.keystore()),
				offchain_db: backend.offchain_storage(),
				transaction_pool: Some(OffchainTransactionPoolFactory::new(
					transaction_pool.clone(),
				)),
				network_provider: Arc::new(network.clone()),
				is_validator: role.is_authority(),
				enable_http_requests: true,
				custom_extensions: move |_| vec![],
			})?;
		task_manager.spawn_handle().spawn(
			"offchain-workers-runner",
			"offchain-work",
			offchain_workers
				.run(client.clone(), task_manager.spawn_handle())
				.boxed(),
		);
	}

	Ok(NewFullBase {
		task_manager,
		client,
		network,
		sync: sync_service,
		transaction_pool,
		rpc_handlers,
	})
}

/// Builds a new service for a full client.
pub fn new_full(config: Configuration, cli: Cli) -> Result<TaskManager, ServiceError> {
	let database_path = config.database.path().map(Path::to_path_buf);
	let storage_param = cli.storage_monitor.clone();
	let kate_rpc_deps = kate_rpc::Deps {
		max_cells_size: cli.kate_max_cells_size,
		rpc_enabled: cli.kate_rpc_enabled,
		rpc_metrics_enabled: cli.kate_rpc_metrics_enabled,
	};

	let task_manager = match config.network.network_backend {
		sc_network::config::NetworkBackendType::Libp2p => {
			let task_manager = new_full_base::<sc_network::NetworkWorker<_, _>>(
				config,
				cli.no_hardware_benchmarks,
				|_, _| (),
				cli.unsafe_da_sync,
				kate_rpc_deps,
				cli.grandpa_justification_period,
			)
			.map(|NewFullBase { task_manager, .. }| task_manager)?;
			task_manager
		},
		sc_network::config::NetworkBackendType::Litep2p => {
			let task_manager = new_full_base::<sc_network::Litep2pNetworkBackend>(
				config,
				cli.no_hardware_benchmarks,
				|_, _| (),
				cli.unsafe_da_sync,
				kate_rpc_deps,
				cli.grandpa_justification_period,
			)
			.map(|NewFullBase { task_manager, .. }| task_manager)?;
			task_manager
		},
	};

	if let Some(database_path) = database_path {
		sc_storage_monitor::StorageMonitorService::try_spawn(
			storage_param,
			database_path,
			&task_manager.spawn_essential_handle(),
		)
		.map_err(|e| ServiceError::Application(e.into()))?;
	}

	Ok(task_manager)
}

fn extend_metrics(prometheus: &Registry) -> Result<(), PrometheusError> {
	use avail_observability::metrics::{AvailMetrics, AVAIL_METRICS};

	AVAIL_METRICS.get_or_try_init(|| AvailMetrics::new(prometheus))?;
	Ok(())
}
