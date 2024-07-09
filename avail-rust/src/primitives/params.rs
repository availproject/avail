// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::{Compact, Encode};
use scale_info::PortableRegistry;
use subxt::client::ClientState;
use subxt::config::signed_extensions::CheckNonceParams;
use subxt::config::{
	signed_extensions, Config, ExtrinsicParams, ExtrinsicParamsEncoder, Header, RefineParams,
	SignedExtension,
};
use subxt::error::ExtrinsicParamsError;

use crate::AppId;

pub struct CheckAppId(pub AppId);

impl<T: Config> SignedExtension<T> for CheckAppId {
	type Decoded = Compact<u32>;

	fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
		identifier == "CheckAppId"
	}
}

impl<T: Config> RefineParams<T> for AppId {}
impl<T: Config> RefineParams<T> for CheckAppId {}

impl<T: Config> subxt::config::ExtrinsicParams<T> for CheckAppId {
	type Params = AppId;

	fn new(_client: &ClientState<T>, id: Self::Params) -> Result<Self, ExtrinsicParamsError> {
		Ok(CheckAppId(id))
	}
}

impl ExtrinsicParamsEncoder for CheckAppId {
	fn encode_extra_to(&self, v: &mut Vec<u8>) {
		Compact::<u32>(self.0 .0 .0).encode_to(v);
	}

	fn encode_additional_to(&self, _: &mut Vec<u8>) {}
}

/// The default [`super::ExtrinsicParams`] implementation understands common signed extensions
/// and how to apply them to a given chain.
pub type DefaultExtrinsicParams<T> = signed_extensions::AnyOf<
	T,
	(
		signed_extensions::CheckSpecVersion,
		signed_extensions::CheckTxVersion,
		signed_extensions::CheckGenesis<T>,
		signed_extensions::CheckMortality<T>,
		signed_extensions::CheckNonce,
		signed_extensions::ChargeTransactionPayment,
		CheckAppId,
	),
>;

/// A builder that outputs the set of [`super::ExtrinsicParams::Params`] required for
/// [`DefaultExtrinsicParams`]. This may expose methods that aren't applicable to the current
/// chain; such values will simply be ignored if so.
pub struct DefaultExtrinsicParamsBuilder<T: Config> {
	/// `None` means the tx will be immortal.
	mortality: Option<Mortality<T::Hash>>,
	/// `None` means the nonce will be automatically set.
	nonce: Option<u64>,
	tip: u128,
	app_id: AppId,
}

struct Mortality<Hash> {
	/// Block hash that mortality starts from
	checkpoint_hash: Hash,
	/// Block number that mortality starts from (must
	// point to the same block as the hash above)
	checkpoint_number: u64,
	/// How many blocks the tx is mortal for
	period: u64,
}

impl<T: Config> Default for DefaultExtrinsicParamsBuilder<T> {
	fn default() -> Self {
		Self {
			mortality: None,
			tip: 0,
			nonce: None,
			app_id: AppId(avail_core::AppId(0)),
		}
	}
}

impl<T: Config> DefaultExtrinsicParamsBuilder<T> {
	/// Configure new extrinsic params. We default to providing no tip
	/// and using an immortal transaction unless otherwise configured
	pub fn new() -> Self {
		Default::default()
	}

	/// Make the transaction mortal, given a block header that it should be mortal from,
	/// and the number of blocks (roughly; it'll be rounded to a power of two) that it will
	/// be mortal for.
	pub fn mortal(mut self, from_block: &T::Header, for_n_blocks: u64) -> Self {
		self.mortality = Some(Mortality {
			checkpoint_hash: from_block.hash(),
			checkpoint_number: from_block.number().into(),
			period: for_n_blocks,
		});
		self
	}

	/// Provide a specific nonce for the submitter of the extrinsic
	pub fn nonce(mut self, nonce: u64) -> Self {
		self.nonce = Some(nonce);
		self
	}

	/// App Id
	pub fn app_id(mut self, app_id: u32) -> Self {
		self.app_id = AppId(avail_core::AppId(app_id));
		self
	}

	/// Make the transaction mortal, given a block number and block hash (which must both point to
	/// the same block) that it should be mortal from, and the number of blocks (roughly; it'll be
	/// rounded to a power of two) that it will be mortal for.
	///
	/// Prefer to use [`DefaultExtrinsicParamsBuilder::mortal()`], which ensures that the block hash
	/// and number align.
	pub fn mortal_unchecked(
		mut self,
		from_block_number: u64,
		from_block_hash: T::Hash,
		for_n_blocks: u64,
	) -> Self {
		self.mortality = Some(Mortality {
			checkpoint_hash: from_block_hash,
			checkpoint_number: from_block_number,
			period: for_n_blocks,
		});
		self
	}

	/// Provide a tip to the block author in the chain's native token.
	pub fn tip(mut self, tip: u128) -> Self {
		self.tip = tip;
		self
	}

	/// Build the extrinsic parameters.
	pub fn build(self) -> <DefaultExtrinsicParams<T> as ExtrinsicParams<T>>::Params {
		let check_mortality_params = if let Some(mortality) = self.mortality {
			signed_extensions::CheckMortalityParams::mortal(
				mortality.period,
				mortality.checkpoint_number,
				mortality.checkpoint_hash,
			)
		} else {
			signed_extensions::CheckMortalityParams::immortal()
		};

		let charge_transaction_params =
			signed_extensions::ChargeTransactionPaymentParams::tip(self.tip);

		let check_nonce_params = CheckNonceParams(self.nonce);

		(
			(),
			(),
			(),
			check_mortality_params,
			check_nonce_params,
			charge_transaction_params,
			self.app_id,
		)
	}
}
