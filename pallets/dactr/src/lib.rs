#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

use avail_core::{
	currency::Balance, AppId, BlockLengthColumns, BlockLengthRows, BLOCK_CHUNK_SIZE,
	DA_DISPATCH_RATIO, NORMAL_DISPATCH_RATIO,
};
use codec::{Compact, CompactLen as _, Encode};
use frame_support::ensure;
use frame_support::traits::{ValidatorSet, ValidatorSetWithIdentification};
use frame_support::weights::constants::ExtrinsicBaseWeight;
use frame_support::{
	dispatch::DispatchClass,
	traits::{Currency, Get, ReservableCurrency},
	weights::Weight,
};
use frame_system::{limits::BlockLength, pallet::DynamicBlockLength};
use sp_arithmetic::traits::{CheckedAdd, One, SaturatedConversion};
use sp_core::H256;
use sp_io::hashing::keccak_256;
use sp_runtime::traits::Convert;
use sp_runtime::Perbill;
use sp_std::{mem::replace, vec, vec::Vec};

pub use crate::{pallet::*, weights::WeightInfo};

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod extensions;
#[cfg(feature = "std")]
pub mod mock;
#[cfg(test)]
mod tests;
use frame_support::dispatch::{DispatchFeeModifier, DispatchResult};
pub mod types;
pub mod weights;
pub use types::*;

pub const LOG_TARGET: &str = "runtime::da_control";
pub const DA_DISPATCH_RATIO_PERBILL: Perbill = Perbill::from_percent(DA_DISPATCH_RATIO as u32);
pub const NORMAL_DISPATCH_RATIO_PERBILL: Perbill =
	Perbill::from_percent(NORMAL_DISPATCH_RATIO as u32);

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

/// A type for representing the validator id in a session.
pub type ValidatorId<T> = <<T as Config>::ValidatorSet as ValidatorSet<
	<T as frame_system::Config>::AccountId,
>>::ValidatorId;

