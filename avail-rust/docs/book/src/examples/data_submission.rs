use avail_rust::{
	avail,
	block::{Block, DataSubmission},
	error::ClientError,
	Nonce::BestBlockAndTxPool,
	Options, SDK,
};

type DataSubmissionCall = avail::data_availability::calls::types::SubmitData;
type ApplicationKeyCreatedEvent = avail::data_availability::events::ApplicationKeyCreated;

pub async fn run() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;
	let online_client = &sdk.online_client;

	let account = SDK::alice()?;

	// Application Key Creation
	let key = String::from("My Key").into_bytes();
	let options = Some(Options::new().nonce(BestBlockAndTxPool));
	let tx = sdk.tx.data_availability.create_application_key(key);
	let res = tx.execute_wait_for_inclusion(&account, options).await?;

	let Some(event) = res.find_first_event::<ApplicationKeyCreatedEvent>() else {
		return Ok(());
	};
	let app_id = event.id.0;

	// Data Submission
	let data = String::from("My Data").into_bytes();
	let options = Some(Options::new().nonce(BestBlockAndTxPool).app_id(app_id));
	let tx = sdk.tx.data_availability.submit_data(data);
	let res = tx.execute_wait_for_inclusion(&account, options).await?;

	println!(
		"Block Hash: {:?}, Block Number: {}, Tx Hash: {:?}, Tx Index: {}",
		res.block_hash, res.block_number, res.tx_hash, res.tx_index
	);

	let Some(call_data) = res.get_data::<DataSubmissionCall>(online_client).await else {
		return Ok(());
	};
	println!("Call data: {:?}", call_data.data);

	// Getting Data Submission from Block #1
	let block = Block::new(online_client, res.block_hash).await?;
	let data_submissions = block.data_submissions_all();
	for ds in data_submissions {
		println!(
			"Tx Hash: {:?}, Tx Index: {}, Data {:?}, Tx Signer: {:?}, App Id: {}",
			ds.tx_hash, ds.tx_index, ds.data, ds.tx_signer, ds.app_id
		);

		println!("Ascii data: {}", ds.to_ascii().expect("qed"));
	}

	// Getting Data Submission from Block #2
	for tx in block.transaction_all_static::<DataSubmissionCall>() {
		println!("Call data: {:?}", tx.value.data);

		let ds = DataSubmission::from_static(tx);
		println!(
			"Tx Hash: {:?}, Tx Index: {}, Data {:?}, Tx Signer: {:?}, App Id: {}",
			ds.tx_hash, ds.tx_index, ds.data, ds.tx_signer, ds.app_id
		);

		println!("Ascii data: {}", ds.to_ascii().expect("qed"));
	}

	Ok(())
}
