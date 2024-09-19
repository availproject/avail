#[derive(Debug, Clone)]
pub enum CoreError {
	FromHexError(hex::FromHexError),
	ConversionError(String),
}
