// #region Imports
use avail_fri::{
	core::{FriBiniusPCS, B128},
	encoding::BytesEncoder,
	eval_utils::{derive_evaluation_point, derive_seed_from_inputs, eval_claim_to_bytes},
	FriParamsVersion,
};
use avail_rust::codec::Encode;
use avail_rust::{avail_rust_core::rpc::blob::submit_blob, prelude::*};
use clap::Parser;
use rayon::ThreadPoolBuilder;
use sp_crypto_hashing::keccak_256;
use std::{
	collections::BTreeMap,
	error::Error,
	fs,
	path::PathBuf,
	sync::{
		atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
		Arc,
	},
	time::{Duration, Instant, SystemTime},
};
use tokio::{
	sync::{mpsc, Semaphore},
	task::JoinSet,
};
// #endregion

// #region Defaults
const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:8546";
const DEFAULT_SIZE_MB: usize = 31;
const MAX_SIZE_MB: usize = 31;
const MAX_PAYLOAD_BYTES: usize = MAX_SIZE_MB * 1024 * 1024;
const DEFAULT_COUNT: usize = 50;
const MAX_COUNT: usize = 1000;
const DEFAULT_IN_FLIGHT: usize = 4;
const FUND_EACH_AVAIL: u128 = 10;
const RANDOMNESS_REFRESH_SECS: u64 = 30;
const APP_ID_SHARDS: usize = 5;
// #endregion

// #region Args
#[derive(Parser, Debug)]
#[command(
	name = "da-spammer",
	about = "Single-file Avail DA spammer with optional sybil mode"
)]
struct Args {
	/// Sender account in regular mode, or funder account in sybil mode
	#[arg(long, value_parser = validate_account)]
	account: String,

	/// Payload size in MiB [1..=31]
	#[arg(long)]
	size_mb: Option<usize>,

	/// Use a file as the payload template
	#[arg(long)]
	file: Option<PathBuf>,

	/// Total submitted transactions [1..=1000]
	#[arg(long, default_value_t = DEFAULT_COUNT)]
	count: usize,

	/// RPC endpoint
	#[arg(long, default_value = DEFAULT_ENDPOINT)]
	endpoint: String,

	/// Number of prepared transactions to keep ahead; 0 = on-the-fly
	#[arg(long = "prepare", default_value_t = 0)]
	prepare: usize,

	/// Max concurrent submissions across accounts; each account stays nonce-ordered
	#[arg(long, default_value_t = DEFAULT_IN_FLIGHT)]
	in_flight: usize,

	/// Number of deterministic sybil accounts to derive and round-robin
	#[arg(long, default_value_t = 1)]
	sybil: usize,
}
// #endregion

// #region Structs
/// Babe epoch randomness
pub struct BabeRandomness;
impl StorageValue for BabeRandomness {
	type VALUE = [u8; 32];

	const PALLET_NAME: &str = "Babe";
	const STORAGE_NAME: &str = "Randomness";
}

#[derive(Debug)]
struct PreparedTx {
	index: usize,
	account_idx: usize,
	blob: Vec<u8>,
	hash: H256,
	commitment: Vec<u8>,
	seed: [u8; 32],
	claim: [u8; 16],
}

#[derive(Debug)]
struct SubmitResult {
	index: usize,
	account_idx: usize,
	nonce: u32,
	blob_len: usize,
	elapsed: Duration,
	err: Option<String>,
}

#[derive(Default)]
struct Stats {
	ok: AtomicUsize,
	failed: AtomicUsize,
	attempted: AtomicUsize,
	bytes_ok: AtomicU64,
	submit_ms_ok: AtomicU64,
}

struct SenderState {
	signer: Arc<Keypair>,
	account_id: AccountId,
	nonce: u32,
}
// #endregion

// #region Validation
fn validate_account(value: &str) -> Result<String, String> {
	let value = value.to_lowercase();
	match value.as_str() {
		"alice" | "bob" | "charlie" | "dave" | "eve" | "ferdie" | "one" | "two" => Ok(value),
		_ => Err("must be one of: alice,bob,charlie,dave,eve,ferdie,one,two".into()),
	}
}

