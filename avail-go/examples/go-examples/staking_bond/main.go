package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"
	"avail-go-sdk/src/sdk/types"
	"math/big"

	"fmt"
)

// submitData creates a transaction and makes a Avail data submission
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
	bondAmount.SetString("1000000000000000000000", 10) // Set bondAmount to 1000000000000000000000

	// Convert big.Int to types.UCompact
	bondAmountUCompact := types.NewUCompact(bondAmount)
	BlockHash, txHash, err := tx.Bond(api, config.Seed, WaitFor, bondAmountUCompact)
	if err != nil {
		fmt.Printf("cannot submit data:%v", err)
	}
	fmt.Printf("Data submitted successfully with block hash: %v\n and ext hash:%v", BlockHash.Hex(), txHash.Hex())
}
