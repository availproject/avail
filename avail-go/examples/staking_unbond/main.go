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
