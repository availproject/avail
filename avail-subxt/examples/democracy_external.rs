use std::time::Duration;

use anyhow::{bail, ensure, Result};
use avail_subxt::{
	api::{
		self,
		council::events as CouncilEvent,
		democracy::events as DemocracyEvent,
		preimage::events as PreimageEvent,
		runtime_types::{
			frame_system::pallet::Call as SystemCall,
			pallet_democracy::{pallet::Call as DemocracyCall, vote::AccountVote},
			sp_weights::weight_v2::Weight,
		},
		scheduler::events as SchedulerEvent,
		system::events as SystemEvent,
		technical_committee::events as TechComEvent,
	},
	avail::{Bounded, Client, PairSigner, TxProgress},
	build_client, tx_asend, tx_send, Call, Opts,
};
use codec::Encode;
use derive_more::Constructor;
use futures::stream::{self, StreamExt as _, TryStreamExt as _};
use sp_core::crypto::Pair as _;
use sp_keyring::{
	sr25519::sr25519::Pair,
	AccountKeyring::{Alice, Bob, Charlie, Dave, Eve, Ferdie},
};
use structopt::StructOpt;
use subxt::utils::H256;

#[rustfmt::skip]
pub mod constants {
	pub const COUNCIL_SUPER_MAJORITY: [&'static str; 9] = [ "Alice", "Bob", "Charlie", "Dave", "Eve", "Ferdie", "Alice//stash", "Bob//stash", "Charlie//stash"];
	pub const THREASHOLD: u32 = 2;
}

#[derive(Debug, Default, Constructor)]
pub struct IndexedProposalContext {
	index: u32,
	hash: H256,
	len: u32,
}

#[derive(Debug, Default, Constructor)]
pub struct ProposalContext {
	hash: H256,
	#[allow(dead_code)]
	len: u32,
}

/// Builds a signer from `seed`.
fn signer_from_seed(seed: &str) -> PairSigner {
	let pair = Pair::from_string(&format!("//{}", seed), None).expect("Valid seed .qed");
	PairSigner::new(pair)
}

mod council {
	use super::{constants::*, *};

	pub async fn create_external_proposal(
		client: &Client,
	) -> Result<(IndexedProposalContext, ProposalContext)> {
		log::trace!("Creating the external proposal ...");
		let signer = PairSigner::new(Alice.pair());
		let remark = b"Proposal Done".to_vec();

		// 1. Push pre-image of inner call
		let inner_call = Call::System(SystemCall::remark_with_event { remark }).encode();
		let inner_call_len = inner_call.len() as u32;
		let note_preimage = api::tx().preimage().note_preimage(inner_call);

		let noted_event = tx_send!(client, &note_preimage, &signer)
			.find_first::<PreimageEvent::Noted>()?
			.expect("Pre Image Noted event is emitted .qed");

		log::info!("Inner call pre-image hash {}", noted_event.hash);

		// 2. Push the coucil proposal: external propose on democracy.
		let proposal = Bounded::Lookup {
			hash: noted_event.hash,
			len: inner_call_len,
		};
		let democracy_external_prop =
			Call::Democracy(DemocracyCall::external_propose_majority { proposal });
		let len_bound = democracy_external_prop.encode().len() as u32;
		let council_proposal =
			api::tx()
				.council()
				.propose(THREASHOLD, democracy_external_prop, len_bound);

		let event = tx_send!(client, &council_proposal, &signer)
			.find_first::<CouncilEvent::Proposed>()?
			.expect("Council proposed event is emitted .qed");
		let context = (
			IndexedProposalContext::new(event.proposal_index, event.proposal_hash, len_bound),
			ProposalContext::new(noted_event.hash, inner_call_len),
		);
		log::info!("New council proposal {:?}", context);

		Ok(context)
	}

	async fn vote_yes(
		client: &Client,
		signer: &PairSigner,
		proposal: &IndexedProposalContext,
	) -> Result<TxProgress> {
		let vote = api::tx()
			.council()
			.vote(proposal.hash, proposal.index, true);
		tx_asend!(client, &vote, signer).map_err(Into::into)
	}

