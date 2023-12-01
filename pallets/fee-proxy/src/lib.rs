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
use sp_runtime::FixedPointNumber;
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
			+ GetDispatchInfo
			+ IsType<<Self as frame_system::Config>::RuntimeCall>;

		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		/// Currency type.
		type Currency: Currency<Self::AccountId>;

		/// What we do with fees.
		type FeesCollector: OnUnbalanced<NegativeImbalanceOf<Self>>;
	}

	/// The proxy account used to pay for fees.
	#[pallet::storage]
	#[pallet::getter(fn fee_proxy_account)]
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

			// Compute fee
			let fee = fee_helper::get_fee::<T>(&call)?;

			// Check if proxy account has enough balance or InsufficientBalanceInProxyAccount
			let reason = WithdrawReasons::FEE;
			let imbalance = T::Currency::withdraw(&fee_proxy_account, fee, reason, KeepAlive)
				.map_err(|_| Error::<T>::InsufficientBalanceInProxyAccount)?;

			// Take fee from proxy account
			T::FeesCollector::on_unbalanced(imbalance);

			// Dispatch the call
			let res = call.dispatch(origin);

			// Event
			Self::deposit_event(Event::WrappedOp {
				result: res.map(|_| ()).map_err(|e| e.error),
			});

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
		/// An error occured while computing the fee
		FeeComputationError,
	}
}

mod fee_helper {
	use super::*;

	pub(crate) fn get_fee<T: Config>(
		call: &<T as Config>::RuntimeCall,
	) -> Result<
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance,
		Error<T>,
	>
	where
		T: pallet_transaction_payment::Config,
		<OnChargeTransactionOf<T> as OnChargeTransaction<T>>::Balance: FixedPointOperand,
	{
		// Here we do the same calculation as the transaction payment pallet, to monitor in case it changes
		let dispatch_info = call.get_dispatch_info();
		let total_weight = T::WeightInfo::wrap().saturating_add(dispatch_info.weight);
		let unadjusted_weight_fee =
			pallet_transaction_payment::Pallet::<T>::weight_to_fee(total_weight);
		let multiplier = pallet_transaction_payment::Pallet::<T>::next_fee_multiplier();
		let adjusted_weight_fee = multiplier.saturating_mul_int(unadjusted_weight_fee);
		let len_fee =
			pallet_transaction_payment::Pallet::<T>::length_to_fee(call.encode().len() as u32); // Here it's just the inner call, but it should be ok
		let base_fee = pallet_transaction_payment::Pallet::<T>::weight_to_fee(
			T::BlockWeights::get()
				.get(dispatch_info.class)
				.base_extrinsic,
		);
		let fee_data = pallet_transaction_payment::FeeDetails {
			inclusion_fee: Some(pallet_transaction_payment::InclusionFee {
				base_fee,
				len_fee,
				adjusted_weight_fee,
			}),
			tip: 0u32.into(), // Here we put 0 as tip cause if the real sender put a tip, he will pay for it himself
		};
		let final_fee = fee_data.final_fee();

		// We convert the fee to u128
		let maybe_fee_u128: Result<u128, _> = final_fee.try_into();
		match maybe_fee_u128 {
			Ok(fee) => fee.try_into().map_err(|_| Error::<T>::FeeComputationError),
			Err(_) => Err(Error::<T>::FeeComputationError),
		}
	}
}
