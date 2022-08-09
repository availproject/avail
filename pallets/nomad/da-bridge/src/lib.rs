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
	use frame_support::{pallet_prelude::*, sp_runtime::traits::Header};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};

	#[pallet::config]
	pub trait Config: frame_system::Config + home::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Leaf index to root
	#[pallet::storage]
	#[pallet::getter(fn block_number_to_block_hash)]
	pub type BlockNumberToBlockHash<T: Config> =
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
			let hash = frame_system::Pallet::<T>::block_hash(block_number);
			BlockNumberToBlockHash::<T>::insert(block_number, hash);
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ExtRootDispatched {
			sender: T::AccountId,
			ext_root: T::Hash,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		InitializationError,
		BlockNotFinal,
		HashOfBlockNotMatchBlockNumber,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100)]
		pub fn try_enqueue_ext_root(origin: OriginFor<T>, header: T::Header) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Self::ensure_valid_header(&header)?;
			let ext_root = header.extrinsics_root();

			// TODO: Dispatch ext root to home in message

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn ensure_valid_header(header: &T::Header) -> Result<(), DispatchError> {
			let block_number = header.number();
			let hash = header.hash();

			let mapped_hash = BlockNumberToBlockHash::<T>::try_get(block_number)
				.ok()
				.ok_or(Error::<T>::BlockNotFinal)?;

			ensure!(
				mapped_hash == hash,
				Error::<T>::HashOfBlockNotMatchBlockNumber,
			);

			Ok(())
		}
	}
}
