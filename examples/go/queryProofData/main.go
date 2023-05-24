package main

import (
	"avail-gsrpc-examples/internal/config"
	"flag"
	"fmt"
	"log"
	"os"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

// The following example shows how to query Merkle proof for particular data and block hash.
func main() {
	// block hash to query proof
	blockHash := "fcedf5363a5e5126d8d53fdd9a251dd9bf9ee965dd3ed15212aa356709bd4bbf"
	h, _ := types.NewHashFromHexString(blockHash)
    dataIndex := types.NewU32(0)

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
	var data HeaderF
	err = api.Client.Call(&data, "kate_queryDataProof", dataIndex, h)
	if err != nil {
		panic(fmt.Sprintf("%v\n", err))
	}

	fmt.Printf("%v\n", data)

	fmt.Printf("Root:%v\n", data.Root.Hex())
	// print array of proof
	fmt.Printf("Proof:\n")
	for _, p := range data.Proof {
		fmt.Printf("%v\n", p.Hex())
	}

	fmt.Printf("Number of leaves: %v\n", data.Number_Of_Leaves)
	fmt.Printf("Leaf index: %v\n", data.Leaf_index)
	fmt.Printf("Leaf: %v\n", data.Leaf.Hex())
}

// HeaderF struct represents response from queryDataProof
type HeaderF struct {
	Root             types.Hash
	Proof            []types.Hash
	Number_Of_Leaves int
	Leaf_index       int
	Leaf             types.Hash
}
