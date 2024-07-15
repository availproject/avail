# Data Availability

Runtime Component: DataAvailability\
Runtime Index: 29\
Interface Module Name: dataAvailability

## Create Application Key

Origin Level: Signed

### Interface

```rust
async fn create_application_key(&self, key: Key, wait_for: WaitFor, account: &Keypair) -> Result<CreateApplicationKeyTxSuccess, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| key       | Key         | false    | name of the application key                     |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, ApplicationKeyCreated event, transaction hash and block hash is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "da-create-application-key"
edition = "2021"

[dependencies]
avail-rust = { path = "../../../../." }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Key, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let key = String::from("MyAwesomeKey").as_bytes().to_vec();
	let key = Key { 0: key };

	let result = sdk
		.tx
		.data_availability
		.create_application_key(key, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"Key={:?}, Owner={}, Id={:?}",
		result.event.key, result.event.owner, result.event.id
	);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}
```

## Submit Data

Origin Level: Signed

### Interface

```rust
async fn submit_data(&self, data: Data, wait_for: WaitFor, account: &Keypair) -> Result<SubmitDataTxSuccess, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| data      | Data        | false    | data to be submitted                            |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, DataSubmitted event, transaction data, transaction hash and block hash is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "da-submit-data"
edition = "2021"

[dependencies]
avail-rust = { path = "../../../../." }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Data, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let data = String::from("My Awesome Data").as_bytes().to_vec();
	let data = Data { 0: data };

	let result = sdk
		.tx
		.data_availability
		.submit_data(data, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"Who={}, DataHash={:?}",
		result.event.who, result.event.data_hash
	);
	println!("TxData={:?}", result.tx_data.data);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}
```

## Submit Block Length Proposal

Origin Level: Root

### Interface

```rust
async fn submit_block_length_proposal(&self, rows: u32, cols: u32, wait_for: WaitFor, account: &Keypair) -> Result<SubmitBlockLengthProposalTxSuccess, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| rows      | u32         | false    | number of rows in block                         |
| cols      | u32         | false    | number of cols in block                         |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, BlockLengthProposalSubmitted event, transaction hash and block hash is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "da-submit-block-length-proposal"
edition = "2021"

[dependencies]
avail-rust = { path = "../../../../." }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();

	let rows = 128;
	let cols = 128;

	let result = sdk
		.tx
		.data_availability
		.submit_block_length_proposal(rows, cols, WaitFor::BlockInclusion, &account)
		.await?;

	println!("Rows={:?}, Cols={:?}", result.event.rows, result.event.cols);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}
```

## Set Application Key

Origin Level: Root

### Interface

```rust
async fn set_application_key(&self, old_key: Key, new_key: Key, wait_for: WaitFor, account: &Keypair) -> Result<SetApplicationKeyTxSuccess, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| oldKey    | Key         | false    | application key to be replaced                  |
| newKey    | Key         | false    | application key that will replace the old one   |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, ApplicationKeySet event, transaction hash and block hash is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "da-set-application-key"
edition = "2021"

[dependencies]
avail-rust = { path = "../../../../." }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Key, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();

	let old_key = String::from("MyAwesomeKey").as_bytes().to_vec();
	let old_key = Key { 0: old_key };
	let new_key = String::from("MyAwesomeKey2").as_bytes().to_vec();
	let new_key = Key { 0: new_key };

	let result = sdk
		.tx
		.data_availability
		.set_application_key(old_key, new_key, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"OldKey={:?}, NewKey={:?}",
		result.event.old_key, result.event.new_key
	);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}
```

## Set Submit Data Fee Modifer

Origin Level: Root

### Interface

```rust
async fn set_submit_data_fee_modifier(&self, modifier: DispatchFeeModifier, wait_for: WaitFor, account: &Keypair) -> Result<SetSubmitDataFeeModifierTxSuccess, String>;
```

#### Parameters

| parameter | type                | optional | description                                     |
| --------- | ------------------- | -------- | ----------------------------------------------- |
| modifier  | DispatchFeeModifier | false    | new fee modifier values                         |
| waitFor   | WaitFor             | false    | wait for block inclusion or finalization        |
| account   | KeyringPair         | false    | account that will send and sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, SubmitDataFeeModifierSet event, transaction hash and block hash is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "da-set-submit-data-fee-proposal"
edition = "2021"

[dependencies]
avail-rust = { path = "../../../../." }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{DispatchFeeModifier, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();

	let modifier = DispatchFeeModifier {
		weight_maximum_fee: None,
		weight_fee_divider: Some(2),
		weight_fee_multiplier: None,
	};

	let result = sdk
		.tx
		.data_availability
		.set_submit_data_fee_modifier(modifier, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"WeightMaximumFee={:?}, WeightFeeMultiplier={:?}, WeightFeeDivider={:?}",
		result.event.value.weight_maximum_fee,
		result.event.value.weight_fee_multiplier,
		result.event.value.weight_fee_divider
	);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}
