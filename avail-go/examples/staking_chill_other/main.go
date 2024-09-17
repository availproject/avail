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
		fmt.Printf("cannot submit data:%v", err)
	}
	fmt.Printf("Transaction submitted successfully with block hash: %v\n and ext hash:%v", BlockHash.Hex(), txHash.Hex())
}
