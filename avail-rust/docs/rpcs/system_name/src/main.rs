use avail_rust::SDK;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944")
		.await
		.map_err(|e| e.to_string())?;

	let name = sdk
		.rpc
		.system
		.name()
		.await
		.map_err(|e| e.to_string())?;
	println!("Name={:?}", name);

	Ok(())
}
