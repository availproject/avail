#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod common;
mod deserialization;
mod messages;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
mod verify;
mod weights;

use frame_support::{
	dispatch::{GetDispatchInfo, UnfilteredDispatchable},
	pallet_prelude::*,
	parameter_types,
};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use sp_std::prelude::*;
pub use weights::WeightInfo;

type PublicInputsDef<T> = BoundedVec<u8, <T as Config>::MaxPublicInputsLength>;
type ProofDef<T> = BoundedVec<u8, <T as Config>::MaxProofLength>;
type VerificationKeyDef<T> = BoundedVec<u8, <T as Config>::MaxVerificationKeyLength>;

// TODO do we need to store constants in the storage?
parameter_types! {
	pub const  MinSyncCommitteeParticipants: u16=10;
	pub const  SyncCommitteeSize: u32=512;
	pub const  FinalizedRootIndex: u32=105;
	pub const  NextSyncCommitteeIndex: u32= 55;
	pub const  ExecutionStateRootIndex: u32= 402;
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::traits::{Hash, UnixTime};
	use frame_support::{pallet_prelude::ValueQuery, DefaultNoBound};
	use sp_core::H256;

	use crate::messages::{LightClientStep, SuccinctConfig};
	use crate::{
		common::prepare_verification_key,
		deserialization::{deserialize_public_inputs, Proof, VKey},
		verify::{
			prepare_public_inputs, verify, G1UncompressedBytes, G2UncompressedBytes, GProof,
			VerificationKey, SUPPORTED_CURVE, SUPPORTED_PROTOCOL,
		},
	};

	#[pallet::error]
	pub enum Error<T> {
		UpdaterMisMatch,
		CannotChangeUpdater,
		UpdateSlotIsFarInTheFuture,
		UpdateSlotLessThanCurrentHead,
		NotEnoughParticipants,
		VerificationError,
		SyncCommitteeNotInitialized,
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
		SyncCommitteeUpdate { period: u64, root: Hash },
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
	pub type ProofStorage<T: Config> = StorageValue<_, ProofDef<T>, ValueQuery>;
	#[pallet::storage]
	pub type VerificationKeyStorage<T: Config> = StorageValue<_, VerificationKeyDef<T>, ValueQuery>;

	#[pallet::storage]
	pub type SuccinctCfg<T: Config> = StorageValue<_, SuccinctConfig, ValueQuery>;

	#[pallet::storage]
	pub type Consistent<T> = StorageValue<_, bool, ValueQuery>;

	//The latest slot the light client has a finalized header for.
	#[pallet::storage]
	#[pallet::getter(fn get_head)]
	pub type Head<T> = StorageValue<_, u64, ValueQuery>;

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
	pub type SyncCommitteePoseidons<T> = StorageMap<_, Identity, u64, Hash, ValueQuery>;

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
		pub _phantom: PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			// TODO time cannot be called at Genesis
			// T::TimeProvider::now().as_secs()
			<SuccinctCfg<T>>::put(SuccinctConfig {
				updater: self.updater,
				genesis_validators_root: H256([0u8; 32]),
				genesis_time: 1695897840,
				seconds_per_slot: 12,
				slots_per_period: 8192,
				source_chain_id: 1,
				finality_threshold: 461,
			});

			<Consistent<T>>::put(true);
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		[u8; 32]: From<T::AccountId>,
		T::AccountId: From<[u8; 32]>,
	{
		/// Sets the sync committee for the next sync committee period.
		/// A commitment to the the next sync committee is signed by the current sync committee.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::rotate())]
		pub fn rotate(origin: OriginFor<T>, update: messages::LightClientRotate) -> DispatchResult {
			let sender: [u8; 32] = ensure_signed(origin)?.into();
			let config = SuccinctCfg::<T>::get();
			ensure!(H256(sender) == config.updater, Error::<T>::UpdaterMisMatch);

			let light_client_step = update.step.clone();

			let finalized = process_step::<T>(light_client_step.clone())?;
			let current_period = get_sync_committee_period::<T>(light_client_step.finalized_slot);
			let next_period = current_period + 1;

			let vk = get_verification_key::<T>()?;
			let proof = get_proof::<T>(update.proof)?;
			let inputs = get_public_inputs::<T>()?;

			// proof verification
			let success = verify(vk, proof, prepare_public_inputs(inputs))
				.map_err(|_| Error::<T>::VerificationError)?;

			if success {
				Self::deposit_event(Event::VerificationSuccess { who: sender.into() });
				if finalized {
					let is_set = set_sync_committee_poseidon::<T>(
						next_period,
						update.sync_committee_poseidon,
					);
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
			let config = SuccinctCfg::<T>::get();
			ensure!(H256(sender) == config.updater, Error::<T>::UpdaterMisMatch);

			let finalized = process_step::<T>(update.clone())?;

			let current_slot = get_current_slot::<T>(config.genesis_time, config.slots_per_period);

			// ensure!(current_slot > update.attested_slot,  Error::<T>::UpdateSlotIsFarInTheFuture);
			if current_slot < update.attested_slot {
				return Err(Error::<T>::UpdateSlotIsFarInTheFuture.into());
			}

			let head = Head::<T>::get();
			// ensure!(update.finalized_slot > head,  Error::<T>::UpdateSlotLessThanCurrentHead);
			if update.finalized_slot < head {
				return Err(Error::<T>::UpdateSlotLessThanCurrentHead.into());
			}

			ensure!(finalized, Error::<T>::NotEnoughParticipants);

			let updated = set_slot_roots::<T>(
				update.finalized_slot,
				update.finalized_header_root,
				update.execution_state_root,
			);
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
			let old = SuccinctCfg::<T>::get();
			SuccinctCfg::<T>::try_mutate(|cfg| -> Result<(), DispatchError> {
				cfg.updater = updater;
				Ok(())
			})?;

			Self::deposit_event(Event::<T>::NewUpdater {
				old: old.updater,
				new: updater,
			});
			Ok(())
		}

		// TODO this is a POC of simple verification that gets public inputs and tries to verify the proof
		// this uses bls12_381
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::step())]
		pub fn verify(origin: OriginFor<T>, vec_proof: Vec<u8>) -> DispatchResult {
			let sender: [u8; 32] = ensure_signed(origin)?.into();

			let proof = store_proof::<T>(vec_proof)?;
			let vk = get_verification_key::<T>()?;
			let inputs = get_public_inputs::<T>()?;

			Self::deposit_event(Event::<T>::VerificationProofSet);

			match verify(vk, proof, prepare_public_inputs(inputs)) {
				Ok(true) => {
					Self::deposit_event(Event::<T>::VerificationSuccess { who: sender.into() });
					Ok(())
				},
				Ok(false) => {
					Self::deposit_event(Event::<T>::VerificationFailed);
					Ok(())
				},
				Err(_) => Err(Error::<T>::ProofVerificationError.into()),
			}
		}

		/// Sets the public input params into storage
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::step())]
		pub fn setup_verification(
			origin: OriginFor<T>,
			pub_input: Vec<u8>,
			vec_vk: Vec<u8>,
		) -> DispatchResult {
			ensure_root(origin)?;
			let inputs = store_public_inputs::<T>(pub_input)?;
			let vk = store_verification_key::<T>(vec_vk)?;
			ensure!(
				vk.public_inputs_len == inputs.len() as u8,
				Error::<T>::PublicInputsMismatch
			);
			Self::deposit_event(Event::<T>::VerificationSetupCompleted);
			Ok(())
		}
	}

	fn get_current_slot<T: Config>(genesis_time: u64, slots_per_period: u64) -> u64 {
		//  TODO check block time provider
		let block_time: u64 = T::TimeProvider::now().as_secs();

		log::info!(
			"Genesis time: {}, block time: {}, slot per period: {}",
			genesis_time,
			block_time,
			slots_per_period
		);

		(block_time - genesis_time) / slots_per_period
	}

	fn set_slot_roots<T: Config>(
		slot: u64,
		finalized_header_root: H256,
		execution_state_root: H256,
	) -> bool {
		let header = Headers::<T>::get(slot);

		if header != H256::zero() && header != finalized_header_root {
			Consistent::<T>::put(false);
			return false;
		}
		let state_root = ExecutionStateRoots::<T>::get(slot);

		if state_root != H256::zero() && state_root != execution_state_root {
			Consistent::<T>::put(false);
			return false;
		}

		Head::<T>::put(slot);
		Headers::<T>::insert(slot, finalized_header_root);
		// TODO can this time be used as block time?
		Timestamps::<T>::insert(slot, T::TimeProvider::now().as_secs());

		true
	}

	fn set_sync_committee_poseidon<T: Config>(period: u64, poseidon: H256) -> bool {
		let sync_committee_poseidons = SyncCommitteePoseidons::<T>::get(period);

		log::info!(
			"Check: poseidon != H256::zero() {}, poseidon {} ",
			poseidon != H256::zero(),
			poseidon
		);

		if poseidon != H256::zero() && sync_committee_poseidons != poseidon {
			Consistent::<T>::put(false);
			return false;
		}
		SyncCommitteePoseidons::<T>::set(period, poseidon);

		true
	}

	fn get_sync_committee_period<T: Config>(slot: u64) -> u64 {
		// TODO this read can be avoided
		let cfg = SuccinctCfg::<T>::get();
		slot / cfg.slots_per_period
	}

	fn process_step<T: Config>(update: LightClientStep) -> Result<bool, Error<T>> {
		let current_period = get_sync_committee_period::<T>(update.attested_slot);

		let sc_poseidons = SyncCommitteePoseidons::<T>::get(current_period);

		let cfg = SuccinctCfg::<T>::get();

		ensure!(
			sc_poseidons == H256::zero(),
			Error::<T>::SyncCommitteeNotInitialized
		);
		ensure!(
			update.participation < MinSyncCommitteeParticipants::get(),
			Error::<T>::SyncCommitteeNotInitialized
		);

		Ok(update.participation > cfg.finality_threshold)
	}

	fn get_public_inputs<T: Config>() -> Result<Vec<u64>, DispatchError> {
		let public_inputs = PublicInputStorage::<T>::get();
		let deserialized_public_inputs = deserialize_public_inputs(public_inputs.as_slice())
			.map_err(|_| Error::<T>::MalformedPublicInputs)?;
		Ok(deserialized_public_inputs)
	}

	fn store_public_inputs<T: Config>(pub_input: Vec<u8>) -> Result<Vec<u64>, DispatchError> {
		let public_inputs: PublicInputsDef<T> = pub_input
			.try_into()
			.map_err(|_| Error::<T>::TooLongPublicInputs)?;
		let deserialized_public_inputs = deserialize_public_inputs(public_inputs.as_slice())
			.map_err(|_| Error::<T>::MalformedPublicInputs)?;
		PublicInputStorage::<T>::put(public_inputs);
		Ok(deserialized_public_inputs)
	}

	fn get_verification_key<T: Config>() -> Result<VerificationKey, Error<T>> {
		let vk = VerificationKeyStorage::<T>::get();

		ensure!(!vk.is_empty(), Error::<T>::VerificationKeyIsNotSet);
		let deserialized_vk = VKey::from_json_u8_slice(vk.as_slice())
			.map_err(|_| Error::<T>::MalformedVerificationKey)?;
		let vk = prepare_verification_key(deserialized_vk)
			.map_err(|_| Error::<T>::VerificationKeyCreationError)?;
		Ok(vk)
	}

	fn store_verification_key<T: Config>(vec_vk: Vec<u8>) -> Result<VKey, Error<T>> {
		let vk: VerificationKeyDef<T> = vec_vk
			.try_into()
			.map_err(|_| Error::<T>::TooLongVerificationKey)?;
		let deserialized_vk = VKey::from_json_u8_slice(vk.as_slice())
			.map_err(|_| Error::<T>::MalformedVerificationKey)?;
		ensure!(
			deserialized_vk.curve == SUPPORTED_CURVE.as_bytes(),
			Error::<T>::NotSupportedCurve
		);
		ensure!(
			deserialized_vk.protocol == SUPPORTED_PROTOCOL.as_bytes(),
			Error::<T>::NotSupportedProtocol
		);

		VerificationKeyStorage::<T>::put(vk);
		Ok(deserialized_vk)
	}

	fn get_proof<T: Config>(vec_proof: Vec<u8>) -> Result<GProof, Error<T>> {
		ensure!(!vec_proof.is_empty(), Error::<T>::ProofIsEmpty);
		let proof: ProofDef<T> = vec_proof.try_into().map_err(|_| Error::<T>::TooLongProof)?;
		let deserialized_proof =
			Proof::from_json_u8_slice(proof.as_slice()).map_err(|_| Error::<T>::MalformedProof)?;
		ensure!(
			deserialized_proof.curve == SUPPORTED_CURVE.as_bytes(),
			Error::<T>::NotSupportedCurve
		);
		ensure!(
			deserialized_proof.protocol == SUPPORTED_PROTOCOL.as_bytes(),
			Error::<T>::NotSupportedProtocol
		);

		let proof = GProof::from_uncompressed(
			&G1UncompressedBytes::new(deserialized_proof.a[0], deserialized_proof.a[1]),
			&G2UncompressedBytes::new(
				deserialized_proof.b[0][0],
				deserialized_proof.b[0][1],
				deserialized_proof.b[1][0],
				deserialized_proof.b[1][1],
			),
			&G1UncompressedBytes::new(deserialized_proof.c[0], deserialized_proof.c[1]),
		)
		.map_err(|_| Error::<T>::ProofCreationError)?;

		Ok(proof)
	}

	fn store_proof<T: Config>(vec_proof: Vec<u8>) -> Result<GProof, sp_runtime::DispatchError> {
		ensure!(!vec_proof.is_empty(), Error::<T>::ProofIsEmpty);
		let proof: ProofDef<T> = vec_proof.try_into().map_err(|_| Error::<T>::TooLongProof)?;
		let deserialized_proof =
			Proof::from_json_u8_slice(proof.as_slice()).map_err(|_| Error::<T>::MalformedProof)?;
		ensure!(
			deserialized_proof.curve == SUPPORTED_CURVE.as_bytes(),
			Error::<T>::NotSupportedCurve
		);
		ensure!(
			deserialized_proof.protocol == SUPPORTED_PROTOCOL.as_bytes(),
			Error::<T>::NotSupportedProtocol
		);

		ProofStorage::<T>::put(proof);

		let proof = GProof::from_uncompressed(
			&G1UncompressedBytes::new(deserialized_proof.a[0], deserialized_proof.a[1]),
			&G2UncompressedBytes::new(
				deserialized_proof.b[0][0],
				deserialized_proof.b[0][1],
				deserialized_proof.b[1][0],
				deserialized_proof.b[1][1],
			),
			&G1UncompressedBytes::new(deserialized_proof.c[0], deserialized_proof.c[1]),
		)
		.map_err(|_| Error::<T>::ProofCreationError)?;

		Ok(proof)
	}
}
