use super::MAX_ITERATIONS;
use crate::{Call as DACall, Config as DAConfig};
use avail_core::InvalidTransactionCustomId;

use codec::{Decode, Encode};
use frame_support::{
	ensure,
	traits::{IsSubType, IsType},
};
use frame_system::Config as SystemConfig;
use pallet_multisig::{Call as MultisigCall, Config as MultisigConfig};
use pallet_proxy::{Call as ProxyCall, Config as ProxyConfig};
use pallet_scheduler::{Call as SchedulerCall, Config as SchedulerConfig};
use pallet_utility::{Call as UtilityCall, Config as UtilityConfig};
use pallet_vector::{Call as VectorCall, Config as VectorConfig};
use scale_info::TypeInfo;
use sp_runtime::transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction};
use sp_std::{default::Default, vec::Vec};

struct WrappedCall<'a, T>(pub &'a <T as SystemConfig>::RuntimeCall)
where
	T: DAConfig
		+ UtilityConfig
		+ VectorConfig
		+ MultisigConfig
		+ ProxyConfig
		+ SchedulerConfig
		+ Send
		+ Sync,
	<T as SystemConfig>::RuntimeCall: IsSubType<DACall<T>>
		+ IsSubType<UtilityCall<T>>
		+ IsSubType<VectorCall<T>>
		+ IsSubType<MultisigCall<T>>
		+ IsSubType<ProxyCall<T>>
		+ IsSubType<SchedulerCall<T>>,
	[u8; 32]: From<<T as frame_system::Config>::AccountId>;

impl<'a, T> WrappedCall<'a, T>
where
	T: DAConfig
		+ UtilityConfig
		+ VectorConfig
		+ MultisigConfig
		+ ProxyConfig
		+ SchedulerConfig
		+ Send
		+ Sync,
	<T as SystemConfig>::RuntimeCall: IsSubType<DACall<T>>
		+ IsSubType<UtilityCall<T>>
		+ IsSubType<VectorCall<T>>
		+ IsSubType<MultisigCall<T>>
		+ IsSubType<ProxyCall<T>>
		+ IsSubType<SchedulerCall<T>>,
	[u8; 32]: From<<T as frame_system::Config>::AccountId>,
{
	pub fn is_submit_data_call(&self) -> bool {
		matches!(self.0.is_sub_type(), Some(DACall::<T>::submit_data { .. }))
	}

	pub fn is_send_message_call(&self) -> bool {
		matches!(
			self.0.is_sub_type(),
			Some(VectorCall::<T>::send_message { .. })
		)
	}

	pub fn get_scheduler_call(&self) -> Option<&<T as SchedulerConfig>::RuntimeCall> {
		match self.0.is_sub_type() {
			Some(SchedulerCall::<T>::schedule {
				call,
				when: _,
				maybe_periodic: _,
				priority: _,
			})
			| Some(SchedulerCall::<T>::schedule_after {
				after: _,
				maybe_periodic: _,
				priority: _,
				call,
			})
			| Some(SchedulerCall::<T>::schedule_named {
				id: _,
				when: _,
				maybe_periodic: _,
				priority: _,
				call,
			})
			| Some(SchedulerCall::<T>::schedule_named_after {
				id: _,
				after: _,
				maybe_periodic: _,
				priority: _,
				call,
			}) => Some(call),
			_ => None,
		}
	}

	pub fn get_batch_call(&self) -> Option<&Vec<<T as UtilityConfig>::RuntimeCall>> {
		match self.0.is_sub_type() {
			Some(UtilityCall::<T>::batch { calls })
			| Some(UtilityCall::<T>::batch_all { calls })
			| Some(UtilityCall::<T>::force_batch { calls }) => Some(calls),
			_ => None,
		}
	}

	pub fn get_as_multi_call(&self) -> Option<&<T as MultisigConfig>::RuntimeCall> {
		match self.0.is_sub_type() {
			Some(MultisigCall::<T>::as_multi {
				call,
				threshold: _,
				other_signatories: _,
				maybe_timepoint: _,
				max_weight: _,
			})
			| Some(MultisigCall::as_multi_threshold_1 {
				other_signatories: _,
				call,
			}) => {
				//
				Some(call)
			},
			_ => None,
		}
	}

	pub fn get_proxy_call(&self) -> Option<&<T as ProxyConfig>::RuntimeCall> {
		match self.0.is_sub_type() {
			Some(ProxyCall::<T>::proxy {
				call,
				real: _,
				force_proxy_type: _,
			}) => Some(call),
			_ => None,
		}
	}
}

