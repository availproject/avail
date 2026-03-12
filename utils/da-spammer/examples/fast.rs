use std::time::SystemTime;

use avail_fri::{
	B128, BytesEncoder, FriBiniusPCS, FriParamsVersion,
	eval_utils::{derive_evaluation_point, derive_seed_from_inputs, eval_claim_to_bytes},
};
use avail_rust::{Client, H256, Keypair, LOCAL_ENDPOINT, Options, StorageValue, prelude::alice};
use parking_lot::Mutex;
use sp_crypto_hashing::keccak_256;

pub struct BabeRandomness;
impl StorageValue for BabeRandomness {
	type VALUE = [u8; 32];

	const PALLET_NAME: &str = "Babe";
	const STORAGE_NAME: &str = "Randomness";
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let client = Client::connect(LOCAL_ENDPOINT).await.unwrap();
	let arena = DataArena::new();

	let handle = arena.reserve_free().unwrap();
	println!("Salting..");
	let n = std::time::Instant::now();
	arena.mutate(handle, salt_data);
	dbg!(n.elapsed());

	println!("Randomness..");
	let n = std::time::Instant::now();
	let random = current_randomness(&client).await?;
	dbg!(n.elapsed());

	println!("Prepare..");
	let n = std::time::Instant::now();
	let tx = prepare_tx(&arena, handle, random).unwrap();
	dbg!(n.elapsed());

	println!("Submit..");
	let n = std::time::Instant::now();
	submit(&client, &alice(), &arena, tx).await;
	dbg!(n.elapsed());

	Ok(())
}

async fn current_randomness(client: &Client) -> anyhow::Result<[u8; 32]> {
	BabeRandomness::fetch(&client.rpc_client, None)
		.await
		.map_err(|e| anyhow::anyhow!(e.to_string()))?
		.ok_or(anyhow::anyhow!("No Babe Randomness. Sad Story"))
}

fn salt_data(data: &mut Vec<u8>) {
	let now = SystemTime::now()
		.duration_since(SystemTime::UNIX_EPOCH)
		.unwrap()
		.as_nanos();
	data[0..16].copy_from_slice(&now.to_le_bytes());
}

pub type DataHandle = usize;
pub type Data = Vec<u8>;

#[derive(Debug)]
pub struct TxData {
	pub handle: DataHandle,
	pub blob_hash: H256,
	pub commitment: Vec<u8>,
	pub seed: [u8; 32],
	pub claim: [u8; 16],
}

fn prepare_tx(
	arena: &DataArena,
	handle: DataHandle,
	babe_randomness: [u8; 32],
) -> Result<TxData, String> {
	let blob = unsafe { &mut (*arena.get_data(handle)) };

	let blob_hash = H256::from(keccak_256(blob));

	let encoder = BytesEncoder::<B128>::new();
	let packed = encoder
		.bytes_to_packed_mle(&blob)
		.map_err(|e| format!("encode error: {e}"))?;

	let cfg = FriParamsVersion::V0.to_config(packed.total_n_vars);
	let pcs = FriBiniusPCS::new(cfg);
	let ctx = pcs
		.initialize_fri_context::<B128>(packed.packed_mle.log_len())
		.map_err(|e| format!("init fri context error: {e}"))?;

	let commit_output = pcs
		.commit(&packed.packed_mle, &ctx)
		.map_err(|e| format!("commit error: {e}"))?;

	let eval_point_seed = derive_seed_from_inputs(&babe_randomness, &blob_hash.0);
	let eval_point = derive_evaluation_point(eval_point_seed, packed.total_n_vars);
	let eval_claim = pcs
		.calculate_evaluation_claim(&packed.packed_values, &eval_point)
		.map_err(|e| format!("evaluation claim error: {e}"))?;

	let eval_claim_bytes: [u8; 16] = eval_claim_to_bytes(eval_claim)
		.try_into()
		.map_err(|_| "invalid claim byte length".to_string())?;

	Ok(TxData {
		handle,
		blob_hash,
		commitment: commit_output.commitment.to_vec(),
		seed: eval_point_seed,
		claim: eval_claim_bytes,
	})
}

#[derive(Debug)]
pub struct DataArena {
	inner: Mutex<DataArenaInner>,
}

impl DataArena {
	pub fn new() -> Self {
		Self {
			inner: Mutex::new(DataArenaInner {
				data: vec![(vec![1u8; 32 * 1000 * 1000], false)],
			}),
		}
	}
}

impl DataArena {
	pub fn reserve_free(&self) -> Option<DataHandle> {
		let mut lock = self.inner.lock();
		if let Some(x) = lock.data.iter_mut().enumerate().find(|x| x.1.1 == false) {
			x.1.1 = true;
			return Some(x.0);
		}

		None
	}

	pub fn free_reserved(&self, handle: DataHandle) {
		let mut lock = self.inner.lock();
		lock.data[handle].1 = false;
	}

	pub fn get_data(&self, handle: DataHandle) -> *mut Vec<u8> {
		let mut lock = self.inner.lock();
		&mut lock.data[handle].0
	}

	pub fn data_len(&self, handle: DataHandle) -> usize {
		let lock = self.inner.lock();
		lock.data[handle].0.len()
	}

	pub fn mutate(&self, handle: DataHandle, f: impl Fn(&mut Vec<u8>)) {
		let data: *mut Vec<u8> = {
			let mut lock = self.inner.lock();
			&mut lock.data[handle].0
		};
		unsafe { f(&mut *data) };
	}
}

#[derive(Debug)]
pub struct DataArenaInner {
	pub data: Vec<(Data, bool)>,
}

static GLOBAL_INDEX: Mutex<usize> = Mutex::new(0);
const APP_ID_SHARDS: usize = 5;

async fn submit(client: &Client, signer: &Keypair, arena: &DataArena, tx: TxData) {
	let app_id = {
		let mut l = GLOBAL_INDEX.lock();
		let app_id = *l % APP_ID_SHARDS;
		*l = *l + 1;
		app_id
	} as u32;

	let blob = unsafe { &mut (*arena.get_data(tx.handle)) };
	let result = client
		.blob()
		.submit_blob_and_blob_metadata(
			app_id,
			tx.blob_hash,
			blob.len() as u64,
			tx.commitment,
			Some(tx.seed),
			Some(tx.claim),
			&signer,
			Options::new().app_id(app_id),
			blob,
		)
		.await;

	result.unwrap();
}
