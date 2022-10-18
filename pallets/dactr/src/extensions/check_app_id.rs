use codec::{Decode, Encode};
use da_primitives::{
	asdr::{AppId, GetAppId},
	InvalidTransactionCustomId,
};
use frame_support::{ensure, traits::IsSubType};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{DispatchInfoOf, SignedExtension},
	transaction_validity::{
		InvalidTransaction, TransactionValidity, TransactionValidityError, ValidTransaction,
	},
};
use sp_std::{
	default::Default,
	fmt::{self, Debug, Formatter},
	marker::PhantomData,
};

use crate::{Call as DACall, Config, Pallet};

/// Check for Application Id.
///
/// # Transaction Validity
///
/// Only registered application can be used by transactions.
///
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CheckAppId<T: Config + Send + Sync>(pub AppId, sp_std::marker::PhantomData<T>);

impl<T> CheckAppId<T>
where
	T: Config + Send + Sync,
	T::Call: IsSubType<DACall<T>>,
{
	/// utility constructor. Used only in client/factory code.
	pub fn from(app_id: AppId) -> Self { Self(app_id, sp_std::marker::PhantomData) }

	/// Transaction validation:
	///  - Only `DataAvailability::submit_data(..)` extrinsic can use `AppId != 0`. Any other call
	///  must use `AppId == 0`.
	///  - It validates that `AppId` is already registered.
	pub fn do_validate(&self, call: &T::Call) -> TransactionValidity {
		match call.is_sub_type() {
			// Only `dactrl::submit_data` can use `AppId != 0`.
			Some(DACall::<T>::submit_data { .. }) => {
				let next_app_id = <Pallet<T>>::peek_next_application_id();
				ensure!(
					self.app_id() < next_app_id,
					InvalidTransaction::Custom(InvalidTransactionCustomId::InvalidAppId as u8)
				);
			},
			_ => {
				// Any other call must use `AppId == 0`.
				ensure!(
					self.app_id().0 == 0,
					InvalidTransaction::Custom(InvalidTransactionCustomId::ForbiddenAppId as u8)
				);
			},
		};

		Ok(ValidTransaction::default())
	}
}
impl<T: Config + Send + Sync> Default for CheckAppId<T> {
	fn default() -> Self { Self(AppId::default(), PhantomData) }
}

impl<T> Debug for CheckAppId<T>
where
	T: Config + Send + Sync,
{
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result { write!(f, "CheckAppId: {}", self.0) }

	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut Formatter) -> fmt::Result { Ok(()) }
}

impl<T> SignedExtension for CheckAppId<T>
where
	T: Config + Send + Sync,
	T::Call: IsSubType<DACall<T>>,
{
	type AccountId = T::AccountId;
	type AdditionalSigned = ();
	type Call = T::Call;
	type Pre = ();

	const IDENTIFIER: &'static str = "CheckAppId";

	fn validate(
		&self,
		_who: &Self::AccountId,
		call: &Self::Call,
		_info: &DispatchInfoOf<Self::Call>,
		_len: usize,
	) -> TransactionValidity {
		self.do_validate(call)
	}

	fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
		Ok(())
	}
}

impl<T> GetAppId for CheckAppId<T>
where
	T: Config + Send + Sync,
{
	#[inline]
	fn app_id(&self) -> AppId { self.0 }
}

#[cfg(test)]
mod tests {
	use da_primitives::InvalidTransactionCustomId::{ForbiddenAppId, InvalidAppId};
	use frame_system::pallet::Call as SysCall;
	use sp_runtime::transaction_validity::InvalidTransaction;
	use test_case::test_case;

	use super::*;
	use crate::{
		mock::{new_test_ext, Call, Test},
		pallet::Call as DACall,
	};

	fn remark_call() -> Call { Call::System(SysCall::remark { remark: vec![] }) }

	fn submit_data_call() -> Call {
		Call::DataAvailability(DACall::submit_data {
			data: vec![].try_into().unwrap(),
		})
	}

	fn to_invalid_tx(custom_id: InvalidTransactionCustomId) -> TransactionValidity {
		Err(TransactionValidityError::Invalid(
			InvalidTransaction::Custom(custom_id as u8),
		))
	}

	#[test_case(100, submit_data_call() => to_invalid_tx(InvalidAppId); "100 AppId is invalid" )]
	#[test_case(0, remark_call() => Ok(ValidTransaction::default()); "System::remark can be called if AppId == 0" )]
	#[test_case(1, remark_call() => to_invalid_tx(ForbiddenAppId); "System::remark cannot be called if AppId != 0" )]
	#[test_case(1, submit_data_call() => Ok(ValidTransaction::default()); "submit_data can be called with any valid AppId" )]
	fn do_validate_test<A: Into<AppId>>(app_id: A, call: Call) -> TransactionValidity {
		new_test_ext().execute_with(|| CheckAppId::<Test>::from(app_id.into()).do_validate(&call))
	}
}
