use anyhow::Result;
use avail_subxt::{
	api::{
		self,
		data_availability::events as DaEvent,
		runtime_types::{
			da_control::pallet::Call as DaCall, pallet_sudo::pallet::Call as SudoCall,
			sp_core::bounded::bounded_vec::BoundedVec, sp_weights::weight_v2::Weight,
		},
		sudo::events as SudoEvent,
		technical_committee::events as TCEvent,
	},
	avail::{Client, PairSigner, TxProgress},
	build_client, tx_asend, tx_send, Call, Opts,
};
use codec::Encode;
use derive_more::Constructor;
use futures::stream::{self, StreamExt as _, TryStreamExt as _};
use sp_core::crypto::Pair as _;
use sp_keyring::{sr25519::sr25519::Pair, AccountKeyring};
use structopt::StructOpt;
use subxt::utils::H256;

#[derive(Debug, Default, Constructor)]
pub struct IndexedProposalContext {
	index: u32,
	hash: H256,
	len: u32,
}

const BLOCK_DIM_VALUE: u32 = 32;

/// Builds a signer from `seed`.
fn signer_from_seed(seed: &str) -> PairSigner {
	let pair = Pair::from_string(&format!("//{}", seed), None).expect("Valid seed .qed");
	PairSigner::new(pair)
}

/// TC yes vote
async fn vote_yes(
	client: &Client,
	signer: &PairSigner,
	proposal: &IndexedProposalContext,
) -> Result<TxProgress> {
	let vote = api::tx()
		.technical_committee()
		.vote(proposal.hash, proposal.index, true);
	tx_asend!(client, &vote, signer).map_err(Into::into)
}

/// Sets the block dimensions to default
pub async fn reset(client: &Client, signer: &PairSigner) -> Result<()> {
	println!("Resetting block dimensions for further tests");
	let block_length_update = Call::DataAvailability(DaCall::submit_block_length_proposal {
		rows: 256,
		cols: 256,
	});
	let sudo_call = api::tx().sudo().sudo(block_length_update);
	tx_send!(client, &sudo_call, signer);
	Ok(())
}

/// Make `n` data submission transaction
pub async fn submit_data(client: &Client, signer: &PairSigner, n: u8) -> Result<()> {
	let example_data = b"X".repeat(1000).to_vec();
	let data_submission = api::tx()
		.data_availability()
		.submit_data(BoundedVec(example_data));
	for _ in 0..n {
		tx_asend!(client, &data_submission, signer)?;
	}

	Ok(())
}

/// This example submits transactions to reduce the block dimensions
/// This should work only if the block is mostly empty (using batch or not)
#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = build_client(args.ws, args.validate_codegen).await?;
	let signer = PairSigner::new(AccountKeyring::Alice.pair());

	reset(&client, &signer).await?;

	// Success cases
	simple_tx(&client, &signer).await?;
	batch_tx(&client, &signer).await?;
	// tech_committee_tx(&client, &signer).await?;

	// Fail cases
	fail_simple_tx(&client, &signer).await?;
	fail_batch_tx(&client, &signer).await?;

	Ok(())
}

/** Success cases **/
pub async fn simple_tx(client: &Client, signer: &PairSigner) -> Result<()> {
	println!("1 - Sudo call to reduce the dimensions of the block.");
	let block_length_update = Call::DataAvailability(DaCall::submit_block_length_proposal {
		rows: BLOCK_DIM_VALUE,
		cols: BLOCK_DIM_VALUE,
	});
	let sudo_call = api::tx().sudo().sudo(block_length_update);
	tx_send!(client, &sudo_call, signer)
		.find_first::<DaEvent::BlockLengthProposalSubmitted>()?
		.expect("1 - Block Length Proposal Submitted event is emitted .qed");
	println!("1 - Block Length Proposal Submitted found.");
	reset(client, signer).await?;
	Ok(())
}

