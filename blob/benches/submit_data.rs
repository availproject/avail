use crate::avail_rust_core::ExtrinsicAdditional;
use crate::avail_rust_core::ExtrinsicPayload;
use crate::avail_rust_core::GenericExtrinsic;
use avail_blob;
use avail_blob::nonce_cache::NonceCache;
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
use sp_core::crypto::AccountId32;
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

pub struct DummyNonceCache;
impl NonceCacheApiT for DummyNonceCache {
	fn check(&self, who: &AccountId32, onchain_nonce: u32, tx_nonce: u32) -> Result<(), String> {
		Ok(())
	}

	fn commit(&self, who: &AccountId32, tx_nonce: u32) {}
}

pub struct DummyRuntimeApi;
impl RuntimeApiT for DummyRuntimeApi {
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

	fn account_nonce(&self, _block_hash: H256, _who: AccountId) -> Result<u32, ApiError> {
		Ok(0)
	}
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

	fn account_nonce(&self, _block_hash: H256, _who: AccountId) -> Result<u32, ApiError> {
		Ok(0)
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

/* fn setting_the_stage() -> (
	BuildTxOutput,
	Friends,
	Vec<u8>,
	Arc<dyn CommitmentQueueApiT>,
	std::thread::JoinHandle<()>,
) {
	let data = Arc::new(std::fs::read("./../32MiB").unwrap());
	let tx = build_transaction(None, data);


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
		nonce_cache: Arc::new(NonceCache::new()),
	};
	let t1 = std::thread::spawn(move || {
		CommitmentQueue::run_task(queue_rx);
	});

	(tx, friends, tx_bytes, dummy.clone(), t1)
} */

struct BuildTxOutput {
	pub tx_bytes: Vec<u8>,
	pub data_hash: H256,
	pub commitments: Vec<u8>,
	pub data: Arc<Vec<u8>>,
}

fn build_transaction(nonce: Option<u32>, data: Arc<Vec<u8>>) -> BuildTxOutput {
	// Hash and Commitments
	let data_hash = H256::from(keccak_256(&*data));
	let commitments = build_da_commitments(&*data, 1024, 4096, Default::default());

	// Tx
	let account_id: avail_rust::AccountId = avail_rust::AccountId::from([0u8; 32]);
	let signature = [0u8; 64];

	let avail_rust_blob_hash = avail_rust::H256::from(data_hash.0);
	let call = avail::data_availability::tx::SubmitBlobMetadata {
		blob_hash: avail_rust_blob_hash,
		size: data.len() as u64,
		commitments: commitments.clone(),
	};
	let call = ExtrinsicCall::from(&call);
	let extra = ExtrinsicExtra {
		era: Default::default(),
		nonce: 0,
		tip: 0,
		app_id: 2,
	};
	let additional = ExtrinsicAdditional {
		spec_version: 0,
		tx_version: 0,
		genesis_hash: Default::default(),
		fork_hash: Default::default(),
	};

	let ext_payload = ExtrinsicPayload::new(call, extra, additional);
	let ext = GenericExtrinsic::new(account_id, signature, ext_payload);

	let avail_rust_blob_hash = avail_rust::H256::from(data_hash.0);
	BuildTxOutput {
		tx_bytes: ext.encode(),
		data_hash,
		commitments,
		data,
	}
}

mod validation {
	use super::*;

	#[divan::bench(max_time = 2)]
	fn initial_validation(bencher: Bencher) {
		let data = Arc::new(std::fs::read("./../32MiB").unwrap());
		let tx = build_transaction(None, data.clone());
		bencher.with_inputs(|| &tx).bench_local_refs(|tx| {
			avail_blob::validation::initial_validation(
				tx.data.len(),
				tx.data.as_slice(),
				&tx.tx_bytes,
			)
			.expect("Ok")
		});
	}

	#[divan::bench(max_time = 2)]
	fn keccak(bencher: Bencher) {
		let data = Arc::new(std::fs::read("./../32MiB").unwrap());
		bencher.with_inputs(|| &data).bench_local_refs(|data| {
			keccak_256(data.as_slice());
		});
	}

	#[divan::bench(max_time = 2)]
	fn tx_validation(bencher: Bencher) {
		let data = Arc::new(std::fs::read("./../32MiB").unwrap());
		let tx = build_transaction(None, data.clone());
		let dummy_runtime_api = Arc::new(DummyRuntimeApi);
		let dummy_nonce_cache = Arc::new(DummyNonceCache);

		bencher
			.with_inputs(|| {
				(
					&tx.tx_bytes,
					dummy_runtime_api.clone(),
					dummy_nonce_cache.clone(),
				)
			})
			.bench_local_refs(|params| {
				let api_dyn: Arc<dyn RuntimeApiT> = params.1.clone();
				let nonce_dyn: Arc<dyn NonceCacheApiT> = params.2.clone();

				avail_blob::validation::tx_validation(
					Default::default(),
					&params.0,
					0,
					u64::MAX,
					&api_dyn,
					&nonce_dyn,
				)
				.expect("Ok")
			});
	}
}

/* #[divan::bench(max_time = 10)]
fn submit_data_benchmark(bencher: Bencher) {
	bencher
		.with_inputs(|| {
			let (data, friends, metadata, queue, _t1) = setting_the_stage();
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
 */
