#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

use da_primitives::{
	asdr::AppId, BlockLengthColumns, BlockLengthRows, BLOCK_CHUNK_SIZE, NORMAL_DISPATCH_RATIO,
};
use frame_support::{dispatch::DispatchClass, weights::Weight};
use frame_system::{limits::BlockLength, pallet::DynamicBlockLength};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_arithmetic::traits::{CheckedAdd, One, SaturatedConversion};
use sp_std::mem::replace;

pub use crate::{pallet::*, weights::WeightInfo};

#[cfg(feature = "std")]
pub mod mock;
#[cfg(test)]
mod tests;

mod benchmarking;
mod extensions;
pub use extensions::check_app_id::CheckAppId;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use super::*;

	pub type AppKeyFor<T> = BoundedVec<u8, <T as Config>::MaxAppKeyLength>;
	pub type AppDataFor<T> = BoundedVec<u8, <T as Config>::MaxAppDataLength>;

	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[derive(Clone, Encode, Decode, TypeInfo, PartialEq, RuntimeDebug, MaxEncodedLen)]
	pub struct AppKeyInfo<Acc: PartialEq> {
		/// Owner of the key
		pub owner: Acc,
		/// Application ID associated.
		pub id: AppId,
	}

	pub type AppKeyInfoFor<T> = AppKeyInfo<<T as frame_system::Config>::AccountId>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Pallet Event
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Block length proposal Id.
		type BlockLenProposalId: Parameter + Default + One + CheckedAdd + MaxEncodedLen;

		/// The max length of application key.
		#[pallet::constant]
		type MaxAppKeyLength: Get<u32>;

		/// The max length of app data.
		#[pallet::constant]
		type MaxAppDataLength: Get<u32>;

		/// Minimum number of rows in a block.
		#[pallet::constant]
		type MinBlockRows: Get<BlockLengthRows>;

		/// Maximum number of rows in a block.
		#[pallet::constant]
		type MaxBlockRows: Get<BlockLengthRows>;

		/// Minimum number of cols in a block.
		#[pallet::constant]
		type MinBlockCols: Get<BlockLengthColumns>;

		/// Maximum number of cols in a block.
		#[pallet::constant]
		type MaxBlockCols: Get<BlockLengthColumns>;

		/// Weights for this pallet.
		type WeightInfo: weights::WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	/// Last application ID
	#[pallet::storage]
	#[pallet::getter(fn peek_next_application_id)]
	pub type NextAppId<T: Config> = StorageValue<_, AppId, ValueQuery>;

	/// Store all application keys.
	#[pallet::storage]
	#[pallet::getter(fn application_key)]
	pub type AppKeys<T: Config> = StorageMap<_, Blake2_128Concat, AppKeyFor<T>, AppKeyInfoFor<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates an application key if `key` does not exist yet.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_application_key())]
		pub fn create_application_key(
			origin: OriginFor<T>,
			key: AppKeyFor<T>,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			ensure!(!key.is_empty(), Error::<T>::AppKeyCannotBeEmpty);
			let id = AppKeys::<T>::try_mutate(&key, |key_info| -> Result<AppId, Error<T>> {
				ensure!(key_info.is_none(), Error::<T>::AppKeyAlreadyExists);

				let id = Self::next_application_id()?;
				*key_info = Some(AppKeyInfo {
					id,
					owner: owner.clone(),
				});

				Ok(id)
			})?;

			Self::deposit_event(Event::ApplicationKeyCreated { key, owner, id });
			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(weight_helper::submit_data::<T>(data.len()))]
		pub fn submit_data(
			origin: OriginFor<T>,
			data: AppDataFor<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			ensure!(!data.is_empty(), Error::<T>::DataCannotBeEmpty);
			Self::deposit_event(Event::DataSubmitted { who, data });

			Ok(().into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::submit_block_length_proposal())]
		pub fn submit_block_length_proposal(
			origin: OriginFor<T>,
			rows: u32,
			cols: u32,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			let rows = BlockLengthRows(rows);
			let cols = BlockLengthColumns(cols);

			ensure!(
				rows <= T::MaxBlockRows::get() && cols <= T::MaxBlockCols::get(),
				Error::<T>::BlockDimensionsOutOfBounds
			);
			ensure!(
				rows >= T::MinBlockRows::get() && cols >= T::MinBlockCols::get(),
				Error::<T>::BlockDimensionsTooSmall
			);

			let current_block_dimension = DynamicBlockLength::<T>::get();
			let is_increase =
				rows >= current_block_dimension.rows && cols >= current_block_dimension.cols;
			ensure!(
				is_increase || Self::is_block_weight_acceptable(),
				Error::<T>::InvalidBlockWeightReduction
			);

			let block_length =
				BlockLength::with_normal_ratio(rows, cols, BLOCK_CHUNK_SIZE, NORMAL_DISPATCH_RATIO)
					.map_err(|_| Error::<T>::BlockDimensionsOutOfBounds)?;

			DynamicBlockLength::<T>::put(block_length);

			Self::deposit_event(Event::BlockLengthProposalSubmitted { rows, cols });

			Ok(().into())
		}
	}

	/// Event for the pallet.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new application key was created.
		ApplicationKeyCreated {
			key: AppKeyFor<T>,
			owner: T::AccountId,
			id: AppId,
		},
		DataSubmitted {
			who: T::AccountId,
			data: AppDataFor<T>,
		},
		BlockLengthProposalSubmitted {
			rows: BlockLengthRows,
			cols: BlockLengthColumns,
		},
	}

	/// Error for the System pallet
	#[pallet::error]
	pub enum Error<T> {
		/// The application key already exists.
		AppKeyAlreadyExists,
		/// The application key is an empty string.
		AppKeyCannotBeEmpty,
		/// The last application ID overflowed.
		LastAppIdOverflowed,
		/// The submitted data is empty.
		DataCannotBeEmpty,
		/// The last block length proposal Id overflowed.
		LastBlockLenProposalIdOverflowed,
		/// The proposed block dimensions are out of bounds.
		BlockDimensionsOutOfBounds,
		/// The proposed block dimensions are too small.
		BlockDimensionsTooSmall,
		/// The request to reduce block dimensions was made in a non-empty block
		InvalidBlockWeightReduction,
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub app_keys: Vec<(Vec<u8>, AppKeyInfoFor<T>)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self { Self { app_keys: vec![] } }
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			// Ensure app ids are unique.
			let mut ids = self
				.app_keys
				.iter()
				.map(|(_, info)| info.id)
				.collect::<Vec<_>>();
			ids.sort_unstable();
			ids.dedup();
			if ids.len() != self.app_keys.len() {
				panic!("DA Control Genesis contains duplicated application ID");
			}

			// Insert app keys. It verifies the length limitation of each key.
			self.app_keys
				.iter()
				.cloned()
				.try_for_each(|(key, info)| -> Result<(), Vec<u8>> {
					let key = AppKeyFor::<T>::try_from(key)?;
					AppKeys::<T>::insert(key, info);
					Ok(())
				})
				.expect("DA Control Genesis contains invalid keys");

			// Last app Id will be the greater one.
			let last_id = ids
				.iter()
				.max()
				.cloned()
				.map(Into::into)
				.unwrap_or(0u32)
				.checked_add(1u32)
				.expect("DA Control Genesis overflows the last application id");
			NextAppId::<T>::put::<AppId>(last_id.into());
		}
	}

	#[cfg(feature = "std")]
	impl<T: Config> GenesisConfig<T> {
		/// Direct implementation of `GenesisBuild::build_storage`.
		///
		/// Kept in order not to break dependency.
		pub fn build_storage(&self) -> Result<sp_runtime::Storage, String> {
			<Self as GenesisBuild<T>>::build_storage(self)
		}

		/// Direct implementation of `GenesisBuild::assimilate_storage`.
		///
		/// Kept in order not to break dependency.
		pub fn assimilate_storage(&self, storage: &mut sp_runtime::Storage) -> Result<(), String> {
			<Self as GenesisBuild<T>>::assimilate_storage(self, storage)
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Returns the latest available application ID and increases it.
	pub fn next_application_id() -> Result<AppId, Error<T>> {
		NextAppId::<T>::try_mutate(|id| {
			let new_id = AppId(id.0.checked_add(1).ok_or(Error::<T>::LastAppIdOverflowed)?);
			Ok(replace(id, new_id))
		})
	}

	/// Check if the block weight is acceptable to execute the extrinsic
	/// We check the current normal ratio weight of the block and compare it with the extrinsic weight
	pub fn is_block_weight_acceptable() -> bool {
		let current_weight = <frame_system::Pallet<T>>::block_weight();
		let current_normal_weight = current_weight.get(DispatchClass::Normal);
		let acceptable_limit = T::WeightInfo::submit_block_length_proposal().saturating_mul(2);

		current_normal_weight.all_lte(acceptable_limit)
	}
}

mod weight_helper {

	use super::*;

	/// Weight for `dataAvailability::submit_data`.
	pub(crate) fn submit_data<T: Config>(data_len: usize) -> (Weight, DispatchClass) {
		let data_len: u32 = data_len.saturated_into();
		let basic_weight = T::WeightInfo::submit_data(data_len);
		let data_root_weight = T::WeightInfo::data_root(data_len);
		let total_weight = basic_weight.saturating_add(data_root_weight);
		(total_weight, DispatchClass::Normal)
	}
}