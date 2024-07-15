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

#### Return value

On failure, a reason of failure is returned. On Success, ApplicationKeyCreated event, transaction hash and block hash is returned.

### Minimal Example

```js
import { Keyring, SDK, WaitFor } from "../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const key = "MyAwesomeKey";

const result = await sdk.tx.dataAvailability.createApplicationKey(key, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("Key=" + result.event.key + ", Owner=" + result.event.owner + ", Id=" + result.event.id);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
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

#### Return value

On failure, a reason of failure is returned. On Success, DataSubmitted event, transaction data, transaction hash and block hash is returned.

### Minimal Example

```js
import { Keyring, SDK, WaitFor } from "../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const data = "My Awesome Data";

const result = await sdk.tx.dataAvailability.submitData(data, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("Data=" + result.txData.data);
console.log("Who=" + result.event.who + ", DataHash=" + result.event.dataHash);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
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

#### Return value

On failure, a reason of failure is returned. On Success, BlockLengthProposalSubmitted event, transaction hash and block hash is returned.

### Minimal Example

```js
import { Keyring, SDK, WaitFor } from "../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const rows = 128;
const cols = 128;

const result = await sdk.tx.dataAvailability.submitBlockLengthProposal(rows, cols, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("Rows=" + result.event.rows + ", Cols=" + result.event.cols);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
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

#### Return value

On failure, a reason of failure is returned. On Success, ApplicationKeySet event, transaction hash and block hash is returned.

### Minimal Example

```js
import { Keyring, SDK, WaitFor } from "../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const oldKey = "MyNewKeyAwesome1";
const newKey = "MyNewKeyAwesome2";

const result = await sdk.tx.dataAvailability.setApplicationKey(oldKey, newKey, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("OldKey=" + result.event.oldKey + ", NewKey=" + result.event.newKey);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
```

## Set Submit Data Fee Modifer

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

#### Return value

On failure, a reason of failure is returned. On Success, SubmitDataFeeModifierSet event, transaction hash and block hash is returned.

### Minimal Example

```js
import { BN, DispatchFeeModifier, Keyring, SDK, WaitFor } from "../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const modifier = { weightMaximumFee: new BN("10").pow(new BN("18")), weightFeeDivider: 20 } as DispatchFeeModifier;

const result = await sdk.tx.dataAvailability.setSubmitDataFeeModifier(modifier, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log(
	"WeightMaximumFee=" + result.event.weightMaximumFee + ", WeightFeeMultiplier=" + result.event.weightFeeMultiplier +
		", WeightFeeDivider=" + result.event.weightFeeDivider,
);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
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

#### Return value

On failure, a reason of failure is returned. On Success, TransferEvent event, transaction hash and block hash is returned.

### Minimal Example

```js
import { BN, Keyring, SDK, WaitFor } from "../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"; // Eve
const amount = new BN(10).pow(new BN(18)); // one Avail

const result = await sdk.tx.balances.transferKeepAlive(dest, amount, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("From=" + result.event.from + ", To=" + result.event.to + ", Amount=" + result.event.amount);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
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

#### Return value

On failure, a reason of failure is returned. On Success, TransferEvent event, KilledAccount (optionally) event, transaction hash and block
hash is returned.

### Minimal Example

```js
import { BN, Keyring, SDK, WaitFor } from "../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"; // Eve
const amount = new BN(10).pow(new BN(18)); // one Avail

const result = await sdk.tx.balances.transferAllowDeath(dest, amount, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("From=" + result.event.from + ", To=" + result.event.to + ", Amount=" + result.event.amount);
console.log("MaybeKilled=" + result.event2?.account);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
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

#### Return value

On failure, a reason of failure is returned. On Success, TransferEvent event, KilledAccount (optionally) event, transaction hash and block
hash is returned.

### Minimal Example

```js
import { Keyring, SDK, WaitFor } from "../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"; // Eve
const keepAlive = true;

const result = await sdk.tx.balances.transferAll(dest, keepAlive, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("From=" + result.event.from + ", To=" + result.event.to + ", Amount=" + result.event.amount);
console.log("MaybeKilled=" + result.event2?.account);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
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

#### Return value

On failure, a reason of failure is returned. On Success, Bonded event, transaction hash and block hash is returned.

### Minimal Example

```js
import { BN, Keyring, SDK, WaitFor } from "../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const value = new BN(100_000).mul(new BN(10).pow(new BN("18"))); // 100 000 Avail
const payee = "Staked";

const result = await sdk.tx.staking.bond(value, payee, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("Stash=" + result.event.stash + ", Amount=" + result.event.amount);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
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

#### Return value

On failure, a reason of failure is returned. On Success, Bonded event, transaction hash and block hash is returned.

### Minimal Example

```js
import { BN, Keyring, SDK, WaitFor } from "../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice//stash");
const maxAdditional = new BN(10).pow(new BN(18)); // one Avail

const result = await sdk.tx.staking.bondExtra(maxAdditional, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("Stash=" + result.event.stash + ", Amount=" + result.event.amount);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
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

#### Return value

On failure, a reason of failure is returned. On Success, Chilled event, transaction hash and block hash is returned.

### Minimal Example

```js
import { Keyring, SDK, WaitFor } from "../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice//stash");

const result = await sdk.tx.staking.chill(WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("Stash=" + result.event.stash);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
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

#### Return value

On failure, a reason of failure is returned. On Success, Chilled event, transaction hash and block hash is returned.

### Minimal Example

```js
import { Keyring, SDK, WaitFor } from "../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice//stash");
const stash = "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"; // Alice Stash

const result = await sdk.tx.staking.chillOther(stash, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("Stash=" + result.event.stash);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
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

#### Return value

On failure, a reason of failure is returned. On Success, Nominate transaction data, transaction hash and block hash is returned.

### Minimal Example

```js
import { Keyring, SDK, WaitFor } from "../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const targets = [
	"5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY", // Alice Stash
	"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", // Bob
];

const result = await sdk.tx.staking.nominate(targets, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("TxDataTargets=" + result.txData.targets);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
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

#### Return value

On failure, a reason of failure is returned. On Success, Unbonded event, transaction hash and block hash is returned.

### Minimal Example

```js
import { BN, Keyring, SDK, WaitFor } from "../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const value = new BN(10).pow(new BN(18)); // one Avail

const result = await sdk.tx.staking.unbond(value, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("Stash=" + result.event.stash + ", Amount=" + result.event.amount);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
```

## Validate

Origin Level: Signed

### Interface

```js
function validate(commission: number, blocked: boolean, waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<ValidatexSuccess | GenericFailure>;
```

#### Parameters

| parameter  | type          | optional | description                                           |
| ---------- | ------------- | -------- | ----------------------------------------------------- |
| commission | number        | false    | how much validator charge nominators in 0 - 100 range |
| blocked    | boolean       | false    | whether or not this validator accepts nominations     |
| waitFor    | WaitFor       | false    | wait for block inclusion or finalization              |
| account    | KeyringPair   | false    | account that will send and sign the transaction       |
| options    | SignerOptions | true     | used to overwrite existing signer options             |

#### Return value

On failure, a reason of failure is returned. On Success, ValidatorPrefsSet event, transaction hash and block hash is returned.

### Minimal Example

```js
import { Keyring, SDK, WaitFor } from "../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const commission = 5; // 5%
const blocked = false;

const result = await sdk.tx.staking.validate(commission, blocked, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("Stash=" + result.event.stash + ", Commission=" + result.event.commission + ", Blocked=" + result.event.blocked);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
```
