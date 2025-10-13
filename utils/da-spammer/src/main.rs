use avail_rust::prelude::*;
use clap::Parser;
use da_commitment::build_da_commitments::build_da_commitments;
use sp_crypto_hashing::keccak_256;
use std::{
	collections::HashMap,
	sync::{Arc, Mutex},
	time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::mpsc;
use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;

/// Default grid (tune to your runtime)
pub const DEFAULT_ROWS: usize = 1024;
pub const DEFAULT_COLS: usize = 4096;
/// Simple CLI for spamming blobs + metadata to an Avail node.
#[derive(Parser, Debug)]
#[command(name = "da-spammer", about = "Submit blobs + metadata to Avail")]
struct Args {
	/// One of: alice,bob,charlie,dave,eve,ferdie,one,two
	#[arg(short, long, value_delimiter = ',')]
	accounts: Option<Vec<String>>,

	/// Payload size in MiB [1..=31] (default: 31)
	#[arg(short, long, default_value_t = 31)]
	size_mb: usize,

	/// Number of transactions [1..=1000] (default: 50)
	#[arg(long, default_value_t = 50)]
	count: usize,

	/// RPC endpoint
	#[arg(short, long, default_value = "http://127.0.0.1:8546")]
	endpoint: String,

	/// Delay after first submit is done. In milliseconds
	#[arg(short, long, default_value_t = 0)]
	warmup_delay: u64,

	/// Delay after subsequent submit was done. In milliseconds (default: 750)
	#[arg(long, default_value_t = 750)]
	subsequent_delay: u64,

	/// sprinkle some random bytes (default: enabled)
	#[arg(short, long, default_value_t = false)]
	randomize_disabled: bool,

	/// Read blob bytes from disk instead of repeating a character
	#[arg(short, long, default_value = None)]
	file: Option<String>,
}

// fn validate_account(s: &str) -> Result<String, String> {
// 	let s = s.to_lowercase();
// 	match s.as_str() {
// 		"alice" | "bob" | "charlie" | "dave" | "eve" | "ferdie" | "one" | "two" => Ok(s),
// 		_ => Err("must be one of: alice,bob,charlie,dave,eve,ferdie,one,two".into()),
// 	}
// }

fn keypair_for(account: &str) -> Keypair {
	match account {
		"alice" => alice(),
		"bob" => bob(),
		"charlie" => charlie(),
		"dave" => dave(),
		"eve" => eve(),
		"ferdie" => ferdie(),
		"one" => one(),
		"two" => two(),
		_ => panic!("Who is {}?", account),
	}
}

#[tokio::main]
async fn main() -> Result<(), avail_rust::error::Error> {
	// Enable logs
	_ = tracing_subscriber::fmt::SubscriberBuilder::default()
		.finish()
		.try_init();

	let args = Args::parse();

	if !(1..=31).contains(&args.size_mb) {
		panic!("--size-mb must be within 1..=31");
	}
	if !(1..=1000).contains(&args.count) {
		panic!("--count must be within 1..=1000");
	}

	let mut len_bytes = args.size_mb * 1024 * 1024;

	let mut signers = Vec::new();
	if let Some(accounts) = args.accounts {
		for account in accounts {
			let signer = keypair_for(&account);
			signers.push(signer);
		}
	}
	if signers.is_empty() {
		signers = vec![keypair_for("alice")];
	}

	// This is the main signer
	let mut data = if let Some(path) = args.file {
		std::fs::read(path).unwrap()
	} else {
		vec![0; len_bytes]
	};
	if data.len() < len_bytes {
		len_bytes = data.len();
	}

	info!("========== Avail DA Spammer ==========");
	info!("Endpoint : {}", args.endpoint);
	info!(
		"Size     : {} MiB ({} bytes)",
		len_bytes / 1024 / 1024,
		len_bytes
	);
	info!("Count    : {}", args.count);

	if !args.randomize_disabled {
		for _ in 0..10 {
			let pos = generate_random_number(len_bytes as u32 - 1);
			let value = generate_random_number(255) as u8;
			data[pos as usize] = value;
		}
	}

	// This is our OG blob. It will be shared by everyone. YAY
	let data = Arc::new(data);

	// Create Client Pool
	let mut clients = Vec::with_capacity(CLIENT_COUNT);
	for _ in 0..CLIENT_COUNT {
		let c = Client::new(&args.endpoint).await?;
		clients.push(c);
	}

	let (tx, rx) = mpsc::channel(100);

	let _ = std::thread::spawn(move || {
		producer_task(data, len_bytes, args.count, tx);
	});

	let subsequent_delay = args.subsequent_delay;
	let warmup_delay = args.warmup_delay;

	consumer_task(clients, signers, subsequent_delay, warmup_delay, rx).await;

	Ok(())
}

async fn submit_blob_task(
	client: Client,
	metadata: Vec<u8>,
	blob: Arc<Vec<u8>>,
	blob_len: usize,
	index: usize,
	time: std::time::Instant,
	account: AccountId,
	nonce: u32,
) {
	info!(
		"Ready to submit new blob. Time passed {} ms. Index: [{index}]. Account: {}, Nonce: {}",
		time.elapsed().as_millis(),
		account,
		nonce
	);
	match client
		.chain()
		.blob_submit_blob(&metadata, &blob[0..blob_len])
		.await
	{
		Ok(_) => info!("    ✓ [{}] submission done", index),
		Err(e) => info!("    ✗ [{}] error: {e}", index),
	}
}

#[derive(Debug, Clone)]
struct ChannelMessage {
	// Not needed but why not
	pub data: Arc<Vec<u8>>,
	pub length: usize,
	pub commitments: Vec<u8>,
	pub hash: H256,
}

const CLIENT_COUNT: usize = 10;
type ChannelSender = mpsc::Sender<ChannelMessage>;
type ChannelReceiver = mpsc::Receiver<ChannelMessage>;
type NonceCache = Arc<Mutex<HashMap<H256, u32>>>;

fn producer_task(data: Arc<Vec<u8>>, mut length: usize, count: usize, tx: ChannelSender) {
	// Let's do it!
	let mut i = 0;
	loop {
		let blob = &data[0..length];
		let commitments = build_commitments(blob);
		let hash = hash_blob(blob);

		let message = ChannelMessage {
			commitments,
			length,
			hash,
			data: data.clone(),
		};

		// Error means that the channel is closed
		_ = tx.blocking_send(message).unwrap();
		length -= 1;

		i += 1;
		if i >= count {
			return;
		}
	}
}

async fn consumer_task(
	clients: Vec<Client>,
	signers: Vec<Keypair>,
	subsequent_delay: u64,
	warmup_delay: u64,
	mut rx: ChannelReceiver,
) {
	if warmup_delay != 0 {
		tokio::time::sleep(Duration::from_millis(warmup_delay)).await;
	}

	let nonce_cache: NonceCache = Arc::new(Mutex::new(HashMap::new()));

	let mut i = 0;
	let mut handles = Vec::with_capacity(100);
	let mut now = std::time::Instant::now();
	let mut signer_index = 0;
	let signers_len = signers.len();

	// Let's do it!
	loop {
		let signer = signers.get(signer_index % signers_len).unwrap();
		let account_id = signer.account_id();
		let nonce = next_nonce(&nonce_cache, &clients[0], &account_id)
			.await
			.unwrap();

		// Error means that the channel is closed
		let msg = match rx.recv().await {
			Some(x) => x,
			None => break,
		};

		let app_id = (i % 5) as u32;
		let options = Options::new(app_id).nonce(nonce);
		let c = clients[i % CLIENT_COUNT].clone();

		let unsigned = c.tx().data_availability().submit_blob_metadata(
			msg.hash,
			msg.length as u64,
			msg.commitments,
		);

		let metadata = unsigned.sign(&signer, options).await.unwrap().encode();

		let elapsed = now.elapsed();
		if elapsed.as_millis() < subsequent_delay as u128 {
			tokio::time::sleep(Duration::from_millis(
				subsequent_delay - elapsed.as_millis() as u64,
			))
			.await;
		}

		let h = tokio::spawn(
			async move {
				submit_blob_task(
					c,
					metadata,
					msg.data.clone(),
					msg.length,
					i,
					now,
					account_id,
					nonce,
				)
			}
			.await,
		);
		handles.push(h);
		now = std::time::Instant::now();

		i += 1;
		signer_index += 1;
	}

	for h in handles {
		_ = h.await;
	}
}

fn generate_random_number(to: u32) -> u32 {
	let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
	let micro = now.as_micros();
	let nanos = now.as_nanos();
	let millis = now.as_millis();
	let secs = now.as_secs();

	let magic_number: u128 = micro + nanos + millis + secs as u128;
	((magic_number * 7) as u32) % to
}

/// Build a blob filled with `byte` (length `len_bytes`), its keccak256 hash,
/// and DA commitments using KZG (rows/cols + Seed::default()).
pub fn build_blob_and_commitments(byte: u8, len_bytes: usize) -> (Vec<u8>, H256, Vec<u8>) {
	let blob = vec![byte; len_bytes];
	let blob_hash = H256::from(keccak_256(&blob));
	let commitments = build_da_commitments(&blob, DEFAULT_ROWS, DEFAULT_COLS, Default::default());
	(blob, blob_hash, commitments)
}

pub fn hash_blob(data: &[u8]) -> H256 {
	H256::from(keccak_256(data))
}

pub fn build_commitments(data: &[u8]) -> Vec<u8> {
	build_da_commitments(data, DEFAULT_ROWS, DEFAULT_COLS, Default::default())
}

async fn next_nonce(
	cache: &NonceCache,
	client: &Client,
	account_id: &AccountId,
) -> Result<u32, avail_rust::error::Error> {
	let key = account_key(account_id);
	let mut map = cache.lock().unwrap();

	let nonce = match map.get_mut(&key) {
		Some(n) => {
			let current = *n;
			*n += 1;
			current
		},
		None => {
			let initial = client.chain().account_nonce(account_id.clone()).await?;
			map.insert(key, initial + 1);
			initial
		},
	};

	Ok(nonce)
}

fn account_key(account_id: &AccountId) -> H256 {
	H256::from_slice(account_id.as_ref())
}
