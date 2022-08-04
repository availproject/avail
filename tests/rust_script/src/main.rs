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
use clap::Parser;
use futures::future::join_all;
use rand::{distributions::Alphanumeric, Rng};
use sp_keyring::AccountKeyring;
use subxt::{AvailExtra, AvailExtraParameters, ClientBuilder, DefaultConfig, PairSigner};
use tokio::sync::Mutex;

use avail::runtime_types::frame_support::storage::bounded_vec::BoundedVec;

#[subxt::subxt(runtime_metadata_path = "./avail.metadata.scale")]
pub mod avail {}

#[derive(Parser, Default, Debug)]
struct Args {
    #[clap(short, long, value_parser, default_value = "submit_data")]
    mode: String,
    #[clap(short, long, value_parser)]
    num: Option<u32>,
    #[clap(short, long, value_parser)]
    size: Option<u32>,
}

async fn submit_data(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<avail::RuntimeApi<DefaultConfig, AvailExtra<DefaultConfig>>>();
    let size = match args.size {
        Some(size) => size,
        None => 10,
    };
    let data = random_generate(size as usize);
    let hash = api
        .tx()
        .data_availability()
        .submit_data(BoundedVec((data.as_bytes()).to_vec()))
        .sign_and_submit_with_additional(&signer, AvailExtraParameters { tip: 0, app_id: 1 })
        .await?;

    println!("Data extrinsic submitted: {}", hash);

    Ok(())
}

static LOCK: Mutex<()> = Mutex::const_new(());

async fn submit_bulk_data(num_transactions: u32, args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<avail::RuntimeApi<DefaultConfig, AvailExtra<DefaultConfig>>>();
    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    // let data = b"Example data".to_vec();
    let size = match args.size {
        Some(size) => size,
        None => 10,
    };
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
            .context(format!("Failed on transaction number {}", i + 1))
            .unwrap();
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
    println!("{:?}", hashes);
    Ok(())
}

fn random_generate(size: usize) -> String{
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();
    s
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.mode == "submit_data" {
        submit_data(args).await?;
    } else if args.mode == "submit_bulk_data" {
        match args.num {
            Some(size) => submit_bulk_data(size, args).await?,
            None => submit_bulk_data(10, args).await?,
        }
    } else {
        println!("Unknown mode: {}", args.mode);
    }
    Ok(())
}
