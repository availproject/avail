# Data Availability

Runtime Component: DataAvailability\
Runtime Index: 29\
Interface Module Name: dataAvailability

## Create Application Key

Origin Level: Signed

### Interface

```rust
async fn create_application_key(&self, key: Key, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<CreateApplicationKeyTx, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| key       | Key         | false    | name of the application key                     |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "da-create-application-key"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Key, Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let key = String::from("MyAwesomeKey").as_bytes().to_vec();
	let key = Key { 0: key };

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.data_availability
		.create_application_key(key, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

```rust
Error: "Runtime error: Pallet error: DataAvailability::AppKeyAlreadyExists"
```

#### On Success

If the operation is successful, the function will return a object of type `CreateApplicationKeyTx`.

```rust
CreateApplicationKeyTx {
    event: ApplicationKeyCreated {
        key: BoundedVec(...),
        owner: AccountId32(...),
        id: AppId(13),
    },
    events: ExtrinsicEvents {
        ext_hash: 0x2beb45ead24d997053c9c4c7edb4d22acf83ce1319d97555ca862c98a934f8b9,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 9,
        },
    },
    tx_hash: 0x2beb45ead24d997053c9c4c7edb4d22acf83ce1319d97555ca862c98a934f8b9,
    tx_index: 1,
    block_hash: 0xd4f3c52da5bdb7d4d3d1b14794ae18b08979c90bb4a98c10c2955841aeaae631,
    block_number: 56,
}
```

## Submit Data

Origin Level: Signed

### Interface

```rust
async fn submit_data(&self, data: Data, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<SubmitDataTx, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| data      | Data        | false    | data to be submitted                            |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "da-submit-data"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Data, Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let data = String::from("My Awesome Data").as_bytes().to_vec();
	let data = Data { 0: data };

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.data_availability
		.submit_data(data, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `SubmitDataTx`.

```rust
SubmitDataTx {
    event: DataSubmitted {
        who: AccountId32(...),
        data_hash: 0x8846d900ea89aab9bce96402846c0ac74a853acc00cb99ff5ddb1a0f052594bd,
    },
    events: ExtrinsicEvents {
        ext_hash: 0xf049c9d4676589bf9c0e66d77646e3b03f99691de34ac160b75d55dd487c3c5d,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 9,
        },
    },
    tx_data: SubmitData {
        data: BoundedVec(...),
    },
    tx_hash: 0xf049c9d4676589bf9c0e66d77646e3b03f99691de34ac160b75d55dd487c3c5d,
    tx_index: 1,
    block_hash: 0x960e7ffc08b34d2fa161160dd8373627f250fb965f9dfdb9e4f8031b02c5dcf0,
    block_number: 250,
}
```

## Submit Block Length Proposal

Origin Level: Root

### Interface

```rust
async fn submit_block_length_proposal(&self, rows: u32, cols: u32, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<SubmitBlockLengthProposalTx, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| rows      | u32         | false    | number of rows in block                         |
| cols      | u32         | false    | number of cols in block                         |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "da-submit-block-length-proposal"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();

	let rows = 128;
	let cols = 128;

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.data_availability
		.submit_block_length_proposal(rows, cols, wait_for, &account, Some(options))
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
async fn set_application_key(&self, old_key: Key, new_key: Key, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<SetApplicationKeyTx, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| oldKey    | Key         | false    | application key to be replaced                  |
| newKey    | Key         | false    | application key that will replace the old one   |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "da-set-application-key"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Key, Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
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

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.data_availability
		.set_application_key(old_key, new_key, wait_for, &account, Some(options))
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

## Set Submit Data Fee Modifier

Origin Level: Root

### Interface

```rust
async fn set_submit_data_fee_modifier(&self, modifier: DispatchFeeModifier, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<SetSubmitDataFeeModifierTx, String>;
```

#### Parameters

| parameter | type                | optional | description                                     |
| --------- | ------------------- | -------- | ----------------------------------------------- |
| modifier  | DispatchFeeModifier | false    | new fee modifier values                         |
| waitFor   | WaitFor             | false    | wait for block inclusion or finalization        |
| account   | KeyringPair         | false    | account that will send and sign the transaction |
| options   | Options             | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "da-set-submit-data-fee-proposal"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{DispatchFeeModifier, Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
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

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.data_availability
		.set_submit_data_fee_modifier(modifier, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `SetSubmitDataFeeModifierTx`.

```rust
SetSubmitDataFeeModifierTx {
    event: SubmitDataFeeModifierSet {
        value: DispatchFeeModifier {
            weight_maximum_fee: None,
            weight_fee_divider: Some(
                2,
            ),
            weight_fee_multiplier: None,
        },
    },
    events: ExtrinsicEvents {
        ext_hash: 0x0c64af6c695b887fabee5b8673bb6f2261b30b8020323295e15fccaa19315de6,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 8,
        },
    },
    tx_hash: 0x0c64af6c695b887fabee5b8673bb6f2261b30b8020323295e15fccaa19315de6,
    tx_index: 1,
    block_hash: 0xd7315dc33eecf9d14d840cc934bb625ea51832d4a8edccd0db9667631a88a6d3,
    block_number: 306,
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
async fn transfer_keep_alive(&self, dest: &str, value: u128, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<TransferKeepAliveTx, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| dest      | &str        | false    | account that will receive funds                 |
| value     | u128        | false    | amount that is send. 10^18 is equal to 1 AVL    |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "balance-transfer-keep-alive"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let dest: &str = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"; // Eve
	let amount = 1_000_000_000_000_000_000u128; // 1 Avail

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.balances
		.transfer_keep_alive(dest, amount, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `TransferKeepAliveTx`.

```rust
TransferKeepAliveTx {
    event: Transfer {
        from: AccountId32(...),
        to: AccountId32(...),
        amount: 1000000000000000000,
    },
    events: ExtrinsicEvents {
        ext_hash: 0x71e2bbd33fbdae2f22d0e5f389fc3b2fe146d8d6bfb679b301c506e36d6b3add,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 9,
        },
    },
    tx_hash: 0x71e2bbd33fbdae2f22d0e5f389fc3b2fe146d8d6bfb679b301c506e36d6b3add,
    tx_index: 1,
    block_hash: 0x9ddf13d41dfbfcf953f662457b14a2eeae0c7b2b0cdc67e6c6e8ce2935b779fc,
    block_number: 344,
}
```

## Transfer Allow Death

Origin Level: Signed

### Interface

```rust
async fn transfer_allow_death(&self, dest: &str, value: u128, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<TransferAllowDeathTx, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| dest      | string      | false    | account that will receive funds                 |
| value     | BN          | false    | amount that is send. 10^18 is equal to 1 AVL    |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "balance-transfer-allow-death"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"; // Eve
	let amount = 1_000_000_000_000_000_00u128; // 1 Avail

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.balances
		.transfer_allow_death(dest, amount, wait_for, &account, Some(options))
		.await?;

	if let Some(event) = &result.event2 {
		println!("Killed={}", event.account);
	}

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `TransferAllowDeathTx`.

```rust
TransferAllowDeathTx {
    event: Transfer {
        from: AccountId32(...),
        to: AccountId32(...),
        amount: 100000000000000000,
    },
    event2: None,
    events: ExtrinsicEvents {
        ext_hash: 0xae428af56f062d089d9988c3c217a745f71d8fc5f53c882211d795cf45037e71,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 9,
        },
    },
    tx_hash: 0xae428af56f062d089d9988c3c217a745f71d8fc5f53c882211d795cf45037e71,
    tx_index: 1,
    block_hash: 0xd97940fb917ce6b9d3ca4c6179204756e660a828c9ab449f5cb7b63440706656,
    block_number: 370,
}
```

## Transfer All

Origin Level: Signed

### Interface

```rust
async fn transfer_all(&self, dest: &str, keep_alive: bool, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<TransferAllTx, String>;
```

#### Parameters

| parameter | type        | optional | description                                      |
| --------- | ----------- | -------- | ------------------------------------------------ |
| dest      | &str        | false    | account that will receive funds                  |
| keepAlive | bool        | false    | if set to false it will reap the account as well |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization         |
| account   | KeyringPair | false    | account that will send and sign the transaction  |
| options   | Options     | true     | transaction parameters                           |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "balance-transfer-all"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"; // Eve
	let keep_alive = false;

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.balances
		.transfer_all(dest, keep_alive, wait_for, &account, Some(options))
		.await?;

	if let Some(event) = &result.event2 {
		println!("Killed={}", event.account);
	}

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `TransferAllTx`.

```rust
TransferAllTx {
    event: Transfer {
        from: AccountId32(...),
        to: AccountId32(...),
        amount: 9999999873434890300738572,
    },
    event2: Some(
        KilledAccount {
            account: AccountId32(...),
        },
    ),
    events: ExtrinsicEvents {
        ext_hash: 0x00b7eafbc9dbabced82b52914ef98260039e038bdd63942e142a7999e9d0aec4,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 10,
        },
    },
    tx_hash: 0x00b7eafbc9dbabced82b52914ef98260039e038bdd63942e142a7999e9d0aec4,
    tx_index: 1,
    block_hash: 0x1d4fc5850e24dcb41703958e11607243d989c25917aba63415e5dab2430d707e,
    block_number: 20,
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
async fn bond(&self, value: u128, payee: RewardDestination, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<BondTxSuccess, String>;
```

#### Parameters

| parameter | type              | optional | description                                             |
| --------- | ----------------- | -------- | ------------------------------------------------------- |
| value     | u128              | false    | amount that is bond. 10^18 is equal to 1 AVL            |
| payee     | RewardDestination | false    | Can be: "Staked", "Stash", "None" or an account address |
| waitFor   | WaitFor           | false    | wait for block inclusion or finalization                |
| account   | KeyringPair       | false    | account that will send and sign the transaction         |
| options   | Options           | true     | transaction parameters                                  |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "staking-bond"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, RewardDestination, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let value = 1_000_000_000_000_000_000u128 * 100_000u128; // 100_000 Avail
	let payee = RewardDestination::Staked;

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.staking
		.bond(value, payee, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `BondTxSuccess`.

```rust
BondTxSuccess {
    event: Bonded {
        stash: AccountId32(...),
        amount: 100000000000000000000000,
    },
    events: ExtrinsicEvents {
        ext_hash: 0x665f5ab61ceb3afa877eabe3b65a115a9e84e9be5520fcfb5b86b8cf87c5b25b,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 10,
        },
    },
    tx_hash: 0x665f5ab61ceb3afa877eabe3b65a115a9e84e9be5520fcfb5b86b8cf87c5b25b,
    tx_index: 1,
    block_hash: 0xb11af88f9b2d0d043c0fb886d71437e8f22a283ee8830c93e1ec21850c8b9caf,
    block_number: 21,
}
```

## Bond Extra

Origin Level: Signed

### Interface

```rust
async fn bond_extra(&self, max_additional: u128, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<BondExtraTxSuccess, String>;
```

#### Parameters

| parameter     | type        | optional | description                                               |
| ------------- | ----------- | -------- | --------------------------------------------------------- |
| maxAdditional | u128        | false    | additional amount that is bond. 10^18 is equal to 1 Avail |
| waitFor       | WaitFor     | false    | wait for block inclusion or finalization                  |
| account       | KeyringPair | false    | account that will send and sign the transaction           |
| options       | Options     | true     | transaction parameters                                    |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "staking-bond-extra"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let max_additional = 1_000_000_000_000_000_000u128; // 1 AVAIL

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.staking
		.bond_extra(max_additional, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `BondExtraTxSuccess`.

```rust
BondExtraTxSuccess {
    event: Bonded {
        stash: AccountId32(...),
        amount: 1000000000000000000,
    },
    events: ExtrinsicEvents {
        ext_hash: 0x290add36ab4f3643867e2d303d1fb231bf8268be1ef6d82d5a6d786f94f62c26,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 10,
        },
    },
    tx_hash: 0x290add36ab4f3643867e2d303d1fb231bf8268be1ef6d82d5a6d786f94f62c26,
    tx_index: 1,
    block_hash: 0x0ed5886e5da2a7c8e27d45f2d8de992554f9c7377887976edb3bb31ab0a02f62,
    block_number: 52,
}
```

## Chill

Origin Level: Signed

### Interface

```rust
async fn chill(&self, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<ChillTxSuccess, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "staking-chill"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice//stash").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.staking
		.chill(wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ChillTxSuccess`.

```rust
ChillTxSuccess {
    event: Some(
        Chilled {
            stash: AccountId32(...),
        },
    ),
    events: ExtrinsicEvents {
        ext_hash: 0x140765031a92c7636641bb119c6ade861bb9086e29a88eee728def4913cc66a4,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 9,
        },
    },
    tx_hash: 0x140765031a92c7636641bb119c6ade861bb9086e29a88eee728def4913cc66a4,
    tx_index: 1,
    block_hash: 0x2df031b45292c5e7c0ec62c9267aa6fcfab411d0b488f54ddec06fcabe813848,
    block_number: 76,
}
```

## Chill Other

Origin Level: Signed

### Interface

```rust
async fn chill_other(&self, stash: &str, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<ChillOtherTxSuccess, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| stash     | &str        | false    | address of stash account to chill               |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "staking-chill-other"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let stash = "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"; // Alice Stash

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.staking
		.chill_other(stash, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

## Nominate

Origin Level: Signed

### Interface

```rust
async fn nominate( &self, targets: &[String], wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<NominateTxSuccess, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| targets   | &[String]   | false    | list od addresses to nominate                   |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "staking-nominate"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
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

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.staking
		.nominate(&targets, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

```rust
Error: "Runtime error: Pallet error: Staking::NotController"
```

#### On Success

If the operation is successful, the function will return a object of type `NominateTxSuccess`.

```rust
NominateTxSuccess {
    events: ExtrinsicEvents {
        ext_hash: 0x6e0ae6fde353974f8b46aace441c49ba7ab135fa3743e0e1331d35c4528dacfb,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 8,
        },
    },
    tx_data: Nominate {
        targets: [
            Id(AccountId32(...)),
            Id(AccountId32(...)),
        ],
    },
    tx_hash: 0x6e0ae6fde353974f8b46aace441c49ba7ab135fa3743e0e1331d35c4528dacfb,
    tx_index: 1,
    block_hash: 0xd9b3c0e77d6b376b3963055f65156e30c63b4ecc54d6c113ecb431b9cf877bb8,
    block_number: 28,
}
```

## Unbond

Origin Level: Signed

### Interface

```rust
async fn unbond(&self, value: u128, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<UnbondTxSuccess, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| value     | u128        | false    | amount of tokens to unbond                      |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "staking-unbond"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let value = 1_000_000_000_000_000_000u128; // 1 Avail

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.staking
		.unbond(value, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `UnbondTxSuccess`.

```rust
UnbondTxSuccess {
    event: Unbonded {
        stash: AccountId32(...),
        amount: 1000000000000000000,
    },
    events: ExtrinsicEvents {
        ext_hash: 0x71239f5ae621a32049e2397872d85fd4c36c93cf05a18c9371805c01e2e17949,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 11,
        },
    },
    tx_hash: 0x71239f5ae621a32049e2397872d85fd4c36c93cf05a18c9371805c01e2e17949,
    tx_index: 1,
    block_hash: 0xc8fdf3834fa4f4e0d84089dbcbf0773e3f423beaecfadd217ad31eb793ac436c,
    block_number: 50,
}
```

## Validate

Origin Level: Signed

### Interface

```rust
async fn validate(&self, commission: u8, blocked: bool, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<ValidateTxSuccess, String>;
```

#### Parameters

| parameter  | type        | optional | description                                           |
| ---------- | ----------- | -------- | ----------------------------------------------------- |
| commission | u8          | false    | how much validator charge nominators in 0 - 100 range |
| blocked    | bool        | false    | whether or not this validator accepts nominations     |
| waitFor    | WaitFor     | false    | wait for block inclusion or finalization              |
| account    | KeyringPair | false    | account that will send and sign the transaction       |
| options    | Options     | true     | transaction parameters                                |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "staking-validate"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let commission = 100;
	let blocked = false;

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.staking
		.validate(commission, blocked, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ValidateTxSuccess`.

```rust
ValidateTxSuccess {
    event: ValidatorPrefsSet {
        stash: AccountId32(...),
        prefs: ValidatorPrefs {
            commission: Perbill(
                100,
            ),
            blocked: false,
        },
    },
    events: ExtrinsicEvents {
        ext_hash: 0x6da71de8764033f3f42d04b135b2d2b747904523005886d7682ba02309603abb,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 9,
        },
    },
    tx_hash: 0x6da71de8764033f3f42d04b135b2d2b747904523005886d7682ba02309603abb,
    tx_index: 1,
    block_hash: 0x99c6ef69cb02bbd93d0bbed8a6971896382f990a1e7352684bdf265e8f44c523,
    block_number: 16,
}
```

# Session

Runtime Component: Session\
Runtime Index: 11\
Interface Module Name: session

## Set Keys

Origin Level: Signed

### Interface

```rust
async fn set_keys(&self, keys: SessionKeys, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<SetKeysTxSuccess, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| keys      | SessionKeys | false    | session keys                                    |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "session-set-keys"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let keys = sdk.rpc.author.rotate_keys().await.unwrap();
	let keys = sdk.util.deconstruct_session_keys(keys)?;

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.session
		.set_keys(keys, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `BondTxSuccess`.

```rust
SetKeysTxSuccess {
    events: ExtrinsicEvents {
        ext_hash: 0x1f573b1b3b5b3de44dc6ca673101b50a652f44ee364c32283e370d553e47a129,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 8,
        },
    },
    tx_data: SetKeys {
        keys: SessionKeys {
            babe: Public(
                Public(...),
            ),
            grandpa: Public(
                Public(...),
            ),
            im_online: Public(
                Public(...),
            ),
            authority_discovery: Public(
                Public(...),
            ),
        },
        proof: [],
    },
    tx_hash: 0x1f573b1b3b5b3de44dc6ca673101b50a652f44ee364c32283e370d553e47a129,
    tx_index: 1,
    block_hash: 0x6ac39cc7e7452179b34a92376321b66a912f48faa3e1619de1e3f255a808ae8f,
    block_number: 124,
}
```

# Nomination Pools

Runtime Component: Nomination Pools\
Runtime Index: 36\
Interface Module Name: nominationPools

## Create

Origin Level: Signed

### Interface

```rust
async fn create(&self, amount: u128, root: &str, nominator: &str, bouncer: &str, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<CreateWithPoolIdTx, String>;
```

#### Parameters

| parameter | type        | optional | description                                        |
| --------- | ----------- | -------- | -------------------------------------------------- |
| amount    | u128        | false    | The amount of funds to delegate to the pool        |
| root      | &str        | false    | The account to set as [`PoolRoles::root`]          |
| nominator | &str        | false    | The account to set as the [`PoolRoles::nominator`] |
| bouncer   | &str        | false    | The account to set as the [`PoolRoles::bouncer`]   |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization           |
| account   | KeyringPair | false    | account that will send and sign the transaction    |
| options   | Options     | true     | transaction parameters                             |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "nomination-pools-create"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let amount = 1_000_000_000_000_000_000_000_000u128; // 1_000_000 Avail tokens
	let root = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"; // Alice
	let nominator = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"; // Alice
	let bouncer = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"; // Alice

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.create(
			amount,
			root,
			nominator,
			bouncer,
			wait_for,
			&account,
			Some(options),
		)
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `CreateTx`.

```rust
CreateTx {
    event: Created {
        depositor: AccountId32(...),
        pool_id: 1,
    },
    event2: Bonded {
        member: AccountId32(...),
        pool_id: 1,
        bonded: 1000000000000000000000000,
        joined: true,
    },
    events: ExtrinsicEvents {
        ext_hash: 0xd68cd496c042b1de9484c03160dcaea0b66d939a7293d457b721e908542ce4dd,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 19,
        },
    },
    tx_hash: 0xd68cd496c042b1de9484c03160dcaea0b66d939a7293d457b721e908542ce4dd,
    tx_index: 1,
    block_hash: 0x21119a080adf597abb22db237f8824a0dbd823feb6a809e2f2d9bb7872377e9d,
    block_number: 1,
}
```

## Create with Pool Id

Origin Level: Signed

### Interface

```rust
async fn create_with_pool_id(&self, amount: u128, root: &str, nominator: &str, bouncer: &str, pool_id: u32, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<CreateWithPoolIdTx, String>;
```

#### Parameters

| parameter | type        | optional | description                                        |
| --------- | ----------- | -------- | -------------------------------------------------- |
| amount    | u128        | false    | The amount of funds to delegate to the pool        |
| root      | &str        | false    | The account to set as [`PoolRoles::root`]          |
| nominator | &str        | false    | The account to set as the [`PoolRoles::nominator`] |
| bouncer   | &str        | false    | The account to set as the [`PoolRoles::bouncer`]   |
| pool_id   | u32         | false    | pool id                                            |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization           |
| account   | KeyringPair | false    | account that will send and sign the transaction    |
| options   | Options     | true     | transaction parameters                             |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "nomination-pools-create-with-pool-id"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let amount = 1_000_000_000_000_000_000_000_000u128; // 1_000_000 Avail tokens
	let root = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"; // Alice
	let nominator = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"; // Alice
	let bouncer = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"; // Alice
	let pool_id = 0;

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.create_with_pool_id(
			amount,
			root,
			nominator,
			bouncer,
			pool_id,
			wait_for,
			&account,
			Some(options),
		)
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `CreateWithPoolIdTx`.

```rust
CreateWithPoolIdTx {
    event: Created {
        depositor: AccountId32(...),
        pool_id: 0,
    },
    event2: Bonded {
        member: AccountId32(...),
        pool_id: 0,
        bonded: 1000000000000000000000000,
        joined: true,
    },
    events: ExtrinsicEvents {
        ext_hash: 0xaa16bad7378608bda89476353a61c1ae1ecc36166f0c5adda50cd563162889db,
        idx: 1,
        events: Events {
            event_bytes: [],
            start_idx: 1,
            num_events: 19,
        },
    },
    tx_hash: 0xaa16bad7378608bda89476353a61c1ae1ecc36166f0c5adda50cd563162889db,
    tx_index: 1,
    block_hash: 0xc04789228e6fa209119336ac33bcd6280b6b0c22e5ef9125c36b9f4a04e58adc,
    block_number: 32,
}
```

## Join

Origin Level: Signed

### Interface

```rust
async fn join(&self, amount: u128, pool_id: u32, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<JoinTx, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| amount    | u128        | false    | The amount of funds to delegate to the pool     |
| pool_id   | u32         | false    | pool id                                         |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "nomination-pools-join"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Bob").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let amount = 1_000_000_000_000_000_000_000_000u128; // 1_000_000 Avail tokens
	let pool_id = 1;

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.join(amount, pool_id, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `JoinTx`.

```rust
JoinTx {
    event: Bonded {
        member: AccountId32(...),
        pool_id: 1,
        bonded: 1000000000000000000000000,
        joined: true,
    },
    events: ExtrinsicEvents {
        ext_hash: 0x1c3c2412859e9c1d29a17cdaad48ff835bfbc7bb1b2bda5686d152f7c5145a40,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 12,
        },
    },
    tx_hash: 0x1c3c2412859e9c1d29a17cdaad48ff835bfbc7bb1b2bda5686d152f7c5145a40,
    tx_index: 1,
    block_hash: 0x67f28bfd6826522dc53ccfdec24dffbe9954ff4af8d96e81e983227af101786b,
    block_number: 24,
}
```

## Nominate

Origin Level: Signed

### Interface

```rust
async fn nominate(&self, pool_id: u32, validators: Vec<String>, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<NominateTx, String>;
```

#### Parameters

| parameter  | type        | optional | description                                     |
| ---------- | ----------- | -------- | ----------------------------------------------- |
| pool_id    | u32         | false    | pool id                                         |
| validators | String      | false    | list of validators to nominate                  |
| waitFor    | WaitFor     | false    | wait for block inclusion or finalization        |
| account    | KeyringPair | false    | account that will send and sign the transaction |
| options    | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "nomination-pools-nominate"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let pool_id = 1;
	let validators = vec![
		String::from("5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"), // Alice_Stash
		String::from("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"), // Bob
	];

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.nominate(pool_id, validators, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `NominateTx`.

```rust
NominateTx {
    events: ExtrinsicEvents {
        ext_hash: 0xde74e9df59143b84ed216e4e52fd58ec8bd557fae4b54d992a9abb1adf750446,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 8,
        },
    },
    tx_data: Nominate {
        pool_id: 1,
        validators: [
            AccountId32(...),
            AccountId32(...),
        ],
    },
    tx_hash: 0xde74e9df59143b84ed216e4e52fd58ec8bd557fae4b54d992a9abb1adf750446,
    tx_index: 1,
    block_hash: 0x599dd28c28fe3d892ebbe7dfdc315bee03fa2a3d968a5c53f4cd031656a94a9a,
    block_number: 86,
}
```

## Bond Extra

Origin Level: Signed

### Interface

```rust
async fn bond_extra(&self, extra: BondExtra<u128>, wait_for: WaitFor, account: &Keypair, options: Option<Options>,) -> Result<BondExtraTx, String>;
```

#### Parameters

| parameter | type            | optional | description                                                                                            |
| --------- | --------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| extra     | BondExtra<u128> | false    | Additional funds can come from either the free balance of the account, of from the accumulated rewards |
| waitFor   | WaitFor         | false    | wait for block inclusion or finalization                                                               |
| account   | KeyringPair     | false    | account that will send and sign the transaction                                                        |
| options   | Options         | true     | transaction parameters                                                                                 |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "nomination-pools-bond-extra"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{BondExtra, Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let extra = BondExtra::FreeBalance(1_000_000_000_000_000_000u128);

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.bond_extra(extra, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `BondExtraTx`.

```rust
BondExtraTx {
    event: Bonded {
        member: AccountId32(...),
        pool_id: 1,
        bonded: 1000000000000000000,
        joined: false,
    },
    events: ExtrinsicEvents {
        ext_hash: 0xcbf6ac8e1371a18ff7a888924abc51d486a48a366816282ef67f117dc4d9471d,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 12,
        },
    },
    tx_hash: 0xcbf6ac8e1371a18ff7a888924abc51d486a48a366816282ef67f117dc4d9471d,
    tx_index: 1,
    block_hash: 0xa594a8b39d484e352ea129ea982c80c326f4842946d2e5d9168c94e776d02ec9,
    block_number: 69,
}
```

## Set Commission

Origin Level: Signed

### Interface

```rust
async fn set_commission(&self, pool_id: u32, new_commission: Option<NewCommission>, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<SetCommissionTx, String>;
```

#### Parameters

| parameter      | type                  | optional | description                                                      |
| -------------- | --------------------- | -------- | ---------------------------------------------------------------- |
| pool_id        | u32                   | false    | pool id                                                          |
| new_commission | Option<NewCommission> | false    | if empty it removes the existing commission otherwise it sets it |
| waitFor        | WaitFor               | false    | wait for block inclusion or finalization                         |
| account        | KeyringPair           | false    | account that will send and sign the transaction                  |
| options        | Options               | true     | transaction parameters                                           |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "nomination-pools-set-commission"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, NewCommission, Nonce, Options, Perbill, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let pool_id = 1;
	let new_commission = NewCommission {
		payee: String::from("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"), // Alice
		amount: Perbill(10_000_000u32),                                          // 1%
	};

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.set_commission(
			pool_id,
			Some(new_commission),
			wait_for,
			&account,
			Some(options),
		)
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `SetCommissionTx`.

```rust
SetCommissionTx {
    event: PoolCommissionUpdated {
        pool_id: 1,
        current: Some(
            (
                Perbill(
                    10000000,
                ),
                AccountId32(...),
            ),
        ),
    },
    events: ExtrinsicEvents {
        ext_hash: 0x30f8f926e64a4aa7c55005026a261090a2d8a455efea9657a27d7b1aa668819e,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 9,
        },
    },
    tx_hash: 0x30f8f926e64a4aa7c55005026a261090a2d8a455efea9657a27d7b1aa668819e,
    tx_index: 1,
    block_hash: 0xb019177c166724ec40500a92775af5cf2fab4b1203363b2b3a5aba2c6bd13f75,
    block_number: 211,
}
```

## Set Metadata

Origin Level: Signed

### Interface

```rust
async fn set_metadata(&self, pool_id: u32, metadata: Vec<u8>, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<SetMetadataTx, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| pool_id   | u32         | false    | pool id                                         |
| metadata  | Vec<u8>     | false    | metadata                                        |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "nomination-pools-set-metadata"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let pool_id = 1;
	let metadata = String::from("This is metadata").as_bytes().to_vec();

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.set_metadata(pool_id, metadata, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `SetMetadataTx`.

```rust
SetMetadataTx {
    events: ExtrinsicEvents {
        ext_hash: 0x0b4c5b4dbc573e88fa96729622c8f3a303ae35db2144365ed951b55c9a9a0f9e,
        idx: 1,
        events: Events {
            event_bytes: [],
            start_idx: 1,
            num_events: 8,
        },
    },
    tx_hash: 0x0b4c5b4dbc573e88fa96729622c8f3a303ae35db2144365ed951b55c9a9a0f9e,
    tx_index: 1,
    block_hash: 0xd64694931e040911287026590e74327f9c053ffe94f8854e9bcdc4727c81b497,
    block_number: 876,
}
```

## Set Claim Permission

Origin Level: Signed

### Interface

```rust
async fn set_claim_permission(&self, permission: Permission, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<SetClaimPermissionTx, String>;
```

#### Parameters

| parameter  | type        | optional | description                                     |
| ---------- | ----------- | -------- | ----------------------------------------------- |
| permission | Permission  | false    | permission                                      |
| waitFor    | WaitFor     | false    | wait for block inclusion or finalization        |
| account    | KeyringPair | false    | account that will send and sign the transaction |
| options    | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "nomination-pools-set-claim-permission"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{
	nomination_pools_types::Permission, Keypair, Nonce, Options, SecretUri, WaitFor, SDK,
};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let permission = Permission::PermissionlessAll;

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.set_claim_permission(permission, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `SetClaimPermissionTx`.

```rust
SetClaimPermissionTx {
    events: ExtrinsicEvents {
        ext_hash: 0xf69cea124fe7823532821f73d3cb4c93dac58951b3bc28b770c54fc323b94bc0,
        idx: 1,
        events: Events {
            event_bytes: [],
            start_idx: 1,
            num_events: 8,
        },
    },
    tx_hash: 0xf69cea124fe7823532821f73d3cb4c93dac58951b3bc28b770c54fc323b94bc0,
    tx_index: 1,
    block_hash: 0x5344a7243307b20e0fee2badb84beebaf96e5c38d4e5d12c0475d4976737c26a,
    block_number: 945,
}
```

## Set State

Origin Level: Signed

### Interface

```rust
async fn set_state(&self, pool_id: u32, state: State, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<SetStateTx, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| pool_id   | u32         | false    | pool id                                         |
| state     | State       | false    | state                                           |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "nomination-pools-set-state"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{nomination_pools_types::State, Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let pool_id = 1;
	let state = State::Open;

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.set_state(pool_id, state, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `SetStateTx`.

```rust
SetStateTx {
    event: None,
    events: ExtrinsicEvents {
        ext_hash: 0xa5745e02a0a257e79b193efc66c9ac85138cb2a454eb52235687013430e1b932,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 8,
        },
    },
    tx_hash: 0xa5745e02a0a257e79b193efc66c9ac85138cb2a454eb52235687013430e1b932,
    tx_index: 1,
    block_hash: 0x7d9528f12db2980334c56181a94de9065b41b0482e31bd0d43fd57e32ab3f371,
    block_number: 1065,
}
```

## Unbond

Origin Level: Signed

### Interface

```rust
async fn unbond(&self, member_account: &str, unbonding_points: u128, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<UnbondTx, String>;
```

#### Parameters

| parameter        | type        | optional | description                                     |
| ---------------- | ----------- | -------- | ----------------------------------------------- |
| member_account   | &str        | false    | member account                                  |
| unbonding_points | u128        | false    | defines how many tokens will be unbond          |
| waitFor          | WaitFor     | false    | wait for block inclusion or finalization        |
| account          | KeyringPair | false    | account that will send and sign the transaction |
| options          | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "nomination-pools-unbond"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let unbonding_points = 1_000_000_000_000_000_000u128; // 1 Avail token
	let member_account = String::from("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"); // Alice

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.unbond(
			member_account,
			unbonding_points,
			wait_for,
			&account,
			Some(options),
		)
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `UnbondTx`.

```rust
UnbondTx {
    event: Some(
        Unbonded {
            member: AccountId32(...),
            pool_id: 1,
            balance: 1000000000000000000,
            points: 1000000000000000000,
            era: 3,
        },
    ),
    events: ExtrinsicEvents {
        ext_hash: 0x90c57843cd45ca0e5f45274543494a72a8948bb807c094b6b53a60d4381e194e,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 10,
        },
    },
    tx_hash: 0x90c57843cd45ca0e5f45274543494a72a8948bb807c094b6b53a60d4381e194e,
    tx_index: 1,
    block_hash: 0xf8a033a20e5394827ae075c240003749d6f8b65e9414435eafe4ca00395a0b7e,
    block_number: 50,
}
```

## Withdraw Unbond

Origin Level: Signed

### Interface

```rust
async fn withdraw_unbonded(&self, member_account: &str, num_slashing_spans: u32, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<WithdrawUnbondedTx, String>;
```

#### Parameters

| parameter          | type        | optional | description                                     |
| ------------------ | ----------- | -------- | ----------------------------------------------- |
| member_account     | &str        | false    | member account                                  |
| num_slashing_spans | u32         | false    | number of slashing spans                        |
| waitFor            | WaitFor     | false    | wait for block inclusion or finalization        |
| account            | KeyringPair | false    | account that will send and sign the transaction |
| options            | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "nomination-pools-withdraw-unbonded"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let member_account = String::from("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"); // Alice
	let num_slashing_spans = 0;

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.withdraw_unbonded(
			member_account,
			num_slashing_spans,
			wait_for,
			&account,
			Some(options),
		)
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `WithdrawUnbondedTx`.

```rust
WithdrawUnbondedTx {
    event: Some(
        Withdrawn {
            member: AccountId32(...),
            pool_id: 1,
            balance: 1000000000000000000,
            points: 1000000000000000000,
        },
    ),
    events: ExtrinsicEvents {
        ext_hash: 0x9a0dd9668568dc6e30dbf3513b9ba1efcbff029f2f996f8c166108f0ec9f2dbe,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 13,
        },
    },
    tx_hash: 0x9a0dd9668568dc6e30dbf3513b9ba1efcbff029f2f996f8c166108f0ec9f2dbe,
    tx_index: 1,
    block_hash: 0xbec264f965245c0293653c6370abb2b32644a4acd51f6e4d076f9eb178f90a27,
    block_number: 136,
}
```

## Chill

Origin Level: Signed

### Interface

```rust
async fn chill(&self, pool_id: u32, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<ChillTx, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| pool_id   | u32         | false    | pool id                                         |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "nomination-pools-chill"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let pool_id = 1;

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.chill(pool_id, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ChillTx`.

```rust
ChillTx {
    events: ExtrinsicEvents {
        ext_hash: 0xef47e23b303005a010bfc854a73143596849fc6e2b3db4b01d4b1f53800cde94,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 8,
        },
    },
    tx_hash: 0xef47e23b303005a010bfc854a73143596849fc6e2b3db4b01d4b1f53800cde94,
    tx_index: 1,
    block_hash: 0xf8220ea5fb98b27833aeaab9ef8a95dca39dfbb55ec7b0467fb9162e2eb082d5,
    block_number: 183,
}
```

## Claim Payout

Origin Level: Signed

### Interface

```rust
async fn claim_payout(&self, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<ClaimPayoutTx, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "nomination-pools-claim-payout"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.claim_payout(wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ClaimPayoutTx`.

```rust
ClaimPayoutTx {
    event: Some(
        PaidOut {
            member: AccountId32(...),
            pool_id: 1,
            payout: 292545391972746000,
        },
    ),
    events: ExtrinsicEvents {
        ext_hash: 0x6068c14cbd3cdbac8a5cf40f2cb943f8c3f213d1c199181bf957af0de3df9411,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 10,
        },
    },
    tx_hash: 0x6068c14cbd3cdbac8a5cf40f2cb943f8c3f213d1c199181bf957af0de3df9411,
    tx_index: 1,
    block_hash: 0xfdb4766c474980eeb2dc3c7c0f08ca0f7e38f66dcb8cf98ef0d55835062ee758,
    block_number: 222,
}
```

## Claim Commission

Origin Level: Signed

### Interface

```rust
async fn claim_commission(&self, pool_id: u32, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<ClaimCommissionTx, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| pool_id   | u32         | false    | pool id                                         |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "nomination-pools-claim-commission"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let pool_id = 1;

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.claim_commission(pool_id, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ClaimCommissionTx`.

```rust
ClaimCommissionTx {
    event: PoolCommissionClaimed {
        pool_id: 1,
        commission: 2952048955361375559,
    },
    events: ExtrinsicEvents {
        ext_hash: 0xd5c24d3b5c6fa81ab0d60b39ad77084f4dd704c7b31991636c8662b6d543403d,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 10,
        },
    },
    tx_hash: 0xd5c24d3b5c6fa81ab0d60b39ad77084f4dd704c7b31991636c8662b6d543403d,
    tx_index: 1,
    block_hash: 0x14e2357f8c47ca7c2c8b236547c0ffbbdc2ee0daa613b794ac48e31c8def8191,
    block_number: 253,
}
```

## Claim Payout Other

Origin Level: Signed

### Interface

```rust
async fn claim_payout_other(&self, other: &str, wait_for: WaitFor, account: &Keypair, options: Option<Options>) -> Result<ClaimPayoutOtherTx, String>;
```

#### Parameters

| parameter | type        | optional | description                                     |
| --------- | ----------- | -------- | ----------------------------------------------- |
| other     | &str        | false    | other account to claim payout                   |
| waitFor   | WaitFor     | false    | wait for block inclusion or finalization        |
| account   | KeyringPair | false    | account that will send and sign the transaction |
| options   | Options     | true     | transaction parameters                          |

### Minimal Example

#### Cargo.toml

```rust
[package]
name = "nomination-pools-claim-payout-other"
edition = "2021"

[dependencies]
avail-rust = { git = "https://github.com/availproject/avail" }
tokio = { version = "1.38.0", features = ["rt-multi-thread"] }
```

#### main.rs

```rust
use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Alice").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let other = "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL";

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.claim_payout_other(other, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ClaimPayoutOtherTx`.

```rust
ClaimPayoutOtherTx {
    event: Some(
        PaidOut {
            member: AccountId32(...),
            pool_id: 1,
            payout: 2659503563388600000,
        },
    ),
    events: ExtrinsicEvents {
        ext_hash: 0x5d93a79b019521261d83ec79a8768234c78d84f5984bfdc230ab67f3357cd260,
        idx: 1,
        events: Events {
            event_bytes: [...],
            start_idx: 1,
            num_events: 10,
        },
    },
    tx_hash: 0x5d93a79b019521261d83ec79a8768234c78d84f5984bfdc230ab67f3357cd260,
    tx_index: 1,
    block_hash: 0x080c20e46e2c26e1a148ce8f862a7b7c709cd89d11a77907aa8807a921a7a778,
    block_number: 285,
}
```
