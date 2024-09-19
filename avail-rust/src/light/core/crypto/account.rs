use super::{ss58::ByteArray, Ss58Codec};
use parity_scale_codec::Encode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccountId(pub [u8; 32]);
impl AccountId {
	pub fn to_hex_string(&self) -> String {
		std::format!("0x{}", hex::encode(self.0))
	}
}
impl Encode for AccountId {
	fn size_hint(&self) -> usize {
		self.0.size_hint()
	}

	fn encode_to<T: parity_scale_codec::Output + ?Sized>(&self, dest: &mut T) {
		self.0.encode_to(dest)
	}
}
impl AsRef<[u8]> for AccountId {
	fn as_ref(&self) -> &[u8] {
		&self.0[..]
	}
}
impl AsMut<[u8]> for AccountId {
	fn as_mut(&mut self) -> &mut [u8] {
		&mut self.0[..]
	}
}
impl From<[u8; 32]> for AccountId {
	fn from(x: [u8; 32]) -> Self {
		AccountId(x)
	}
}
impl<'a> TryFrom<&'a [u8]> for AccountId {
	type Error = ();
	fn try_from(x: &'a [u8]) -> Result<AccountId, ()> {
		if x.len() == 32 {
			let mut data = [0; 32];
			data.copy_from_slice(x);
			Ok(AccountId(data))
		} else {
			Err(())
		}
	}
}
impl ByteArray for AccountId {
	const LEN: usize = 32;
}
impl Ss58Codec for AccountId {}
