use avail_rust::SDK;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944")
		.await
		.map_err(|e| e.to_string())?;

	let sync_state = sdk
		.rpc
		.system
		.sync_state()
		.await
		.map_err(|e| e.to_string())?;
	println!("SyncState={:?}", sync_state);

	Ok(())
}
