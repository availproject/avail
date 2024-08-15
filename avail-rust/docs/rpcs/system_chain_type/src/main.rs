use avail_rust::SDK;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944")
		.await
		.map_err(|e| e.to_string())?;

	let chain_type = sdk
		.rpc
		.system
		.chain_type()
		.await
		.map_err(|e| e.to_string())?;
	println!("ChainType={:?}", chain_type);

	Ok(())
}
