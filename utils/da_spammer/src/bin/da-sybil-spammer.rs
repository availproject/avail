use avail_rust::avail_rust_core::rpc::blob::submit_blob;
use avail_rust::prelude::*;
use clap::Parser;

use da_spammer::build_blob_and_commitments;

/// Multi-account spammer:
/// 1) Generates N keypairs
/// 2) Funds them via utility.batchAll( balances.transfer_keep_alive(...) )
/// 3) Loops `loops` times; each iter uses account i%N, builds commitments and submits a blob+metadata
#[derive(Parser, Debug)]
#[command(
	name = "da-sybil-spammer",
	about = "Generate accounts, batch fund, round-robin spam"
)]
struct Args {
	/// RPC endpoint
	#[arg(long, default_value = "http://127.0.0.1:8546")]
	endpoint: String,

	/// Funder account to pay the batchAll; one of: alice,bob,charlie,dave,eve,ferdie,one,two
	#[arg(long, default_value = "alice", value_parser = validate_account)]
	funder: String,

	/// Number of accounts to generate
	#[arg(long, default_value_t = 100)]
	accounts: usize,

	/// Amount to send to each account (e.g., "10").
	#[arg(long, default_value_t = 10)]
	fund_each: u128,

	/// Chunk size for batch_all to avoid oversized calls
	#[arg(long, default_value_t = 100)]
	batch_size: usize,

	/// Payload size in MiB [1..=64] per tx
	#[arg(long, default_value_t = 32)]
	size_mb: usize,

	/// Total number of blob submissions (loop count)
	#[arg(long, default_value_t = 1000)]
	loops: usize,

	/// Optional delay (ms) between tx submissions (useful to smooth load)
	#[arg(long, default_value_t = 0)]
	sleep_ms: u64,

	/// If set, use a fixed ASCII char for blobs; otherwise derive a per-account char
	#[arg(long)]
	ch: Option<char>,
}

fn validate_account(s: &str) -> Result<String, String> {
	let s = s.to_lowercase();
	match s.as_str() {
		"alice" | "bob" | "charlie" | "dave" | "eve" | "ferdie" | "one" | "two" => Ok(s),
		_ => Err("must be one of: alice,bob,charlie,dave,eve,ferdie,one,two".into()),
	}
}

/// Try to construct a Keypair from a mnemonic using the SDK helper.
/// (If your SDK exposes a different constructor, adjust here.)
fn keypair_from_mnemonic(mnemonic: &str) -> Keypair {
	Keypair::from_str(mnemonic).expect("mnemonic -> Keypair")
}

fn dev_keypair(name: &str) -> Keypair {
	match name {
		"alice" => alice(),
		"bob" => bob(),
		"charlie" => charlie(),
		"dave" => dave(),
		"eve" => eve(),
		"ferdie" => ferdie(),
		"one" => one(),
		"two" => two(),
		_ => unreachable!("validated"),
	}
}

