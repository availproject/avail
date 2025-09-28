use avail_blob::rpc::submit_blob_main_task;
use avail_blob::rpc::Friends;
use avail_blob::store::RocksdbBlobStore;
use avail_blob::traits::*;
use avail_blob::types::BlobNotification;
use avail_blob::utils::CommitmentQueue;
use avail_blob::utils::CommitmentQueueMessage;
use avail_rust::prelude::*;
use da_commitment::build_da_commitments::build_da_commitments;
use da_runtime::AccountId;
use da_runtime::UncheckedExtrinsic;
use divan::Bencher;
use frame_system::limits::BlockLength;
use jsonrpsee::core::async_trait;
use sc_keystore::LocalKeystore;
use sc_network::PeerId;
use sc_service::Role;
use sp_api::ApiError;
use sp_core::crypto::KeyTypeId;
use sp_core::keccak_256;
use sp_core::Encode;
use sp_core::H256;
use sp_runtime::transaction_validity::TransactionSource;
use sp_runtime::transaction_validity::TransactionValidity;
use std::sync::Arc;

fn main() {
	divan::main();
}

#[allow(dead_code)]
pub struct DummyDummy {
	keystore: Arc<LocalKeystore>,
	rx: async_channel::Receiver<BlobNotification>,
	tx: async_channel::Sender<BlobNotification>,
	queue_tx: tokio::sync::mpsc::Sender<CommitmentQueueMessage>,
}

impl DummyDummy {
	pub fn new() -> (Self, tokio::sync::mpsc::Receiver<CommitmentQueueMessage>) {
		let (tx, rx) = async_channel::bounded(10);
		let (queue_tx, queue_rx) = tokio::sync::mpsc::channel(10);

		let s = Self {
			keystore: Arc::new(LocalKeystore::in_memory()),
			rx,
			tx,
			queue_tx,
		};

		(s, queue_rx)
	}
}

impl RuntimeApiT for DummyDummy {
	fn get_blob_runtime_parameters(
		&self,
		_block_hash: H256,
	) -> Result<da_control::BlobRuntimeParameters, ApiError> {
		let mut x = da_control::BlobRuntimeParameters::default();
		x.max_transaction_validity = u64::MAX;
		Ok(x)
	}

	fn validate_transaction(
		&self,
		_at: H256,
		_source: TransactionSource,
		_uxt: UncheckedExtrinsic,
		_block_hash: H256,
	) -> Result<TransactionValidity, ApiError> {
		Ok(Ok(Default::default()))
	}

	fn get_active_validators(&self, _block_hash: H256) -> Result<Vec<AccountId>, ApiError> {
		Ok(vec![AccountId::from([0u8; 32])])
	}

	fn get_validator_from_key(
		&self,
		_at: H256,
		_id: KeyTypeId,
		_key_data: Vec<u8>,
	) -> Result<Option<AccountId>, ApiError> {
		Ok(Some(AccountId::from([0u8; 32])))
	}
}

impl ExternalitiesT for DummyDummy {
	fn client_info(&self) -> ClientInfo {
		ClientInfo::default()
	}

	fn local_peer_id(&self) -> Result<PeerId, ()> {
		Ok(PeerId::random())
	}

	fn role(&self) -> Role {
		Role::Authority
	}

	fn keystore(&self) -> std::option::Option<&Arc<LocalKeystore>> {
		Some(&self.keystore)
	}

	fn gossip_cmd_sender(&self) -> std::option::Option<&async_channel::Sender<BlobNotification>> {
		Some(&self.tx)
	}
}

impl BackendApiT for DummyDummy {
	fn storage(&self, _at: H256, _key: &[u8]) -> Result<Option<Vec<u8>>, String> {
		Ok(Some(BlockLength::default().encode()))
	}
}

#[async_trait]
impl TransactionPoolApiT for DummyDummy {
	async fn submit_one(
		&self,
		_block_hash: H256,
		_source: TransactionSource,
		_uxt: UncheckedExtrinsic,
	) -> Result<H256, String> {
		Ok(H256::default())
	}
}

impl CommitmentQueueApiT for DummyDummy {
	fn send(&self, value: CommitmentQueueMessage) -> bool {
		self.queue_tx.try_send(value).is_ok()
	}
}

fn setting_the_stage() -> (
	Vec<u8>,
	Friends,
	Vec<u8>,
	Arc<dyn CommitmentQueueApiT>,
	std::thread::JoinHandle<()>,
) {
	// Create runtime
	let runtime = tokio::runtime::Runtime::new().unwrap();

	let rc = runtime
		.block_on(async { Client::new(LOCAL_ENDPOINT).await })
		.unwrap();
	let data = std::fs::read("./../32MiB").unwrap();
	let blob_hash = H256::from(keccak_256(&data));
	let commitments = build_da_commitments(&data, 1024, 4096, Default::default());

	let avail_rust_blob_hash = avail_rust::H256::from(blob_hash.0);
	let unsigned = rc.tx().data_availability().submit_blob_metadata(
		avail_rust_blob_hash,
		data.len() as u64,
		commitments,
	);

	let tx_bytes = runtime
		.block_on(async { unsigned.sign(&alice(), Options::new(2)).await })
		.unwrap()
		.encode();

	let blob_store = RocksdbBlobStore::open("./tmp_01").unwrap();
	let blob_data_store = RocksdbBlobStore::open("./tmp_02").unwrap();
	let (dummy, queue_rx) = DummyDummy::new();
	let dummy = Arc::new(dummy);
	let friends = Friends {
		externalities: dummy.clone(),
		runtime_client: dummy.clone(),
		backend_client: dummy.clone(),
		tx_pool_client: dummy.clone(),
		blob_store: Arc::new(blob_store),
		blob_data_store: Arc::new(blob_data_store),
	};
	let t1 = std::thread::spawn(move || {
		let runtime = tokio::runtime::Runtime::new().unwrap();
		runtime.block_on(async { CommitmentQueue::run_task(queue_rx).await });
	});

	(data, friends, tx_bytes, dummy.clone(), t1)
}

#[divan::bench(max_time = 10)]
fn submit_data_benchmark(bencher: Bencher) {
	let (data, friends, metadata, queue, _t1) = setting_the_stage();
	bencher
		.with_inputs(|| {
			(
				data.clone(),
				friends.clone(),
				metadata.clone(),
				queue.clone(),
			)
		})
		.bench_local_refs(|d| {
			let runtime = tokio::runtime::Runtime::new().unwrap();
			runtime.block_on(async {
				let t = submit_blob_main_task(d.3.clone(), d.2.clone(), d.0.clone(), d.1.clone())
					.await
					.unwrap();
				t.await.unwrap();
			});
		});
}
