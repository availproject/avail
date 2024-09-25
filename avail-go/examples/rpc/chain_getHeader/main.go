package main

import (
	"fmt"
	"log"

	"github.com/availproject/avail-go-sdk/src/config"
	"github.com/availproject/avail-go-sdk/src/rpc"
	"github.com/availproject/avail-go-sdk/src/sdk"
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
	if api == nil || api.Client == nil {
		log.Fatal("API client is not properly initialized")
	}
	resp, err := rpc.GetHeaderLatest(api.Client)
	if err != nil {
		fmt.Printf("cannot call latest header RPC:%v", err)
	}
	fmt.Println(resp)
}
