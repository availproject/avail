package main

import (
	"avail-gsrpc-examples/internal/config"
	"flag"
	"fmt"
	"log"
	"math"
	"os"
	"time"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/vedhavyas/go-subkey/v2"
)

func transfer(api *gsrpc.SubstrateAPI, senderSeed string, receiver string, amount uint64) error {

	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		return fmt.Errorf("cannot get metadata:%w", err)
	}

	_, pubkeyBytes, _ := subkey.SS58Decode(receiver)
	hexString := subkey.EncodeHex(pubkeyBytes)

	dest, err := types.NewMultiAddressFromHexAccountID(hexString)
	if err != nil {
		return fmt.Errorf("cannot create address from given hex:%w", err)
	}

	balanceCall, err := types.NewCall(meta, "Balances.transfer", dest, types.NewUCompactFromUInt(amount))
	if err != nil {
		return fmt.Errorf("cannot create balance call:%w", err)
	}

	// Create the extrinsic
	ext := types.NewExtrinsic(balanceCall)

	genesisHash, err := api.RPC.Chain.GetBlockHash(0)
	if err != nil {
		return fmt.Errorf("cannot get block hash:%w", err)
	}

	rv, err := api.RPC.State.GetRuntimeVersionLatest()
	if err != nil {
		return fmt.Errorf("cannot runtime version:%w", err)
	}

	keyringPair, err := signature.KeyringPairFromSecret(senderSeed, 42)
	if err != nil {
		return fmt.Errorf("cannot create KeyPair:%w", err)
	}

	key, err := types.CreateStorageKey(meta, "System", "Account", signature.TestKeyringPairAlice.PublicKey, nil)
	if err != nil {
		return fmt.Errorf("cannot create storage key:%w", err)
	}

	var accountInfo types.AccountInfo
	ok, err := api.RPC.State.GetStorageLatest(key, &accountInfo)
	if err != nil || !ok {
		return fmt.Errorf("cannot get latest storage:%w", err)
	}

	nonce := uint32(accountInfo.Nonce)
	options := types.SignatureOptions{
		BlockHash:          genesisHash,
		Era:                types.ExtrinsicEra{IsMortalEra: false},
		GenesisHash:        genesisHash,
		Nonce:              types.NewUCompactFromUInt(uint64(nonce)),
		SpecVersion:        rv.SpecVersion,
		Tip:                types.NewUCompactFromUInt(0),
		AppID:              types.NewUCompactFromUInt(uint64(0)),
		TransactionVersion: rv.TransactionVersion,
	}

	err = ext.Sign(keyringPair, options)
	if err != nil {
		return fmt.Errorf("cannot sign:%w", err)
	}

	// Send the extrinsic
	sub, err := api.RPC.Author.SubmitAndWatchExtrinsic(ext)
	if err != nil {
		return fmt.Errorf("cannot submit extrinsic:%w", err)
	}

	defer sub.Unsubscribe()
	timeout := time.After(100 * time.Second)
	for {
		// get status of the submitted transaction
		select {
		case status := <-sub.Chan():
			if status.IsInBlock {
				fmt.Printf("\nTxn inside block %v\n", status.AsInBlock.Hex())
			}
			if status.IsFinalized {
				fmt.Printf("\nTxn finalized %v\n", status.AsFinalized.Hex())
				return nil
			}
			if status.IsDropped || status.IsInvalid {
				fmt.Printf("unexpected extrinsic status from Avail: %#v", status)
			}

		case <-timeout:
			fmt.Printf("timeout of 100 seconds reached without getting finalized status for extrinsic")
			return fmt.Errorf("timeout")
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
		panic(fmt.Sprintf("cannot get config:%v", err))
	}

	api, err := gsrpc.NewSubstrateAPI(config.ApiURL)
	if err != nil {
		panic(fmt.Sprintf("cannot create api client:%v", err))
	}

	fmt.Printf("Sending amount %d....", config.Amount)
	err = transfer(api, config.Seed, config.Dest, uint64(math.Pow(10, 18))*config.Amount)
	if err != nil {
		panic(fmt.Sprintf("cannot create transfer:%v", err))
	}
}
