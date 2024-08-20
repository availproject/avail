package main

import (
	config "avail-go-sdk-examples/internal/config"

	"flag"
	"fmt"
	"log"
	"math/big"
	"os"

	"avail-go-sdk/extrinsic"
	"avail-go-sdk/rpc"
	"avail-go-sdk/sdk"

	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types/codec"
)

// submitData creates a transaction and makes a Avail data submission
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
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		fmt.Printf("cannot create api:%v", err)
	}
	keyringPair, err := signature.KeyringPairFromSecret(config.Seed, 42)
	if err != nil {
		fmt.Printf("cannot create LeyPair:%v", err)
	}

	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		fmt.Printf("cannot get metadata:%v", err)
	}
	// sessionKeys := config.SessionKeys
	// keys := deconstructSessionKeys(sessionKeys)
	bondAmount := new(big.Int)
	bondAmount.SetString("1000000000000000000000", 10) // Set bondAmount to 1000000000000000000000

	// Convert big.Int to types.UCompact
	bondAmountUCompact := types.NewUCompact(bondAmount)

	c, err := types.NewCall(meta, "Staking.bond", bondAmountUCompact, types.NewU8(0)) // "Staked" is usually represented as a constant in the API, here represented as 1 for demonstration
	if err != nil {
		fmt.Printf("cannot create new call:%v", err)
	}
	// Create the extrinsic
	ext := extrinsic.NewExtrinsic(c)
	// proof := types.NewU8(0)

	// d, err := types.NewCall(meta, "Session.set_keys", keys, proof) // "Staked" is usually represented as a constant in the API, here represented as 1 for demonstration
	// if err != nil {
	// 	fmt.Printf("cannot create new call:%v", err)
	// }
	// // Create the extrinsic
	// ext := types.NewExtrinsic(d)

	// batchCall, err := types.NewCall(meta, "Utility.batch_all", []types.Call{c, d})
	// if err != nil {
	// 	panic(err)
	// }
	genesisHash, err := api.RPC.Chain.GetBlockHash(0)
	if err != nil {
		fmt.Printf("cannot get block hash:%v", err)
	}

	rv, err := api.RPC.State.GetRuntimeVersionLatest()
	if err != nil {
		fmt.Printf("cannot get runtime version:%v", err)
	}

	key, err := types.CreateStorageKey(meta, "System", "Account", keyringPair.PublicKey)
	if err != nil {
		fmt.Printf("cannot create storage key:%v", err)
	}

	var accountInfo types.AccountInfo
	ok, err := api.RPC.State.GetStorageLatest(key, &accountInfo)
	if err != nil || !ok {
		fmt.Printf("cannot get latest storage:%v", err)
	}

	nonce := uint32(accountInfo.Nonce)
	o := extrinsic.SignatureOptions{
		BlockHash:          genesisHash,
		Era:                extrinsic.ExtrinsicEra{IsMortalEra: false},
		GenesisHash:        genesisHash,
		Nonce:              types.NewUCompactFromUInt(uint64(nonce)),
		SpecVersion:        rv.SpecVersion,
		Tip:                types.NewUCompactFromUInt(0),
		AppID:              types.NewUCompactFromUInt(uint64(0)),
		TransactionVersion: rv.TransactionVersion,
	}

	// Sign the transaction using Alice's default account
	err = ext.Sign(keyringPair, o)
	if err != nil {
		fmt.Printf("cannot sign:%v", err)
	}
	// Send the extrinsic
	hash, err := rpc.SubmitExtrinsic(ext, api.Client)
	if err != nil {
		fmt.Printf("cannot submit extrinsic:%v", err)
	}

	fmt.Println(codec.EncodeToHex(hash))

}

type SessionKeys struct {
	Babe               string
	Grandpa            string
	ImOnline           string
	AuthorityDiscovery string
}

type ValidatorPreference struct {
	Commission string
	Block      bool
}

func defineValidatorPreference() ValidatorPreference {
	// "5" means 5 percent.
	commission := "5" + "0000000"

	// For some reason 0 commission is not defined as "0" but as "1".
	if commission == "00000000" {
		commission = "1"
	}

	// Returning the ValidatorPreference struct.
	return ValidatorPreference{
		Commission: commission,
		Block:      false,
	}
}

func deconstructSessionKeys(sessionKeys string) SessionKeys {
	// Removing the "0x" prefix from the session keys
	keys := sessionKeys[2:]

	// Splitting the keys into four parts
	babeKey := "0x" + keys[0:64]
	grandpaKey := "0x" + keys[64:128]
	imonlineKey := "0x" + keys[128:192]
	authorityDiscoveryKey := "0x" + keys[192:256]

	// Returning the keys as a SessionKeys struct
	return SessionKeys{
		Babe:               babeKey,
		Grandpa:            grandpaKey,
		ImOnline:           imonlineKey,
		AuthorityDiscovery: authorityDiscoveryKey,
	}
}
