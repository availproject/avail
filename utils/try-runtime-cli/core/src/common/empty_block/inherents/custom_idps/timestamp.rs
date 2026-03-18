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

//! Inherent data provider for the timestamp, for empty block production on top of an existing
//! externalities.

use sp_inherents::{InherentData, InherentIdentifier};
use sp_runtime::Digest;
use sp_timestamp::{Timestamp, TimestampInherentData};

pub struct InherentDataProvider {
    pub blocktime_millis: u64,
    pub maybe_parent_info: Option<(InherentData, Digest)>,
}

impl InherentDataProvider {
    pub fn timestamp(&self) -> Timestamp {
        match &self.maybe_parent_info {
            Some((prev_inherent_data, _)) => sp_timestamp::InherentDataProvider::new(
                prev_inherent_data
                    .timestamp_inherent_data()
                    .unwrap()
                    .unwrap()
                    + self.blocktime_millis,
            )
            .timestamp(),
            None => sp_timestamp::InherentDataProvider::from_system_time().timestamp(),
        }
    }
}

#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for InherentDataProvider {
    async fn provide_inherent_data(
        &self,
        inherent_data: &mut sp_inherents::InherentData,
    ) -> Result<(), sp_inherents::Error> {
        match &self.maybe_parent_info {
            Some((prev_inherent_data, _)) => {
                let idp = sp_timestamp::InherentDataProvider::new(
                    prev_inherent_data
                        .timestamp_inherent_data()
                        .unwrap()
                        .unwrap()
                        + self.blocktime_millis,
                );
                idp.provide_inherent_data(inherent_data)
                    .await
                    .expect("Failed to provide timestamp inherent");
            }
            None => {
                let idp = sp_timestamp::InherentDataProvider::from_system_time();
                idp.provide_inherent_data(inherent_data)
                    .await
                    .expect("Failed to provide timestamp inherent");
            }
        };

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
