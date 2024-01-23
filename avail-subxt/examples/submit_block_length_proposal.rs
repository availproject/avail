use anyhow::Result;
use avail_subxt::{
	api::{
		self,
		data_availability::events as DaEvent,
		runtime_types::{
			bounded_collections::bounded_vec::BoundedVec, da_control::pallet::Call as DaCall,
			pallet_sudo::pallet::Call as SudoCall,
		},
		sudo::events as SudoEvent,
	},
	avail::{Client, PairSigner},
	build_client, tx_async_send, tx_send_in_block, tx_send_in_finalized, Call, Opts,
};
use sp_keyring::AccountKeyring;
use structopt::StructOpt;

const BLOCK_DIM_VALUE: u32 = 32;

/// Sets the block dimensions to default
async fn reset(client: &Client, signer: &PairSigner) -> Result<()> {
	log::info!("Resetting block dimensions for further tests");
	let block_length_update = Call::DataAvailability(DaCall::submit_block_length_proposal {
		rows: 256,
		cols: 256,
	});
	let sudo_call = api::tx().sudo().sudo(block_length_update);
	let _ = tx_send_in_finalized!(client, &sudo_call, signer).await;
	Ok(())
}

/// Make `n` data submission transaction
async fn submit_data(client: &Client, signer: &PairSigner, n: u8) -> Result<()> {
	let example_data = b"X".repeat(1000).to_vec();
	let data_submission = api::tx()
		.data_availability()
		.submit_data(BoundedVec(example_data));
	for _ in 0..n {
		let _ = tx_async_send!(client, &data_submission, signer);
	}

	Ok(())
}

/// This example submits transactions to reduce the block dimensions
/// This should work only if the block is mostly empty (using batch or not)
/// To see logs:
/// RUST_LOG="submit_block_length_proposal=info" cargo run --example submit_block_length_proposal
#[async_std::main]
async fn main() -> Result<()> {
	pretty_env_logger::init();
	let args = Opts::from_args();
	let (client, _) = build_client(args.ws, args.validate_codegen).await?;
	let signer = PairSigner::new(AccountKeyring::Alice.pair());

	reset(&client, &signer).await?;

	// Success cases
	simple_tx(&client, &signer).await?;
	batch_tx(&client, &signer).await?;

	// Fail cases
	fail_simple_tx(&client, &signer).await?;
	fail_batch_tx(&client, &signer).await?;

	Ok(())
}

/** Success cases **/
pub async fn simple_tx(client: &Client, signer: &PairSigner) -> Result<()> {
	log::info!("1 - Sudo call to reduce the dimensions of the block.");
	let block_length_update = Call::DataAvailability(DaCall::submit_block_length_proposal {
		rows: BLOCK_DIM_VALUE,
		cols: BLOCK_DIM_VALUE,
	});
	let sudo_call = api::tx().sudo().sudo(block_length_update);
	tx_send_in_block!(client, &sudo_call, signer)
		.fetch_events()
		.await?
		.find_first::<DaEvent::BlockLengthProposalSubmitted>()?
		.expect("1 - Block Length Proposal Submitted event is emitted .qed");
	log::info!("1 - Block Length Proposal Submitted found.");
	reset(client, signer).await?;
	Ok(())
}

pub async fn batch_tx(client: &Client, signer: &PairSigner) -> Result<()> {
	log::info!("2 - Sudo call in a batch to reduce the dimensions of the block.");
	let block_length_update = Call::DataAvailability(DaCall::submit_block_length_proposal {
		rows: BLOCK_DIM_VALUE,
		cols: BLOCK_DIM_VALUE,
	});
	let sudo_call = Call::Sudo(SudoCall::sudo {
		call: Box::new(block_length_update),
	});
	let batch_call = api::tx().utility().batch(vec![sudo_call]);
	tx_send_in_block!(client, &batch_call, signer)
		.fetch_events()
		.await?
		.find_first::<DaEvent::BlockLengthProposalSubmitted>()?
		.expect("2 - Block Length Proposal Submitted event is emitted .qed");
	log::info!("2 - Block Length Proposal Submitted found.");
	reset(client, signer).await?;
	Ok(())
}

/** Fail cases **/
pub async fn fail_simple_tx(client: &Client, signer: &PairSigner) -> Result<()> {
	log::info!("1-Fail - Should fail: Sudo call to reduce the dimensions of the block, after data transmission.");
	submit_data(client, signer, 2).await?;

	let block_length_update = Call::DataAvailability(DaCall::submit_block_length_proposal {
		rows: BLOCK_DIM_VALUE,
		cols: BLOCK_DIM_VALUE,
	});
	let sudo_call = api::tx().sudo().sudo(block_length_update);
	let events = tx_send_in_block!(client, &sudo_call, signer)
		.fetch_events()
		.await?;
	let event = events
		.find_first::<SudoEvent::Sudid>()?
		.expect("1-Fail - Sudid event is emitted .qed");
	assert!(
		event.sudo_result.is_err(),
		"1-Fail - BlockLengthProposal was abnormally successful"
	);
	assert!(
		events
			.find_first::<DaEvent::BlockLengthProposalSubmitted>()?
			.is_none(),
		"1-Fail - BlockLengthProposal was abnormally successful"
	);

	log::info!("1-Fail - BlockLengthProposal submission correctly failed after another tx.");
	reset(client, signer).await?;
	Ok(())
}

pub async fn fail_batch_tx(client: &Client, signer: &PairSigner) -> Result<()> {
	log::info!("2-Fail - Should fail: Batch call to reduce the dimensions of the block, after data transmission.");
	submit_data(client, signer, 2).await?;

	let block_length_update = Call::DataAvailability(DaCall::submit_block_length_proposal {
		rows: BLOCK_DIM_VALUE,
		cols: BLOCK_DIM_VALUE,
	});
	let sudo_call = Call::Sudo(SudoCall::sudo {
		call: Box::new(block_length_update),
	});
	let batch_call = api::tx().utility().batch(vec![sudo_call]);

	let events = tx_send_in_block!(client, &batch_call, signer)
		.fetch_events()
		.await?;
	let event = events
		.find_first::<SudoEvent::Sudid>()?
		.expect("2-Fail - Sudid event is emitted .qed");
	assert!(
		event.sudo_result.is_err(),
		"2-Fail - BlockLengthProposal was abnormally successful"
	);
	assert!(
		events
			.find_first::<DaEvent::BlockLengthProposalSubmitted>()?
			.is_none(),
		"2-Fail - BlockLengthProposal was abnormally successful"
	);

	log::info!("2-Fail - BlockLengthProposal submission correctly failed after another tx.");
	reset(client, signer).await?;
	Ok(())
}
