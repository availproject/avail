use super::{AlreadyEncoded, H256};
use parity_scale_codec::{Compact, Encode};

/// Era period
pub type Period = u64;
/// Era phase
pub type Phase = u64;

#[derive(Debug, Clone)]
pub struct Call {
	pub pallet_index: u8,
	pub call_index: u8,
	pub fields: AlreadyEncoded,
}
impl Call {
	pub fn new(pallet_index: u8, call_index: u8, fields: AlreadyEncoded) -> Self {
		Self {
			pallet_index,
			call_index,
			fields,
		}
	}
}
impl Encode for Call {
	fn size_hint(&self) -> usize {
		self.pallet_index.size_hint() + self.call_index.size_hint() + self.fields.size_hint()
	}

	fn encode_to<T: parity_scale_codec::Output + ?Sized>(&self, dest: &mut T) {
		self.pallet_index.encode_to(dest);
		self.call_index.encode_to(dest);
		self.fields.encode_to(dest);
	}
}

#[derive(Debug, Clone)]
pub struct Extra {
	pub mortality: Era,
	pub nonce: Compact<u32>,
	pub tip: Compact<u128>,
	pub app_id: Compact<u32>,
}
impl Encode for Extra {
	fn size_hint(&self) -> usize {
		self.mortality.size_hint()
			+ self.nonce.0.size_hint()
			+ self.tip.0.size_hint()
			+ self.app_id.0.size_hint()
	}

	fn encode_to<T: parity_scale_codec::Output + ?Sized>(&self, dest: &mut T) {
		self.mortality.encode_to(dest);
		self.nonce.encode_to(dest);
		self.tip.encode_to(dest);
		self.app_id.encode_to(dest);
	}
}

#[derive(Debug, Clone)]
pub struct Additional {
	spec_version: u32,
	tx_version: u32,
	genesis_hash: H256,
	fork_hash: H256,
}

impl Additional {
	pub fn new(spec_version: u32, tx_version: u32, genesis_hash: H256, fork_hash: H256) -> Self {
		Self {
			spec_version,
			tx_version,
			genesis_hash,
			fork_hash,
		}
	}
}

impl Encode for Additional {
	fn size_hint(&self) -> usize {
		self.spec_version.size_hint()
			+ self.tx_version.size_hint()
			+ self.genesis_hash.size_hint()
			+ self.fork_hash.size_hint()
	}

	fn encode_to<T: parity_scale_codec::Output + ?Sized>(&self, dest: &mut T) {
		self.spec_version.encode_to(dest);
		self.tx_version.encode_to(dest);
		self.genesis_hash.encode_to(dest);
		self.fork_hash.encode_to(dest);
	}
}

#[derive(Debug, Clone)]
pub enum Era {
	Immortal,
	Mortal(Period, Phase),
}
impl Era {
	/// Create a new era based on a period (which should be a power of two between 4 and 65536
	/// inclusive) and a block number on which it should start (or, for long periods, be shortly
	/// after the start).
	///
	/// If using `Era` in the context of `FRAME` runtime, make sure that `period`
	/// does not exceed `BlockHashCount` parameter passed to `system` module, since that
	/// prunes old blocks and renders transactions immediately invalid.
	pub fn mortal(period: Period, block_number: u64) -> Self {
		let period = period
			.checked_next_power_of_two()
			.unwrap_or(1 << 16)
			.clamp(4, 1 << 16);
		let phase = block_number % period;
		let quantize_factor = (period >> 12).max(1);
		let quantized_phase = phase / quantize_factor * quantize_factor;

		Self::Mortal(period, quantized_phase)
	}

	pub fn immortal() -> Self {
		Self::Immortal
	}
}
impl Encode for Era {
	fn size_hint(&self) -> usize {
		match self {
			Era::Immortal => 1,
			Era::Mortal(x, y) => (*x, *y).size_hint(),
		}
	}

	fn encode_to<T: parity_scale_codec::Output + ?Sized>(&self, dest: &mut T) {
		match self {
			Self::Immortal => dest.push_byte(0),
			Self::Mortal(period, phase) => {
				let quantize_factor = (*period >> 12).max(1);
				let encoded = (period.trailing_zeros() - 1).clamp(1, 15) as u16
					| ((phase / quantize_factor) << 4) as u16;
				encoded.encode_to(dest);
			},
		}
	}
}