fn validate_args(args: &Args) -> Result<(), Box<dyn Error>> {
	if args.file.is_some() && args.size_mb.is_some() {
		return Err("--file and --size-mb are mutually exclusive".into());
	}
	if let Some(size_mb) = args.size_mb {
		if !(1..=MAX_SIZE_MB).contains(&size_mb) {
			return Err(format!("--size-mb must be within 1..={MAX_SIZE_MB}").into());
		}
	}
	if args.count == 0 || args.count > MAX_COUNT {
		return Err(format!("--count must be within 1..={MAX_COUNT}").into());
	}
	if args.sybil == 0 {
		return Err("--sybil must be >= 1".into());
	}
	if args.in_flight == 0 {
		return Err("--in-flight must be > 0".into());
	}
	Ok(())
}
// #endregion

// #region Accounts
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
		_ => unreachable!("account validated"),
	}
}

fn secret_uri_root(account: &str) -> &'static str {
	match account {
		"alice" => "//Alice",
		"bob" => "//Bob",
		"charlie" => "//Charlie",
		"dave" => "//Dave",
		"eve" => "//Eve",
		"ferdie" => "//Ferdie",
		"one" => "//One",
		"two" => "//Two",
		_ => unreachable!("account validated"),
	}
}

fn derive_sybil_keypair(account: &str, index: usize) -> Result<Keypair, String> {
	let suri = format!("{}//da-spammer//{index}", secret_uri_root(account));
	let secret_uri: SecretUri = suri
		.parse()
		.map_err(|e| format!("failed to parse secret URI {suri}: {e:?}"))?;
	Keypair::from_uri(&secret_uri).map_err(|e| format!("failed to derive keypair for {suri}: {e}"))
}
// #endregion

// #region Randomness / Tx
fn make_run_salt() -> u64 {
	let nanos = SystemTime::now()
		.duration_since(SystemTime::UNIX_EPOCH)
		.map(|d| d.as_nanos() as u64)
		.unwrap_or(0);
	nanos ^ ((std::process::id() as u64) << 32)
}

fn fill_byte(run_salt: u64, unique_id: u64, account_idx: usize) -> u8 {
	let mut seed = [0_u8; 24];
	seed[..8].copy_from_slice(&run_salt.to_le_bytes());
	seed[8..16].copy_from_slice(&unique_id.to_le_bytes());
	seed[16..24].copy_from_slice(&(account_idx as u64).to_le_bytes());
	let byte = keccak_256(&seed)[0];
	if byte == 0 {
		1
	} else {
		byte
	}
}

fn mutate_file_blob(
	blob: &mut [u8],
	index: usize,
	run_salt: u64,
	unique_id: u64,
) {
	if blob.is_empty() {
		return;
	}

	if blob.len() >= 16 {
		blob[..8].copy_from_slice(&run_salt.to_le_bytes());
		blob[8..16].copy_from_slice(&unique_id.to_le_bytes());
	} else if blob.len() >= 8 {
		blob[..8].copy_from_slice(&unique_id.to_le_bytes());
	} else {
		blob[0] = blob[0].wrapping_add((index % 255) as u8);
	}
}

fn load_file_blob(path: &PathBuf) -> Result<Vec<u8>, Box<dyn Error>> {
	let mut blob = fs::read(path)?;
	if blob.is_empty() {
		return Err("--file must not be empty".into());
	}
	if blob.len() > MAX_PAYLOAD_BYTES {
		blob.truncate(MAX_PAYLOAD_BYTES);
	}
	Ok(blob)
}

fn prepare_tx(
	index: usize,
	account_idx: usize,
	len_bytes: usize,
	file_blob: Option<Arc<Vec<u8>>>,
	run_salt: u64,
	unique_id: u64,
	babe_randomness: [u8; 32],
) -> Result<PreparedTx, String> {
	let blob = if let Some(file_blob) = file_blob {
		let mut blob = (*file_blob).clone();
		mutate_file_blob(&mut blob, index, run_salt, unique_id);
		blob
	} else {
		let mut blob = vec![fill_byte(run_salt, unique_id, account_idx); len_bytes];
		if len_bytes >= 16 {
			blob[..8].copy_from_slice(&run_salt.to_le_bytes());
			blob[8..16].copy_from_slice(&unique_id.to_le_bytes());
		} else if len_bytes >= 8 {
			blob[..8].copy_from_slice(&unique_id.to_le_bytes());
		} else {
			blob[0] = blob[0].wrapping_add((index % 255) as u8);
		}
		blob
	};

	if blob.is_empty() {
		return Err("payload must not be empty".into());
	}

	let blob_hash = H256::from(keccak_256(&blob));

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

	Ok(PreparedTx {
		index,
		account_idx,
		blob,
		hash: blob_hash,
		commitment: commit_output.commitment.to_vec(),
		seed: eval_point_seed,
		claim: eval_claim_bytes,
	})
}

