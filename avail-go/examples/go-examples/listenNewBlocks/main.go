package main

import (
	"avail-go-sdk-examples/internal/config"
	"flag"
	"fmt"
	"log"
	"os"

	"avail-go-sdk/sdk"
)

// The following example shows how to connect to a node and listen for a new blocks
func main() {
	var configJSON string
	var config config.Config
	flag.StringVar(&configJSON, "config", "", "config json file")
	flag.Parse()

	if configJSON == "" {
		log.Println("No config file provided. Exiting...")
		os.Exit(0)
	}

	err := config.GetConfig(configJSON)
	if err != nil {
		panic(fmt.Sprintf("cannot get config:%v", err))
	}

	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		panic(fmt.Sprintf("cannot create api client:%v", err))
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
