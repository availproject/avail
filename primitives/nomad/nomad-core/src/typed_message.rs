use alloc::vec::Vec;

/// This trait provides structure for encoding a Vec<u8> as a xapp message.
/// First byte of Vec<u8> is a u8 corresponding to a message type. The remaining
/// bytes make up the message body.
pub trait TypedMessage: AsRef<[u8]> {
	type MessageEnum: From<u8>;

	/// Return the message type
	fn message_type(&self) -> Self::MessageEnum {
		let slice: &[u8] = self.as_ref();
		slice[0].into()
	}

	/// Return the message body after the type byte
	fn message_body(&self) -> Vec<u8> {
		let slice: &[u8] = self.as_ref();
		slice[1..].to_vec()
	}
}
