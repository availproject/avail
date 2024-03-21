#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

use avail_core::{
	AppId, BlockLengthColumns, BlockLengthRows, BLOCK_CHUNK_SIZE, DA_DISPATCH_RATIO,
	NORMAL_DISPATCH_RATIO,
};
use frame_support::{dispatch::DispatchClass, traits::Get, weights::Weight};
use frame_system::{limits::BlockLength, pallet::DynamicBlockLength};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_arithmetic::traits::{CheckedAdd, One, SaturatedConversion};
use sp_core::H256;
use sp_io::{hashing::blake2_256, transaction_index};
use sp_runtime::Perbill;
use sp_std::{mem::replace, vec, vec::Vec};

pub use crate::{pallet::*, weights::WeightInfo};

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod extensions;
#[cfg(feature = "std")]
pub mod mock;
#[cfg(test)]
mod tests;
pub use extensions::check_app_id::CheckAppId;
pub mod kate;
pub mod weights;

pub const LOG_TARGET: &str = "runtime::da_control";

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, DefaultNoBound};
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

	/// Default implementations of [`DefaultConfig`], which can be used to implement [`Config`].
	pub mod config_preludes {
		use super::*;
		use frame_support::derive_impl;

		/// Provides a viable default config that can be used with
		/// [`derive_impl`](`frame_support::derive_impl`) to derive a testing pallet config
		/// based on this one.
		pub struct TestDefaultConfig;

		#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig, no_aggregated_types)]
		impl frame_system::DefaultConfig for TestDefaultConfig {}

		#[frame_support::register_default_impl(TestDefaultConfig)]
		impl DefaultConfig for TestDefaultConfig {
			type BlockLenProposalId = u32;
			type MaxAppDataLength = ();
			type MaxAppKeyLength = ();
			type MaxBlockCols = ();
			type MaxBlockRows = ();
			type MinBlockCols = ();
			type MinBlockRows = ();
			type WeightInfo = ();
			#[inject_runtime_type]
			type RuntimeEvent = ();
		}
	}

	#[pallet::config(with_default)]
	pub trait Config: frame_system::Config {
		/// Pallet Event
		#[pallet::no_default_bounds]
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

			// SAFETY: `data.len()` is always less than `u32::MAX` because it is bounded by
			// `BoundedVec`
			let len = data.len() as u32;

			// Index Tx in DB block.
			let data_hash = blake2_256(&data);
			let extrinsic_index =
				<frame_system::Pallet<T>>::extrinsic_index().ok_or(Error::<T>::BadContext)?;
			transaction_index::index(extrinsic_index, len, data_hash);

			Self::deposit_event(Event::DataSubmitted {
				who,
				data_hash: H256(data_hash),
			});

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
				BlockLength::with_normal_ratio(rows, cols, BLOCK_CHUNK_SIZE, DA_DISPATCH_RATIO)
					.map_err(|_| Error::<T>::BlockDimensionsOutOfBounds)?;

			DynamicBlockLength::<T>::put(block_length);

			Self::deposit_event(Event::BlockLengthProposalSubmitted { rows, cols });

			Ok(().into())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::set_application_key())]
		pub fn set_application_key(
			origin: OriginFor<T>,
			old_key: AppKeyFor<T>,
			new_key: AppKeyFor<T>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			ensure!(!old_key.is_empty(), Error::<T>::AppKeyCannotBeEmpty);
			ensure!(!new_key.is_empty(), Error::<T>::AppKeyCannotBeEmpty);

			// Check for uniqueness
			let maybe_existing_key = AppKeys::<T>::get(&new_key);
			ensure!(
				maybe_existing_key.is_none(),
				Error::<T>::AppKeyAlreadyExists
			);

			// Get app info for the given key
			let app_key_info = AppKeys::<T>::get(&old_key).ok_or(Error::<T>::UnknownAppKey)?;

			// Remove the old key
			AppKeys::<T>::remove(&old_key);

			// Insert the app info under the new key
			AppKeys::<T>::insert(&new_key, app_key_info);

			Self::deposit_event(Event::ApplicationKeySet { old_key, new_key });

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
			data_hash: H256,
		},
		BlockLengthProposalSubmitted {
			rows: BlockLengthRows,
			cols: BlockLengthColumns,
		},
		ApplicationKeySet {
			old_key: AppKeyFor<T>,
			new_key: AppKeyFor<T>,
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
		/// Submit data call outside of block execution context.
		BadContext,
		/// App info was not found for the given App key
		UnknownAppKey,
	}

	#[pallet::genesis_config]
	#[derive(DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		#[allow(clippy::type_complexity)]
		pub app_keys: Vec<(Vec<u8>, (T::AccountId, u32))>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			// Ensure app ids are unique.
			let mut app_keys = self.app_keys.clone();
			app_keys.sort_by(|a, b| a.1 .1.cmp(&b.1 .1));

			// With a sorted array, we can just use the id of the last element :)
			let last_key = app_keys.last();
			let last_id = last_key.map(|x| x.1 .1).unwrap_or(0);
			let next_app_id = AppId(last_id.saturating_add(1));
			NextAppId::<T>::put::<AppId>(next_app_id);

			// Insert app keys. It verifies the length limitation of each key.
			for (key, (owner, id)) in app_keys {
				let key = AppKeyFor::<T>::try_from(key)
					.expect("DA Control Genesis contains invalid keys");
				let value = AppKeyInfo {
					id: AppId(id),
					owner,
				};
				AppKeys::<T>::insert(key, value);
			}
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
	/// We check the current normal ratio weight, if it's too high, it means we won't reduce the block size
	pub fn is_block_weight_acceptable() -> bool {
		let current_weight = <frame_system::Pallet<T>>::block_weight();
		let current_normal_weight: &Weight = current_weight.get(DispatchClass::Normal);
		let acceptable_limit: Weight =
			T::WeightInfo::submit_block_length_proposal().saturating_mul(5);
		current_normal_weight.all_gte(acceptable_limit)
	}
}