#[tokio::main]
async fn main() -> Result<(), avail_rust::error::Error> {
	let args = Args::parse();
	if !(1..=64).contains(&args.size_mb) {
		panic!("--size-mb must be within 1..=64");
	}
	if args.accounts == 0 {
		panic!("--accounts must be > 0");
	}
	let len_bytes = args.size_mb * 1024 * 1024;
	let amount_units = args.fund_each * constants::ONE_AVAIL;

	println!("========== Avail DA Sybil Spammer ==========");
	println!("Endpoint   : {}", args.endpoint);
	println!("Funder     : {}", args.funder);
	println!("Accounts   : {}", args.accounts);
	println!("Fund each  : {}", args.fund_each);
	println!("Batch size : {}", args.batch_size);
	println!("Blob size  : {} MiB ({} bytes)", args.size_mb, len_bytes);
	println!("Loops      : {}", args.loops);

	// Connect
	let client = Client::new(&args.endpoint).await?;

	// 1) Generate N accounts (mnemonics -> Keypair)
	println!("---- Generating {} accounts ...", args.accounts);
	let mut accts: Vec<(Keypair, String)> = Vec::with_capacity(args.accounts);
	for _ in 0..args.accounts {
		/*         let (pair, phrase, _seed) = sp_core::sr25519::Pair::generate_with_phrase(None);
		// Prefer building from mnemonic (keeps SDK signing consistent)
		let kp = keypair_from_mnemonic(&phrase);
		// (sanity) ensure same public
		assert_eq!(kp.public_key().0, pair.public().0);
		accts.push((kp, phrase)); */
		todo!();
	}
	println!("✓ Generated {} accounts", accts.len());
	println!("  Example #0 SS58: {}", accts[0].0.account_id());

	// 2) Fund via utility.batchAll(transfers)
	let funder = dev_keypair(&args.funder);
	let mut funder_nonce = client.chain().account_nonce(funder.account_id()).await?;
	println!("---- Funding accounts with batchAll (nonce starts at {funder_nonce}) ...");

	// Build calls in chunks
	for (chunk_idx, chunk) in accts.chunks(args.batch_size).enumerate() {
		let mut calls = Vec::with_capacity(chunk.len());
		for (kp, _) in chunk.iter() {
			// balances::transfer_keep_alive(dest, amount_units)
			let call = client
				.tx()
				.balances()
				.transfer_keep_alive(kp.account_id(), amount_units)
				.call;
			calls.push(call);
		}
		let batch = client.tx().utility().batch_all(calls);
		batch
			.sign_and_submit(&funder, Options::default().nonce(funder_nonce))
			.await?;
		println!(
			"  → batch #{chunk_idx} with {} transfers, nonce={}",
			chunk.len(),
			funder_nonce,
		);
		funder_nonce += 1;
	}
	println!("✓ Funding submitted for {} accounts", accts.len());

	// Optional: small grace if your chain needs a moment to reflect balances
	if args.sleep_ms > 0 {
		println!("Sleeping {} ms before spamming ...", args.sleep_ms);
		tokio::time::sleep(std::time::Duration::from_millis(args.sleep_ms)).await;
	}

	// 3) Round-robin spam:
	// Track nonces per account (query once at start)
	println!("---- Fetching starting nonces for each account ...");
	let mut nonces: Vec<u32> = Vec::with_capacity(accts.len());
	for (kp, _) in accts.iter() {
		let n = client.chain().account_nonce(kp.account_id()).await?;
		nonces.push(n);
	}
	println!("✓ Nonces fetched");

	println!(
		"---- Submitting {} blobs round-robin over {} accounts ...",
		args.loops,
		accts.len()
	);
	for i in 0..args.loops {
		let idx = i % accts.len();
		let (signer, _mnemonic) = &accts[idx];
		let nonce = nonces[idx];

		// Pick content char
		let byte = if let Some(ch) = args.ch {
			ch as u8
		} else {
			// deterministic per-account letter: 'a' + (idx % 26)
			(b'a' + (idx as u8 % 26)) as u8
		};

		// Build blob + commitments
		let (blob, hash, commitments) = build_blob_and_commitments(byte, len_bytes - i);

		// app_id rotation as before
		let app_id = (i % 5) as u32;
		let options = Options::new(app_id).nonce(nonce);

		let unsigned = client.tx().data_availability().submit_blob_metadata(
			hash,
			blob.len() as u64,
			commitments,
		);

		let tx_bytes = unsigned.sign(signer, options).await.unwrap().encode();

		println!(
			"  → [{}] acct#{} ({}) nonce={} app_id={} tx_bytes={}B ...",
			i,
			idx,
			signer.account_id(),
			nonce,
			app_id,
			tx_bytes.len()
		);

		match submit_blob(&client.rpc_client, &tx_bytes, &blob).await {
			Ok(_) => {
				println!("    ✓ [{}] submitted", i);
				nonces[idx] += 1;
			},
			Err(e) => {
				eprintln!("    ✗ [{}] error: {e}", i);
			},
		}

		if args.sleep_ms > 0 {
			tokio::time::sleep(std::time::Duration::from_millis(args.sleep_ms)).await;
		}
	}

	println!("✅ Finished. Submitted {} transactions.", args.loops);
	Ok(())
}
