use avail_rust::{error::ClientError, SDK};

pub async fn run() -> Result<(), ClientError> {
	let sdk = SDK::new_insecure(SDK::local_endpoint()).await?;
	Ok(())
}
