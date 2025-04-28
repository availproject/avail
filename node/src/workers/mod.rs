mod chain_api;
mod common;
mod macros;

pub(crate) mod block_explorer;
pub(crate) mod indexer;
pub(crate) use common::{events::*, CachedValue, NodeContext, Timer, TransactionId};

#[derive(Clone, Default)]
pub struct CliDeps {
	pub block_explorer: block_explorer::CliDeps,
	pub indexer: indexer::CliDeps,
}

#[derive(Default)]
pub struct Deps {
	pub block_explorer: Option<block_explorer::Deps>,
	pub indexer: Option<indexer::Deps>,
}
