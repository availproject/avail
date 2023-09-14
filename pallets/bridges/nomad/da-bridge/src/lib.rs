#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

mod message;
pub mod weights;

pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use avail_core::traits::ExtendedHeader;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use nomad_core::TypedMessage;
	use nomad_home::Pallet as Home;
	use sp_core::{bounded::BoundedVec, Get, H256};
	use sp_runtime::{traits::Header as _, SaturatedConversion};
	use sp_std::boxed::Box;

	use super::weights::WeightInfo;
	use crate::message::{DABridgeMessages, DataRootMessage};

	/// Default implementations of [`DefaultConfig`], which can be used to implement [`Config`].
	pub mod config_preludes {
		use super::DefaultConfig;

		/// Provides a viable default config that can be used with
		/// [`derive_impl`](`frame_support::derive_impl`) to derive a testing pallet config
		/// based on this one.
		pub struct TestDefaultConfig;

		#[frame_support::register_default_impl(TestDefaultConfig)]
		impl DefaultConfig for TestDefaultConfig {
			type DABridgePalletId = ();
			type WeightInfo = ();
		}
	}

	#[pallet::config(with_default)]
	pub trait Config: frame_system::Config + nomad_home::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type DABridgePalletId: Get<H256>;

		/// Weights for this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		DataRootDispatched {
			destination_domain: u32,
			recipient_address: H256,
			block_number: BlockNumberFor<T>,
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
	{
		/// Dispatch a data root message to the home if the header is valid.
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::try_dispatch_data_root())]
		pub fn try_dispatch_data_root(
			origin: OriginFor<T>,
			#[pallet::compact] destination_domain: u32,
			recipient_address: H256,
			header: Box<DaHeaderFor<T>>,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;
			Self::ensure_valid_header(&header)?;
			Self::do_dispatch_data_root(destination_domain, recipient_address, &header)
		}
	}

	impl<T: Config> Pallet<T>
	where
		[u8; 32]: From<T::AccountId>,
		H256: From<T::Hash>,
	{
		/// Dispatch a data root message for a valid header.
		fn do_dispatch_data_root(
			destination_domain: u32,
			recipient_address: H256,
			header: &DaHeaderFor<T>,
		) -> DispatchResultWithPostInfo {
			// Safety: Even if a BlockNumber type is larger than u32, it won't pose any issues for the next 2000+ years
			let block_number: u32 = (*header.number()).saturated_into();
			let data_root = header.extension().data_root();

			let message: DABridgeMessages = DataRootMessage {
				block_number,
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
				block_number: block_number.into(),
				data_root,
			});

			Ok(().into())
		}

		/// Ensure a given header's hash has been recorded in the block hash
		/// mapping.
		fn ensure_valid_header(header: &DaHeaderFor<T>) -> DispatchResultWithPostInfo {
			// Ensure header's block number is in the mapping
			let number: u32 = (*header.number()).saturated_into();
			let stored_hash =
				frame_system::Pallet::<T>::block_hash::<BlockNumberFor<T>>(number.into());

			// Ensure header's hash matches that in the block number to hash
			// mapping
			let hash = header.hash();
			ensure!(
				stored_hash == hash,
				Error::<T>::HashOfBlockNotMatchBlockNumber,
			);

			Ok(().into())
		}
	}
}
