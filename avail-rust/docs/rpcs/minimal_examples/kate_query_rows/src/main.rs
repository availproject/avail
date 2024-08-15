use avail_rust::{Data, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944")
		.await
		.map_err(|e| e.to_string())?;

	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let data = String::from("My Awesome Data").as_bytes().to_vec();
	let data = Data { 0: data };

	let result = sdk
		.tx
		.data_availability
		.submit_data(data, WaitFor::BlockFinalization, &account)
		.await?;

	let rows = vec![0u32];
	let rpc_result = sdk
		.rpc
		.kate
		.query_rows(rows, Some(result.block_hash))
		.await
		.map_err(|e| e.to_string())?;
	println!("Rows={:?}", rpc_result);

	Ok(())
}
