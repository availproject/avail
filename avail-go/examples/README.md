# Avail Go SDK documentation / examples

## Running Examples in the Docs Directory

To run the examples provided in the `/examples` directory, follow these steps:

1. [Install](https://go.dev/doc/install) go globally if you haven't already:

2. From the avail-go/examples, install all necessary dependencies:

```bash
go mod tidy
```

3. Ensure that a config file is generated in the root of go-examples folder

for example

```json
{
  "seed": "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice",
  "api_url": "ws://127.0.0.1:9944",
  "size": 1000,
  "app_id": 0,
  "dest": "5H3qehpRTFiB3XwyTzYU43SpG7e8jW87vFug95fxdew76Vyi",
  "amount": 10
}
```

4. Ensure you're running a local Avail node. You can do this with the following command from the root directory:

```bash
cargo build --release
./target/release/avail-node --dev
```

You can also take the latest release from [Github](https://github.com/availproject/avail/releases)

5. To run any example script from the examples/go-examples folder, use the following command format, replacing NAME_OF_THE_FILE with the actual file name you want to run:

```bash
go run extrinsicFoldername/main.go --config config.json
```

For example, to run the staking_nominate.ts script:

```bash
go run dataSubmit/main.go --config config.json
```

This will execute the chosen example script, showcasing how to interact with the Avail network using the avail-js-sdk.

# Data Availability

Runtime Component: DataAvailability\
Runtime Index: 29\
Interface Module Name: dataAvailability

## Create Application Key

Origin Level: Signed

### Interface

```go
func CreateApplicationKey(api *sdk.SubstrateAPI, seed string, data string, WaitForInclusion sdk.WaitFor) (types.Hash, types.Hash, error)
```

#### Parameters

| parameter        | type     | optional | description                                            |
| ---------------- | -------- | -------- | ------------------------------------------------------ |
| api              | API      | false    | api for avail chain                                    |
| WaitForInclusion | WaitFor  | false    | wait for block inclusion or finalization               |
| Seed             | Mnemonic | false    | seed of the account that needs to sign the transaction |
| data             | string   | false    | name of the application key                            |

#### Return value

On failure, a reason of failure is returned. On Success, ApplicationKeyCreated event, transaction hash and block hash is returned.

### Minimal Example

```go
package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"

	"fmt"
)

func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}

	fmt.Println("Submitting data ...")
	WaitFor := sdk.BlockInclusion
	blockHash, txHash, err := tx.CreateApplicationKey(api, config.Seed, "my happyyy", WaitFor)
	if err != nil {
		fmt.Printf("cannot create application key:%v", err)
	}
	fmt.Printf("Application key created successfully with block hash: %v\n and ext hash:%v\n", blockHash.Hex(), txHash.Hex())
	sdk.EventParser(api, blockHash, "ApplicationKeyCreated")
}
```

## Submit Data

Origin Level: Signed

### Interface

```go
func SubmitData(api *sdk.SubstrateAPI, seed string, AppID int, data string, WaitForInclusion sdk.WaitFor) (types.Hash, types.Hash, error)
```

#### Parameters

| parameter        | type          | optional | description                                            |
| ---------------- | ------------- | -------- | ------------------------------------------------------ |
| api              | API           | false    | api for avail chain                                    |
| WaitForInclusion | WaitFor       | false    | wait for block inclusion or finalization               |
| Seed             | Mnemonic      | false    | seed of the account that needs to sign the transaction |
| data             | SignerOptions | true     | data to be submitted                                   |
| AppID            | SignerOptions | true     | AppID in which the transaction needs to be signed      |

#### Return value

On failure, a reason of failure is returned. On Success, DataSubmitted event, transaction data, transaction hash and block hash is returned.

### Minimal Example

```go
package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"

	"fmt"
)

func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}

	appID := 0

	// if app id is greater than 0 then it must be created before submitting data
	if config.AppID != 0 {
		appID = config.AppID
	}
	fmt.Println("Submitting data ...")
	WaitFor := sdk.BlockInclusion

	BlockHash, txHash, err := tx.SubmitData(api, config.Seed, appID, "my happy data", WaitFor)
	if err != nil {
		fmt.Printf("cannot submit data:%v", err)
	}
	fmt.Printf("Data submitted successfully with block hash: %v\n and ext hash:%v", BlockHash.Hex(), txHash.Hex())
}
```

## Submit Block Length Proposal

Origin Level: Root

### Interface

```go
func SubmitBlockLength(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, rows uint32, cols uint32) (types.Hash, types.Hash, error)
```

#### Parameters

| parameter        | type     | optional | description                                            |
| ---------------- | -------- | -------- | ------------------------------------------------------ |
| api              | API      | false    | api for avail chain                                    |
| rows             | number   | false    | number of rows in block                                |
| cols             | number   | false    | number of cols in block                                |
| WaitForInclusion | WaitFor  | false    | wait for block inclusion or finalization               |
| Seed             | Mnemonic | false    | seed of the account that needs to sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, transaction hash and block hash is returned.

### Minimal Example

```go
package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"

	"fmt"
)

func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}

	fmt.Println("Submitting data ...")
	WaitFor := sdk.BlockInclusion
	rows := uint32(128)
	cols := uint32(128)
	blockHash, txHash, err := tx.SubmitBlockLength(api, config.Seed, WaitFor, rows, cols)
	if err != nil {
		fmt.Printf("cannot submit block length:%v", err)
	}
	fmt.Printf("Block Length updated successfully with block hash: %v\n and ext hash:%v\n", blockHash.Hex(), txHash.Hex())
}
```

## Set Application Key

Origin Level: Root

### Interface

```go
func SetApplicationKey(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, oldKey string, newKey string) (types.Hash, types.Hash, error)
```

#### Parameters

| parameter        | type     | optional | description                                            |
| ---------------- | -------- | -------- | ------------------------------------------------------ |
| api              | API      | false    | api for avail chain                                    |
| oldKey           | string   | false    | application key to be replaced                         |
| newKey           | string   | false    | application key that will replace the old one          |
| WaitForInclusion | WaitFor  | false    | wait for block inclusion or finalization               |
| Seed             | Mnemonic | false    | seed of the account that needs to sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, ApplicationKeySet event, transaction hash and block hash is returned.

### Minimal Example

```go
package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"

	"fmt"
)

func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}
	fmt.Println("Submitting data ...")
	WaitFor := sdk.BlockInclusion
	newKey := "newKey"
	oldKey := "oldKey"
	blockHash, txHash, err := tx.SetApplicationKey(api, config.Seed, WaitFor, oldKey, newKey)
	if err != nil {
		fmt.Printf("cannot set key:%v", err)
	}
	fmt.Printf("Application Key updated successfully with block hash: %v\n and ext hash:%v\n", blockHash.Hex(), txHash.Hex())
	sdk.EventParser(api, blockHash, "ApplicationKeySet")
}
```

## Set Submit Data Fee Modifer

Origin Level: Root

### Interface

```go
func SetSubmitDataFeeModifier(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, modifier sdk.DispatchFeeModifier) (types.Hash, types.Hash, error)
```

#### Parameters

| parameter        | type                | optional | description                                            |
| ---------------- | ------------------- | -------- | ------------------------------------------------------ |
| api              | API                 | false    | api for avail chain                                    |
| modifier         | DispatchFeeModifier | false    | new fee modifier values                                |
| WaitForInclusion | WaitFor             | false    | wait for block inclusion or finalization               |
| Seed             | Mnemonic            | false    | seed of the account that needs to sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, transaction hash and block hash is returned.

### Minimal Example

```go
package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"
	"math/big"

	"fmt"
)

func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}
	tenPow18 := new(big.Int).Exp(big.NewInt(10), big.NewInt(18), nil)

	weightMaximumFee := sdk.NewU128(tenPow18)
	weightFeeDivider := sdk.NewU32(20)
	weightFeeMultiplier := sdk.NewU32(1)

	// Create the DispatchFeeModifier
	modifier := sdk.DispatchFeeModifier{
		WeightMaximumFee:    weightMaximumFee,
		WeightFeeDivider:    weightFeeDivider,
		WeightFeeMultiplier: weightFeeMultiplier,
	}
	fmt.Println("Submitting data ...")
	WaitFor := sdk.BlockInclusion
	// submit data
	blockHash, txHash, err := tx.SetSubmitDataFeeModifier(api, config.Seed, WaitFor, modifier)
	if err != nil {
		fmt.Printf("cannot update DA fee:%v", err)
	}
	fmt.Printf("Data Fee modified successfully with block hash: %v\n and ext hash:%v\n", blockHash.Hex(), txHash.Hex())
}
```

## Type Definitions

```go
type WaitFor int

const (
	BlockInclusion WaitFor = iota + 1
	BlockFinalization
)
```

```go
type DispatchFeeModifier struct {
	WeightMaximumFee    types.U128
	WeightFeeDivider    types.U32
	WeightFeeMultiplier types.U32
}
```

# Balances

Runtime Component: Balances\
Runtime Index: 6\
Interface Module Name: balances

## Transfer Keep Alive

Origin Level: Signed

### Interface

```go
func TransferKeepAlive(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, dest string, amount types.UCompact) (types.Hash, types.Hash, error)
```

#### Parameters

| parameter        | type     | optional | description                                            |
| ---------------- | -------- | -------- | ------------------------------------------------------ |
| api              | API      | false    | api for avail chain                                    |
| dest             | string   | false    | account that will receive funds                        |
| amount           | Ucompact | false    | amount that is send. 10^18 is equal to 1 AVL           |
| WaitForInclusion | WaitFor  | false    | wait for block inclusion or finalization               |
| Seed             | Mnemonic | false    | seed of the account that needs to sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, TransferEvent event, transaction hash and block hash is returned.

### Minimal Example

```go
package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"
	"avail-go-sdk/src/sdk/types"
	"math"

	"fmt"
)

func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}
	WaitFor := sdk.BlockInclusion

	amount := uint64(math.Pow(10, 18)) * 10 // send amount 10 AVAIL
	dest := "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
	bondAmountUCompact := types.NewUCompactFromUInt(amount)
	BlockHash, txHash, err := tx.TransferKeepAlive(api, config.Seed, WaitFor, dest, bondAmountUCompact)
	if err != nil {
		fmt.Printf("cannot submit Transaction:%v", err)
	}
	fmt.Printf("Transaction submitted successfully with block hash: %v\n and ext hash:%v", BlockHash.Hex(), txHash.Hex())
	sdk.EventParser(api, BlockHash, "BalanceTransfer")
}
```

## Transfer Allow Death

Origin Level: Signed

### Interface

```go
func TransferAllowDeath(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, dest string, amount types.UCompact) (types.Hash, types.Hash, error)
```

#### Parameters

| parameter        | type     | optional | description                                            |
| ---------------- | -------- | -------- | ------------------------------------------------------ |
| api              | API      | false    | api for avail chain                                    |
| dest             | string   | false    | account that will receive funds                        |
| amount           | Ucompact | false    | amount that is send. 10^18 is equal to 1 AVL           |
| WaitForInclusion | WaitFor  | false    | wait for block inclusion or finalization               |
| Seed             | Mnemonic | false    | seed of the account that needs to sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, TransferEvent event, KilledAccount (optionally) event, transaction hash and block
hash is returned.

### Minimal Example

```go
package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"
	"avail-go-sdk/src/sdk/types"
	"math"

	"fmt"
)

func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}
	WaitFor := sdk.BlockInclusion

	amount := uint64(math.Pow(10, 18)) * 10 // send amount 10 AVAIL
	dest := "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
	bondAmountUCompact := types.NewUCompactFromUInt(amount)
	BlockHash, txHash, err := tx.TransferAllowDeath(api, config.Seed, WaitFor, dest, bondAmountUCompact)
	if err != nil {
		fmt.Printf("cannot submit Transaction:%v", err)
	}
	fmt.Printf("Transaction submitted successfully with block hash: %v\n and ext hash:%v", BlockHash.Hex(), txHash.Hex())
	sdk.EventParser(api, BlockHash, "BalanceTransfer")
}
```

## Transfer All

Origin Level: Signed

### Interface

```go
func TransferAll(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, dest string) (types.Hash, types.Hash, error)
```

#### Parameters

| parameter        | type     | optional | description                                            |
| ---------------- | -------- | -------- | ------------------------------------------------------ |
| api              | API      | false    | api for avail chain                                    |
| dest             | string   | false    | account that will receive funds                        |
| WaitForInclusion | WaitFor  | false    | wait for block inclusion or finalization               |
| Seed             | Mnemonic | false    | seed of the account that needs to sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success transaction hash and block hash is returned.

### Minimal Example

```go
package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"

	"fmt"
)

func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}
	WaitFor := sdk.BlockInclusion

	dest := "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"
	BlockHash, txHash, err := tx.TransferAll(api, config.Seed, WaitFor, dest)
	if err != nil {
		fmt.Printf("cannot submit Transaction:%v", err)
	}
	fmt.Printf("Transaction submitted successfully with block hash: %v\n and ext hash:%v", BlockHash.Hex(), txHash.Hex())
}
```

# Staking

Runtime Component: Staking\
Runtime Index: 10\
Interface Module Name: staking

### Type Definitions

```go
type Payee uint8

const (
Staked WaitFor = iota
Stash
Controller
Account
None
)
```

## Bond

Origin Level: Signed

### Interface

```go
func Bond(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, amount types.UCompact, Payee sdk.Payee) (types.Hash, types.Hash, error)
```

#### Parameters

| parameter        | type     | optional | description                                             |
| ---------------- | -------- | -------- | ------------------------------------------------------- |
| api              | API      | false    | api for avail chain                                     |
| amount           | Ucompact | false    | amount that is bond.                                    |
| payee            | Payee    | false    | Can be: "Staked", "Stash", "None" or an account address |
| WaitForInclusion | WaitFor  | false    | wait for block inclusion or finalization                |
| Seed             | Mnemonic | false    | seed of the account that needs to sign the transaction  |

#### Return value

On failure, a reason of failure is returned. On Success, Bonded event, transaction hash and block hash is returned.

### Minimal Example

```go
package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"

	"fmt"
)

func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}
	WaitFor := sdk.BlockInclusion
	Payee := sdk.Staked

	bondAmount := int64(1000)

	BlockHash, txHash, err := tx.Bond(api, config.Seed, WaitFor, bondAmount, sdk.Payee(Payee))
	if err != nil {
		fmt.Printf("cannot submit Transaction:%v", err)
	}
	fmt.Printf("Transaction submitted successfully with block hash: %v\n and ext hash:%v", BlockHash.Hex(), txHash.Hex())
	sdk.EventParser(api, BlockHash, "Bond")
}
```

## Bond Extra

Origin Level: Signed

### Interface

```go
func BondExtra(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, amount types.UCompact) (types.Hash, types.Hash, error)
```

#### Parameters

| parameter        | type        | optional | description                                             |
| ---------------- | ----------- | -------- | ------------------------------------------------------- |
| api              | API         | false    | api for avail chain                                     |
| amount           | Ucompact    | false    | additional amount that is bond. 10^18 is equal to 1 AVL |
| WaitForInclusion | WaitFor     | false    | wait for block inclusion or finalization                |
| account          | KeyringPair | false    | account that will send and sign the transaction         |
| Seed             | Mnemonic    | false    | seed of the account that needs to sign the transaction  |

#### Return value

On failure, a reason of failure is returned. On Success, transaction hash and block hash is returned.

### Minimal Example

```go
package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"

	"fmt"
)

func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}
	WaitFor := sdk.BlockInclusion

	bondAmount := int64(1000)

	BlockHash, txHash, err := tx.BondExtra(api, config.Seed, WaitFor, bondAmount)
	if err != nil {
		fmt.Printf("cannot submit Transaction:%v", err)
	}
	fmt.Printf("Transaction submitted successfully with block hash: %v\n and ext hash:%v", BlockHash.Hex(), txHash.Hex())
}
```

## Chill

Origin Level: Signed

### Interface

```go
func Chill(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor) (types.Hash, types.Hash, error)
```

#### Parameters

| parameter        | type     | optional | description                                            |
| ---------------- | -------- | -------- | ------------------------------------------------------ |
| WaitForInclusion | WaitFor  | false    | wait for block inclusion or finalization               |
| api              | API      | false    | api for avail chain                                    |
| Seed             | Mnemonic | false    | seed of the account that needs to sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, transaction hash and block hash is returned.

### Minimal Example

```go
package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"

	"fmt"
)

func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}
	WaitFor := sdk.BlockInclusion

	BlockHash, txHash, err := tx.Chill(api, config.Seed, WaitFor)
	if err != nil {
		fmt.Printf("cannot submit Transaction:%v", err)
	}
	fmt.Printf("Transaction submitted successfully with block hash: %v\n and ext hash:%v", BlockHash.Hex(), txHash.Hex())
}
```

## Chill Other

Origin Level: Signed

### Interface

```go
func ChillOther(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, stash string) (types.Hash, types.Hash, error)
```

#### Parameters

| parameter        | type     | optional | description                                            |
| ---------------- | -------- | -------- | ------------------------------------------------------ |
| stash            | string   | false    | Address that needs to be chilled                       |
| WaitForInclusion | WaitFor  | false    | wait for block inclusion or finalization               |
| api              | API      | false    | api for avail chain                                    |
| Seed             | Mnemonic | false    | seed of the account that needs to sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, transaction hash and block hash is returned.

### Minimal Example

```go
package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"

	"fmt"
)

func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}
	WaitFor := sdk.BlockInclusion

	stash := "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"
	BlockHash, txHash, err := tx.ChillOther(api, config.Seed, WaitFor, stash)
	if err != nil {
		fmt.Printf("cannot submit Transaction:%v", err)
	}
	fmt.Printf("Transaction submitted successfully with block hash: %v\n and ext hash:%v", BlockHash.Hex(), txHash.Hex())
}
```

## Nominate

Origin Level: Signed

### Interface

```go
func Nominate(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, stash []string) (types.Hash, types.Hash, error)
```

#### Parameters

| parameter        | type     | optional | description                                            |
| ---------------- | -------- | -------- | ------------------------------------------------------ |
| stash            | string[] | false    | list od addresses to nominate                          |
| WaitForInclusion | WaitFor  | false    | wait for block inclusion or finalization               |
| api              | API      | false    | api for avail chain                                    |
| Seed             | Mnemonic | false    | seed of the account that needs to sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, transaction hash and block hash is returned.

### Minimal Example

```go
package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"

	"fmt"
)

func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}
	WaitFor := sdk.BlockInclusion

	stash := []string{"5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY", "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"}
	BlockHash, txHash, err := tx.Nominate(api, config.Seed, WaitFor, stash)
	if err != nil {
		fmt.Printf("cannot submit Transaction:%v", err)
	}
	fmt.Printf("Transaction submitted successfully with block hash: %v\n and ext hash:%v", BlockHash.Hex(), txHash.Hex())
}
```

## Unbond

Origin Level: Signed

### Interface

```go
func Unbond(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, amount types.UCompact) (types.Hash, types.Hash, error)
```

#### Parameters

| parameter        | type     | optional | description                                            |
| ---------------- | -------- | -------- | ------------------------------------------------------ |
| amount           | Ucompact | false    | amount of tokens to unbond                             |
| WaitForInclusion | WaitFor  | false    | wait for block inclusion or finalization               |
| api              | API      | false    | api for avail chain                                    |
| Seed             | Mnemonic | false    | seed of the account that needs to sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, Unbonded event, transaction hash and block hash is returned.

### Minimal Example

```go
package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"
	"avail-go-sdk/src/sdk/types"
	"math/big"

	"fmt"
)

func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}
	WaitFor := sdk.BlockInclusion

	bondAmount := new(big.Int)
	bondAmount.SetString("100000000000000000000", 10)

	// Convert big.Int to types.UCompact
	bondAmountUCompact := types.NewUCompact(bondAmount)
	BlockHash, txHash, err := tx.Unbond(api, config.Seed, WaitFor, bondAmountUCompact)
	if err != nil {
		fmt.Printf("cannot submit Transaction:%v", err)
	}
	fmt.Printf("Transaction submitted successfully with block hash: %v\n and ext hash:%v", BlockHash.Hex(), txHash.Hex())
}
```

## Validate

Origin Level: Signed

### Interface

```go
func Validate(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, commissionNum int) (types.Hash, types.Hash, error)
```

#### Parameters

| parameter        | type     | optional | description                                            |
| ---------------- | -------- | -------- | ------------------------------------------------------ |
| commission       | number   | false    | how much validator charge nominators in 0 - 100 range  |
| WaitForInclusion | WaitFor  | false    | wait for block inclusion or finalization               |
| api              | API      | false    | api for avail chain                                    |
| Seed             | Mnemonic | false    | seed of the account that needs to sign the transaction |

#### Return value

On failure, a reason of failure is returned. On Success, transaction hash and block hash is returned.

### Minimal Example

```js
package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"
	"fmt"
)

func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}
	WaitFor := sdk.BlockInclusion
	commission := 5
	BlockHash, txHash, err := tx.Validate(api, config.Seed, WaitFor, commission)
	if err != nil {
		fmt.Printf("cannot submit Transaction:%v", err)
	}
	fmt.Printf("Transaction submitted successfully with block hash: %v\n and ext hash:%v", BlockHash.Hex(), txHash.Hex())
}
```