```

# Balances

Runtime Component: Balances\
Runtime Index: 6\
Interface Module Name: balances

## Transfer Keep Alive

Origin Level: Signed

### Interface

```rust
async fn transfer_keep_alive(&self, dest: &str, value: u128, wait_for: WaitFor, account: &Keypair) -> Result<TransferKeepAliveTxSuccess, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| dest      | &str        | false    | account that will receive funds                 |
| value     | u128        | false    | amount that is send. 10^18 is equal to 1 AVL    |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, TransferEvent event, transaction hash and block hash is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "balance-transfer-keep-alive"
edition = "2021"

[dependencies]
avail-rust = { path = "../../../../." }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

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

	println!(
		"From={}, To={}, Amount={}",
		result.event.from, result.event.to, result.event.amount
	);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}
```

## Transfer Allow Death

Origin Level: Signed

### Interface

```rust
async fn transfer_allow_death(&self, dest: &str, value: u128, wait_for: WaitFor, account: &Keypair) -> Result<TransferAllowDeathTxSuccess, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| dest      | string      | false    | account that will receive funds                 |
| value     | BN          | false    | amount that is send. 10^18 is equal to 1 AVL    |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, TransferEvent event, KilledAccount (optionally) event, transaction hash and block
hash is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "balance-transfer-allow-death"
edition = "2021"

[dependencies]
avail-rust = { path = "../../../../." }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"; // Eve
	let amount = 1_000_000_000_000_000_00u128; // 1 Avail

	let result = sdk
		.tx
		.balances
		.transfer_allow_death(dest, amount, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"From={}, To={}, Amount={}",
		result.event.from, result.event.to, result.event.amount
	);
	if let Some(event) = result.event2 {
		println!("Killed={}", event.account);
	}
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}
```

## Transfer All

Origin Level: Signed

### Interface

```rust
async fn transfer_all(&self, dest: &str, keep_alive: bool, wait_for: WaitFor, account: &Keypair) -> Result<TransferAllTxSuccess, String>;
```

#### Parameters

| parameter | type        | optional | description                                      |
| --------- | ----------- | -------- | ------------------------------------------------ |
| dest      | &str        | false    | account that will receive funds                  |
| keepAlive | bool        | false    | if set to false it will reap the account as well |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization         |
| account   | KeyringPair | false    | account that will send and sign the transaction  |

#### Return value

On failure, a reason of failure is returned. On Success, TransferEvent event, KilledAccount (optionally) event, transaction hash and block
hash is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "balance-transfer-all"
edition = "2021"

[dependencies]
avail-rust = { path = "../../../../." }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"; // Eve
	let keep_alive = false;

	let result = sdk
		.tx
		.balances
		.transfer_all(dest, keep_alive, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"From={}, To={}, Amount={}",
		result.event.from, result.event.to, result.event.amount
	);
	if let Some(event) = result.event2 {
		println!("Killed={}", event.account);
	}
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}
```

# Staking

Runtime Component: Staking\
Runtime Index: 10\
Interface Module Name: staking

## Bond

Origin Level: Signed

### Interface

```rust
async fn bond(&self, value: u128, payee: RewardDestination, wait_for: WaitFor, account: &Keypair) -> Result<BondTxSuccess, String>;
```

#### Parameters

| parameter | type              | optional | description                                             |
| --------- | ----------------- | -------- | ------------------------------------------------------- |
| value     | u128              | false    | amount that is bond. 10^18 is equal to 1 AVL            |
| payee     | RewardDestination | false    | Can be: "Staked", "Stash", "None" or an account address |
| waitFor   | WaitFor           | false    | wait for block inclusion or finalization                |
| account   | KeyringPair       | false    | account that will send and sign the transaction         |

#### Return value

On failure, a reason of failure is returned. On Success, Bonded event, transaction hash and block hash is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "staking-bond"
edition = "2021"

[dependencies]
avail-rust = { path = "../../../../." }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, RewardDestination, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let value = 1_000_000_000_000_000_000u128 * 100_000u128; // 100_000 Avail
	let payee = RewardDestination::Staked;

	let result = sdk
		.tx
		.staking
		.bond(value, payee, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"Stash={}, Amount={:?}",
		result.event.stash, result.event.amount
	);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}
