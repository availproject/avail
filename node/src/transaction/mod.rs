mod macros;

pub mod data;
pub mod state;

#[derive(Clone, Default)]
pub struct CliDeps {
	pub data: data::CliDeps,
	pub state: state::CliDeps,
}

#[derive(Default)]
pub struct Deps {
	pub data: Option<data::Deps>,
	pub state: Option<state::Deps>,
}
