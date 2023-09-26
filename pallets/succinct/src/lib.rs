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

use frame_support::error::BadOrigin;
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
	pub const  MinSyncCommitteeParticipants: u32=10;
	pub const  SyncCommitteeSize: u32=512;
	pub const  FinalizedRootIndex: u32=105;
	pub const  NextSyncCommitteeIndex: u32= 55;
	pub const  ExecutionStateRootIndex: u32= 402;
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::traits::Hash;
	use frame_support::{
		pallet_prelude::{ValueQuery, *},
		transactional, DefaultNoBound,
	};
	use serde::Serialize;
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

	#[pallet::storage]
	pub type PublicInputStorage<T: Config> = StorageValue<_, PublicInputsDef<T>, ValueQuery>;
	#[pallet::storage]
	pub type ProofStorage<T: Config> = StorageValue<_, ProofDef<T>, ValueQuery>;
	#[pallet::storage]
	pub type VerificationKeyStorage<T: Config> = StorageValue<_, VerificationKeyDef<T>, ValueQuery>;

	#[pallet::storage]
	pub type SuccinctCfg<T: Config> = StorageValue<_, SuccinctConfig, ValueQuery>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type MaxPublicInputsLength: Get<u32>;
		#[pallet::constant]
		type MaxProofLength: Get<u32>;
		#[pallet::constant]
		type MaxVerificationKeyLength: Get<u32>;

		// =========== Constants ============

		type MinSyncCommitteeParticipants: Get<u32>;
		type SyncCommitteeSize: Get<u32>;
		type FinalizedRootIndex: Get<u32>;
		type NextSyncCommitteeIndex: Get<u32>;
		type ExecutionStateRootIndex: Get<u32>;

		/// A sudo-able call.
		type RuntimeCall: Parameter
			+ UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ GetDispatchInfo;

		type WeightInfo: WeightInfo;

		type ApprovedOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	// ====================== Storage =============================

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
	pub type Timestamps<T> = StorageMap<_, Identity, u64, Hash, ValueQuery>;

	// Maps from a slot to the current finalized ethereum1 execution state root.
	#[pallet::storage]
	#[pallet::getter(fn get_state_root)]
	pub type ExecutionStateRoots<T> = StorageMap<_, Identity, u64, Hash, ValueQuery>;

	// Maps from a period to the poseidon commitment for the sync committee.
	#[pallet::storage]
	#[pallet::getter(fn get_poseidon)]
	pub type SyncCommitteePoseidons<T> = StorageMap<_, Identity, u64, Hash, ValueQuery>;

	// #[pallet::storage]
	// pub type Updater<T: Config> = StorageValue<_, <T as frame_system::Config>::AccountId, OptionQuery>;

	// ======================================================================

	//  ====== Genesis config ==========
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
			<SuccinctCfg<T>>::put(SuccinctConfig {
				updater: self.updater.into(),
				genesis_validators_root: self.genesis_validators_root,
				genesis_time: self.genesis_time,
				seconds_per_slot: self.seconds_per_slot,
				slots_per_period: self.slots_per_period,
				source_chain_id: self.source_chain_id,
				finality_threshold: self.finality_threshold,
			});
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		[u8; 32]: From<T::AccountId>,
		H256: From<T::Hash>,
	{
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::rotate())]
		pub fn rotate(origin: OriginFor<T>, update: messages::LightClientRotate) -> DispatchResult {
			let sender: [u8; 32] = ensure_signed(origin)?.into();
			let config = SuccinctCfg::<T>::get();
			ensure_valid_sender::<T>(sender, config.updater)?;

			log::info!("Update: {:?}", update);

			log::info!("Succinct configuration: {:?}", config);

			// TODO dummy period
			let current_period: u64 = update.step.finalized_slot / 10;
			let next_period: u64 = current_period + 1;

			Self::deposit_event(Event::SyncCommitteeUpdate {
				period: next_period,
				root: update.sync_committee_poseidon,
			});

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::step())]
		pub fn step(origin: OriginFor<T>, update: LightClientStep) -> DispatchResult {
			let sender: [u8; 32] = ensure_signed(origin)?.into();
			let config = SuccinctCfg::<T>::get();
			ensure_valid_sender::<T>(sender, config.updater)?;

			log::info!("Update: {:?}", update);

			Self::deposit_event(Event::HeadUpdate {
				slot: update.finalized_slot,
				finalization_root: update.finalized_header_root,
			});

			Ok(())
		}

		// POC verification zk-SNARK
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::step())]
		pub fn verify(origin: OriginFor<T>, vec_proof: Vec<u8>) -> DispatchResult {
			let proof = store_proof::<T>(vec_proof)?;
			let vk = get_verification_key::<T>()?;
			let inputs = get_public_inputs::<T>()?;
			let sender = ensure_signed(origin)?;
			Self::deposit_event(Event::<T>::VerificationProofSet);

			match verify(vk, proof, prepare_public_inputs(inputs)) {
				Ok(true) => {
					Self::deposit_event(Event::<T>::VerificationSuccess { who: sender });
					Ok(())
				},
				Ok(false) => {
					Self::deposit_event(Event::<T>::VerificationFailed);
					Ok(())
				},
				Err(_) => Err(Error::<T>::ProofVerificationError.into()),
			}
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::step())]
		pub fn setup_verification(
			_origin: OriginFor<T>,
			pub_input: Vec<u8>,
			vec_vk: Vec<u8>,
		) -> DispatchResult {
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

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		// emit event once the head is updated
		HeadUpdate {
			slot: u64,
			finalization_root: Hash,
		},
		SyncCommitteeUpdate {
			period: u64,
			root: Hash,
		},

		/// A root operation was executed, show result
		VerificationSetupCompleted,
		VerificationProofSet,
		VerificationSuccess {
			who: T::AccountId,
		},
		NewUpdater {
			old: <T as frame_system::Config>::AccountId,
			new: <T as frame_system::Config>::AccountId,
		},
		VerificationFailed,
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

	fn get_verification_key<T: Config>() -> Result<VerificationKey, DispatchError> {
		let vk = VerificationKeyStorage::<T>::get();

		ensure!(!vk.is_empty(), Error::<T>::VerificationKeyIsNotSet);
		let deserialized_vk = VKey::from_json_u8_slice(vk.as_slice())
			.map_err(|_| Error::<T>::MalformedVerificationKey)?;
		let vk = prepare_verification_key(deserialized_vk)
			.map_err(|_| Error::<T>::VerificationKeyCreationError)?;
		Ok(vk)
	}

	fn store_verification_key<T: Config>(vec_vk: Vec<u8>) -> Result<VKey, DispatchError> {
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

	pub fn ensure_valid_sender<T: Config>(
		sender: [u8; 32],
		updater: H256,
	) -> Result<(), DispatchError> {
		if H256(sender) != updater {
			return Err(Error::<T>::UpdaterMisMatch.into());
		}
		Ok(())
	}
}
