# Data Availability

Runtime Component: DataAvailability\
Runtime Index: 29\
Interface Module Name: dataAvailability

## Create Application Key

Origin Level: Signed

### Interface

```js
function createApplicationKey(key: string, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<CreateApplicationKeyTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| key       | string        | false    | name of the application key                     |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const key = "MyAwesomeKey"

  const result = await sdk.tx.dataAvailability.createApplicationKey(key, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))

  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `CreateApplicationKeyTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "key": "0x4d79417765736f6d654b6579",
    "owner": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "id": "10"
  },
  "events": [...],
  "txHash": "0x5ae9edbd2a2da96eeffc14cf9050d711082890fa6bfb8749ad2c4947565f3bd2",
  "txIndex": 1,
  "blockHash": "0x152338c1b0696d12664cf3d4c159af3d54beca151ba1ea8b00989a66dc8050b0",
  "blockNumber": 1
}
```

## Submit Data

Origin Level: Signed

### Interface

```js
function submitData(data: string, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<SubmitDataTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| data      | string        | false    | data to be submitted                            |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const data = "My Awesome Data"

  const result = await sdk.tx.dataAvailability.submitData(data, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `SubmitDataTxSuccess`.

```js
{
  "isErr": false,
  "txData": {
    "data": "4d7920417765736f6d652044617461"
  },
  "event": {
    "who": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "dataHash": "0x8846d900ea89aab9bce96402846c0ac74a853acc00cb99ff5ddb1a0f052594bd"
  },
  "events": [...],
  "txHash": "0xec6f9fd5e002c9ddbcd24764380f57a014de7f2007cc0e2ae11a4dda17ab8062",
  "txIndex": 1,
  "blockHash": "0x043c2b88ff960f2f7042521b55a943676938948febefe8684022b524795340d9",
  "blockNumber": 9
}
```

## Submit Block Length Proposal

Origin Level: Root

### Interface

```js
function submitBlockLengthProposal(rows: number, cols: number, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<SubmitBlockLengthProposalTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| rows      | number        | false    | number of rows in block                         |
| cols      | number        | false    | number of cols in block                         |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const rows = 128
  const cols = 128

  const result = await sdk.tx.dataAvailability.submitBlockLengthProposal(rows, cols, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))

  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `SubmitBlockLengthProposalTxSuccess`.

```js

```

## Set Application Key

Origin Level: Root

### Interface

```js
function setApplicationKey(oldKey: string, newKey: string, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<SetApplicationKeyTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| oldKey    | string        | false    | application key to be replaced                  |
| newKey    | string        | false    | application key that will replace the old one   |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const oldKey = "MyNewKeyAwesome1"
  const newKey = "MyNewKeyAwesome2"

  const result = await sdk.tx.dataAvailability.setApplicationKey(oldKey, newKey, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))

  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `SetApplicationKeyTxSuccess`.

```js

```

## Set Submit Data Fee Modifier

Origin Level: Root

### Interface

```js
function setSubmitDataFeeModifier(modifier: DispatchFeeModifier, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<SetSubmitDataFeeModifierTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type                | optional | description                                     |
| --------- | ------------------- | -------- | ----------------------------------------------- |
| modifier  | DispatchFeeModifier | false    | new fee modifier values                         |
| waitFor   | WaitFor             | false    | wait for block inclusion or finalization        |
| account   | KeyringPair         | false    | account that will send and sign the transaction |
| options   | SignerOptions       | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN, sdkTransactions } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const modifier = {
    weightMaximumFee: new BN("10").pow(new BN("18")),
    weightFeeDivider: 20,
  } as sdkTransactions.DispatchFeeModifier

  const result = await sdk.tx.dataAvailability.setSubmitDataFeeModifier(modifier, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))

  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `SetSubmitDataFeeModifierTxSuccess`.

```js

```

## Type Definitions

```js
enum WaitFor {
	BlockInclusion,
	BlockFinalization,
}
```

```js
type DispatchFeeModifier = { weightMaximumFee: BN | null,  weightFeeDivider: number | null, weightFeeMultiplier: number | null };
```

# Balances

Runtime Component: Balances\
Runtime Index: 6\
Interface Module Name: balances

## Transfer Keep Alive

Origin Level: Signed

### Interface

```js
function transferKeepAlive(dest: string, value: BN,  waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<TransferKeepAliveTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| dest      | string        | false    | account that will receive funds                 |
| value     | BN            | false    | amount that is send. 10^18 is equal to 1 AVL    |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // Eve
  const amount = new BN(10).pow(new BN(18)) // one Avail

  const result = await sdk.tx.balances.transferKeepAlive(dest, amount, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))

  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `TransferKeepAliveTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "from": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "to": "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
    "amount": "1000000000000000000"
  },
  "events": [...],
  "txHash": "0x812a3f3960afb8df72de0e5b86ff564c8ce7d93c837182c24d1796fb68a7f5f4",
  "txIndex": 1,
  "blockHash": "0xfdee1faced02696d692df1d896fa2822f4eb02f260c95e11041df86b2c229dfb",
  "blockNumber": 1
}
```

## Transfer Allow Death

Origin Level: Signed

### Interface

```js
function transferAllowDeath(dest: string, value: BN, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<TransferAllowDeathTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| dest      | string        | false    | account that will receive funds                 |
| value     | BN            | false    | amount that is send. 10^18 is equal to 1 AVL    |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // Eve
  const amount = new BN(10).pow(new BN(18)) // one Avail

  const result = await sdk.tx.balances.transferAllowDeath(dest, amount, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))

  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `TransferAllowDeathTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "from": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "to": "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
    "amount": "1000000000000000000"
  },
  "events": [...],
  "txHash": "0x63a73d2d1210ab9840341506788cca9592fd968609fecb5106cf0370c611061c",
  "txIndex": 1,
  "blockHash": "0xde2e95b63a4ca5927f9105931e4676b0634d12f524d4fff1048b403393419489",
  "blockNumber": 2
}
```

## Transfer All

Origin Level: Signed

### Interface

```js
function transferAll(dest: string, keepAlive: boolean, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<TransferAllTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                      |
| --------- | ------------- | -------- | ------------------------------------------------ |
| dest      | string        | false    | account that will receive funds                  |
| keepAlive | boolean       | false    | if set to false it will reap the account as well |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization         |
| account   | KeyringPair   | false    | account that will send and sign the transaction  |
| options   | SignerOptions | true     | used to overwrite existing signer options        |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // Eve
  const keepAlive = true

  const result = await sdk.tx.balances.transferAll(dest, keepAlive, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))

  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `TransferAllTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "from": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "to": "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
    "amount": "9999999873433871068464733"
  },
  "events": [...],
  "txHash": "0x343d3e8890bd479b4619cb7b0f2dfa91b7b91c0cedc0646247215f85baf1f63e",
  "txIndex": 1,
  "blockHash": "0xaec4adfad11f8aa902e1a985abb62737fc02445072b168238a956c3a0d8820f2",
  "blockNumber": 2
}
```