/// A tuple of (ValidatorId, Identification) where `Identification` is the full identification of
/// `ValidatorId`.
pub type IdentificationTuple<T> = (
	ValidatorId<T>,
	<<T as Config>::ValidatorSet as ValidatorSetWithIdentification<
		<T as frame_system::Config>::AccountId,
	>>::Identification,
);

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, DefaultNoBound};
	use frame_system::pallet_prelude::*;
	use sp_staking::{offence::ReportOffence, SessionIndex};

	use super::*;

	/// Default implementations of [`DefaultConfig`], which can be used to implement [`Config`].
	pub mod config_preludes {
		use super::*;
		use frame_support::derive_impl;
		use frame_support::parameter_types;

		/// Provides a viable default config that can be used with
		/// [`derive_impl`](`frame_support::derive_impl`) to derive a testing pallet config
		/// based on this one.
		pub struct TestDefaultConfig;

		#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig, no_aggregated_types)]
		impl frame_system::DefaultConfig for TestDefaultConfig {}

		parameter_types! {
			pub const MinBlockRows: BlockLengthRows = BlockLengthRows(32);
			pub const MaxBlockRows: BlockLengthRows = BlockLengthRows(4096);
			pub const MinBlockCols: BlockLengthColumns = BlockLengthColumns(32);
			pub const MaxBlockCols: BlockLengthColumns = BlockLengthColumns(1024);
			pub const MaxAppKeyLength: u32 = 32;
			pub const MaxAppDataLength: u32 = 1_048_576; // 1 Mb
			pub const MaxVouchesPerRecord: u32 = 256; // Need to be greater than vouch threshold
			pub const BlobVouchFeeReserve: Balance = 0;
		}

		#[frame_support::register_default_impl(TestDefaultConfig)]
		impl DefaultConfig for TestDefaultConfig {
			type BlockLenProposalId = u32;
			type MaxAppDataLength = MaxAppDataLength;
			type MaxAppKeyLength = MaxAppKeyLength;
			type MaxBlockCols = MaxBlockCols;
			type MaxBlockRows = MaxBlockRows;
			type MinBlockCols = MinBlockCols;
			type MinBlockRows = MinBlockRows;
			type MaxVouchesPerRecord = MaxVouchesPerRecord;
			type SessionDataProvider = ();
			type BlobVouchFeeReserve = BlobVouchFeeReserve;
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

		/// Currency mechanism used for deposits.
		#[pallet::no_default]
		type Currency: ReservableCurrency<Self::AccountId>;

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

		/// Maximum number of validators vouching for a blob validator accusation.
		#[pallet::constant]
		type MaxVouchesPerRecord: Get<u32>;

		/// The amount reserve to add an offence report voucher.
		#[pallet::constant]
		type BlobVouchFeeReserve: Get<Balance>;

		/// A provider that gives the information about era and validators.
		type SessionDataProvider: SessionDataProvider<Self::AccountId>;

		/// A type for retrieving the validators in a session.
		#[pallet::no_default]
		type ValidatorSet: ValidatorSetWithIdentification<Self::AccountId>;

		/// Hook into the offences pipeline.
		#[pallet::no_default]
		type ReportOffence: sp_staking::offence::ReportOffence<
			Self::AccountId,
			IdentificationTuple<Self>,
			BlobOffence<IdentificationTuple<Self>>,
		>;

		/// Weights for this pallet.
		type WeightInfo: weights::WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Last application ID
	#[pallet::storage]
	#[pallet::getter(fn peek_next_application_id)]
	pub type NextAppId<T: Config> = StorageValue<_, AppId, ValueQuery>;

	/// Store all application keys.
	#[pallet::storage]
	#[pallet::getter(fn application_key)]
	pub type AppKeys<T: Config> = StorageMap<_, Blake2_128Concat, AppKeyFor<T>, AppKeyInfoFor<T>>;

	/// Store data fee modifier for submit_data call.
	#[pallet::storage]
	pub type SubmitDataFeeModifier<T: Config> = StorageValue<_, DispatchFeeModifier, ValueQuery>;

	/// Store data fee modifier for submit_blob_metadata call.
	#[pallet::storage]
	pub type SubmitBlobMetadataFeeModifier<T: Config> =
		StorageValue<_, DispatchFeeModifier, ValueQuery>;

	/// Store the runtime parameter for the blob module
	#[pallet::storage]
	#[pallet::getter(fn blob_runtime_parameters)]
	pub type BlobRuntimeParams<T: Config> = StorageValue<_, BlobRuntimeParameters, ValueQuery>;

	/// Era wide blob offences
	#[pallet::storage]
	#[pallet::getter(fn blob_offence_records)]
	pub type BlobOffenceRecords<T: Config> =
		StorageMap<_, Blake2_128Concat, OffenceKey, OffenceRecord<T>>;

	/// Last seen active era
	#[pallet::storage]
	#[pallet::getter(fn last_seen_era)]
	pub type LastSeenSession<T: Config> = StorageValue<_, SessionIndex, OptionQuery>;

	/// Last seen validator set
	#[pallet::storage]
	#[pallet::getter(fn last_seen_validator_set)]
	pub type LastSeenValidatorSet<T: Config> = StorageValue<_, Vec<ValidatorId<T>>, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
			let mut weight = Weight::zero();
			let session_index = T::ValidatorSet::session_index();
			let previous_session_index = LastSeenSession::<T>::get().unwrap_or(0);
			weight = weight.saturating_add(<T as frame_system::Config>::DbWeight::get().reads(2));

			if session_index > previous_session_index {
				let mut previous_validator_set = LastSeenValidatorSet::<T>::get();
				if previous_validator_set.is_empty() {
					previous_validator_set = T::ValidatorSet::validators();
				}
				let blob_runtime_params = BlobRuntimeParams::<T>::get();
				let vouch_threshold = blob_runtime_params.vouch_threshold;
				let validator_length = previous_validator_set.len() as u32;
				let threshold = vouch_threshold.min(validator_length);
				weight =
					weight.saturating_add(<T as frame_system::Config>::DbWeight::get().reads(2));

				for (key, record) in BlobOffenceRecords::<T>::iter() {
					weight = weight
						.saturating_add(<T as frame_system::Config>::DbWeight::get().reads(1));
					if !key.is_valid() {
						continue;
					}

					let vouch_fee: BalanceOf<T> = T::BlobVouchFeeReserve::get().saturated_into();
					weight = weight
						.saturating_add(<T as frame_system::Config>::DbWeight::get().reads(1));

					if !record.has_reached_threshold(validator_length, threshold) {
						// Slash vouch fee
						for voucher in record.vouches.iter() {
							let validator =
								T::AccountId::decode(&mut &voucher.validator.encode()[..]).ok();
							if let Some(validator) = validator {
								Self::slash_vouch_fee(&validator, Some(vouch_fee));
								weight = weight.saturating_add(
									<T as frame_system::Config>::DbWeight::get().writes(1),
								);
							}
						}
						continue;
					}

					let reporters: Vec<T::AccountId> = record
						.vouches
						.iter()
						.filter_map(|v| T::AccountId::decode(&mut &v.validator.encode()[..]).ok())
						.collect();

					// Return vouch fee
					for validator in reporters.clone().iter() {
						Self::return_vouch_fee(validator, Some(vouch_fee));
						weight = weight
							.saturating_add(<T as frame_system::Config>::DbWeight::get().writes(1));
					}

					let offender = match record.kind {
						BlobOffenceKind::MissingValidatorForBlob => record
							.missing_validator
							.and_then(|a| T::AccountId::decode(&mut &a.encode()[..]).ok()),
						_ => record
							.get_block_author()
							.and_then(|a| T::AccountId::decode(&mut &a.encode()[..]).ok()),
					};

					let Some(offender) = offender else {
						continue;
					};

					let offender =
						<T::ValidatorSet as ValidatorSet<T::AccountId>>::ValidatorIdOf::convert(
							offender,
						)
						.and_then(|validator_id| {
							<T::ValidatorSet as ValidatorSetWithIdentification<
								T::AccountId,
							>>::IdentificationOf::convert(validator_id.clone())
							.map(|full_id| (validator_id, full_id))
						});
					weight = weight
						.saturating_add(<T as frame_system::Config>::DbWeight::get().reads(2));

					let Some(offender) = offender else {
						continue;
					};

					let offence = BlobOffence {
						kind: record.kind.clone(),
						key: key.clone(),
						session_index: previous_session_index,
						validator_set_count: validator_length,
						offenders: vec![offender],
					};

					match T::ReportOffence::report_offence(reporters, offence) {
						Ok(()) => {
							log::info!(target: crate::LOG_TARGET, "Reported blob offence: {:?}", key);
						},
						Err(e) => {
							log::warn!(target: crate::LOG_TARGET, "Failed to report offence {:?}: {:?}", key, e);
						},
					}
				}

				LastSeenSession::<T>::put(session_index);
				LastSeenValidatorSet::<T>::put(T::ValidatorSet::validators());
				weight =
					weight.saturating_add(<T as frame_system::Config>::DbWeight::get().writes(2));

				let cleared_count = BlobOffenceRecords::<T>::clear(u32::max_value(), None);
				weight = weight.saturating_add(
					<T as frame_system::Config>::DbWeight::get()
						.reads_writes(cleared_count.loops.into(), cleared_count.unique.into()),
				);
			}

			weight
		}
	}
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
		#[pallet::weight((
			weight_helper::submit_data::<T>(data.len()),
			DispatchClass::Normal,
			SubmitDataFeeModifier::<T>::get()
		))]
		pub fn submit_data(
			origin: OriginFor<T>,
			app_id: AppId,
			data: AppDataFor<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			ensure!(
				app_id < Self::peek_next_application_id(),
				Error::<T>::InvalidAppId
			);
			ensure!(!data.is_empty(), Error::<T>::DataCannotBeEmpty);
			ensure!(
				!BlobRuntimeParams::<T>::get().disable_old_da_submission,
				Error::<T>::OldDaSubmissionDisabled
			);

			let data_hash = keccak_256(&data);
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

			// Check if rows and cols are powers of 2
			// Check if `rows` or `cols` are a power of 2: they must be nonzero and have no bits in common with `(rows or cols) - 1`.
			ensure!(
				rows.0 != 0 && (rows.0 & (rows.0 - 1)) == 0,
				Error::<T>::NotPowerOfTwo
			);
			ensure!(
				cols.0 != 0 && (cols.0 & (cols.0 - 1)) == 0,
				Error::<T>::NotPowerOfTwo
			);

			let current_block_dimension = DynamicBlockLength::<T>::get();
			let is_increase =
				rows >= current_block_dimension.rows && cols >= current_block_dimension.cols;
			ensure!(
				is_increase || Self::is_block_weight_acceptable(),
				Error::<T>::InvalidBlockWeightReduction
			);

			let block_length = BlockLength::with_normal_ratio(
				rows,
				cols,
				BLOCK_CHUNK_SIZE,
				DA_DISPATCH_RATIO_PERBILL,
			)
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

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::set_submit_data_fee_modifier())]
		pub fn set_submit_data_fee_modifier(
			origin: OriginFor<T>,
			modifier: DispatchFeeModifier,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			SubmitDataFeeModifier::<T>::put(modifier);

			Self::deposit_event(Event::SubmitDataFeeModifierSet { value: modifier });

			Ok(().into())
		}

		#[pallet::call_index(5)]
		#[pallet::weight((
			weight_helper::submit_blob_metadata::<T>(*size),
			DispatchClass::Normal,
			SubmitBlobMetadataFeeModifier::<T>::get(),
		))]
		pub fn submit_blob_metadata(
			origin: OriginFor<T>,
			app_id: AppId,
			blob_hash: H256,
			size: u64,
			commitment: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			ensure!(
				app_id < Self::peek_next_application_id(),
				Error::<T>::InvalidAppId
			);
			ensure!(size > 0, Error::<T>::DataCannotBeEmpty);
			ensure!(!commitment.is_empty(), Error::<T>::CommitmentCannotBeEmpty);
			ensure!(blob_hash != H256::zero(), Error::<T>::DataCannotBeEmpty);

			Self::deposit_event(Event::SubmitBlobMetadataRequest { who, blob_hash });

			Ok(().into())
		}

		#[pallet::call_index(6)]
		#[pallet::weight((
			T::WeightInfo::submit_blob_txs_summary(*nb_blobs),
			DispatchClass::Mandatory
		))]
		pub fn submit_blob_txs_summary(
			origin: OriginFor<T>,
			_total_blob_size: u64,
			#[allow(unused_variables)] nb_blobs: u32,
			_blob_txs_summary: Vec<BlobTxSummaryRuntime>,
		) -> DispatchResult {
			ensure_none(origin)?;
			// All the checks are done client side by validators

			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::set_blob_runtime_parameters())]
		pub fn set_blob_runtime_parameters(
			origin: OriginFor<T>,
			max_blob_size: Option<u64>,
			min_blob_holder_percentage: Option<Perbill>,
			min_blob_holder_count: Option<u32>,
			blob_ttl: Option<u64>,
			temp_blob_ttl: Option<u64>,
			min_transaction_validity: Option<u64>,
			max_transaction_validity: Option<u64>,
			max_blob_retry_before_discarding: Option<u16>,
			max_block_size: Option<u64>,
			max_total_old_submission_size: Option<u64>,
			disable_old_da_submission: Option<bool>,
			vouch_threshold: Option<u32>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			BlobRuntimeParams::<T>::try_mutate(|params| -> Result<(), Error<T>> {
				if let Some(v) = max_blob_size {
					ensure!(v <= 31 * 1024 * 1024, Error::<T>::BlobSizeTooLarge);
					params.max_blob_size = v;
				}
				if let Some(v) = min_blob_holder_percentage {
					ensure!(
						v > Perbill::from_percent(0),
						Error::<T>::MinBlobHolderPercentageInvalid
					);
					params.min_blob_holder_percentage = v;
				}
				if let Some(v) = min_blob_holder_count {
					ensure!(v > 0, Error::<T>::MinBlobHolderCountInvalid);
					params.min_blob_holder_count = v;
				}
				if let Some(v) = blob_ttl {
					ensure!(v >= 1440, Error::<T>::BlobTtlTooShort);
					params.blob_ttl = v;
				}
				if let Some(v) = temp_blob_ttl {
					ensure!(v >= 50, Error::<T>::TempBlobTtlTooShort);
					params.temp_blob_ttl = v;
				}
				if let Some(v) = min_transaction_validity {
					ensure!(v > 3, Error::<T>::MinTransactionValidityTooLow);
					params.min_transaction_validity = v;
				}
				if let Some(v) = max_transaction_validity {
					ensure!(v < 150, Error::<T>::MaxTransactionValidityTooHigh);
					params.max_transaction_validity = v;
				}
				if let Some(v) = max_blob_retry_before_discarding {
					ensure!(v > 2, Error::<T>::MaxBlobRetryTooLow);
					params.max_blob_retry_before_discarding = v;
				}
				if let Some(v) = max_block_size {
					ensure!(
						v <= 5 * 1024 * 1024 * 1024,
						Error::<T>::MaxBlockSizeTooLarge
					);
					params.max_block_size = v;
				}
				if let Some(v) = max_total_old_submission_size {
					ensure!(v <= 4 * 1024 * 1024, Error::<T>::MaxOldSubmissionTooLarge);
					params.max_total_old_submission_size = v;
				}
				if let Some(v) = disable_old_da_submission {
					params.disable_old_da_submission = v;
				}
				if let Some(v) = vouch_threshold {
					ensure!(
						v > 0 && v <= T::MaxVouchesPerRecord::get(),
						Error::<T>::InvalidVouchThreshold
					);
					params.vouch_threshold = v;
				}

				Ok(())
			})?;

			let updated = BlobRuntimeParams::<T>::get();
			Self::deposit_event(Event::SubmitBlobRuntimeParametersSet {
				new_params: updated,
			});

			Ok(().into())
		}

		#[pallet::call_index(8)]
		#[pallet::weight(T::WeightInfo::set_submit_blob_metadata_fee_modifier())]
		pub fn set_submit_blob_metadata_fee_modifier(
			origin: OriginFor<T>,
			modifier: DispatchFeeModifier,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			SubmitBlobMetadataFeeModifier::<T>::put(modifier);

			Self::deposit_event(Event::SubmitBlobMetadataFeeModifierSet { value: modifier });

			Ok(().into())
		}

		#[pallet::call_index(9)]
		#[pallet::weight(T::WeightInfo::register_blob_offence())]
		pub fn register_blob_offence(
			origin: OriginFor<T>,
			offence_key: OffenceKey,
			voucher: ValidatorVoucher,
		) -> DispatchResultWithPostInfo {
			ensure_none(origin)?;

			// Ensure the key is valid based on the kind and params
			ensure!(offence_key.is_valid(), Error::<T>::InvalidOffenceKey);

			// Get validators list
			let validators = T::SessionDataProvider::validators();

			// Check given session
			let provided_session = voucher.session_index;
			let current_session = T::ValidatorSet::session_index();
			ensure!(
				provided_session == current_session,
				Error::<T>::InvalidVoucherSession
			);

			// Checks only run outside of tests and runtime benchmarks
			let Some(validator_id) =
				T::AccountId::decode(&mut &voucher.validator.encode()[..]).ok()
			else {
				return Err(Error::<T>::InvalidVoucherValidator.into());
			};

			// Check the sender and the signature
			Self::check_validator_and_signature(
				&offence_key,
				&voucher,
				current_session,
				&validator_id,
				&validators,
			)?;

			// Get or create the offence
			let mut record = BlobOffenceRecords::<T>::get(&offence_key).unwrap_or_else(|| {
				OffenceRecord::<T>::new(
					offence_key.kind.clone(),
					offence_key.block_hash,
					offence_key.blob_hash,
					offence_key.missing_validator.clone(),
				)
			});

			// If the vouchers already contains my voucher, return an error
			ensure!(
				!record
					.vouches
					.iter()
					.any(|v| voucher.validator == v.validator),
				Error::<T>::DuplicateVouch
			);

			// If the offence record has not reached threshold, add my boucher
			let mut added = false;
			let threshold_reached = record
				.has_reached_threshold(validators.len() as u32, T::MaxVouchesPerRecord::get());

			if !threshold_reached {
				// Reserve the report fee
				Self::reserve_vouch_fee(&validator_id)?;

				record
					.vouches
					.try_push(voucher.clone())
					.map_err(|_| Error::<T>::VouchListFull)?;
				BlobOffenceRecords::<T>::insert(&offence_key, record.clone());
				added = true;
			}

			Self::deposit_event(Event::BlobOffenceReported {
				who: validator_id,
				offence_key,
				voucher,
				added,
			});

			// Refund full fee since it was valid
			Ok(().into())
		}

		#[pallet::call_index(10)]
		#[pallet::weight(T::WeightInfo::clear_blob_offence_records())]
		pub fn clear_blob_offence_records(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let _ = BlobOffenceRecords::<T>::clear(u32::max_value(), None);

			Ok(().into())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			// Let blob txs summary pass
			if matches!(call, Call::submit_blob_txs_summary { .. }) {
				return Ok(Default::default());
			}

			// Check extrinsic type
			let Call::register_blob_offence {
				offence_key,
				voucher,
			} = call
			else {
				return InvalidTransaction::Call.into();
			};

			// Check the key
			if !offence_key.is_valid() {
				return InvalidTransaction::BadProof.into();
			}

			// Check given session
			let provided_session = voucher.session_index;
			let current_session = T::ValidatorSet::session_index();
			if provided_session != current_session {
				return InvalidTransaction::Stale.into();
			}

			// Verify that key owner is the correct account id
			let key = voucher.key;
			let key_type = sp_core::crypto::key_types::BABE;
			let Some(key_owner) =
				T::SessionDataProvider::get_validator_from_key(key_type, key.encode())
			else {
				return InvalidTransaction::BadProof.into();
			};
			let Some(validator) = T::AccountId::decode(&mut &voucher.validator.encode()[..]).ok()
			else {
				return InvalidTransaction::BadProof.into();
			};

			if key_owner != validator {
				return InvalidTransaction::BadProof.into();
			}

			// Check that validator is part of the active set
			let validators = T::SessionDataProvider::validators();
			if !validators.contains(&validator) {
				return InvalidTransaction::BadProof.into();
			}

			// Check that validator has enough balance
			let free_balance = T::Currency::free_balance(&validator).saturated_into::<u128>();
			let fee = T::BlobVouchFeeReserve::get();
			if free_balance < fee {
				return InvalidTransaction::Payment.into();
			}

			// Check the voucher signature
			if !voucher.verify_signature((offence_key.clone(), current_session).encode()) {
				return InvalidTransaction::BadProof.into();
			}

			ValidTransaction::with_tag_prefix("BlobOffence")
				.longevity(32)
				.and_provides((offence_key, voucher))
				.propagate(true)
				.build()
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
		SubmitDataFeeModifierSet {
			value: DispatchFeeModifier,
		},
		SubmitBlobMetadataRequest {
			who: T::AccountId,
			blob_hash: H256,
		},
		SubmitBlobRuntimeParametersSet {
			new_params: BlobRuntimeParameters,
		},
		SubmitBlobMetadataFeeModifierSet {
			value: DispatchFeeModifier,
		},
		BlobOffenceReported {
			who: T::AccountId,
			offence_key: OffenceKey,
			voucher: ValidatorVoucher,
			added: bool,
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
		/// Submit block length proposal was made with values not power of 2
		NotPowerOfTwo,
		/// The commitment is empty
		CommitmentCannotBeEmpty,
		/// The blob size exceeds the allowed maximum (e.g., > 31 MB).
		BlobSizeTooLarge,
		/// The minimum percentage of validators required to hold a blob is invalid (must be > 0).
		MinBlobHolderPercentageInvalid,
		/// The minimum number of validators required to hold a blob is invalid (must be > 0).
		MinBlobHolderCountInvalid,
		/// The blob TTL (time-to-live) is too short (must be at least ~1 day = 1440 blocks).
		BlobTtlTooShort,
		/// The temporary blob TTL is too short (must be at least 50 blocks).
		TempBlobTtlTooShort,
		/// The minimum transaction validity period is too low (must be > 3 blocks).
		MinTransactionValidityTooLow,
		/// The maximum transaction validity period is too high (must be < 150 blocks).
		MaxTransactionValidityTooHigh,
		/// The maximum retry count before discarding a blob is too low (must be > 2).
		MaxBlobRetryTooLow,
		/// The maximum block size is too big.
		MaxBlockSizeTooLarge,
		/// The maximum old submissions in a block is too large.
		MaxOldSubmissionTooLarge,
		/// Old data submission are disabled.
		OldDaSubmissionDisabled,
		/// The vouch threshold cannot be zero.
		InvalidVouchThreshold,
		/// Attempted to register an invalid offence key.
		InvalidOffenceKey,
		/// Provided voucher session is invalid.
		InvalidVoucherSession,
		/// Provided voucher validator is invalid.
		InvalidVoucherValidator,
		/// The caller is not part of the current active validator set.
		NotAnActiveValidator,
		/// The voucher signature is invalid or does not match the offence key payload.
		InvalidVoucherSignature,
		/// This validator has already vouched for this offence record.
		DuplicateVouch,
		/// Failed to add a new vouch entry because the vouch list is full.
		VouchListFull,
		/// Unable to reserve the vouch fee (insufficient funds or unexpected reserve failure).
		InsufficientBalanceForVouch,
		/// Invalid AppId
		InvalidAppId,
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
		// Offsetting the base_weight multiplication done for all txs
		let base_weight = ExtrinsicBaseWeight::get().saturating_mul(100);
		let acceptable_limit: Weight = T::WeightInfo::submit_block_length_proposal()
			.saturating_mul(5)
			.saturating_add(base_weight);
		current_normal_weight.all_lte(acceptable_limit)
	}

	/// Try to reserve the vouch fee for `who`.
	/// If the account doesn't have enough free balance, returns an error.
	pub fn reserve_vouch_fee(who: &T::AccountId) -> DispatchResult {
		let amount: BalanceOf<T> = T::BlobVouchFeeReserve::get().saturated_into();

		T::Currency::reserve(who, amount).map_err(|_| Error::<T>::InsufficientBalanceForVouch)?;

		Ok(())
	}

	/// Return (unreserve) the vouch fee back to `who`.
	pub fn return_vouch_fee(who: &T::AccountId, vouch_fee: Option<BalanceOf<T>>) {
		let amount: BalanceOf<T> =
			vouch_fee.unwrap_or(T::BlobVouchFeeReserve::get().saturated_into());
		let _unreserved = T::Currency::unreserve(who, amount);
	}

	/// Slash the vouch fee, permanently remove the reserved balance from `who`.
	pub fn slash_vouch_fee(who: &T::AccountId, vouch_fee: Option<BalanceOf<T>>) {
		let amount: BalanceOf<T> =
			vouch_fee.unwrap_or(T::BlobVouchFeeReserve::get().saturated_into());
		let (imbalance, _remainder) = T::Currency::slash_reserved(who, amount);

		drop(imbalance);
	}

	pub fn check_validator_and_signature(
		offence_key: &OffenceKey,
		voucher: &ValidatorVoucher,
		current_session: u32,
		validator_id: &T::AccountId,
		validators: &Vec<T::AccountId>,
	) -> DispatchResult {
		// Early return if test or benchmark mode
		if cfg!(test) || cfg!(feature = "runtime-benchmarks") {
			return Ok(());
		}

		let key = voucher.key;
		let key_type = sp_core::crypto::key_types::BABE;

		// Check validator address validity
		ensure!(
			Some(validator_id.clone())
				== T::SessionDataProvider::get_validator_from_key(key_type, key.encode()),
			Error::<T>::InvalidVoucherValidator
		);

		// Check if the extrinsic is coming from an active validator
		ensure!(
			validators.contains(validator_id),
			Error::<T>::NotAnActiveValidator
		);

		// Check that the offence key is correctly signed
		ensure!(
			voucher.verify_signature((offence_key.clone(), current_session).encode()),
			Error::<T>::InvalidVoucherSignature
		);

		Ok(())
	}
}

