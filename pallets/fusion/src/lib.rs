#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
mod weights;

use frame_support::{
	dispatch::GetDispatchInfo,
	pallet_prelude::*,
	traits::{Currency, LockableCurrency, UnfilteredDispatchable},
};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use scale_info::{build::Fields, Path, Type};
use sp_core::H160;
use sp_runtime::{AccountId32, Perbill, Saturating};
use sp_staking::EraIndex;
use sp_std::prelude::*;
pub use weights::WeightInfo;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// Type representing an EVM address
pub type EvmAddress = H160;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct FusionLedger<Balance> {
	pub balance: Balance,
	pub start_era: EraIndex,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen)]
pub struct FusionPool<T: Config>
where
	T::AccountId: TypeInfo,
{
	pub owner: T::AccountId,
	pub members: BoundedVec<EvmAddress, ConstU32<100>>,
	pub candidates: BoundedVec<T::AccountId, ConstU32<100>>,
}
impl<T: Config> Default for FusionPool<T>
where
	T::AccountId: TypeInfo,
{
	fn default() -> Self {
		let account: AccountId32 =
			hex_literal::hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"]
				.into();

		let alice_account_id =
			T::AccountId::decode(&mut &account.encode()[..]).expect("Valid AccountId; qed");

		FusionPool {
			owner: alice_account_id,
			members: BoundedVec::default(),
			candidates: BoundedVec::default(),
		}
	}
}
impl<T: Config> TypeInfo for FusionPool<T>
where
	T::AccountId: TypeInfo,
{
	type Identity = Self;

	fn type_info() -> Type {
		Type::builder()
			.path(Path::new("FusionPool", module_path!()))
			.type_params(Vec::new()) // Or include type params if necessary
			.composite(
				Fields::named()
					.field(|f| {
						f.ty::<T::AccountId>()
							.name("owner")
							.type_name("T::AccountId")
					})
					.field(|f| {
						f.ty::<BoundedVec<EvmAddress, ConstU32<100>>>()
							.name("members")
							.type_name("BoundedVec<H160, ConstU32<100>>")
					})
					.field(|f| {
						f.ty::<BoundedVec<T::AccountId, ConstU32<100>>>()
							.name("candidates")
							.type_name("BoundedVec<T::AccountId, ConstU32<100>>")
					}),
			)
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// A sudo-able call.
		type RuntimeCall: Parameter
			+ UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ GetDispatchInfo;

		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		/// Currency type for this pallet.
		type Currency: LockableCurrency<Self::AccountId, Moment = BlockNumberFor<Self>>;

		/// A provider that gives the current era.
		type EraProvider: EraProvider;

		/// The percentage of reward to get from total era payout.
		#[pallet::constant]
		type FusionPayoutPercentage: Get<Perbill>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn fusion_ledgers)]
	pub type FusionLedgers<T: Config> =
		StorageMap<_, Blake2_128Concat, EvmAddress, FusionLedger<BalanceOf<T>>>;

	#[pallet::storage]
	#[pallet::getter(fn eras_fusion_reward)]
	pub type ErasFusionReward<T: Config> = StorageMap<_, Blake2_128Concat, EraIndex, BalanceOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn total_in_ledgers)]
	pub type TotalInLedgers<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn main_fusion_pool)]
	pub type MainFusionPool<T: Config> = StorageValue<_, FusionPool<T>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new entry was added in the FusionLedgers
		FusionLedgerEntryAdded {
			evm_address: EvmAddress,
			amount: BalanceOf<T>,
			start_era: EraIndex,
		},
		/// Total in ledgers has been updated
		FusionLedgerTotalUpdated { total: BalanceOf<T> },
		/// The Fusion part of payout was insterted in the Fusion storage
		EraFusionRewardInserted { era: EraIndex, reward: BalanceOf<T> },
		/// A reward was claimed from the Era Fusion Reward storage
		EraFusionRewardClaimed {
			who: T::AccountId,
			owner_account: T::AccountId,
			era: EraIndex,
			reward: BalanceOf<T>,
		},
		/// The Fusion Pool has been updated
		FusionPoolUpdated {
			owner: T::AccountId,
			members: BoundedVec<EvmAddress, ConstU32<100>>,
			candidates: BoundedVec<T::AccountId, ConstU32<100>>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// No Era Fusion Reward exists for the given era.
		NoEraFusionReward,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::add_fusion_ledger_entry())]
		pub fn add_fusion_ledger_entry(
			origin: OriginFor<T>,
			evm_address: EvmAddress,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			Self::do_add_fusion_ledger_entry(evm_address, amount)?;
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::set_total_in_ledgers())]
		pub fn set_total_in_ledgers(origin: OriginFor<T>, total: BalanceOf<T>) -> DispatchResult {
			ensure_signed(origin)?;
			Self::do_set_total_in_ledgers(total)?;
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::claim_era_fusion_reward())]
		pub fn claim_era_fusion_reward(origin: OriginFor<T>, era: EraIndex) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_claim_era_fusion_reward(who, era)?;
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::set_fusion_pool())]
		pub fn set_fusion_pool(
			origin: OriginFor<T>,
			owner: T::AccountId,
			members: BoundedVec<EvmAddress, ConstU32<100>>,
			candidates: BoundedVec<T::AccountId, ConstU32<100>>,
		) -> DispatchResult {
			ensure_signed(origin)?;

			// Call the trait function to update the pool
			Self::do_set_fusion_pool(owner, members, candidates)?;

			Ok(())
		}
	}
}

