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
	use frame_support::pallet_prelude::*;
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use nomad_core::TypedMessage;
	use nomad_home::Pallet as Home;
	use primitive_types::H256;

	use crate::message::{DABridgeMessages, DataRootMessage};

	#[pallet::config]
	pub trait Config: frame_system::Config + nomad_home::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[pallet::constant]
		type DABridgePalletId: Get<H256>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

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

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		DataRootDispatched {
			destination_domain: u32,
			recipient_address: H256,
			block_number: T::BlockNumber,
			data_root: H256,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		InitializationError,
		HashOfBlockNotMatchBlockNumber,
		DABridgeMessageExceedsMaxMessageSize,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		[u8; 32]: From<T::AccountId>,
		H256: From<T::Hash>,
		u32: From<T::BlockNumber>,
	{
		/// Dispatch a data root message to the home if the header is valid.
		#[pallet::weight(100)]
		pub fn try_dispatch_data_root(
			origin: OriginFor<T>,
			#[pallet::compact] destination_domain: u32,
			recipient_address: H256,
			#[pallet::compact] block_number: T::BlockNumber,
			data_root: H256,
		) -> DispatchResult {
			ensure_signed(origin)?;
			Self::ensure_valid_block_number(block_number)?;
			Self::do_dispatch_data_root(
				destination_domain,
				recipient_address,
				block_number,
				data_root,
			)
		}
	}

	impl<T: Config> Pallet<T>
	where
		[u8; 32]: From<T::AccountId>,
		H256: From<T::Hash>,
		u32: From<T::BlockNumber>,
	{
		/// Dispatch a data root message for a valid header.
		fn do_dispatch_data_root(
			destination_domain: u32,
			recipient_address: H256,
			block_number: T::BlockNumber,
			data_root: H256,
		) -> DispatchResult {
			let message: DABridgeMessages = DataRootMessage {
				block_number: block_number.into(),
				data_root,
			}
			.into();

			let body: BoundedVec<u8, T::MaxMessageBodyBytes> = message
				.encode()
				.try_into()
				.map_err(|_| Error::<T>::DABridgeMessageExceedsMaxMessageSize)?;

			Home::<T>::do_dispatch(
				T::DABridgePalletId::get(),
				destination_domain,
				recipient_address,
				body,
			)?;

			Self::deposit_event(Event::<T>::DataRootDispatched {
				destination_domain,
				recipient_address,
				block_number,
				data_root,
			});

			Ok(())
		}

		fn ensure_valid_block_number(number: T::BlockNumber) -> Result<(), DispatchError> {
			// Ensure header's block number is in the mapping
			let hash = frame_system::Pallet::<T>::block_hash(number);
			ensure!(
				hash != Default::default(),
				Error::<T>::HashOfBlockNotMatchBlockNumber,
			);

			Ok(())
		}
	}
}
