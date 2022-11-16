use core::{convert::TryFrom, mem};

use sp_std::vec::Vec;

/// This trait provides structure for encoding a Vec<u8> as a xapp message.
/// First byte of Vec<u8> is a u8 corresponding to a message type. The remaining
/// bytes make up the message body.
pub trait TypedMessage {
	type MessageEnum: TryFrom<u8>;

	/// Return the message body after the type byte
	fn encode(&self) -> Vec<u8>;
}

pub trait TypedMessageVariant
where
	Self: Sized,
{
	const MESSAGE_TYPE: u8;

	/// Size of encoded struct (+ 1 for u8 type tag)
	fn len(&self) -> usize { mem::size_of::<Self>() + 1 }

	fn is_empty(&self) -> bool { self.len() == 0 }

	/// Encode self into `BoundedVec<u8, S>`
	fn encode(&self) -> Vec<u8>;
}
