use crate::{InherentError, INHERENT_IDENTIFIER};

use async_trait::async_trait;
use codec::Decode;
use sp_inherents::{Error as IError, InherentData, InherentDataProvider, InherentIdentifier};

pub struct IDProvider {}

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
}
