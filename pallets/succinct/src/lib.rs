#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{pallet_prelude::*, parameter_types};
use sp_core::U256;

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
mod target_amb;
mod verifier;
mod weights;

type VerificationKeyDef<T> = BoundedVec<u8, <T as Config>::MaxVerificationKeyLength>;

// TODO remove unused and define correct values
parameter_types! {
	pub const MinSyncCommitteeParticipants: u16=10;
	pub const SyncCommitteeSize: u32=512;
	pub const FinalizedRootIndex: u32=105;
	pub const NextSyncCommitteeIndex: u32= 55;
	pub const ExecutionStateRootIndex: u32= 402;
	pub const MaxPublicInputsLength: u32 = 9;
	pub const MaxVerificationKeyLength: u32 = 4143;
	pub const MaxProofLength: u32 = 1133;

	pub const MessageVersion: u8 = 1;
	pub const MinLightClientDelay: u64 = 120;
	pub const MessageMappingStorageIndex:u64 = 1;
}

#[frame_support::pallet]
pub mod pallet {
	use ark_std::string::String;
	use ark_std::string::ToString;
	use ark_std::{vec, vec::Vec};
	use codec::KeyedVec;
	use ethabi::Token::Uint;
	use frame_support::dispatch::{GetDispatchInfo, UnfilteredDispatchable};
	use frame_support::sp_core_hashing_proc_macro::keccak_256;
	use frame_support::traits::{Hash, Len, UnixTime};
	use frame_support::{pallet_prelude::ValueQuery, DefaultNoBound};
	use patricia_merkle_trie::{EIP1186Layout, StorageProof};
	use primitive_types::H160;
	use primitive_types::{H256, U256};
	use rlp::{Decodable, Rlp};
	use sp_io::hashing::keccak_256;
	use sp_runtime::TokenError::Frozen;
	use trie_db::{DBValue, Trie, TrieDBBuilder};

	use frame_system::pallet_prelude::*;
	pub use weights::WeightInfo;

	use crate::state::{LightClientStep, State};
	use crate::target_amb::{decode_message, Message};
	use crate::verifier::zk_light_client_rotate;
	use crate::verifier::zk_light_client_step;

	use super::*;

	#[pallet::error]
	pub enum Error<T> {
		UpdaterMisMatch,
		VerificationError,
		CannotUpdateStateStorage,
		UpdateSlotIsFarInTheFuture,
		UpdateSlotLessThanCurrentHead,
		NotEnoughParticipants,
		SyncCommitteeNotInitialized,
		NotEnoughSyncCommitteeParticipants,
		// verification
		TooLongVerificationKey,
		ProofIsEmpty,
		VerificationKeyIsNotSet,
		MalformedVerificationKey,
		NotSupportedCurve,
		NotSupportedProtocol,
		ProofCreationError,
		InvalidRotateProof,
		InvalidStepProof,
		//     Message execution
		MessageAlreadyExecuted,
		WrongChain,
		WrongVersion,
		BroadcasterSourceChainNotSet,
		LightClientInconsistent,
		LightClientNotSet,
		SourceChainFrozen,
		TimestampNotSet,
		MustWaitLongerForSlot,
		CannotDecodeRlpItems,
		AccountNotFound,
		CannotGetStorageRoot,
		TrieError,
		StorageValueNotFount,
		StorageRootNotFount,
		InvalidMessageHash,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		// emit event once the head is updateds
		HeadUpdate {
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

	#[derive(
		Clone, Copy, Default, Encode, Decode, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen,
	)]
	pub enum MessageStatusEnum {
		#[default]
		NotExecuted,
		ExecutionFailed,
		ExecutionSucceeded,
	}

	// Storage definitions

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

	// Maps from a slot to a block header root.
	#[pallet::storage]
	#[pallet::getter(fn get_header)]
	pub type Headers<T> = StorageMap<_, Identity, u64, H256, ValueQuery>;

	// Maps slot to the timestamp of when the headers mapping was updated with slot as a key
	#[pallet::storage]
	#[pallet::getter(fn get_timestamp)]
	pub type Timestamps<T> = StorageMap<_, Identity, u64, u64, ValueQuery>;

