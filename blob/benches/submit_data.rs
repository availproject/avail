use crate::avail_rust_core::ExtrinsicAdditional;
use crate::avail_rust_core::ExtrinsicPayload;
use crate::avail_rust_core::GenericExtrinsic;
use avail_blob;
use avail_blob::store::RocksdbBlobStore;
use avail_blob::store::StorageApiT;
use avail_blob::traits::*;
use avail_blob::types::CompressedBlob;
use avail_blob::utils::CommitmentQueue;
use avail_rust::prelude::*;
use da_commitment::build_da_commitments;
use da_commitment::build_da_commitments::build_da_commitments;
use da_runtime::AccountId;
use da_runtime::UncheckedExtrinsic;
use divan::Bencher;
use sp_api::ApiError;
use sp_core::crypto::AccountId32;
use sp_core::crypto::KeyTypeId;
use sp_core::keccak_256;
use sp_core::H256;
use sp_runtime::transaction_validity::TransactionSource;
use sp_runtime::transaction_validity::TransactionValidity;
use std::sync::Arc;
use std::sync::OnceLock;

static DATA_TO_BENCH: OnceLock<Arc<Vec<u8>>> = OnceLock::new();

fn main() -> Result<(), String> {
	read_data()?;
	divan::main();

	Ok(())
}

fn read_data() -> Result<(), String> {
	let file_path = std::env::var("FILE");
	let Ok(file_path) = file_path else {
		return Err("You must set FILE env variable to point to your file. Example: `FILE=./../rustfmt.toml cargo bench`".into());
	};

	println!("📁 Benchmarking: {}", file_path);

	// Store file for benchmarking
	let data = std::fs::read(file_path).unwrap();
	println!(
		"📁 Data Size: {} B / {} kB / {} MB",
		data.len(),
		data.len() / 1_000,
		data.len() / 1_000 / 1_000
	);

	DATA_TO_BENCH.get_or_init(move || Arc::new(data));

	Ok(())
}

pub const DEFAULT_ROWS: usize = 1024;
pub const DEFAULT_COLS: usize = 4096;

pub struct DummyNonceCache;
impl NonceCacheApiT for DummyNonceCache {
	fn check(&self, _who: &AccountId32, _onchain_nonce: u32, _tx_nonce: u32) -> Result<(), String> {
		Ok(())
	}

	fn commit(&self, _who: &AccountId32, _tx_nonce: u32) {}
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

struct BuildTxOutput {
	pub tx_bytes: Vec<u8>,
	pub data_hash: H256,
	pub commitments: Vec<u8>,
	pub data: Arc<Vec<u8>>,
}

fn build_transaction(data: Arc<Vec<u8>>) -> BuildTxOutput {
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
		let data = DATA_TO_BENCH.get().unwrap().clone();
		let tx = build_transaction(data);
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
		let data = DATA_TO_BENCH.get().unwrap().clone();
		bencher.with_inputs(|| &data).bench_local_refs(|data| {
			keccak_256(data.as_slice());
		});
	}

	#[divan::bench(max_time = 2)]
	fn tx_validation(bencher: Bencher) {
		let data = DATA_TO_BENCH.get().unwrap().clone();
		let tx = build_transaction(data);
		let runtime_dyn: Arc<dyn RuntimeApiT> = Arc::new(DummyRuntimeApi);
		let nonce_cache_dyn: Arc<dyn NonceCacheApiT> = Arc::new(DummyNonceCache);

		bencher
			.with_inputs(|| (&tx.tx_bytes, &runtime_dyn, &nonce_cache_dyn))
			.bench_local_refs(|params| {
				avail_blob::validation::tx_validation(
					Default::default(),
					&params.0,
					0,
					u64::MAX,
					&params.1,
					&params.2,
				)
				.expect("Ok")
			});
	}

