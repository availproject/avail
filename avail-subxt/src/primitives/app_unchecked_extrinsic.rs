use std::mem::size_of;

use codec::{Compact, Decode, Encode, EncodeLike, Error, Input};
use parity_util_mem::MallocSizeOf;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use subxt::ext::{sp_core::bytes, sp_runtime::traits::Extrinsic};

use crate::{Call, SignaturePayload};
/// Current version of the [`AppUncheckedExtrinsic`] format.
const EXTRINSIC_VERSION: u8 = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppUncheckedExtrinsic {
	/// The signature, address, number of extrinsics have come before from
	/// the same signer and an era describing the longevity of this transaction,
	/// if this is a signed extrinsic.
	pub signature: Option<SignaturePayload>,
	/// The function that should be called.
	pub function: Call,
}

impl Decode for AppUncheckedExtrinsic {
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
		// This is a little more complicated than usual since the binary format must be compatible
		// with substrate's generic `Vec<u8>` type. Basically this just means accepting that there
		// will be a prefix of vector length (we don't need
		// to use this).
		let _length_do_not_remove_me_see_above: Compact<u32> = Decode::decode(input)?;

		let version = input.read_byte()?;

		let is_signed = version & 0b1000_0000 != 0;
		let version = version & 0b0111_1111;
		if version != EXTRINSIC_VERSION {
			return Err("Invalid transaction version".into());
		}

		let signature = if is_signed {
			Some(SignaturePayload::decode(input)?)
		} else {
			None
		};
		let function = Decode::decode(input)?;

		Ok(Self {
			signature,
			function,
		})
	}
}

impl Encode for AppUncheckedExtrinsic {
	fn encode(&self) -> Vec<u8> {
		let mut tmp = Vec::with_capacity(size_of::<Self>());

		// 1 byte version id.
		match self.signature.as_ref() {
			Some(s) => {
				tmp.push(EXTRINSIC_VERSION | 0b1000_0000);
				s.encode_to(&mut tmp);
			},
			None => {
				tmp.push(EXTRINSIC_VERSION & 0b0111_1111);
			},
		}
		self.function.encode_to(&mut tmp);

		let compact_len = codec::Compact::<u32>(tmp.len() as u32);

		// Allocate the output buffer with the correct length
		let mut output = Vec::with_capacity(compact_len.size_hint() + tmp.len());

		compact_len.encode_to(&mut output);
		output.extend(tmp);

		output
	}
}

impl EncodeLike for AppUncheckedExtrinsic {}

impl<'a> Deserialize<'a> for AppUncheckedExtrinsic {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'a>,
	{
		let encoded = bytes::deserialize(deserializer)?;
		AppUncheckedExtrinsic::decode(&mut &encoded[..])
			.map_err(|codec_err| D::Error::custom(format!("Invalid decoding: {:?}", codec_err)))
	}
}

impl Serialize for AppUncheckedExtrinsic {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let encoded = self.encode();
		bytes::serialize(&encoded, serializer)
	}
}

impl Extrinsic for AppUncheckedExtrinsic {
	type Call = Call;
	type SignaturePayload = SignaturePayload;

	fn is_signed(&self) -> Option<bool> { Some(self.signature.is_some()) }
}

impl MallocSizeOf for AppUncheckedExtrinsic {
	fn size_of(&self, _ops: &mut parity_util_mem::MallocSizeOfOps) -> usize {
		size_of::<SignaturePayload>() + size_of::<Call>()
	}
}
