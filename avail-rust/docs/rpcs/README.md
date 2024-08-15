# Author

## Rotate Keys

### Interface

```rust
async fn rotate_keys(&self) -> Result<Vec<u8>, subxt::Error>l
```

#### Return value

On failure, a reason of failure is returned. On Success, rotated keys are returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "author-rotate-keys"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::SDK;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944")
		.await
		.map_err(|e| e.to_string())?;

	let keys = sdk
		.rpc
		.author
		.rotate_keys()
		.await
		.map_err(|e| e.to_string())?;
	println!("Keys={:?}", keys);

	Ok(())
}
```

# Kate

## Block Length

### Interface

```rust
async fn block_length(&self, at: Option<H256>) -> Result<BlockLength, subxt::Error>;
```

#### Parameters

| parameter | type         | optional | description      |
| --------- | ------------ | -------- | ---------------- |
| at        | Option<H256> | true     | transaction hash |

#### Return value

On failure, subxt::Error is returned. On Success, BlockLength is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "kate-block-length"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::SDK;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944")
		.await
		.map_err(|e| e.to_string())?;

	let block_length = sdk
		.rpc
		.kate
		.block_length(None)
		.await
		.map_err(|e| e.to_string())?;
	println!("BlockLength={:?}", block_length);

	Ok(())
}
```

## Query Rows

### Interface

```rust
async fn query_rows(&self, rows: Vec<u32>, at: Option<H256>) -> Result<Vec<GRow>, subxt::Error>;
```

#### Parameters

| parameter | type         | optional | description      |
| --------- | ------------ | -------- | ---------------- |
| rows      | Option<u32>  | false    | rows to query    |
| at        | Option<H256> | true     | transaction hash |

#### Return value

On failure, subxt::Error is returned. On Success, rows are returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "kate-query-rows"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Data, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944")
		.await
		.map_err(|e| e.to_string())?;

	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let data = String::from("My Awesome Data").as_bytes().to_vec();
	let data = Data { 0: data };

	let result = sdk
		.tx
		.data_availability
		.submit_data(data, WaitFor::BlockFinalization, &account)
		.await?;

	let rows = vec![0u32];
	let rpc_result = sdk
		.rpc
		.kate
		.query_rows(rows, Some(result.block_hash))
		.await
		.map_err(|e| e.to_string())?;
	println!("Rows={:?}", rpc_result);

	Ok(())
}
```

## Query Proof

### Interface

```rust
async fn query_proof(&self, cells: Vec<Cell>, at: Option<H256>) -> Result<Vec<GDataProof>, subxt::Error>;
```

#### Parameters

| parameter | type         | optional | description      |
| --------- | ------------ | -------- | ---------------- |
| cells     | Vec<Cell>    | false    | cells to query   |
| at        | Option<H256> | true     | transaction hash |

#### Return value

On failure, subxt::Error is returned. On Success, proof is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "kate-query-proof"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Cell, Data, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944")
		.await
		.map_err(|e| e.to_string())?;

	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let data = String::from("My Awesome Data").as_bytes().to_vec();
	let data = Data { 0: data };

	let result = sdk
		.tx
		.data_availability
		.submit_data(data, WaitFor::BlockFinalization, &account)
		.await?;

	let cell: Cell = (0u32, 0u32).into();
	let rpc_result = sdk
		.rpc
		.kate
		.query_proof(vec![cell], Some(result.block_hash))
		.await
		.map_err(|e| e.to_string())?;
	println!("Result={:?}", rpc_result);

	Ok(())
}
```

## Query Data Proof

### Interface

```rust
async fn query_data_proof(&self, transaction_index: u32, at: Option<H256>) -> Result<ProofResponse, subxt::Error>;
```

#### Parameters

| parameter         | type         | optional | description       |
| ----------------- | ------------ | -------- | ----------------- |
| transaction_index | u32          | false    | transaction index |
| at                | Option<H256> | true     | transaction hash  |

#### Return value

On failure, subxt::Error is returned. On Success, proof is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "kate-query-data-proof"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Data, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944")
		.await
		.map_err(|e| e.to_string())?;

	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let data = String::from("My Awesome Data").as_bytes().to_vec();
	let data = Data { 0: data };

	let result = sdk
		.tx
		.data_availability
		.submit_data(data, WaitFor::BlockFinalization, &account)
		.await?;

	let transaction_index = 1u32;
	let rpc_result = sdk
		.rpc
		.kate
		.query_data_proof(transaction_index, Some(result.block_hash))
		.await
		.map_err(|e| e.to_string())?;
	println!("ProofResponse={:?}", rpc_result);

	Ok(())
}
```
