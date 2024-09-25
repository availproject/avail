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
	rows := uint32(128)
	cols := uint32(128)
	blockHash, txHash, err := tx.SubmitBlockLength(api, config.Seed, WaitFor, rows, cols)
	if err != nil {
		fmt.Printf("cannot submit block length:%v", err)
	}
	fmt.Printf("Block Length updated successfully with block hash: %v\n and ext hash:%v\n", blockHash.Hex(), txHash.Hex())
}
