use subxt::error::DispatchError;

#[derive(Debug)]
pub enum ClientError {
	Custom(String),
	Subxt(subxt::Error),
}

impl ClientError {
	pub fn to_string(&self) -> String {
		match self {
			ClientError::Custom(c) => c.clone(),
			ClientError::Subxt(error) => error.to_string(),
		}
	}
}

impl From<&str> for ClientError {
	fn from(value: &str) -> Self {
		Self::Custom(value.to_string())
	}
}

impl From<String> for ClientError {
	fn from(value: String) -> Self {
		Self::Custom(value.to_string())
	}
}

impl From<subxt::Error> for ClientError {
	fn from(value: subxt::Error) -> Self {
		Self::Subxt(value)
	}
}

impl From<DispatchError> for ClientError {
	fn from(value: DispatchError) -> Self {
		Self::Subxt(value.into())
	}
}