	// Maps from a slot to the current finalized ethereum execution state root.
	#[pallet::storage]
	#[pallet::getter(fn get_state_root)]
	pub type ExecutionStateRoots<T> = StorageMap<_, Identity, u64, H256, ValueQuery>;

	// Maps from a period to the poseidon commitment for the sync committee.
	#[pallet::storage]
	#[pallet::getter(fn get_poseidon)]
	pub type SyncCommitteePoseidons<T> = StorageMap<_, Identity, u64, U256, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_message_status)]
	pub type MessageStatus<T> = StorageMap<_, Identity, H256, MessageStatusEnum, ValueQuery>;

	// Mapping between source chainId and the address of the Telepathy broadcaster on that chain.
	#[pallet::storage]
	#[pallet::getter(fn get_broadcaster)]
	pub type Broadcasters<T> = StorageMap<_, Identity, u32, H160, ValueQuery>;

	// Mapping between source chainId and the corresponding light client.
	#[pallet::storage]
	#[pallet::getter(fn get_light_client)]
	pub type LightClients<T> = StorageMap<_, Identity, u32, H160, ValueQuery>;

	// Ability to froze source, must support possibility to update value
	#[pallet::storage]
	#[pallet::getter(fn is_frozen)]
	pub type SourceChainFrozen<T> = StorageMap<_, Identity, H256, bool, ValueQuery>;

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
		type MessageVersion: Get<u8>;

		#[pallet::constant]
		type MinLightClientDelay: Get<u64>;

		#[pallet::constant]
		type MessageMappingStorageIndex: Get<u64>;

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
			let current_period = step.finalized_slot / state.slots_per_period;
			let next_period = current_period + 1;

			let verifier = get_rotate_verifier::<T>()?;

			// proof verification
			let success = zk_light_client_rotate(&update, verifier)
				.map_err(|_| Error::<T>::VerificationError)?;

			ensure!(success, Error::<T>::InvalidRotateProof);

			Self::deposit_event(Event::VerificationSuccess {
				who: sender.into(),
				attested_slot: step.attested_slot,
				finalized_slot: step.finalized_slot,
			});
			if finalized {
				let is_set =
					set_sync_committee_poseidon::<T>(next_period, update.sync_committee_poseidon)?;
				if is_set {
					Self::deposit_event(Event::SyncCommitteeUpdate {
						period: next_period,
						root: update.sync_committee_poseidon,
					});
				}
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

			let block_time: u64 = T::TimeProvider::now().as_secs();
			let current_slot = (block_time - state.genesis_time) / state.seconds_per_slot;

			ensure!(
				current_slot >= update.attested_slot,
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

		/// Sets updater that can call step and rotate functions
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

		/// Sets verification public inputs for step function.
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

		/// Sets verification public inputs for rotate function.
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

		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::step())]
		pub fn execute(
			origin: OriginFor<T>,
			slot: u64,
			message_bytes: Vec<u8>,
			account_proof: Vec<Vec<u8>>,
			storage_proof: Vec<Vec<u8>>,
		) -> DispatchResult {
			let message = decode_message(message_bytes);
			let message_root = H256(keccak_256!(message_bytes));
			check_preconditions::<T>(&message, message_root)?;

			let state = StateStorage::<T>::get();
			ensure!(state.consistent, Error::<T>::LightClientInconsistent);

			ensure!(
				SourceChainFrozen::<T>::get(message.source_address) == false,
				Error::<T>::SourceChainFrozen
			);
			// TODO require delay, why?
			require_lc_delay::<T>(slot, message.source_chain_id)?;

			let storage_root = get_storage_root::<T>(slot, message.source_chain_id, account_proof)?;

			let nonce = Uint(U256::from(message.nonce));
			let mm_idx = Uint(U256::from(MessageMappingStorageIndex::get()));
			let slot_key = keccak_256(ethabi::encode(&[nonce, mm_idx]).as_slice());

			let slot_value = get_storage_value::<T>(H256(slot_key), storage_root, storage_proof)?;

			ensure!(slot_value == message_root, Error::<T>::InvalidMessageHash);

			// TODO message is valid can be executed

			Ok(())
		}
	}

	pub fn get_storage_value<T: Config>(
		slot_hash: H256,
		storage_root: H256,
		proof: Vec<Vec<u8>>,
	) -> Result<H256, DispatchError> {
		let db = StorageProof::new(proof).into_memory_db::<target_amb::keccak256::KeccakHasher>();
		let trie = TrieDBBuilder::<EIP1186Layout<target_amb::keccak256::KeccakHasher>>::new(
			&db,
			&storage_root,
		)
		.build();

		if let Some(storage_root) = trie
			.get(&slot_hash.as_bytes())
			.map_err(|_| Error::<T>::TrieError)?
		{
			let r = Rlp::new(storage_root.as_slice());
			ensure!(r.data().len() > 0, Error::<T>::CannotDecodeRlpItems);
			let storage_value = r.data().map_err(|_| Error::<T>::CannotDecodeRlpItems)?;
			Ok(H256::from_slice(storage_value))
		} else {
			Err(Error::<T>::StorageValueNotFount.into())
		}
	}

	pub fn get_storage_root<T: Config>(
		slot: u64,
		source_chain_id: u32,
		proof: Vec<Vec<u8>>,
	) -> Result<H256, DispatchError> {
		let address = Broadcasters::<T>::get(source_chain_id);
		let state_root = ExecutionStateRoots::<T>::get(slot);

		let key = keccak_256(address.as_bytes());
		let db = StorageProof::new(proof).into_memory_db::<target_amb::keccak256::KeccakHasher>();
		let trie = TrieDBBuilder::<EIP1186Layout<target_amb::keccak256::KeccakHasher>>::new(
			&db,
			&state_root,
		)
		.build();

		let result: DBValue = trie.get(&key.as_slice()).unwrap().unwrap();
		let byte_slice = result.as_slice();
		let r = Rlp::new(byte_slice);

		let item_count = r
			.item_count()
			.map_err(|_| Error::<T>::CannotDecodeRlpItems)?;

		ensure!(item_count == 4, Error::<T>::AccountNotFound);

		let item = r
			.at(2)
			.map_err(|_| Error::<T>::CannotDecodeRlpItems)?
			.data()
			.map_err(|_| Error::<T>::CannotDecodeRlpItems)?;

		let storage_root = H256::from_slice(item);

		Ok(storage_root)
	}

	pub fn require_lc_delay<T: Config>(slot: u64, chain_id: u32) -> Result<(), DispatchError> {
		ensure!(
			LightClients::<T>::get(chain_id) != H160::zero(),
			Error::<T>::LightClientNotSet
		);
		let ts = Timestamps::<T>::get(slot);
		ensure!(ts != 0, Error::<T>::TimestampNotSet);
		let elapsed_time = T::TimeProvider::now().as_secs() - ts;

		ensure!(
			elapsed_time >= MinLightClientDelay::get(),
			Error::<T>::MustWaitLongerForSlot
		);

		Ok(())
	}

	pub fn check_preconditions<T: Config>(
		message: &Message,
		message_root: H256,
	) -> Result<(), DispatchError> {
		let message_status = MessageStatus::<T>::get(message_root);
		// Message must not be executed
		ensure!(
			message_status == MessageStatusEnum::NotExecuted,
			Error::<T>::MessageAlreadyExecuted
		);

		// TODO check chainID?
		let source_chain_id: u32 = 1001;
		// Version must match for storage
		ensure!(
			message.version == MessageVersion::get(),
			Error::<T>::WrongVersion
		);
		// TODO check chainID?
		// only H160 address
		let source_chain = Broadcasters::<T>::get(source_chain_id);
		ensure!(
			source_chain != H160::zero(),
			Error::<T>::BroadcasterSourceChainNotSet
		);

		Ok(())
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

	fn process_step<T: Config>(
		state: State,
		update: &LightClientStep,
	) -> Result<bool, DispatchError> {
		let current_period = update.finalized_slot / state.slots_per_period; //get_sync_committee_period(state, update.attested_slot);
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
			.map_err(|_| Error::<T>::VerificationError)?;

		ensure!(success, Error::<T>::InvalidStepProof);
		Ok(update.participation > state.finality_threshold)
	}

	fn get_step_verifier<T: Config>() -> Result<Verifier, Error<T>> {
		let vk = StepVerificationKeyStorage::<T>::get();
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

		StepVerificationKeyStorage::<T>::put(vk);
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
