use avail_subxt::{
	api::{
		self,
		data_availability::events as DaEvent,
		runtime_types::{
			da_control::pallet::Call as DaCall, pallet_sudo::pallet::Call as SudoCall,
		},
		sudo::events as SudoEvent,
	},
	avail::TxInBlock,
	submit::submit_data_with_nonce,
	tx, AvailClient, AvailConfig, Call, Opts,
};

use futures::stream::{FuturesOrdered, TryStreamExt as _};
use structopt::StructOpt;
use subxt::{tx::Signer as SignerT, utils::H256, Error};
use subxt_signer::sr25519::dev;

const BLOCK_ROWS: u32 = 32;
const BLOCK_COLS: u32 = 64;

fn length_proposal_call(rows: u32, cols: u32) -> Call {
	DaCall::submit_block_length_proposal { rows, cols }.into()
}
/// Sets the block dimensions to default
async fn reset<S: SignerT<AvailConfig>>(
	client: &AvailClient,
	signer: &S,
	nonce: u64,
) -> Result<u64, Error> {
	println!("Resetting block dimensions for further tests");
	let call = length_proposal_call(256, 256);

	let sudo_call = api::tx().sudo().sudo(call);
	let progress = tx::send_with_nonce(client, &sudo_call, signer, 0, nonce).await?;
	let _ = tx::then_in_block(progress).await?;

	Ok(nonce + 1)
}

#[async_std::main]
async fn main() -> Result<(), Error> {
	let args = Opts::from_args();
	let client = AvailClient::new(args.ws).await?;

	let alice = dev::alice();
	let alice_id = alice.public_key().into();
	let nonce = client.tx().account_nonce(&alice_id).await?;
	let nonce = reset(&client, &alice, nonce).await?;

	// Success cases
	let nonce = simple_tx(&client, &alice, nonce).await?;
	let nonce = batch_tx(&client, &alice, nonce).await?;

	// Fail cases
	let nonce = fail_simple_tx(&client, &alice, nonce).await?;
	let _ = fail_batch_tx(&client, &alice, nonce).await?;

	Ok(())
}

/** Success cases **/
pub async fn simple_tx<S>(client: &AvailClient, signer: &S, nonce: u64) -> Result<u64, Error>
where
	S: SignerT<AvailConfig>,
{
	println!("1 - Sudo call to reduce the dimensions of the block.");
	let call = length_proposal_call(BLOCK_ROWS, BLOCK_COLS);
	let sudo_call = api::tx().sudo().sudo(call);

	let progress = tx::send_with_nonce(client, &sudo_call, signer, 0, nonce).await?;
	let _event = tx::in_finalized(progress)
		.await?
		.fetch_events()
		.await?
		.find_first::<DaEvent::BlockLengthProposalSubmitted>()?
		.ok_or_else(|| {
			Error::Other("1 - Block Length Proposal Submitted event not emitted".to_string())
		})?;

	println!("1 - Block Length Proposal Submitted found.");
	reset(client, signer, nonce + 1).await
}

pub async fn batch_tx<S>(client: &AvailClient, signer: &S, nonce: u64) -> Result<u64, Error>
where
	S: SignerT<AvailConfig>,
{
	println!("2 - Sudo call in a batch to reduce the dimensions of the block.");
	let call = Box::new(length_proposal_call(BLOCK_ROWS, BLOCK_COLS));
	let sudo_call = SudoCall::sudo { call }.into();
	let batch_call = api::tx().utility().batch(vec![sudo_call]);

	let progress = tx::send_with_nonce(client, &batch_call, signer, 0, nonce).await?;
	let _ = tx::in_finalized(progress)
		.await?
		.fetch_events()
		.await?
		.find_first::<DaEvent::BlockLengthProposalSubmitted>()?
		.ok_or_else(|| {
			Error::Other("2 - Block Length Proposal Submitted event is emitted .qed".to_string())
		})?;

	println!("2 - Block Length Proposal Submitted found.");
	reset(client, signer, nonce + 1).await
}

/** Fail cases **/
pub async fn fail_simple_tx<S>(client: &AvailClient, signer: &S, nonce: u64) -> Result<u64, Error>
where
	S: SignerT<AvailConfig>,
{
	println!("1-Fail - Should fail: Sudo call to reduce the dimensions of the block, after data submissions.");
	let data = b"X".repeat(10_000).to_vec();

	let call = length_proposal_call(BLOCK_ROWS, BLOCK_COLS);
	let sudo_call = api::tx().sudo().sudo(call);

	let events = loop {
		let tx_1 = submit_data_with_nonce(client, signer, data.as_slice(), 2, nonce).await?;
		let tx_2 = submit_data_with_nonce(client, signer, data.as_slice(), 2, nonce + 1).await?;
		let tx_3 = tx::send_with_nonce(client, &sudo_call, signer, 0, nonce + 2).await?;

		let in_block = vec![tx_1, tx_2, tx_3]
			.into_iter()
			.map(tx::in_finalized)
			.collect::<FuturesOrdered<_>>()
			.try_collect::<Vec<_>>()
			.await?;

		let tx_blocks = in_block
			.iter()
			.map(TxInBlock::block_hash)
			.collect::<Vec<H256>>();
		println!("All tx should be in the same block: {tx_blocks:?}");

		let hash = tx_blocks[0];
		if tx_blocks.iter().all(|h| h == &hash) {
			// Ensure that the sudo call is included in the same block as the data submissions
			break in_block[2].fetch_events().await?;
		}
	};

	let event = events
		.find_first::<SudoEvent::Sudid>()?
		.ok_or_else(|| Error::Other("1-Fail - Sudid event is emitted .qed".to_string()))?;
	assert!(
		event.sudo_result.is_err(),
		"1-Fail - BlockLengthProposal was abnormally successful"
	);

	let event = events.find_first::<DaEvent::BlockLengthProposalSubmitted>()?;
	assert!(
		event.is_none(),
		"1-Fail - BlockLengthProposal was abnormally successful"
	);

	println!("1-Fail - BlockLengthProposal submission correctly failed after another tx.");
	reset(client, signer, nonce + 3).await
}

pub async fn fail_batch_tx<S>(client: &AvailClient, signer: &S, nonce: u64) -> Result<u64, Error>
where
	S: SignerT<AvailConfig>,
{
	println!("2-Fail - Should fail: Batch call to reduce the dimensions of the block, after data submissions.");
	let data = b"X".repeat(1000).to_vec();
	let _ = submit_data_with_nonce(client, signer, data.clone(), 2, nonce).await?;
	let _ = submit_data_with_nonce(client, signer, data, 2, nonce + 1).await?;

	let call = Box::new(length_proposal_call(BLOCK_ROWS, BLOCK_COLS));
	let sudo_call = SudoCall::sudo { call }.into();
	let batch_call = api::tx().utility().batch(vec![sudo_call]);

	let progress = tx::send_with_nonce(client, &batch_call, signer, 0, nonce + 2).await?;
	let events = tx::in_finalized(progress).await?.fetch_events().await?;
	let event = events
		.find_first::<SudoEvent::Sudid>()?
		.ok_or_else(|| Error::Other("2-Fail - Sudid event is emitted .qed".to_string()))?;
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

	println!("2-Fail - BlockLengthProposal submission correctly failed after another tx.");
	reset(client, signer, nonce + 3).await
}
