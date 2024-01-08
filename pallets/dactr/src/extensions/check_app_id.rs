use avail_core::{traits::GetAppId, AppId, InvalidTransactionCustomId};
use codec::{Decode, Encode};
use frame_support::{
	ensure,
	traits::{IsSubType, IsType},
};
use frame_system::Config as SystemConfig;
use pallet_utility::{Call as UtilityCall, Config as UtilityConfig};
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
	vec::Vec,
};

use crate::{Call as DACall, Config as DAConfig, Pallet};

const MAX_ITERATIONS: usize = 2;

/// Check for Application Id.
///
/// # Transaction Validity
///
/// Only registered application can be used by transactions.
///
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CheckAppId<T: DAConfig + UtilityConfig + Send + Sync>(
	pub AppId,
	sp_std::marker::PhantomData<T>,
);

impl<T> CheckAppId<T>
where
	T: DAConfig + UtilityConfig + Send + Sync,
	<T as SystemConfig>::RuntimeCall: IsSubType<DACall<T>> + IsSubType<UtilityCall<T>>,
{
	/// utility constructor. Used only in client/factory code.
	pub fn from(app_id: AppId) -> Self {
		Self(app_id, sp_std::marker::PhantomData)
	}

	/// It validates that `AppId` is correct and already registered for the call and potential nested calls.
	/// Transaction validation:
	///  - `DataAvailability::submit_data(..)` extrinsic can use `AppId != 0`.
	///  - `Utility::batch/batch_all/force_batch(..)` extrinsic can use `AppId != 0` If the wrapped calls are ALL `DataAvailability::submit_data(..)`.
	///  - Any other call must use `AppId == 0`.
	pub fn do_validate(&self, call: &<T as SystemConfig>::RuntimeCall) -> TransactionValidity {
		if self.app_id() == AppId(0) {
			return Ok(ValidTransaction::default());
		}

		let mut stack = Vec::new();
		stack.push(call);

		let mut maybe_next_app_id: Option<AppId> = None;
		let mut iterations = 0;

		while let Some(call) = stack.pop() {
			if let Some(DACall::<T>::submit_data { .. }) = call.is_sub_type() {
				let next_app_id =
					maybe_next_app_id.get_or_insert_with(<Pallet<T>>::peek_next_application_id);
				ensure!(
					self.app_id() < *next_app_id,
					InvalidTransaction::Custom(InvalidTransactionCustomId::InvalidAppId as u8)
				);
			} else {
				match call.is_sub_type() {
					Some(UtilityCall::<T>::batch { calls })
					| Some(UtilityCall::<T>::batch_all { calls })
					| Some(UtilityCall::<T>::force_batch { calls }) => {
						iterations += 1;
						ensure!(
							iterations < MAX_ITERATIONS,
							InvalidTransaction::Custom(
								InvalidTransactionCustomId::MaxRecursionExceeded as u8
							)
						);
						for call in calls.iter() {
							stack.push(call.into_ref());
						}
						Ok(())
					},
					_ => Err(TransactionValidityError::Invalid(
						InvalidTransaction::Custom(
							InvalidTransactionCustomId::ForbiddenAppId as u8,
						),
					)),
				}?;
			}
		}

		Ok(ValidTransaction::default())
	}
}

impl<T: DAConfig + UtilityConfig + Send + Sync> Default for CheckAppId<T> {
	fn default() -> Self {
		Self(AppId::default(), PhantomData)
	}
}

impl<T> Debug for CheckAppId<T>
where
	T: DAConfig + UtilityConfig + Send + Sync,
{
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "CheckAppId: {}", self.0)
	}

	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut Formatter) -> fmt::Result {
		Ok(())
	}
}

impl<T> SignedExtension for CheckAppId<T>
where
	T: DAConfig + UtilityConfig + Send + Sync,
	<T as frame_system::Config>::RuntimeCall:
		IsSubType<DACall<T>> + IsSubType<pallet_utility::Call<T>>,
{
	type AccountId = T::AccountId;
	type AdditionalSigned = ();
	type Call = <T as frame_system::Config>::RuntimeCall;
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

	fn pre_dispatch(
		self,
		_who: &Self::AccountId,
		call: &Self::Call,
		_info: &DispatchInfoOf<Self::Call>,
		_len: usize,
	) -> Result<Self::Pre, TransactionValidityError> {
		self.do_validate(call)?;
		Ok(())
	}

	fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
		Ok(())
	}
}

impl<T> GetAppId for CheckAppId<T>
where
	T: DAConfig + UtilityConfig + Send + Sync,
{
	#[inline]
	fn app_id(&self) -> AppId {
		self.0
	}
}

#[cfg(test)]
mod tests {
	use avail_core::InvalidTransactionCustomId::{ForbiddenAppId, InvalidAppId};
	use frame_system::pallet::Call as SysCall;
	use pallet_utility::pallet::Call as UtilityCall;
	use sp_runtime::transaction_validity::InvalidTransaction;
	use test_case::test_case;

	use super::*;
	use crate::{
		mock::{new_test_ext, RuntimeCall, Test},
		pallet::Call as DACall,
	};

	fn remark_call() -> RuntimeCall {
		RuntimeCall::System(SysCall::remark { remark: vec![] })
	}

	fn submit_data_call() -> RuntimeCall {
		RuntimeCall::DataAvailability(DACall::submit_data {
			data: vec![].try_into().unwrap(),
		})
	}

	fn batch_submit_call() -> RuntimeCall {
		let call = submit_data_call();
		RuntimeCall::Utility(UtilityCall::batch {
			calls: vec![call.clone(), call.clone(), call],
		})
	}

	fn batch_mixed_call() -> RuntimeCall {
		let call = submit_data_call();
		let remark = remark_call();
		RuntimeCall::Utility(UtilityCall::batch {
			calls: vec![remark, call.clone(), call],
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
	#[test_case(1, batch_submit_call() => Ok(ValidTransaction::default()); "utility batch filled with submit_data can be called with any valid AppId" )]
	#[test_case(1, batch_mixed_call() => to_invalid_tx(ForbiddenAppId); "utility batch filled with submit_data and remark cannot be called if AppId != 0" )]
	#[test_case(0, batch_mixed_call() => Ok(ValidTransaction::default()); "utility batch filled with submit_data and remark can be called if AppId == 0" )]
	fn do_validate_test(id: u32, call: RuntimeCall) -> TransactionValidity {
		new_test_ext().execute_with(|| CheckAppId::<Test>::from(AppId(id)).do_validate(&call))
	}
}
