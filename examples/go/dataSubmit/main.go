package main

import (
	"avail-gsrpc-examples/internal/config"
	"avail-gsrpc-examples/internal/extrinsics"
	"flag"
	"fmt"
	"log"
	"os"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

// submitData creates a transaction and makes a Avail data submission
func submitData(size int, ApiURL string, Seed string, AppID int) error {
	api, err := gsrpc.NewSubstrateAPI(ApiURL)
	if err != nil {
		return fmt.Errorf("cannot create api:%w", err)
	}

	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		return fmt.Errorf("cannot get metadata:%w", err)
	}

	// Set data and appID according to need
	data, _ := extrinsics.RandToken(size)
	appID := 0

	// if app id is greater than 0 then it must be created before submitting data
	if AppID != 0 {
		appID = AppID
	}

	c, err := types.NewCall(meta, "DataAvailability.submit_data", types.NewBytes([]byte(data)))
	if err != nil {
		return fmt.Errorf("cannot create new call:%w", err)
	}

	// Create the extrinsic
	ext := types.NewExtrinsic(c)

	genesisHash, err := api.RPC.Chain.GetBlockHash(0)
	if err != nil {
		return fmt.Errorf("cannot get block hash:%w", err)
	}

	rv, err := api.RPC.State.GetRuntimeVersionLatest()
	if err != nil {
		return fmt.Errorf("cannot get runtime version:%w", err)
	}

	keyringPair, err := signature.KeyringPairFromSecret(Seed, 42)
	if err != nil {
		return fmt.Errorf("cannot create LeyPair:%w", err)
	}

	key, err := types.CreateStorageKey(meta, "System", "Account", keyringPair.PublicKey)
	if err != nil {
		return fmt.Errorf("cannot create storage key:%w", err)
	}

	var accountInfo types.AccountInfo
	ok, err := api.RPC.State.GetStorageLatest(key, &accountInfo)
	if err != nil || !ok {
		return fmt.Errorf("cannot get latest storage:%w", err)
	}

	nonce := uint32(accountInfo.Nonce)
	o := types.SignatureOptions{
		BlockHash:          genesisHash,
		Era:                types.ExtrinsicEra{IsMortalEra: false},
		GenesisHash:        genesisHash,
		Nonce:              types.NewUCompactFromUInt(uint64(nonce)),
		SpecVersion:        rv.SpecVersion,
		Tip:                types.NewUCompactFromUInt(0),
		AppID:              types.NewUCompactFromUInt(uint64(appID)),
		TransactionVersion: rv.TransactionVersion,
	}

	// Sign the transaction using Alice's default account
	err = ext.Sign(keyringPair, o)
	if err != nil {
		return fmt.Errorf("cannot sign:%w", err)
	}

	// Send the extrinsic
	hash, err := api.RPC.Author.SubmitExtrinsic(ext)
	if err != nil {
		return fmt.Errorf("cannot submit extrinsic:%w", err)
	}
	fmt.Printf("Data submitted by Alice: %v against appID %v  sent with hash %#x\n", data, appID, hash)

	return nil
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

	size := 100
	if config.Size > 0 {
		size = config.Size
	}
	err = submitData(size, config.ApiURL, config.Seed, config.AppID)
	if err != nil {
		panic(fmt.Sprintf("cannot submit data:%v", err))
	}
}
