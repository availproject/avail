// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Runtime API definition required by System RPC extensions.
//!
//! This API should be imported and implemented by the runtime,
//! of a node that wants to use the custom RPC extension
//! adding System access methods.

#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
	/// The API to query account nonce.
	pub trait AccountNonceApi<AccountId, Nonce> where
		AccountId: codec::Codec,
		Nonce: codec::Codec,
	{
		/// Get current account nonce of given `AccountId`.
		fn account_nonce(account: AccountId) -> Nonce;
	}

	#[api_version(1)]
	pub trait SystemEventsApi {
		fn fetch_transaction_success_status() -> Vec<TransactionSuccessStatus>;
		fn fetch_events(tx_index: Vec<u32>, enable_decoding: bool) -> SystemFetchEventsResult;
	}
}

#[derive(Debug, Clone, Copy, scale_info::TypeInfo, codec::Decode, codec::Encode)]
pub struct TransactionSuccessStatus {
	pub tx_index: u32,
	pub tx_success: bool,
}

// If any change is done here, `version`` needs to be bumped! This is a breaking change!!
#[derive(Debug, Clone, scale_info::TypeInfo, codec::Decode, codec::Encode)]
pub struct SystemFetchEventsResult {
	pub version: u8,
	pub error: u8,
	pub last_tx_index: u32,
	pub encoded: Vec<events::EncodedTransactionEvents>,
	pub decoded: Vec<events::DecodedTransactionEvents>,
}

#[derive(Debug, Clone, scale_info::TypeInfo, codec::Decode, codec::Encode)]
pub struct SystemFetchEventsParams {
	pub tx_indices: Option<Vec<u32>>,
	pub enable_decoding: Option<bool>,
}

pub mod events {
	// If any change is done here, `version`` needs to be bumped! This is a breaking change!!
	#[derive(Debug, Clone, scale_info::TypeInfo, codec::Decode, codec::Encode)]
	pub struct EncodedTransactionEvents {
		pub tx_index: u32,
		pub value: Vec<Vec<u8>>,
	}

	impl EncodedTransactionEvents {
		pub fn new(tx_index: u32) -> Self {
			Self {
				tx_index,
				value: Vec::new(),
			}
		}
	}

	// If any change is done here, `decoded_version` needs to be bumped! This is a breaking change!!
	#[derive(Debug, Clone, scale_info::TypeInfo, codec::Decode, codec::Encode)]
	pub struct DecodedTransactionEvents {
		pub tx_index: u32,
		pub events: Vec<SemiDecodedEvent>,
	}

	impl DecodedTransactionEvents {
		pub fn new(tx_index: u32) -> Self {
			Self {
				tx_index,
				events: Vec::default(),
			}
		}
	}

	// If any change is done here, `decoded_version` needs to be bumped! This is a breaking change!!
	#[derive(Debug, Clone, scale_info::TypeInfo, codec::Decode, codec::Encode)]
	pub struct DataSubmittedEvent {
		pub who: Vec<u8>,
		pub data_hash: Vec<u8>,
	}

	impl DataSubmittedEvent {
		pub fn new(who: Vec<u8>, data_hash: Vec<u8>) -> Self {
			Self { who, data_hash }
		}
	}

	// If any change is done here, `decoded_version` needs to be bumped! This is a breaking change!!
	#[derive(Debug, Clone, scale_info::TypeInfo, codec::Decode, codec::Encode)]
	pub struct SemiDecodedEvent {
		pub index: u32,
		pub pallet_id: u8,
		pub event_id: u8,
		pub event_data: Vec<u8>,
	}

	impl SemiDecodedEvent {
		pub fn new(index: u32, pallet_id: u8, event_id: u8, event_data: Vec<u8>) -> Self {
			Self {
				index,
				pallet_id,
				event_id,
				event_data,
			}
		}
	}

	pub mod event_id {
		pub mod system {
			pub const PALLET_ID: u8 = 0;
			pub const EXTRINSIC_SUCCESS: u8 = 0;
			pub const EXTRINSIC_FAILED: u8 = 1;
		}

		pub mod sudo {
			pub const PALLET_ID: u8 = 19;
			pub const SUDID: u8 = 0;
			pub const SUDO_AS_DONE: u8 = 3;
		}

		pub mod data_availability {
			pub const PALLET_ID: u8 = 29;
			pub const DATA_SUBMITTED: u8 = 1;
		}

		pub mod multisig {
			pub const PALLET_ID: u8 = 34;
			pub const MULTISIG_EXECUTED: u8 = 2;
		}

		pub mod proxy {
			pub const PALLET_ID: u8 = 40;
			pub const PROXY_EXECUTED: u8 = 0;
		}
	}
}
