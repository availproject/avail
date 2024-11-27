mod nonce;

use avail_rust::error::ClientError;

pub async fn run() -> Result<(), ClientError> {
	nonce::run().await
}
