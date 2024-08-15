use avail_rust::SDK;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944")
		.await
		.map_err(|e| e.to_string())?;

	let local_peer_id = sdk
		.rpc
		.system
		.local_peer_id()
		.await
		.map_err(|e| e.to_string())?;
	println!("LocalPeerId={:?}", local_peer_id);

	Ok(())
}