/// TODO
///
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CheckBatchTransactions<
	T: DAConfig + UtilityConfig + MultisigConfig + ProxyConfig + SchedulerConfig + Send + Sync,
>(sp_std::marker::PhantomData<T>);

impl<T> CheckBatchTransactions<T>
where
	T: DAConfig
		+ UtilityConfig
		+ VectorConfig
		+ MultisigConfig
		+ ProxyConfig
		+ SchedulerConfig
		+ Send
		+ Sync,
	<T as MultisigConfig>::RuntimeCall: IsSubType<VectorCall<T>>
		+ IsSubType<ProxyCall<T>>
		+ IsSubType<UtilityCall<T>>
		+ IsSubType<MultisigCall<T>>
		+ IsSubType<SchedulerCall<T>>,
	<T as SchedulerConfig>::RuntimeCall: IsSubType<VectorCall<T>>
		+ IsSubType<ProxyCall<T>>
		+ IsSubType<UtilityCall<T>>
		+ IsSubType<MultisigCall<T>>
		+ IsSubType<SchedulerCall<T>>,
	<T as SystemConfig>::RuntimeCall: IsSubType<DACall<T>>
		+ IsSubType<UtilityCall<T>>
		+ IsSubType<VectorCall<T>>
		+ IsSubType<MultisigCall<T>>
		+ IsSubType<ProxyCall<T>>
		+ IsSubType<SchedulerCall<T>>,
	[u8; 32]: From<<T as frame_system::Config>::AccountId>,
{
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		Self(sp_std::marker::PhantomData)
	}

	/// DataAvailability::submit_data and any Bridge::* transactions are forbidden to be included inside batch transactions.
	pub fn do_validate(
		&self,
		call: &<T as SystemConfig>::RuntimeCall,
		_len: usize,
	) -> TransactionValidity {
		let iterations = 0;
		let call = WrappedCall::<T>(call);

		if let Some(call) = call.get_proxy_call() {
			Self::recursive_proxy_call(call, iterations, false)?;
			return Ok(ValidTransaction::default());
		}

		if let Some(call) = call.get_as_multi_call() {
			Self::recursive_multisig_call(call, iterations, false)?;
			return Ok(ValidTransaction::default());
		}

		if let Some(calls) = call.get_batch_call() {
			Self::recursive_batch_call(calls, iterations, false)?;
			return Ok(ValidTransaction::default());
		}

		if let Some(call) = call.get_scheduler_call() {
			Self::recursive_scheduler_call(call, iterations, false)?;
			return Ok(ValidTransaction::default());
		}

		Ok(ValidTransaction::default())
	}

	// No Send Message or Submit Data calls are allowed inside Batch Call
	fn recursive_batch_call(
		calls: &Vec<<T as UtilityConfig>::RuntimeCall>,
		iteration: usize,
		inside_batch: bool,
	) -> TransactionValidity {
		use InvalidTransactionCustomId::*;

		if iteration >= MAX_ITERATIONS {
			return Err(InvalidTransaction::Custom(MaxRecursionExceeded as u8).into());
		}

		for call in calls {
			let call: &<T as SystemConfig>::RuntimeCall = call.into_ref();
			let call = WrappedCall::<T>(call);

			ensure!(
				!call.is_submit_data_call(),
				InvalidTransaction::Custom(UnexpectedSubmitDataCall as u8)
			);
			ensure!(
				!call.is_send_message_call(),
				InvalidTransaction::Custom(UnexpectedSendMessageCall as u8)
			);

			if let Some(call) = call.get_proxy_call() {
				Self::recursive_proxy_call(call, iteration + 1, true)?;
			};

			if let Some(call) = call.get_as_multi_call() {
				Self::recursive_multisig_call(call, iteration + 1, true)?;
			};

			if let Some(calls) = call.get_batch_call() {
				if inside_batch {
					return Err(InvalidTransaction::Custom(MaxRecursionExceeded as u8).into());
				}
				Self::recursive_batch_call(calls, iteration + 1, true)?;
			};

			if let Some(call) = call.get_scheduler_call() {
				Self::recursive_scheduler_call(call, iteration + 1, true)?;
			};
		}

		Ok(ValidTransaction::default())
	}

	// Send Message is allowed if it is behind a zero, single or double transaction that accepts a call.
	fn recursive_proxy_call(
		call: &<T as ProxyConfig>::RuntimeCall,
		iteration: usize,
		inside_batch: bool,
	) -> TransactionValidity {
		use InvalidTransactionCustomId::*;
		if iteration >= MAX_ITERATIONS {
			return Err(InvalidTransaction::Custom(MaxRecursionExceeded as u8).into());
		}

		let call: &<T as SystemConfig>::RuntimeCall = call.into_ref();
		let call = WrappedCall::<T>(call);

		if iteration > 1 || inside_batch {
			ensure!(
				!call.is_send_message_call(),
				InvalidTransaction::Custom(UnexpectedSendMessageCall as u8)
			);
		}

		if let Some(call) = call.get_proxy_call() {
			return Self::recursive_proxy_call(call, iteration + 1, inside_batch);
		}

		if let Some(call) = call.get_as_multi_call() {
			return Self::recursive_multisig_call(call, iteration + 1, inside_batch);
		}

		if let Some(calls) = call.get_batch_call() {
			return Self::recursive_batch_call(calls, iteration + 1, inside_batch);
		}

		if let Some(call) = call.get_scheduler_call() {
			return Self::recursive_scheduler_call(call, iteration + 1, inside_batch);
		};

		// Everything else is OK
		return Ok(ValidTransaction::default());
	}

	// Send Message is allowed if it is behind a zero, single or double transaction that accepts a call.
	fn recursive_multisig_call(
		call: &<T as MultisigConfig>::RuntimeCall,
		iteration: usize,
		inside_batch: bool,
	) -> TransactionValidity {
		use InvalidTransactionCustomId::*;
		if iteration >= MAX_ITERATIONS {
			return Err(InvalidTransaction::Custom(MaxRecursionExceeded as u8).into());
		}

		if iteration > 1 || inside_batch {
			match call.is_sub_type() {
				Some(VectorCall::<T>::send_message { .. }) => {
					return Err(InvalidTransaction::Custom(UnexpectedSendMessageCall as u8).into())
				},
				_ => (),
			}
		}

		match call.is_sub_type() {
			Some(ProxyCall::<T>::proxy {
				call,
				real: _,
				force_proxy_type: _,
			}) => return Self::recursive_proxy_call(call, iteration + 1, inside_batch),
			_ => (),
		}

		match call.is_sub_type() {
			Some(UtilityCall::<T>::batch { calls })
			| Some(UtilityCall::<T>::batch_all { calls })
			| Some(UtilityCall::<T>::force_batch { calls }) => {
				return Self::recursive_batch_call(calls, iteration + 1, inside_batch);
			},
			_ => (),
		}

		match call.is_sub_type() {
			Some(MultisigCall::<T>::as_multi {
				call,
				threshold: _,
				other_signatories: _,
				maybe_timepoint: _,
				max_weight: _,
			})
			| Some(MultisigCall::as_multi_threshold_1 {
				other_signatories: _,
				call,
			}) => {
				return Self::recursive_multisig_call(call, iteration + 1, inside_batch);
			},
			_ => (),
		}

		match call.is_sub_type() {
			Some(SchedulerCall::<T>::schedule {
				call,
				when: _,
				maybe_periodic: _,
				priority: _,
			})
			| Some(SchedulerCall::<T>::schedule_after {
				after: _,
				maybe_periodic: _,
				priority: _,
				call,
			})
			| Some(SchedulerCall::<T>::schedule_named {
				id: _,
				when: _,
				maybe_periodic: _,
				priority: _,
				call,
			})
			| Some(SchedulerCall::<T>::schedule_named_after {
				id: _,
				after: _,
				maybe_periodic: _,
				priority: _,
				call,
			}) => {
				return Self::recursive_scheduler_call(call, iteration + 1, inside_batch);
			},
			_ => (),
		}

		// Everything else is OK
		return Ok(ValidTransaction::default());
	}

	// Send Message is not allowed at all
	fn recursive_scheduler_call(
		call: &<T as SchedulerConfig>::RuntimeCall,
		iteration: usize,
		inside_batch: bool,
	) -> TransactionValidity {
		use InvalidTransactionCustomId::*;
		if iteration >= MAX_ITERATIONS {
			return Err(InvalidTransaction::Custom(MaxRecursionExceeded as u8).into());
		}

		match call.is_sub_type() {
			Some(VectorCall::<T>::send_message { .. }) => {
				return Err(InvalidTransaction::Custom(UnexpectedSendMessageCall as u8).into())
			},
			_ => (),
		}

		match call.is_sub_type() {
			Some(ProxyCall::<T>::proxy {
				call,
				real: _,
				force_proxy_type: _,
			}) => return Self::recursive_proxy_call(call, iteration + 1, inside_batch),
			_ => (),
		}

		match call.is_sub_type() {
			Some(UtilityCall::<T>::batch { calls })
			| Some(UtilityCall::<T>::batch_all { calls })
			| Some(UtilityCall::<T>::force_batch { calls }) => {
				return Self::recursive_batch_call(calls, iteration + 1, inside_batch);
			},
			_ => (),
		}

		match call.is_sub_type() {
			Some(MultisigCall::<T>::as_multi {
				call,
				threshold: _,
				other_signatories: _,
				maybe_timepoint: _,
				max_weight: _,
			})
			| Some(MultisigCall::as_multi_threshold_1 {
				other_signatories: _,
				call,
			}) => {
				return Self::recursive_multisig_call(call, iteration + 1, inside_batch);
			},
			_ => (),
		}

		match call.is_sub_type() {
			Some(SchedulerCall::<T>::schedule {
				call,
				when: _,
				maybe_periodic: _,
				priority: _,
			})
			| Some(SchedulerCall::<T>::schedule_after {
				after: _,
				maybe_periodic: _,
				priority: _,
				call,
			})
			| Some(SchedulerCall::<T>::schedule_named {
				id: _,
				when: _,
				maybe_periodic: _,
				priority: _,
				call,
			})
			| Some(SchedulerCall::<T>::schedule_named_after {
				id: _,
				after: _,
				maybe_periodic: _,
				priority: _,
				call,
			}) => {
				return Self::recursive_scheduler_call(call, iteration + 1, inside_batch);
			},
			_ => (),
		}

		// Everything else is OK
		return Ok(ValidTransaction::default());
	}
}

