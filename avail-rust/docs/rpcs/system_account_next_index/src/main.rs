use avail_rust::SDK;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944")
		.await
		.map_err(|e| e.to_string())?;

	let account = String::from("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");

	let account_next_index = sdk
		.rpc
		.system
		.account_next_index(account)
		.await
		.map_err(|e| e.to_string())?;
	println!("AccountNextIndex={:?}", account_next_index);

	Ok(())
}
