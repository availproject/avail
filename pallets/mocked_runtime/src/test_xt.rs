use codec::{Codec, Decode, Encode};
use avail_core::{AppId, traits::GetAppId};
use derive_more::From;
use frame_support::{dispatch, traits::ExtrinsicCall};
use scale_info::TypeInfo;
use serde::Serializer;
use sp_runtime::{
	testing::TestXt as SPTestXt,
	traits::{
		Applyable, Checkable, DispatchInfoOf, Dispatchable, Extrinsic, ExtrinsicMetadata,
		PostDispatchInfoOf, SignedExtension, ValidateUnsigned,
	},
	transaction_validity::{TransactionSource, TransactionValidity, TransactionValidityError},
	ApplyExtrinsicResultWithInfo, Serialize,
};
use sp_std::fmt::{self, Debug};

/// Test transaction wrapper with AppId.
#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, From)]
pub struct TestXt<Call, Extra>(SPTestXt<Call, Extra>);

impl<Call, Extra> TestXt<Call, Extra> {
	/// Create a new `TextXt`.
	pub fn new(call: Call, signature: Option<(u64, Extra)>) -> Self {
		Self(SPTestXt::<Call, Extra>::new(call, signature))
	}
}

// Non-opaque extrinsics always 0.
parity_util_mem::malloc_size_of_is_0!(any: TestXt<Call, Extra>);

impl<Call, Extra> Serialize for TestXt<Call, Extra>
where
	TestXt<Call, Extra>: Encode,
{
	fn serialize<S>(&self, seq: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		self.using_encoded(|bytes| seq.serialize_bytes(bytes))
	}
}

impl<Call, Extra> Debug for TestXt<Call, Extra> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.0.fmt(f) }
}

impl<Call: Codec + Sync + Send, Context, Extra> Checkable<Context> for TestXt<Call, Extra> {
	type Checked = Self;

	fn check(self, _: &Context) -> Result<Self::Checked, TransactionValidityError> { Ok(self) }

	#[cfg(feature = "try-runtime")]
	fn unchecked_into_checked_i_know_what_i_am_doing(
		self,
		_: &Context,
	) -> Result<Self::Checked, TransactionValidityError> {
		unreachable!()
	}
}

impl<Call: Codec + Sync + Send, Extra> Extrinsic for TestXt<Call, Extra> {
	type Call = Call;
	type SignaturePayload = (u64, Extra);

	fn is_signed(&self) -> Option<bool> { self.0.is_signed() }

	fn new(c: Call, sig: Option<Self::SignaturePayload>) -> Option<Self> {
		let sp_test = SPTestXt::<Call, Extra>::new(c, sig);
		Some(Self(sp_test))
	}
}

impl<Call, Extra> ExtrinsicMetadata for TestXt<Call, Extra>
where
	Call: Codec + Sync + Send,
	Extra: SignedExtension<AccountId = u64, Call = Call>,
{
	type SignedExtensions = Extra;

	const VERSION: u8 = 0u8;
}

impl<Origin, Call, Extra> Applyable for TestXt<Call, Extra>
where
	Call: 'static
		+ Sized
		+ Send
		+ Sync
		+ Clone
		+ Eq
		+ Codec
		+ Debug
		+ Dispatchable<RuntimeOrigin = Origin>,
	Extra: SignedExtension<AccountId = u64, Call = Call>,
	Origin: From<Option<u64>>,
{
	type Call = Call;

	/// Checks to see if this is a valid *transaction*. It returns information on it if so.
	fn validate<U: ValidateUnsigned<Call = Self::Call>>(
		&self,
		source: TransactionSource,
		info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> TransactionValidity {
		self.0.validate::<U>(source, info, len)
	}

	/// Executes all necessary logic needed prior to dispatch and deconstructs into function call,
	/// index and sender.
	fn apply<U: ValidateUnsigned<Call = Self::Call>>(
		self,
		info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> ApplyExtrinsicResultWithInfo<PostDispatchInfoOf<Self::Call>> {
		self.0.apply::<U>(info, len)
	}
}

impl<Call, Extra> ExtrinsicCall for TestXt<Call, Extra>
where
	Call: Codec + Sync + Send,
{
	fn call(&self) -> &Self::Call { self.0.call() }
}

impl<Call, Extra> GetAppId for TestXt<Call, Extra> {
	fn app_id(&self) -> AppId { AppId::default() }
}

impl<Call: dispatch::GetDispatchInfo, Extra> dispatch::GetDispatchInfo for TestXt<Call, Extra> {
	fn get_dispatch_info(&self) -> dispatch::DispatchInfo { self.0.call.get_dispatch_info() }
}
