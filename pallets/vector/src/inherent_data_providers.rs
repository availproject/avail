use codec::{Decode, Encode};
use sp_core::RuntimeDebug;
use sp_inherents::{Error as IError, InherentData, InherentIdentifier, IsFatalError};

/// The identifier for the `FailedSendMsg` inherent.
pub const ID: InherentIdentifier = *b"vector00";

/// Errors that can occur while checking the timestamp inherent.
#[derive(Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode))]
pub enum InherentError {
	MismatchFailedList,
}

impl IsFatalError for InherentError {
	fn is_fatal_error(&self) -> bool {
		true
	}
}

pub trait InherentUpdater {
	fn update_inherent() -> Result<(), IError>;
}

/*
pub struct IDProvider {}

use async_trait::async_trait;
#[async_trait]
impl InherentDataProvider for IDProvider {
	async fn provide_inherent_data(&self, inherent_data: &mut InherentData) -> Result<(), IError> {
		let failed :Vec<u32> = vec![];
		inherent_data.put_data(INHERENT_IDENTIFIER, &failed)
	}

	async fn try_handle_error(
		&self,
		identifier: &InherentIdentifier,
		mut error: &[u8],
	) -> Option<Result<(), IError>> {
		if *identifier != INHERENT_IDENTIFIER {
			return None;
		}

		let error = InherentError::decode(&mut error).ok()?;
		Some(Err(IError::Application(Box::from(format!("{:?}", error)))))
	}
}*/
