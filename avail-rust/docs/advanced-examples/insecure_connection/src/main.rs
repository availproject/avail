use avail_rust::SDK;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new_insecure("ws://127.0.0.1:9944").await.unwrap();

	Ok(())
}
