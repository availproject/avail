package main

import (
	"fmt"
	"log"

	"github.com/availproject/avail-go-sdk/src/config"
	"github.com/availproject/avail-go-sdk/src/rpc"
	"github.com/availproject/avail-go-sdk/src/sdk"
	"github.com/availproject/avail-go-sdk/src/sdk/call"
	"github.com/availproject/avail-go-sdk/src/sdk/tx"
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

	blockHash, _, err := tx.SubmitData(api, config.Seed, appID, "my happy data", WaitFor)
	if err != nil {
		fmt.Printf("cannot submit data:%v", err)
	}
	fmt.Printf("Transaction included in finalized block: %v\n", blockHash.Hex())

	transactionIndex := sdk.NewU32(1)

	// query proof
	response, err := call.Query_dataproof(api, transactionIndex, blockHash)
	if err != nil {
		fmt.Printf("cannot query proof:%v", err)
	}
	fmt.Printf("DataRoot:%v\n", response.DataProof.Roots.DataRoot.Hex())
	fmt.Printf("BlobRoot:%v\n", response.DataProof.Roots.BlobRoot.Hex())
	fmt.Printf("BridgeRoot:%v\n", response.DataProof.Roots.BridgeRoot.Hex())
	// print array of proof
	fmt.Printf("Proof:\n")
	for _, p := range response.DataProof.Proof {
		fmt.Printf("%v\n", p.Hex())
	}

	fmt.Printf("Number of leaves: %v\n", response.DataProof.NumberOfLeaves)
	fmt.Printf("Leaf index: %v\n", response.DataProof.LeafIndex)
	fmt.Printf("Leaf: %v\n", response.DataProof.Leaf.Hex())

	// Access the message based on its type
	if response.Message != nil {
		switch m := response.Message.(type) {
		case *rpc.ArbitraryMessage:
			fmt.Println("Arbitrary Message:", m.BoundedData)
		case *rpc.FungibleToken:
			fmt.Println("Fungible Token AssetID:", m.AssetID.Hex(), "Amount:", m.Amount)
		default:
			fmt.Println("Unknown Message Type")
		}
	} else {
		fmt.Println("Message: null")
	}
}