pub trait EraProvider {
	fn current_era() -> EraIndex;
}

pub trait FusionExt<AccountId, Balance> {
	fn get_fusion_payout_percentage() -> Perbill;
	fn insert_era_fusion_reward(era: EraIndex, reward: Balance);
	fn do_add_fusion_ledger_entry(evm_address: EvmAddress, amount: Balance) -> DispatchResult;
	fn do_set_total_in_ledgers(new_total: Balance) -> DispatchResult;
	fn do_claim_era_fusion_reward(who: AccountId, era: EraIndex) -> DispatchResult;
	fn do_set_fusion_pool(
		owner: AccountId,
		members: BoundedVec<EvmAddress, ConstU32<100>>,
		candidates: BoundedVec<AccountId, ConstU32<100>>,
	) -> DispatchResult;
}

impl<T: Config> FusionExt<T::AccountId, BalanceOf<T>> for Pallet<T> {
	fn get_fusion_payout_percentage() -> Perbill {
		T::FusionPayoutPercentage::get()
	}

	fn insert_era_fusion_reward(era: EraIndex, reward: BalanceOf<T>) {
		ErasFusionReward::<T>::insert(era, reward);
		Self::deposit_event(Event::EraFusionRewardInserted { era, reward });
	}

	fn do_add_fusion_ledger_entry(evm_address: EvmAddress, amount: BalanceOf<T>) -> DispatchResult {
		let start_era = T::EraProvider::current_era();

		FusionLedgers::<T>::insert(
			evm_address,
			FusionLedger {
				balance: amount,
				start_era,
			},
		);

		let total_in_ledgers = TotalInLedgers::<T>::get();
		let new_total = total_in_ledgers.saturating_add(amount);
		Self::do_set_total_in_ledgers(new_total)?;

		Self::deposit_event(Event::FusionLedgerEntryAdded {
			evm_address,
			amount,
			start_era,
		});
		Ok(())
	}

	fn do_set_total_in_ledgers(new_total: BalanceOf<T>) -> DispatchResult {
		TotalInLedgers::<T>::put(new_total);
		Self::deposit_event(Event::FusionLedgerTotalUpdated { total: new_total });
		Ok(())
	}

	fn do_claim_era_fusion_reward(who: T::AccountId, era: EraIndex) -> DispatchResult {
		let fusion_pool = MainFusionPool::<T>::get();

		let owner = fusion_pool.owner;

		let era_reward = ErasFusionReward::<T>::get(&era).ok_or(Error::<T>::NoEraFusionReward)?;

		T::Currency::deposit_creating(&owner, era_reward);

		ErasFusionReward::<T>::remove(era);

		Self::deposit_event(Event::EraFusionRewardClaimed {
			who,
			owner_account: owner,
			era,
			reward: era_reward,
		});

		Ok(())
	}

	fn do_set_fusion_pool(
		owner: T::AccountId,
		members: BoundedVec<EvmAddress, ConstU32<100>>,
		candidates: BoundedVec<T::AccountId, ConstU32<100>>,
	) -> DispatchResult {
		// Create a new FusionPool with the provided values
		let new_pool = FusionPool {
			owner: owner.clone(),
			members: members.clone(),
			candidates: candidates.clone(),
		};

		// Update the storage with the new FusionPool
		MainFusionPool::<T>::put(new_pool);

		// Emit an event for the update
		Self::deposit_event(Event::FusionPoolUpdated {
			owner,
			members,
			candidates,
		});

		Ok(())
	}
}
