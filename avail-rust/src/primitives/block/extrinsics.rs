use super::extrinsics_params::OnlyCodecExtra;
use crate::{avail::runtime_types::da_runtime::RuntimeCall, Address, Signature};

use codec::{Compact, Decode, Error, Input};
use serde::Deserialize;
use subxt::backend::legacy::rpc_methods::Bytes;

pub type SignaturePayload = (Address, Signature, OnlyCodecExtra);

/// Current version of the [`UncheckedExtrinsic`] encoded format.
///
/// This version needs to be bumped if the encoded representation changes.
/// It ensures that if the representation is changed and the format is not known,
/// the decoding fails.
const EXTRINSIC_FORMAT_VERSION: u8 = 4;

#[derive(Debug, Clone)]
pub struct AppUncheckedExtrinsic {
	/// The signature, address, number of extrinsics have come before from
	/// the same signer and an era describing the longevity of this transaction,
	/// if this is a signed extrinsic.
	pub signature: Option<SignaturePayload>,
	/// The function that should be called.
	pub function: RuntimeCall,
}

impl AppUncheckedExtrinsic {
	pub fn app_id(&self) -> crate::AppId {
		self.signature
			.as_ref()
			.map(|(_, _, extra)| extra.8)
			.unwrap_or_default()
			.into()
	}
}

impl Decode for AppUncheckedExtrinsic {
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
		// This is a little more complicated than usual since the binary format must be compatible
		// with SCALE's generic `Vec<u8>` type. Basically this just means accepting that there
		// will be a prefix of vector length.
		let expected_length: Compact<u32> = Decode::decode(input)?;
		let before_length = input.remaining_len()?;

		let version = input.read_byte()?;

		let is_signed = version & 0b1000_0000 != 0;
		let version = version & 0b0111_1111;
		if version != EXTRINSIC_FORMAT_VERSION {
			return Err("Invalid transaction version".into());
		}

		let signature = is_signed.then(|| Decode::decode(input)).transpose()?;
		let function = Decode::decode(input)?;

		if let Some((before_length, after_length)) = input
			.remaining_len()?
			.and_then(|a| before_length.map(|b| (b, a)))
		{
			let length = before_length.saturating_sub(after_length);

			if length != expected_length.0 as usize {
				return Err("Invalid length prefix".into());
			}
		}

		Ok(Self {
			signature,
			function,
		})
	}
}

impl<'a> Deserialize<'a> for AppUncheckedExtrinsic {
	fn deserialize<D>(de: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'a>,
	{
		let r = sp_core::bytes::deserialize(de)?;
		Decode::decode(&mut &r[..])
			.map_err(|e| serde::de::Error::custom(format!("Decode error: {}", e)))
	}
}

impl TryFrom<Bytes> for AppUncheckedExtrinsic {
	type Error = String;

	fn try_from(value: Bytes) -> Result<Self, Self::Error> {
		let value: Vec<u8> = value.to_vec();
		let mut value_as_slice = value.as_slice();

		AppUncheckedExtrinsic::decode(&mut value_as_slice).map_err(|s| s.to_string())
	}
}