pub mod weight_helper {

	use super::*;

	/// Weight for `dataAvailability::submit_data`.
	pub fn submit_data<T: Config>(data_len: usize) -> Weight {
		/* Compute regular substrate weight. */
		let data_len: u32 = data_len.saturated_into();
		let data_prefix_len: u32 =
			compact_len(&data_len).unwrap_or(4 /* We imply the maximum */);
		// Get the encoded len.
		let encoded_data_len: u32 = match data_len.checked_add(data_prefix_len) {
			Some(l) => l,
			None => data_len,
		};
		let basic_weight = T::WeightInfo::submit_data(data_len);
		let data_root_weight = T::WeightInfo::data_root(data_len);
		let regular_weight = basic_weight.saturating_add(data_root_weight);

		/* Compute weight based on size taken in the matrix and hence computation. */
		// We get the current settings for matrix columns, rows and chunk_size.
		// Before, we got this value from the runtime, now that the matric size is increased for blobs,
		// We take raw value corresponding to 4mb of data when multiplied by the chunk size
		let blob_runtime_params = BlobRuntimeParams::<T>::get();
		let max_total_old_submission_size: u32 = blob_runtime_params
			.max_total_old_submission_size
			.saturated_into();
		let chunk_size: u32 = 32;
		let max_scalar = max_total_old_submission_size.div_ceil(chunk_size);

		// We compute the maximum numbers of scalars in the matrix and multiply with the DA dispatch ratio.
		let max_scalar_da_ratio = DA_DISPATCH_RATIO_PERBILL * max_scalar;

		// We get the current maximum weight in a block and multiply with normal dispatch ratio.
		let block_weights = <T as frame_system::Config>::BlockWeights::get();
		let max_weight_normal_ratio: u64 =
			NORMAL_DISPATCH_RATIO_PERBILL * block_weights.max_block.ref_time();

		// We compute the number of scalars
		let nb_scalar = encoded_data_len
			.saturating_add(chunk_size - 1)
			.saturating_div(chunk_size - 1);

		// We compute the ratio of nb scalars / max scalars in the matrix and multiply with the maximum weight.
		let data_scalar_ratio = Perbill::from_rational(nb_scalar, max_scalar_da_ratio);
		let ref_time = data_scalar_ratio * max_weight_normal_ratio;
		let scalar_based_weight = Weight::from_parts(ref_time, regular_weight.proof_size());

		// We return the biggest value between the regular weight and scalar based weight.
		// I cannot think of a case where regular weight > matrix based weight.
		scalar_based_weight.max(regular_weight)
	}

