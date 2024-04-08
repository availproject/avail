package main

import (
	"avail-gsrpc-examples/internal/config"
	"avail-gsrpc-examples/internal/extrinsics"
	"flag"
	"fmt"
	"log"
	"os"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

// type x [][]types.U256

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
	if api == nil || api.Client == nil {
		log.Fatal("API client is not properly initialized")
	}
	if err != nil {
		panic(fmt.Sprintf("cannot create api client:%v", err))
	}

	var finalizedBlockCh = make(chan types.Hash)
	go func() {
		err = extrinsics.SubmitData(api, "data", config.Seed, 1, finalizedBlockCh)
		if err != nil {
			panic(fmt.Sprintf("cannot submit data:%v", err))
		}
	}()

	// block hash to query proof
	blockHash := <-finalizedBlockCh
	fmt.Printf("Transaction included in finalized block: %v\n", blockHash.Hex())
	h, err := types.NewHashFromHexString(blockHash.Hex())
	if err != nil {
		panic(fmt.Sprintf("cannot create api client:%v", err))
	}
	appId := types.NewUCompactFromUInt(uint64(1))

	// query proof
	// var response x
	outerSize := 5 // Adjust this as needed
	innerSize := 5 // Adjust this as needed

	// Initialize the outer slice.
	response := make([][]types.U256, outerSize)

	// Initialize each inner slice.
	for i := range response {
		response[i] = make([]types.U256, innerSize)
	}
	err = api.Client.Call(&response, "kate_queryAppData", appId, h)
	if err != nil {
		fmt.Println("Error calling api.Client.Call:", err)
		return
	}
	// fmt.Print(response)

}
