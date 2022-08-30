package main

import (
	"avail-gsrpc-examples/internal/config"
	"avail-gsrpc-examples/internal/extrinsics"
	"crypto/rand"
	"flag"
	"log"
	"os"
	"sync/atomic"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"go.uber.org/ratelimit"
)

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
	log.Println("gsrpc connected to Substrate API...")

	sub, err := api.RPC.Chain.SubscribeNewHeads()
	if err != nil {
		panic(err)
	}
	defer sub.Unsubscribe()
	log.Println("Subscribed to new headers...")

	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		panic(err)
	}

	keyringPair, err := signature.KeyringPairFromSecret(config.Seed, 42)
	if err != nil {
		panic(err)
	}

	// if testing locally with Alice account, use signature.TestKeyringPairAlice.PublicKey as last param
	// mneumonic for local Alice account: `bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice`
	// key, err := types.CreateStorageKey(meta, "System", "Account", signature.TestKeyringPairAlice.PublicKey)
	// if err != nil {
	// 	panic(err)
	// }

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
	appID := 0

	//if app id is greater than 0 then it must be created before submitting data
	if config.AppID != 0 {
		appID = config.AppID
	}

	rl := ratelimit.New(10) // per second
	for {
		rl.Take()
		go func() {
			data := make([]byte, config.Size)
			rand.Read(data)
			submittedHash, err := extrinsics.SubmitData(api, string(data), config.Seed, appID, (atomic.AddUint32(&nonce, 1)))
			// atomic.AddUint32(&nonce, 1)
			if err != nil {
				panic(err)
			}
			log.Printf("Extrinsic submitted with hash: %s", submittedHash)
		}()
	}

}
