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

// The following example shows how to query Merkle proof for particular data and block hash.
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
	var data HeaderF
	err = api.Client.Call(&data, "kate_queryDataProof", transactionIndex, h)
	if err != nil {
		panic(fmt.Sprintf("%v\n", err))
	}
	fmt.Printf("Root:%v\n", data.Root.Hex())
	// print array of proof
	fmt.Printf("Proof:\n")
	for _, p := range data.Proof {
		fmt.Printf("%v\n", p.Hex())
	}

	fmt.Printf("Number of leaves: %v\n", data.NumberOfLeaves)
	fmt.Printf("Leaf index: %v\n", data.LeafIndex)
	fmt.Printf("Leaf: %v\n", data.Leaf.Hex())
}

// HeaderF struct represents response from queryDataProof
type HeaderF struct {
	Root             types.Hash
	Proof            []types.Hash
	NumberOfLeaves	 int
	LeafIndex		 int
	Leaf             types.Hash
}
