use avail_subxt::avail::runtime_types::frame_support::storage::bounded_vec::BoundedVec;
use avail_subxt::*;
use sp_keyring::AccountKeyring;
use std::time::Instant;
use subxt::ext::sp_core::H256;
use subxt::{tx::PairSigner, OnlineClient};

/// This example attempts to submit data to fill the entire block. Note that this doesn't guarantee
/// that the block will be filled, but if you submit more than a full block, then it will spill over
/// to the next block. The limit for the transaction is currently set to 16 kB, and limit for the block
/// is 2 MB, so this means 128 data transactions are needed to fill the block. Depending on the network,
/// it may not be possible to transfer so many in 20 s (the default block time)

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::new(AccountKeyring::Alice.pair());

    let api = OnlineClient::<AvailConfig>::new().await?;
    let size: usize = 2 * 1024 * 1024;
    let max_size: usize = 16 * 1024;
    let num_chunks = size / max_size;
    let mut hashes = Vec::<H256>::new();
    let extrinsic_params = AvailExtrinsicParams::new_with_app_id(1);
    let start = Instant::now();
    for i in 1..=num_chunks {
        let data_transfer =
            avail::tx()
                .data_availability()
                .submit_data(BoundedVec(vec![(i & 255) as u8; max_size]));
        let h = api
            .tx()
            .sign_and_submit(&data_transfer, &signer, extrinsic_params.clone())
            .await
            .unwrap();
        println!("hash #{i}: {h}");
        hashes.push(h);
    }
    let end = start.elapsed();

    println!("Done in {end:?}!");

    Ok(())
}