```

## Bond Extra

Origin Level: Signed

### Interface

```rust
async fn bond_extra(&self, max_additional: u128, wait_for: WaitFor, account: &Keypair) -> Result<BondExtraTxSuccess, String>;
```

#### Parameters

| parameter     | type        | optional | description                                               |
| ------------- | ----------- | -------- | --------------------------------------------------------- |
| maxAdditional | u128        | false    | additional amount that is bond. 10^18 is equal to 1 Avail |
| waitFor       | WaitFor     | false    | wait for block inclusion or finalization                  |
| account       | KeyringPair | false    | account that will send and sign the transaction           |

#### Return value

On failure, a reason of failure is returned. On Success, Bonded event, transaction hash and block hash is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "staking-bond-extra"
edition = "2021"

[dependencies]
avail-rust = { path = "../../../../." }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let max_additional = 1_000_000_000_000_000_000u128; // 1 AVAIL

	let result = sdk
		.tx
		.staking
		.bond_extra(max_additional, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"Stash={}, Amount={:?}",
		result.event.stash, result.event.amount
	);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}
```

## Chill

Origin Level: Signed

### Interface

```rust
async fn chill(&self, wait_for: WaitFor, account: &Keypair) -> Result<ChillTxSuccess, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, Chilled event, transaction hash and block hash is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "staking-chill"
edition = "2021"

[dependencies]
avail-rust = { path = "../../../../." }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice//stash").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();

	let result = sdk
		.tx
		.staking
		.chill(WaitFor::BlockInclusion, &account)
		.await?;

	if let Some(event) = result.event {
		println!("Stash={}", event.stash);
	}

	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}
```

## Chill Other

Origin Level: Signed

### Interface

```rust
async fn chill_other(&self, stash: &str, wait_for: WaitFor, account: &Keypair) -> Result<ChillOtherTxSuccess, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| stash     | &str        | false    | address of stash account to chill               |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, Chilled event, transaction hash and block hash is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "staking-chill-other"
edition = "2021"

[dependencies]
avail-rust = { path = "../../../../." }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let stash = "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"; // Alice Stash

	let result = sdk
		.tx
		.staking
		.chill_other(stash, WaitFor::BlockInclusion, &account)
		.await?;

	println!("Stash={}", result.event.stash);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}
```

## Nominate

Origin Level: Signed

### Interface

```rust
async fn nominate( &self, targets: &[String], wait_for: WaitFor, account: &Keypair) -> Result<NominateTxSuccess, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| targets   | &[String]   | false    | list od addresses to nominate                   |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, Nominate transaction data, transaction hash and block hash is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "staking-nominate"
edition = "2021"

[dependencies]
avail-rust = { path = "../../../../." }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let targets = [
		String::from("5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"), // Alice Stash
		String::from("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"), // Bob;
	];

	let result = sdk
		.tx
		.staking
		.nominate(&targets, WaitFor::BlockInclusion, &account)
		.await?;

	println!("TxDataTargets={:?}", result.tx_data.targets);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}
```

## Unbond

Origin Level: Signed

### Interface

```rust
async fn unbond(&self, value: u128, wait_for: WaitFor, account: &Keypair) -> Result<UnbondTxSuccess, String>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| value     | u128          | false    | amount of tokens to unbond                      |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

#### Return value

On failure, a reason of failure is returned. On Success, Unbonded event, transaction hash and block hash is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "staking-unbond"
edition = "2021"

[dependencies]
avail-rust = { path = "../../../../." }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let value = 1_000_000_000_000_000_000u128; // 1 Avail

	let result = sdk
		.tx
		.staking
		.unbond(value, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"Stash={}, Amount={:?}",
		result.event.stash, result.event.amount
	);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}
```

## Validate

Origin Level: Signed

### Interface

```rust
async fn validate(&self, commission: u8, blocked: bool, wait_for: WaitFor, account: &Keypair) -> Result<ValidateTxSuccess, String>;
```

#### Parameters

| parameter  | type        | optional | description                                           |
| ---------- | ----------- | -------- | ----------------------------------------------------- |
| commission | u8          | false    | how much validator charge nominators in 0 - 100 range |
| blocked    | bool        | false    | whether or not this validator accepts nominations     |
| waitFor    | WaitFor     | false    | wait for block inclusion or finalization              |
| account    | KeyringPair | false    | account that will send and sign the transaction       |

#### Return value

On failure, a reason of failure is returned. On Success, ValidatorPrefsSet event, transaction hash and block hash is returned.

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "staking-validate"
edition = "2021"

[dependencies]
avail-rust = { path = "../../../../." }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let commission = 100;
	let blocked = false;

	let result = sdk
		.tx
		.staking
		.validate(commission, blocked, WaitFor::BlockInclusion, &account)
		.await?;

	println!(
		"Stash={}, Commission={:?}, Blocked={:?}",
		result.event.stash, result.event.prefs.commission, result.event.prefs.blocked
	);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);

	Ok(())
}
```
