mod examples;
mod rpc;
mod storage;

use avail_rust::error::ClientError;

#[tokio::main]
async fn main() -> Result<(), ClientError> {
	storage::run().await?;
	rpc::run().await?;
	examples::run().await?;

	Ok(())
}
