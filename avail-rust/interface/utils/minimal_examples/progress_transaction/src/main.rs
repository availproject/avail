use avail_rust::{avail, subxt::utils::AccountId32, AccountId, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let dest: AccountId32 = AccountId::from_str("5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw")
		.map_err(|e| e.to_string())?;
	let amount = 1_000_000_000_000_000_000u128; // 1 Avail

	let tx_api = sdk.api.tx();
	let call = avail::tx()
		.balances()
		.transfer_keep_alive(dest.into(), amount);

	let maybe_tx_progress = tx_api
		.sign_and_submit_then_watch_default(&call, &account)
		.await;

	let tx_in_block = sdk
		.util
		.progress_transaction(maybe_tx_progress, WaitFor::BlockInclusion)
		.await?;

	println!("BlockHash={:?}", tx_in_block.block_hash());
	println!("ExtrinsicHash={:?}", tx_in_block.extrinsic_hash());

	Ok(())
}
