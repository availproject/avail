use avail_rust::error::ClientError;

mod rpc;
mod storage;

#[tokio::main]
async fn main() -> Result<(), ClientError> {
	storage::run().await?;
	rpc::run().await?;

	Ok(())
}
