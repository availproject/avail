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

// ProofResponse struct represents the response from the queryDataProof2 RPC call
type ProofResponse struct {
	DataProof DataProof
	Message   Message // Interface to capture different message types
}

type TxDataRoot struct {
	DataRoot   types.Hash
	BlobRoot   types.Hash
	BridgeRoot types.Hash
}

// DataProof struct represents the data proof response
type DataProof struct {
	Roots          TxDataRoot
	Proof          []types.Hash
	NumberOfLeaves uint32 // Change to uint32 to match Rust u32
	LeafIndex      uint32 // Change to uint32 to match Rust u32
	Leaf           types.Hash
}

// Message interface represents the enum variants
type Message interface {
	isMessage()
}

type BoundedData struct {
	Data []byte
}

// BoundedDataMaxLen is the maximum length for the bounded data
const BoundedDataMaxLen = 32 // Adjust the maximum length as needed

// ArbitraryMessage struct represents the ArbitraryMessage variant
type ArbitraryMessage struct {
	BoundedData
}

func (a *ArbitraryMessage) isMessage() {}

// FungibleToken struct represents the FungibleToken variant
type FungibleToken struct {
	AssetID types.Hash
	Amount  uint128 // Define uint128 type as needed
}

func (f *FungibleToken) isMessage() {}

// Define uint128 type if not already defined
type uint128 uint64

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
	h, _ := types.NewHashFromHexString(blockHash.Hex())
	transactionIndex := types.NewU32(1)

	// query proof
	var response ProofResponse
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
		case *ArbitraryMessage:
			fmt.Println("Arbitrary Message:", m.BoundedData)
		case *FungibleToken:
			fmt.Println("Fungible Token AssetID:", m.AssetID.Hex(), "Amount:", m.Amount)
		default:
			fmt.Println("Unknown Message Type")
		}
	} else {
		fmt.Println("Message: null")
	}
}
