use codec::{Compact, Encode};
use subxt::{
	ext::sp_core::H256,
	tx::{Era, ExtrinsicParams, PlainTip},
};

use crate::api::runtime_types::da_primitives::asdr::AppId;

#[derive(Debug, Clone)]
pub struct AvailExtrinsicParams {
	pub spec_version: u32,
	pub tx_version: u32,
	pub nonce: u32,
	pub mortality: Era,
	pub genesis_hash: H256,
	pub tip: PlainTip,
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
			nonce,
			mortality: other_params.mortality,
			genesis_hash,
			tip: other_params.tip,
			app_id: other_params.app_id,
		}
	}

	fn encode_extra_to(&self, v: &mut Vec<u8>) {
		(self.mortality, Compact(self.nonce), self.tip, self.app_id).encode_to(v);
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
			nonce: Default::default(),
			mortality: Era::Immortal,
			genesis_hash: Default::default(),
			tip: Default::default(),
			app_id: Default::default(),
		}
	}
}
impl AvailExtrinsicParams {
	/// Create params with the addition of tip and app_id
	pub fn new_with_tip_and_app_id(tip: u128, app_id: AppId) -> Self {
		Self {
			tip: PlainTip::new(tip),
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
