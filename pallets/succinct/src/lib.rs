#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{pallet_prelude::*, parameter_types};
use hex_literal::hex;
use sp_core::{H256, U256};

pub use pallet::*;

use crate::verifier::Verifier;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
// mod verify;
mod state;
pub(crate) mod verifier;
mod weights;

type VerificationKeyDef<T> = BoundedVec<u8, <T as Config>::MaxVerificationKeyLength>;

parameter_types! {
	pub const MaxPublicInputsLength: u32 = 9;
	pub const MaxVerificationKeyLength: u32 = 4143;
	pub const MaxProofLength: u32 = 1133;

	// TODO set function verifiers
	pub const StepFunctionId: H256 = H256(hex!("af44af6890508b3b7f6910d4a4570a0d524769a23ce340b2c7400e140ad168ab"));
	pub const RotateFunctionId: H256 = H256(hex!("9aed23f9e6e8f8b98751cf508069b5b7f015d4d510b6a4820d41ba1ce88190d9"));

	// Constants
	pub const MinSyncCommitteeParticipants: u16=10;
	pub const SyncCommitteeSize: u32=512;
	pub const FinalizedRootIndex: u32=105;
	pub const NextSyncCommitteeIndex: u32= 55;
	pub const ExecutionStateRootIndex: u32= 402;

	pub const InputMaxLen: u32 = 256;
	pub const OutputMaxLen: u32 = 512;
	pub const ProofMaxLen: u32 = 2048;
}

#[frame_support::pallet]
pub mod pallet {
	use ark_std::string::String;
	use ark_std::{vec, vec::Vec};
	use ethabi::Token;
	use frame_support::dispatch::{GetDispatchInfo, UnfilteredDispatchable};
	use frame_support::traits::{Hash, UnixTime};
	use frame_support::{pallet_prelude::ValueQuery, DefaultNoBound};
	use sp_core::H256;
	use sp_io::hashing::sha2_256;

	use frame_system::pallet_prelude::*;
	pub use weights::WeightInfo;

	use crate::state::{
		parse_rotate_output, parse_step_output, State, VerifiedRotateCallStore,
		VerifiedStepCallStore, VerifiedStepOutput,
	};
	use crate::verifier::encode_packed;

	use super::*;

	#[pallet::error]
	pub enum Error<T> {
		UpdaterMisMatch,
		VerificationError,
		NotEnoughParticipants,
		TooLongVerificationKey,
		VerificationKeyIsNotSet,
		MalformedVerificationKey,
		NotSupportedCurve,
		NotSupportedProtocol,
		StepVerificationError,
		RotateVerificationError,
		HeaderRootNotSet,
		VerificationFailed,
		FunctionIdNotRecognised,
		HeaderRootAlreadySet,
		StateRootAlreadySet,
		SyncCommitteeAlreadySet,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		// emit event once the head is updated
		HeaderUpdate {
			slot: u64,
			finalization_root: H256,
		},
		// emit event once the sync committee updates
		SyncCommitteeUpdate {
			period: u64,
			root: U256,
		},
		// emit event when verification setup is completed
		VerificationSetupCompleted,
		// emit event if verification is success
		VerificationSuccess {
			who: H256,
			attested_slot: u64,
			finalized_slot: u64,
		},
		// emit when new updater is set
		NewUpdater {
			old: H256,
			new: H256,
		},
	}

	// The latest slot the light client has a finalized header for.
	#[pallet::storage]
	pub type Head<T: Config> = StorageValue<_, u64, ValueQuery>;

	// Maps from a slot to a block header root.
	#[pallet::storage]
	pub type Headers<T> = StorageMap<_, Identity, u64, H256, ValueQuery>;

	// Maps slot to the timestamp of when the headers mapping was updated with slot as a key
	#[pallet::storage]
	pub type Timestamps<T> = StorageMap<_, Identity, u64, u64, ValueQuery>;

	// Maps from a slot to the current finalized ethereum execution state root.
	#[pallet::storage]
	pub type ExecutionStateRoots<T> = StorageMap<_, Identity, u64, H256, ValueQuery>;

	// Maps from a period to the poseidon commitment for the sync committee.
	#[pallet::storage]
	pub type SyncCommitteePoseidons<T> = StorageMap<_, Identity, u64, U256, ValueQuery>;

