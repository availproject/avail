package main

import (
	"github.com/availproject/avail-go-sdk/src/config"
	"github.com/availproject/avail-go-sdk/src/sdk"
	"github.com/availproject/avail-go-sdk/src/sdk/tx"

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
