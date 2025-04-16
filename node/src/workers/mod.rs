mod chain_api;
mod macros;

pub(crate) mod block_explorer;
pub(crate) mod deps;
pub(crate) mod indexer;

pub(crate) use deps::*;

use codec::Encode;
use da_runtime::UncheckedExtrinsic;
use std::time::{Duration, Instant};

fn read_pallet_call_index(ext: &UncheckedExtrinsic) -> Option<(u8, u8)> {
	let ext = ext.function.encode();
	if ext.len() < 2 {
		return None;
	}
	let pallet_index = ext[0];
	let call_index = ext[1];

	Some((pallet_index, call_index))
}

struct Timer {
	now: Instant,
	duration: u64,
}

impl Timer {
	pub fn new(duration: u64) -> Self {
		Self {
			now: Instant::now(),
			duration,
		}
	}

	pub fn restart(&mut self) -> Instant {
		self.now = Instant::now();
		self.now.clone()
	}

	pub fn elapsed(&self) -> Duration {
		self.now.elapsed()
	}

	pub fn expired(&self) -> bool {
		self.elapsed().as_secs() > self.duration
	}

	pub fn duration(&self) -> u64 {
		self.duration
	}
}
