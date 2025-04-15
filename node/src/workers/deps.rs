use super::{data, indexer};

#[derive(Clone, Default)]
pub struct CliDeps {
	pub data: data::CliDeps,
	pub state: indexer::CliDeps,
}

#[derive(Default)]
pub struct Deps {
	pub data: Option<data::Deps>,
	pub state: Option<indexer::Deps>,
}