	pub async fn simple_majority_of_vote_yes(
		client: &Client,
		proposal: &IndexedProposalContext,
	) -> Result<()> {
		log::trace!("Council will vote proposal using 3/4 of council ...");

		let super_majority = COUNCIL_SUPER_MAJORITY
			.into_iter()
			.map(signer_from_seed)
			.collect::<Vec<_>>();

		let pending_txs = stream::iter(super_majority.iter())
			.map(|signer| vote_yes(client, signer, proposal))
			.buffer_unordered(32)
			.try_collect::<Vec<_>>()
			.await?;

		for tx in pending_txs {
			log::trace!(
				"Waiting for `Ayes` voting finalization of {}",
				tx.extrinsic_hash()
			);
			let _ = tx.wait_for_in_block().await?;
		}
		Ok(())
	}

	pub async fn close(client: &Client, proposal: &IndexedProposalContext) -> Result<()> {
		log::trace!("Closing the council proposal ...");

		// TODO Calculate in advance the real weight.
		let close_weight = Weight {
			ref_time: 1_000_000_000,
			proof_size: 0,
		};
		let close =
			api::tx()
				.council()
				.close(proposal.hash, proposal.index, close_weight, proposal.len);
		let events = tx_send!(client, &close, &PairSigner::new(Alice.pair()));

		ensure!(
			events.has::<CouncilEvent::Closed>()?,
			"Proposal was not closed"
		);
		ensure!(
			events.has::<CouncilEvent::Approved>()?,
			"Proposal was not approved"
		);

		let executed = events
			.find_first::<CouncilEvent::Executed>()?
			.expect("An approved proposal is always executed .qed");
		ensure!(executed.result.is_ok(), "Council proposal fails");

		log::info!(
			"Proposal {} has been dispatched: {:?}",
			executed.proposal_hash,
			executed.result
		);

		Ok(())
	}
}

mod techies {
	use super::*;

	pub async fn create_fast_track_proposal(
		client: &Client,
		proposal: &ProposalContext,
	) -> Result<IndexedProposalContext> {
		let signer = PairSigner::new(Alice.pair());
		let fast_track = Call::Democracy(DemocracyCall::fast_track {
			proposal_hash: proposal.hash,
			voting_period: 3,
			delay: 1,
		});
		let fast_track_len = fast_track.encode().len() as u32;

		let call = api::tx()
			.technical_committee()
			.propose(5, fast_track, fast_track_len);

		let event = tx_send!(client, &call, &signer)
			.find_first::<TechComEvent::Proposed>()?
			.expect("Techies proposed event is emitted .qed");

		Ok(IndexedProposalContext::new(
			event.proposal_index,
			event.proposal_hash,
			fast_track_len,
		))
	}

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

	pub async fn super_majority_votes_yes(
		client: &Client,
		proposal: &IndexedProposalContext,
	) -> Result<()> {
		let techies = [Alice, Bob, Charlie, Dave, Eve, Ferdie]
			.into_iter()
			.map(|acc| PairSigner::new(acc.pair()))
			.collect::<Vec<_>>();

		let txs = stream::iter(techies.iter())
			.map(|signer| vote_yes(client, signer, proposal))
			.buffer_unordered(32)
			.try_collect::<Vec<_>>()
			.await?;

		for tx in txs {
			log::trace!(
				"Waiting for `Ayes` voting finalization of {}",
				tx.extrinsic_hash()
			);
			let _ = tx.wait_for_in_block().await?;
		}
		Ok(())
	}

