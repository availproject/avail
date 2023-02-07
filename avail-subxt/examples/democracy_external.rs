use anyhow::{ensure, Result};
use avail_subxt::{
	api::{
		self,
		council::events as CouncilEvent,
		preimage::events as PreimageEvent,
		runtime_types::{
			frame_system::pallet::Call as SystemCall,
			pallet_democracy::pallet::Call as DemocracyCall, sp_weights::weight_v2::Weight,
		},
		technical_committee::events as TechComEvent,
	},
	avail::{Bounded, Client, TxEvents, TxProgress},
	build_client, tx_asend, tx_send, AvailConfig, Call, Opts,
};
use codec::Encode;
use derive_more::Constructor;
use futures::stream::{self, StreamExt as _, TryStreamExt as _};
use sp_keyring::AccountKeyring::{self, Alice, Bob, Charlie, Dave, Eve, Ferdie};
use structopt::StructOpt;
use subxt::{
	ext::sp_core::{sr25519::Pair, Pair as _, H256},
	tx::{PairSigner, Signer},
};

const THREASHOLD: u32 = 5;

#[derive(Debug, Default, Constructor)]
pub struct IndexedProposalContext {
	index: u32,
	hash: H256,
	len: u32,
}

#[derive(Debug, Default, Constructor)]
pub struct ProposalContext {
	hash: H256,
	len: u32,
}

#[derive(Debug, Default, Constructor)]
pub struct Context {
	council: IndexedProposalContext,
	democracy: ProposalContext,
}

mod council {
	use super::*;

	pub async fn create_external_proposal(
		client: &Client,
		signer: AccountKeyring,
	) -> Result<Context> {
		log::trace!("Creating the external proposal ...");
		let signer = PairSigner::new(signer.pair());
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
		let context = Context::new(
			IndexedProposalContext::new(event.proposal_index, event.proposal_hash, len_bound),
			ProposalContext::new(noted_event.hash, inner_call_len),
		);
		log::info!("New council proposal {:?}", context);

		Ok(context)
	}

	async fn vote_yes(
		client: &Client,
		signer: &(dyn Signer<AvailConfig> + Send + Sync),
		proposal: &IndexedProposalContext,
	) -> Result<TxProgress> {
		let vote = api::tx()
			.council()
			.vote(proposal.hash, proposal.index, true);
		tx_asend!(client, &vote, signer).map_err(Into::into)
	}

	pub async fn simple_majority_of_council_vote_yes(
		client: &Client,
		proposal: &IndexedProposalContext,
	) -> Result<()> {
		log::trace!("Council will vote proposal ...");
		let alice_stash = Pair::from_string("//Alice//stash", None).expect("Wellknow key .qed");
		let simple_majority_of_council = [Alice, Bob, Charlie, Dave, Eve, Ferdie]
			.into_iter()
			.map(|s| s.pair())
			.chain([alice_stash])
			.map(|p| PairSigner::new(p))
			.collect::<Vec<_>>();

		let pending_txs = stream::iter(simple_majority_of_council.iter())
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

	async fn close(
		client: &Client,
		signer: AccountKeyring,
		proposal: &IndexedProposalContext,
	) -> Result<TxEvents> {
		// TODO Calculate in advance the real weight.
		let close_weight = Weight {
			ref_time: 1_000_000_000,
			proof_size: 0,
		};
		let close =
			api::tx()
				.council()
				.close(proposal.hash, proposal.index, close_weight, proposal.len);
		let events = tx_send!(client, &close, &PairSigner::new(signer.pair()));
		Ok(events)
	}

	pub async fn close_and_check(
		client: &Client,
		signer: AccountKeyring,
		proposal: &IndexedProposalContext,
	) -> Result<()> {
		log::trace!("Closing the council proposal ...");
		let events = close(&client, signer, &proposal).await?;

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

		log::info!(
			"Proposal {} has been dispatched: {:?}",
			executed.proposal_hash,
			executed.result
		);

		Ok(())
	}
}

mod democracy {
	use super::*;

	pub async fn create_fast_track_proposal(
		client: &Client,
		signer: AccountKeyring,
		proposal: &ProposalContext,
	) -> Result<IndexedProposalContext> {
		let signer = PairSigner::new(signer.pair());
		let fast_track = Call::Democracy(DemocracyCall::fast_track {
			proposal_hash: proposal.hash,
			voting_period: 361,
			delay: 10,
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
		signer: &(dyn Signer<AvailConfig> + Send + Sync),
		proposal: &IndexedProposalContext,
	) -> Result<TxProgress> {
		let vote = api::tx()
			.technical_committee()
			.vote(proposal.hash, proposal.index, true);
		tx_asend!(client, &vote, signer).map_err(Into::into)
	}

	pub async fn techies_votes(client: &Client, proposal: &IndexedProposalContext) -> Result<()> {
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

	pub async fn close(
		client: &Client,
		signer: AccountKeyring,
		proposal: &IndexedProposalContext,
	) -> Result<TxEvents> {
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
		let events = tx_send!(client, &close, &PairSigner::new(signer.pair()));
		Ok(events)
	}
}

/// # Trace
/// Use `RUST_LOG="democracy_external=trace"` to see traces.
#[async_std::main]
async fn main() -> Result<()> {
	use crate::{council::*, democracy::*};

	pretty_env_logger::init();
	let args = Opts::from_args();
	let client = build_client(args.ws).await?;

	// In Council,  create, approve and execute ...
	let context = create_external_proposal(&client, Alice).await?;
	simple_majority_of_council_vote_yes(&client, &context.council).await?;
	close_and_check(&client, Alice, &context.council).await?;

	// In Democracy, external proposal is accepted.
	// Let's fast-track it.
	let tech_proposal = create_fast_track_proposal(&client, Alice, &context.democracy).await?;
	techies_votes(&client, &tech_proposal).await?;
	close(&client, Alice, &tech_proposal).await?;

	Ok(())
}
