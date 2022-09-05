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
			// Store every block's corresponding hash
			let hash = frame_system::Pallet::<T>::block_hash(block_number);
			FinalizedBlockNumberToBlockHash::<T>::insert(block_number, hash);
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ExtrinsicsRootDispatched {
			sender: T::AccountId,
			block_number: T::BlockNumber,
			extrinsics_root: T::Hash,
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
		T::AccountId: Into<[u8; 32]>,
		T::BlockNumber: Into<u64>,
		T::Hash: Into<H256>,
	{
		#[pallet::weight(100)]
		pub fn try_enqueue_extrinsics_root(
			origin: OriginFor<T>,
			destination_domain: u32,
			recipient_address: H256,
			header: T::Header,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::ensure_valid_header(&header)?;
			Self::do_enqueue_extrinsics_root(sender, destination_domain, recipient_address, header)
		}
	}

	impl<T: Config> Pallet<T>
	where
		T::AccountId: Into<[u8; 32]>,
		T::BlockNumber: Into<u64>,
		T::Hash: Into<H256>,
	{
		fn do_enqueue_extrinsics_root(
			sender: T::AccountId,
			destination_domain: u32,
			recipient_address: H256,
			header: T::Header,
		) -> DispatchResult {
			let mut block_number = *header.number();
			let ext_root = *header.extrinsics_root();

			let message = DABridgeMessage::format_extrinsics_root_message(
				block_number.clone(),
				ext_root.clone(),
			);

			Home::<T>::do_dispatch(
				sender.clone(),
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

				// If we have cleared the very first block_number to hash mapping, there is nothing more to clear.
				if block_number.into() == 0 as u64 {
					break;
				}

				block_number = block_number
					.checked_sub(&1u32.into())
					.ok_or(ArithmeticError::Underflow)?;
			}

			Self::deposit_event(Event::<T>::ExtrinsicsRootDispatched {
				sender,
				block_number,
				extrinsics_root: ext_root,
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
