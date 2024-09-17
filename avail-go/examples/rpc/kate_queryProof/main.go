package main

import (
	"avail-go-sdk/src/config"
	"avail-go-sdk/src/rpc"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/call"
	"avail-go-sdk/src/sdk/tx"
	"encoding/json"
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

	appID := 0

	// if app id is greater than 0 then it must be created before submitting data
	if config.AppID != 0 {
		appID = config.AppID
	}
	WaitFor := sdk.BlockFinalization

	blockHash, txHash, err := tx.SubmitData(api, config.Seed, appID, "my happy data", WaitFor)
	if err != nil {
		fmt.Printf("cannot submit data:%v", err)
	}
	fmt.Printf("Data submitted successfully with block hash: %v\n and ext hash:%v\n", blockHash.Hex(), txHash.Hex())
	testCell := []rpc.Cell{rpc.NewCell(0, 0), rpc.NewCell(0, 1)}
	result, err := call.Query_proof(api, testCell, blockHash)
	if err != nil {
		fmt.Printf("cannot query proof:%v", err)
	}

	resultJSON, err := json.Marshal(result)
	if err != nil {
		fmt.Println("Error marshalling result to JSON:", err)
		return
	}

	fmt.Println(string(resultJSON))
}
