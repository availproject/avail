package main

import (
	"avail-gsrpc-examples/internal/config"
	"flag"
	"log"
	"os"

	"github.com/centrifuge/go-substrate-rpc-client/signature"
	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

// Example use: go run cmd/blockListener/blockListener.go -config config.json

func main() {
	// This example shows how to subscribe to new blocks.

	// It displays the block number every time a new block is seen by the node you are connected to.

	// To use the default node url - config.Default().RPCURL
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
		panic(err)
	}

	api, err := gsrpc.NewSubstrateAPI(config.ApiURL)
	if err != nil {
		panic(err)
	}
	log.Println("gsrpc connected to Substrate API...")

	sub, err := api.RPC.Chain.SubscribeNewHeads()
	if err != nil {
		panic(err)
	}
	defer sub.Unsubscribe()
	log.Println("Subscribed to new headers...")

	count := 0

	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		panic(err)
	}

	// if testing locally with Alice account, use signature.TestKeyringPairAlice.PublicKey as last param
	// mneumonic for local Alice account: `bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice`
	key, err := types.CreateStorageKey(meta, "System", "Account", signature.TestKeyringPairAlice.PublicKey)
	if err != nil {
		panic(err)
	}

	var accountInfo types.AccountInfo
	ok, err := api.RPC.State.GetStorageLatest(key, &accountInfo)
	if err != nil || !ok {
		panic(err)
	}

	for {
		head := <-sub.Chan()
		count++

		blockHash, err := api.RPC.Chain.GetBlockHash(uint64(head.Number))
		if err != nil {
			panic(err)
		}
		log.Printf("Chain is at block: #%v with hash %v\n", head.Number, blockHash.Hex())

		if count == 10 {
			sub.Unsubscribe()
			break
		}
	}
}
