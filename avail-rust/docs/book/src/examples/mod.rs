mod batch;
mod data_submission;
mod events;
mod transactions;
mod validator;

use avail_rust::error::ClientError;

pub async fn run() -> Result<(), ClientError> {
	data_submission::run().await?;
	events::run().await?;
	transactions::run().await?;
	validator::run().await?;
	batch::run().await?;

	Ok(())
}
