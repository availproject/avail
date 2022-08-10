use clap::Parser;

mod balance_transfer;
mod submit_data;
pub mod avail_subxt_config;
use avail_subxt_config::*;

use subxt::{AvailExtra, ClientBuilder};

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
    #[clap(short, long, value_parser)]
    amount: Option<u128>,
}

async fn finalized_head() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<avail::RuntimeApi<AvailConfig, AvailExtra<AvailConfig>>>();

    let mut finalized_blocks = api
        .client
        .rpc()
        .subscribe_finalized_blocks()
        .await?;

    while let Some(finalized_block) = finalized_blocks.next().await {
        println!("\nFinalized Block: {:?}", finalized_block.unwrap());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let size = match args.size {
        Some(size) => size,
        None => 10,
    };
    if args.mode == "submit_data" {
        submit_data::submit_data(size).await?;
    } else if args.mode == "submit_bulk_data" {
        match args.num {
            Some(num) => submit_data::submit_bulk_data(num, size).await?,
            None => submit_data::submit_bulk_data(10, size).await?,
        }
    } else if args.mode == "transfer" {
        let amount = match args.amount {
            Some(amount) => amount,
            None => 10000,
        };
        balance_transfer::transfer(amount).await?;
    }else if args.mode == "subscribe" {
        finalized_head().await?;
    } 
    else {
        println!("Unknown mode: {}", args.mode);
    }
    Ok(())
}
