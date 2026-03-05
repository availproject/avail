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

use system_events_api::fetch_events_v1::ApiResult as FetchEventsResult;
use system_events_api::fetch_events_v1::Options as FetchEventsOptions;

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
		fn fetch_events_v1(options: FetchEventsOptions) -> FetchEventsResult;
	}
}

pub mod system_events_api {
	use sp_std::vec::Vec;

	pub mod fetch_events_v1 {
		use super::*;

		pub const MAX_INDICES_COUNT: usize = 30;
		pub const ERROR_INVALID_INPUTS: u8 = 1;

		pub type ApiResult = Result<Vec<GroupedRuntimeEvents>, u8>;

		#[derive(Clone, Default, scale_info::TypeInfo, codec::Decode, codec::Encode)]
		#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
		pub struct Options {
			pub filter: Option<Filter>,
			pub enable_encoding: Option<bool>,
			pub enable_decoding: Option<bool>,
		}

		#[derive(Clone, scale_info::TypeInfo, codec::Decode, codec::Encode)]
		#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
		#[repr(u8)]
		pub enum Filter {
			All = 0,
			OnlyExtrinsics = 1,
			OnlyNonExtrinsics = 2,
			Only(Vec<u32>) = 3,
		}

		impl Default for Filter {
			fn default() -> Self {
				Self::All
			}
		}

		impl Filter {
			pub fn is_valid(&self) -> bool {
				match self {
					Self::Only(list) => list.len() <= MAX_INDICES_COUNT,
					_ => true,
				}
			}

			pub fn should_allow(&self, phase: frame_system::Phase) -> bool {
				let tx_index = match phase {
					frame_system::Phase::ApplyExtrinsic(x) => Some(x),
					_ => None,
				};

				match self {
					Self::All => true,
					Self::OnlyExtrinsics => tx_index.is_some(),
					Self::OnlyNonExtrinsics => tx_index.is_none(),
					Self::Only(list) => {
						let Some(tx_index) = tx_index else {
							return false;
						};

						list.contains(&tx_index)
					},
				}
			}
		}

		#[derive(scale_info::TypeInfo, codec::Decode, codec::Encode, Clone)]
		pub struct GroupedRuntimeEvents {
			pub phase: frame_system::Phase,
			pub events: Vec<RuntimeEvent>,
		}

		impl GroupedRuntimeEvents {
			pub fn new(phase: frame_system::Phase) -> Self {
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
	}
}
