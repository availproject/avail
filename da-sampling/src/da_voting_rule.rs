use crate::{BlockVerificationStatus, VerificationTracker, LOG_TARGET};
use log::debug;
use sc_consensus_grandpa::{VotingRule, VotingRuleResult};
use sp_runtime::{
	testing::H256,
	traits::{Block as BlockT, Header},
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
	Block: BlockT<Hash = H256>,
	B: sc_client_api::HeaderBackend<Block>,
{
	fn restrict_vote(
		&self,
		_backend: Arc<B>,
		_base: &Block::Header,
		_best_target: &Block::Header,
		current_target: &Block::Header,
	) -> VotingRuleResult<Block> {
		let block_hash = current_target.hash();

		match self.tracker.get_status(&block_hash) {
			Some(BlockVerificationStatus::Verified) => {
				debug!(target: LOG_TARGET, "Block {} is verified", block_hash);
			},
			other => {
				debug!(target: LOG_TARGET, "Block {} verification status: {:?}", block_hash, other);
			},
		}
		if !matches!(
			self.tracker.get_status(&block_hash),
			Some(BlockVerificationStatus::Verified)
		) {
			debug!(
				target: LOG_TARGET,
				"Restricting vote for block {} - DA verification missing/failed",
				block_hash
			);
			return Box::pin(std::future::ready(Some((
				block_hash,
				*current_target.number(),
			))));
		}

		Box::pin(async { None })
	}
}