	//TODO step and rotate verification keys can be stored as constants and not in the storage which can simplify implementation.
	#[pallet::storage]
	pub type StepVerificationKeyStorage<T: Config> =
		StorageValue<_, VerificationKeyDef<T>, ValueQuery>;

	#[pallet::storage]
	pub type RotateVerificationKeyStorage<T: Config> =
		StorageValue<_, VerificationKeyDef<T>, ValueQuery>;

	// Storage for a general state.
	#[pallet::storage]
	pub type StateStorage<T: Config> = StorageValue<_, State, ValueQuery>;

	#[pallet::storage]
	pub type VerifiedStepCall<T> = StorageValue<_, VerifiedStepCallStore, ValueQuery>;

	#[pallet::storage]
	pub type VerifiedRotateCall<T> = StorageValue<_, VerifiedRotateCallStore, ValueQuery>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type TimeProvider: UnixTime;
		#[pallet::constant]
		type MaxPublicInputsLength: Get<u32>;
		// 9
		#[pallet::constant]
		type MaxProofLength: Get<u32>;
		// 1133
		#[pallet::constant]
		type MaxVerificationKeyLength: Get<u32>;
		// 4143
		#[pallet::constant]
		type MinSyncCommitteeParticipants: Get<u32>;
		#[pallet::constant]
		type SyncCommitteeSize: Get<u32>;
		#[pallet::constant]
		type FinalizedRootIndex: Get<u32>;
		#[pallet::constant]
		type NextSyncCommitteeIndex: Get<u32>;
		#[pallet::constant]
		type ExecutionStateRootIndex: Get<u32>;

		#[pallet::constant]
		type StepFunctionId: Get<H256>;

		#[pallet::constant]
		type RotateFunctionId: Get<H256>;

		type RuntimeCall: Parameter
			+ UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ GetDispatchInfo;

