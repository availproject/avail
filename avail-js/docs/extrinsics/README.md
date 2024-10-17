# Balances

Runtime Component: Balances\
Runtime Index: 6\
Interface Module Name: balances

## Transfer All

Origin Level: Signed

### Interface

```js
function transferAll(dest: string, keepAlive: boolean, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<TransferAllTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| dest      | string             | false    | Account that will receive funds                               |
| keepAlive | boolean            | false    | if set to false it will reap the account as well              |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // Eve
  const keepAlive = true

  const result = await sdk.tx.balances.transferAll(dest, keepAlive, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `TransferAllTx`.

```json
{
  "event": {
    "from": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "to": "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
    "amount": "9999999873433871068464733"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x48b0e56ad89280f9a94d1f1ab48af2a9cf25fe4c9b41b916e0efa983fa5726c4",
    "txIndex": 1,
    "blockHash": "0x3a4fe6a2618db91cd1786f853c6c0548bcefa15849c810a37bbfcfb7eedb5b7f",
    "blockNumber": 2
  }
}
```

## Transfer All No Wait

Origin Level: Signed

### Interface

```js
function transferAllNoWait(dest: string, keepAlive: boolean, account: KeyringPair, options?: TransactionOptions): Promise<H256>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| dest      | string             | false    | Account that will receive funds                               |
| keepAlive | boolean            | false    | if set to false it will reap the account as well              |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // Eve
  const keepAlive = true

  const txHash = await sdk.tx.balances.transferAllNoWait(dest, keepAlive, account)

  console.log(txHash)
  process.exit()
}
main()
```

### Example Output

The function will return a object of type `H256`.

```json
"0x3481a87e6ebfd2af2fad83fb0305c93e8cf59b6eacc7fdc37d3cf4a6ab0929b1"
```

## Transfer Allow Death

Origin Level: Signed

### Interface

```js
function transferAllowDeath(dest: string, value: BN, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<TransferAllowDeathTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| dest      | string             | false    | Account that will receive funds                               |
| value     | BN                 | false    | Amount that is send. 10^18 is equal to 1 AVL                  |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // Eve
  const amount = SDK.oneAvail()

  const result = await sdk.tx.balances.transferAllowDeath(dest, amount, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `TransferAllowDeathTx`.

```json
{
  "event": {
    "from": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "to": "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
    "amount": "1000000000000000000"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x03218ef4ef981238f31ff83d369bbb020b5f3fd30a1009c42993c99069062731",
    "txIndex": 1,
    "blockHash": "0xf2f47bb6093e56a22bd23ff39c320e2c1282d397045240921546fd4dfe49fc49",
    "blockNumber": 3
  }
}
```

## Transfer Allow Death No Wait

Origin Level: Signed

### Interface

```js
function transferAllowDeathNoWait(dest: string, value: BN, account: KeyringPair, options?: TransactionOptions): Promise<H256>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| dest      | string             | false    | Account that will receive funds                               |
| value     | BN                 | false    | Amount that is send. 10^18 is equal to 1 AVL                  |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // Eve
  const amount = SDK.oneAvail()

  const txHash = await sdk.tx.balances.transferAllowDeathNoWait(dest, amount, account)

  console.log(JSON.stringify(txHash, null, 2))
  process.exit()
}
main()
```

### Example Output

The function will return a object of type `H256`.

```json
"0x7ee1ba58c7e71a3222850e01d8fd3661b97935f38d959d0ad704ad632bd7e6b5"
```

## Transfer Keep Alive

Origin Level: Signed

### Interface

```js
function transferKeepAlive(dest: string, value: BN, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<TransferKeepAliveTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| dest      | string             | false    | Account that will receive funds                               |
| value     | BN                 | false    | Amount that is send. 10^18 is equal to 1 AVL                  |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // Eve
  const amount = SDK.oneAvail()

  const result = await sdk.tx.balances.transferKeepAlive(dest, amount, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `TransferKeepAliveTx`.

```json
{
  "event": {
    "from": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "to": "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
    "amount": "1000000000000000000"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x84a745ebb9642dcf382935c882519bfa6b957898365fb56a3499543d157e4b08",
    "txIndex": 1,
    "blockHash": "0x79e32fd8129d052b90f2c01c60c5c81e08fe461bdef1d403712df20cee5b6303",
    "blockNumber": 56
  }
}
```

## Transfer Keep Alive No Wait

Origin Level: Signed

### Interface

```js
function transferKeepAliveNoWait(dest: string, value: BN, account: KeyringPair, options?: TransactionOptions): Promise<H256>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| dest      | string             | false    | Account that will receive funds                               |
| value     | BN                 | false    | Amount that is send. 10^18 is equal to 1 AVL                  |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // Eve
  const amount = SDK.oneAvail()

  const txHash = await sdk.tx.balances.transferKeepAliveNoWait(dest, amount, account)

  console.log(JSON.stringify(txHash, null, 2))
  process.exit()
}
main()
```

### Example Output

The function will return a object of type `H256`.

```json
"0xa6700ad4a056b9096c940e464b41acc7ac13c4df9ea5e214a3549794916d789a"
```

# Data Availability

Runtime Component: DataAvailability\
Runtime Index: 29\
Interface Module Name: dataAvailability

## Create Application Key

Origin Level: Signed

### Interface

```js
function createApplicationKey(key: string, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<CreateApplicationKeyTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| key       | string             | false    | name of the application key                                   |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const key = "MyAwesomeKey"

  const result = await sdk.tx.dataAvailability.createApplicationKey(key, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `CreateApplicationKeyTx`.

```json
{
  "event": {
    "key": "0x4d79417765736f6d654b6579",
    "owner": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "id": "10"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x8bb6e5167bb0808a0e12236ed89aba5b8147737e2f4e35580cc157a5a17f8ae3",
    "txIndex": 1,
    "blockHash": "0xba5cdf11ad6847617d58d16d931436a5e1fba2191bb50f08912107dcd8440189",
    "blockNumber": 94
  }
}
```

## Create Application Key No Wait

Origin Level: Signed

### Interface

```js
function createApplicationKeyNoWait(key: string, account: KeyringPair, options?: TransactionOptions): Promise<H256>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| key       | string             | false    | name of the application key                                   |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const key = "MyAwesomeKey"

  const txHash = await sdk.tx.dataAvailability.createApplicationKeyNoWait(key, account)

  console.log(JSON.stringify(txHash, null, 2))
  process.exit()
}
main()
```

### Example Output

The function will return a object of type `H256`.

```json
"0x8d523a3b36327651bb11e619d8a1fceaa205cb414b411e6cd00116272278042e"
```

## Submit Data

Origin Level: Signed

### Interface

```js
function submitData(data: string | Bytes, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<SubmitDataTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| data      | string             | false    | data to be submitted                                          |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, TransactionOptions } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const data = "My Awesome Data"

  const options: TransactionOptions = { app_id: 1 }
  const result = await sdk.tx.dataAvailability.submitData(data, WaitFor.BlockInclusion, account, options)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `SubmitDataTx`.

```json
{
  "txData": {
    "data": "4d7920417765736f6d652044617461"
  },
  "event": {
    "who": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "dataHash": "0x8846d900ea89aab9bce96402846c0ac74a853acc00cb99ff5ddb1a0f052594bd"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0xc189150ae2a0ebe6df1ec20b8096bbffc44e2ec5c56045049ab92e3e0a781565",
    "txIndex": 1,
    "blockHash": "0x0221659ca85a47bb11a8926c52e9273b908193a2420644c59f65d9da3754535a",
    "blockNumber": 2
  }
}
```

## Submit Data No Wait

Origin Level: Signed

### Interface

```js
function submitDataNoWait(data: string | Bytes, account: KeyringPair, options?: TransactionOptions): Promise<H256>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| data      | string             | false    | data to be submitted                                          |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, Keyring, TransactionOptions } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const data = "My Awesome Data"

  const options: TransactionOptions = { app_id: 1 }
  const txHash = await sdk.tx.dataAvailability.submitDataNoWait(data, account, options)

  console.log(JSON.stringify(txHash, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `SubmitDataTx`.

```json
"0x8643a0f94f1928eb542aaa44aacf943c7fe50ab4949c53a934debf006049bc7e"
```

# Multisig

Runtime Component: Multisig\
Runtime Index: 34\
Interface Module Name: multisig

## Approving As Multi

Origin Level: Signed

### Interface

```js
function approveAsMulti(threshold: number, otherSignatures: string[], timepoint: MultisigTimepoint | null, callHash: string, maxWeight: Weight, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<ApproveAsMultiTx, TransactionFailed>>;
```

#### Parameters

| parameter       | type                      | optional | description                                                                                                                                                                                                  |
| --------------- | ------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| threshold       | number                    | false    | The total number of approvals for this dispatch before it is executed.                                                                                                                                       |
| otherSignatures | string[]                  | false    | The accounts (other than the sender) who can approve this dispatch. May not be empty.                                                                                                                        |
| timepoint       | MultisigTimepoint or null | false    | If this is the first approval, then this must be `None`. If it is not the first approval, then it must be `Some`, with the timepoint (block number and transaction index) of the first approval transaction. |
| callHash        | string                    | false    | The hash of the call to be executed                                                                                                                                                                          |
| maxWeight       | Weight                    | false    | This should be equal to the weight of the call                                                                                                                                                               |
| waitFor         | WaitFor                   | false    | Wait for block inclusion or finalization                                                                                                                                                                     |
| account         | KeyringPair               | false    | Account that will send and sign the transaction                                                                                                                                                              |
| options         | TransactionOptions        | true     | Can be used to set nonce, app id or other transaction options                                                                                                                                                |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "./../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Multisig Signatures
  const alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const bobAddress = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"

  // Create Multisig Account
  const threshold = 2
  const multisigAddress = sdk.util.generateMultisig([alice.address, bobAddress], threshold)

  // Define what action will be taken by the multisig account
  const amount = SDK.oneAvail()
  const call = sdk.api.tx.balances.transferKeepAlive(multisigAddress, amount)
  // Data needed for multisig approval and execution
  const callHash = call.method.hash.toString()
  const maxWeight = (await call.paymentInfo(alice.address)).weight

  // Create New Multisig
  console.log("Alice is creating a Multisig Transaction...")
  const call1signatures = sdk.util.sortMultisigAddresses([bobAddress])
  const result = await sdk.tx.multisig.approveAsMulti(
    threshold,
    call1signatures,
    null,
    callHash,
    maxWeight,
    WaitFor.BlockInclusion,
    alice,
  )
  if (result.isErr()) {
    console.log(result.error)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ApproveAsMultiTx`.

```json
{
  "event": {
    "approving": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "multisig": "5F3QVbS78a4aTYLiRAD8N3czjqVoNyV42L19CXyhqUMCh4Ch",
    "callHash": "0x239258ef81456d5dbf5bf8a37709c7cf14c74ea7ed89961f47c7f3701de1e86b"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x7beb1e4edb785c57533e0793f0d9051bbe6290e9551ee7e907d9c34ba9cad1a2",
    "txIndex": 1,
    "blockHash": "0xb23ef6eada32c5c19e75d3c0713c62ec88153613ceab70b28cac9ccd743e6b60",
    "blockNumber": 103
  }
}
```

## As Multi

Origin Level: Signed

### Interface

```js
function asMulti(threshold: number, otherSignatures: string[], timepoint: MultisigTimepoint | null, call: string, maxWeight: Weight, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<AsMultiTx, TransactionFailed>>;
```

#### Parameters

| parameter       | type                      | optional | description                                                                                                                                                                                                  |
| --------------- | ------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| threshold       | number                    | false    | The total number of approvals for this dispatch before it is executed.                                                                                                                                       |
| otherSignatures | string[]                  | false    | The accounts (other than the sender) who can approve this dispatch. May not be empty.                                                                                                                        |
| timepoint       | MultisigTimepoint or null | false    | If this is the first approval, then this must be `None`. If it is not the first approval, then it must be `Some`, with the timepoint (block number and transaction index) of the first approval transaction. |
| call            | string                    | false    | The call to be executed.                                                                                                                                                                                     |
| maxWeight       | Weight                    | false    | This should be equal to the weight of the call                                                                                                                                                               |
| waitFor         | WaitFor                   | false    | Wait for block inclusion or finalization                                                                                                                                                                     |
| account         | KeyringPair               | false    | Account that will send and sign the transaction                                                                                                                                                              |
| options         | TransactionOptions        | true     | Can be used to set nonce, app id or other transaction options                                                                                                                                                |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, MultisigTimepoint } from "./../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Multisig Signatures
  const bob = new Keyring({ type: "sr25519" }).addFromUri("//Bob")
  const aliceAddress = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

  // Create Multisig Account
  const threshold = 2
  const multisigAddress = sdk.util.generateMultisig([aliceAddress, bob.address], threshold)

  // Define what action will be taken by the multisig account
  const amount = SDK.oneAvail()
  const call = sdk.api.tx.balances.transferKeepAlive(multisigAddress, amount)
  // Data needed for multisig approval and execution
  const callData = call.unwrap().toHex()
  const maxWeight = (await call.paymentInfo(aliceAddress)).weight
  const timepoint: MultisigTimepoint = { height: 4, index: 1 }

  // Approving and executing Multisig transaction
  console.log("Bob is approving and executing the existing Multisig Transaction...")
  const call2signatures = sdk.util.sortMultisigAddresses([aliceAddress])
  const secondResult = await sdk.tx.multisig.asMulti(
    threshold,
    call2signatures,
    timepoint,
    callData,
    maxWeight,
    WaitFor.BlockInclusion,
    bob,
  )
  if (secondResult.isErr()) {
    console.log(secondResult.error)
    process.exit(1)
  }

  console.log(JSON.stringify(secondResult.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `AsMultiTx`.

```json
{
  "event": {
    "approving": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    "timepoint": {
      "height": 103,
      "index": 1
    },
    "multisig": "5F3QVbS78a4aTYLiRAD8N3czjqVoNyV42L19CXyhqUMCh4Ch",
    "callHash": "0x239258ef81456d5dbf5bf8a37709c7cf14c74ea7ed89961f47c7f3701de1e86b",
    "result": "Ok"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0xada53aa3d7e7891404228c07251f51c86214143e57205be10b6f55b1dd63b933",
    "txIndex": 1,
    "blockHash": "0x84a33cb83078795f8be128835c5699f20a314119c8b6efa432cc626fa6ac8812",
    "blockNumber": 150
  }
}
```

# Nomination Pools

Runtime Component: Nomination Pools\
Runtime Index: 36\
Interface Module Name: nominationPools

## Bond Extra

Origin Level: Signed

### Interface

```js
function bondExtra(extra: BondExtra, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<BondExtraTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                                                            |
| --------- | ------------------ | -------- | ------------------------------------------------------------------------------------------------------ |
| extra     | BondExtra          | false    | Additional funds can come from either the free balance of the account, of from the accumulated rewards |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                                                               |
| account   | KeyringPair        | false    | Account that will send and sign the transaction                                                        |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options                                          |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BondExtra, BN } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const amount = SDK.oneAvail().mul(new BN(10000)) // 10_000 Avail
  const bondExtra = { FreeBalance: amount } as BondExtra

  const result = await sdk.tx.nominationPools.bondExtra(bondExtra, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `BondExtraTx`.

```json
{
  "event": {
    "member": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "poolId": "1",
    "bonded": "10000000000000000000000",
    "joined": "false"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0xc09bdfbc1703763850a2f4255ccb0dd7d7e9f4b793d2f9f05782b9e3c63682d9",
    "txIndex": 1,
    "blockHash": "0xe06e35e61e2b59900e22d627dd8cd9cab39c87a304a40aba0fce87936e921843",
    "blockNumber": 112
  }
}
```

## Chill

Origin Level: Signed

### Interface

```js
function chill(poolId: number, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<ChillTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| poolId    | number             | false    | pool id                                                       |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const poolId = 1

  const result = await sdk.tx.nominationPools.chill(poolId, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ChillTx`.

```json
{
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x678fd95154114bb899598f8ef4413e3fdb5a90317a132d5a49f868ffb029cecf",
    "txIndex": 1,
    "blockHash": "0xa66f5eacd4c4294ba6679c868d90df11fccf5c85c773e8554856882ac64b4be7",
    "blockNumber": 1106
  }
}
```

## Claim Commission

Origin Level: Signed

### Interface

```js
function claimCommission(poolId: number, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<ClaimCommissionTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| poolId    | number             | false    | pool id                                                       |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const poolId = 1

  const result = await sdk.tx.nominationPools.claimCommission(poolId, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ClaimCommissionTx`.

```json
{
  "event": {
    "poolId": "1",
    "commission": "7652149502759813012"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x52292c2af7c2505c0bd39f5f1308bc10d92db7117f1b9fec208dc778e0c49654",
    "txIndex": 1,
    "blockHash": "0xdceaa5b12517f9634ad568e355061e8e13a8af25074e8966381c92fcea214285",
    "blockNumber": 418
  }
}
```

## Claim Payout Other

Origin Level: Signed

### Interface

```js
function claimPayoutOther(other: string, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<ClaimPayoutOtherTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| other     | &str               | false    | other account to claim payout                                 |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const other = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" // Bob

  const result = await sdk.tx.nominationPools.claimPayoutOther(other, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ClaimPayoutOtherTx`.

```json
{
  "event": {
    "member": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    "poolId": "1",
    "payout": "8198513381719500000"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0xe14015b25afc66c4954dda18631b3fd5efaeba80989070ed12712ca312dbec40",
    "txIndex": 1,
    "blockHash": "0x3abde836ab66ac38bc3ca260f081d370f1c15094f43048423e0995170569a51a",
    "blockNumber": 594
  }
}
```

## Claim Payout

Origin Level: Signed

### Interface

```js
function claimPayout(waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<ClaimPayoutTx, TransactionFailed>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")

  const result = await sdk.tx.nominationPools.claimPayout(WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ClaimPayoutTx`.

```json
{
  "event": {
    "member": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "poolId": "1",
    "payout": "4008268787159890000"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x15b50cb2a3885432811ba1417b7715e69e05b8d4dd8f7c9951f0b4f7a8ba5a61",
    "txIndex": 1,
    "blockHash": "0xdd24eca2111c6b7eea4a34c9bed02cfa6effde65c65c3dc66fb13c88e2fe6985",
    "blockNumber": 426
  }
}
```

## Create with Pool Id

Origin Level: Signed

### Interface

```js
function createWithPoolId(amount: BN, root: string, nominator: string, bouncer: string, poolId: number, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<CreateWithPoolIdTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| amount    | BN                 | false    | The amount of funds to delegate to the pool                   |
| root      | string             | false    | The account to set as [`PoolRoles::root`]                     |
| nominator | string             | false    | The account to set as the [`PoolRoles::nominator`]            |
| bouncer   | string             | false    | The account to set as the [`PoolRoles::bouncer`]              |
| poolId    | number             | false    | pool id                                                       |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob")
  const amount = SDK.oneAvail().mul(new BN(10000)) // 10_000 Avail

  const root: string = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // Alice
  const nominator: string = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // Alice
  const bouncer: string = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // Alice
  const poolId = 0

  const result = await sdk.tx.nominationPools.createWithPoolId(
    amount,
    root,
    nominator,
    bouncer,
    poolId,
    WaitFor.BlockInclusion,
    account,
  )
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `CreateWithPoolIdTx`.

```json
{
  "event": {
    "depositor": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    "poolId": "0"
  },
  "event2": {
    "member": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    "poolId": "0",
    "bonded": "10000",
    "joined": "true"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x6b50caed7950e67934cabbf88a1f7dc2e7e995ac608402f91a4db19be0da5c41",
    "txIndex": 1,
    "blockHash": "0xc06df7dbb1e404f54499f942479ddcffc92665c021ea07c2798fc2f354f403d3",
    "blockNumber": 6
  }
}
```

## Create

Origin Level: Signed

### Interface

```js
function create(amount: BN, root: string, nominator: string, bouncer: string, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<CreateTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| amount    | BN                 | false    | The amount of funds to delegate to the pool                   |
| root      | string             | false    | The account to set as [`PoolRoles::root`]                     |
| nominator | string             | false    | The account to set as the [`PoolRoles::nominator`]            |
| bouncer   | string             | false    | The account to set as the [`PoolRoles::bouncer`]              |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const amount = SDK.oneAvail().mul(new BN(10000)) // 10_000 Avail

  const root: string = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // Alice
  const nominator: string = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // Alice
  const bouncer: string = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // Alice

  const result = await sdk.tx.nominationPools.create(amount, root, nominator, bouncer, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `PoolCreateTxSuccess`.

```json
{
  "event": {
    "depositor": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "poolId": "1"
  },
  "event2": {
    "member": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "poolId": "1",
    "bonded": "10000",
    "joined": "true"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x9f019464c676682d9cbfb62814d019a85738e01b0ce92b18fa77878f80e16f66",
    "txIndex": 1,
    "blockHash": "0xdaa49f09b8c519bbd4c52ed05652d3de3f777afa3ac85db5c697cf705907e2d2",
    "blockNumber": 5
  }
}
```

## Join

Origin Level: Signed

### Interface

```js
function join(amount: BN, poolId: number, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<JoinTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| amount    | BN                 | false    | The amount of funds to delegate to the pool                   |
| poolId    | number             | false    | pool id                                                       |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob")
  const amount = SDK.oneAvail().mul(new BN(10000)) // 10_000 Avail
  const poolId = 1

  const result = await sdk.tx.nominationPools.join(amount, poolId, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `JoinTx`.

```json
{
  "event": {
    "member": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    "poolId": "1",
    "bonded": "10000",
    "joined": "true"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x06baecbb8680e90d025d1fd08044d0d251054a89e82dd460022bdf3796020050",
    "txIndex": 1,
    "blockHash": "0x82078130da46adacf5bdff86618ab6e1c443fda6d883d9fcf967a41a2e29d612",
    "blockNumber": 19
  }
}
```

## Nominate

Origin Level: Signed

### Interface

```js
function nominate(poolId: number, validators: string[], waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<NominateTx, TransactionFailed>>;
```

#### Parameters

| parameter  | type               | optional | description                                                   |
| ---------- | ------------------ | -------- | ------------------------------------------------------------- |
| poolId     | number             | false    | pool id                                                       |
| validators | string[]           | false    | list of validators to nominate                                |
| waitFor    | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account    | KeyringPair        | false    | Account that will send and sign the transaction               |
| options    | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const validators: string[] = [
    "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
    "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
  ]
  const poolId = 1

  const result = await sdk.tx.nominationPools.nominate(poolId, validators, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `NominateTx`.

```json
{
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x98b993baf90183d85dece9357d3bc32311f4201b015b63845a13dbc22bf22370",
    "txIndex": 1,
    "blockHash": "0x84ef5a0ada4af71358ee701a2500bce7f6688efb554c32ba1a30c459f64d5370",
    "blockNumber": 48
  }
}
```

## Set Claim Permission

Origin Level: Signed

### Interface

```js
function setClaimPermission(permission: ClaimPermission, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<SetClaimPermissionTx, TransactionFailed>>;
```

#### Parameters

| parameter  | type               | optional | description                                                   |
| ---------- | ------------------ | -------- | ------------------------------------------------------------- |
| permission | ClaimPermission    | false    | permission type                                               |
| waitFor    | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account    | KeyringPair        | false    | Account that will send and sign the transaction               |
| options    | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, ClaimPermission } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob")
  const permission: ClaimPermission = "PermissionlessAll"

  const result = await sdk.tx.nominationPools.setClaimPermission(permission, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `SetClaimPermissionTx`.

```json
{
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x3096fb312d22f2aa4fa51532fbae4b812a99d60c85a0750662816146bce997bc",
    "txIndex": 1,
    "blockHash": "0xfb41995f2e5aaa1b92126792dfc1e7b2863773fa4865eca2d96f458da17d6f82",
    "blockNumber": 249
  }
}
```

## Set Commission

Origin Level: Signed

### Interface

```js
function setCommission(poolId: number, newCommission: NewCommission | null, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<CommissionTx, TransactionFailed>>;
```

#### Parameters

| parameter     | type                  | optional | description                                                      |
| ------------- | --------------------- | -------- | ---------------------------------------------------------------- |
| poolId        | number                | false    | pool id                                                          |
| newCommission | NewCommission or Null | false    | if empty it removes the existing commission otherwise it sets it |
| waitFor       | WaitFor               | false    | Wait for block inclusion or finalization                         |
| account       | KeyringPair           | false    | Account that will send and sign the transaction                  |
| options       | TransactionOptions    | true     | Can be used to set nonce, app id or other transaction options    |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, NewCommission } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const poolId = 1
  const newCommission: NewCommission = { amount: 25, payee: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" }

  const result = await sdk.tx.nominationPools.setCommission(poolId, newCommission, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `CommissionTx`.

```json
{
  "event": {
    "poolId": "1",
    "current": "[250000000,\"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY\"]"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x8256693f9648abe0d47b12380cdcfcb56c3078080216a7487f0b34d7ff80e047",
    "txIndex": 1,
    "blockHash": "0xedf037ef9de56cba1a81a45bb5ed5b0ec3424725a521c6255976a68d5638015e",
    "blockNumber": 552
  }
}
```

## Set Metadata

Origin Level: Signed

### Interface

```js
function setMetadata(poolId: number, metadata: string, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<SetMetadataTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| poolId    | number             | false    | pool id                                                       |
| metadata  | string             | false    | metadata                                                      |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const poolId = 1
  const metadata = "My Metadata"

  const result = await sdk.tx.nominationPools.setMetadata(poolId, metadata, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `SetMetadataTx`.

```json
{
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x59121cd7e320acab280a1ad0d8b8385581ca7e03d973477ae812b1d967e2cb1d",
    "txIndex": 1,
    "blockHash": "0x6f19a8c993dc33676605a1dfabbb5a1008929ee965772a3b3edd12f9fe1eb296",
    "blockNumber": 369
  }
}
```

## Set State

Origin Level: Signed

### Interface

```js
function setState(poolId: number, state: PoolState, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<SetStateTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| poolId    | number             | false    | pool id                                                       |
| state     | PoolState          | false    | "Open" or "Blocked" or "Destroying"                           |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, PoolState } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const poolId = 2
  const state: PoolState = "Blocked"

  const result = await sdk.tx.nominationPools.setState(poolId, state, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `SetStateTx`.

```json
{
  "event": {
    "poolId": "1",
    "newState": "Blocked"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x921a187171a1d15a0779be75b9ecadc62eb1446f798bd0f4f1542700f32c724c",
    "txIndex": 1,
    "blockHash": "0xf2b85aae8a41eb4a27c72dfb2c4bce258213b829ef5f97f416a24d3989e7b3da",
    "blockNumber": 184
  }
}
```

## Unbond

Origin Level: Signed

### Interface

```js
function unbond(memberAccount: string, unbondingPoints: BN, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<UnbondTx, TransactionFailed>>;
```

#### Parameters

| parameter       | type               | optional | description                                                   |
| --------------- | ------------------ | -------- | ------------------------------------------------------------- |
| memberAccount   | string             | false    | member account                                                |
| unbondingPoints | BN                 | false    | defines how many tokens will be unbond                        |
| waitFor         | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account         | KeyringPair        | false    | Account that will send and sign the transaction               |
| options         | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const memberAccount = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // Alice
  const unbondingPoints = SDK.oneAvail().mul(new BN(100)) // 100 Avail

  const result = await sdk.tx.nominationPools.unbond(memberAccount, unbondingPoints, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `UnbondTx`.

```json
{
  "event": {
    "member": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "poolId": "1",
    "balance": "100000000000000000000",
    "points": "100000000000000000000",
    "era": "23"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x87eeadd1dcf643b898dd46a681704565741c428fffc0fbcd0f9f587d47b43c5d",
    "txIndex": 1,
    "blockHash": "0xb38d12ccdc7a5c8190b8948597d40008540574150ccd96426a611695f0969115",
    "blockNumber": 1032
  }
}
```

## Withdraw Unbonded

Origin Level: Signed

### Interface

```js
function withdrawUnbonded(memberAccount: string, numSlashingSpans: number, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<WithdrawUnbodedTx, TransactionFailed>>;
```

#### Parameters

| parameter        | type               | optional | description                                                   |
| ---------------- | ------------------ | -------- | ------------------------------------------------------------- |
| memberAccount    | string             | false    | member account                                                |
| numSlashingSpans | number             | false    | number of slashing spans                                      |
| waitFor          | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account          | KeyringPair        | false    | Account that will send and sign the transaction               |
| options          | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const memberAccount = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // Alice
  const numSlashingSpans = 0

  const result = await sdk.tx.nominationPools.withdrawUnbonded(
    memberAccount,
    numSlashingSpans,
    WaitFor.BlockInclusion,
    account,
  )
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `WithdrawUnbodedTx`.

```json
{
  "event": {
    "member": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "poolId": "1",
    "balance": "100000000000000000000",
    "points": "100000000000000000000"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0xfaad26fc9bc45d02303772cbdddaf4866430fc72f1b52129aebda5fcbb50a964",
    "txIndex": 1,
    "blockHash": "0x1062735f5e11d1ffd1724d9d2892609dbb7b7065fadc3d0e7aa77618179016b7",
    "blockNumber": 266
  }
}
```

# Staking

Runtime Component: Staking\
Runtime Index: 10\
Interface Module Name: staking

## Bond Extra

Origin Level: Signed

### Interface

```js
function bondExtra(maxAdditional: BN, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<BondExtraTx, TransactionFailed>>;
```

#### Parameters

| parameter     | type               | optional | description                                                   |
| ------------- | ------------------ | -------- | ------------------------------------------------------------- |
| maxAdditional | BN                 | false    | Additional amount that is bond. 10^18 is equal to 1 AVL       |
| waitFor       | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account       | KeyringPair        | false    | Account that will send and sign the transaction               |
| options       | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice//stash")
  const maxAdditional = SDK.oneAvail()

  const result = await sdk.tx.staking.bondExtra(maxAdditional, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `BondExtraTx`.

```json
{
  "event": {
    "stash": "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
    "amount": "1"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x940df5141925aeef2ab9aa767f6870689426de533f5f1d84b6d7be203e68ee77",
    "txIndex": 1,
    "blockHash": "0xc2a8375be07956586833f497a429ca2e29bafbb78ee5e051d5157df0ad5c8cb6",
    "blockNumber": 7
  }
}
```

## Bond

Origin Level: Signed

### Interface

```js
function bond(value: BN, payee: StakingRewardDestination, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<BondTx, TransactionFailed>>;
```

#### Parameters

| parameter | type                     | optional | description                                                   |
| --------- | ------------------------ | -------- | ------------------------------------------------------------- |
| value     | BN                       | false    | Amount that is bond. 10^18 is equal to 1 AVL                  |
| payee     | StakingRewardDestination | false    | Can be: "Staked", "Stash", "None" or an account address       |
| waitFor   | WaitFor                  | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair              | false    | ccount that will send and sign the transaction                |
| options   | TransactionOptions       | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const value = SDK.oneAvail().mul(new BN(100000)) // 100_000 Avail
  const payee = "Staked"

  const result = await sdk.tx.staking.bond(value, payee, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `BondTx`.

```json
{
  "event": {
    "stash": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "amount": "100000"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x3e1cc48207b02ca5d680cf1beeb270ce7cbf0d18a6191844bc963d4081a0ca90",
    "txIndex": 1,
    "blockHash": "0xf854e74cb428d0baf22454cb15007731a84263e57c64d019a304c0ca1bd30276",
    "blockNumber": 2
  }
}
```

## Chill Other

Origin Level: Signed

### Interface

```js
function chillOther(stash: string, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<ChillOtherTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| stash     | string             | false    | address of stash account to chill                             |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src/index"
const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice//stash")
  const stash = "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY" // Alice Stash

  const result = await sdk.tx.staking.chillOther(stash, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ChillOtherTx`.

```json

```

## Chill

Origin Level: Signed

### Interface

```js
function chill( waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<ChillTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src/index"
const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice//stash")

  const result = await sdk.tx.staking.chill(WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ChillTx`.

```json
{
  "event": {
    "stash": "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x4572681f19af32fdfb4759c914697697b0e82fde48a5dd7e28c2b3a263772b0d",
    "txIndex": 1,
    "blockHash": "0xad2e5376f53e6257e7bc0c842e5b6952f1d4af6f7499319b2d1ab59bdd742628",
    "blockNumber": 13
  }
}
```

## Nominate

Origin Level: Signed

### Interface

```js
function nominate(targets: string[], waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<NominateTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| targets   | string[]           | false    | list od addresses to nominate                                 |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const targets = [
    "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY", // Alice Stash
    "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", // Bob
  ]

  const result = await sdk.tx.staking.nominate(targets, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `NominateTxSuccess`.

```json
{
  "txData": {
      "targets": [
          "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
          "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
      ]
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x2f81a34f59d36eb7ada96ec1070358043026d7bd7cfb6fa5a532cc474190880b",
    "txIndex": 1,
    "blockHash": "0x49a57953aa2b2ba508f1c6991515309a0fe89723a79f3831f9a9263ba8c7baa4",
    "blockNumber": 4
  }
}
```

## Unbond

Origin Level: Signed

### Interface

```js
function unbond(value: BN, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<UnbondTx, TransactionFailed>>;
```

#### Parameters

| parameter | type               | optional | description                                                   |
| --------- | ------------------ | -------- | ------------------------------------------------------------- |
| value     | BN                 | false    | amount of tokens to unbond                                    |
| waitFor   | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account   | KeyringPair        | false    | Account that will send and sign the transaction               |
| options   | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const value = SDK.oneAvail()

  const result = await sdk.tx.staking.unbond(value, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `UnbondTx`.

```json
{
  "event": {
    "stash": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "amount": "1000000000000000000"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0xbf264e0e95885fd64a35d5c64bd4e1cc17056a1e6b05fa9207d7c777395dffdf",
    "txIndex": 1,
    "blockHash": "0x9ef43aaca71ba7b91a53976de5170f80d8a1ed4fe3e95fae237f7ed91f953963",
    "blockNumber": 9
  }
}
```

## Validate

Origin Level: Signed

### Interface

```js
function validate(commission: number, blocked: boolean, waitFor: WaitFor, account: KeyringPair, options?: TransactionOptions): Promise<Result<ValidateTx, TransactionFailed>>;
```

#### Parameters

| parameter  | type               | optional | description                                                   |
| ---------- | ------------------ | -------- | ------------------------------------------------------------- |
| commission | number             | false    | how much validator charge nominators in 0 - 100 range         |
| blocked    | boolean            | false    | whether or not this validator accepts nominations             |
| waitFor    | WaitFor            | false    | Wait for block inclusion or finalization                      |
| account    | KeyringPair        | false    | Account that will send and sign the transaction               |
| options    | TransactionOptions | true     | Can be used to set nonce, app id or other transaction options |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const commission = 5 // 5%
  const blocked = false

  const result = await sdk.tx.staking.validate(commission, blocked, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ValidateTx`.

```json
{
  "event": {
    "stash": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "commission": "50000000",
    "blocked": "false"
  },
  "details": {
    "txResult": {...},
    "events": [...],
    "txHash": "0x31f047da16a350e32b832cc73d3351c8d5e5991625fde6e8c36fc45ebb9d2735",
    "txIndex": 1,
    "blockHash": "0xa7735804f52602d4b73e1dd7f718cf0ab5cc00d111c927a9f8a2b3d02b66e09a",
    "blockNumber": 14
  }
}
```
