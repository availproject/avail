package main

import (
	config "avail-go-sdk-examples/internal/config"
	"avail-go-sdk/extrinsic"
	"avail-go-sdk/rpc"
	"avail-go-sdk/sdk"

	"flag"
	"fmt"
	"log"
	"math/big"
	"os"

	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
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

	bondAmount := new(big.Int)
	bondAmount.SetString("1000000000000000000000", 10) // Set bondAmount to 1000000000000000000000

	// Convert big.Int to types.UCompact
	bondAmountUCompact := types.NewUCompact(bondAmount)

	c, err := types.NewCall(meta, "Staking.bond", bondAmountUCompact, types.NewU8(0)) // "Staked" is usually represented as a constant in the API, here represented as 1 for demonstration
	if err != nil {
		fmt.Printf("cannot create new call:%v", err)
	}
	//Alice Stash Key
	targetHex := "0xbe5ddb1579b72e84524fc29e78609e3caf42e85aa118ebfe0b0ad404b5bdd25f"

	// Convert the hex string to a MultiAddress
	targetMultiAddress, err := types.NewMultiAddressFromHexAccountID(targetHex)
	if err != nil {
		fmt.Printf("Error converting hex string to MultiAddress: %v", err)
		// handle the error appropriately
	}

	//Uncomment following to use address
	// _, pubkeyBytes, _ := subkey.SS58Decode("5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY")
	// hexString := subkey.EncodeHex(pubkeyBytes)

	// dest, err := types.NewMultiAddressFromHexAccountID(hexString)
	// if err != nil {
	// 	fmt.Printf("cannot create new call:%v", err)
	// }
	d, err := types.NewCall(meta, "Staking.nominate", []types.MultiAddress{targetMultiAddress})
	if err != nil {
		fmt.Printf("cannot create new call:%v", err)
	}

	batchCall, err := types.NewCall(meta, "Utility.batch_all", []types.Call{c, d})
	if err != nil {
		panic(err)
	}
	ext := extrinsic.NewExtrinsic(batchCall)
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

	o := extrinsic.SignatureOptions{
		BlockHash:          genesisHash,
		Era:                extrinsic.ExtrinsicEra{IsMortalEra: false},
		GenesisHash:        genesisHash,
		Nonce:              types.NewUCompactFromUInt(uint64(accountInfo.Nonce)),
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

	fmt.Println(hash)

}
