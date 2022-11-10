#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

mod message;
pub mod weights;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use da_primitives::traits::ExtendedHeader;
	use frame_support::{pallet_prelude::*, sp_runtime::traits::Header};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use nomad_core::TypedMessage;
	use nomad_home::Pallet as Home;
	use primitive_types::H256;

	use super::weights::WeightInfo;
	use crate::message::{DABridgeMessages, DataRootMessage};

	#[pallet::config]
	pub trait Config: frame_system::Config + nomad_home::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[pallet::constant]
		type DABridgePalletId: Get<H256>;

		/// Weights for this pallet.
		type WeightInfo: WeightInfo;
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
		#[pallet::weight(<T as Config>::WeightInfo::try_dispatch_data_root())]
		pub fn try_dispatch_data_root(
			origin: OriginFor<T>,
			#[pallet::compact] destination_domain: u32,
			recipient_address: H256,
			header: T::Header,
		) -> DispatchResult {
			ensure_signed(origin)?;
			Self::ensure_valid_header(&header)?;
			Self::do_dispatch_data_root(destination_domain, recipient_address, header)
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
			header: T::Header,
		) -> DispatchResult {
			let block_number = *header.number();
			let data_root = header.extension().data_root();

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

		/// Ensure a given header's hash has been recorded in the block hash
		/// mapping.
		fn ensure_valid_header(header: &T::Header) -> Result<(), DispatchError> {
			// Ensure header's block number is in the mapping
			let number = header.number();
			let stored_hash = frame_system::Pallet::<T>::block_hash(number);

			// Ensure header's hash matches that in the block number to hash
			// mapping
			let hash = header.hash();
			ensure!(
				stored_hash == hash,
				Error::<T>::HashOfBlockNotMatchBlockNumber,
			);

			Ok(())
		}
	}
}
