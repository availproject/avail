use avail_rust::prelude::*;
use clap::Parser;
use da_spammer::{build_commitments, hash_blob};
use std::{
	sync::Arc,
	time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::mpsc;

/// Simple CLI for spamming blobs + metadata to an Avail node.
#[derive(Parser, Debug)]
#[command(name = "da-spammer", about = "Submit blobs + metadata to Avail")]
struct Args {
	/// One of: alice,bob,charlie,dave,eve,ferdie,one,two
	#[arg(short, long, value_parser = validate_account)]
	account: String,

	/// Payload size in MiB [1..=31] (default: 31)
	#[arg(short, long, default_value_t = 31)]
	size_mb: usize,

	/// Number of transactions [1..=1000] (default: 50)
	#[arg(long, default_value_t = 50)]
	count: usize,

	/// Single character to repeat for the blob. Default: first char of `--account`
	#[arg(long)]
	ch: Option<char>,

	/// RPC endpoint
	#[arg(short, long, default_value = "http://127.0.0.1:8546")]
	endpoint: String,

	/// Delay before first submit is done. In milliseconds
	#[arg(short, long, default_value_t = 0)]
	initial_delay: u64,

	/// Delay after first submit is done. In milliseconds
	#[arg(short, long, default_value_t = 0)]
	warmup_delay: u64,

	/// Delay after subsequent submit was done. In milliseconds (default: 0)
	#[arg(long, default_value_t = 0)]
	subsequent_delay: u64,

	/// sprinkle some random bytes (default: enabled)
	#[arg(short, long, default_value_t = false)]
	randomize_disabled: bool,

	/// Read blob bytes from disk instead of repeating a character
	#[arg(short, long, default_value = None)]
	file: Option<String>,
}

fn validate_account(s: &str) -> Result<String, String> {
	let s = s.to_lowercase();
	match s.as_str() {
		"alice" | "bob" | "charlie" | "dave" | "eve" | "ferdie" | "one" | "two" => Ok(s),
		_ => Err("must be one of: alice,bob,charlie,dave,eve,ferdie,one,two".into()),
	}
}

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
		_ => unreachable!("validated above"),
	}
}

#[tokio::main]
async fn main() -> Result<(), avail_rust::error::Error> {
	let args = Args::parse();

	if !(1..=31).contains(&args.size_mb) {
		panic!("--size-mb must be within 1..=31");
	}
	if !(1..=1000).contains(&args.count) {
		panic!("--count must be within 1..=1000");
	}
	if let Some(ch) = args.ch {
		if ch.len_utf8() != 1 {
			panic!("--ch must be a single ASCII character");
		}
	}

	let len_bytes = args.size_mb * 1024 * 1024;

	println!("========== Avail DA Spammer ==========");
	println!("Endpoint : {}", args.endpoint);
	println!("Account  : {}", args.account);
	println!("Size     : {} MiB ({} bytes)", args.size_mb, len_bytes);
	println!("Count    : {}", args.count);

	let signer = keypair_for(&args.account);

	let default_ch = args.account.chars().next().unwrap();
	let ch = args.ch.unwrap_or(default_ch);
	let byte = ch as u8;

	let mut data = if let Some(path) = args.file {
		std::fs::read(path).unwrap()
	} else {
		vec![byte; len_bytes]
	};

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

	consumer_task(clients, signer, subsequent_delay, warmup_delay, rx).await;

	Ok(())
}

async fn submit_blob_task(
	client: Client,
	metadata: Vec<u8>,
	blob: Arc<Vec<u8>>,
	blob_len: usize,
	index: usize,
	elapsed: Duration,
) {
	println!(
		"New submit block task executed after {} ms. Index: {index}",
		elapsed.as_millis()
	);
	match client
		.chain()
		.blob_submit_blob(&metadata, &blob[0..blob_len])
		.await
	{
		Ok(_) => println!("    ✓ [{}] submitted", index),
		Err(e) => eprintln!("    ✗ [{}] error: {e}", index),
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

const CLIENT_COUNT: usize = 25;
type ChannelSender = mpsc::Sender<ChannelMessage>;
type ChannelReceiver = mpsc::Receiver<ChannelMessage>;

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
	signer: Keypair,
	subsequent_delay: u64,
	warmup_delay: u64,
	mut rx: ChannelReceiver,
) {
	if warmup_delay != 0 {
		tokio::time::sleep(Duration::from_millis(warmup_delay)).await;
	}

	let account_id = signer.account_id();
	let mut nonce = clients[0]
		.chain()
		.account_nonce(account_id.clone())
		.await
		.unwrap();
	println!("AccountId: {account_id}");
	println!("Start nonce: {nonce}");

	let mut i = 0;
	let mut handles = Vec::with_capacity(100);
	let mut now = std::time::Instant::now();

	// Let's do it!
	loop {
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

		let elapsed = now.elapsed();
		if elapsed.as_millis() < subsequent_delay as u128 {
			tokio::time::sleep(Duration::from_millis(
				subsequent_delay - elapsed.as_millis() as u64,
			))
			.await;
		}

		let metadata = unsigned.sign(&signer, options).await.unwrap().encode();
		let h =
			tokio::spawn(
				async move {
					submit_blob_task(c, metadata, msg.data.clone(), msg.length, i, now.elapsed())
				}
				.await,
			);
		handles.push(h);
		now = std::time::Instant::now();

		i += 1;
		nonce += 1;
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