pub async fn batch_tx(client: &Client, signer: &PairSigner) -> Result<()> {
	println!("2 - Sudo call in a batch to reduce the dimensions of the block.");
	let block_length_update = Call::DataAvailability(DaCall::submit_block_length_proposal {
		rows: BLOCK_DIM_VALUE,
		cols: BLOCK_DIM_VALUE,
	});
	let sudo_call = Call::Sudo(SudoCall::sudo {
		call: Box::new(block_length_update),
	});
	let batch_call = api::tx().utility().batch(vec![sudo_call]);
	tx_send!(client, &batch_call, signer)
		.find_first::<DaEvent::BlockLengthProposalSubmitted>()?
		.expect("2 - Block Length Proposal Submitted event is emitted .qed");
	println!("2 - Block Length Proposal Submitted found.");
	reset(client, signer).await?;
	Ok(())
}

// TODO: See if submit_block_length_proposal needs to work with TC, Council and/or democracy
pub async fn tech_committee_tx(client: &Client, signer: &PairSigner) -> Result<()> {
	println!("3 - TC call to reduce the dimensions of the block.");
	let block_length_update = Call::DataAvailability(DaCall::submit_block_length_proposal {
		rows: BLOCK_DIM_VALUE,
		cols: BLOCK_DIM_VALUE,
	});
	let block_length_update_len = block_length_update.encode().len() as u32;

	// Send the proposal
	let tc_call =
		api::tx()
			.technical_committee()
			.propose(5, block_length_update, block_length_update_len);
	let event = tx_send!(client, &tc_call, signer)
		.find_first::<TCEvent::Proposed>()?
		.expect("3 - TC Proposed event is emitted .qed");

	let tc_proposal = IndexedProposalContext::new(
		event.proposal_index,
		event.proposal_hash,
		block_length_update_len,
	);

	let super_majority = ["Alice", "Bob", "Charlie", "Dave", "Eve"]
		.into_iter()
		.map(signer_from_seed)
		.collect::<Vec<_>>();

	let pending_txs = stream::iter(super_majority.iter())
		.map(|tc_signer| vote_yes(client, tc_signer, &tc_proposal))
		.buffer_unordered(32)
		.try_collect::<Vec<_>>()
		.await?;

	for tx in pending_txs {
		tx.wait_for_in_block().await?;
	}

	let close_weight = Weight {
		ref_time: 1_000_000_000,
		proof_size: 0,
	};
	let close = api::tx().technical_committee().close(
		tc_proposal.hash,
		tc_proposal.index,
		close_weight,
		tc_proposal.len,
	);
	tx_send!(client, &close, signer);

	reset(client, signer).await?;
	Ok(())
}

/** Fail cases **/
pub async fn fail_simple_tx(client: &Client, signer: &PairSigner) -> Result<()> {
	println!("1-Fail - Should fail: Sudo call to reduce the dimensions of the block, after data transmission.");
	submit_data(client, signer, 2).await?;

	let block_length_update = Call::DataAvailability(DaCall::submit_block_length_proposal {
		rows: BLOCK_DIM_VALUE,
		cols: BLOCK_DIM_VALUE,
	});
	let sudo_call = api::tx().sudo().sudo(block_length_update);
	let event = tx_send!(client, &sudo_call, signer)
		.find_first::<SudoEvent::Sudid>()?
		.expect("1-Fail - Sudid event is emitted .qed");

	assert!(
		event.sudo_result.is_err(),
		"1-Fail - BlockLengthProposal was abnormally successful"
	);
	println!("1-Fail - BlockLengthProposal submission correctly failed after another tx.");

	reset(client, signer).await?;
	Ok(())
}

pub async fn fail_batch_tx(client: &Client, signer: &PairSigner) -> Result<()> {
	println!("2-Fail - Should fail: Batch call to reduce the dimensions of the block, after data transmission.");
	submit_data(client, signer, 2).await?;

	let block_length_update = Call::DataAvailability(DaCall::submit_block_length_proposal {
		rows: BLOCK_DIM_VALUE,
		cols: BLOCK_DIM_VALUE,
	});
	let sudo_call = Call::Sudo(SudoCall::sudo {
		call: Box::new(block_length_update),
	});
	let batch_call = api::tx().utility().batch(vec![sudo_call]);

	let event = tx_send!(client, &batch_call, signer)
		.find_first::<SudoEvent::Sudid>()?
		.expect("2-Fail - Sudid event is emitted .qed");

	assert!(
		event.sudo_result.is_err(),
		"2-Fail - BlockLengthProposal was abnormally successful"
	);
	println!("2-Fail - BlockLengthProposal submission correctly failed after another tx.");

	reset(client, signer).await?;
	Ok(())
}