pub mod weight_helper {

	use super::*;

	/// Weight for `dataAvailability::submit_data`.
	pub fn submit_data<T: Config>(data_len: usize) -> (Weight, DispatchClass) {
		/* Compute regular substrate weight. */
		let data_len: u32 = data_len.saturated_into();
		let basic_weight = T::WeightInfo::submit_data(data_len);
		let data_root_weight = T::WeightInfo::data_root(data_len);
		let regular_weight = basic_weight.saturating_add(data_root_weight);

		/* Compute weight based on size taken in the matrix and hence computation. */
		// We get the current settings for matrix columns, rows and chunk_size.
		let current_block_dimension = DynamicBlockLength::<T>::get();
		let cols: u32 = current_block_dimension.cols.0;
		let rows: u32 = current_block_dimension.rows.0;
		let chunk_size: u32 = 32;

		// We compute the matrix size and multiply with the DA dispatch ratio.
		let max_matrix_size = cols.saturating_mul(rows).saturating_mul(chunk_size);
		let max_matrix_size_normal_ratio = DA_DISPATCH_RATIO * max_matrix_size;

		// We get the current maximum weight in a block and multiply with normal dispatch ratio.
		let block_weights = <T as frame_system::Config>::BlockWeights::get();
		let max_weight_normal_ratio: u64 =
			NORMAL_DISPATCH_RATIO * block_weights.max_block.ref_time();

		// We compute the ratio of data length / max matrix size  and multiply with the maximum weight.
		let data_matrix_ratio = Perbill::from_rational(data_len, max_matrix_size_normal_ratio);
		let matrix_based_ref_time = data_matrix_ratio * max_weight_normal_ratio;

		// We create a weight object from the matrix based weight.
		let matrix_based_weight =
			Weight::from_parts(matrix_based_ref_time, regular_weight.proof_size());

		// We return the biggest value between the regular weight and matrix based weight.
		// I cannot think of a case where regular weight > matrix based weight.
		(
			matrix_based_weight.max(regular_weight),
			DispatchClass::Normal,
		)
	}
}

impl<Acc> AppKeyInfo<Acc>
where
	Acc: PartialEq,
{
	pub fn new(owner: Acc, id: AppId) -> Self {
		Self { owner, id }
	}
}
