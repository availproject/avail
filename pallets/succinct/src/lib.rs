#![cfg_attr(not(feature = "std"), no_std)]

use crate::target_amb::MessageStatusEnum;
use crate::verifier::Verifier;
use frame_support::traits::{Currency, ExistenceRequirement, UnixTime};
use frame_support::{pallet_prelude::*, parameter_types, PalletId};
use frame_system::submitted_data::{BoundedData, MessageType};
use hex_literal::hex;
pub use pallet::*;
use sp_core::H256;
use sp_runtime::SaturatedConversion;

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
pub type BridgeData<T> = BoundedVec<u8, <T as Config>::MaxBridgeDataLength>;

// whitelist of supported domains
// TODO: Create a storage & extrinsic around it to support onchain updation of supported domains, also can act as the panic button
const WHITELISTED_DOMAINS: [u32; 1] = [2];

parameter_types! {
	// function identifiers
	pub const StepFunctionId: H256 = H256(hex!("af44af6890508b3b7f6910d4a4570a0d524769a23ce340b2c7400e140ad168ab"));
	pub const RotateFunctionId: H256 = H256(hex!("9aed23f9e6e8f8b98751cf508069b5b7f015d4d510b6a4820d41ba1ce88190d9"));

	// Max length constants
	pub const MaxVerificationKeyLength: u32 = 4143;
	pub const MaxProofLength: u32 = 1133;

	pub const MessageMappingStorageIndex:u64 = 1;

	// BoundedVec max size for fulfill call.
	pub const InputMaxLen: u32 = 256;
	pub const OutputMaxLen: u32 = 512;
	pub const ProofMaxLen: u32 = 1048;
	// BoundedVec max size for execute call.
	pub const MessageBytesMaxLen: u32 = 2048;
	pub const AccountProofMaxLen: u32 = 2048;
	pub const AccountProofLen: u32 = 2048;
	pub const StorageProofMaxLen: u32 = 2048;
	pub const StorageProofLen: u32 = 2048;
	pub const BridgePalletId: PalletId = PalletId(*b"avl/brdg");

	pub const MaxBridgeDataLength: u32= 256;

	pub const AvailDomain: u32 = 1;
	pub const SupportedDomain:u32 = 2;
	pub const SupportedAssetId:H256 = H256::zero(); // Avail asset is supported for now
}

#[frame_support::pallet]
pub mod pallet {
	use ark_std::string::String;
	use ark_std::{vec, vec::Vec};
	use ethabi::Token;
	use ethabi::Token::Uint;
	use frame_support::dispatch::{GetDispatchInfo, UnfilteredDispatchable};
	use frame_support::traits::{Hash, LockableCurrency};
	use frame_support::{pallet_prelude::ValueQuery, DefaultNoBound};
	use frame_system::pallet_prelude::*;
	use frame_system::submitted_data::Message;
	use primitive_types::H160;
	use primitive_types::{H256, U256};
	use sp_io::hashing::keccak_256;
	use sp_io::hashing::sha2_256;
	use sp_runtime::traits::AccountIdConversion;
	pub use weights::WeightInfo;

	use crate::state::State;
	use crate::state::{
		parse_rotate_output, parse_step_output, VerifiedRotate, VerifiedStep, VerifiedStepOutput,
	};
	use crate::target_amb::{get_storage_root, get_storage_value};
	use crate::verifier::encode_packed;

	use super::*;

