use avail_rust::SDK;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944")
		.await
		.map_err(|e| e.to_string())?;

	let health = sdk
		.rpc
		.system
		.health()
		.await
		.map_err(|e| e.to_string())?;
	println!("Health={:?}", health);

	Ok(())
}