# Staking

Runtime Component: Staking\
Runtime Index: 10\
Interface Module Name: staking

## Bond

Origin Level: Signed

### Interface

```js
function bond(value: BN, payee: StakingRewardDestination, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<BondTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type                     | optional | description                                             |
| --------- | ------------------------ | -------- | ------------------------------------------------------- |
| value     | BN                       | false    | amount that is bond. 10^18 is equal to 1 AVL            |
| payee     | StakingRewardDestination | false    | Can be: "Staked", "Stash", "None" or an account address |
| waitFor   | WaitFor                  | false    | wait for block inclusion or finalization                |
| account   | KeyringPair              | false    | account that will send and sign the transaction         |
| options   | SignerOptions            | true     | used to overwrite existing signer options               |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const value = new BN(100_000).mul(new BN(10).pow(new BN("18"))) // 100 000 Avail
  const payee = "Staked"

  const result = await sdk.tx.staking.bond(value, payee, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))

  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `BondTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "stash": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "amount": "100000"
  },
  "events": [...],
  "txHash": "0x3e1cc48207b02ca5d680cf1beeb270ce7cbf0d18a6191844bc963d4081a0ca90",
  "txIndex": 1,
  "blockHash": "0xf854e74cb428d0baf22454cb15007731a84263e57c64d019a304c0ca1bd30276",
  "blockNumber": 2
}
```

## Bond Extra

Origin Level: Signed

### Interface

```js
function bondExtra(maxAdditional: BN, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<BondExtraTxSuccess | GenericFailure>;
```

#### Parameters

| parameter     | type          | optional | description                                             |
| ------------- | ------------- | -------- | ------------------------------------------------------- |
| maxAdditional | BN            | false    | additional amount that is bond. 10^18 is equal to 1 AVL |
| waitFor       | WaitFor       | false    | wait for block inclusion or finalization                |
| account       | KeyringPair   | false    | account that will send and sign the transaction         |
| options       | SignerOptions | true     | used to overwrite existing signer options               |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice//stash")
  const maxAdditional = new BN(10).pow(new BN(18)) // one Avail

  const result = await sdk.tx.staking.bondExtra(maxAdditional, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))

  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `BondExtraTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "stash": "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
    "amount": "1"
  },
  "events": [...],
  "txHash": "0x940df5141925aeef2ab9aa767f6870689426de533f5f1d84b6d7be203e68ee77",
  "txIndex": 1,
  "blockHash": "0xc2a8375be07956586833f497a429ca2e29bafbb78ee5e051d5157df0ad5c8cb6",
  "blockNumber": 7
}
```

## Chill

Origin Level: Signed

### Interface

```js
function chill(waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<ChillTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "avail-js-sdk"
const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice//stash")

  const result = await sdk.tx.staking.chill(WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))

  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ChillTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "stash": "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"
  },
  "events": [...],
  "txHash": "0x4572681f19af32fdfb4759c914697697b0e82fde48a5dd7e28c2b3a263772b0d",
  "txIndex": 1,
  "blockHash": "0xad2e5376f53e6257e7bc0c842e5b6952f1d4af6f7499319b2d1ab59bdd742628",
  "blockNumber": 13
}
```

## Chill Other

Origin Level: Signed

### Interface

```js
function chillOther(stash: string, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<ChillOtherTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| stash     | string        | false    | address of stash account to chill               |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "avail-js-sdk"
const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice//stash")
  const stash = "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY" // Alice Stash

  const result = await sdk.tx.staking.chillOther(stash, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))

  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ChillOtherTxSuccess`.

```js

