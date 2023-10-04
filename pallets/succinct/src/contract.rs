use crate::Error;
use codec::{Decode, Encode};
use frame_support::dispatch::TypeInfo;
use frame_support::{Deserialize, Serialize};
use frame_system::Config;
use sp_runtime::DispatchError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, TypeInfo)]
pub enum ContractError {
	InvalidStepProof,
	SyncCommitteeNotInitialized,
	NotEnoughSyncCommitteeParticipants,
	ProofNotValid,
	VerificationError,
}

// impl<T: Config> Into<Error<T>> for ContractError {
//     fn into(self) -> Error<T> {
//         match self {
//             ContractError::VerificationError => {
//                 Error::<T>::VerificationError
//             }
//             _ => {
//                 Error::<T>::VerificationError
//             }
//         }
//         // Number { value: self }
//     }
// }
//
impl<T: Config> From<ContractError> for Error<T> {
	fn from(e: ContractError) -> Error<T> {
		match e {
			ContractError::InvalidStepProof => Error::<T>::VerificationError,
			// ContractError::SyncCommitteeNotInitialized => {}
			// ContractError::NotEnoughSyncCommitteeParticipants => {}
			// ContractError::ProofNotValid => {}
			// ContractError::VerificationError => {}
			_ => Error::<T>::VerificationError,
		}
	}
}
