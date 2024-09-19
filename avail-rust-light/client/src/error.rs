use sdk_core::types::error::CoreError;

#[derive(Debug)]
pub enum ClientError {
	Jsonrpsee(jsonrpsee_core::client::error::Error),
	Core(CoreError),
	CodecError(parity_scale_codec::Error),
	SerdeJson(serde_json::Error),
	FromHexError(hex::FromHexError),
}
