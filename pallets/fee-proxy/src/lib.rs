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
	traits::{Currency, ExistenceRequirement::KeepAlive, OnUnbalanced, WithdrawReasons},
};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use pallet_transaction_payment::OnChargeTransaction;
use sp_runtime::{traits::Dispatchable, FixedPointOperand};
use sp_std::prelude::*;
pub use weights::WeightInfo;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	// pub type BalanceOf<T> =
	// 	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Type aliases used for interaction with `OnChargeTransaction`.
	pub type OnChargeTransactionOf<T> =
		<T as pallet_transaction_payment::Config>::OnChargeTransaction;
	/// Balance type alias.
	pub type BalanceOf<T> = <OnChargeTransactionOf<T> as OnChargeTransaction<T>>::Balance;

	pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::NegativeImbalance;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_transaction_payment::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// A call to wrap and leverage fees from caller.
		type RuntimeCall: Parameter
			+ Dispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ GetDispatchInfo;

		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		/// Currency type.
		type Currency: Currency<Self::AccountId>;

		/// What we do with fees.
		type FeesCollector: OnUnbalanced<NegativeImbalanceOf<Self>>;
	}

	/// The proxy account used to pay for fees.
	#[pallet::storage]
	#[pallet::getter(fn fee_procy_account)]
	pub(super) type FeeProxyAccount<T: Config> = StorageValue<_, Option<T::AccountId>, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		BalanceOf<T>: FixedPointOperand,
	{
		#[pallet::call_index(0)]
		#[pallet::weight({
			let dispatch_info = call.get_dispatch_info();
			(
				T::WeightInfo::wrap().saturating_add(dispatch_info.weight),
				dispatch_info.class,
				Pays::No
			)
		})]
		pub fn wrap(origin: OriginFor<T>, call: Box<<T as Config>::RuntimeCall>) -> DispatchResult {
			ensure_signed(origin.clone())?;

			// Get proxy account
			let fee_proxy_account =
				FeeProxyAccount::<T>::get().ok_or(Error::<T>::ProxyAccountNotSet)?;

			// Check if tip -> TipIsNotAllowed

			// Compute fee
			let dispatch_info = call.get_dispatch_info();
			let feeee = pallet_transaction_payment::Pallet::<T>::compute_fee_details(
				call.encode().len() as u32,
				&dispatch_info.into(),
				0u32.into()
			);

			let total_weight = T::WeightInfo::wrap().saturating_add(dispatch_info.weight);

			// let fee: BalanceOf<T> = 100u32.into();

			// // Check if proxy account has enough balance or InsufficientBalanceInProxyAccount
			// let reason = WithdrawReasons::FEE;
			// let imbalance = T::Currency::withdraw(&fee_proxy_account, fee, reason, KeepAlive)
			// 	.map_err(|_| Error::<T>::ProxyAccountNotSet)?;

			// // Take fee from proxy account
			// T::FeesCollector::on_unbalanced(imbalance);

			// // Dispatch the call
			// let res = call.dispatch(origin);

			// // Event
			// Self::deposit_event(Event::WrappedOp {
			// 	result: res.map(|_| ()).map_err(|e| e.error),
			// });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::set_proxy_account())]
		pub fn set_proxy_account(
			origin: OriginFor<T>,
			account: Option<T::AccountId>,
		) -> DispatchResult {
			ensure_root(origin)?;

			FeeProxyAccount::<T>::put(account.clone());

			Self::deposit_event(Event::ProxyAccountSet { account });

			Ok(())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A wrapped operation was executed, show result
		WrappedOp { result: DispatchResult },
		/// The fee proxy account was set or unset
		ProxyAccountSet { account: Option<T::AccountId> },
	}

	/// Error for the Fee proxy pallet
	#[pallet::error]
	pub enum Error<T> {
		/// The proxy account has insufficient balance to cover for fees.
		InsufficientBalanceInProxyAccount,
		/// The proxy is not set and the feature is disabled.
		ProxyAccountNotSet,
		/// Tipping is not allowed while using this pallet
		TipIsNotAllowed,
	}
}
