#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{pallet_prelude::*, parameter_types};
use sp_core::U256;

pub use pallet::*;

use crate::verifier::Verifier;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod common;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
// mod verify;
mod state;
mod verifier;
mod weights;

type PublicInputsDef<T> = BoundedVec<u8, <T as Config>::MaxPublicInputsLength>;
type VerificationKeyDef<T> = BoundedVec<u8, <T as Config>::MaxVerificationKeyLength>;

parameter_types! {
	pub const  MinSyncCommitteeParticipants: u16=10;
	pub const  SyncCommitteeSize: u32=512;
	pub const  FinalizedRootIndex: u32=105;
	pub const  NextSyncCommitteeIndex: u32= 55;
	pub const  ExecutionStateRootIndex: u32= 402;
}

#[frame_support::pallet]
pub mod pallet {
	use ark_std::string::String;
	use ark_std::string::ToString;
	use ark_std::{vec, vec::Vec};
	use frame_support::dispatch::{GetDispatchInfo, UnfilteredDispatchable};
	use frame_support::traits::{Hash, UnixTime};
	use frame_support::{pallet_prelude::ValueQuery, DefaultNoBound};
	use sp_core::H256;

	use frame_system::pallet_prelude::*;
	pub use weights::WeightInfo;

	use crate::state::{LightClientStep, State};
	use crate::verifier::zk_light_client_rotate;
	use crate::verifier::zk_light_client_step;

	use super::*;

	#[pallet::error]
	pub enum Error<T> {
		UpdaterMisMatch,
		CannotChangeUpdater,
		VerificationError,
		CannotUpdateStateStorage,

		UpdateSlotIsFarInTheFuture,
		UpdateSlotLessThanCurrentHead,
		NotEnoughParticipants,
		SyncCommitteeNotInitialized,
		NotEnoughSyncCommitteeParticipants,
		MinSCNotSigned,
		// verification
		ProofNotValid,
		PublicInputsMismatch,
		TooLongPublicInputs,
		TooLongVerificationKey,
		TooLongProof,
		ProofIsEmpty,
		VerificationKeyIsNotSet,
		MalformedVerificationKey,
		MalformedProof,
		MalformedPublicInputs,
		NotSupportedCurve,
		NotSupportedProtocol,
		ProofVerificationError,
		ProofCreationError,
		VerificationKeyCreationError,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		// emit event once the head is updated
		HeadUpdate { slot: u64, finalization_root: Hash },
		SyncCommitteeUpdate { period: u64, root: U256 },
		VerificationSetupCompleted,
		VerificationProofSet,
		VerificationSuccess { who: H256 },
		NewUpdater { old: H256, new: H256 },
		VerificationFailed,
	}

	// Storage definitions
	#[pallet::storage]
	pub type PublicInputStorage<T: Config> = StorageValue<_, PublicInputsDef<T>, ValueQuery>;
	#[pallet::storage]
	pub type VerificationKeyStorage<T: Config> = StorageValue<_, VerificationKeyDef<T>, ValueQuery>;

	#[pallet::storage]
	pub type RotateVerificationKeyStorage<T: Config> =
		StorageValue<_, VerificationKeyDef<T>, ValueQuery>;

	#[pallet::storage]
	pub type StateStorage<T: Config> = StorageValue<_, State, ValueQuery>;

	// @notice Maps from a slot to a beacon block header root.
	#[pallet::storage]
	#[pallet::getter(fn get_header)]
	pub type Headers<T> = StorageMap<_, Identity, u64, Hash, ValueQuery>;

	// Maps from a slot to the timestamp of when the headers mapping was updated with slot as a key
	#[pallet::storage]
	#[pallet::getter(fn get_timestamp)]
	pub type Timestamps<T> = StorageMap<_, Identity, u64, u64, ValueQuery>;

	// Maps from a slot to the current finalized ethereum1 execution state root.
	#[pallet::storage]
	#[pallet::getter(fn get_state_root)]
	pub type ExecutionStateRoots<T> = StorageMap<_, Identity, u64, Hash, ValueQuery>;

	// Maps from a period to the poseidon commitment for the sync committee.
	#[pallet::storage]
	#[pallet::getter(fn get_poseidon)]
	pub type SyncCommitteePoseidons<T> = StorageMap<_, Identity, u64, U256, ValueQuery>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type TimeProvider: UnixTime;
		#[pallet::constant]
		type MaxPublicInputsLength: Get<u32>;
		#[pallet::constant]
		type MaxProofLength: Get<u32>;
		#[pallet::constant]
		type MaxVerificationKeyLength: Get<u32>;
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

		type RuntimeCall: Parameter
			+ UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ GetDispatchInfo;

		type WeightInfo: WeightInfo;

