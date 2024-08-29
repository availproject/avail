package main

import (
	"fmt"

	"avail-go-sdk/src/config"
	"avail-go-sdk/src/rpc"
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
	h, _ := sdk.NewHashFromHexString(blockHash.Hex())
	transactionIndex := sdk.NewU32(1)

	// query proof
	var response rpc.ProofResponse
	err = api.Client.Call(&response, "kate_queryDataProof", transactionIndex, h)
	if err != nil {
		panic(fmt.Sprintf("%v\n", err))
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
