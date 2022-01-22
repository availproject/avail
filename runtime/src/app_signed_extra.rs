use codec::{Decode, Encode};
use da_control::{CheckAppId, Config as DataAvailability};
use da_primitives::asdr::{AppId, GetAppId};
use frame_support::weights::{DispatchInfo, PostDispatchInfo};
use frame_system::{
	CheckEra, CheckGenesis, CheckNonce, CheckSpecVersion, CheckTxVersion, CheckWeight,
	Config as System,
};
use pallet_transaction_payment::{
	ChargeTransactionPayment, Config as TransactionPayment, OnChargeTransaction,
};
use scale_info::TypeInfo;
use sp_runtime::{
	generic::{Era, Phase},
	traits::{
		DispatchInfoOf, Dispatchable, PostDispatchInfoOf, SignedExtension, SignedExtensionMetadata,
	},
	transaction_validity::{TransactionValidity, TransactionValidityError, ValidTransaction},
	DispatchResult, FixedPointOperand,
};
use sp_std::{fmt::Debug, vec, vec::Vec};

type BalanceOf<T> =
	<<T as TransactionPayment>::OnChargeTransaction as OnChargeTransaction<T>>::Balance;
type IndexOf<T> = <T as System>::Index;

/// Era period
pub type Period = u64;

#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct AppSignedExtra<T: System + DataAvailability + TransactionPayment + Send + Sync> {
	pub check_spec_version: CheckSpecVersion<T>,
	pub check_tx_version: CheckTxVersion<T>,
	pub check_genesis: CheckGenesis<T>,
	pub check_era: CheckEra<T>,
	pub check_nonce: CheckNonce<T>,
	pub check_weight: CheckWeight<T>,
	pub check_app_id: CheckAppId<T>,
	pub charge_tx_payment: ChargeTransactionPayment<T>,
}

impl<T> AppSignedExtra<T>
where
	T: System + DataAvailability + TransactionPayment + Send + Sync,
	T::Call: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
	BalanceOf<T>: Send + Sync + FixedPointOperand,
{
	pub fn new(
		period: Period,
		best_block: Phase,
		nonce: IndexOf<T>,
		tip: BalanceOf<T>,
		app_id: AppId,
	) -> Self {
		Self {
			check_spec_version: CheckSpecVersion::<T>::new(),
			check_tx_version: CheckTxVersion::<T>::new(),
			check_genesis: CheckGenesis::<T>::new(),
			check_era: CheckEra::<T>::from(Era::mortal(period, best_block)),
			check_nonce: CheckNonce::<T>::from(nonce),
			check_weight: CheckWeight::<T>::new(),
			check_app_id: CheckAppId::<T>::from(app_id),
			charge_tx_payment: ChargeTransactionPayment::<T>::from(tip),
		}
	}
}

impl<T> GetAppId<AppId> for AppSignedExtra<T>
where
	T: System + DataAvailability + TransactionPayment + Send + Sync,
{
	#[inline]
	fn app_id(&self) -> AppId { self.check_app_id.app_id() }
}

impl<T> Debug for AppSignedExtra<T>
where
	T: System + DataAvailability + TransactionPayment + Send + Sync,
{
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		write!(f, "AppSignedExtra")
	}

	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result { Ok(()) }
}

