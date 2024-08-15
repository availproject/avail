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
async fn block_length(&self, at: Option<BlockHash>) -> Result<BlockLength, subxt::Error>;
```

#### Parameters

| parameter | type         | optional | description      |
| --------- | ------------ | -------- | ---------------- |
| at        | Option<BlockHash> | true     | block hash |

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
async fn query_rows(&self, rows: Vec<u32>, at: Option<BlockHash>) -> Result<Vec<GRow>, subxt::Error>;
```

#### Parameters

| parameter | type         | optional | description      |
| --------- | ------------ | -------- | ---------------- |
| rows      | Option<u32>  | false    | rows to query    |
| at        | Option<BlockHash> | true     | block hash |

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
async fn query_proof(&self, cells: Vec<Cell>, at: Option<BlockHash>) -> Result<Vec<GDataProof>, subxt::Error>;
```

#### Parameters

| parameter | type         | optional | description      |
| --------- | ------------ | -------- | ---------------- |
| cells     | Vec<Cell>    | false    | cells to query   |
| at        | Option<BlockHash> | true     | block hash |

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
| at                | Option<BlockHash> | true     | block hash  |

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

# Chain

## Get Block

### Interface

```rust
async fn get_block(&self, at: Option<BlockHash>) -> Result<AvailBlockDetailsRPC, subxt::Error>;
```

#### Parameters

| parameter         | type         | optional | description       |
| ----------------- | ------------ | -------- | ----------------- |
| at                | Option<BlockHash> | true     | block hash  |

#### Return value

On failure, subxt::Error is returned. On Success, block details are returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "chain-get-block"
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

	let block = sdk
		.rpc
		.chain
		.get_block(None)
		.await
		.map_err(|e| e.to_string())?;
	println!("Block={:?}", block);

	Ok(())
}
```

## Get Block Hash

### Interface

```rust
async fn get_block_hash(&self, block_number: Option<BlockNumber>) -> Result<BlockHash, subxt::Error>;
```

#### Parameters

| parameter         | type         | optional | description       |
| ----------------- | ------------ | -------- | ----------------- |
| block_number | Option<BlockNumber> | true     | block number  |

#### Return value

On failure, subxt::Error is returned. On Success, block hash is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "chain-get-block-hash"
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

	let block_hash = sdk
		.rpc
		.chain
		.get_block_hash(None)
		.await
		.map_err(|e| e.to_string())?;
	println!("BlockHash={:?}", block_hash);

	Ok(())
}
```

## Get Finalized Head

### Interface

```rust
async fn get_finalized_head(&self) -> Result<BlockHash, subxt::Error>;
```

#### Return value

On failure, subxt::Error is returned. On Success, block hash of finalized is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "chain-get-finalized-head"
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

	let block_hash = sdk
		.rpc
		.chain
		.get_finalized_head()
		.await
		.map_err(|e| e.to_string())?;
	println!("FinalizedHeadBlockHash={:?}", block_hash);

	Ok(())
}
```

## Get Header

### Interface

```rust
async fn get_header(&self, at: Option<BlockHash>) -> Result<AvailHeader, subxt::Error>;
```

#### Parameters

| parameter         | type         | optional | description       |
| ----------------- | ------------ | -------- | ----------------- |
| at | Option<BlockHash> | true     | block hash  |

#### Return value

On failure, subxt::Error is returned. On Success, header of a block is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "chain-get-header"
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

	let header = sdk
		.rpc
		.chain
		.get_header(None)
		.await
		.map_err(|e| e.to_string())?;
	println!("BlockHeader={:?}", header);

	Ok(())
}
```

# System

## Account Next Index

### Interface

```rust
async fn account_next_index(&self, account: String) -> Result<u32, subxt::Error>;
```

#### Parameters

| parameter         | type         | optional | description       |
| ----------------- | ------------ | -------- | ----------------- |
| account         | String | false     | account address  |

#### Return value

On failure, subxt::Error  is returned. On Success, account nonce is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "system-account-next-index"
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

	let account = String::from("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");

	let account_next_index = sdk
		.rpc
		.system
		.account_next_index(account)
		.await
		.map_err(|e| e.to_string())?;
	println!("AccountNextIndex={:?}", account_next_index);

	Ok(())
}
```

## Chain

### Interface

```rust
async fn chain(&self) -> Result<String, subxt::Error>;
```

#### Return value