fn prepare_tx_with_logs(
	index: usize,
	account_idx: usize,
	len_bytes: usize,
	file_blob: Option<Arc<Vec<u8>>>,
	run_salt: u64,
	unique_id: u64,
	babe_randomness: [u8; 32],
) -> Result<PreparedTx, String> {
	let started = Instant::now();
	let result = prepare_tx(
		index,
		account_idx,
		len_bytes,
		file_blob,
		run_salt,
		unique_id,
		babe_randomness,
	);

	match &result {
		Ok(_) => println!(
			"  prep-ok tx={} acct#{} size={}B elapsed={:.2?}",
			index,
			account_idx,
			len_bytes,
			started.elapsed()
		),
		Err(_) => {},
	}

	result
}

fn is_already_imported(msg: &str) -> bool {
	msg.to_lowercase().contains("already imported")
}

fn explain_submit_error(err: String) -> String {
	if err
		.to_lowercase()
		.contains("eval_point_seed does not match derived seed")
	{
		format!("seed mismatch: tx was prepared with stale BABE randomness ({err})")
	} else {
		err
	}
}

fn format_avail(amount: u128) -> String {
	let unit = constants::ONE_AVAIL;
	let whole = amount / unit;
	let frac = amount % unit;

	if frac == 0 {
		format!("{whole}")
	} else {
		let mut frac_str = format!("{:018}", frac);
		while frac_str.ends_with('0') {
			frac_str.pop();
		}
		format!("{whole}.{frac_str}")
	}
}
// #endregion

// #region Submission
async fn submit_once(
	client: Arc<Client>,
	signer: Arc<Keypair>,
	prepared: PreparedTx,
	nonce: u32,
) -> SubmitResult {
	let started = Instant::now();
	let app_id = (prepared.index % APP_ID_SHARDS) as u32;
	println!(
		"  submit tx={} acct#{} nonce={} size={}B",
		prepared.index,
		prepared.account_idx,
		nonce,
		prepared.blob.len()
	);

	let unsigned = client.tx().data_availability().submit_blob_metadata(
		app_id,
		prepared.hash,
		prepared.blob.len() as u64,
		prepared.commitment.clone(),
		Some(prepared.seed),
		Some(prepared.claim),
	);

	let tx_bytes = match unsigned
		.sign(&signer, Options::default().app_id(app_id).nonce(nonce))
		.await
	{
		Ok(tx) => tx.encode(),
		Err(err) => {
			return SubmitResult {
				index: prepared.index,
				account_idx: prepared.account_idx,
				nonce,
				blob_len: prepared.blob.len(),
				elapsed: started.elapsed(),
				err: Some(format!("sign error: {err}")),
			};
		},
	};

	match submit_blob(&client.rpc_client, &tx_bytes, &prepared.blob).await {
		Ok(_) => {
			let elapsed = started.elapsed();
			println!(
				"  submit-ok tx={} acct#{} nonce={} size={}B elapsed={:.2?}",
				prepared.index,
				prepared.account_idx,
				nonce,
				prepared.blob.len(),
				elapsed
			);
			SubmitResult {
				index: prepared.index,
				account_idx: prepared.account_idx,
				nonce,
				blob_len: prepared.blob.len(),
				elapsed,
				err: None,
			}
		},
		Err(err) => {
			let err_s = err.to_string();
			let elapsed = started.elapsed();
			if is_already_imported(&err_s) {
				println!(
					"  submit-ok tx={} acct#{} nonce={} size={}B elapsed={:.2?} already-imported",
					prepared.index,
					prepared.account_idx,
					nonce,
					prepared.blob.len(),
					elapsed
				);
				SubmitResult {
					index: prepared.index,
					account_idx: prepared.account_idx,
					nonce,
					blob_len: prepared.blob.len(),
					elapsed,
					err: None,
				}
			} else {
				SubmitResult {
					index: prepared.index,
					account_idx: prepared.account_idx,
					nonce,
					blob_len: prepared.blob.len(),
					elapsed,
					err: Some(explain_submit_error(err_s)),
				}
			}
		},
	}
}

