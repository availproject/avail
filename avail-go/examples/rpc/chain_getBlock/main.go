package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/rpc"
	"avail-go-sdk/src/sdk"
	"fmt"
	"log"
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
	resp, err := rpc.GetAvailBlockLatest(api.Client)
	if err != nil {
		fmt.Printf("cannot author rotate:%v", err)
	}
	fmt.Println(resp)
}
