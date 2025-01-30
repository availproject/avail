use super::MAX_ITERATIONS;
use crate::{Call as DACall, CheckBatchTransactions, Config as DAConfig, Pallet, LOG_TARGET};
use avail_core::{traits::GetAppId, AppId, InvalidTransactionCustomId};

use crate::extensions::native::hosted_commitment_builder::build_da_commitments;
use codec::{Decode, Encode};
use frame_support::{
	ensure,
	traits::{IsSubType, IsType},
};
use frame_system::{AllExtrinsicsLen, Config as SystemConfig, DynamicBlockLength, ExtrinsicLenOf};
use pallet_utility::{Call as UtilityCall, Config as UtilityConfig};
use pallet_vector::{Call as VectorCall, Config as VectorConfig};
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
	T: DAConfig + VectorConfig + UtilityConfig + Send + Sync,
	<T as SystemConfig>::RuntimeCall:
		IsSubType<DACall<T>> + IsSubType<UtilityCall<T>> + IsSubType<VectorCall<T>>,
	[u8; 32]: From<<T as frame_system::Config>::AccountId>,
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
	///  - It also ensures that Kate's evaluation grid can be generated during the header
	///  production.
	pub fn do_validate(
		&self,
		call: &<T as SystemConfig>::RuntimeCall,
		len: usize,
	) -> TransactionValidity {
		self.ensure_valid_app_id(call)?;
		// if let Some(DACall::<T>::submit_data { .. }) = call.is_sub_type() {
		// 	let all_extrinsics_len = self
		// 		.next_all_extrinsics_len(len)
		// 		.ok_or(InvalidTransaction::ExhaustsResources)?;
		// 	AllExtrinsicsLen::<T>::put(all_extrinsics_len);
		// }

		CheckBatchTransactions::<T>::new().do_validate(call, len)?;
		if let Some(DACall::<T>::submit_data_with_commitments { data, commitments }) =
			call.is_sub_type()
		{
			ensure!(
				!commitments.to_vec().is_empty(),
				InvalidTransaction::Custom(0)
			);

			// Fetch the block_length value from the frame_system pallet
			let block_length = frame_system::Pallet::<T>::block_length();
			let seed = [0u8; 32];

			let generated_commitments =
				build_da_commitments(data.to_vec().clone(), block_length, seed);
			ensure!(
				generated_commitments == commitments.to_vec(),
				InvalidTransaction::Custom(1)
			);
		}
		Ok(ValidTransaction::default())
	}

	fn next_all_extrinsics_len(&self, len: usize) -> Option<ExtrinsicLenOf<T>> {
		let app_id = self.app_id();
		let len = u32::try_from(len).ok()?;

		// Get maximum padded length of current block length.
		let curr_len = DynamicBlockLength::<T>::get();
		let max_scalars = curr_len.rows.0.checked_mul(curr_len.cols.0)?;

		// Update extrinsics length info.
		let mut all_extrinsics_len = AllExtrinsicsLen::<T>::get().unwrap_or_default();
		let _ = all_extrinsics_len.add_padded(app_id, len)?;

		// Calculate total padded length
		let total_scalars = all_extrinsics_len.total_num_scalars()?;

		if total_scalars < max_scalars {
			Some(all_extrinsics_len)
		} else {
			log::warn!(
				target: LOG_TARGET,
				"Padded block length (max {max_scalars} scalars) is exhausted, requested {total_scalars}");

			None
		}
	}

	fn ensure_valid_app_id(
		&self,
		call: &<T as SystemConfig>::RuntimeCall,
	) -> Result<(), TransactionValidityError> {
		if self.app_id() == AppId(0) {
			return Ok(());
		}

		let mut stack = Vec::new();
		stack.push(call);

		let mut maybe_next_app_id: Option<AppId> = None;
		let mut iterations = 0;

		while let Some(call) = stack.pop() {
			if let Some(DACall::<T>::submit_data { .. })
			| Some(DACall::<T>::submit_data_with_commitments { .. }) = call.is_sub_type()
			{
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

		Ok(())
	}
}

impl<T: DAConfig + UtilityConfig + VectorConfig + Send + Sync> Default for CheckAppId<T> {
	fn default() -> Self {
		Self(AppId::default(), PhantomData)
	}
}

impl<T> Debug for CheckAppId<T>
where
	T: DAConfig + UtilityConfig + VectorConfig + Send + Sync,
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
	T: DAConfig + VectorConfig + UtilityConfig + Send + Sync,
	<T as frame_system::Config>::RuntimeCall:
		IsSubType<DACall<T>> + IsSubType<pallet_utility::Call<T>> + IsSubType<VectorCall<T>>,
	[u8; 32]: From<<T as frame_system::Config>::AccountId>,
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
		len: usize,
	) -> TransactionValidity {
		self.do_validate(call, len)
	}

	fn pre_dispatch(
		self,
		_who: &Self::AccountId,
		call: &Self::Call,
		_info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> Result<Self::Pre, TransactionValidityError> {
		self.do_validate(call, len)?;
		Ok(())
	}

	fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
		Ok(())
	}
}

impl<T> GetAppId for CheckAppId<T>
where
	T: DAConfig + UtilityConfig + VectorConfig + Send + Sync,
{
	#[inline]
	fn app_id(&self) -> AppId {
		self.0
	}
}

#[cfg(test)]
mod tests {
	use avail_core::{
		asdr::AppUncheckedExtrinsic,
		InvalidTransactionCustomId::{ForbiddenAppId, InvalidAppId},
	};
	use frame_system::pallet::Call as SysCall;
	use sp_runtime::transaction_validity::InvalidTransaction;
	use test_case::test_case;

	use super::*;
	use crate::extensions::extensions_mock::{new_test_ext, RuntimeCall, Test};
	use crate::pallet::Call as DACall;

	fn remark_call() -> RuntimeCall {
		RuntimeCall::System(SysCall::remark { remark: vec![] })
	}

	fn submit_data_call() -> RuntimeCall {
		RuntimeCall::DataAvailability(DACall::submit_data {
			data: vec![].try_into().unwrap(),
		})
	}

	fn to_invalid_tx(custom_id: InvalidTransactionCustomId) -> TransactionValidity {
		Err(TransactionValidityError::Invalid(
			InvalidTransaction::Custom(custom_id as u8),
		))
	}

	#[test_case(1, submit_data_call() => Ok(ValidTransaction::default()); "Submit Data call should be allowed to use any valid AppId" )]
	#[test_case(100, submit_data_call() => to_invalid_tx(InvalidAppId); "Submit Data call with invalid AppId should be blocked" )]
	#[test_case(0, remark_call() => Ok(ValidTransaction::default()); "Any Non-Submit-Data call with AppId == 0 should be allowed" )]
	#[test_case(1, remark_call() => to_invalid_tx(ForbiddenAppId); "Any Non-Submit-Data call with valid AppId != 0 should be blocked" )]
	fn do_validate_test(id: u32, call: RuntimeCall) -> TransactionValidity {
		let extrinsic =
			AppUncheckedExtrinsic::<u32, RuntimeCall, (), ()>::new_unsigned(call.clone());
		let len = extrinsic.encoded_size();
		new_test_ext().execute_with(|| CheckAppId::<Test>::from(AppId(id)).do_validate(&call, len))
	}
}
