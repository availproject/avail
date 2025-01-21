use crate::{Call as DACall, Config as DAConfig, LOG_TARGET};
use avail_core::{traits::GetDaCommitments, DaCommitments};
use codec::{Decode, Encode};
use frame_support::{ensure, traits::IsSubType};
use frame_system::Config as SystemConfig;
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{DispatchInfoOf, SignedExtension},
	transaction_validity::{
		InvalidTransaction, TransactionLongevity, TransactionValidity, TransactionValidityError,
		ValidTransaction,
	},
};
use sp_std::{
	default::Default,
	fmt::{self, Debug, Formatter},
	marker::PhantomData,
};

use super::native::build_da_commitments::{build_da_commitments, DaCommitmentsError};
use sp_core::hexdisplay::HexDisplay;

/// Check for DA Commitments.
///
/// # Transaction Validity
///
/// Ensures that the DA Commitments are valid.
///
#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CheckDaCommitments<T: DAConfig + Send + Sync>(pub DaCommitments, PhantomData<T>);

impl<T> CheckDaCommitments<T>
where
	T: DAConfig + Send + Sync,
	<T as SystemConfig>::RuntimeCall: IsSubType<DACall<T>>,
{
	pub fn new() -> Self {
		Self(Default::default(), PhantomData)
	}

	/// Utility constructor. Used only in client/factory code.
	pub fn from(da_commitments: DaCommitments) -> Self {
		Self(da_commitments, PhantomData)
	}

	/// Validates the DA Commitments.
	pub fn do_validate(
		&self,
		call: &<T as SystemConfig>::RuntimeCall,
		_len: usize,
	) -> TransactionValidity {
		log::info!(target: LOG_TARGET, "CheckDaCommitments::do_validate");
		if let Some(DACall::<T>::submit_data { data }) = call.is_sub_type() {
			ensure!(
				!self.da_commitments().is_empty(),
				InvalidTransaction::Custom(0)
			);

			// Fetch the block_length value from the frame_system pallet
			let block_length = frame_system::Pallet::<T>::block_length();
			let seed = [0u8; 32];

			match build_da_commitments(data.to_vec().clone(), block_length, seed) {
				Ok(commitments) => {
					// log::info!(target: LOG_TARGET, "Generated commitments: {:?}", HexDisplay::from(&commitments));
					// log::info!(target: LOG_TARGET, "Passed commitments: {:?}", HexDisplay::from(&self.da_commitments()));
					ensure!(
						commitments == self.da_commitments(),
						InvalidTransaction::Custom(1)
					);
					// log::info!(target: LOG_TARGET, "CheckDaCommitments::do_validate -> passed");
				},
				Err(DaCommitmentsError::GridConstructionFailed(_)) => {
					return Err(TransactionValidityError::Invalid(
						InvalidTransaction::Custom(2),
					));
				},
				Err(DaCommitmentsError::MakePolynomialGridFailed(_)) => {
					return Err(TransactionValidityError::Invalid(
						InvalidTransaction::Custom(3),
					));
				},
				Err(DaCommitmentsError::GridExtensionFailed(_)) => {
					return Err(TransactionValidityError::Invalid(
						InvalidTransaction::Custom(4),
					));
				},
				Err(DaCommitmentsError::CommitmentSerializationFailed(_)) => {
					return Err(TransactionValidityError::Invalid(
						InvalidTransaction::Custom(5),
					));
				},
			}
		}
		Ok(ValidTransaction::default())
	}
}

impl<T> Debug for CheckDaCommitments<T>
where
	T: DAConfig + Send + Sync,
{
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "CheckDaCommitments: {:?}", self.0)
	}

	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut Formatter) -> fmt::Result {
		Ok(())
	}
}

impl<T> SignedExtension for CheckDaCommitments<T>
where
	T: DAConfig + Send + Sync,
	<T as frame_system::Config>::RuntimeCall: IsSubType<DACall<T>>,
{
	type AccountId = T::AccountId;
	type AdditionalSigned = ();
	type Call = <T as frame_system::Config>::RuntimeCall;
	type Pre = ();

	const IDENTIFIER: &'static str = "CheckDaCommitments";

	fn validate(
		&self,
		_who: &Self::AccountId,
		call: &Self::Call,
		_info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> TransactionValidity {
		self.do_validate(call, len)?;
		Ok(ValidTransaction {
			longevity: TransactionLongevity::max_value(),
			..Default::default()
		})
	}

	fn pre_dispatch(
		self,
		_who: &Self::AccountId,
		_call: &Self::Call,
		_info: &DispatchInfoOf<Self::Call>,
		_len: usize,
	) -> Result<Self::Pre, TransactionValidityError> {
		// self.do_validate(call, len)?;
		Ok(())
	}

	fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
		Ok(())
	}
}

impl<T> GetDaCommitments for CheckDaCommitments<T>
where
	T: DAConfig + Send + Sync,
{
	#[inline]
	fn da_commitments(&self) -> DaCommitments {
		self.0.clone()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use avail_core::constants::kate::COMMITMENT_SIZE;

	#[test]
	fn check_da_commitments_default() {
		let check_da_commitments = CheckDaCommitments::<()>::default();
		assert_eq!(check_da_commitments.da_commitments(), DaCommitments::new());
	}

	#[test]
	fn check_da_commitments_custom() {
		let da_commitments = vec![[0u8; COMMITMENT_SIZE]];
		let check_da_commitments = CheckDaCommitments::<()>::from(da_commitments.clone());
		assert_eq!(check_da_commitments.da_commitments(), da_commitments);
	}
}
