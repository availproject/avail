use avail_rust::avail_rust_core::rpc::blob::submit_blob;
use avail_rust::prelude::*;
use clap::Parser;

use da_spammer::build_blob_and_commitments;

/// Simple CLI for spamming blobs + metadata to an Avail node.
#[derive(Parser, Debug)]
#[command(name = "da-spammer", about = "Submit blobs + metadata to Avail")]
struct Args {
	/// One of: alice,bob,charlie,dave,eve,ferdie,one,two
	#[arg(long, value_parser = validate_account)]
	account: String,

	/// Payload size in MiB [1..=64] (default: 32)
	#[arg(long, default_value_t = 32)]
	size_mb: usize,

	/// Number of transactions [1..=100] (default: 50)
	#[arg(long, default_value_t = 50)]
	count: usize,

	/// Single character to repeat for the blob. Default: first char of `--account`
	#[arg(long)]
	ch: Option<char>,

	/// RPC endpoint
	#[arg(long, default_value = "http://127.0.0.1:8546")]
	endpoint: String,
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
async fn main() -> Result<(), ClientError> {
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

	let account_id = signer.account_id();
	let mut nonce = client.nonce(&account_id).await?;
	println!("AccountId: {account_id}");
	println!("Start nonce: {nonce}");

	// Precompute blobs & commitments
	println!("---- Precomputing {} blobs & commitments ...", args.count);
	let mut prepared: Vec<(Vec<u8>, H256, Vec<u8>)> = Vec::with_capacity(args.count);
	for i in 0..args.count {
		let this_len = len_bytes - i;
		let (blob, hash, commitments) = build_blob_and_commitments(byte, this_len);
		// use our prepared blob (same content) to keep prints identical to before
		println!(
			"  [{}] blob_len={}B  hash={:?}  commitments_len={}",
			i,
			blob.len(),
			hash,
			commitments.len()
		);
		prepared.push((blob, hash, commitments));
	}
	println!("✓ Precompute done");

	println!("---- Submitting {} blobs ...", prepared.len());
	for (i, (blob, hash, commitments)) in prepared.into_iter().enumerate() {
		let app_id = (i % 5) as u32;
		let options = Options::new().app_id(app_id).nonce(nonce);

		let unsigned = client.tx().data_availability().submit_blob_metadata(
			hash,
			blob.len() as u64,
			commitments,
		);

		let tx_bytes = unsigned.sign(&signer, options).await.unwrap().0.encode();

		println!(
			"  → [{}] nonce={} app_id={} tx_bytes={}B ...",
			i,
			nonce,
			app_id,
			tx_bytes.len()
		);

		match submit_blob(&client.rpc_client, tx_bytes, blob).await {
			Ok(_) => println!("    ✓ [{}] submitted", i),
			Err(e) => eprintln!("    ✗ [{}] error: {e}", i),
		}

		nonce += 1;
	}

	println!("✅ Finished. Submitted {} transactions.", args.count);
	Ok(())
}
