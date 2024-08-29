package main

import (
	"fmt"
	"math/big"

	"avail-go-sdk/src/config"
	"avail-go-sdk/src/sdk"
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

	var finalizedBlockCh = sdk.CreateChannel()
	go func() {
		err = sdk.SubmitData(api, "data", config.Seed, 1, finalizedBlockCh)
		if err != nil {
			panic(fmt.Sprintf("cannot submit data:%v", err))
		}
	}()

	// block hash to query proof
	blockHash := <-finalizedBlockCh
	fmt.Printf("Transaction included in finalized block: %v\n", blockHash.Hex())
	h, err := sdk.NewHashFromHexString(blockHash.Hex())
	if err != nil {
		panic(fmt.Sprintf("cannot create api client:%v", err))
	}

	response := make([][]sdk.BigInt, 1)
	response[0] = make([]sdk.BigInt, 1)

	// Assuming types.U256 has a constructor like NewU256
	zeroValue := sdk.NewU256(big.NewInt(0)) // Replace with the actual constructor or method
	response[0][0] = sdk.BigInt{Int: zeroValue.Int}

	defer func() {
		if r := recover(); r != nil {
			fmt.Println("Recovered in main", r)
		}
	}()
	myArr := make([]uint32, 1)
	myArr[0] = 0
	err = api.Client.Call(&response, "kate_queryRows", myArr, h)
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