	#[pallet::error]
	pub enum Error<T> {
		UpdaterMisMatch,
		VerificationError,
		NotEnoughParticipants,
		SlotBehindHead,
		TooLongVerificationKey,
		VerificationKeyIsNotSet,
		MalformedVerificationKey,
		FunctionIdNotKnown,
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
		SyncCommitteeNotSet,
		//     Message execution
		MessageAlreadyExecuted,
		WrongDestinationChain,
		UnsupportedDestinationChain,
		BroadcasterSourceChainNotSet,
		BroadcasterNotValid,
		SourceChainFrozen,
		CannotGetStorageRoot,
		CannotGetStorageValue,
		InvalidMessageHash,
		CannotDecodeData,
		CannotDecodeDestinationAccountId,
		AssetNotSupported,
		// bridge
		/// Given inputs for the selected MessageType are invalid
		InvalidBridgeInputs,
		/// Domain is not supported
		DomainNotSupported,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		// emit event once the head is updated.
		HeaderUpdate {
			slot: u64,
			finalization_root: H256,
		},
		// emit event once the sync committee updates.
		SyncCommitteeUpdate {
			period: u64,
			root: U256,
		},
		// emit event when verification setup is completed.
		VerificationSetupCompleted,
		// emit when new updater is set
		NewUpdater {
			old: H256,
			new: H256,
		},
		// emit when message gets executed.
		ExecutedMessage {
			chain_id: u32,
			nonce: u64,
			message_root: H256,
			status: bool,
		},
		// emit if source chain gets frozen.
		SourceChainFrozen {
			source_chain_id: u32,
			frozen: bool,
		},
		BridgeDataSubmitted {
			who: T::AccountId,
			data_hash: H256,
		},
		MessageSubmitted {
			from: T::AccountId,
			to: H256,
			message_type: MessageType,
		},
	}

	// Step verification key storage.
	#[pallet::storage]
	pub type StepVerificationKeyStorage<T: Config> =
		StorageValue<_, VerificationKeyDef<T>, ValueQuery>;

	// Rotate verification key storage.
	#[pallet::storage]
	pub type RotateVerificationKeyStorage<T: Config> =
		StorageValue<_, VerificationKeyDef<T>, ValueQuery>;

	// Storage for a general state.
	#[pallet::storage]
	#[pallet::getter(fn head)]
	pub type Head<T: Config> = StorageValue<_, u64, ValueQuery>;

	// Maps from a slot to a block header root.
	#[pallet::storage]
	#[pallet::getter(fn headers)]
	pub type Headers<T> = StorageMap<_, Identity, u64, H256, ValueQuery>;

	// Maps slot to the timestamp of when the headers mapping was updated with slot as a key
	#[pallet::storage]
	pub type Timestamps<T> = StorageMap<_, Identity, u64, u64, ValueQuery>;

	// Maps from a slot to the current finalized ethereum execution state root.
	#[pallet::storage]
	pub type ExecutionStateRoots<T> = StorageMap<_, Identity, u64, H256, ValueQuery>;

	// Maps from a period to the poseidon commitment for the sync committee.
	#[pallet::storage]
	#[pallet::getter(fn sync_committee_poseidons)]
	pub type SyncCommitteePoseidons<T> = StorageMap<_, Identity, u64, U256, ValueQuery>;

	// Storage for a general state.
	#[pallet::storage]
	pub type StateStorage<T: Config> = StorageValue<_, State, ValueQuery>;

	// Maps status of the message to the message root
	#[pallet::storage]
	pub type MessageStatus<T> = StorageMap<_, Identity, H256, MessageStatusEnum, ValueQuery>;

	// Mapping between source chainId and the address of the Telepathy broadcaster on that chain.
	#[pallet::storage]
	pub type Broadcasters<T> = StorageMap<_, Identity, u32, H256, ValueQuery>;

	// Ability to froze source chain
	#[pallet::storage]
	pub type SourceChainFrozen<T> = StorageMap<_, Identity, u32, bool, ValueQuery>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Currency: LockableCurrency<Self::AccountId, Moment = BlockNumberFor<Self>>;

		type TimeProvider: UnixTime;
		#[pallet::constant]
		type MaxProofLength: Get<u32>;
		// 1133
		#[pallet::constant]
		type MaxVerificationKeyLength: Get<u32>;
		#[pallet::constant]
		type MaxBridgeDataLength: Get<u32>;
		#[pallet::constant]
		type StepFunctionId: Get<H256>;

		#[pallet::constant]
		type RotateFunctionId: Get<H256>;

		#[pallet::constant]
		type MessageMappingStorageIndex: Get<u64>;

		/// Bridge's pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		#[pallet::constant]
		type AvailDomain: Get<u32>;
		#[pallet::constant]
		type SupportedDomain: Get<u32>;

		type RuntimeCall: Parameter
			+ UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ GetDispatchInfo;