impl<T> SignedExtension for AppSignedExtra<T>
where
	T: System + DataAvailability + TransactionPayment + Send + Sync,
	BalanceOf<T>: Send + Sync + From<u64> + FixedPointOperand,
	T::Call: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
{
	type AccountId = T::AccountId;
	type AdditionalSigned = (
		<CheckSpecVersion<T> as SignedExtension>::AdditionalSigned,
		<CheckTxVersion<T> as SignedExtension>::AdditionalSigned,
		<CheckGenesis<T> as SignedExtension>::AdditionalSigned,
		<CheckEra<T> as SignedExtension>::AdditionalSigned,
		<CheckNonce<T> as SignedExtension>::AdditionalSigned,
		<CheckWeight<T> as SignedExtension>::AdditionalSigned,
		<CheckAppId<T> as SignedExtension>::AdditionalSigned,
		<ChargeTransactionPayment<T> as SignedExtension>::AdditionalSigned,
	);
	type Call = T::Call;
	type Pre = (
		<CheckSpecVersion<T> as SignedExtension>::Pre,
		<CheckTxVersion<T> as SignedExtension>::Pre,
		<CheckGenesis<T> as SignedExtension>::Pre,
		<CheckEra<T> as SignedExtension>::Pre,
		<CheckNonce<T> as SignedExtension>::Pre,
		<CheckWeight<T> as SignedExtension>::Pre,
		<CheckAppId<T> as SignedExtension>::Pre,
		<ChargeTransactionPayment<T> as SignedExtension>::Pre,
	);

	const IDENTIFIER: &'static str = "You should call `identifier()`!";

	fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
		Ok((
			self.check_spec_version.additional_signed()?,
			self.check_tx_version.additional_signed()?,
			self.check_genesis.additional_signed()?,
			self.check_era.additional_signed()?,
			self.check_nonce.additional_signed()?,
			self.check_weight.additional_signed()?,
			self.check_app_id.additional_signed()?,
			self.charge_tx_payment.additional_signed()?,
		))
	}

	fn validate(
		&self,
		who: &Self::AccountId,
		call: &Self::Call,
		info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> TransactionValidity {
		let valid = ValidTransaction::default();
		let valid = valid.combine_with(self.check_spec_version.validate(who, call, info, len)?);
		let valid = valid.combine_with(self.check_tx_version.validate(who, call, info, len)?);
		let valid = valid.combine_with(self.check_genesis.validate(who, call, info, len)?);
		let valid = valid.combine_with(self.check_era.validate(who, call, info, len)?);
		let valid = valid.combine_with(self.check_nonce.validate(who, call, info, len)?);
		let valid = valid.combine_with(self.check_weight.validate(who, call, info, len)?);
		let valid = valid.combine_with(self.check_app_id.validate(who, call, info, len)?);
		let valid = valid.combine_with(self.charge_tx_payment.validate(who, call, info, len)?);

		Ok(valid)
	}

	fn pre_dispatch(
		self,
		who: &Self::AccountId,
		call: &Self::Call,
		info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> Result<Self::Pre, TransactionValidityError> {
		Ok((
			self.check_spec_version.pre_dispatch(who, call, info, len)?,
			self.check_tx_version.pre_dispatch(who, call, info, len)?,
			self.check_genesis.pre_dispatch(who, call, info, len)?,
			self.check_era.pre_dispatch(who, call, info, len)?,
			self.check_nonce.pre_dispatch(who, call, info, len)?,
			self.check_weight.pre_dispatch(who, call, info, len)?,
			self.check_app_id.pre_dispatch(who, call, info, len)?,
			self.charge_tx_payment.pre_dispatch(who, call, info, len)?,
		))
	}

	fn validate_unsigned(
		call: &Self::Call,
		info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> TransactionValidity {
		let valid = ValidTransaction::default();

		let valid = valid.combine_with(CheckSpecVersion::<T>::validate_unsigned(call, info, len)?);
		let valid = valid.combine_with(CheckTxVersion::<T>::validate_unsigned(call, info, len)?);
		let valid = valid.combine_with(CheckGenesis::<T>::validate_unsigned(call, info, len)?);
		let valid = valid.combine_with(CheckEra::<T>::validate_unsigned(call, info, len)?);
		let valid = valid.combine_with(CheckNonce::<T>::validate_unsigned(call, info, len)?);
		let valid = valid.combine_with(CheckWeight::<T>::validate_unsigned(call, info, len)?);
		let valid = valid.combine_with(CheckAppId::<T>::validate_unsigned(call, info, len)?);
		let valid = valid.combine_with(ChargeTransactionPayment::<T>::validate_unsigned(
			call, info, len,
		)?);

		Ok(valid)
	}

	fn pre_dispatch_unsigned(
		call: &Self::Call,
		info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> Result<Self::Pre, TransactionValidityError> {
		Ok((
			CheckSpecVersion::<T>::pre_dispatch_unsigned(call, info, len)?,
			CheckTxVersion::<T>::pre_dispatch_unsigned(call, info, len)?,
			CheckGenesis::<T>::pre_dispatch_unsigned(call, info, len)?,
			CheckEra::<T>::pre_dispatch_unsigned(call, info, len)?,
			CheckNonce::<T>::pre_dispatch_unsigned(call, info, len)?,
			CheckWeight::<T>::pre_dispatch_unsigned(call, info, len)?,
			CheckAppId::<T>::pre_dispatch_unsigned(call, info, len)?,
			ChargeTransactionPayment::<T>::pre_dispatch_unsigned(call, info, len)?,
		))
	}

	fn post_dispatch(
		pre: Self::Pre,
		info: &DispatchInfoOf<Self::Call>,
		post_info: &PostDispatchInfoOf<Self::Call>,
		len: usize,
		result: &DispatchResult,
	) -> Result<(), TransactionValidityError> {
		CheckSpecVersion::<T>::post_dispatch(pre.0, info, post_info, len, result)?;
		CheckTxVersion::<T>::post_dispatch(pre.1, info, post_info, len, result)?;
		CheckGenesis::<T>::post_dispatch(pre.2, info, post_info, len, result)?;
		CheckEra::<T>::post_dispatch(pre.3, info, post_info, len, result)?;
		CheckNonce::<T>::post_dispatch(pre.4, info, post_info, len, result)?;
		CheckWeight::<T>::post_dispatch(pre.5, info, post_info, len, result)?;
		CheckAppId::<T>::post_dispatch(pre.6, info, post_info, len, result)?;
		ChargeTransactionPayment::<T>::post_dispatch(pre.7, info, post_info, len, result)?;

		Ok(())
	}

	fn metadata() -> Vec<SignedExtensionMetadata> {
		vec![
			CheckSpecVersion::<T>::metadata(),
			CheckTxVersion::<T>::metadata(),
			CheckGenesis::<T>::metadata(),
			CheckEra::<T>::metadata(),
			CheckNonce::<T>::metadata(),
			CheckWeight::<T>::metadata(),
			CheckAppId::<T>::metadata(),
			ChargeTransactionPayment::<T>::metadata(),
		]
		.into_iter()
		.flatten()
		.collect()
	}
}
