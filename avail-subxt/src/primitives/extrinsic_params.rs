use crate::{api::runtime_types::sp_runtime::generic::era::Era, AppId, AvailConfig};

use codec::{Compact, Encode};
use scale_info::PortableRegistry;
use subxt::{
	client::OfflineClientT,
	config::{
		signed_extensions::{
			AnyOf, ChargeTransactionPayment, CheckGenesis, CheckMortality, CheckNonce,
			CheckSpecVersion, CheckTxVersion,
		},
		ExtrinsicParamsEncoder, ExtrinsicParamsError, SignedExtension,
	},
	Config,
};

/// Type used only for decoding extrinsic from blocks.
pub type OnlyCodecExtra = (
	(),            // CheckNonZeroSender,
	(),            // CheckSpecVersion<Runtime>,
	(),            // CheckTxVersion<Runtime>,
	(),            // CheckGenesis<Runtime>,
	Era,           // CheckEra<Runtime>,
	Compact<u32>,  // CheckNonce<Runtime>,
	(),            // CheckWeight<Runtime>,
	Compact<u128>, // ChargeTransactionPayment<Runtime>,
	AppId,         // CheckAppId<Runtime>,
);

pub type Extra = (
	// CheckNonZeroSender,
	CheckSpecVersion,
	CheckTxVersion,
	CheckGenesis<AvailConfig>,
	CheckMortality<AvailConfig>,
	CheckNonce,
	ChargeTransactionPayment,
	CheckAppId,
);

pub type ExtrinsicParams = AnyOf<AvailConfig, Extra>;
pub type OtherParams =
	<ExtrinsicParams as subxt::config::ExtrinsicParams<AvailConfig>>::OtherParams;

pub fn new_params_from_app_id<Id: Into<AppId>>(id: Id) -> OtherParams {
	(
		(),
		(),
		(),
		Default::default(),
		(),
		Default::default(),
		id.into(),
	)
}

pub struct CheckAppId(pub AppId);

impl<T: Config> SignedExtension<T> for CheckAppId {
	type Decoded = Compact<u32>;

	fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
		identifier == "CheckAppId"
	}
}

impl<T: Config> subxt::config::ExtrinsicParams<T> for CheckAppId {
	type OtherParams = AppId;

	fn new<Client: OfflineClientT<T>>(
		_nonce: u64,
		_client: Client,
		id: Self::OtherParams,
	) -> Result<Self, ExtrinsicParamsError> {
		Ok(CheckAppId(id))
	}
}

impl ExtrinsicParamsEncoder for CheckAppId {
	fn encode_extra_to(&self, v: &mut Vec<u8>) {
		Compact::<u32>(self.0 .0).encode_to(v);
	}

	fn encode_additional_to(&self, _: &mut Vec<u8>) {}
}