async fn current_randomness(client: &Client) -> Result<[u8; 32], Box<dyn Error>> {
	Ok(BabeRandomness::fetch(&client.rpc_client, None)
		.await?
		.expect("epoch randomness must exist"))
}
// #endregion

// #region Senders
async fn build_senders(
	client: &Arc<Client>,
	args: &Args,
) -> Result<Vec<SenderState>, Box<dyn Error>> {
	if args.sybil <= 1 {
		let signer = Arc::new(keypair_for(&args.account));
		let account_id = signer.public_key().to_account_id();
		let nonce = client.chain().account_nonce(account_id.clone()).await?;
		return Ok(vec![SenderState {
			signer,
			account_id,
			nonce,
		}]);
	}

	let funder = keypair_for(&args.account);
	let funder_id = funder.public_key().to_account_id();
	let mut funder_nonce = client.chain().account_nonce(funder_id.clone()).await?;

	let mut sybil_accounts = Vec::with_capacity(args.sybil);
	for index in 0..args.sybil {
		sybil_accounts.push(Arc::new(derive_sybil_keypair(&args.account, index)?));
	}

	let min_balance_units = FUND_EACH_AVAIL.saturating_mul(constants::ONE_AVAIL);
	let mut top_up_targets = Vec::new();

	println!("Derived sybil accounts:");
	for (index, signer) in sybil_accounts.iter().enumerate() {
		let account_id = signer.public_key().to_account_id();
		let balance = client.best().account_balance(account_id.clone()).await?;
		if balance.free < min_balance_units {
			println!(
				"  acct#{} {} balance={} AVAIL -> fund {} AVAIL",
				index,
				account_id,
				format_avail(balance.free),
				FUND_EACH_AVAIL
			);
			top_up_targets.push((Arc::clone(signer), min_balance_units));
		} else {
			println!(
				"  acct#{} {} balance={} AVAIL -> no funding",
				index,
				account_id,
				format_avail(balance.free)
			);
		}
	}

	if top_up_targets.is_empty() {
		println!(
			"All {} sybil accounts already have at least {} AVAIL",
			sybil_accounts.len(),
			FUND_EACH_AVAIL
		);
	} else {
		println!(
			"Topping up {} of {} sybil accounts from {} to at least {} AVAIL",
			top_up_targets.len(),
			sybil_accounts.len(),
			funder_id,
			FUND_EACH_AVAIL
		);
	}

	for (batch_idx, chunk) in top_up_targets.chunks(200).enumerate() {
		let mut calls = Vec::with_capacity(chunk.len());
		for (signer, amount) in chunk {
			let transfer = client
				.tx()
				.balances()
				.transfer_keep_alive(signer.public_key().to_account_id(), *amount)?;
			calls.push(transfer);
		}

		client
			.tx()
			.utility()
			.batch_all(calls)
			.submit(&funder, Options::default().nonce(funder_nonce))
			.await?;

		println!(
			"  funded batch {} with {} top-ups (nonce={})",
			batch_idx,
			chunk.len(),
			funder_nonce
		);
		funder_nonce = funder_nonce.saturating_add(1);
	}

	let mut senders = Vec::with_capacity(sybil_accounts.len());
	for signer in sybil_accounts {
		let account_id = signer.public_key().to_account_id();
		let nonce = client.chain().account_nonce(account_id.clone()).await?;
		senders.push(SenderState {
			signer,
			account_id,
			nonce,
		});
	}

	Ok(senders)
}
// #endregion

