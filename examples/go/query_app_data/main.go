package main

import (
	"avail-gsrpc-examples/internal/config"
	"avail-gsrpc-examples/internal/extrinsics"
	"encoding/json"
	"flag"
	"fmt"
	"log"
	"math/big"
	"os"
	"strings"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

// type x [][]types.U256
type BigInt struct {
	*big.Int
}

// UnmarshalJSON defines custom unmarshalling for BigInt.
func (bi *BigInt) UnmarshalJSON(data []byte) error {
	// Unmarshal as raw JSON string
	var rawString string
	if err := json.Unmarshal(data, &rawString); err != nil {
		return err
	}

	// Remove 0x prefix if present
	str := strings.TrimPrefix(rawString, "0x")

	// Initialize bi.Int if it's nil
	if bi.Int == nil {
		bi.Int = new(big.Int)
	}

	// If the string is empty, set bi to zero
	if str == "" {
		bi.SetInt64(0)
		return nil
	}

	// Parse the string in base 16
	_, success := bi.SetString(str, 16)
	if !success {
		return fmt.Errorf("invalid hex string")
	}

	return nil
}

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

	response := make([][]BigInt, 1)
	response[0] = make([]BigInt, 1)

	// Assuming types.U256 has a constructor like NewU256
	zeroValue := types.NewU256(*big.NewInt(0)) // Replace with the actual constructor or method
	response[0][0] = BigInt{zeroValue.Int}

	defer func() {
		if r := recover(); r != nil {
			fmt.Println("Recovered in main", r)
		}
	}()

	err = api.Client.Call(&response, "kate_queryAppData", appId, h)
	if err != nil {
		fmt.Println("Error calling api.Client.Call:", err)
		return
	}
	var sb strings.Builder
	sb.WriteString("[\n  [\n") // Start the outer array

	for i, innerSlice := range response {
		for j, num := range innerSlice {
			formattedNum := formatBigInt(num)
			sb.WriteString("    ")
			sb.WriteString(formattedNum)
			if j < len(innerSlice)-1 {
				sb.WriteString(",\n")
			}
		}
		if i < len(response)-1 {
			sb.WriteString("\n  ],\n  [\n") // Close the current inner array and start a new one
		} else {
			sb.WriteString("\n  ]\n") // Close the last inner array
		}
	}

	sb.WriteString("]") // Close the outer array
	formattedResponse := sb.String()

	fmt.Println(formattedResponse)

}

func formatBigInt(n BigInt) string {
	s := n.String() // Convert number to string
	var result strings.Builder
	count := 0

	for i := len(s) - 1; i >= 0; i-- {
		if count == 3 {
			result.WriteString(",")
			count = 0
		}
		result.WriteByte(s[i])
		count++
	}

	// Reverse the string to get the correct order
	reversed := result.String()
	var formatted strings.Builder
	for i := len(reversed) - 1; i >= 0; i-- {
		formatted.WriteByte(reversed[i])
	}

	return formatted.String()
}
