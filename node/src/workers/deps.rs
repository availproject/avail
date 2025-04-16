use super::{block_explorer, indexer};

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