On failure, subxt::Error is returned. On Success, chain name is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "system-chain"
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

	let chain = sdk.rpc.system.chain().await.map_err(|e| e.to_string())?;
	println!("Chain={:?}", chain);

	Ok(())
}
```

## Chain Type

### Interface

```rust
async fn chain_type(&self) -> Result<String, subxt::Error> ;
```

#### Return value

On failure, subxt::Error is returned. On Success, chain type is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "system-chain-type"
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

	let chain_type = sdk
		.rpc
		.system
		.chain_type()
		.await
		.map_err(|e| e.to_string())?;
	println!("ChainType={:?}", chain_type);

	Ok(())
}
```

## Health

### Interface

```rust
async fn health(&self) -> Result<SystemHealth, subxt::Error>;
```

#### Return value

On failure, subxt::Error is returned. On Success, health status of the node is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "system-health"
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

	let health = sdk
		.rpc
		.system
		.health()
		.await
		.map_err(|e| e.to_string())?;
	println!("Health={:?}", health);

	Ok(())
}
```

## Local Listen Addresses

### Interface

```rust
async fn local_listen_addresses(&self) -> Result<Vec<String>, subxt::Error>;
```

#### Return value

On failure, subxt::Error is returned. On Success, local addresses are returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "system-local-listen-addresses"
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

	let local_listen_addresses = sdk
		.rpc
		.system
		.local_listen_addresses()
		.await
		.map_err(|e| e.to_string())?;
	println!("LocalListenAddresses={:?}", local_listen_addresses);

	Ok(())
}
```

## Local Peer Id

### Interface

```rust
async fn local_peer_id(&self) -> Result<String, subxt::Error>;
```

#### Return value

On failure, subxt::Error is returned. On Success, base58-encoded Peerid of the node returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "system-local-peer-id"
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

	let local_peer_id = sdk
		.rpc
		.system
		.local_peer_id()
		.await
		.map_err(|e| e.to_string())?;
	println!("LocalPeerId={:?}", local_peer_id);

	Ok(())
}
```

## Name

### Interface

```rust
async fn name(&self) -> Result<String, subxt::Error>;
```

#### Return value

On failure, subxt::Error is returned. On Success, node name is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "system-name"
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

	let name = sdk
		.rpc
		.system
		.name()
		.await
		.map_err(|e| e.to_string())?;
	println!("Name={:?}", name);

	Ok(())
}
```

## Node Roles

### Interface

```rust
async fn node_roles(&self) -> Result<Vec<NodeRole>, subxt::Error>;
```

#### Return value

On failure, subxt::Error is returned. On Success, roles of the node are returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "system-node-roles"
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

	let node_roles = sdk
		.rpc
		.system
		.node_roles()
		.await
		.map_err(|e| e.to_string())?;
	println!("NodeRoles={:?}", node_roles);

	Ok(())
}
```

## Peers

### Interface

```rust
async fn peers(&self) -> Result<Vec<PeerInfo>, subxt::Error>;
```

#### Return value

On failure, subxt::Error is returned. On Success, list of currently connected peers are  returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "system-peers"
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

	let peers = sdk
		.rpc
		.system
		.peers()
		.await
		.map_err(|e| e.to_string())?;
	println!("Peers={:?}", peers);

	Ok(())
}
```

## Properties

### Interface

```rust
async fn properties(&self) -> Result<Properties, subxt::Error>;
```

#### Return value

On failure, subxt::Error is returned. On Success, custom set of properties are returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "system-properties"
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

	let properties = sdk
		.rpc
		.system
		.properties()
		.await
		.map_err(|e| e.to_string())?;
	println!("Properties={:?}", properties);

	Ok(())
}
```

## Sync Status

### Interface

```rust
async fn sync_state(&self) -> Result<SyncState, subxt::Error>;
```

#### Return value

On failure, subxt::Error is returned. On Success, state of the syncing of the node is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "system-sync-state"
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

	let sync_state = sdk
		.rpc
		.system
		.sync_state()
		.await
		.map_err(|e| e.to_string())?;
	println!("SyncState={:?}", sync_state);

	Ok(())
}
```

## Sync Status

### Interface

```rust
async fn version(&self) -> Result<String, subxt::Error>;
```

#### Return value

On failure, subxt::Error is returned. On Success, version of the node is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "system-version"
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

	let version = sdk
		.rpc
		.system
		.version()
		.await
		.map_err(|e| e.to_string())?;
	println!("Version={:?}", version);

	Ok(())
}
```
