package main

import (
	"fmt"

	"github.com/availproject/avail-go-sdk/src/config"
	"github.com/availproject/avail-go-sdk/src/sdk"
	"github.com/availproject/avail-go-sdk/src/sdk/types"
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

	var properties types.ChainProperties
	err = api.Client.Call(&properties, "system_properties", nil)

	if err != nil {
		panic(fmt.Sprintf("cannot get properties:%v", err))
	}

	fmt.Printf("\nChain properties:TokenSymbol:%#v\nTokenDecimals:%d\nSS58Format:%d\n", properties.TokenSymbol, properties.TokenDecimals, properties.SS58Format)

}
