use std::{process::Stdio, str, time::Duration};

use anyhow::{anyhow, ensure, Result};
use jsonrpsee::ws_client::WsClientBuilder;
use structopt::StructOpt;
use tokio::{
	fs::remove_dir_all,
	process::{Child, Command},
	time::sleep,
};

use avail_subxt::rpc::*;

#[derive(Debug, StructOpt)]
pub struct E2EOpts {
	/// Chain spec for tests
	#[structopt(long, default_value = "dev")]
	pub chain: String,
	/// Path to avail-data binary
	#[structopt(long, default_value = "../target/release/data-avail")]
	pub avail_path: String,
	/// Base path for node storage
	#[structopt(long, default_value = "base/alice")]
	pub base_path: String,
	/// RPC port
	#[structopt(long, default_value = "9933")]
	pub rpc_port: u16,
	/// The WebSocket address of the target the Avail Node,
	#[structopt(name = "ws_uri", long, default_value = "ws://127.0.0.1:9944")]
	pub ws: String,
}

#[tokio::main]
async fn main() -> Result<()> {
	let args = E2EOpts::from_args();

	// Clean storage & previous chain.
	clean_environment(&args).await?;

	// Run the node as Alice.
	let mut alice_proc = run_alice_node(&args).await?;

	// Run examples using the running node.
	let examples = fetch_all_examples().await?;
	for example in examples.into_iter() {
		println!("E2E Running example {}", example);
		run_example(example, &args).await?;
	}

	// Kill alice and wait for finish.
	println!("Stopping Avail Node");
	alice_proc.kill().await?;
	alice_proc.wait().await?;
	println!("Stopped Avail Node");
	Ok(())
}

async fn run_example(example: impl AsRef<str>, _opts: &E2EOpts) -> Result<()> {
	let status = Command::new("cargo")
		.args(["run", "--release", "--example", example.as_ref()])
		.kill_on_drop(true)
		.spawn()?
		.wait()
		.await?;

	ensure!(
		status.success(),
		anyhow!("Example {} failed", example.as_ref())
	);
	Ok(())
}

async fn fetch_all_examples() -> Result<Vec<String>> {
	let cmd = Command::new("cargo")
		.args(["run", "--release", "--example"])
		.kill_on_drop(true)
		.stderr(Stdio::piped())
		.spawn()?;
	let output = cmd.wait_with_output().await?;

	// Extract examples from stderr.
	let lines = str::from_utf8(&output.stderr)?
		.lines()
		.skip(2)
		.map(str::trim)
		.filter(|line| !line.is_empty())
		.map(str::to_string)
		.collect::<Vec<_>>();
	Ok(lines)
}

/// Clean alice storage.
async fn clean_environment(opts: &E2EOpts) -> Result<()> {
	let _ = remove_dir_all(&opts.base_path).await;
	Ok(())
}

/// Run alice node for `dev`.
async fn run_alice_node(opts: &E2EOpts) -> Result<Child> {
	let args = [
		"--chain",
		opts.chain.as_ref(),
		"--base-path=base/alice",
		"--execution=NativeElseWasm",
		"--alice",
		"--port=30333",
		"--rpc-port=9944",
		"--validator",
		"--rpc-cors=all",
	];
	let child = Command::new(opts.avail_path.clone())
		.args(args)
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.kill_on_drop(true)
		.spawn()?;

	// Wait until node is ready up to 5 secs.
	for i in 1..=5 {
		if let Ok(client) = WsClientBuilder::default().build(opts.ws.clone()).await {
			if let Ok(health) = client.health().await {
				println!("Client Health: {:?}", health);
				break;
			}
		}

		ensure!(i != 5, anyhow!("Client cannot connect to `{}`", opts.ws));
		sleep(Duration::from_secs(1)).await;
	}

	Ok(child)
}