	#[divan::bench(max_time = 2)]
	fn commitment_validation(bencher: Bencher) {
		let data = DATA_TO_BENCH.get().unwrap().clone();
		let tx = build_transaction(data);

		// Queue
		let (queue, rx) = CommitmentQueue::new(1);
		CommitmentQueue::spawn_background_task(rx);
		let queue: Arc<dyn CommitmentQueueApiT> = Arc::new(queue);

		// grid & Commitment
		let grid = build_da_commitments::build_polynomal_grid(
			&*tx.data,
			DEFAULT_ROWS,
			DEFAULT_COLS,
			Default::default(),
		);
		let provided_commitment = &tx.commitments;

		bencher
			.with_inputs(|| (provided_commitment, &grid, &queue))
			.bench_local_refs(|params| {
				let runtime = tokio::runtime::Runtime::new().unwrap();
				runtime.block_on(async {
					avail_blob::validation::commitment_validation(
						&params.0,
						params.1.clone(),
						&params.2,
					)
					.await
					.expect("Ok")
				});
			});
	}

	#[divan::bench(max_time = 2)]
	fn build_polynomal_grid(bencher: Bencher) {
		let data = DATA_TO_BENCH.get().unwrap().clone();
		let tx = build_transaction(data);

		bencher.with_inputs(|| &tx).bench_local_refs(|tx| {
			build_da_commitments::build_polynomal_grid(
				&*tx.data,
				DEFAULT_ROWS,
				DEFAULT_COLS,
				Default::default(),
			);
		});
	}
}

mod storage {
	use super::*;

	mod rocks_db {
		use super::*;

		#[divan::bench(max_time = 2)]
		fn compress(bencher: Bencher) {
			let data = DATA_TO_BENCH.get().unwrap().clone();
			let tx = build_transaction(data);
			bencher.with_inputs(|| &tx).bench_local_refs(|tx| {
				CompressedBlob::new_zstd_compress_with_fallback(&tx.data);
			});
		}

		#[divan::bench(max_time = 2)]
		fn insert_blob(bencher: Bencher) {
			const DB_PATH: &str = "./db_tmp_01";

			let data = DATA_TO_BENCH.get().unwrap().clone();
			let tx = build_transaction(data);
			let rocksdb = RocksdbBlobStore::open(DB_PATH).unwrap();
			let rocksdb: Arc<dyn StorageApiT> = Arc::new(rocksdb);
			let data = CompressedBlob::new_nocompression(tx.data.as_ref().clone());

			bencher
				.with_inputs(|| (&tx, &rocksdb, &data))
				.bench_local_refs(|params| {
					let (tx, rocksdb, data) = params;
					rocksdb.insert_blob(&tx.data_hash, &data).unwrap();
				});

			_ = std::fs::remove_dir_all(DB_PATH).unwrap();
		}

		#[divan::bench(max_time = 2)]
		fn insert_blob_one_third(bencher: Bencher) {
			const DB_PATH: &str = "./db_tmp_02";

			let data = DATA_TO_BENCH.get().unwrap().clone();
			let tx = build_transaction(data);
			let rocksdb = RocksdbBlobStore::open(DB_PATH).unwrap();
			let rocksdb: Arc<dyn StorageApiT> = Arc::new(rocksdb);
			let mut data = tx.data.as_ref().clone();
			data.truncate(data.len() / 3);
			let data = CompressedBlob::new_nocompression(data);

			bencher
				.with_inputs(|| (&tx, &rocksdb, &data))
				.bench_local_refs(|params| {
					let (tx, rocksdb, data) = params;
					rocksdb.insert_blob(&tx.data_hash, &data).unwrap();
				});

			_ = std::fs::remove_dir_all(DB_PATH).unwrap();
		}

		#[divan::bench(max_time = 2)]
		fn insert_blob_and_compressed(bencher: Bencher) {
			const DB_PATH: &str = "./db_tmp_03";

			let data = DATA_TO_BENCH.get().unwrap().clone();
			let tx = build_transaction(data);
			let rocksdb = RocksdbBlobStore::open(DB_PATH).unwrap();
			let rocksdb: Arc<dyn StorageApiT> = Arc::new(rocksdb);

			bencher
				.with_inputs(|| (&tx, &rocksdb))
				.bench_local_refs(|params| {
					let (tx, rocksdb) = params;
					let compressed_blob = CompressedBlob::new_zstd_compressed(&tx.data, 3).unwrap();
					rocksdb
						.insert_blob(&tx.data_hash, &compressed_blob)
						.unwrap();
				});

			_ = std::fs::remove_dir_all(DB_PATH).unwrap();
		}
	}
}
