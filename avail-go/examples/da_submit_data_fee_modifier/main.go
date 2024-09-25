package main

import (
	"math/big"

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
	blockHash, txHash, err := tx.SetSubmitDataFeeModifier(api, config.Seed, WaitFor, modifier)
	if err != nil {
		fmt.Printf("cannot update DA fee:%v", err)
	}
	fmt.Printf("Data Fee modified successfully with block hash: %v\n and ext hash:%v\n", blockHash.Hex(), txHash.Hex())

}
