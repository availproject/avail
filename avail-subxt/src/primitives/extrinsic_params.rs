use codec::{Compact, Decode, Encode, Error, Input, Output};
use serde::{Deserialize, Serialize};
use subxt::{
	config::{extrinsic_params::Era, ExtrinsicParams},
	utils::H256,
};

use crate::api::runtime_types::avail_core::AppId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvailExtrinsicParams {
	pub spec_version: u32,
	pub tx_version: u32,
	pub nonce: Compact<u32>,
	pub mortality: Era,
	pub genesis_hash: H256,
	pub tip: Compact<u128>,
	pub app_id: AppId,
}

impl ExtrinsicParams<u32, H256> for AvailExtrinsicParams {
	type OtherParams = AvailExtrinsicParams;

	fn new(
		spec_version: u32,
		tx_version: u32,
		nonce: u32,
		genesis_hash: H256,
		other_params: Self::OtherParams,
	) -> Self {
		Self {
			spec_version,
			tx_version,
			nonce: nonce.into(),
			mortality: other_params.mortality,
			genesis_hash,
			tip: other_params.tip,
			app_id: other_params.app_id,
		}
	}

	fn encode_extra_to(&self, v: &mut Vec<u8>) {
		(self.mortality, self.nonce, self.tip, self.app_id).encode_to(v);
	}

	fn encode_additional_to(&self, v: &mut Vec<u8>) {
		(
			self.spec_version,
			self.tx_version,
			self.genesis_hash,
			self.genesis_hash,
		)
			.encode_to(v);
	}
}

impl Default for AvailExtrinsicParams {
	fn default() -> Self {
		Self {
			spec_version: Default::default(),
			tx_version: Default::default(),
			nonce: 0u32.into(),
			mortality: Era::Immortal,
			genesis_hash: Default::default(),
			tip: 0u128.into(),
			app_id: Default::default(),
		}
	}
}
impl AvailExtrinsicParams {
	/// Create params with the addition of tip and app_id
	pub fn new_with_tip_and_app_id(tip: u128, app_id: AppId) -> Self {
		Self {
			tip: tip.into(),
			app_id,
			..Default::default()
		}
	}

	/// Create params with the addition of app_id
	pub fn new_with_app_id(app_id: AppId) -> Self {
		Self {
			app_id,
			..Default::default()
		}
	}
}

impl Encode for AvailExtrinsicParams {
	fn encode_to<T: Output + ?Sized>(&self, dest: &mut T) {
		// CheckMortality, CheckNonce, ChargeTransationPayment, CheckAppId
		(self.mortality, self.nonce, self.tip, self.app_id).encode_to(dest);
	}
}

impl Decode for AvailExtrinsicParams {
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
		let (mortality, nonce, tip, app_id) =
			<(Era, Compact<u32>, Compact<u128>, AppId)>::decode(input)?;
		Ok(Self {
			mortality,
			nonce,
			tip,
			app_id,
			..Default::default()
		})
	}
}
