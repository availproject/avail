# Utils

## Fetch Transaction

### Interface

```rust
async fn fetch_transaction<E: StaticExtrinsic>(&self, block_hash: H256, tx_hash: H256) -> Result<FoundExtrinsic<AvailConfig, Api, E>, FetchTransactionError>;
```

#### Parameters

| parameter  | type | optional | description      |
| ---------- | ---- | -------- | ---------------- |
| block_hash | H256 | false    | block hash       |
| tx_hash    | H256 | false    | transaction hash |

#### Return value

On failure, FetchTransactionError is returned. On Success, extrinsic is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "fetch-transaction"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{avail, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

use avail::balances::calls::types as BalanceCalls;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let dest: &str = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"; // Eve
	let amount = 1_000_000_000_000_000_000u128; // 1 Avail

	let result = sdk
		.tx
		.balances
		.transfer_keep_alive(dest, amount, WaitFor::BlockInclusion, &account)
		.await?;

	let tx = sdk
		.util
		.fetch_transaction::<BalanceCalls::TransferKeepAlive>(result.block_hash, result.tx_hash)
		.await
		.map_err(|e| e.to_string())?;

	println!("Value={:?}", tx.value);
	println!("PalletName={:?}", tx.details.pallet_name());

	Ok(())
}
```

## Fetch Transactions

### Interface

```rust
async fn fetch_transactions(&self, block_hash: H256) -> Result<Extrinsics<AvailConfig, Api>, FetchTransactionError> ;
```

#### Parameters

| parameter  | type | optional | description |
| ---------- | ---- | -------- | ----------- |
| block_hash | H256 | false    | block hash  |

#### Return value

On failure, FetchTransactionError is returned. On Success, extrinsics are returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "fetch-transactions"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{avail, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

use avail::balances::calls::types as BalanceCalls;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let dest: &str = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"; // Eve
	let amount = 1_000_000_000_000_000_000u128; // 1 Avail

	let result = sdk
		.tx
		.balances
		.transfer_keep_alive(dest, amount, WaitFor::BlockInclusion, &account)
		.await?;

	let txs = sdk
		.util
		.fetch_transactions(result.block_hash)
		.await
		.map_err(|e| e.to_string())?;

	let tx = txs
		.find_first::<BalanceCalls::TransferKeepAlive>()
		.map_err(|e| e.to_string())?;

	let Some(tx) = tx else {
		println!("Not transaction found");
		return Ok(());
	};

	println!("Value={:?}", tx.value);
	println!("PalletName={:?}", tx.details.pallet_name());

	Ok(())
}
```

## Progress Transaction

### Interface

```rust
async fn progress_transaction(&self, maybe_tx_progress: Result<TxProgress<AvailConfig, Api>, subxt::Error>, wait_for: WaitFor) -> Result<TransactionInBlock, String>;
```

#### Parameters

| parameter         | type                                               | optional | description                              |
| ----------------- | -------------------------------------------------- | -------- | ---------------------------------------- |
| maybe_tx_progress | Result<TxProgress<AvailConfig, Api>, subxt::Error> | false    | transaction in progress                  |
| waitFor           | WaitFor                                            | false    | wait for block inclusion or finalization |

#### Return value

On failure, a reason of failure is returned. On Success, it progresses and returns the transaction included in the block details.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "progress-transaction"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
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
```
