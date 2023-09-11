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
		DefaultNoBound,
	};
	use sp_core::H160;
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// Updater
	#[pallet::storage]
	#[pallet::getter(fn updater)]
	pub type Updater<T> = StorageValue<_, H160, ValueQuery>;

	// Genesis config
	#[pallet::genesis_config]
	#[derive(DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub updater: H160,
		pub _phantom: PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			<Updater<T>>::put(self.updater);
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NewUpdater {
			old_updater: H160,
			new_updater: H160,
		},
		FakeSlashed {
			reporter: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		InitializationError,
	}

	// No exposed methods. The updater manager can only be called through the
	// Home pallet.
	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> Pallet<T> {
		pub fn get_updater() -> H160 {
			Updater::<T>::get()
		}

		pub fn set_updater(new_updater: H160) -> DispatchResult {
			let old_updater = Updater::<T>::get();
			Updater::<T>::put(new_updater);

			Self::deposit_event(Event::<T>::NewUpdater {
				old_updater,
				new_updater,
			});

			Ok(())
		}

		pub fn slash_updater(reporter: T::AccountId) {
			Self::deposit_event(Event::<T>::FakeSlashed { reporter });
		}
	}
}
