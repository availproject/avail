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
	"golang.org/x/crypto/sha3"
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

	var batchHash [32]byte

	k := sha3.NewLegacyKeccak256()
	k.Write([]byte("data"))
	k.Sum(batchHash[:0])

	// block hash to query proof
	blockHash := <-finalizedBlockCh
	fmt.Printf("Transaction included in finalized block: %v\n", blockHash.Hex())
	h, _ := types.NewHashFromHexString(blockHash.Hex())

	block, err := api.RPC.Chain.GetBlock(blockHash)
	if err != nil {
		panic(fmt.Sprintf("cannot get block:%w", err))
	}

	var dataProof DataProof
	for i := 1; i <= len(block.Block.Extrinsics); i++ {
		// query proof
		var data ProofResponse
		err = api.Client.Call(&data, "kate_queryDataProofV2", i, h)
		if err != nil {
			panic(fmt.Sprintf("%v\n", err))
		}

		if data.DataProof.Leaf.Hex() == fmt.Sprintf("%#x", batchHash) {
			dataProof = data.DataProof
			break
		}
	}

	fmt.Printf("Root:%v\n", dataProof.DataRoot.Hex())
	// print array of proof
	fmt.Printf("Proof:\n")
	for _, p := range dataProof.Proof {
		fmt.Printf("%v\n", p.Hex())
	}

	fmt.Printf("Number of leaves: %v\n", dataProof.NumberOfLeaves)
	fmt.Printf("Leaf index: %v\n", dataProof.LeafIndex)
	fmt.Printf("Leaf: %v\n", dataProof.Leaf.Hex())
}

type ProofResponse struct {
	DataProof DataProof `koanf:"dataProof"`
	message   []byte    `koanf:"message"`
}

// HeaderF struct represents response from queryDataProof
type DataProof struct {
	DataRoot       types.Hash   `koanf:"dataRoot"`
	BlobRoot       types.Hash   `koanf:"blobRoot"`
	BridgeRoot     types.Hash   `koanf:"bridgeRoot"`
	Proof          []types.Hash `koanf:"proof"`
	NumberOfLeaves int          `koanf:"numberOfLeaves"`
	LeafIndex      int          `koanf:"leafIndex"`
	Leaf           types.Hash   `koanf:"leaf"`
}
