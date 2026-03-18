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

//! Inherent data provider for the [cumulus parachin inherents](https://github.com/paritytech/polkadot-sdk/blob/master/cumulus/primitives/parachain-inherent/src/lib.rs)
//! for empty block production on top of an existing externalities.

use std::{ops::DerefMut, sync::Arc};

use parity_scale_codec::{Decode, Encode};
use polkadot_primitives::HeadData;
use sp_consensus_babe::SlotDuration;
use sp_core::twox_128;
use sp_inherents::InherentIdentifier;
use sp_runtime::traits::{Block as BlockT, HashingFor};
use sp_state_machine::TestExternalities;
use tokio::sync::Mutex;

/// Get the para id if it exists
pub fn get_para_id<B: BlockT>(ext: &mut TestExternalities<HashingFor<B>>) -> Option<u32> {
    let para_id_key = [twox_128(b"ParachainInfo"), twox_128(b"ParachainId")].concat();

    ext.execute_with(|| sp_io::storage::get(&para_id_key))
        .and_then(|b| -> Option<u32> { Decode::decode(&mut &b[..]).ok() })
}

/// Provides parachain-system pallet inherents.
pub struct InherentDataProvider<B: BlockT> {
    pub timestamp: sp_timestamp::Timestamp,
    pub blocktime_millis: u64,
    pub parent_header: B::Header,
    pub ext_mutex: Arc<Mutex<TestExternalities<HashingFor<B>>>>,
    pub relay_parent_offset: u32,
}

#[async_trait::async_trait]
impl<B: BlockT> sp_inherents::InherentDataProvider for InherentDataProvider<B> {
    async fn provide_inherent_data(
        &self,
        inherent_data: &mut sp_inherents::InherentData,
    ) -> Result<(), sp_inherents::Error> {
        let mut ext_guard = self.ext_mutex.lock().await;
        let ext = ext_guard.deref_mut();
        let Some(para_id) = get_para_id::<B>(ext) else {
            log::debug!("Unable to provide para parachains inherent for this chain.");
            return Ok(());
        };

        let relay_chain_slot = cumulus_primitives_core::relay_chain::Slot::from_timestamp(
            self.timestamp,
            SlotDuration::from_millis(self.blocktime_millis),
        );

        cumulus_client_parachain_inherent::MockValidationDataInherentDataProvider {
            relay_offset: *relay_chain_slot as u32,
            current_para_block_head: Some(HeadData(self.parent_header.encode())),
            relay_parent_offset: self.relay_parent_offset,
            relay_randomness_config: (),
            para_id: para_id.into(),
            ..Default::default()
        }
        .provide_inherent_data(inherent_data)
        .await
        .expect("Failed to provide Para Parachain inherent data.");

        Ok(())
    }

    async fn try_handle_error(
        &self,
        _: &InherentIdentifier,
        _: &[u8],
    ) -> Option<Result<(), sp_inherents::Error>> {
        None
    }
}
