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
use codec::MaxEncodedLen;
use scale_info::TypeInfo;
use codec::{Encode, Decode};

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
		fn fetch_events(params: SystemFetchEventsParams) -> SystemFetchEventsResult;
	}
}

// If any change is done here, `version`` needs to be bumped! This is a breaking change!!
#[derive(scale_info::TypeInfo, codec::Decode, codec::Encode)]
pub struct SystemFetchEventsResult {
	pub version: u8,
	pub error: u8,
	pub entries: Vec<events::RuntimeEntryEvents>,
}

#[derive(Clone, Default, scale_info::TypeInfo, codec::Decode, codec::Encode)]
pub struct SystemFetchEventsParams {
	pub filter_tx_indices: Option<Vec<u32>>,
	pub enable_encoding: Option<bool>,
	pub enable_decoding: Option<bool>,
}

/// A phase of a block's execution.
#[derive(Debug, Encode, Decode, TypeInfo, MaxEncodedLen, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum RuntimePhase {
	/// Applying an extrinsic.
	ApplyExtrinsic(u32),
	/// Finalizing the block.
	Finalization,
	/// Initializing the block.
	Initialization,
}

impl From<&frame_system::Phase> for RuntimePhase {
	fn from(value: &frame_system::Phase) -> Self { 
		match value {
			frame_system::Phase::ApplyExtrinsic(x) => Self::ApplyExtrinsic(*x),
			frame_system::Phase::Finalization => Self::Finalization,
			frame_system::Phase::Initialization => Self::Initialization,
		}
	}
}

pub mod events {
	use super::*;

	#[derive(scale_info::TypeInfo, codec::Decode, codec::Encode)]
	pub struct RuntimeEntryEvents {
		pub phase: RuntimePhase,
		pub events: Vec<RuntimeEvent>,
	}

	impl RuntimeEntryEvents {
		pub fn new(phase: RuntimePhase) -> Self {
			Self {
				phase,
				events: Vec::new(),
			}
		}
	}

	#[derive(Clone, scale_info::TypeInfo, codec::Decode, codec::Encode)]
	pub struct RuntimeEvent {
		pub index: u32,
		// (Pallet Id, Event Id)
		pub emitted_index: (u8, u8),
		pub encoded: Option<Vec<u8>>,
		pub decoded: Option<Vec<u8>>,
	}

	impl RuntimeEvent {
		pub fn new(
			index: u32,
			emitted_index: (u8, u8),
			encoded: Option<Vec<u8>>,
			decoded: Option<Vec<u8>>,
		) -> Self {
			Self {
				index,
				emitted_index,
				encoded,
				decoded,
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

	pub mod event_id {
		pub mod system {
			pub const PALLET_ID: u8 = 0;
			pub const EXTRINSIC_SUCCESS: u8 = 0;
			pub const EXTRINSIC_FAILED: u8 = 1;
			pub const NEW_ACCOUNT: u8 = 3;
			pub const KILLED_ACCOUNT: u8 = 4;
			pub const REMARKED: u8 = 5;
		}

		pub mod balances {
			pub const PALLET_ID: u8 = 6;
			pub const ENDOWED: u8 = 0;
			pub const DUST_LOST: u8 = 1;
			pub const TRANSFER: u8 = 2;
			pub const RESERVED: u8 = 4;
			pub const UNRESERVED: u8 = 5;
			pub const DEPOSIT: u8 = 7;
			pub const WITHDRAW: u8 = 8;
			pub const LOCKED: u8 = 17;
			pub const UNLOCKED: u8 = 18;
			pub const FROZEN: u8 = 19;
		}

		pub mod sudo {
			pub const PALLET_ID: u8 = 19;
			pub const SUDID: u8 = 0;
			pub const SUDO_AS_DONE: u8 = 3;
		}

		pub mod data_availability {
			pub const PALLET_ID: u8 = 29;
			pub const APPLICATION_KEY_CREATED: u8 = 0;
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

		pub mod scheduler {
			pub const PALLET_ID: u8 = 24;
			pub const DISPATCHED: u8 = 2;
		}
	}
}