#[cfg(test)]
mod tests {
	use avail_core::{
		asdr::AppUncheckedExtrinsic,
		data_proof::Message,
		AppId,
		InvalidTransactionCustomId::{
			MaxRecursionExceeded, UnexpectedSendMessageCall, UnexpectedSubmitDataCall,
		},
	};
	use frame_system::pallet::Call as SysCall;
	use pallet_utility::pallet::Call as UtilityCall;
	use sp_core::H256;
	use sp_runtime::transaction_validity::{InvalidTransaction, TransactionValidityError};
	use test_case::test_case;

	use super::*;
	use crate::pallet::Call as DACall;
	use crate::{
		extensions::extensions_mock::{new_test_ext, RuntimeCall, Test},
		CheckAppId,
	};

	fn remark_call() -> RuntimeCall {
		RuntimeCall::System(SysCall::remark { remark: vec![] })
	}

	fn submit_data_call() -> RuntimeCall {
		RuntimeCall::DataAvailability(DACall::submit_data {
			data: vec![].try_into().unwrap(),
		})
	}

	fn send_message_call() -> RuntimeCall {
		let message = Message::FungibleToken {
			asset_id: H256::default(),
			amount: 0,
		};
		RuntimeCall::Vector(VectorCall::send_message {
			message,
			to: H256::default(),
			domain: 0,
		})
	}

