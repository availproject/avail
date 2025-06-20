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

use system_events_api::fetch_events_v1::Params as FetchEventsParams;
use system_events_api::fetch_events_v1::Result as FetchEventsResult;

sp_api::decl_runtime_apis! {
	/// The API to query account nonce.
	pub trait AccountNonceApi<AccountId, Nonce> where
		AccountId: codec::Codec,
		Nonce: codec::Codec,
	{
		/// Get current account nonce of given `AccountId`.
		fn account_nonce(account: AccountId) -> Nonce;
	}

	pub trait SystemEventsApi
	{
		fn fetch_events_v1(params: FetchEventsParams) -> FetchEventsResult;
	}
}

pub mod system_events_api {
	use codec::MaxEncodedLen;
	use codec::{Decode, Encode};
	use scale_info::TypeInfo;
	use sp_std::vec::Vec;

	pub mod fetch_events_v1 {
		use super::*;

		pub const MAX_INDICES_COUNT: u8 = 10;
		pub const ERROR_INVALID_INPUTS: u8 = 1;

		// If any change is done here, `version`` needs to be bumped! This is a breaking change!!
		#[derive(
			scale_info::TypeInfo,
			codec::Decode,
			codec::Encode,
			serde::Serialize,
			serde::Deserialize,
			Clone,
		)]
		pub struct Result {
			pub error: Option<u8>,
			pub groups: Vec<GroupedRuntimeEvents>,
		}

		#[derive(
			Clone,
			Default,
			scale_info::TypeInfo,
			codec::Decode,
			codec::Encode,
			serde::Serialize,
			serde::Deserialize,
		)]
		pub struct Params {
			pub filter_by_tx_indices: Option<Vec<u32>>,
			pub enable_encoding: Option<bool>,
			pub enable_decoding: Option<bool>,
		}

		#[derive(
			scale_info::TypeInfo,
			codec::Decode,
			codec::Encode,
			serde::Serialize,
			serde::Deserialize,
			Clone,
		)]
		pub struct GroupedRuntimeEvents {
			pub phase: RuntimePhase,
			pub events: Vec<RuntimeEvent>,
		}

		impl GroupedRuntimeEvents {
			pub fn new(phase: RuntimePhase) -> Self {
				Self {
					phase,
					events: Vec::new(),
				}
			}
		}

		#[derive(
			Clone,
			scale_info::TypeInfo,
			codec::Decode,
			codec::Encode,
			serde::Serialize,
			serde::Deserialize,
		)]
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

		/// A phase of a block's execution.
		#[derive(
			Debug,
			Encode,
			Decode,
			TypeInfo,
			MaxEncodedLen,
			PartialEq,
			Eq,
			Clone,
			Copy,
			serde::Serialize,
			serde::Deserialize,
		)]
		pub enum RuntimePhase {
			/// Applying an extrinsic.
			ApplyExtrinsic(u32),
			/// Finalizing the block.
			Finalization,
			/// Initializing the block.
			Initialization,
		}

		impl RuntimePhase {
			pub fn tx_index(&self) -> Option<u32> {
				match self {
					RuntimePhase::ApplyExtrinsic(x) => Some(*x),
					_ => None,
				}
			}
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
	}
}
