package main

import (
	"avail-gsrpc-examples/internal/config"
	"flag"
	"fmt"
	"log"
	"os"
	"time"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/vedhavyas/go-subkey/v2"
)

func transfer(api *gsrpc.SubstrateAPI, Seed string, Dest string, amount uint64) {

	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		panic(err)
	}

	_, pubkeyBytes, _ := subkey.SS58Decode(Dest)
	hexString := subkey.EncodeHex(pubkeyBytes)

	dest, err := types.NewMultiAddressFromHexAccountID(hexString)
	if err != nil {
		panic(err)
	}

	c, err := types.NewCall(meta, "Balances.transfer", dest, types.NewUCompactFromUInt(amount))
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

	keyringPair, err := signature.KeyringPairFromSecret(Seed, 42)
	if err != nil {
		panic(err)
	}

	key, err := types.CreateStorageKey(meta, "System", "Account", signature.TestKeyringPairAlice.PublicKey, nil)
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
		Tip:                types.NewUCompactFromUInt(0),
		AppID:              types.NewUCompactFromUInt(uint64(0)),
		TransactionVersion: rv.TransactionVersion,
	}

	err = ext.Sign(keyringPair, o)
	if err != nil {
		panic(err)
	}

	// Send the extrinsic
	sub, err := api.RPC.Author.SubmitAndWatchExtrinsic(ext)
	if err != nil {
		panic(err)
	}

	defer sub.Unsubscribe()
	timeout := time.After(100 * time.Second)
	for {
		select {
		case status := <-sub.Chan():
			// NOTE: See first line of this function for supported extrinsic status expectations.
			if status.IsInBlock {
				fmt.Printf("\nTxn inside block %v\n", status.AsInBlock.Hex())
			}
			if status.IsFinalized {
				fmt.Printf("\nTxn finalized %v\n", status.AsFinalized.Hex())
				return
			}
			if status.IsDropped || status.IsInvalid {
				fmt.Printf("unexpected extrinsic status from Avail: %#v", status)
			}

		case <-timeout:
			fmt.Printf("timeout of 100 seconds reached without getting finalized status for extrinsic")
			return
		}
	}

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
		panic(err)
	}

	api, err := gsrpc.NewSubstrateAPI(config.ApiURL)
	if err != nil {
		panic(err)
	}

	fmt.Printf("Sending amount %d....", config.Amount)

	transfer(api, config.Seed, config.Dest, config.Amount)

}