	fn batch_call(calls: Vec<RuntimeCall>) -> RuntimeCall {
		RuntimeCall::Utility(UtilityCall::batch { calls })
	}

	fn batch_all_call(calls: Vec<RuntimeCall>) -> RuntimeCall {
		RuntimeCall::Utility(UtilityCall::batch_all { calls })
	}

	fn force_batch_call(calls: Vec<RuntimeCall>) -> RuntimeCall {
		RuntimeCall::Utility(UtilityCall::force_batch { calls })
	}

	fn to_invalid_tx(custom_id: InvalidTransactionCustomId) -> TransactionValidity {
		Err(TransactionValidityError::Invalid(
			InvalidTransaction::Custom(custom_id as u8),
		))
	}

	fn validate(call: RuntimeCall) -> TransactionValidity {
		let extrinsic =
			AppUncheckedExtrinsic::<u32, RuntimeCall, (), ()>::new_unsigned(call.clone());
		let len = extrinsic.encoded_size();
		new_test_ext().execute_with(|| CheckAppId::<Test>::from(AppId(0)).do_validate(&call, len))
	}

	#[test]
	fn test_batch_iterations() {
		let mut call = batch_call(vec![remark_call()]);
		for _ in 0..MAX_ITERATIONS {
			call = batch_call(vec![call])
		}

		assert_eq!(
			validate(call),
			to_invalid_tx(MaxRecursionExceeded),
			"Stacking too many Batch calls should be blocked"
		);
	}

