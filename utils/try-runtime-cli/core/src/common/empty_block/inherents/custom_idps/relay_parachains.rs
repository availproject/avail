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

//! Inherent data provider for the [polkadot parachins inherent](https://github.com/paritytech/polkadot-sdk/blob/master/polkadot/primitives/src/v7/mod.rs)
//! for empty block production on top of an existing externalities.

use sp_inherents::InherentIdentifier;
use sp_runtime::traits::Block as BlockT;

pub struct InherentDataProvider<B: BlockT> {
    parent_header: B::Header,
}

impl<B: BlockT> InherentDataProvider<B> {
    pub fn new(parent_header: B::Header) -> Self {
        Self { parent_header }
    }
}

#[async_trait::async_trait]
impl<B: BlockT> sp_inherents::InherentDataProvider for InherentDataProvider<B> {
    async fn provide_inherent_data(
        &self,
        inherent_data: &mut sp_inherents::InherentData,
    ) -> Result<(), sp_inherents::Error> {
        let para_data = polkadot_primitives::InherentData {
            bitfields: Vec::new(),
            backed_candidates: Vec::new(),
            disputes: Vec::new(),
            parent_header: self.parent_header.clone(),
        };

        inherent_data.put_data(
            polkadot_primitives::PARACHAINS_INHERENT_IDENTIFIER,
            &para_data,
        )?;

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
