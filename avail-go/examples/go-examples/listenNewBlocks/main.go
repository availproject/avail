package main

import (
	"fmt"

	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
)

// The following example shows how to connect to a node and listen for a new blocks
func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}

	subscription, err := api.RPC.Chain.SubscribeNewHeads()
	if err != nil {
		panic(fmt.Sprintf("cannot subscribe:%v", err))
	}

	// number of blocks to wait
	waitForBlocks := 5
	count := 0
	for i := range subscription.Chan() {
		count++
		fmt.Printf("Chain is at block: #%v\n", i.Number)
		if count == waitForBlocks {
			break
		}
	}

	subscription.Unsubscribe()
}