		type WeightInfo: WeightInfo;
	}

	#[pallet::genesis_config]
	#[derive(DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub updater: Hash,
		pub slots_per_period: u64,
		pub finality_threshold: u16,
		pub sync_committee_poseidon: U256,
		pub period: u64,
		pub _phantom: PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			// Preconfigure init data
			<StateStorage<T>>::put(State {
				updater: self.updater,
				slots_per_period: self.slots_per_period,
				finality_threshold: self.finality_threshold,
			});

			<SyncCommitteePoseidons<T>>::insert(self.period, self.sync_committee_poseidon);

			// TODO TEST
			ExecutionStateRoots::<T>::set(
				8581263,
				H256(hex!(
					"cd187a0c3dddad24f1bb44211849cc55b6d2ff2713be85f727e9ab8c491c621c"
				)),
			);
			// Broadcasters::<T>::set(5, H160(hex!("43f0222552e8114ad8f224dea89976d3bf41659d")));
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		[u8; 32]: From<T::AccountId>,
	{
		/// The entrypoint for fulfilling a call.
		/// function_id Function identifier.
		/// input Function input.
		/// output Function output.
		/// proof  Function proof.
		/// slot  Function slot to update.
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
			// compute hashes
			let input_hash = H256(sha2_256(input.as_slice()));
			let output_hash = H256(sha2_256(output.as_slice()));
			let verifier = Self::get_verifier(function_id)?;

			let is_success = verifier
				.verify(input_hash, output_hash, proof.to_vec())
				.map_err(|_| Error::<T>::VerificationError)?;

			// make sure that verification call is valid
			ensure!(is_success, Error::<T>::VerificationFailed);

			if function_id == StepFunctionId::get() {
				let vs =
					VerifiedStep::new(function_id, input_hash, parse_step_output(output.to_vec()));

				if Self::step_into(slot, state, &vs)? {
					Self::deposit_event(Event::HeaderUpdate {
						slot,
						finalization_root: vs.verified_output.finalized_header_root,
					});
				}
			} else if function_id == RotateFunctionId::get() {
				let vr = VerifiedRotate::new(
					function_id,
					input_hash,
					parse_rotate_output(output.to_vec()),
				);

				if Self::rotate_into(slot, state, &vr)? {
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

		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::step())]
		pub fn execute(
			origin: OriginFor<T>,
			slot: u64,
			message: Message,
			account_proof: BoundedVec<BoundedVec<u8, AccountProofMaxLen>, AccountProofLen>,
			storage_proof: BoundedVec<BoundedVec<u8, StorageProofMaxLen>, StorageProofLen>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			let encoded_data = message.clone().abi_encode();

			let message_root = H256(keccak_256(encoded_data.as_slice()));
			check_preconditions::<T>(&message, message_root)?;

			ensure!(
				!SourceChainFrozen::<T>::get(message.original_domain),
				Error::<T>::SourceChainFrozen
			);
			let root = ExecutionStateRoots::<T>::get(slot);
			let broadcaster = Broadcasters::<T>::get(message.original_domain);
			ensure!(broadcaster == message.from, Error::<T>::BroadcasterNotValid);

			// extract contract address
			let contract_broadcaster_address = H160::from_slice(broadcaster[..20].as_ref());
			let account_proof_vec = account_proof
				.iter()
				.map(|inner_bounded_vec| inner_bounded_vec.iter().copied().collect())
				.collect();

			let storage_root =
				get_storage_root(account_proof_vec, contract_broadcaster_address, root)
					.map_err(|_| Error::<T>::CannotGetStorageRoot)?;

			let nonce = Uint(U256::from(message.id));
			let mm_idx = Uint(U256::from(MessageMappingStorageIndex::get()));
			let slot_key = H256(keccak_256(ethabi::encode(&[nonce, mm_idx]).as_slice()));

			let storage_proof_vec = storage_proof
				.iter()
				.map(|inner_bounded_vec| inner_bounded_vec.iter().copied().collect())
				.collect();

			let slot_value = get_storage_value(slot_key, storage_root, storage_proof_vec)
				.map_err(|_| Error::<T>::CannotGetStorageValue)?;

			ensure!(slot_value == message_root, Error::<T>::InvalidMessageHash);

			let (asset_id, amount) = Self::decode_message_data(message.data.to_vec())?;
			ensure!(
				SupportedAssetId::get() == asset_id,
				Error::<T>::AssetNotSupported
			);

			let success = Self::transfer(amount, message.to)?;

			if success {
				MessageStatus::<T>::set(message_root, MessageStatusEnum::ExecutionSucceeded);
				Self::deposit_event(Event::<T>::ExecutedMessage {
					chain_id: message.original_domain,
					nonce: message.id,
					message_root,
					status: true,
				});
			} else {
				MessageStatus::<T>::set(message_root, MessageStatusEnum::ExecutionFailed);
				Self::deposit_event(Event::<T>::ExecutedMessage {
					chain_id: message.original_domain,
					nonce: message.id,
					message_root,
					status: false,
				});
			}

			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::step())]
		pub fn source_chain_froze(
			origin: OriginFor<T>,
			source_chain_id: u32,
			frozen: bool,
		) -> DispatchResult {
			ensure_root(origin)?;

			SourceChainFrozen::<T>::set(source_chain_id, frozen);
			Self::deposit_event(Event::<T>::SourceChainFrozen {
				source_chain_id,
				frozen,
			});

			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::step())]
		pub fn send_message(
			origin: OriginFor<T>,
			message_type: MessageType,
			to: H256,
			#[pallet::compact] domain: u32,
			value: Option<u128>,
			asset_id: Option<H256>,
			data: Option<BoundedData>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			// Ensure the domain is currently supported
			ensure!(
				Self::is_domain_valid(domain),
				Error::<T>::DomainNotSupported
			);
			// Check MessageType and enforce the rules
			match message_type {
				MessageType::ArbitraryMessage => {
					ensure!(
						value.is_none() && asset_id.is_none() && !data.is_none(),
						Error::<T>::InvalidBridgeInputs
					);
					// What to do?
					Self::deposit_event(Event::MessageSubmitted {
						from: who,
						to,
						message_type,
					});
				},
				MessageType::FungibleToken => {
					ensure!(
						!value.is_none() && !asset_id.is_none() && data.is_none(),
						Error::<T>::InvalidBridgeInputs
					);

					T::Currency::transfer(
						&who,
						&Self::account_id(),
						value.unwrap_or_default().saturated_into(),
						ExistenceRequirement::KeepAlive,
					)?;
					Self::deposit_event(Event::MessageSubmitted {
						from: who,
						to,
						message_type,
					});
				},
			}

			Ok(().into())
		}
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

		ensure!(
			message.destination_domain == AvailDomain::get(),
			Error::<T>::WrongDestinationChain
		);

		ensure!(
			SupportedDomain::get() == message.original_domain,
			Error::<T>::UnsupportedDestinationChain
		);

		let source_chain = Broadcasters::<T>::get(message.original_domain);
		ensure!(
			source_chain != H256::zero(),
			Error::<T>::BroadcasterSourceChainNotSet
		);

		Ok(())
	}

	impl<T: Config> Pallet<T> {
		fn decode_message_data(data: Vec<u8>) -> Result<(H256, U256), DispatchError> {
			//abi.encode(ASSET_ID, msg.value),
			let decoded_data = ethabi::decode(
				&[
					ethabi::ParamType::FixedBytes(32),
					ethabi::ParamType::Uint(256),
				],
				data.as_slice().as_ref(),
			)
			.map_err(|_| Error::<T>::CannotDecodeData)?;
			ensure!(decoded_data.len() == 2, Error::<T>::CannotDecodeData);

			let asset_id_token = decoded_data.get(0).ok_or(Error::<T>::CannotDecodeData)?;
			let asset_id = asset_id_token
				.clone()
				.into_fixed_bytes()
				.ok_or(Error::<T>::CannotDecodeData)?;

			let asset = H256::from_slice(asset_id.as_slice());

			let amount_token = decoded_data.get(1).ok_or(Error::<T>::CannotDecodeData)?;
			let amount = amount_token
				.clone()
				.into_uint()
				.ok_or(Error::<T>::CannotDecodeData)?;

			Ok((asset, amount))
		}

		/// The account ID of the bridge's pot.
		pub fn account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}

		pub fn transfer(amount: U256, destination_account: H256) -> Result<bool, DispatchError> {
			let destination_account_id =
				T::AccountId::decode(&mut &destination_account.encode()[..])
					.map_err(|_| Error::<T>::CannotDecodeDestinationAccountId)?;

			let transferable_amount = amount.as_u128().saturated_into();
			T::Currency::transfer(
				&Self::account_id(),
				&destination_account_id,
				transferable_amount,
				ExistenceRequirement::KeepAlive,
			)?;

			Ok(true)
		}

		fn rotate_into(
			finalized_slot: u64,
			state: State,
			rotate_store: &VerifiedRotate,
		) -> Result<bool, DispatchError> {
			let finalized_header_root = Headers::<T>::get(finalized_slot);
			ensure!(
				finalized_header_root != H256::zero(),
				Error::<T>::HeaderRootNotSet
			);

			let input = ethabi::encode(&[Token::FixedBytes(finalized_header_root.0.to_vec())]);
			let sync_committee_poseidon: U256 =
				Self::verified_rotate_call(RotateFunctionId::get(), input, rotate_store)?;

			let period = finalized_slot / state.slots_per_period;
			let next_period = period + 1;

			let is_set = Self::set_sync_committee_poseidon(next_period, sync_committee_poseidon)?;

			Ok(is_set)
		}

		fn step_into(
			attested_slot: u64,
			state: State,
			step_store: &VerifiedStep,
		) -> Result<bool, DispatchError> {
			let period = attested_slot / state.slots_per_period;

			let sc_poseidon = SyncCommitteePoseidons::<T>::get(period);
			ensure!(sc_poseidon != U256::zero(), Error::<T>::SyncCommitteeNotSet);

			let input = encode_packed(sc_poseidon, attested_slot);
			let result = Self::verified_step_call(StepFunctionId::get(), input, step_store)?;

			ensure!(
				result.participation >= state.finality_threshold,
				Error::<T>::NotEnoughParticipants
			);

			let head = Head::<T>::get();
			ensure!(result.finalized_slot > head, Error::<T>::SlotBehindHead);

			let updated = Self::set_slot_roots(result)?;

			Ok(updated)
		}

		///  Sets the current slot for the chain the light client is reflecting.
		/// checks is the roots exists for the slot already. If there is
		/// an existing header but no conflict, do nothing. Avoids timestamp renewal DoS attacks.
		fn set_slot_roots(step_output: VerifiedStepOutput) -> Result<bool, DispatchError> {
			let header = Headers::<T>::get(step_output.finalized_slot);
			ensure!(header == H256::zero(), Error::<T>::HeaderRootAlreadySet);

			let execution_state_root = ExecutionStateRoots::<T>::get(step_output.finalized_slot);
			ensure!(
				execution_state_root == H256::zero(),
				Error::<T>::StateRootAlreadySet
			);

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

		/// Sets the sync committee poseidon for a given period.
		fn set_sync_committee_poseidon(period: u64, poseidon: U256) -> Result<bool, DispatchError> {
			let sync_committee_poseidons = SyncCommitteePoseidons::<T>::get(period);
			ensure!(
				sync_committee_poseidons == U256::zero(),
				Error::<T>::SyncCommitteeAlreadySet
			);

			SyncCommitteePoseidons::<T>::set(period, poseidon);

			Ok(true)
		}

		/// get_verifier returns verifier based on the provided function id.
		fn get_verifier(function_id: H256) -> Result<Verifier, Error<T>> {
			if function_id == StepFunctionId::get() {
				Self::get_step_verifier()
			} else if function_id == RotateFunctionId::get() {
				Self::get_rotate_verifier()
			} else {
				Err(Error::<T>::FunctionIdNotKnown)
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
			verified_call: &VerifiedStep,
		) -> Result<VerifiedStepOutput, DispatchError> {
			let input_hash = sha2_256(input.as_slice());

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
			verified_call: &VerifiedRotate,
		) -> Result<U256, DispatchError> {
			let input_hash = sha2_256(input.as_slice());
			if verified_call.verified_function_id == function_id
				&& verified_call.verified_input_hash == H256(input_hash)
			{
				Ok(verified_call.sync_committee_poseidon)
			} else {
				Err(Error::<T>::RotateVerificationError.into())
			}
		}

		/// Check if the given domain is supported or not
		fn is_domain_valid(domain: u32) -> bool {
			WHITELISTED_DOMAINS.contains(&domain)
		}
	}
}
