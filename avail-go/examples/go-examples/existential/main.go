package main

import (
	"avail-go-sdk-examples/internal/config"
	"flag"
	"fmt"
	"log"
	"os"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	. "github.com/centrifuge/go-substrate-rpc-client/v4/types/codec"
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
	api, err := gsrpc.NewSubstrateAPI(config.ApiURL)
	if err != nil {
		panic(fmt.Sprintf("cannot create api client:%v", err))
	}
	if api == nil || api.Client == nil {
		log.Fatal("API client is not properly initialized")
	}
	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		panic(fmt.Sprintf("cannot get metadata:%w", err))
	}
	var existentialDeposit types.U128

	for _, mod := range meta.AsMetadataV14.Pallets {
		if string(mod.Name) == "Balances" {
			for _, constant := range mod.Constants {
				if string(constant.Name) == "ExistentialDeposit" {
					err = Decode(constant.Value, &existentialDeposit)
					if err != nil {
						log.Fatalf("Failed to decode ExistentialDeposit: %v", err)
					}
					fmt.Printf("Existential Deposit: %d\n", existentialDeposit)
					return
				}
			}
		}
	}

	fmt.Printf("Existential Deposit: %d\n", existentialDeposit)
}