// #region Execution
fn record_result(stats: &Stats, result: &SubmitResult) {
	stats.attempted.fetch_add(1, Ordering::Relaxed);

	match &result.err {
		None => {
			stats.ok.fetch_add(1, Ordering::Relaxed);
			stats
				.bytes_ok
				.fetch_add(result.blob_len as u64, Ordering::Relaxed);
			stats
				.submit_ms_ok
				.fetch_add(result.elapsed.as_millis() as u64, Ordering::Relaxed);
		},
		Some(_) => {
			stats.failed.fetch_add(1, Ordering::Relaxed);
		},
	}
}

fn print_result(result: &SubmitResult) {
	if let Some(err) = &result.err {
		eprintln!(
			"  x tx={} acct#{} nonce={} err={}",
			result.index, result.account_idx, result.nonce, err
		);
	}
}

async fn dispatch_prepared(
	account_txs: &[mpsc::Sender<PreparedTx>],
	prepared: PreparedTx,
) -> Result<(), String> {
	account_txs[prepared.account_idx]
		.send(prepared)
		.await
		.map_err(|e| format!("dispatch error: {e}"))
}

async fn execute_submissions(
	args: &Args,
	client: Arc<Client>,
	senders: Vec<SenderState>,
	len_bytes: usize,
	file_blob: Option<Arc<Vec<u8>>>,
) -> Result<Arc<Stats>, Box<dyn Error>> {
	let stats = Arc::new(Stats::default());
	let cancelled = Arc::new(AtomicBool::new(false));
	let gate = Arc::new(Semaphore::new(args.in_flight));
	let mut worker_set = JoinSet::new();
	let run_salt = make_run_salt();
	let mut babe_randomness = current_randomness(&client).await?;
	let mut randomness_updated_at = Instant::now();
	let sender_count = senders.len();
	let (result_tx, mut result_rx) = mpsc::channel::<SubmitResult>(args.count.max(1));
	let mut account_txs = Vec::with_capacity(sender_count);

	for sender in senders {
		let (account_tx, mut account_rx) = mpsc::channel::<PreparedTx>(args.prepare.max(1));
		let client_ref = Arc::clone(&client);
		let gate_ref = Arc::clone(&gate);
		let stats_ref = Arc::clone(&stats);
		let result_tx_ref = result_tx.clone();
		let cancelled_ref = Arc::clone(&cancelled);
		let signer = Arc::clone(&sender.signer);
		let mut next_nonce = sender.nonce;

		worker_set.spawn(async move {
			while let Some(prepared) = account_rx.recv().await {
				if cancelled_ref.load(Ordering::Relaxed) {
					break;
				}

				let permit = match gate_ref.clone().acquire_owned().await {
					Ok(permit) => permit,
					Err(_) => break,
				};

				let _permit = permit;
				let result = submit_once(
					Arc::clone(&client_ref),
					Arc::clone(&signer),
					prepared,
					next_nonce,
				)
				.await;
				if result.err.is_none() {
					next_nonce = next_nonce.saturating_add(1);
				} else {
					cancelled_ref.store(true, Ordering::Relaxed);
				}
				record_result(&stats_ref, &result);
				let _ = result_tx_ref.send(result).await;

				if cancelled_ref.load(Ordering::Relaxed) {
					break;
				}
			}
		});
		account_txs.push(account_tx);
	}
	drop(result_tx);

	let mut dispatched = 0usize;

	if args.prepare == 0 {
		for index in 0..args.count {
			if cancelled.load(Ordering::Relaxed) {
				break;
			}

			if randomness_updated_at.elapsed() >= Duration::from_secs(RANDOMNESS_REFRESH_SECS) {
				babe_randomness = current_randomness(&client).await?;
				randomness_updated_at = Instant::now();
			}

			let account_idx = index % sender_count;
			let unique_id = index as u64;
			let prepared = match tokio::task::spawn_blocking({
				let randomness = babe_randomness;
				let file_blob = file_blob.clone();
				move || {
					prepare_tx_with_logs(
						index,
						account_idx,
						len_bytes,
						file_blob,
						run_salt,
						unique_id,
						randomness,
					)
				}
			})
			.await
			{
				Ok(Ok(prepared)) => prepared,
				Ok(Err(err)) => {
					let result = SubmitResult {
						index,
						account_idx,
						nonce: 0,
						blob_len: len_bytes,
						elapsed: Duration::ZERO,
						err: Some(format!("prepare error: {err}")),
					};
					record_result(&stats, &result);
					print_result(&result);
					cancelled.store(true, Ordering::Relaxed);
					break;
				},
				Err(err) => {
					let result = SubmitResult {
						index,
						account_idx,
						nonce: 0,
						blob_len: len_bytes,
						elapsed: Duration::ZERO,
						err: Some(format!("prepare join error: {err}")),
					};
					record_result(&stats, &result);
					print_result(&result);
					cancelled.store(true, Ordering::Relaxed);
					break;
				},
			};

			if let Err(err) = dispatch_prepared(&account_txs, prepared).await {
				let result = SubmitResult {
					index,
					account_idx,
					nonce: 0,
					blob_len: len_bytes,
					elapsed: Duration::ZERO,
					err: Some(err),
				};
				record_result(&stats, &result);
				print_result(&result);
				cancelled.store(true, Ordering::Relaxed);
				break;
			}
			dispatched += 1;
		}
	} else {
		let window = args.prepare.min(args.count);
		let mut prepare_set = JoinSet::new();
		let mut next_to_spawn = 0usize;
		let mut next_to_dispatch = 0usize;
		let mut pending = BTreeMap::new();

		while !cancelled.load(Ordering::Relaxed) && next_to_dispatch < args.count {
			while !cancelled.load(Ordering::Relaxed)
				&& next_to_spawn < args.count
				&& (next_to_spawn - next_to_dispatch) < window
			{
				if randomness_updated_at.elapsed() >= Duration::from_secs(RANDOMNESS_REFRESH_SECS) {
					babe_randomness = current_randomness(&client).await?;
					randomness_updated_at = Instant::now();
				}

				let index = next_to_spawn;
				let randomness = babe_randomness;
				let account_idx = index % sender_count;
				let unique_id = index as u64;
				let file_blob = file_blob.clone();
				prepare_set.spawn(async move {
					let prepared = tokio::task::spawn_blocking(move || {
						prepare_tx_with_logs(
							index,
							account_idx,
							len_bytes,
							file_blob,
							run_salt,
							unique_id,
							randomness,
						)
					})
					.await
					.map_err(|e| format!("prepare join error: {e}"))?;
					Ok::<(usize, Result<PreparedTx, String>), String>((index, prepared))
				});
				next_to_spawn += 1;
			}

			let Some(joined) = prepare_set.join_next().await else {
				break;
			};

			match joined {
				Ok(Ok((index, Ok(prepared)))) => {
					pending.insert(index, prepared);
				},
				Ok(Ok((index, Err(err)))) => {
					let result = SubmitResult {
						index,
						account_idx: index % sender_count,
						nonce: 0,
						blob_len: len_bytes,
						elapsed: Duration::ZERO,
						err: Some(format!("prepare error: {err}")),
					};
					record_result(&stats, &result);
					print_result(&result);
					cancelled.store(true, Ordering::Relaxed);
				},
				Ok(Err(err)) => {
					let result = SubmitResult {
						index: next_to_dispatch,
						account_idx: next_to_dispatch % sender_count,
						nonce: 0,
						blob_len: len_bytes,
						elapsed: Duration::ZERO,
						err: Some(format!("prepare task error: {err}")),
					};
					record_result(&stats, &result);
					print_result(&result);
					cancelled.store(true, Ordering::Relaxed);
				},
				Err(err) => {
					let result = SubmitResult {
						index: next_to_dispatch,
						account_idx: next_to_dispatch % sender_count,
						nonce: 0,
						blob_len: len_bytes,
						elapsed: Duration::ZERO,
						err: Some(format!("prepare worker join error: {err}")),
					};
					record_result(&stats, &result);
					print_result(&result);
					cancelled.store(true, Ordering::Relaxed);
				},
			}

			while !cancelled.load(Ordering::Relaxed) {
				let Some(prepared) = pending.remove(&next_to_dispatch) else {
					break;
				};
				if let Err(err) = dispatch_prepared(&account_txs, prepared).await {
					let result = SubmitResult {
						index: next_to_dispatch,
						account_idx: next_to_dispatch % sender_count,
						nonce: 0,
						blob_len: len_bytes,
						elapsed: Duration::ZERO,
						err: Some(err),
					};
					record_result(&stats, &result);
					print_result(&result);
					cancelled.store(true, Ordering::Relaxed);
					break;
				}
				dispatched += 1;
				next_to_dispatch += 1;
			}
		}
	}

	drop(account_txs);

	let mut received = 0usize;
	while received < dispatched {
		let Some(result) = result_rx.recv().await else {
			break;
		};
		if result.err.is_some() {
			cancelled.store(true, Ordering::Relaxed);
		}
		print_result(&result);
		received += 1;
	}

	while let Some(joined) = worker_set.join_next().await {
		if let Err(err) = joined {
			let result = SubmitResult {
				index: 0,
				account_idx: 0,
				nonce: 0,
				blob_len: len_bytes,
				elapsed: Duration::ZERO,
				err: Some(format!("worker task error: {err}")),
			};
			record_result(&stats, &result);
			print_result(&result);
			cancelled.store(true, Ordering::Relaxed);
		}
	}

	Ok(stats)
}
// #endregion

