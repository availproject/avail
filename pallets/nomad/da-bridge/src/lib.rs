#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

mod message;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use da_primitives::traits::ExtendedHeader;
	use frame_support::{
		pallet_prelude::*,
		sp_runtime::{
			traits::{CheckedSub, Header},
			ArithmeticError,
		},
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use home::Pallet as Home;
	use primitive_types::H256;

	use crate::message::DABridgeMessage;

	#[pallet::config]
	pub trait Config: frame_system::Config + home::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[pallet::constant]
		type DABridgePalletId: Get<H256>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Block number to block hash mapping
	#[pallet::storage]
	#[pallet::getter(fn finalized_block_number_to_block_hash)]
	pub type FinalizedBlockNumberToBlockHash<T: Config> =
		StorageMap<_, Twox64Concat, T::BlockNumber, T::Hash>;

	// Genesis config
	#[pallet::genesis_config]
	pub struct GenesisConfig {}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self { Self {} }
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {}
	}

	// Hooks
	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_finalize(block_number: T::BlockNumber) {
			let last = block_number.checked_sub(&1u32.into()).unwrap();

			// Store every finalized block's corresponding hash
			let hash = frame_system::Pallet::<T>::block_hash(last);
			FinalizedBlockNumberToBlockHash::<T>::insert(last, hash);

			Self::deposit_event(Event::<T>::FinalizedBlockHashStored { number: last, hash });
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		FinalizedBlockHashStored {
			number: T::BlockNumber,
			hash: T::Hash,
		},
		DataRootDispatched {
			sender: T::AccountId,
			block_number: T::BlockNumber,
			data_root: [u8; 32],
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		InitializationError,
		BlockNotFinal,
		HashOfBlockNotMatchBlockNumber,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		[u8; 32]: From<T::AccountId>,
		H256: From<T::Hash>,
		u32: From<T::BlockNumber>,
	{
		#[pallet::weight(100)]
		pub fn try_enqueue_data_root(
			origin: OriginFor<T>,
			destination_domain: u32,
			recipient_address: H256,
			header: T::Header,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::ensure_valid_header(&header)?;
			Self::do_enqueue_data_root(sender, destination_domain, recipient_address, header)
		}
	}

	impl<T: Config> Pallet<T>
	where
		[u8; 32]: From<T::AccountId>,
		H256: From<T::Hash>,
		u32: From<T::BlockNumber>,
	{
		fn do_enqueue_data_root(
			sender: T::AccountId,
			destination_domain: u32,
			recipient_address: H256,
			header: T::Header,
		) -> DispatchResult {
			let mut block_number = *header.number();
			let data_root = *header.data_root();

			let message =
				DABridgeMessage::format_data_root_message(block_number.clone(), data_root.clone());

			Home::<T>::do_dispatch(
				T::DABridgePalletId::get(),
				destination_domain,
				recipient_address,
				message.as_ref().to_vec(),
			)?;

			// Clear previous block_number to hash mappings, starting at the
			// current submitted block_number going backwards. Because the
			// runtime maps _every_ block number's hash, the sequence is always
			// contiguous. If we are tracing backwards and find a block number
			// that doesn't have a hash, we know everything before it has been
			// cleared.
			while FinalizedBlockNumberToBlockHash::<T>::contains_key(block_number) {
				FinalizedBlockNumberToBlockHash::<T>::remove(block_number);

				// If we have cleared the 0th first block_number to hash
				// mapping, there is nothing more to clear.
				let b: u32 = block_number.into();
				if b == 0 as u32 {
					break;
				}

				// Decrement block number
				block_number = block_number
					.checked_sub(&1u32.into())
					.ok_or(ArithmeticError::Underflow)?;
			}

			Self::deposit_event(Event::<T>::DataRootDispatched {
				sender,
				block_number,
				data_root,
			});

			Ok(())
		}

		fn ensure_valid_header(header: &T::Header) -> Result<(), DispatchError> {
			let block_number = header.number();
			let hash = header.hash();

			// Ensure header's block number is in the finalized mapping
			let mapped_hash = FinalizedBlockNumberToBlockHash::<T>::get(block_number)
				.ok_or(Error::<T>::BlockNotFinal)?;

			// Ensure header's hash matches that in the block number to hash
			// mapping
			ensure!(
				mapped_hash == hash,
				Error::<T>::HashOfBlockNotMatchBlockNumber,
			);

			Ok(())
		}
	}
}