		type ApprovedOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	//  pallet initialization data
	// TODO check if genesis is a good place for this
	#[pallet::genesis_config]
	#[derive(DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub updater: Hash,
		pub genesis_validators_root: Hash,
		pub genesis_time: u64,
		pub seconds_per_slot: u64,
		pub slots_per_period: u64,
		pub source_chain_id: u32,
		pub finality_threshold: u16,
		pub consistent: bool,
		pub head: u64,
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
				genesis_validators_root: H256::zero(),
				genesis_time: 1696440023,
				seconds_per_slot: 12000,
				slots_per_period: 8192,
				source_chain_id: 1,
				finality_threshold: 290,
				head: 0,
				consistent: true,
			});

			let s = U256::from_dec_str(
				"7032059424740925146199071046477651269705772793323287102921912953216115444414",
			)
			.unwrap();
			<SyncCommitteePoseidons<T>>::insert(0u64, s);
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		[u8; 32]: From<T::AccountId>,
	{
		/// Sets the sync committee for the next sync committee period.
		/// A commitment to the the next sync committee is signed by the current sync committee.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::rotate())]
		pub fn rotate(origin: OriginFor<T>, update: state::LightClientRotate) -> DispatchResult {
			let sender: [u8; 32] = ensure_signed(origin)?.into();
			let state = StateStorage::<T>::get();
			ensure!(H256(sender) == state.updater, Error::<T>::UpdaterMisMatch);

			let step = &update.step;

			let finalized = process_step::<T>(state, step)?;
			let current_period = get_sync_committee_period(state, step.finalized_slot);
			let next_period = current_period + 1;

			let verifier = get_rotate_verifier::<T>()?;

			// proof verification
			let success = zk_light_client_rotate(&update, verifier)
				.map_err(|_| Error::<T>::VerificationError)?;

			if success {
				Self::deposit_event(Event::VerificationSuccess { who: sender.into() });
				if finalized {
					let is_set = set_sync_committee_poseidon::<T>(
						next_period,
						update.sync_committee_poseidon,
					)?;
					if is_set {
						Self::deposit_event(Event::SyncCommitteeUpdate {
							period: next_period,
							root: update.sync_committee_poseidon,
						});
					}
				}
			} else {
				Self::deposit_event(Event::VerificationFailed);
			}

			Ok(())
		}

		/// Updates the head of the light client to the provided slot.
		/// The conditions for updating the head of the light client involve checking:
		///      1) Enough signatures from the current sync committee for n=512
		///      2) A valid finality proof
		///      3) A valid execution state root proof
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::step())]
		pub fn step(origin: OriginFor<T>, update: LightClientStep) -> DispatchResult {
			let sender: [u8; 32] = ensure_signed(origin)?.into();
			let state = StateStorage::<T>::get();
			// ensure sender is preconfigured
			ensure!(H256(sender) == state.updater, Error::<T>::UpdaterMisMatch);

			let finalized = process_step::<T>(state, &update)?;

			let current_slot = get_current_slot::<T>(state.genesis_time, state.seconds_per_slot);

			ensure!(
				current_slot <= update.attested_slot,
				Error::<T>::UpdateSlotIsFarInTheFuture
			);

			ensure!(
				update.finalized_slot >= state.head,
				Error::<T>::UpdateSlotLessThanCurrentHead
			);

			ensure!(finalized, Error::<T>::NotEnoughParticipants);

			let updated = set_slot_roots::<T>(
				update.finalized_slot,
				update.finalized_header_root,
				update.execution_state_root,
			)?;
			if updated {
				Self::deposit_event(Event::HeadUpdate {
					slot: update.finalized_slot,
					finalization_root: update.finalized_header_root,
				});
			}

			Ok(())
		}

		// sets updater that can call step and rotate functions
		#[pallet::call_index(2)]
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

		/// Sets the public input params into storage
		#[pallet::call_index(3)]
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
			store_step_verification_key::<T>(verification.as_bytes().to_vec())?;

			Self::deposit_event(Event::<T>::VerificationSetupCompleted);
			Ok(())
		}

		/// Sets the public input params into storage
		#[pallet::call_index(4)]
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
			store_rotate_verification_key::<T>(verification.as_bytes().to_vec())?;

			Self::deposit_event(Event::<T>::VerificationSetupCompleted);
			Ok(())
		}
	}

	fn get_current_slot<T: Config>(genesis_time: u64, seconds_per_slot: u64) -> u64 {
		//  TODO check block time provider
		let block_time: u64 = T::TimeProvider::now().as_secs();

		log::info!(
			"Genesis time: {}, block time: {}, slot per period: {}",
			genesis_time,
			block_time,
			seconds_per_slot
		);

		(block_time - genesis_time) / seconds_per_slot
	}

	fn set_slot_roots<T: Config>(
		slot: u64,
		finalized_header_root: H256,
		execution_state_root: H256,
	) -> Result<bool, DispatchError> {
		let header = Headers::<T>::get(slot);

		if header != H256::zero() && header != finalized_header_root {
			StateStorage::<T>::try_mutate(|m| -> Result<(), DispatchError> {
				m.consistent = false;
				Ok(())
			})
			.map_err(|_| Error::<T>::CannotUpdateStateStorage)?;
			return Ok(false);
		}
		let state_root = ExecutionStateRoots::<T>::get(slot);

		if state_root != H256::zero() && state_root != execution_state_root {
			StateStorage::<T>::try_mutate(|m| -> Result<(), DispatchError> {
				m.consistent = false;
				Ok(())
			})
			.map_err(|_| Error::<T>::CannotUpdateStateStorage)?;
			return Ok(false);
		}

		StateStorage::<T>::try_mutate(|m| -> Result<(), DispatchError> {
			m.head = slot;
			Ok(())
		})
		.map_err(|_| Error::<T>::CannotUpdateStateStorage)?;

		Headers::<T>::insert(slot, finalized_header_root);

		// TODO can this time be used as block time?
		// IS this really needed
		Timestamps::<T>::insert(slot, T::TimeProvider::now().as_secs());

		Ok(true)
	}

	fn set_sync_committee_poseidon<T: Config>(
		period: u64,
		poseidon: U256,
	) -> Result<bool, DispatchError> {
		let sync_committee_poseidons = SyncCommitteePoseidons::<T>::get(period);

		if poseidon != U256::zero() && sync_committee_poseidons != poseidon {
			StateStorage::<T>::try_mutate(|m| -> Result<(), DispatchError> {
				m.consistent = false;
				Ok(())
			})
			.map_err(|_| Error::<T>::CannotUpdateStateStorage)?;
			return Ok(false);
		}
		SyncCommitteePoseidons::<T>::set(period, poseidon);

		Ok(true)
	}

	fn get_sync_committee_period(state: State, slot: u64) -> u64 {
		slot / state.slots_per_period
	}

	fn process_step<T: Config>(
		state: State,
		update: &LightClientStep,
	) -> Result<bool, DispatchError> {
		let current_period = get_sync_committee_period(state, update.attested_slot);
		let sc_poseidon = SyncCommitteePoseidons::<T>::get(current_period);

		ensure!(
			sc_poseidon != U256::zero(),
			Error::<T>::SyncCommitteeNotInitialized
		);
		ensure!(
			update.participation >= MinSyncCommitteeParticipants::get(),
			Error::<T>::NotEnoughSyncCommitteeParticipants
		);

		let verifier = get_step_verifier::<T>()?;

		let success = zk_light_client_step(&update, sc_poseidon, verifier)
			.map_err(|e| Error::<T>::from(e))?;

		ensure!(success, Error::<T>::ProofNotValid);
		Ok(update.participation > state.finality_threshold)
	}

	fn get_step_verifier<T: Config>() -> Result<Verifier, Error<T>> {
		let vk = VerificationKeyStorage::<T>::get();
		ensure!(!vk.is_empty(), Error::<T>::VerificationKeyIsNotSet);
		let deserialized_vk = Verifier::from_json_u8_slice(vk.as_slice())
			.map_err(|_| Error::<T>::MalformedVerificationKey)?;
		Ok(deserialized_vk)
	}

	fn get_rotate_verifier<T: Config>() -> Result<Verifier, Error<T>> {
		let vk = RotateVerificationKeyStorage::<T>::get();
		ensure!(!vk.is_empty(), Error::<T>::VerificationKeyIsNotSet);
		let deserialized_vk = Verifier::from_json_u8_slice(vk.as_slice())
			.map_err(|_| Error::<T>::MalformedVerificationKey)?;
		Ok(deserialized_vk)
	}

	fn store_step_verification_key<T: Config>(vec_vk: Vec<u8>) -> Result<Verifier, Error<T>> {
		let vk: VerificationKeyDef<T> = vec_vk
			.try_into()
			.map_err(|_| Error::<T>::TooLongVerificationKey)?;
		let deserialized_vk = Verifier::from_json_u8_slice(vk.as_slice())
			.map_err(|_| Error::<T>::MalformedVerificationKey)?;
		ensure!(
			deserialized_vk.vk_json.curve == "bn128".to_string(),
			Error::<T>::NotSupportedCurve
		);
		ensure!(
			deserialized_vk.vk_json.protocol == "groth16".to_string(),
			Error::<T>::NotSupportedProtocol
		);

		VerificationKeyStorage::<T>::put(vk);
		Ok(deserialized_vk)
	}

	fn store_rotate_verification_key<T: Config>(vec_vk: Vec<u8>) -> Result<Verifier, Error<T>> {
		let vk: VerificationKeyDef<T> = vec_vk
			.try_into()
			.map_err(|_| Error::<T>::TooLongVerificationKey)?;
		let deserialized_vk = Verifier::from_json_u8_slice(vk.as_slice())
			.map_err(|_| Error::<T>::MalformedVerificationKey)?;
		ensure!(
			deserialized_vk.vk_json.curve == "bn128".to_string(),
			Error::<T>::NotSupportedCurve
		);
		ensure!(
			deserialized_vk.vk_json.protocol == "groth16".to_string(),
			Error::<T>::NotSupportedProtocol
		);

		RotateVerificationKeyStorage::<T>::put(vk);
		Ok(deserialized_vk)
	}
}
