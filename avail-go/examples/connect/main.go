package main

import (
	"fmt"

	"github.com/availproject/avail-go-sdk/src/config"
	"github.com/availproject/avail-go-sdk/src/sdk"
)

// The following example shows how to connect to a node and display some basic information.
func main() {
	config, err := config.LoadConfig()
	if err != nil {
		fmt.Printf("cannot load config:%v", err)
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}
	chain, err := api.RPC.System.Chain()
	if err != nil {
		panic(fmt.Sprintf("cannot get chain:%v", err))
	}
	name, err := api.RPC.System.Name()
	if err != nil {
		panic(fmt.Sprintf("cannot get name:%v", err))
	}

	version, err := api.RPC.System.Version()
	if err != nil {
		panic(fmt.Sprintf("cannot get version:%v", err))
	}

	fmt.Printf("Connected to chain %v using %v and node version %v\n", chain, name, version)
}
