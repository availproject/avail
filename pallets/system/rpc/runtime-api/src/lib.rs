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
	pub decoded_version: u8,
	pub decoded: Vec<events::DecodedTransactionEvents>,
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
		pub value: DecodedEvents,
	}

	impl DecodedTransactionEvents {
		pub fn new(tx_index: u32) -> Self {
			Self {
				tx_index,
				value: DecodedEvents::default(),
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
	#[derive(Debug, Clone, Default, scale_info::TypeInfo, codec::Decode, codec::Encode)]
	pub struct DecodedEvents {
		pub system_extrinsic: Option<bool>,
		pub sudo_sudid: Vec<bool>,
		pub sudo_sudo_as_done: Vec<bool>,
		pub multisig_executed: Vec<bool>,
		pub proxy_executed: Vec<bool>,
		pub data_availability_data_submitted: Vec<DataSubmittedEvent>,
	}
}
