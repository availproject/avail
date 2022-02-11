use codec::{Decode, Encode};
use da_primitives::{
	asdr::{AppId, GetAppId},
	InvalidTransactionCustomId::InvalidAppId,
};
use frame_support::ensure;
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{DispatchInfoOf, SignedExtension},
	transaction_validity::{
		InvalidTransaction, TransactionValidity, TransactionValidityError, ValidTransaction,
	},
};

use crate::{Config, Pallet};

/// Check for Application Id.
///
/// # Transaction Validity
///
/// Only registered application can be used by transactions.
///
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CheckAppId<T: Config + Send + Sync>(pub AppId, sp_std::marker::PhantomData<T>);

impl<T: Config + Send + Sync> CheckAppId<T> {
	/// utility constructor. Used only in client/factory code.
	pub fn from(app_id: AppId) -> Self { Self(app_id, sp_std::marker::PhantomData) }

	pub fn do_validate(&self) -> TransactionValidity {
		let last_app_id = <Pallet<T>>::last_application_id();
		ensure!(
			self.0 < last_app_id,
			InvalidTransaction::Custom(InvalidAppId as u8)
		);

		Ok(ValidTransaction::default())
	}
}

impl<T: Config + Send + Sync> sp_std::fmt::Debug for CheckAppId<T> {
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		write!(f, "CheckAppId: {}", self.0)
	}

	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result { Ok(()) }
}

impl<T: Config + Send + Sync> SignedExtension for CheckAppId<T> {
	type AccountId = T::AccountId;
	type AdditionalSigned = ();
	type Call = T::Call;
	type Pre = ();

	const IDENTIFIER: &'static str = "CheckAppId";

	fn validate(
		&self,
		_who: &Self::AccountId,
		_call: &Self::Call,
		_info: &DispatchInfoOf<Self::Call>,
		_len: usize,
	) -> TransactionValidity {
		self.do_validate()
	}

	fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
		Ok(())
	}
}

impl<T> GetAppId<AppId> for CheckAppId<T>
where
	T: Config + Send + Sync,
{
	#[inline]
	fn app_id(&self) -> AppId { self.0 }
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::{new_test_ext, Test};

	#[test]
	fn signed_ext_check_app_id_should_work() {
		new_test_ext().execute_with(|| {
			// invalid App Id
			assert_eq!(
				CheckAppId::<Test>::from(100).do_validate().err().unwrap(),
				InvalidTransaction::Custom(InvalidAppId as u8).into(),
			);

			// correct
			assert!(CheckAppId::<Test>::from(2).do_validate().is_ok());
		})
	}
}