	#[test_case(submit_data_call() =>  Ok(ValidTransaction::default()); "Single Submit Data call should be allowed" )]
	#[test_case(send_message_call() =>  Ok(ValidTransaction::default()); "Single Send Message call should be allowed" )]
	#[test_case(remark_call() =>  Ok(ValidTransaction::default()); "Single Non-Submit-Data and Non-Send-Message call should be allowed" )]
	fn test_single_call(call: RuntimeCall) -> TransactionValidity {
		validate(call)
	}

	#[test_case(vec![remark_call(), submit_data_call()] =>  to_invalid_tx(UnexpectedSubmitDataCall); "Submit Data call inside a Batch call should be blocked" )]
	#[test_case(vec![remark_call(), send_message_call()] =>  to_invalid_tx(UnexpectedSendMessageCall); "Send Message call inside a Batch call should be blocked" )]
	#[test_case(vec![remark_call(), remark_call()] =>  Ok(ValidTransaction::default()); "Non-Submit-Data and Non-Send-Message call inside a Batch call should be allowed" )]
	fn test_batch_call(calls: Vec<RuntimeCall>) -> TransactionValidity {
		validate(batch_call(calls.clone()))?;
		validate(batch_all_call(calls.clone()))?;
		validate(force_batch_call(calls))
	}
}
