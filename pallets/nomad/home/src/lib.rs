#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::{ValueQuery, *},
		sp_runtime::ArithmeticError,
	};
	use frame_system::pallet_prelude::{OriginFor, *};
	use merkle::{Merkle, NomadLightMerkle};
	use nomad_base::NomadBase;
	use nomad_core::{destination_and_nonce, NomadMessage, NomadState, SignedUpdate};
	use sp_core::{H160, H256};
	use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config + updater_manager::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Max allowed message body size
		#[pallet::constant]
		type MaxMessageBodyBytes: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Nomad base
	#[pallet::storage]
	#[pallet::getter(fn base)]
	pub type Base<T> = StorageValue<_, NomadBase, ValueQuery>;

	// Merkle tree
	#[pallet::storage]
	#[pallet::getter(fn tree)]
	pub type Tree<T> = StorageValue<_, NomadLightMerkle, ValueQuery>;

	// Nonces
	#[pallet::storage]
	#[pallet::getter(fn nonces)]
	pub type Nonces<T> = StorageMap<_, Twox64Concat, u32, u32>;

	// Leaf index to root
	#[pallet::storage]
	#[pallet::getter(fn index_to_root)]
	pub type IndexToRoot<T: Config> = StorageMap<_, Twox64Concat, u32, H256>;

	// Root to leaf index
	#[pallet::storage]
	#[pallet::getter(fn root_to_index)]
	pub type RootToIndex<T: Config> = StorageMap<_, Twox64Concat, H256, u32>;

	// Genesis config
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub local_domain: u32,
		pub committed_root: H256,
		pub updater: H160,
		pub _phantom: PhantomData<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				local_domain: Default::default(),
				committed_root: Default::default(),
				updater: Default::default(),
				_phantom: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			<Base<T>>::put(NomadBase::new(
				self.local_domain,
				self.committed_root,
				self.updater,
			));
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Dispatch {
			message_hash: H256,
			leaf_index: u32,
			destination_and_nonce: u64,
			committed_root: H256,
			message: Vec<u8>,
		},
		Update {
			home_domain: u32,
			previous_root: H256,
			new_root: H256,
			signature: Vec<u8>,
		},
		ImproperUpdate {
			previous_root: H256,
			new_root: H256,
			signature: Vec<u8>,
		},
		// UpdateV2 {
		// 	home_domain: u32,
		// 	root: H256,
		// 	signature: Vec<u8>,
		// },
		// ImproperUpdateV2 {
		// 	root: H256,
		// 	signature: Vec<u8>,
		// },
		UpdaterSlashed {
			updater: H160,
			reporter: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		InitializationError,
		IngestionError,
		MessageTooLarge,
		InvalidUpdaterSignature,
		CommittedRootNotMatchUpdatePrevious,
		RootForIndexNotFound,
		IndexForRootNotFound,
		FailedState,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Dispatch a message to the destination domain and recipient address.
		#[pallet::weight(100)]
		pub fn dispatch(
			origin: OriginFor<T>,
			destination_domain: u32,
			recipient_address: H256,
			message_body: Vec<u8>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::do_dispatch(sender, destination_domain, recipient_address, message_body)
		}

		/// Verify/submit signed update.
		#[pallet::weight(100)]
		pub fn update(origin: OriginFor<T>, signed_update: SignedUpdate) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::do_update(sender, signed_update)
		}

		/// Verify/slash updater for improper update.
		#[pallet::weight(100)]
		pub fn improper_update(
			origin: OriginFor<T>,
			signed_update: SignedUpdate,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::do_improper_update(sender, &signed_update)?;
			Ok(())
		}
	}

	// /// Verify/submit signed update.
	// #[pallet::weight(100)]
	// pub fn update_v2(origin: OriginFor<T>, signed_update: SignedUpdate) -> DispatchResult {
	// 	let sender = ensure_signed(origin)?;
	// 	Self::do_update_v2(sender, signed_update)
	// }

	// /// Verify/slash updater for improper update.
	// #[pallet::weight(100)]
	// pub fn improper_update_v2(
	// 	origin: OriginFor<T>,
	// 	signed_update: SignedUpdate,
	// ) -> DispatchResult {
	// 	let sender = ensure_signed(origin)?;
	// 	Self::do_improper_update_v2(sender, &signed_update)?;
	// 	Ok(())
	// }

	impl<T: Config> Pallet<T> {
		pub fn state() -> NomadState { Self::base().state() }

		pub fn root() -> H256 { Self::tree().root() }

		pub fn get_nonce(domain: u32) -> u32 { Self::nonces(domain).unwrap_or_default() }

		fn ensure_not_failed() -> Result<(), Error<T>> {
			ensure!(
				Self::base().state() != NomadState::Failed,
				Error::<T>::FailedState
			);
			Ok(())
		}

		/// Format message, insert hash into merkle tree, and update mappings
		/// between tree roots and message indices.
		fn do_dispatch(
			sender: T::AccountId,
			destination_domain: u32,
			recipient_address: H256,
			message_body: Vec<u8>,
		) -> DispatchResult {
			Self::ensure_not_failed()?;

			// Check message length against max
			let message_length = message_body.len() as u32;
			ensure!(
				message_length < T::MaxMessageBodyBytes::get(),
				Error::<T>::MessageTooLarge
			);

			// Get nonce and set new nonce
			let nonce = Self::nonces(destination_domain).unwrap_or_default();
			let new_nonce = nonce.checked_add(1).ok_or(ArithmeticError::Overflow)?;
			Nonces::<T>::insert(destination_domain, new_nonce);

			// Get info for message to dispatch
			let origin = Self::base().local_domain();

			// Format message and get message hash
			let message = NomadMessage {
				origin,
				sender: sender.encode(), // 32 byte length enforced when message hashed
				nonce,
				destination: destination_domain,
				recipient: recipient_address,
				body: message_body,
			};
			let message_hash = message.hash();

			// Get tree count as message index before inserting message
			let index = Self::tree().count();

			// Insert message hash into tree
			Tree::<T>::try_mutate(|tree| tree.ingest(message_hash))
				.map_err(|_| <Error<T>>::IngestionError)?;

			// Record new tree root for message
			let root = Self::tree().root();
			RootToIndex::<T>::insert(root, index);
			IndexToRoot::<T>::insert(index, root);

			Self::deposit_event(Event::<T>::Dispatch {
				message_hash,
				leaf_index: index,
				destination_and_nonce: destination_and_nonce(destination_domain, nonce),
				committed_root: Self::base().committed_root(),
				message: message.to_vec(),
			});

			Ok(())
		}

		/// Check for improper update, remove all previous root/index mappings,
		/// and emit Update event if valid.
		fn do_update(sender: T::AccountId, signed_update: SignedUpdate) -> DispatchResult {
			Self::ensure_not_failed()?;

			if Self::do_improper_update(sender, &signed_update)? {
				return Ok(());
			}

			Self::deposit_event(Event::<T>::Update {
				home_domain: Self::base().local_domain(),
				previous_root: signed_update.update.previous_root,
				new_root: signed_update.update.new_root,
				signature: signed_update.signature.to_vec(),
			});

			Ok(())
		}

		/// Ensure signed merkle root once existed by checking mapping of roots
		/// to indices.
		fn do_improper_update(
			sender: T::AccountId,
			signed_update: &SignedUpdate,
		) -> Result<bool, DispatchError> {
			Self::ensure_not_failed()?;

			let base = Self::base();

			// Ensure previous root matches current committed root
			ensure!(
				base.committed_root() == signed_update.update.previous_root,
				Error::<T>::CommittedRootNotMatchUpdatePrevious,
			);

			// Ensure updater signature is valid
			ensure!(
				base.is_updater_signature(&signed_update),
				Error::<T>::InvalidUpdaterSignature,
			);

			// Ensure new root is exists in history
			let root = signed_update.update.new_root;
			let get_root_res = RootToIndex::<T>::try_get(root);

			// If signed root invalid, slash updater and fail home
			if get_root_res.is_err() {
				Self::fail(sender);
				Self::deposit_event(Event::<T>::ImproperUpdate {
					previous_root: signed_update.update.previous_root,
					new_root: signed_update.update.new_root,
					signature: signed_update.signature.to_vec(),
				});
				return Ok(true);
			}

			Ok(false)
		}

		// /// Check for improper update, remove all previous root/index mappings,
		/// and emit Update event if valid.
		// fn do_update_v2(sender: T::AccountId, signed_update: SignedUpdate) -> DispatchResult {
		// 	Self::ensure_not_failed()?;

		// 	if Self::do_improper_update_v2(sender, &signed_update)? {
		// 		return Ok(());
		// 	}

		// 	Self::deposit_event(Event::<T>::Update {
		// 		home_domain: Self::base().local_domain(),
		// 		root: signed_update.update.root,
		// 		signature: signed_update.signature.to_vec(),
		// 	});

		// 	Ok(())
		// }

		// /// Ensure signed merkle root once existed by checking mapping of roots
		/// to indices.
		// fn do_improper_update_v2(
		// 	sender: T::AccountId,
		// 	signed_update: &SignedUpdate,
		// ) -> Result<bool, DispatchError> {
		// 	Self::ensure_not_failed()?;

		// 	let base = Self::base();

		// 	// Ensure updater signature is valid
		// 	ensure!(
		// 		base.is_updater_signature(&signed_update),
		// 		Error::<T>::InvalidUpdaterSignature,
		// 	);

		// 	// Ensure signed root is exists in history
		// 	let root = signed_update.update.root;
		// 	let get_root_res = RootToIndex::<T>::try_get(root);

		// 	// If signed root invalid, slash updater and fail home
		// 	if get_root_res.is_err() {
		// 		Self::fail(sender);
		// 		Self::deposit_event(Event::<T>::ImproperUpdate {
		// 			root,
		// 			signature: signed_update.signature.to_vec(),
		// 		});
		// 		return Ok(true);
		// 	}

		// 	Ok(false)
		// }

		/// Set self in failed state and slash updater.
		fn fail(reporter: T::AccountId) {
			Base::<T>::mutate(|base| base.set_state(NomadState::Failed));
			updater_manager::Pallet::<T>::slash_updater(reporter.clone());

			let updater = Self::base().updater();
			Self::deposit_event(Event::<T>::UpdaterSlashed { updater, reporter });
		}

		/// Set new updater on self as well as updater manager.
		/// Note: Will only be callable by the GovernanceRouter pallet
		pub fn set_updater(new_updater: H160) -> DispatchResult {
			// Modify NomadBase updater
			Base::<T>::mutate(|base| base.set_updater(new_updater));

			// Rotate updater on updater manager
			updater_manager::Pallet::<T>::set_updater(new_updater)
		}
	}
}
