package main

import (
	"avail-gsrpc-examples/internal/config"
	// "avail-gsrpc-examples/internal/extrinsics"
	"flag"
	"fmt"
	"log"
	"math/big"
	"os"
	"time"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

func balance(api *gsrpc.SubstrateAPI, Dest signature.KeyringPair) (types.U128, error) {

	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		panic(err)
	}

	key, err := types.CreateStorageKey(meta, "System", "Account", Dest.PublicKey, nil)
	if err != nil {
		return types.U128{}, err
	}

	var accountInfo types.AccountInfo
	ok, err := api.RPC.State.GetStorageLatest(key, &accountInfo)
	if err != nil || !ok {
		return types.U128{}, err
	}
	return accountInfo.Data.Free, nil
}

func transfer(api *gsrpc.SubstrateAPI, Seed string, Dest string, amount *big.Int) {

	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		panic(err)
	}
	// Create a call, transferring 12345 units to Bob

	keyringDest, err := signature.KeyringPairFromSecret(Dest, 42)
	if err != nil {
		panic(err)
	}

	addr, _ := types.NewMultiAddressFromAccountID(keyringDest.PublicKey)

	c, err := types.NewCall(meta, "Balances.transfer", addr, types.NewUCompact(amount))
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
			// if status.IsInBlock{
			// 	fmt.Printf("Txn inside block %v\n", status.AsInBlock.Hex())
			// }
			if status.IsFinalized {
				fmt.Printf("Txn finalized %v\n", status.AsFinalized.Hex())
				balance(api, keyringDest)
			}

			// default:
			// 	if status.IsDropped || status.IsInvalid {
			// 		fmt.Printf("unexpected extrinsic status from Avail: %#v", status)
			// 	}

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
	keyringDest, err := signature.KeyringPairFromSecret(config.Dest, 42)
	if err != nil {
		panic(err)
	}

	x, err := balance(api, keyringDest)
	if err != nil {
		panic(err)
	}
	fmt.Println(x)
	balance_amount := big.NewInt(300_000_000_000_000_000)

	transfer(api, config.Seed, config.Dest, balance_amount)

}