	pub async fn close(client: &Client, proposal: &IndexedProposalContext) -> Result<u32> {
		// TODO Calculate in advance the real weight.
		let close_weight = Weight {
			ref_time: 1_000_000_000,
			proof_size: 0,
		};
		let close = api::tx().technical_committee().close(
			proposal.hash,
			proposal.index,
			close_weight,
			proposal.len,
		);
		let events = tx_send!(client, &close, &PairSigner::new(Alice.pair()));
		ensure!(
			events.has::<TechComEvent::Approved>()?,
			"Fast-track proposal was not approved"
		);
		ensure!(
			events.has::<TechComEvent::Executed>()?,
			"Fast-track proposal was not executed"
		);
		ensure!(
			events.has::<DemocracyEvent::Started>()?,
			"Referendum was not started"
		);
		let referendum_started = events
			.find_first::<DemocracyEvent::Started>()?
			.expect("Referendum was started .qed");
		log::info!("Referendum started {:?}", referendum_started);

		Ok(referendum_started.ref_index)
	}
}

mod democracy {

	use async_std::future;
	use avail_subxt::{
		avail::AVL,
		helpers::democracy::{Conviction, Vote},
	};
	use futures::FutureExt as _;

	use super::*;

	pub async fn vote_yes(client: &Client, referendum: u32) -> Result<()> {
		let signer = PairSigner::new(Alice.pair());
		let vote = AccountVote::Standard {
			vote: Vote::new(true, Conviction::Locked1x).into(),
			balance: 1_000 * AVL,
		};
		let vote_call = api::tx().democracy().vote(referendum, vote);

		let events = tx_send!(client, &vote_call, &signer);
		ensure!(
			events.has::<DemocracyEvent::Voted>()?,
			"Referendum's Vote failed"
		);

		Ok(())
	}

	pub async fn wait_passed_and_dispatch_or_timeout(
		client: &Client,
		referendum: u32,
		duration: Duration,
	) -> Result<()> {
		future::timeout(
			duration,
			wait_passed_and_dispatch(client, referendum).fuse(),
		)
		.await??;
		Ok(())
	}

	pub async fn wait_passed_and_dispatch(client: &Client, referendum: u32) -> Result<()> {
		let mut block_sub = client.blocks().subscribe_finalized().await?;

		while let Some(block) = block_sub.next().await {
			let events = block?.events().await?;

			for event in events.iter() {
				let event = event?;

				if let Some(event) = event.as_event::<SystemEvent::RemarkedByRoot>()? {
					log::info!("Remarked by Root with hash {}", event.hash);
					return Ok(());
				} else if let Some(event) = event.as_event::<DemocracyEvent::Passed>()? {
					log::trace!("Democracy Referendum {} passed", event.ref_index);
				} else if let Some(event) = event.as_event::<SchedulerEvent::Scheduled>()? {
					log::trace!("Referendum {} was scheduled on {}", event.index, event.when);
				} else if let Some(event) = event.as_event::<SchedulerEvent::Dispatched>()? {
					log::trace!(
						"Referendum {referendum} was dispatched at {}: {:?}",
						event.task.0,
						event.result
					);
				}
			}
		}

		bail!("Block subscription fails")
	}
}

/// # Trace
/// Use `RUST_LOG="democracy_external=trace"` to see traces.
#[async_std::main]
async fn main() -> Result<()> {
	use crate::{council, democracy, techies};

	pretty_env_logger::init();
	let args = Opts::from_args();
	let client = build_client(args.ws, args.validate_codegen).await?;

	// In Council,  create, approve and execute ...
	let (council_proposal, proposal) = council::create_external_proposal(&client).await?;
	council::simple_majority_of_vote_yes(&client, &council_proposal).await?;
	council::close(&client, &council_proposal).await?;

	// The technical committee do a fast-track of the proposal.
	let tech_proposal = techies::create_fast_track_proposal(&client, &proposal).await?;
	techies::super_majority_votes_yes(&client, &tech_proposal).await?;
	let referendum_index = techies::close(&client, &tech_proposal).await?;

	// In democracy, vote the `proposal` and wait its execution.
	democracy::vote_yes(&client, referendum_index).await?;
	democracy::wait_passed_and_dispatch_or_timeout(
		&client,
		referendum_index,
		Duration::from_secs(2 * 60),
	)
	.await?;

	Ok(())
}
