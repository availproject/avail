use sp_keyring::AccountKeyring;
use subxt::{tx::PairSigner, OnlineClient};

use avail_subxt::*;

use avail_subxt::avail::runtime_types::frame_support::storage::bounded_vec::BoundedVec;

/// This example submits an Avail data extrinsic, then retrieves the block containing the 
/// extrinsic and matches the data.
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<AvailConfig>::new().await?;
    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let example_data = b"example".to_vec();
    let data_transfer = avail::tx()
        .data_availability()
        .submit_data(BoundedVec(example_data.clone()));
    let extrinsic_params = AvailExtrinsicParams::new_with_app_id(1);
    println!("Sending example data...");
    let h = api
        .tx()
        .sign_and_submit_then_watch(&data_transfer, &signer, extrinsic_params)
        .await
        .unwrap()
        .wait_for_finalized()
        .await
        .unwrap();

    let submitted_block = api
        .rpc()
        .block(Some(h.block_hash()))
        .await
        .unwrap()
        .unwrap();

    let xts = submitted_block.block.extrinsics;
    println!("Submitted block extrinsic: {xts:?}");

    let matched_xt = xts.iter().find(move |e| match e {
        AvailExtrinsic::AvailDataExtrinsic {
            app_id,
            signature: _,
            data,
        } => *app_id == 1 && data.eq(&example_data),
        _ => false,
    });

    assert!(matched_xt.is_some(), "Submitted data not found");

    Ok(())
}
