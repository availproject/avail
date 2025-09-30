use avail_rust::avail_rust_core::rpc::blob::submit_blob;
use avail_rust::prelude::*;
use clap::Parser;
use std::{sync::Arc, time::Duration};

use da_spammer::{build_commitments, hash_blob};

/// Simple CLI for spamming blobs + metadata to an Avail node.
#[derive(Parser, Debug)]
#[command(name = "da-spammer", about = "Submit blobs + metadata to Avail")]
struct Args {
	/// One of: alice,bob,charlie,dave,eve,ferdie,one,two
	#[arg(short, long, value_parser = validate_account)]
	account: String,

	/// Payload size in MiB [1..=64] (default: 32)
	#[arg(short, long, default_value_t = 32)]
	size_mb: usize,

	/// Number of transactions [1..=100] (default: 50)
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

	/// Delay after subsequent submit was done. In milliseconds
	#[arg(short, long, default_value_t = 0)]
	subsequent_delay: u64,
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

#[derive(Clone)]
struct SubmissionData {
	pub len: usize,
	pub hash: H256,
	pub commitments: Vec<u8>,
}

#[tokio::main]
async fn main() -> Result<(), avail_rust::error::Error> {
	let args = Args::parse();

	if !(1..=64).contains(&args.size_mb) {
		panic!("--size-mb must be within 1..=64");
	}
	if !(1..=100).contains(&args.count) {
		panic!("--count must be within 1..=100");
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

	let client = Client::new(&args.endpoint).await?;
	let signer = keypair_for(&args.account);

	let default_ch = args.account.chars().next().unwrap();
	let ch = args.ch.unwrap_or(default_ch);
	let byte = ch as u8;

	// This is our OG blob. It will be shared by everyone. YAY
	let og_blob = Arc::new(vec![byte; len_bytes]);

	let account_id = signer.account_id();
	let mut nonce = client.rpc().account_nonce(&account_id).await?;
	println!("AccountId: {account_id}");
	println!("Start nonce: {nonce}");

	// Precompute blobs & commitments
	println!("---- Precomputing {} blobs & commitments ...", args.count);
	let mut prepared: Vec<SubmissionData> = Vec::with_capacity(args.count);
	for i in 0..args.count {
		let this_len = len_bytes - i;
		let our_blob = &og_blob[0..this_len];
		let sub_data = SubmissionData {
			len: this_len,
			hash: hash_blob(our_blob),
			commitments: build_commitments(our_blob),
		};

		// use our prepared blob (same content) to keep prints identical to before
		println!(
			"  [{}] blob_len={}B  hash={:?}  commitments_len={}",
			i,
			our_blob.len(),
			sub_data.hash,
			sub_data.commitments.len()
		);
		prepared.push(sub_data);
	}
	println!("✓ Precompute done");

	// Create Client Pool
	const CLIENT_COUNT: usize = 20;
	let mut clients = Vec::with_capacity(CLIENT_COUNT);
	for _ in 0..CLIENT_COUNT {
		clients.push(Client::new(&args.endpoint).await?);
	}

	let initial_delay = args.initial_delay;
	let warmup_delay = args.warmup_delay;
	let subsequent_delay = args.subsequent_delay;

	let mut threads = Vec::new();
	println!("---- Submitting {} blobs ...", prepared.len());
	for (i, sub_data) in prepared.into_iter().enumerate() {
		let app_id = (i % 5) as u32;
		let options = Options::new(app_id).nonce(nonce);

		let unsigned = client.tx().data_availability().submit_blob_metadata(
			sub_data.hash,
			sub_data.len as u64,
			sub_data.commitments,
		);

		let tx_bytes = unsigned.sign(&signer, options).await.unwrap().encode();

		println!(
			"  → [{}] nonce={} app_id={} tx_bytes={}B ...",
			i,
			nonce,
			app_id,
			tx_bytes.len()
		);

		let t_handle = {
			let client = clients[i & CLIENT_COUNT].clone();
			let blob = og_blob.clone();
			tokio::spawn(async move {
				{
					let mut sleep_duration = Duration::from_millis(initial_delay);
					if i >= 1 {
						sleep_duration += Duration::from_millis(warmup_delay)
					}
					if i > 1 {
						sleep_duration += Duration::from_millis(subsequent_delay)
					}

					if !sleep_duration.is_zero() {
						tokio::time::sleep(sleep_duration).await;
					}

					submit_blob_task(client, tx_bytes, blob, sub_data.len, i).await
				}
			})
		};
		threads.push(t_handle);

		nonce += 1;
	}

	for t in threads {
		t.await.unwrap();
	}

	println!("✅ Finished. Submitted {} transactions.", args.count);
	Ok(())
}

async fn submit_blob_task(
	client: Client,
	metadata: Vec<u8>,
	blob: Arc<Vec<u8>>,
	blob_len: usize,
	index: usize,
) {
	match submit_blob(&client.rpc_client, &metadata, &blob[0..blob_len]).await {
		Ok(_) => println!("    ✓ [{}] submitted", index),
		Err(e) => eprintln!("    ✗ [{}] error: {e}", index),
	}
}
