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