```

## Nominate

Origin Level: Signed

### Interface

```js
function nominate(targets: string[], waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<NominateTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| targets   | string[]      | false    | list od addresses to nominate                   |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "avail-js-sdk"

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
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))

  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `NominateTxSuccess`.

```js
{
  "isErr": false,
  "txData": {
      "targets": [
          "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
          "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
      ]
  },
  "events": [...],
  "txHash": "0x2f81a34f59d36eb7ada96ec1070358043026d7bd7cfb6fa5a532cc474190880b",
  "txIndex": 1,
  "blockHash": "0x49a57953aa2b2ba508f1c6991515309a0fe89723a79f3831f9a9263ba8c7baa4",
  "blockNumber": 4
}
```

## Unbond

Origin Level: Signed

### Interface

```js
function unbond(value: BN, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<UnbondTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| value     | BN            | false    | amount of tokens to unbond                      |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const value = new BN(10).pow(new BN(18)) // one Avail

  const result = await sdk.tx.staking.unbond(value, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `UnbondTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "stash": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "amount": "1000000000000000000"
  },
  "events": [...],
  "txHash": "0xbf264e0e95885fd64a35d5c64bd4e1cc17056a1e6b05fa9207d7c777395dffdf",
  "txIndex": 1,
  "blockHash": "0x9ef43aaca71ba7b91a53976de5170f80d8a1ed4fe3e95fae237f7ed91f953963",
  "blockNumber": 9
}
```

## Validate

Origin Level: Signed

### Interface

```js
function validate(commission: number, blocked: boolean, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<ValidateTxSuccess | GenericFailure>;
```

#### Parameters

| parameter  | type          | optional | description                                           |
| ---------- | ------------- | -------- | ----------------------------------------------------- |
| commission | number        | false    | how much validator charge nominators in 0 - 100 range |
| blocked    | boolean       | false    | whether or not this validator accepts nominations     |
| waitFor    | WaitFor       | false    | wait for block inclusion or finalization              |
| account    | KeyringPair   | false    | account that will send and sign the transaction       |
| options    | SignerOptions | true     | used to overwrite existing signer options             |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const commission = 5 // 5%
  const blocked = false

  const result = await sdk.tx.staking.validate(commission, blocked, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))

  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `ValidateTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "stash": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "commission": "50000000",
    "blocked": "false"
  },
  "events": [...],
  "txHash": "0x31f047da16a350e32b832cc73d3351c8d5e5991625fde6e8c36fc45ebb9d2735",
  "txIndex": 1,
  "blockHash": "0xa7735804f52602d4b73e1dd7f718cf0ab5cc00d111c927a9f8a2b3d02b66e09a",
  "blockNumber": 14
}
```

# Nomination Pools

Runtime Component: Nomination Pools\
Runtime Index: 36\
Interface Module Name: nominationPools

## Create

Origin Level: Signed

### Interface

```js
function create(amount: BN, root: string, nominator: string, bouncer: string, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<PoolCreateTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                        |
| --------- | ------------- | -------- | -------------------------------------------------- |
| amount    | BN            | false    | The amount of funds to delegate to the pool        |
| root      | string        | false    | The account to set as [`PoolRoles::root`]          |
| nominator | string        | false    | The account to set as the [`PoolRoles::nominator`] |
| bouncer   | string        | false    | The account to set as the [`PoolRoles::bouncer`]   |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization           |
| account   | KeyringPair   | false    | account that will send and sign the transaction    |
| options   | SignerOptions | true     | used to overwrite existing signer options          |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const amount = new BN(10).pow(new BN(18)).mul(new BN(10000)) // 10_000 Avail

  const root: string = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // Alice
  const nominator: string = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // Alice
  const bouncer: string = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // Alice

  const result = await sdk.tx.nominationPools.create(amount, root, nominator, bouncer, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `PoolCreateTxSuccess`.

```js
{
  "isErr": false,
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
  "events": [...],
  "txHash": "0x9f019464c676682d9cbfb62814d019a85738e01b0ce92b18fa77878f80e16f66",
  "txIndex": 1,
  "blockHash": "0xdaa49f09b8c519bbd4c52ed05652d3de3f777afa3ac85db5c697cf705907e2d2",
  "blockNumber": 5
}
```

## Create with Pool Id

Origin Level: Signed

### Interface

```js
function createWithPoolId(amount: BN, root: string, nominator: string, bouncer: string, poolId: number, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<PoolCreateWithPoolIdTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                        |
| --------- | ------------- | -------- | -------------------------------------------------- |
| amount    | BN            | false    | The amount of funds to delegate to the pool        |
| root      | string        | false    | The account to set as [`PoolRoles::root`]          |
| nominator | string        | false    | The account to set as the [`PoolRoles::nominator`] |
| bouncer   | string        | false    | The account to set as the [`PoolRoles::bouncer`]   |
| poolId    | number        | false    | pool id                                            |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization           |
| account   | KeyringPair   | false    | account that will send and sign the transaction    |
| options   | SignerOptions | true     | used to overwrite existing signer options          |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob")
  const amount = new BN(10).pow(new BN(18)).mul(new BN(10000)) // 10_000 Avail

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
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `PoolCreateWithPoolIdTxSuccess`.

```js
{
  "isErr": false,
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
  "events": [...],
  "txHash": "0x6b50caed7950e67934cabbf88a1f7dc2e7e995ac608402f91a4db19be0da5c41",
  "txIndex": 1,
  "blockHash": "0xc06df7dbb1e404f54499f942479ddcffc92665c021ea07c2798fc2f354f403d3",
  "blockNumber": 6
}
```

## Join

Origin Level: Signed

### Interface

```js
function join(amount: BN, poolId: number, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<PoolJoinTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| amount    | BN            | false    | The amount of funds to delegate to the pool     |
| poolId    | number        | false    | pool id                                         |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob")
  const amount = new BN(10).pow(new BN(18)).mul(new BN(10000)) // 10_000 Avail
  const poolId = 1

  const result = await sdk.tx.nominationPools.join(amount, poolId, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `PoolJoinTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "member": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    "poolId": "1",
    "bonded": "10000",
    "joined": "true"
  },
  "events": [...],
  "txHash": "0x06baecbb8680e90d025d1fd08044d0d251054a89e82dd460022bdf3796020050",
  "txIndex": 1,
  "blockHash": "0x82078130da46adacf5bdff86618ab6e1c443fda6d883d9fcf967a41a2e29d612",
  "blockNumber": 19
}
```

## Nominate

Origin Level: Signed

### Interface

```js
function nominate(poolId: number, validators: string[], waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<PoolNominateTxSuccess | GenericFailure>;
```

#### Parameters

| parameter  | type          | optional | description                                     |
| ---------- | ------------- | -------- | ----------------------------------------------- |
| poolId     | number        | false    | pool id                                         |
| validators | string[]      | false    | list of validators to nominate                  |
| waitFor    | WaitFor       | false    | wait for block inclusion or finalization        |
| account    | KeyringPair   | false    | account that will send and sign the transaction |
| options    | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "avail-js-sdk"

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
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `PoolNominateTxSuccess`.

```js
{
  "isErr": false,
  "events": [...],
  "txHash": "0x98b993baf90183d85dece9357d3bc32311f4201b015b63845a13dbc22bf22370",
  "txIndex": 1,
  "blockHash": "0x84ef5a0ada4af71358ee701a2500bce7f6688efb554c32ba1a30c459f64d5370",
  "blockNumber": 48
}
```

## Bond Extra

Origin Level: Signed

### Interface

```js
function bondExtra(extra: BondExtra, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<PoolBondExtraTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                                                                            |
| --------- | ------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| extra     | BondExtra     | false    | Additional funds can come from either the free balance of the account, of from the accumulated rewards |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization                                                               |
| account   | KeyringPair   | false    | account that will send and sign the transaction                                                        |
| options   | SignerOptions | true     | used to overwrite existing signer options                                                              |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BondExtra, BN } from "../../src"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const amount = new BN(10).pow(new BN(18)).mul(new BN(10000)) // 10_000 Avail
  const bondExtra = { FreeBalance: amount } as BondExtra

  const result = await sdk.tx.nominationPools.bondExtra(bondExtra, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `PoolBondExtraTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "member": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "poolId": "1",
    "bonded": "10000000000000000000000",
    "joined": "false"
  },
  "events": [...],
  "txHash": "0xc09bdfbc1703763850a2f4255ccb0dd7d7e9f4b793d2f9f05782b9e3c63682d9",
  "txIndex": 1,
  "blockHash": "0xe06e35e61e2b59900e22d627dd8cd9cab39c87a304a40aba0fce87936e921843",
  "blockNumber": 112
}
```

## Set Metadata

Origin Level: Signed

### Interface

```js
function setMetadata(poolId: number, metadata: string, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<PoolSetMetadataTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| poolId    | number        | false    | pool id                                         |
| metadata  | string        | false    | metadata                                        |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const poolId = 1
  const metadata = "My Metadata"

  const result = await sdk.tx.nominationPools.setMetadata(poolId, metadata, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `PoolSetMetadataTxSuccess`.

```js
{
  "isErr": false,
  "events": [...],
  "txHash": "0x59121cd7e320acab280a1ad0d8b8385581ca7e03d973477ae812b1d967e2cb1d",
  "txIndex": 1,
  "blockHash": "0x6f19a8c993dc33676605a1dfabbb5a1008929ee965772a3b3edd12f9fe1eb296",
  "blockNumber": 369
}
```

## Unbond

Origin Level: Signed

### Interface

```js
function unbond(memberAccount: string, unbondingPoints: BN, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<PoolUnbondTxSuccess | GenericFailure>;
```

#### Parameters

| parameter       | type          | optional | description                                     |
| --------------- | ------------- | -------- | ----------------------------------------------- |
| memberAccount   | string        | false    | member account                                  |
| unbondingPoints | BN            | false    | defines how many tokens will be unbond          |
| waitFor         | WaitFor       | false    | wait for block inclusion or finalization        |
| account         | KeyringPair   | false    | account that will send and sign the transaction |
| options         | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "../../src"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const memberAccount = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // Alice
  const unbondingPoints = new BN(10).pow(new BN(18)).mul(new BN(100)) // 100 Avail

  const result = await sdk.tx.nominationPools.unbond(memberAccount, unbondingPoints, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `PoolUnbondTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "member": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "poolId": "1",
    "balance": "100000000000000000000",
    "points": "100000000000000000000",
    "era": "23"
  },
  "events": [],
  "txHash": "0x87eeadd1dcf643b898dd46a681704565741c428fffc0fbcd0f9f587d47b43c5d",
  "txIndex": 1,
  "blockHash": "0xb38d12ccdc7a5c8190b8948597d40008540574150ccd96426a611695f0969115",
  "blockNumber": 1032
}
```

## Chill

Origin Level: Signed

### Interface

```js
function chill(poolId: number, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<PoolChillTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| poolId    | number        | false    | pool id                                         |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const poolId = 1

  const result = await sdk.tx.nominationPools.chill(poolId, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `PoolChillTxSuccess`.

```js
{
  "isErr": false,
  "events": [...],
  "txHash": "0x678fd95154114bb899598f8ef4413e3fdb5a90317a132d5a49f868ffb029cecf",
  "txIndex": 1,
  "blockHash": "0xa66f5eacd4c4294ba6679c868d90df11fccf5c85c773e8554856882ac64b4be7",
  "blockNumber": 1106
}
```

## Set Claim Permission

Origin Level: Signed

### Interface

```js
function setClaimPermission(permission: ClaimPermission, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<PoolSetClaimPermissionOtherTxSuccess | GenericFailure>;
```

#### Parameters

| parameter  | type            | optional | description                                     |
| ---------- | --------------- | -------- | ----------------------------------------------- |
| permission | ClaimPermission | false    | permission type                                 |
| waitFor    | WaitFor         | false    | wait for block inclusion or finalization        |
| account    | KeyringPair     | false    | account that will send and sign the transaction |
| options    | SignerOptions   | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, ClaimPermission } from "../../src"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob")
  const permission: ClaimPermission = "PermissionlessAll"

  const result = await sdk.tx.nominationPools.setClaimPermission(permission, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `PoolSetClaimPermissionOtherTxSuccess`.

```js
{
  "isErr": false,
  "events": [...],
  "txHash": "0x3096fb312d22f2aa4fa51532fbae4b812a99d60c85a0750662816146bce997bc",
  "txIndex": 1,
  "blockHash": "0xfb41995f2e5aaa1b92126792dfc1e7b2863773fa4865eca2d96f458da17d6f82",
  "blockNumber": 249
}
```

## Claim Commission

Origin Level: Signed

### Interface

```js
function claimCommission(poolId: number, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<PoolClaimCommissionTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| poolId    | number        | false    | pool id                                         |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "../../src"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const poolId = 1

  const result = await sdk.tx.nominationPools.claimCommission(poolId, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `PoolClaimCommissionTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "poolId": "1",
    "commission": "7652149502759813012"
  },
  "events": [...],
  "txHash": "0x52292c2af7c2505c0bd39f5f1308bc10d92db7117f1b9fec208dc778e0c49654",
  "txIndex": 1,
  "blockHash": "0xdceaa5b12517f9634ad568e355061e8e13a8af25074e8966381c92fcea214285",
  "blockNumber": 418
}
```

## Claim Payout

Origin Level: Signed

### Interface

```js
function claimPayout(waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<PoolClaimPayoutTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "../../src"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")

  const result = await sdk.tx.nominationPools.claimPayout(WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `PoolClaimPayoutTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "member": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "poolId": "1",
    "payout": "4008268787159890000"
  },
  "events": [...],
  "txHash": "0x15b50cb2a3885432811ba1417b7715e69e05b8d4dd8f7c9951f0b4f7a8ba5a61",
  "txIndex": 1,
  "blockHash": "0xdd24eca2111c6b7eea4a34c9bed02cfa6effde65c65c3dc66fb13c88e2fe6985",
  "blockNumber": 426
}
```

## Claim Payout Other

Origin Level: Signed

### Interface

```js
function claimPayoutOther(other: string, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<PoolClaimPayoutOtherTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| other     | &str          | false    | other account to claim payout                   |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, BN } from "../../src"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const other = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" // Bob

  const result = await sdk.tx.nominationPools.claimPayoutOther(other, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `PoolClaimPayoutOtherTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "member": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    "poolId": "1",
    "payout": "8198513381719500000"
  },
  "events": [...],
  "txHash": "0xe14015b25afc66c4954dda18631b3fd5efaeba80989070ed12712ca312dbec40",
  "txIndex": 1,
  "blockHash": "0x3abde836ab66ac38bc3ca260f081d370f1c15094f43048423e0995170569a51a",
  "blockNumber": 594
}
```

## Set Commission

Origin Level: Signed

### Interface

```js
function setCommission(poolId: number, newCommission: NewCommission | null, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<PoolSetCommissionTxSuccess | GenericFailure>;
```

#### Parameters

| parameter     | type                  | optional | description                                                      |
| ------------- | --------------------- | -------- | ---------------------------------------------------------------- |
| poolId        | number                | false    | pool id                                                          |
| newCommission | NewCommission or Null | false    | if empty it removes the existing commission otherwise it sets it |
| waitFor       | WaitFor               | false    | wait for block inclusion or finalization                         |
| account       | KeyringPair           | false    | account that will send and sign the transaction                  |
| options       | SignerOptions         | true     | used to overwrite existing signer options                        |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, NewCommission } from "../../src"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const poolId = 1
  const newCommission: NewCommission = { amount: 25, payee: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" }

  const result = await sdk.tx.nominationPools.setCommission(poolId, newCommission, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `PoolSetCommissionTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "poolId": "1",
    "current": "[250000000,\"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY\"]"
  },
  "events": [...],
  "txHash": "0x8256693f9648abe0d47b12380cdcfcb56c3078080216a7487f0b34d7ff80e047",
  "txIndex": 1,
  "blockHash": "0xedf037ef9de56cba1a81a45bb5ed5b0ec3424725a521c6255976a68d5638015e",
  "blockNumber": 552
}
```

## Withdraw Unbonded

Origin Level: Signed

### Interface

```js
function withdrawUnbonded(memberAccount: string, numSlashingSpans: number, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<PoolWithdrawUnbodedTxSuccess | GenericFailure>;
```

#### Parameters

| parameter        | type          | optional | description                                     |
| ---------------- | ------------- | -------- | ----------------------------------------------- |
| memberAccount    | string        | false    | member account                                  |
| numSlashingSpans | number        | false    | number of slashing spans                        |
| waitFor          | WaitFor       | false    | wait for block inclusion or finalization        |
| account          | KeyringPair   | false    | account that will send and sign the transaction |
| options          | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring } from "../../src"

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
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `PoolWithdrawUnbodedTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "member": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "poolId": "1",
    "balance": "100000000000000000000",
    "points": "100000000000000000000"
  },
  "events": [...],
  "txHash": "0xfaad26fc9bc45d02303772cbdddaf4866430fc72f1b52129aebda5fcbb50a964",
  "txIndex": 1,
  "blockHash": "0x1062735f5e11d1ffd1724d9d2892609dbb7b7065fadc3d0e7aa77618179016b7",
  "blockNumber": 266
}
```

## Set State

Origin Level: Signed

### Interface

```js
function setState(poolId: number, state: PoolState, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<PoolSetStateTxSuccess | GenericFailure>;
```

#### Parameters

| parameter | type          | optional | description                                     |
| --------- | ------------- | -------- | ----------------------------------------------- |
| poolId    | number        | false    | pool id                                         |
| state     | PoolState     | false    | "Open" or "Blocked" or "Destroying"             |
| waitFor   | WaitFor       | false    | wait for block inclusion or finalization        |
| account   | KeyringPair   | false    | account that will send and sign the transaction |
| options   | SignerOptions | true     | used to overwrite existing signer options       |

### Minimal Example

```js
import { SDK, WaitFor, Keyring, PoolState } from "../../src"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const poolId = 2
  const state: PoolState = "Blocked"

  const result = await sdk.tx.nominationPools.setState(poolId, state, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
```

### Example Output

#### On Failure

If the operation fails, the function will return an error message indicating the nature of the issue.

#### On Success

If the operation is successful, the function will return a object of type `PoolSetStateTxSuccess`.

```js
{
  "isErr": false,
  "event": {
    "poolId": "1",
    "newState": "Blocked"
  },
  "events": [...],
  "txHash": "0x921a187171a1d15a0779be75b9ecadc62eb1446f798bd0f4f1542700f32c724c",
  "txIndex": 1,
  "blockHash": "0xf2b85aae8a41eb4a27c72dfb2c4bce258213b829ef5f97f416a24d3989e7b3da",
  "blockNumber": 184
}
```
