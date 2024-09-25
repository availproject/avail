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
	blockHash, txHash, err := tx.CreateApplicationKey(api, config.Seed, "my happyyy", WaitFor)
	if err != nil {
		fmt.Printf("cannot create application key:%v", err)
	}
	fmt.Printf("Application key created successfully with block hash: %v\n and ext hash:%v\n", blockHash.Hex(), txHash.Hex())
	sdk.EventParser(api, blockHash, "ApplicationKeyCreated")
}
