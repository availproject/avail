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
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry"
	. "github.com/centrifuge/go-substrate-rpc-client/v4/registry/retriever"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry/state"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
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
	if err != nil {
		panic(fmt.Sprintf("cannot get metadata:%v", err))
	}

	// Set data and appID according to need
	size := 100
	if config.Size > 0 {
		size = config.Size
	}

	subData, _ := extrinsics.RandToken(size)
	fmt.Println("Submitting data ...")
	appID := 0

	// if app id is greater than 0 then it must be created before submitting data
	if config.AppID != 0 {
		appID = config.AppID
	}

	newCall, err := types.NewCall(meta, "DataAvailability.submit_data", types.NewBytes([]byte(subData)))
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
		panic(fmt.Sprintf("cannot create storage key:%w", err))
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
		Tip:                types.NewUCompactFromUInt(100),
		AppID:              types.NewUCompactFromUInt(uint64(appID)),
		TransactionVersion: rv.TransactionVersion,
	}

	// Sign the transaction using Alice's default account
	err = ext.Sign(keyringPair, options)
	if err != nil {
		panic(fmt.Sprintf("cannot sign:%v", err))
	}

	// Send the extrinsic
	sub, err := api.RPC.Author.SubmitAndWatchExtrinsic(ext)
	if err != nil {
		panic(fmt.Sprintf("cannot submit extrinsic:%v", err))
	}

	fmt.Printf("Data submitted using APPID: %v \n", appID)

	defer sub.Unsubscribe()
	timeout := time.After(100 * time.Second)
	for {
		select {
		case status := <-sub.Chan():
			if status.IsInBlock {
				fmt.Printf("Txn inside block %v\n", status.AsInBlock.Hex())
			} else if status.IsFinalized {
				retriever, err := NewDefaultEventRetriever(state.NewEventProvider(api.RPC.State), api.RPC.State)

				if err != nil {
					fmt.Printf("Couldn't create event retriever: %s", err)
				}
				h := status.AsFinalized
				events, err := retriever.GetEvents(h)

				if err != nil {
					log.Printf("Couldn't retrieve events")
				}
				for _, event := range events {
					if event.Name == "DataAvailability.DataSubmitted" {
						from, _ := registry.ProcessDecodedFieldValue[*types.AccountID](
							event.Fields,
							func(fieldIndex int, field *registry.DecodedField) bool {

								return field.Name == "sp_core.crypto.AccountId32.who"
							},
							func(value any) (*types.AccountID, error) {
								fields, ok := value.(registry.DecodedFields)

								if !ok {
									return nil, fmt.Errorf("unexpected value: %v", value)
								}

								accByteSlice, err := registry.GetDecodedFieldAsSliceOfType[types.U8](fields, func(fieldIndex int, field *registry.DecodedField) bool {
									return fieldIndex == 0
								})

								if err != nil {
									return nil, err
								}

								var accBytes []byte

								for _, accByte := range accByteSlice {
									accBytes = append(accBytes, byte(accByte))
								}

								return types.NewAccountID(accBytes)
							},
						)
						a := from.ToHexString()

						// // add, _ := types.NewAddressFromHexAccountID(a)
						// fmt.Println(from)
						fmt.Printf("from address read from event: %s \n", a)

						dataHash, err := registry.ProcessDecodedFieldValue[*types.Hash](
							event.Fields,
							func(fieldIndex int, field *registry.DecodedField) bool {
								return fieldIndex == 1
							},
							func(value any) (*types.Hash, error) {
								fields, ok := value.(registry.DecodedFields)
								if !ok {
									return nil, fmt.Errorf("unexpected value: %v", value)
								}

								hashByteSlice, err := registry.GetDecodedFieldAsSliceOfType[types.U8](fields, func(fieldIndex int, field *registry.DecodedField) bool {
									return fieldIndex == 0
								})

								if err != nil {
									return nil, err
								}

								var hashBytes []byte
								for _, hashByte := range hashByteSlice {
									hashBytes = append(hashBytes, byte(hashByte))
								}

								hash := types.NewHash(hashBytes)
								return &hash, nil
							},
						)
						if err != nil {
							fmt.Printf("DataHash parsing err: %s\n", err.Error())
						} else if dataHash == nil {
							fmt.Println("DataHash is nil")
						} else {
							fmt.Printf("DataHash read from event: %s \n", dataHash.Hex())
						}
						fmt.Printf("DataHash read from event: %s \n", dataHash.Hex())

					}

				}
				fmt.Printf("Txn inside finalized block\n")
				hash := status.AsFinalized
				err = getData(hash, api, subData)
				if err != nil {
					panic(fmt.Sprintf("cannot get data:%v", err))
				}
				return
			}
		case <-timeout:
			fmt.Printf("timeout of 100 seconds reached without getting finalized status for extrinsic")
			return
		}
	}
}

// getData extracts data from the block and compares it
func getData(hash types.Hash, api *gsrpc.SubstrateAPI, data string) error {
	block, err := api.RPC.Chain.GetBlock(hash)
	if err != nil {
		return fmt.Errorf("cannot get block by hash:%w", err)
	}
	for _, ext := range block.Block.Extrinsics {

		// these values below are specific indexes only for data submission, differs with each extrinsic
		if ext.Method.CallIndex.SectionIndex == 29 && ext.Method.CallIndex.MethodIndex == 1 {
			arg := ext.Method.Args
			str := string(arg)
			slice := str[2:]

			fmt.Println("Data retrieved:", data)
			if slice == data {
				fmt.Println("Data found in block")
			}
		}
	}
	return nil
}
