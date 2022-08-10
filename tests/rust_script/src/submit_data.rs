// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

//! To run this example, a local polkadot node should be running. Example verified against polkadot 0.9.13-82616422d0-aarch64-macos.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.13/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```
use anyhow::{anyhow, Context, Result};
use avail::runtime_types::frame_support::storage::bounded_vec::BoundedVec;
use futures::future::join_all;
use futures::StreamExt;
use rand::{distributions::Alphanumeric, Rng};
use sp_keyring::AccountKeyring;
use subxt::{AvailExtra, AvailExtraParameters, ClientBuilder, DefaultConfig, PairSigner};
use tokio::sync::Mutex;

#[subxt::subxt(runtime_metadata_path = "./avail.metadata.scale")]
pub mod avail {}

pub async fn submit_data(size: u32) -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<avail::RuntimeApi<DefaultConfig, AvailExtra<DefaultConfig>>>();
    let data = random_generate(size as usize);
    let mut hash = api
        .tx()
        .data_availability()
        .submit_data(BoundedVec((data.as_bytes()).to_vec()))
        .sign_and_submit_with_aditional_then_watch(
            &signer,
            AvailExtraParameters { tip: 0, app_id: 1 },
        )
        .await?;

    while let Some(ev) = hash.next().await {
        let ev = ev?;
        use subxt::TransactionStatus::*;

        // Made it into a block, but not finalized.
        if let InBlock(details) = ev {
            println!(
                "Transaction {:?} made it into block {:?}",
                details.extrinsic_hash(),
                details.block_hash()
            );
            break;
        }
    }

    Ok(())
}

static LOCK: Mutex<()> = Mutex::const_new(());

pub async fn submit_bulk_data(
    num_transactions: u32,
    size: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<avail::RuntimeApi<DefaultConfig, AvailExtra<DefaultConfig>>>();
    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    // let data = b"Example data".to_vec();
    let data = random_generate(size as usize);
    let mut statuses = vec![];
    for i in 0..num_transactions {
        let _lock = LOCK.lock().await;
        let h = api
            .tx()
            .data_availability()
            .submit_data(BoundedVec((data.as_bytes()).to_vec()))
            .sign_and_submit_with_aditional_then_watch(
                &signer,
                AvailExtraParameters { tip: 0, app_id: 1 },
            )
            .await
            .context(format!("Failed on transaction number {}", i + 1))?;

        let s = h.wait_for_in_block();
        statuses.push(s);
    }
    let status = join_all(statuses.into_iter()).await;
    let hashes = status
        .iter()
        .map(|e| {
            e.as_ref()
                .map(|v| v.block_hash())
                .map_err(|_| anyhow!("failed"))
        })
        .collect::<Vec<_>>()
        .into_iter()
        .collect::<Result<Vec<_>>>()
        .unwrap();
    println!("{:?} is in block", hashes);
    Ok(())
}

fn random_generate(size: usize) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();
    s
}
