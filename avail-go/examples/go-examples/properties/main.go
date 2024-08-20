package main

import (
	"avail-go-sdk/extrinsic"
	"avail-go-sdk/sdk"
	"avail-go-sdk-examples/internal/config"
	"flag"
	"fmt"
	"log"
	"os"
)

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
	if api == nil || api.Client == nil {
		log.Fatal("API client is not properly initialized")
	}

	var properties extrinsic.ChainProperties
	err = api.Client.Call(&properties, "system_properties", nil)

	if err != nil {
		panic(fmt.Sprintf("cannot get properties:%w", err))
	}

	fmt.Printf("Chain properties: \nIsEthereum:%#v\nTokenSymbol:%#v\nTokenDecimals:%d\nSS58Format:%d\n", properties.IsEthereum, properties.TokenSymbol, properties.TokenDecimals, properties.SS58Format)

}
