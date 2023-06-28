mod democracy_external;

use anyhow::Result;
use avail_subxt::{
	api::{
		self,
		data_availability::events as DaEvent,
		runtime_types::{
			da_control::pallet::Call as DaCall, pallet_utility::pallet::Call as UtilityCall,
		},
	},
	avail::{Client, PairSigner},
	build_client, tx_send, Call, Opts,
};
use sp_keyring::AccountKeyring;
use structopt::StructOpt;
use subxt::events::StaticEvent;

const BLOCK_DIM_VALUE: u32 = 32;

/// Sets the block dimensions to default
async fn reset(client: &Client, signer: &PairSigner) -> Result<()> {
	log::info!("Resetting block dimensions for further tests");
	let block_length_update = Call::DataAvailability(DaCall::submit_block_length_proposal {
		rows: 256,
		cols: 256,
	});
	let sudo_call = api::tx().sudo().sudo(block_length_update);
	tx_send!(client, &sudo_call, signer);
	Ok(())
}

/// This example submits transactions to reduce the block dimensions
/// This should work only if the block is mostly empty (using batch or not)
/// To see logs:
/// RUST_LOG="submit_block_length_proposal_democracy=info" cargo run --example submit_block_length_proposal_democracy
#[async_std::main]
async fn main() -> Result<()> {
	pretty_env_logger::init();
	let args = Opts::from_args();
	let client = build_client(args.ws, args.validate_codegen).await?;
	let signer = PairSigner::new(AccountKeyring::Alice.pair());

	reset(&client, &signer).await?;

	// Success cases
	democracy_tx(&client, &signer).await?;
	batch_democracy_tx(&client, &signer).await?;

	Ok(())
}

/** Success cases **/
pub async fn democracy_tx(client: &Client, signer: &PairSigner) -> Result<()> {
	log::info!("1 - Democracy tx to reduce the dimensions of the block.");
	let call = Call::DataAvailability(DaCall::submit_block_length_proposal {
		rows: BLOCK_DIM_VALUE,
		cols: BLOCK_DIM_VALUE,
	});
	let checked_pallet_name = DaEvent::BlockLengthProposalSubmitted::PALLET;
	let checked_event_name = DaEvent::BlockLengthProposalSubmitted::EVENT;
	democracy_external::start_democracy_call(call, checked_pallet_name, checked_event_name).await?;
	log::info!("1 - Block Length Proposal Submitted found.");
	reset(client, signer).await?;
	Ok(())
}

pub async fn batch_democracy_tx(client: &Client, signer: &PairSigner) -> Result<()> {
	log::info!("2 - Democracy batch tx to reduce the dimensions of the block multiple times.");
	let mut calls = vec![];
	let n = 256 - BLOCK_DIM_VALUE;

	for i in 0..n {
		calls.push(Call::DataAvailability(
			DaCall::submit_block_length_proposal {
				rows: BLOCK_DIM_VALUE + n - 1 - i,
				cols: BLOCK_DIM_VALUE + n - 1 - i,
			},
		));
	}
	let call = Call::Utility(UtilityCall::batch { calls });
	let checked_pallet_name = DaEvent::BlockLengthProposalSubmitted::PALLET;
	let checked_event_name = DaEvent::BlockLengthProposalSubmitted::EVENT;
	democracy_external::start_democracy_call(call, checked_pallet_name, checked_event_name).await?;
	log::info!("2 - Block Length Proposal Submitted found.");
	reset(client, signer).await?;
	Ok(())
}
