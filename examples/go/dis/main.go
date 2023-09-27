package main

import (
	"bufio"
	"flag"
	"fmt"
	"log"
	"math/big"
	"os"
	"time"

	"avail-gsrpc-examples/internal/config"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	. "github.com/centrifuge/go-substrate-rpc-client/v4/types/codec"
)

// The following example shows how submit data blob and track transaction status
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
		panic(fmt.Sprintf("cannot create api:%v", err))
	}

	meta, err := api.RPC.State.GetMetadataLatest()
	f, err := os.OpenFile("output.txt", os.O_WRONLY|os.O_CREATE, 0666)
	if err != nil {
		panic(err)
	}
	defer f.Close()
	w := bufio.NewWriter(f)

	fmt.Fprintf(w, "%+v", meta)
	w.Flush()

	if err != nil {
		panic(fmt.Sprintf("cannot get metadata:%v", err))
	}

	// if app id is greater than 0 then it must be created before submitting data
	app_id := types.NewUCompact(big.NewInt(0))

	// if config.AppID != 0 {
	// 	appID = config.AppID
	// }
	hash, err := types.NewHashFromHexString("0xa5ca85383b5d184792a240712ad739a91d989177f3cea8fb754ae8c4d49c7552")
	hash1, err := types.NewHashFromHexString("0x000000000000000000000000aAB16A9fb03D5845193e87F596Fa610FCE6054F0")

	header, err := api.RPC.Chain.GetHeader(types.Hash(hash))
	if err != nil {
		panic(fmt.Sprintf("cannot print to heaedr:%v", err))
	}
	fmt.Printf("header: %v\n", header)
	encoded, err := EncodeToHex(header)
	if err != nil {
		panic(fmt.Sprintf("cannot encode to hex:%v", err))
	}
	fmt.Printf("encoded: %v\n", encoded)
	newCall, err := types.NewCall(meta, "NomadDABridge.try_dispatch_data_root", types.NewUCompactFromUInt(1000), hash1, header)
	if err != nil {
		panic(fmt.Sprintf("cannot create new call:%v", err))
	}

	// Create the extrinsic
	ext := types.NewExtrinsic(newCall)

	genesisHash, err := api.RPC.Chain.GetBlockHash(0)
	if err != nil {
		panic(fmt.Sprintf("cannot get block hash:%v", err))
	}

	rv, err := api.RPC.State.GetRuntimeVersionLatest()
	if err != nil {
		panic(fmt.Sprintf("cannot get latest runtime version:%v", err))
	}

	keyringPair, err := signature.KeyringPairFromSecret(config.Seed, 42)
	if err != nil {
		panic(fmt.Sprintf("cannot create KeyPair:%v", err))
	}

	key, err := types.CreateStorageKey(meta, "System", "Account", keyringPair.PublicKey)
	if err != nil {
		panic(fmt.Sprintf("cannot create storage key:%v", err))
	}

	var accountInfo types.AccountInfo
	ok, err := api.RPC.State.GetStorageLatest(key, &accountInfo)
	if err != nil || !ok {
		panic(fmt.Sprintf("cannot get latest storage:%v", err))
	}

	nonce := uint32(accountInfo.Nonce)
	options := types.SignatureOptions{
		BlockHash:          genesisHash,
		Era:                types.ExtrinsicEra{IsMortalEra: false},
		GenesisHash:        genesisHash,
		Nonce:              types.NewUCompactFromUInt(uint64(nonce)),
		SpecVersion:        rv.SpecVersion,
		Tip:                types.NewUCompactFromUInt(500),
		AppID:              app_id,
		TransactionVersion: rv.TransactionVersion,
	}
	err = ext.Sign(keyringPair, options)
	if err != nil {
		fmt.Printf("cannot sign:%v", err)
	}
	t, err := api.RPC.Author.SubmitAndWatchExtrinsic(ext)

	if err != nil {
		fmt.Printf("cannot dispatch data root:%v", err)
	}

	defer t.Unsubscribe()
	timeout := time.After(100 * time.Second)
	for {
		select {
		case status := <-t.Chan():
			if status.IsInvalid {
				fmt.Printf("Txn is invalid\n")
			} else if status.IsReady == true {
				fmt.Printf("Txn is ready to be included in block\n")
			} else if status.IsBroadcast != false {
				fmt.Printf("=========\n")
			} else if status.IsInBlock {
				fmt.Printf("Txn inside block %v\n", status.AsInBlock.Hex())
				// log.Printf("✅ Data root dispatched by sequencer with AppID %v sent with hash %#v\n", app_id, status.AsFinalized)
			} else if status.IsFinalized {
				fmt.Printf("Txn inside finalized block\n")
				return
			}
		case <-timeout:
			fmt.Printf("timeout of 100 seconds reached without getting finalized status for extrinsic")
			break
		}
	}

}
