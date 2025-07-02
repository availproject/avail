use crate::{BlockVerificationStatus, VerificationTracker, LOG_TARGET};
use da_runtime::Hash;
use log::{debug, trace, warn};
use sc_consensus_grandpa::{VotingRule, VotingRuleResult};
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	traits::{Block as BlockT, Header, NumberFor},
	Saturating,
};
use std::sync::Arc;

/// GRANDPA voting rule that restricts finalization to DA-verified blocks
#[derive(Clone)]
pub struct DaVerificationVotingRule<Block: BlockT> {
	tracker: Arc<VerificationTracker>,
	_phantom: std::marker::PhantomData<Block>,
}

impl<Block: BlockT> DaVerificationVotingRule<Block> {
	pub fn new(tracker: Arc<VerificationTracker>) -> Self {
		Self {
			tracker,
			_phantom: std::marker::PhantomData,
		}
	}
}

impl<Block, B> VotingRule<Block, B> for DaVerificationVotingRule<Block>
where
	Block: BlockT<Hash = Hash>,
	B: sc_client_api::HeaderBackend<Block>,
{
	fn restrict_vote(
		&self,
		backend: Arc<B>,
		base: &Block::Header,
		best_target: &Block::Header,
		current_target: &Block::Header,
	) -> VotingRuleResult<Block> {
		trace!(target: LOG_TARGET, "restrict_vote called with base: {:?}, best: {:?} current_target: {:?}", base.hash(), best_target.hash(), current_target.hash());
		let current_hash = current_target.hash();
		let current_number = *current_target.number();

		// Check all un-finalized blocks from base to current_target
		if let Some((first_unverified, unverified_number)) = find_first_unverified_da_block(
			&*backend,
			&*self.tracker,
			base.hash(),
			*base.number(),
			current_hash,
			current_number,
		) {
			debug!(
				target: LOG_TARGET,
				"Restricting vote due to unverified DA block #{} ({})", unverified_number, first_unverified
			);

			if let Ok(Some(header)) = backend.header(first_unverified) {
				let parent_hash = *header.parent_hash();
				let parent_number = unverified_number.saturating_sub(1u32.into());

				return Box::pin(async move { Some((parent_hash, parent_number)) });
			} else {
				warn!(target: LOG_TARGET, "Could not get parent of unverified block {}", first_unverified);
				let fallback_hash = base.hash();
				let fallback_number = *base.number();
				return Box::pin(async move { Some((fallback_hash, fallback_number)) });
			}
		}

		debug!(
			target: LOG_TARGET,
			"Allowing vote - all un-finalized DA blocks verified up to {}", current_hash
		);
		Box::pin(async { None })
	}
}

/// Walk the chain from `start_hash` to `end_hash` and find the first unverified DA block
fn find_first_unverified_da_block<Block, B>(
	backend: &B,
	tracker: &VerificationTracker,
	start_hash: Block::Hash,
	start_number: NumberFor<Block>,
	end_hash: Block::Hash,
	end_number: NumberFor<Block>,
) -> Option<(Block::Hash, NumberFor<Block>)>
where
	Block: BlockT<Hash = Hash>,
	B: HeaderBackend<Block>,
{
	let mut current_hash = end_hash;
	let mut current_number = end_number;
	let mut deepest_unverified: Option<(Block::Hash, NumberFor<Block>)> = None;

	while current_hash != start_hash && current_number >= start_number {
		match tracker.get_status(&current_hash) {
			Some(BlockVerificationStatus::Verified) => { /* all good */ },
			_ => {
				deepest_unverified = Some((current_hash, current_number));
			},
		}

		// Always walk to parent
		match backend.header(current_hash) {
			Ok(Some(header)) => {
				current_hash = *header.parent_hash();
				current_number = current_number.saturating_sub(1u32.into());
			},
			Ok(None) => break, // Shouldn't happen for un-finalized blocks
			Err(e) => {
				warn!("Error fetching header: {:?}", e);
				break;
			},
		}
	}
	deepest_unverified
}