	fn compact_len(value: &u32) -> Option<u32> {
		let len = Compact::<u32>::compact_len(value);
		u32::try_from(len).ok()
	}

	/// Weight for `dataAvailability::submit_blob_metadata`.
	pub fn submit_blob_metadata<T: Config>(data_len: u64) -> Weight {
		/* Compute regular substrate weight. */
		let data_len_u32: u32 = data_len.saturated_into();
		let regular_weight = T::WeightInfo::submit_blob_metadata(data_len_u32);

		/* Compute weight based on size taken compared to the maximum in a block. */
		// Before we used to compare to the matrix total size, but with new values for blob crate
		// We could store data up to 128mb worth of commitment which is huge
		// Hence we use tha maximum allowed in a block
		let blob_runtime_params = BlobRuntimeParams::<T>::get();
		let max_total_submission_size = blob_runtime_params.max_block_size;

		// We compute the maximum numbers of scalars in the matrix and multiply with the DA dispatch ratio.
		let max_da_ratio = DA_DISPATCH_RATIO_PERBILL * max_total_submission_size;

		// We get the current maximum weight in a block and multiply with normal dispatch ratio.
		let block_weights = <T as frame_system::Config>::BlockWeights::get();
		let max_weight_normal_ratio: u64 =
			NORMAL_DISPATCH_RATIO_PERBILL * block_weights.max_block.ref_time();

		// We compute the ratio of data size / max_da_ratio multiply with the maximum weight.
		let data_ratio = Perbill::from_rational(data_len, max_da_ratio);
		let ref_time = data_ratio * max_weight_normal_ratio;
		let da_weight = Weight::from_parts(ref_time, regular_weight.proof_size());

		// We return the biggest value between the regular weight and da weight.
		// I cannot think of a case where regular weight > da weight.
		da_weight.max(regular_weight)
	}
}
