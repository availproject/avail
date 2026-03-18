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

//! Contains providers for inherents required for empty block production.

use std::{sync::Arc, time::Duration};

use parity_scale_codec::Encode;
use sp_consensus_aura::{Slot, SlotDuration, AURA_ENGINE_ID};
use sp_consensus_babe::{
    digests::{PreDigest, SecondaryPlainPreDigest},
    BABE_ENGINE_ID,
};
use sp_inherents::InherentData;
use sp_runtime::{
    traits::{Block as BlockT, HashingFor},
    Digest, DigestItem,
};
use sp_state_machine::TestExternalities;
use sp_std::prelude::*;
use strum_macros::{Display, EnumIter};
use tokio::sync::Mutex;

use crate::common::empty_block::inherents::custom_idps;

/// Trait for providing the inherent data and digest items for block construction.
pub trait InherentProvider<B: BlockT> {
    type Err;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_parent_info: Option<(InherentData, Digest)>,
        parent_header: B::Header,
        ext: Arc<Mutex<TestExternalities<HashingFor<B>>>>,
        relay_parent_offset: u32,
    ) -> InherentProviderResult<Self::Err>;
}

// Clippy asks that we abstract the return type because it's so long
type InherentProviderResult<Err> =
    Result<(Box<dyn sp_inherents::InherentDataProvider>, Vec<DigestItem>), Err>;

/// Classes of [`InherentProvider`] avaliable.
///
/// Currently only Smart is implemented. New implementations may be added if Smart is not suitable
/// for some edge cases.
#[derive(Debug, Clone, EnumIter, Display, Copy)]
pub enum ProviderVariant {
    /// Smart chain varient will automatically adjust provided inherents based on the given
    /// externalities.
    ///
    /// The blocktime is provided in milliseconds.
    Smart(core::time::Duration),
}

impl<B: BlockT> InherentProvider<B> for ProviderVariant {
    type Err = String;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_parent_info: Option<(InherentData, Digest)>,
        parent_header: B::Header,
        ext: Arc<Mutex<TestExternalities<HashingFor<B>>>>,
        relay_parent_offset: u32,
    ) -> InherentProviderResult<Self::Err> {
        match *self {
            ProviderVariant::Smart(blocktime) => {
                <SmartInherentProvider as InherentProvider<B>>::get_inherent_providers_and_pre_digest(&SmartInherentProvider {
                     blocktime,
                 }, maybe_parent_info, parent_header, ext, relay_parent_offset)
            }
        }
    }
}

/// Attempts to provide inherents in a fashion that works for as many chains as possible.
///
/// It is currently tested for standalone chains that only require timestamp inherents plus
/// consensus digests.
///
/// If it does not work for your Substrate-based chain, [please open an issue](https://github.com/paritytech/try-runtime-cli/issues)
/// and we will look into supporting it.
struct SmartInherentProvider {
    blocktime: Duration,
}

impl<B: BlockT> InherentProvider<B> for SmartInherentProvider {
    type Err = String;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_parent_info: Option<(InherentData, Digest)>,
        parent_header: B::Header,
        ext: Arc<Mutex<TestExternalities<HashingFor<B>>>>,
        relay_parent_offset: u32,
    ) -> InherentProviderResult<Self::Err> {
        let timestamp_idp = custom_idps::timestamp::InherentDataProvider {
            blocktime_millis: self.blocktime.as_millis() as u64,
            maybe_parent_info,
        };
        let _ = (parent_header, ext, relay_parent_offset);

        let slot = Slot::from_timestamp(
            timestamp_idp.timestamp(),
            SlotDuration::from_millis(self.blocktime.as_millis() as u64),
        );
        let digest = vec![
            DigestItem::PreRuntime(
                BABE_ENGINE_ID,
                PreDigest::SecondaryPlain(SecondaryPlainPreDigest {
                    slot,
                    authority_index: 0,
                })
                .encode(),
            ),
            DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode()),
        ];

        Ok((
            Box::new(timestamp_idp),
            digest,
        ))
    }
}
