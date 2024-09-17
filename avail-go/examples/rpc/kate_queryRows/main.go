package main

import (
	"fmt"

	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/call"
	"avail-go-sdk/src/sdk/tx"
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

	defer func() {
		if r := recover(); r != nil {
			fmt.Println("Recovered in main", r)
		}
	}()
	myArr := make([]uint32, 1)
	myArr[0] = 0
	response, err := call.Query_rows(api, myArr, blockHash)
	if err != nil {
		fmt.Println("Error calling api.Client.Call:", err)
		return
	}

	formattedResponse := make([][]string, len(response))

	for i, innerSlice := range response {
		formattedResponse[i] = make([]string, len(innerSlice))
		for j, num := range innerSlice {
			formattedResponse[i][j] = sdk.FormatBN(num)
		}
	}
	fmt.Println(formattedResponse)

}
