package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"time"

	"avail-gsrpc-examples/internal/config"
	"avail-gsrpc-examples/internal/extrinsics"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	// "github.com/centrifuge/go-substrate-rpc-client/v4/types/codec"
)

func main() {

	// This sample shows how to create a transaction to make a Avail data submission
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

	// Instantiate the API (locally)
	api, err := gsrpc.NewSubstrateAPI(config.ApiURL)
	// api, err := gsrpc.NewSubstrateAPI(config.Default().RPCURL)
	if err != nil {
		panic(err)
	}

	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		panic(err)
	}

	// Set data and appID according to need
	size := 100
	if config.Size > 0 {
		size = config.Size
	}

	sub_data, _ := extrinsics.RandToken(size)
	fmt.Println("Submitting data ...")
	appID := 0

	//if app id is greater than 0 then it must be created before submitting data
	if config.AppID != 0 {
		appID = config.AppID
	}

	c, err := types.NewCall(meta, "DataAvailability.submit_data", types.NewBytes([]byte(sub_data)))
	if err != nil {
		panic(err)
	}

	// Create the extrinsic
	ext := types.NewExtrinsic(c)

	genesisHash, err := api.RPC.Chain.GetBlockHash(0)
	if err != nil {
		panic(err)
	}

	rv, err := api.RPC.State.GetRuntimeVersionLatest()
	if err != nil {
		panic(err)
	}

	keyringPair, err := signature.KeyringPairFromSecret(config.Seed, 42)
	if err != nil {
		panic(err)
	}

	key, err := types.CreateStorageKey(meta, "System", "Account", keyringPair.PublicKey)
	if err != nil {
		panic(err)
	}

	var accountInfo types.AccountInfo
	ok, err := api.RPC.State.GetStorageLatest(key, &accountInfo)
	if err != nil || !ok {
		panic(err)
	}

	nonce := uint32(accountInfo.Nonce)
	o := types.SignatureOptions{
		BlockHash:          genesisHash,
		Era:                types.ExtrinsicEra{IsMortalEra: false},
		GenesisHash:        genesisHash,
		Nonce:              types.NewUCompactFromUInt(uint64(nonce)),
		SpecVersion:        rv.SpecVersion,
		Tip:                types.NewUCompactFromUInt(100),
		AppID:              types.NewUCompactFromUInt(uint64(appID)),
		TransactionVersion: rv.TransactionVersion,
	}

	// Sign the transaction using Alice's default account
	err = ext.Sign(keyringPair, o)
	if err != nil {
		panic(err)
	}

	// Send the extrinsic
	sub, err := api.RPC.Author.SubmitAndWatchExtrinsic(ext)
	if err != nil {
		panic(err)
	}

	fmt.Printf("Data submitted by Alice: %v against appID %v\n", sub_data, appID)

	defer sub.Unsubscribe()
	timeout := time.After(100 * time.Second)
	for {
		select {
		case status := <-sub.Chan():
			if status.IsInBlock {
				fmt.Printf("Txn inside block %v\n", status.AsInBlock.Hex())
				hash := status.AsInBlock
				get(hash, api, sub_data)
			} else if status.IsFinalized {
				fmt.Printf("Txn inside finalized block\n")
				return
			}
		case <-timeout:
			fmt.Printf("timeout of 100 seconds reached without getting finalized status for extrinsic")
			return
		}
	}
}

func get(hash types.Hash, api *gsrpc.SubstrateAPI, data string) {

	block, err := api.RPC.Chain.GetBlock(hash)
	if err != nil {
		panic(err)
	}
	for _, ext := range block.Block.Extrinsics {
		// fmt.Println("pritingh appid test", ext.Signature.AppID)
		appId := ext.Signature.AppID
		value := appId.Int64()
		fmt.Printf("\ntype of AppId: %T", value)
		fmt.Println("AppId:", value)
		// these values below are specific indexes only for datasubmission, differs with each extrinsics
		if ext.Method.CallIndex.SectionIndex == 29 && ext.Method.CallIndex.MethodIndex == 1 {
			arg := ext.Method.Args
			str := string(arg)
			slice := str[2:]
			// fmt.Println("string value", slice)
			fmt.Println("data", data)
			if slice == data {
				fmt.Println("Data found in block")
			}
		}
	}

}
