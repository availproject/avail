package main

import (
	"avail-go-sdk-examples/internal/config"
	"encoding/hex"
	"flag"
	"fmt"
	"log"
	"math"
	"math/big"
	"os"
	"strings"
	"time"

	"avail-go-sdk/src/extrinsic"
	"avail-go-sdk/src/rpc"
	"avail-go-sdk/src/sdk"

	"github.com/centrifuge/go-substrate-rpc-client/v4/registry"
	retriever "github.com/centrifuge/go-substrate-rpc-client/v4/registry/retriever"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry/state"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"

	codec "github.com/centrifuge/go-substrate-rpc-client/v4/types/codec"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"golang.org/x/crypto/blake2b"

	"github.com/vedhavyas/go-subkey"
)

type InclusionFee struct {
	BaseFee           sdk.BigInt
	LenFee            sdk.BigInt
	AdjustedWeightFee sdk.BigInt
}

// type BigInt struct {
// 	*big.Int
// }

// // UnmarshalJSON defines custom unmarshalling for BigInt.
// func (bi *BigInt) UnmarshalJSON(data []byte) error {
// 	// Unquote string (since JSON numbers are sent as strings)
// 	str, err := strconv.Unquote(string(data))
// 	if err != nil {
// 		return err
// 	}

// 	// Remove 0x prefix if present and ensure non-empty
// 	str = strings.TrimPrefix(str, "0x")
// 	if str == "" {
// 		return fmt.Errorf("empty string")
// 	}

// 	// Parse the string
// 	bi.Int = new(big.Int)
// 	_, success := bi.Int.SetString(str, 16) // parse in base 16
// 	if !success {
// 		return fmt.Errorf("invalid hex string")
// 	}
// 	return nil
// }

// Corresponding to Rust's FeeDetails
type FeeDetails struct {
	InclusionFee *InclusionFee
}

