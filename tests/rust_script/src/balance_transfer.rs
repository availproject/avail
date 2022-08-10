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

use sp_keyring::AccountKeyring;
use subxt::{AvailExtra, AvailExtraParameters, ClientBuilder, DefaultConfig, PairSigner};

#[subxt::subxt(runtime_metadata_path = "./avail.metadata.scale")]
pub mod avail {}

pub async fn transfer(amount: u128) -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<avail::RuntimeApi<DefaultConfig, AvailExtra<DefaultConfig>>>();
    let hash = api
        .tx()
        .balances()
        .transfer(dest, amount)
        .sign_and_submit_with_additional(&signer, AvailExtraParameters { tip: 0, app_id: 0 })
        .await?;

    println!("Balance transfer extrinsic submitted: {}", hash);

    Ok(())
}
