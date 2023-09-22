#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarking;

pub mod weights;

pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::{ValueQuery, *},
		sp_runtime::ArithmeticError::Overflow,
		transactional, DefaultNoBound,
	};
	use frame_system::pallet_prelude::{OriginFor, *};
	use nomad_base::NomadBase;
	use nomad_core::{destination_and_nonce, NomadMessage, NomadState, SignedUpdate};
	use nomad_merkle::{Merkle, NomadLightMerkle};
	use sp_core::{H160, H256};
	use sp_std::vec::Vec;

	use super::weights::WeightInfo;

	/// Default implementations of [`DefaultConfig`], which can be used to implement [`Config`].
	pub mod config_preludes {
		use super::DefaultConfig;

		/// Provides a viable default config that can be used with
		/// [`derive_impl`](`frame_support::derive_impl`) to derive a testing pallet config
		/// based on this one.
		pub struct TestDefaultConfig;

		#[frame_support::register_default_impl(TestDefaultConfig)]
		impl DefaultConfig for TestDefaultConfig {
			type MaxMessageBodyBytes = frame_support::traits::ConstU32<2048>;
			type WeightInfo = ();
		}
	}

	#[pallet::config(with_default)]
	pub trait Config: frame_system::Config + nomad_updater_manager::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Max allowed message body size
		#[pallet::constant]
		type MaxMessageBodyBytes: Get<u32>;

		/// Weights for this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
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
	pub type Nonces<T> = StorageMap<_, Twox64Concat, u32, u32, ValueQuery>;

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
	#[derive(DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub local_domain: u32,
		pub committed_root: H256,
		pub updater: H160,
		pub _phantom: PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			<Base<T>>::put(NomadBase::new(
				self.local_domain,
				self.committed_root,
				self.updater,
			));
			<Tree<T>>::put(NomadLightMerkle::default());
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
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
		UpdaterSlashed {
			updater: H160,
			reporter: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		InitializationError,
		IngestionError,
		SignatureRecoveryError,
		MessageTooLarge,
		InvalidUpdaterSignature,
		CommittedRootNotMatchUpdatePrevious,
		RootForIndexNotFound,
		IndexForRootNotFound,
		FailedState,
		MaxIndexWitnessExhausted,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		[u8; 32]: From<T::AccountId>,
	{
		/// Dispatch a message to the destination domain and recipient address.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::dispatch(message_body.len() as u32))]
		pub fn dispatch(
			origin: OriginFor<T>,
			#[pallet::compact] destination_domain: u32,
			recipient_address: H256,
			message_body: BoundedVec<u8, T::MaxMessageBodyBytes>,
		) -> DispatchResult {
			let sender: [u8; 32] = ensure_signed(origin)?.into();
			Self::do_dispatch(
				sender.into(),
				destination_domain,
				recipient_address,
				message_body,
			)
		}

		/// Verify/submit signed update.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::update())]
		pub fn update(
			origin: OriginFor<T>,
			signed_update: SignedUpdate,
			#[pallet::compact] max_index: u32,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::do_update(sender, signed_update, max_index)
		}

		/// Verify/slash updater for improper update.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::improper_update())]
		pub fn improper_update(
			origin: OriginFor<T>,
			signed_update: SignedUpdate,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::do_improper_update(sender, &signed_update)?;
			Ok(())
		}

		/// Set new updater on self as well as updater manager.
		/// Note: Not exposed as pallet call, will only be callable by the
		/// GovernanceRouter pallet when implemented.
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::set_updater())]
		pub fn set_updater(origin: OriginFor<T>, new_updater: H160) -> DispatchResult {
			ensure_root(origin)?;

			// Modify NomadBase updater
			Base::<T>::mutate(|base| base.updater = new_updater);

			// update nomad state to active
			Base::<T>::mutate(|base| base.state = NomadState::Active);
			// Rotate updater on updater manager
			nomad_updater_manager::Pallet::<T>::set_updater(new_updater)
		}
	}

	impl<T: Config> Pallet<T>
	where
		[u8; 32]: From<T::AccountId>,
	{
		fn ensure_not_failed() -> Result<(), Error<T>> {
			ensure!(
				Self::base().state != NomadState::Failed,
				Error::<T>::FailedState
			);
			Ok(())
		}

		/// Format message, insert hash into merkle tree, and update mappings
		/// between tree roots and message indices.
		pub fn do_dispatch(
			sender: H256,
			destination_domain: u32,
			recipient_address: H256,
			message_body: BoundedVec<u8, T::MaxMessageBodyBytes>,
		) -> DispatchResult {
			Self::ensure_not_failed()?;
			let base = Self::base();
			let index_of = |tree: &NomadLightMerkle| tree.count() - 1;

			// Get nonce and set new nonce
			Nonces::<T>::try_mutate(destination_domain, |nonce| -> Result<(), DispatchError> {
				let new_nonce = nonce
					.checked_add(1)
					.ok_or_else(|| DispatchError::from(Overflow))?;

				// Format message and get message hash
				let message = NomadMessage {
					origin: base.local_domain,
					sender,
					nonce: *nonce,
					destination: destination_domain,
					recipient: recipient_address,
					body: message_body,
				};
				let message_hash = message.hash();

				// Insert message hash into tree
				let tree =
					Tree::<T>::try_mutate(|tree| -> Result<NomadLightMerkle, DispatchError> {
						tree.ingest(message_hash)
							.map_err(|_| DispatchError::from(<Error<T>>::IngestionError))?;

						// Record new tree root for message
						let root = tree.root();
						let index = index_of(tree);
						RootToIndex::<T>::insert(root, index);
						IndexToRoot::<T>::insert(index, root);

						Ok(*tree)
					})?;

				Self::deposit_event(Event::<T>::Dispatch {
					message_hash,
					leaf_index: index_of(&tree),
					destination_and_nonce: destination_and_nonce(destination_domain, *nonce),
					committed_root: base.committed_root,
					message: message.to_vec(),
				});

				*nonce = new_nonce;
				Ok(())
			})?;

			Ok(())
		}

		/// Check for improper update, remove all previous root/index mappings,
		/// and emit Update event if valid.
		#[transactional]
		fn do_update(
			sender: T::AccountId,
			signed_update: SignedUpdate,
			mut max_index_witness: u32,
		) -> DispatchResult {
			if Self::do_improper_update(sender, &signed_update)? {
				return Ok(());
			}

			let mut root = signed_update.new_root();
			let previous_root = signed_update.previous_root();

			// Clear previous mappings starting from new_root, going back and
			// through previous_root. A new update's previous_root has always
			// been cleared in the previous update, as the last update's
			// new_root is always the next update's previous_root.
			while root != previous_root {
				// Ensure witness
				ensure!(max_index_witness > 0, Error::<T>::MaxIndexWitnessExhausted);
				max_index_witness -= 1;

				// Remove `RootToIndex` & `IndexToRoot` items.
				let index = RootToIndex::<T>::take(root).ok_or(Error::<T>::IndexForRootNotFound)?;
				IndexToRoot::<T>::remove(index);

				// Force an exit if `index ==0 ` or `index -1` is not found.
				root = (index != 0)
					.then(|| IndexToRoot::<T>::get(index - 1))
					.flatten()
					.unwrap_or(previous_root);
			}

			Base::<T>::mutate(|base| {
				base.set_committed_root(signed_update.new_root());

				Self::deposit_event(Event::<T>::Update {
					home_domain: base.local_domain,
					previous_root: signed_update.previous_root(),
					new_root: signed_update.new_root(),
					signature: signed_update.signature.to_vec(),
				});
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
				base.committed_root == signed_update.previous_root(),
				Error::<T>::CommittedRootNotMatchUpdatePrevious,
			);

			// Ensure updater signature is valid
			ensure!(
				base.is_updater_signature(signed_update)
					.map_err(|_| Error::<T>::SignatureRecoveryError)?,
				Error::<T>::InvalidUpdaterSignature,
			);

			// If new root not in history (invalid), slash updater and fail home
			let root_not_found = RootToIndex::<T>::get(signed_update.new_root()).is_none();
			if root_not_found {
				Self::fail(sender);
				Self::deposit_event(Event::<T>::ImproperUpdate {
					previous_root: signed_update.previous_root(),
					new_root: signed_update.new_root(),
					signature: signed_update.signature.to_vec(),
				});
			}

			Ok(root_not_found)
		}

		/// Set self in failed state and slash updater.
		fn fail(reporter: T::AccountId) {
			Base::<T>::mutate(|base| base.state = NomadState::Failed);
			nomad_updater_manager::Pallet::<T>::slash_updater(reporter.clone());

			let updater = Self::base().updater;
			Self::deposit_event(Event::<T>::UpdaterSlashed { updater, reporter });
		}
	}
}

#[cfg(any(test, feature = "runtime-benchmarks"))]
pub mod common_tests_and_benches {
	use hex_literal::hex;
	use nomad_core::{SignedUpdate, Update};
	use nomad_signature::Signature;
	use sp_core::{H256, U256};

	const EXPECTED_NEW_ROOT_LONGEST_TREE: H256 = H256(hex!(
		"dd0a05d7b71c171d06b51f11d1191f2ce23dbe679ecabea374ee3a7909383fb6"
	));

	pub fn expected_longest_tree_signed_update() -> SignedUpdate {
		SignedUpdate {
			update: Update {
				home_domain: 1111,
				previous_root: H256::zero(),
				new_root: EXPECTED_NEW_ROOT_LONGEST_TREE,
			},
			signature: Signature {
				r: U256::from_dec_str(
					"14322696571982726287696567121385710541178197176749280748421005645809674945701",
				)
				.unwrap(),
				s: U256::from_dec_str(
					"30270442654181520335096087658805894014217975025586657453237735885054021806875",
				)
				.unwrap(),
				v: 27,
			},
		}
	}
}
