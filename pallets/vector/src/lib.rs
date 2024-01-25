#![cfg_attr(not(feature = "std"), no_std)]

use crate::target_amb::MessageStatusEnum;
use crate::verifier::Verifier;
use frame_support::traits::{Currency, ExistenceRequirement, UnixTime};
use frame_support::{pallet_prelude::*, PalletId};
use frame_system::submitted_data::{BoundedData, MessageType};
pub use pallet::*;
use sp_core::H256;
use sp_runtime::SaturatedConversion;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
mod state;
mod target_amb;
#[cfg(test)]
mod tests;
mod verifier;
mod weights;

pub type FunctionInput = BoundedVec<u8, ConstU32<256>>;
pub type FunctionOutput = BoundedVec<u8, ConstU32<512>>;
pub type FunctionProof = BoundedVec<u8, ConstU32<1048>>;
pub type ValidProof = BoundedVec<BoundedVec<u8, ConstU32<2048>>, ConstU32<32>>;

// Avail asset is supported for now
pub const SUPPORTED_ASSET_ID: H256 = H256::zero();

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use ark_std::{vec, vec::Vec};
	use ethabi::Token;
	use ethabi::Token::Uint;
	use frame_support::dispatch::{GetDispatchInfo, UnfilteredDispatchable};
	use frame_support::traits::LockableCurrency;
	use frame_support::{pallet_prelude::ValueQuery, DefaultNoBound};
	use frame_system::pallet_prelude::*;
	use frame_system::submitted_data::Message;
	use primitive_types::H160;
	use primitive_types::{H256, U256};
	use sp_io::hashing::keccak_256;
	use sp_io::hashing::sha2_256;
	use sp_runtime::traits::AccountIdConversion;
	pub use weights::WeightInfo;

	use crate::state::Configuration;
	use crate::state::{
		parse_rotate_output, parse_step_output, VerifiedRotate, VerifiedStep, VerifiedStepOutput,
	};
	use crate::target_amb::{get_storage_root, get_storage_value};
	use crate::verifier::encode_packed;

	use super::*;

	#[pallet::error]
	pub enum Error<T> {
		VerificationError,
		NotEnoughParticipants,
		ConfigurationNotSet,
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
		HeaderRootAlreadySet,
		StateRootAlreadySet,
		SyncCommitteeAlreadySet,
		SyncCommitteeNotSet,
		MessageAlreadyExecuted,
		WrongDestinationChain,
		UnsupportedOriginChain,
		BroadcasterSourceChainNotSet,
		SourceChainFrozen,
		CannotGetStorageRoot,
		CannotGetStorageValue,
		InvalidMessageHash,
		CannotDecodeData,
		CannotDecodeDestinationAccountId,
		AssetNotSupported,
		/// Given inputs for the selected MessageType are invalid
		InvalidBridgeInputs,
		/// Domain is not supported
		DomainNotSupported,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// emit event once the head is updated.
		HeaderUpdate {
			slot: u64,
			finalization_root: H256,
			execution_state_root: H256,
		},
		/// emit event once the sync committee updates.
		SyncCommitteeUpdate { period: u64, root: U256 },
		/// emit when new updater is set
		BroadcasterUpdate { old: H256, new: H256, domain: u32 },
		/// emit when message gets executed.
		ExecutedMessage {
			from: H256,
			to: H256,
			message_id: u64,
			message_root: H256,
		},
		/// emit if source chain gets frozen.
		SourceChainFrozen { source_chain_id: u32, frozen: bool },
		/// emit when message is submitted.
		MessageSubmitted {
			from: T::AccountId,
			to: H256,
			message_type: MessageType,
			destination_domain: u32,
		},
		/// Whitelisted domains were updated.
		WhitelistedDomainsUpdated,
		/// Configuration was updated.
		ConfigurationUpdated {
			slots_per_period: u64,
			finality_threshold: u16,
		},
	}

	/// Storage for a head updates.
	#[pallet::storage]
	#[pallet::getter(fn head)]
	pub type Head<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Maps from a slot to a block header root.
	#[pallet::storage]
	#[pallet::getter(fn headers)]
	pub type Headers<T> = StorageMap<_, Identity, u64, H256, ValueQuery>;

	/// Maps slot to the timestamp of when the headers mapping was updated with slot as a key
	#[pallet::storage]
	pub type Timestamps<T> = StorageMap<_, Identity, u64, u64, ValueQuery>;

	/// Maps from a slot to the current finalized ethereum execution state root.
	#[pallet::storage]
	pub type ExecutionStateRoots<T> = StorageMap<_, Identity, u64, H256, ValueQuery>;

	/// Maps from a period to the poseidon commitment for the sync committee.
	#[pallet::storage]
	#[pallet::getter(fn sync_committee_poseidons)]
	pub type SyncCommitteePoseidons<T> = StorageMap<_, Identity, u64, U256, ValueQuery>;

	/// Storage for a config of finality threshold and slots per period.
	#[pallet::storage]
	pub type ConfigurationStorage<T: Config> = StorageValue<_, Configuration, ValueQuery>;

	/// Maps status of the message to the message root.
	#[pallet::storage]
	pub type MessageStatus<T> = StorageMap<_, Identity, H256, MessageStatusEnum, ValueQuery>;

	/// Mapping between source chainId and the address of the broadcaster on that chain.
	#[pallet::storage]
	pub type Broadcasters<T> = StorageMap<_, Identity, u32, H256, ValueQuery>;

	/// Flags source chain to be frozen.
	#[pallet::storage]
	pub type SourceChainFrozen<T> = StorageMap<_, Identity, u32, bool, ValueQuery>;

	/// List of permitted domains.
	#[pallet::storage]
	pub type WhitelistedDomains<T> = StorageValue<_, BoundedVec<u32, ConstU32<10_000>>, ValueQuery>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Because this pallet has dispatchables, it depends on the runtime's definition of a call.
		type RuntimeCall: Parameter
			+ UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ GetDispatchInfo;
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
		/// Currency type for this pallet.
		type Currency: LockableCurrency<Self::AccountId, Moment = BlockNumberFor<Self>>;
		/// Dependency that can provide current time.
		type TimeProvider: UnixTime;
		/// Step verification key constant
		#[pallet::constant]
		type StepVerificationKey: Get<Vec<u8>>;
		/// Rotate verification key constant
		#[pallet::constant]
		type RotateVerificationKey: Get<Vec<u8>>;
		/// The step function identifier is used to distinguish step-related functionality within the fulfill_call function.
		/// When the provided function_id matches the step function identifier, specific logic related to step functions is executed.
		#[pallet::constant]
		type StepFunctionId: Get<H256>;
		/// The rotate function identifier is used to identify and handle rotate-related functionality within the fulfill_call function.
		/// When the provided function_id matches the rotate function identifier, specific logic related to rotate functions is executed.
		#[pallet::constant]
		type RotateFunctionId: Get<H256>;
		/// The index of the `messages` mapping in contract.
		/// This is mandatory when calling execute messages via storage proofs.
		#[pallet::constant]
		type MessageMappingStorageIndex: Get<u64>;
		/// Bridge's pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
		/// Unique value associated with Avail Network. Used to distinguish messages between Avail and non-Avail networks.
		#[pallet::constant]
		type AvailDomain: Get<u32>;
	}

	#[pallet::genesis_config]
	#[derive(DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub slots_per_period: u64,
		pub finality_threshold: u16,
		pub broadcaster: H256,
		pub broadcaster_domain: u32,
		pub whitelisted_domains: Vec<u32>,
		pub _phantom: PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			// Preconfigure init data
			<ConfigurationStorage<T>>::put(Configuration {
				slots_per_period: self.slots_per_period,
				finality_threshold: self.finality_threshold,
			});

			let mut domains = self.whitelisted_domains.clone();

			// Whitelisted domains sanitization.
			domains.sort();
			domains.dedup();
			let domains =
				BoundedVec::try_from(domains).expect("Cannot have more than 10_000 domains.");
			WhitelistedDomains::<T>::put(domains);

			Broadcasters::<T>::set(self.broadcaster_domain, self.broadcaster);
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
		#[pallet::weight(T::WeightInfo::fulfill_call())]
		pub fn fulfill_call(
			origin: OriginFor<T>,
			function_id: H256,
			input: FunctionInput,
			output: FunctionOutput,
			proof: FunctionProof,
			slot: u64,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;
			let state = ConfigurationStorage::<T>::get();
			// compute hashes
			let input_hash = H256(sha2_256(input.as_slice()));
			let output_hash = H256(sha2_256(output.as_slice()));
			let verifier = Self::get_verifier(function_id)?;

			let is_success = verifier
				.verify(input_hash, output_hash, proof.to_vec())
				.map_err(|_| Error::<T>::VerificationError)?;

			// make sure that verification call is valid
			ensure!(is_success, Error::<T>::VerificationFailed);

			if function_id == T::StepFunctionId::get() {
				let vs =
					VerifiedStep::new(function_id, input_hash, parse_step_output(output.to_vec()));

				if Self::step_into(slot, state, &vs)? {
					Self::deposit_event(Event::HeaderUpdate {
						slot: vs.verified_output.finalized_slot,
						finalization_root: vs.verified_output.finalized_header_root,
						execution_state_root: vs.verified_output.execution_state_root,
					});
				}
			} else if function_id == T::RotateFunctionId::get() {
				let vr = VerifiedRotate::new(
					function_id,
					input_hash,
					parse_rotate_output(output.to_vec()),
				);

				let period = Self::rotate_into(slot, state, &vr)?;
				Self::deposit_event(Event::SyncCommitteeUpdate {
					period,
					root: vr.sync_committee_poseidon,
				});
			} else {
				return Err(Error::<T>::FunctionIdNotKnown.into());
			}

			Ok(().into())
		}

		/// Executes message if a valid proofs are provided for the supported message type, assets and domains.
		#[pallet::call_index(1)]
		#[pallet::weight({
        match message.message_type {
        MessageType::ArbitraryMessage => T::WeightInfo::execute_arbitrary_message(message.data.len() as u32),
        MessageType::FungibleToken => T::WeightInfo::execute_fungible_token(),
        }
        })]
		pub fn execute(
			origin: OriginFor<T>,
			slot: u64,
			message: Message,
			account_proof: ValidProof,
			storage_proof: ValidProof,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;
			let encoded_data = message.clone().abi_encode();
			let message_root = H256(keccak_256(encoded_data.as_slice()));

			Self::check_preconditions(&message, message_root)?;

			ensure!(
				!SourceChainFrozen::<T>::get(message.origin_domain),
				Error::<T>::SourceChainFrozen
			);
			let root = ExecutionStateRoots::<T>::get(slot);
			let broadcaster = Broadcasters::<T>::get(message.origin_domain);

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
			let mm_idx = Uint(U256::from(T::MessageMappingStorageIndex::get()));
			let slot_key = H256(keccak_256(ethabi::encode(&[nonce, mm_idx]).as_slice()));

			let storage_proof_vec = storage_proof
				.iter()
				.map(|inner_bounded_vec| inner_bounded_vec.iter().copied().collect())
				.collect();

			let slot_value = get_storage_value(slot_key, storage_root, storage_proof_vec)
				.map_err(|_| Error::<T>::CannotGetStorageValue)?;

			ensure!(slot_value == message_root, Error::<T>::InvalidMessageHash);

			match message.message_type {
				MessageType::ArbitraryMessage => {
					MessageStatus::<T>::set(message_root, MessageStatusEnum::ExecutionSucceeded);
					Self::deposit_event(Event::<T>::ExecutedMessage {
						from: message.from,
						to: message.to,
						message_id: message.id,
						message_root,
					})
				},

				MessageType::FungibleToken => {
					let (asset_id, amount) = Self::decode_message_data(message.data.to_vec())?;
					ensure!(
						SUPPORTED_ASSET_ID == asset_id,
						Error::<T>::AssetNotSupported
					);

					let destination_account_id =
						T::AccountId::decode(&mut &message.to.encode()[..])
							.map_err(|_| Error::<T>::CannotDecodeDestinationAccountId)?;

					T::Currency::transfer(
						&Self::account_id(),
						&destination_account_id,
						amount.as_u128().saturated_into(),
						ExistenceRequirement::AllowDeath,
					)?;

					MessageStatus::<T>::set(message_root, MessageStatusEnum::ExecutionSucceeded);
					Self::deposit_event(Event::<T>::ExecutedMessage {
						from: message.from,
						to: message.to,
						message_id: message.id,
						message_root,
					})
				},
			}

			Ok(().into())
		}

		/// source_chain_froze froze source chain and prevent messages to be executed.
		//
		// Test names: source_chain_froze_works_with_root(), source_chain_froze_does_not_work_with_non_root()
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::source_chain_froze())]
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

		/// send_message sends a message from a origin chain to the destination chain.
		//
		// Test names:
		//	send_message_fungible_token_works(), send_message_fungible_token_doesnt_accept_data(),
		//	send_message_fungible_token_doesnt_accept_empty_asset_id(), send_message_fungible_token_doesnt_accept_empty_value(),
		//	send_message_arbitrary_message_works(), send_message_arbitrary_message_doesnt_accept_value(),
		//	send_message_arbitrary_message_doesnt_accept_asset_id(), send_message_arbitrary_message_doesnt_accept_empty_data()
		#[pallet::call_index(3)]
		#[pallet::weight({
        match message_type {
        MessageType::ArbitraryMessage => T::WeightInfo::send_message_arbitrary_message(data.as_ref().unwrap_or(& Default::default()).len() as u32),
        MessageType::FungibleToken => T::WeightInfo::send_message_fungible_token(),
        }
        })]
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
						value.is_none() && asset_id.is_none() && data.is_some(),
						Error::<T>::InvalidBridgeInputs
					);
					Self::deposit_event(Event::MessageSubmitted {
						from: who,
						to,
						message_type: message_type.clone(),
						destination_domain: domain,
					});
				},
				MessageType::FungibleToken => {
					ensure!(
						value.is_some() && asset_id.is_some() && data.is_none(),
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
						message_type: message_type.clone(),
						destination_domain: domain,
					});
				},
			}
			Ok(().into())
		}

		/// set_poseidon_hash sets poseidon hash of the sync commettee for the particular period.
		//
		// Test names: set_poseidon_hash_works_with_root(), set_poseidon_hash_does_not_work_with_non_root()
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::set_poseidon_hash())]
		pub fn set_poseidon_hash(
			origin: OriginFor<T>,
			period: u64,
			poseidon_hash: BoundedVec<u8, ConstU32<200>>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let hash = U256::from(poseidon_hash.to_vec().as_slice());

			SyncCommitteePoseidons::<T>::insert(period, hash);
			Self::deposit_event(Event::SyncCommitteeUpdate { period, root: hash });
			Ok(().into())
		}

		/// set_broadcaster sets the broadcaster address of the message from the origin chain.
		//
		// Test names: set_broadcaster_works_with_root(), set_broadcaster_does_not_work_with_non_root()
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::set_broadcaster())]
		pub fn set_broadcaster(
			origin: OriginFor<T>,
			broadcaster_domain: u32,
			broadcaster: H256,
		) -> DispatchResult {
			ensure_root(origin)?;
			let old_bc = Broadcasters::<T>::get(broadcaster_domain);

			Broadcasters::<T>::set(broadcaster_domain, broadcaster);

			Self::deposit_event(Event::BroadcasterUpdate {
				old: old_bc,
				new: broadcaster,
				domain: broadcaster_domain,
			});

			Ok(())
		}

		/// The set_whitelisted_domains function allows the root (administrator) to set the whitelisted domains. It is a
		/// privileged function intended for administrative purposes, used to manage a list of permitted domains.
		//
		// Test names: set_whitelisted_domains_works_with_root(), set_whitelisted_domains_does_not_work_with_non_root()
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::set_whitelisted_domains())]
		pub fn set_whitelisted_domains(
			origin: OriginFor<T>,
			value: BoundedVec<u32, ConstU32<10_000>>,
		) -> DispatchResult {
			ensure_root(origin)?;
			WhitelistedDomains::<T>::put(value);

			Self::deposit_event(Event::WhitelistedDomainsUpdated);

			Ok(())
		}

		/// The set_configuration function allows the root (administrator) to set the configuration. It is a
		/// privileged function intended for administrative purposes, used to manage slots_per_period and finality_threshold values.
		//
		// Test names: set_configuration_works_with_root(), set_configuration_does_not_work_with_non_root()
		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::set_configuration())]
		pub fn set_configuration(origin: OriginFor<T>, value: Configuration) -> DispatchResult {
			ensure_root(origin)?;
			ConfigurationStorage::<T>::put(value);

			Self::deposit_event(Event::ConfigurationUpdated {
				slots_per_period: value.slots_per_period,
				finality_threshold: value.finality_threshold,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn check_preconditions(message: &Message, message_root: H256) -> Result<(), DispatchError> {
			let message_status = MessageStatus::<T>::get(message_root);
			// Message must not be executed
			ensure!(
				message_status == MessageStatusEnum::NotExecuted,
				Error::<T>::MessageAlreadyExecuted
			);

			ensure!(
				message.destination_domain == T::AvailDomain::get(),
				Error::<T>::WrongDestinationChain
			);

			ensure!(
				WhitelistedDomains::<T>::get().contains(&message.origin_domain),
				Error::<T>::UnsupportedOriginChain
			);

			let source_chain = Broadcasters::<T>::get(message.origin_domain);
			ensure!(
				source_chain != H256::zero(),
				Error::<T>::BroadcasterSourceChainNotSet
			);

			Ok(())
		}

		fn decode_message_data(data: Vec<u8>) -> Result<(H256, U256), DispatchError> {
			//abi.encode(ASSET_ID, msg.value),
			let decoded_data = ethabi::decode(
				&[
					ethabi::ParamType::FixedBytes(32),
					ethabi::ParamType::Uint(256),
				],
				data.as_slice(),
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

		fn rotate_into(
			finalized_slot: u64,
			cfg: Configuration,
			rotate_store: &VerifiedRotate,
		) -> Result<u64, DispatchError> {
			let finalized_header_root = Headers::<T>::get(finalized_slot);
			ensure!(
				finalized_header_root != H256::zero(),
				Error::<T>::HeaderRootNotSet
			);

			let input = ethabi::encode(&[Token::FixedBytes(finalized_header_root.0.to_vec())]);
			let sync_committee_poseidon: U256 =
				Self::verified_rotate_call(T::RotateFunctionId::get(), input, rotate_store)?;

			let period = finalized_slot
				.checked_div(cfg.slots_per_period)
				.ok_or(Error::<T>::ConfigurationNotSet)?;
			let next_period = period + 1;

			Self::set_sync_committee_poseidon(next_period, sync_committee_poseidon)?;

			Ok(period)
		}

		fn step_into(
			attested_slot: u64,
			cfg: Configuration,
			step_store: &VerifiedStep,
		) -> Result<bool, DispatchError> {
			let period = attested_slot
				.checked_div(cfg.slots_per_period)
				.ok_or(Error::<T>::ConfigurationNotSet)?;

			let sc_poseidon = SyncCommitteePoseidons::<T>::get(period);
			ensure!(sc_poseidon != U256::zero(), Error::<T>::SyncCommitteeNotSet);

			let input = encode_packed(sc_poseidon, attested_slot);
			let result = Self::verified_step_call(T::StepFunctionId::get(), input, step_store)?;

			ensure!(
				result.participation >= cfg.finality_threshold,
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
		fn set_sync_committee_poseidon(period: u64, poseidon: U256) -> Result<(), DispatchError> {
			let sync_committee_poseidons = SyncCommitteePoseidons::<T>::get(period);
			ensure!(
				sync_committee_poseidons == U256::zero(),
				Error::<T>::SyncCommitteeAlreadySet
			);

			SyncCommitteePoseidons::<T>::set(period, poseidon);

			Ok(())
		}

		/// get_verifier returns verifier based on the provided function id.
		fn get_verifier(function_id: H256) -> Result<Verifier, Error<T>> {
			if function_id == T::StepFunctionId::get() {
				Self::get_step_verifier()
			} else if function_id == T::RotateFunctionId::get() {
				Self::get_rotate_verifier()
			} else {
				Err(Error::<T>::FunctionIdNotKnown)
			}
		}

		fn get_step_verifier() -> Result<Verifier, Error<T>> {
			let vk = T::StepVerificationKey::get();
			ensure!(!vk.is_empty(), Error::<T>::VerificationKeyIsNotSet);
			let deserialized_vk = Verifier::from_json_u8_slice(vk.as_slice())
				.map_err(|_| Error::<T>::MalformedVerificationKey)?;
			Ok(deserialized_vk)
		}

		fn get_rotate_verifier() -> Result<Verifier, Error<T>> {
			let vk = T::RotateVerificationKey::get();
			ensure!(!vk.is_empty(), Error::<T>::VerificationKeyIsNotSet);
			let deserialized_vk = Verifier::from_json_u8_slice(vk.as_slice())
				.map_err(|_| Error::<T>::MalformedVerificationKey)?;
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
			WhitelistedDomains::<T>::get().contains(&domain)
		}
	}
}
