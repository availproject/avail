package main

import (
	"avail-go-sdk-examples/internal/config"
	"flag"
	"fmt"
	"log"
	"os"

	"avail-go-sdk/sdk"
)

// The following example shows how to connect to a node and display some basic information.
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
