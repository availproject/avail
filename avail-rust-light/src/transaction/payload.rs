/*
	A Transaction has five parts (in this order):
		1. Some random stuff at the beginning
		2. Address of the caller (basically AccountId32)
		3. Signature of Payload
		4. Extra part of Payload
		5. Call part of Payload

	In this file we try to deal with 3., 4., and 5.
	The Payload contains the following parts:
		1. Call (must be SCALE encoded)
		2. Extra (Nonce, AppId, Mortality and Tip)
		3. Additional (Spec Version, Tx Version, Genesis Hash, Best Block Hash (aka fork picker))

*/

use super::AlreadyEncoded;
use crate::{
	crypto::{blake2_256, Keypair, Signature},
	interface::BlockHash,
};
use parity_scale_codec::{Compact, Encode};

pub type PayloadSignature = Signature;

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
pub struct UnsignedPayload {
	call: Call,
	extra: Extra,
	additional: Additional,
}
impl UnsignedPayload {
	pub fn new(call: Call, extra: Extra, additional: Additional) -> Self {
		Self {
			call,
			extra,
			additional,
		}
	}

	pub fn encode(self) -> UnsignedEncodedPayload {
		UnsignedEncodedPayload::new(
			AlreadyEncoded(self.call.encode()),
			AlreadyEncoded(self.extra.encode()),
			AlreadyEncoded(self.additional.encode()),
		)
	}
}

#[derive(Debug, Clone)]
pub struct UnsignedEncodedPayload {
	pub call: AlreadyEncoded,
	pub extra: AlreadyEncoded,
	pub additional: AlreadyEncoded,
}
impl UnsignedEncodedPayload {
	pub fn new(call: AlreadyEncoded, extra: AlreadyEncoded, additional: AlreadyEncoded) -> Self {
		Self {
			call,
			extra,
			additional,
		}
	}

	pub fn sign(&self, signer: &Keypair) -> PayloadSignature {
		let mut bytes: Vec<u8> = Vec::with_capacity(
			self.call.size_hint() + self.extra.size_hint() + self.additional.size_hint(),
		);

		self.call.encode_to(&mut bytes);
		self.extra.encode_to(&mut bytes);
		self.additional.encode_to(&mut bytes);

		if bytes.len() > 256 {
			let blake = blake2_256(&bytes);
			signer.sign(blake.as_ref())
		} else {
			signer.sign(&bytes)
		}
	}
}

/*
	Why do we need this?
	We want to make sure that our transaction will be executed on the right chain
	and on the right fork.

	- Spec Version - This is self-explanatory.
	- Tx Version - This is self-explanatory.
	- Genesis Hash - This is self-explanatory.
	- Fork Hash - This one is interesting. It defines on what fork of a network
		we want to execute our transaction. For Immortal transaction this value
		is the genesis hash. For mortal transaction this is can be either
		the genesis hash or the hash of the best block. If there are no big forks
		(separate networks with the same genesis has) using the genesis hash is
		more than good enough.
*/
#[derive(Debug, Clone)]
pub struct Additional {
	spec_version: u32,
	tx_version: u32,
	genesis_hash: BlockHash,
	fork_hash: BlockHash,
}

impl Additional {
	pub fn new(
		spec_version: u32,
		tx_version: u32,
		genesis_hash: BlockHash,
		fork_hash: BlockHash,
	) -> Self {
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
pub struct Extra {
	pub mortality: CheckedMortality,
	pub nonce: CheckedNonce,
	pub tip: CheckedTip,
	pub app_id: CheckedAppId,
}
impl Encode for Extra {
	fn size_hint(&self) -> usize {
		self.mortality.extra_size_hint()
			+ self.nonce.0.size_hint()
			+ self.tip.0.size_hint()
			+ self.app_id.0.size_hint()
	}

	fn encode_to<T: parity_scale_codec::Output + ?Sized>(&self, dest: &mut T) {
		self.mortality.encode_extra_to(dest);
		self.nonce.0.encode_to(dest);
		self.tip.0.encode_to(dest);
		self.app_id.0.encode_to(dest);
	}
}

#[derive(Debug, Clone)]
pub struct CheckedNonce(pub Compact<u32>);
#[derive(Debug, Clone)]
pub struct CheckedTip(pub Compact<u128>);
#[derive(Debug, Clone)]
pub struct CheckedAppId(pub Compact<u32>);
#[derive(Debug, Clone)]
pub struct CheckedMortality(pub Era);

impl CheckedMortality {
	fn extra_size_hint(&self) -> usize {
		self.0.size_hint()
	}
	fn encode_extra_to<T: parity_scale_codec::Output + ?Sized>(&self, v: &mut T) {
		self.0.encode_to(v);
	}
}

impl CheckedMortality {
	pub fn mortal(period: u64, block_number: u32) -> Self {
		Self(Era::mortal(period, block_number as u64))
	}
}

/// Era period
pub type Period = u64;
/// Era phase
pub type Phase = u64;

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