// #region Main
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let args = Args::parse();
	validate_args(&args)?;

	let threads = std::cmp::max(2, num_cpus::get() / 2);
	ThreadPoolBuilder::new()
		.num_threads(threads)
		.build_global()
		.expect("failed to init rayon");

	ctrlc::set_handler(|| {
		eprintln!("\nCtrl-C received, exiting immediately...");
		std::process::exit(130);
	})
	.expect("failed setting Ctrl-C handler");

	let file_blob = match &args.file {
		Some(path) => Some(Arc::new(load_file_blob(path)?)),
		None => None,
	};
	let len_bytes = file_blob
		.as_ref()
		.map(|blob| blob.len())
		.unwrap_or_else(|| args.size_mb.unwrap_or(DEFAULT_SIZE_MB) * 1024 * 1024);
	println!("========== Avail DA Spammer New ==========");
	println!("Endpoint      : {}", args.endpoint);
	println!("Account       : {}", args.account);
	if let Some(path) = &args.file {
		println!("Blob source   : file {}", path.display());
	} else {
		println!(
			"Blob source   : generated {} MiB payload",
			args.size_mb.unwrap_or(DEFAULT_SIZE_MB)
		);
	}
	println!("Blob size     : {} bytes", len_bytes);
	println!("Count         : {}", args.count);
	println!("Sybil         : {}", args.sybil);
	println!(
		"Prepare mode  : {}",
		if args.prepare == 0 {
			"on-the-fly".to_string()
		} else {
			format!("prepare {} ahead", args.prepare)
		}
	);
	println!("In-flight     : {}", args.in_flight);
	println!("Workers       : {}", threads);

	let client = Arc::new(Client::connect(&args.endpoint).await?);
	let senders = build_senders(&client, &args).await?;

	println!("Sender count  : {}", senders.len());
	println!("First sender  : {}", &senders[0].account_id);

	let started = Instant::now();
	let stats = execute_submissions(&args, client, senders, len_bytes, file_blob).await?;

	let attempted = stats.attempted.load(Ordering::Relaxed);
	let ok = stats.ok.load(Ordering::Relaxed);
	let failed = stats.failed.load(Ordering::Relaxed);
	let bytes_ok = stats.bytes_ok.load(Ordering::Relaxed);
	let avg_submit_ms = if ok == 0 {
		0.0
	} else {
		stats.submit_ms_ok.load(Ordering::Relaxed) as f64 / ok as f64
	};

	println!("\n==== Final Summary ====");
	println!("requested       : {}", args.count);
	println!("attempted       : {}", attempted);
	println!("success         : {}", ok);
	println!("failed          : {}", failed);
	println!("bytes submitted : {}", bytes_ok);
	println!("avg submit ms   : {:.2}", avg_submit_ms);
	println!("wall time       : {:.2?}", started.elapsed());
	println!(
		"completed       : {}",
		if ok + failed == args.count {
			"yes"
		} else {
			"no"
		}
	);
	println!("=======================");

	Ok(())
}
// #endregion
