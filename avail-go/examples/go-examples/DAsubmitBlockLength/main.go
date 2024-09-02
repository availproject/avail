package main

import (
	// "avail-go-sdk-examples/internal/config"

	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/tx"

	// "avail-gsrpc-examples/internal/extrinsics"

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
	// submit data
	blockHash, txHash, err := tx.SubmitBlockLength(api, config.Seed, WaitFor)
	if err != nil {
		fmt.Printf("cannot submit data:%v", err)
	}
	fmt.Printf("Data submitted successfully with block hash: %v\n and ext hash:%v\n", blockHash.Hex(), txHash.Hex())
}
