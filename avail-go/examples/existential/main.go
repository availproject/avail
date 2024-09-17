package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
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
	sdk.ExistentialDeposit(api)
}
