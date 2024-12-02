use subxt::error::DispatchError;
use subxt_signer::sr25519;
use subxt_signer::SecretUriError;

use crate::transactions::TransactionFailed;
use crate::utils::TransactionExecutionError;

type RpcError = subxt::backend::rpc::reconnecting_rpc_client::Error;

#[derive(Debug)]
pub enum ClientError {
	Custom(String),
	TransactionExecution(TransactionExecutionError),
	RpcError(RpcError),
	SerdeJson(serde_json::Error),
	Subxt(subxt::Error),
	SubxtSigner(SecretUriError),
	Sr25519(sr25519::Error),
}

impl ClientError {
	pub fn to_string(&self) -> String {
		match self {
			ClientError::Custom(e) => e.clone(),
			ClientError::TransactionExecution(e) => e.to_string(),
			ClientError::RpcError(e) => e.to_string(),
			ClientError::SerdeJson(e) => e.to_string(),
			ClientError::Subxt(e) => e.to_string(),
			ClientError::SubxtSigner(e) => e.to_string(),
			ClientError::Sr25519(e) => e.to_string(),
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

impl From<SecretUriError> for ClientError {
	fn from(value: SecretUriError) -> Self {
		Self::SubxtSigner(value)
	}
}

impl From<sr25519::Error> for ClientError {
	fn from(value: sr25519::Error) -> Self {
		Self::Sr25519(value)
	}
}

impl From<TransactionFailed> for ClientError {
	fn from(value: TransactionFailed) -> Self {
		value.reason
	}
}

impl From<RpcError> for ClientError {
	fn from(value: RpcError) -> Self {
		Self::RpcError(value)
	}
}

impl From<serde_json::Error> for ClientError {
	fn from(value: serde_json::Error) -> Self {
		Self::SerdeJson(value)
	}
}