func transfer(api *sdk.SubstrateAPI, senderSeed string, receiver string, amount uint64) error {

	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		return fmt.Errorf("cannot get metadata:%w", err)
	}

	_, pubkeyBytes, _ := subkey.SS58Decode(receiver)
	address := subkey.EncodeHex(pubkeyBytes)

	dest, err := types.NewMultiAddressFromHexAccountID(address)
	if err != nil {
		return fmt.Errorf("cannot create address from given hex:%w", err)
	}

	balanceCall, err := types.NewCall(meta, "Balances.transfer_keep_alive", dest, types.NewUCompactFromUInt(amount))
	if err != nil {
		return fmt.Errorf("cannot create balance call:%w", err)
	}

	// Create the extrinsic
	ext := extrinsic.NewExtrinsic(balanceCall)

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
	key, err := types.CreateStorageKey(meta, "System", "Account", keyringPair.PublicKey, nil)
	if err != nil {
		return fmt.Errorf("cannot create storage key:%w", err)
	}

	var accountInfo types.AccountInfo
	ok, err := api.RPC.State.GetStorageLatest(key, &accountInfo)
	if err != nil || !ok {
		return fmt.Errorf("cannot get latest storage:%w", err)
	}

	nonce := uint32(accountInfo.Nonce)
	options := extrinsic.SignatureOptions{
		BlockHash:          genesisHash,
		Era:                extrinsic.ExtrinsicEra{IsMortalEra: false},
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

	enc, _ := codec.EncodeToHex(ext)

	cleanedHexString := strings.TrimPrefix(enc, "0x")
	bytes, err := hex.DecodeString(cleanedHexString)
	if err != nil {
		log.Fatal(err)
	}
	hash := blake2b.Sum256(bytes)
	ext_z := hexutil.Encode(hash[:])
	fmt.Printf("ext hash is %v", ext_z)
	// Send the extrinsic
	sub, err := rpc.SubmitAndWatchExtrinsic(ext, api.Client)
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
				h := status.AsInBlock
				block, err := rpc.GetAvailBlock(h, api.Client)
				if err != nil {
					fmt.Printf("err occuerd")
				}
				var enc string
				exts := (block.Block.Extrinsics)
				// fmt.Print(ext)
				for i, j := range exts {
					if j.IsSigned() && j.Method.CallIndex.SectionIndex == 6 {
						enc, _ = codec.EncodeToHex(j)
						cleanedHexString := strings.TrimPrefix(enc, "0x")
						bytes, err := hex.DecodeString(cleanedHexString)
						if err != nil {
							log.Fatal(err)
						}
						hash := blake2b.Sum256(bytes)
						z := hexutil.Encode(hash[:])
						fmt.Printf("ext hash is %v", z)
						if z == ext_z {
							fmt.Printf("\n the Transfer extrinsic Index is %d", i+1)
							signer := j.Signature.Signer
							fmt.Printf("\nfrom address hex %x\n", signer.AsID)
							address := fmt.Sprintf("\n%x", j.Method.Args[1:])
							fmt.Printf("to address hex: %s \n", address)
						}
					}
				}

				// key, err := types.CreateStorageKey(meta, "System", "Events", nil, nil)
				// if err != nil {
				// 	log.Fatalf("Failed to create storage key: %v", err)
				// }
				// rawEvents, err := api.RPC.State.GetStorageRaw(key, h)
				// if err != nil {
				// 	log.Fatalf("Failed to fetch events: %v", err)
				// }
				// events := types.EventRecords{}
				// err = types.EventRecordsRaw(*rawEvents).DecodeEventRecords(meta, &events)
				// if err != nil {
				// 	log.Fatalf("Failed to decode events: %v", err)
				// }

				// if rawEvents != nil && len(*rawEvents) > 0 {
				// 	err = types.EventRecordsRaw(*rawEvents).DecodeEventRecords(meta, &events)
				// 	if err != nil {
				// 		log.Fatalf("Failed to decode events: %v", err)
				// 	}

				// 	for _, e := range events.Balances_Transfer {

				// 		vals := convInt(fmt.Sprintf("%v", e.Value))
				// 		fmt.Printf("Transfer event: %v\n", vals)

				// 	}
				// 	f := events.TransactionPayment_TransactionFeePaid
				// 	for _, i := range f {
				// 		fee := convInt(i.ActualFee.String())
				// 		fmt.Printf("Fee Paid %v", fee)
				// 	}

				// } else {
				// 	fmt.Println("No events found in the block")
				// }

				//Updating the event from generic decoding to registry
				retriever, err := retriever.NewDefaultEventRetriever(state.NewEventProvider(api.RPC.State), api.RPC.State)

				if err != nil {
					fmt.Printf("Couldn't create event retriever: %s", err)
				}
				events, err := retriever.GetEvents(h)

				if err != nil {
					log.Printf("Couldn't retrieve events")
				}
				for _, event := range events {
					if event.Name == "Balances.Transfer" {
						from, err := registry.ProcessDecodedFieldValue[*types.AccountID](
							event.Fields,
							func(fieldIndex int, field *registry.DecodedField) bool {

								return field.Name == "sp_core.crypto.AccountId32.from"
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

						to, err := registry.ProcessDecodedFieldValue[*types.AccountID](
							event.Fields,

							func(fieldIndex int, field *registry.DecodedField) bool {

								return field.Name == "sp_core.crypto.AccountId32.to"
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
						if err != nil {
							fmt.Printf("TO parsing err: %s\n", err.Error())
						}
						fmt.Printf("To address read from event: %s \n", to.ToHexString())
						amount, err := registry.GetDecodedFieldAsType[types.U128](
							event.Fields,
							func(fieldIndex int, field *registry.DecodedField) bool {
								return fieldIndex == 2
							},
						)
						if err != nil {
							fmt.Printf("Amount parsing err: %s\n", err.Error())
						}
						fmt.Printf("Amount transferred : %s \n", convInt(amount.String()))
						if err != nil {
							fmt.Printf("Balances.Deposit.Who: %s\n", err.Error())
						}
					}
					if event.Name == "TransactionPayment.TransactionFeePaid" {

						amount, err := registry.GetDecodedFieldAsType[types.U128](
							event.Fields,
							func(fieldIndex int, field *registry.DecodedField) bool {
								return fieldIndex == 1
							},
						)
						if err != nil {
							fmt.Printf("Amount parsing err: %s\n", err.Error())
						}
						fmt.Printf("Actual Fee from TransactionPayment.TransactionFeePaid event: %s \n", convInt(amount.String()))
						if err != nil {
							fmt.Printf("Balances.Deposit.Who: %s\n", err.Error())
						}
					}
				}
				var inclusionFee InclusionFee = InclusionFee{
					BaseFee:           sdk.BigInt{big.NewInt(0)},
					LenFee:            sdk.BigInt{big.NewInt(0)},
					AdjustedWeightFee: sdk.BigInt{big.NewInt(0)},
				}

				var feeDetails FeeDetails = FeeDetails{
					InclusionFee: &inclusionFee,
				}

				err = api.Client.Call(&feeDetails, "payment_queryFeeDetails", enc, h)
				if err != nil {
					panic(fmt.Sprintf("%v\n", err))
				}

				fmt.Printf("Base Fee: %s\n", formatFee(inclusionFee.BaseFee, false))
				fmt.Printf("Length Fee: %s\n", formatFee(inclusionFee.LenFee, true))
				fmt.Printf("Adjusted Weight Fee: %s\n", formatFee(inclusionFee.AdjustedWeightFee, false))

				return nil
			}

		case <-timeout:
			fmt.Printf("timeout of 100 seconds reached without getting finalized status for extrinsic")
			return fmt.Errorf("timeout")
		}
	}

}
func formatFee(fee sdk.BigInt, isLenFee bool) string {
	var value float64
	var unit string

	feeFloat := new(big.Float).SetInt(fee.Int)

	if isLenFee {

		feeFloat.Quo(feeFloat, big.NewFloat(1e12))
		unit = "µAVAIL"
	} else {

		feeFloat.Quo(feeFloat, big.NewFloat(1e15))
		unit = "mAVAIL"
	}

	value, _ = feeFloat.Float64()

	return fmt.Sprintf("%.4f %s", value, unit)
}
func convInt(val string) string {
	bigIntValue := new(big.Int)
	bigIntValue.SetString(val, 10)

	divisor := new(big.Int)
	divisor.SetString("1000000000000000000", 10)

	bigFloatValue := new(big.Float).SetInt(bigIntValue)
	divisorFloat := new(big.Float).SetInt(divisor)
	result := new(big.Float).Quo(bigFloatValue, divisorFloat)

	x := (result.Text('f', 18))
	return x
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

	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		panic(fmt.Sprintf("cannot create api client:%v", err))
	}

	fmt.Printf("Sending amount %d....", config.Amount)
	err = transfer(api, config.Seed, config.Dest, uint64(math.Pow(10, 18))*config.Amount)
	if err != nil {
		panic(fmt.Sprintf("cannot create transfer:%v", err))
	}
}