		type WeightInfo: WeightInfo;
	}

	//  pallet initialization data
	// TODO check if genesis is a good place for this
	#[pallet::genesis_config]
	#[derive(DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub updater: Hash,
		pub slots_per_period: u64,
		pub source_chain_id: u32,
		pub finality_threshold: u16,
		pub sync_committee_poseidon: U256,
		pub period: u64,
		pub _phantom: PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		// TODO init state
		fn build(&self) {
			// TODO time cannot be called at Genesis
			// T::TimeProvider::now().as_secs()
			// Preconfigure init data
			<StateStorage<T>>::put(State {
				updater: self.updater,
				slots_per_period: self.slots_per_period,
				source_chain_id: self.source_chain_id,
				finality_threshold: self.finality_threshold,
			});

			Head::<T>::set(0);
			<SyncCommitteePoseidons<T>>::insert(self.period, self.sync_committee_poseidon);
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		[u8; 32]: From<T::AccountId>,
	{
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::step())]
		pub fn fulfill_call(
			origin: OriginFor<T>,
			function_id: H256,
			input: BoundedVec<u8, InputMaxLen>,
			output: BoundedVec<u8, OutputMaxLen>,
			proof: BoundedVec<u8, ProofMaxLen>,
			slot: u64,
		) -> DispatchResult {
			let sender: [u8; 32] = ensure_signed(origin)?.into();
			let state = StateStorage::<T>::get();
			// ensure sender is preconfigured
			ensure!(H256(sender) == state.updater, Error::<T>::UpdaterMisMatch);
			let input_hash = H256(sha2_256(input.as_slice()));
			let output_hash = H256(sha2_256(output.as_slice()));
			let verifier = Self::get_verifier(function_id)?;

			let success = verifier
				.verify(input_hash, output_hash, proof.to_vec())
				.map_err(|_| Error::<T>::VerificationError)?;

			ensure!(success, Error::<T>::VerificationFailed);

			if function_id == StepFunctionId::get() {
				let vs = VerifiedStepCallStore::new(
					function_id,
					input_hash,
					parse_step_output(output.to_vec()),
				);
				VerifiedStepCall::<T>::set(vs);
				if Self::step_into(slot, state)? {
					Self::deposit_event(Event::HeaderUpdate {
						slot,
						finalization_root: vs.verified_output.finalized_header_root,
					});
				}
			} else if function_id == RotateFunctionId::get() {
				let vr = VerifiedRotateCallStore::new(
					function_id,
					input_hash,
					parse_rotate_output(output.to_vec()),
				);

				VerifiedRotateCall::<T>::set(vr);
				if Self::rotate_into(slot, state)? {
					Self::deposit_event(Event::SyncCommitteeUpdate {
						period: slot,
						root: vr.sync_committee_poseidon,
					});
				}
			} else {
				return Err(Error::<T>::FunctionIdNotRecognised.into());
			}

			Ok(())
		}

		/// Sets updater that can call step and rotate functions
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::step())]
		pub fn set_updater(origin: OriginFor<T>, updater: H256) -> DispatchResult {
			ensure_root(origin)?;
			let old = StateStorage::<T>::get();
			StateStorage::<T>::try_mutate(|cfg| -> Result<(), DispatchError> {
				cfg.updater = updater;
				Ok(())
			})?;

			Self::deposit_event(Event::<T>::NewUpdater {
				old: old.updater,
				new: updater,
			});
			Ok(())
		}

		/// Sets verification public inputs for step function.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::step())]
		pub fn setup_step_verification(
			origin: OriginFor<T>,
			verification: String,
		) -> DispatchResult {
			ensure_root(origin)?;
			// try from json to Verifier struct
			Verifier::from_json_u8_slice(verification.as_bytes())
				.map_err(|_| Error::<T>::MalformedVerificationKey)?;
			// store verification to storage
			Self::store_step_verification_key(verification.as_bytes().to_vec())?;

			Self::deposit_event(Event::<T>::VerificationSetupCompleted);
			Ok(())
		}

		/// Sets verification public inputs for rotate function.
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::step())]
		pub fn setup_rotate_verification(
			origin: OriginFor<T>,
			verification: String,
		) -> DispatchResult {
			ensure_root(origin)?;
			// try from json to Verifier struct
			Verifier::from_json_u8_slice(verification.as_bytes())
				.map_err(|_| Error::<T>::MalformedVerificationKey)?;
			// store verification to storage
			Self::store_rotate_verification_key(verification.as_bytes().to_vec())?;

			Self::deposit_event(Event::<T>::VerificationSetupCompleted);
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn rotate_into(finalized_slot: u64, state: State) -> Result<bool, DispatchError> {
			let finalized_header_root = Headers::<T>::get(finalized_slot);
			ensure!(
				finalized_header_root != H256::zero(),
				Error::<T>::HeaderRootNotSet
			);

			let input = ethabi::encode(&[Token::FixedBytes(finalized_header_root.0.to_vec())]);
			let sync_committee_poseidon: U256 =
				Self::verified_rotate_call(RotateFunctionId::get(), input)?;

			let current_period = finalized_slot / state.slots_per_period;
			let next_period = current_period + 1;

			let is_set = Self::set_sync_committee_poseidon(next_period, sync_committee_poseidon)?;

			Ok(is_set)
		}

		fn step_into(attested_slot: u64, state: State) -> Result<bool, DispatchError> {
			let current_period = attested_slot / state.slots_per_period;
			let sc_poseidon = SyncCommitteePoseidons::<T>::get(current_period);

			let input = encode_packed(sc_poseidon, attested_slot);
			let result = Self::verified_step_call(StepFunctionId::get(), input)?;

			ensure!(
				result.participation >= state.finality_threshold,
				Error::<T>::NotEnoughParticipants
			);

			let updated = Self::set_slot_roots(result)?;

			Ok(updated)
		}

		fn set_slot_roots(step_output: VerifiedStepOutput) -> Result<bool, DispatchError> {
			let header = Headers::<T>::get(step_output.finalized_slot);

			ensure!(header == H256::zero(), Error::<T>::HeaderRootAlreadySet);

			let state_root = ExecutionStateRoots::<T>::get(step_output.finalized_slot);

			ensure!(state_root == H256::zero(), Error::<T>::StateRootAlreadySet);

			Head::<T>::set(step_output.finalized_slot);

			Headers::<T>::insert(
				step_output.finalized_slot,
				step_output.finalized_header_root,
			);

			ExecutionStateRoots::<T>::insert(
				step_output.finalized_slot,
				step_output.execution_state_root,
			);

			Timestamps::<T>::insert(step_output.finalized_slot, T::TimeProvider::now().as_secs());

			Ok(true)
		}

		fn set_sync_committee_poseidon(period: u64, poseidon: U256) -> Result<bool, DispatchError> {
			let sync_committee_poseidons = SyncCommitteePoseidons::<T>::get(period);

			ensure!(
				sync_committee_poseidons == U256::zero(),
				Error::<T>::SyncCommitteeAlreadySet
			);

			SyncCommitteePoseidons::<T>::set(period, poseidon);

			Ok(true)
		}

		fn get_verifier(function_id: H256) -> Result<Verifier, Error<T>> {
			if function_id == StepFunctionId::get() {
				Self::get_step_verifier()
			} else {
				Self::get_rotate_verifier()
			}
		}

		fn get_step_verifier() -> Result<Verifier, Error<T>> {
			let vk = StepVerificationKeyStorage::<T>::get();
			ensure!(!vk.is_empty(), Error::<T>::VerificationKeyIsNotSet);
			let deserialized_vk = Verifier::from_json_u8_slice(vk.as_slice())
				.map_err(|_| Error::<T>::MalformedVerificationKey)?;
			Ok(deserialized_vk)
		}

		fn get_rotate_verifier() -> Result<Verifier, Error<T>> {
			let vk = RotateVerificationKeyStorage::<T>::get();
			ensure!(!vk.is_empty(), Error::<T>::VerificationKeyIsNotSet);
			let deserialized_vk = Verifier::from_json_u8_slice(vk.as_slice())
				.map_err(|_| Error::<T>::MalformedVerificationKey)?;
			Ok(deserialized_vk)
		}

		fn store_step_verification_key(vec_vk: Vec<u8>) -> Result<Verifier, Error<T>> {
			let vk: VerificationKeyDef<T> = vec_vk
				.try_into()
				.map_err(|_| Error::<T>::TooLongVerificationKey)?;
			let deserialized_vk = Verifier::from_json_u8_slice(vk.as_slice())
				.map_err(|_| Error::<T>::MalformedVerificationKey)?;
			ensure!(
				deserialized_vk.vk_json.curve == *"bn128",
				Error::<T>::NotSupportedCurve
			);
			ensure!(
				deserialized_vk.vk_json.protocol == *"groth16",
				Error::<T>::NotSupportedProtocol
			);

			StepVerificationKeyStorage::<T>::put(vk);
			Ok(deserialized_vk)
		}

		fn store_rotate_verification_key(vec_vk: Vec<u8>) -> Result<Verifier, Error<T>> {
			let vk: VerificationKeyDef<T> = vec_vk
				.try_into()
				.map_err(|_| Error::<T>::TooLongVerificationKey)?;
			let deserialized_vk = Verifier::from_json_u8_slice(vk.as_slice())
				.map_err(|_| Error::<T>::MalformedVerificationKey)?;
			ensure!(
				deserialized_vk.vk_json.curve == *"bn128",
				Error::<T>::NotSupportedCurve
			);
			ensure!(
				deserialized_vk.vk_json.protocol == *"groth16",
				Error::<T>::NotSupportedProtocol
			);

			RotateVerificationKeyStorage::<T>::put(vk);
			Ok(deserialized_vk)
		}

		fn verified_step_call(
			function_id: H256,
			input: ethabi::Bytes,
		) -> Result<VerifiedStepOutput, DispatchError> {
			let input_hash = sha2_256(input.as_slice());
			let verified_call = VerifiedStepCall::<T>::get();
			if verified_call.verified_function_id == function_id
				&& verified_call.verified_input_hash == H256(input_hash)
			{
				let trait_object: VerifiedStepOutput = verified_call.verified_output;
				Ok(trait_object)
			} else {
				Err(Error::<T>::StepVerificationError.into())
			}
		}

		fn verified_rotate_call(
			function_id: H256,
			input: ethabi::Bytes,
		) -> Result<U256, DispatchError> {
			let input_hash = sha2_256(input.as_slice());
			let verified_call = VerifiedRotateCall::<T>::get();

			if verified_call.verified_function_id == function_id
				&& verified_call.verified_input_hash == H256(input_hash)
			{
				Ok(verified_call.sync_committee_poseidon)
			} else {
				Err(Error::<T>::RotateVerificationError.into())
			}
		}
	}

	pub fn parse_slot(callback_data: Vec<u8>) -> u64 {
		let mut slot_data: [u8; 8] = [0; 8];
		slot_data[..8].copy_from_slice(&callback_data[callback_data.len() - 8..]);
		u64::from_be_bytes(slot_data)
	}
}
