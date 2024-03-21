use codec::{Decode, Encode};
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

pub mod extrinsic_params;
pub use extrinsic_params::{
	new_params_from_app_id, CheckAppId, Extra, ExtrinsicParams, OnlyCodecExtra,
};

pub mod header;
pub use header::Header;

pub mod babe;
pub mod grandpa;

pub mod app_unchecked_extrinsic;
pub use app_unchecked_extrinsic::AppUncheckedExtrinsic;

/// Compatible with `kate::com::Cell`
#[derive(Clone, Constructor, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Cell {
	#[codec(compact)]
	pub row: u32,
	#[codec(compact)]
	pub col: u32,
}

impl<R, C> From<(R, C)> for Cell
where
	R: Into<u32>,
	C: Into<u32>,
{
	fn from((row, col): (R, C)) -> Self {
		Self {
			row: row.into(),
			col: col.into(),
		}
	}
}

use crate::{api::runtime_types::avail_core::data_proof::message::Message, BoundedVec};
use avail_core::data_proof::Message as CMessage;

impl From<CMessage> for Message {
	fn from(m: CMessage) -> Self {
		match m {
			CMessage::ArbitraryMessage(data) => {
				Message::ArbitraryMessage(BoundedVec(data.into_inner()))
			},
			CMessage::FungibleToken { asset_id, amount } => {
				Message::FungibleToken { asset_id, amount }
			},
		}
	}
}
