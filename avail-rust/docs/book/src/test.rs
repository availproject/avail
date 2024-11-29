use avail_rust::{avail, error::ClientError, Nonce::BestBlockAndTxPool, Options, SDK};

type DataSubmissionCall = avail::data_availability::calls::types::SubmitData;

pub async fn run() -> Result<(), ClientError> {
	let sdk = SDK::new("wss://rpc-hex-devnet.avail.tools/ws").await?;
	let online_client = &sdk.online_client;

	let account = SDK::charlie()?;
	let data = String::from("My Data").into_bytes();
	let options = Some(Options::new().nonce(BestBlockAndTxPool).app_id(1));
	let tx = sdk.tx.data_availability.submit_data(data);
	let mut si = 0;
	let mut ei = 0;

	// Data Submission
	loop {
		println!("Preparing new Tx execution.");
		let res = tx.execute_wait_for_inclusion(&account, options).await?;
		if let Some(call_data) = res.get_data::<DataSubmissionCall>(online_client).await {
			si += 1
		} else {
			println!("Something went wrong :O");
			ei += 1
		}

		println!(
			"Tx Success. Block Number: {}. Number of success: {}, Number of Errors: {}",
			res.block_number, si, ei
		);
	}
}
