package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/rpc"
	"avail-go-sdk/src/sdk"
	"encoding/json"
	"fmt"
	"log"

	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
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

	var finalizedBlockCh = make(chan types.Hash)
	go func() {
		err = sdk.SubmitData(api, "data", config.Seed, 1, finalizedBlockCh)
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

	defer func() {
		if r := recover(); r != nil {
			fmt.Println("Recovered in main", r)
		}
	}()
	var result []rpc.GDataProof
	testCell := []rpc.Cell{rpc.NewCell(0, 0), rpc.NewCell(0, 1)}

	testCellJSON, _ := json.Marshal(testCell)
	fmt.Println("Test Cell JSON:", string(testCellJSON))

	err = api.Client.Call(&result, "kate_queryProof", testCell, h)
	if err != nil {
		fmt.Println("Error calling api.Client.Call:", err)
	}

	resultJSON, err := json.Marshal(result)
	if err != nil {
		fmt.Println("Error marshalling result to JSON:", err)
		return
	}

	fmt.Println(string(resultJSON))
}
