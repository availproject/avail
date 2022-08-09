use alloc::vec::Vec;

pub struct TypedView(Vec<u8>);

impl TypedView {
	pub fn new(view: Vec<u8>) -> Self { Self(view) }
}

impl AsRef<[u8]> for TypedView {
	fn as_ref(&self) -> &[u8] { self.0.as_ref() }
}

/// First byte of Vec<u8> is a u8 corresponding to a message type. The remaining
/// bytes make up the message body. This trait provides structure for encoding a
/// Vec<u8> as a xapp message.
pub trait TypedMessage: AsRef<[u8]> {
	type MessageEnum: From<u8>;

	/// Return the message type
	fn message_type(&self) -> Self::MessageEnum {
		let slice: &[u8] = self.as_ref();
		slice[0].into()
	}
}
