package main

import (
	"math"

	"github.com/availproject/avail-go-sdk/src/config"
	"github.com/availproject/avail-go-sdk/src/sdk"
	"github.com/availproject/avail-go-sdk/src/sdk/tx"
	"github.com/availproject/avail-go-sdk/src/sdk/types"

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
