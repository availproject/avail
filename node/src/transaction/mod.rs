mod macros;

pub mod data;
pub mod state;

use codec::Encode;
use da_runtime::UncheckedExtrinsic;

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

fn read_pallet_call_index(ext: &UncheckedExtrinsic) -> Option<(u8, u8)> {
	let ext = ext.function.encode();
	if ext.len() < 2 {
		return None;
	}
	let pallet_index = ext[0];
	let call_index = ext[1];

	Some((pallet_index, call_index))
}
