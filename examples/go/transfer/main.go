package main

import (
	"avail-gsrpc-examples/internal/config"
	"flag"
	"fmt"
	"log"
	"math"
	"math/big"
	"os"
	"strconv"
	"strings"
	"time"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	. "github.com/centrifuge/go-substrate-rpc-client/v4/types/codec"

	"github.com/vedhavyas/go-subkey"
)

type InclusionFee struct {
	BaseFee           BigInt
	LenFee            BigInt
	AdjustedWeightFee BigInt
}

type BigInt struct {
	*big.Int
}

// UnmarshalJSON defines custom unmarshalling for BigInt.
func (bi *BigInt) UnmarshalJSON(data []byte) error {
	// Unquote string (since JSON numbers are sent as strings)
	str, err := strconv.Unquote(string(data))
	if err != nil {
		return err
	}

	// Remove 0x prefix if present and ensure non-empty
	str = strings.TrimPrefix(str, "0x")
	if str == "" {
		return fmt.Errorf("empty string")
	}

	// Parse the string
	bi.Int = new(big.Int)
	_, success := bi.Int.SetString(str, 16) // parse in base 16
	if !success {
		return fmt.Errorf("invalid hex string")
	}
	return nil
}

// Corresponding to Rust's FeeDetails
type FeeDetails struct {
	InclusionFee *InclusionFee
}

func transfer(api *gsrpc.SubstrateAPI, senderSeed string, receiver string, amount uint64) error {

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
				h := status.AsInBlock
				block, err := api.RPC.Chain.GetBlock(h)
				if err != nil {
					fmt.Printf("err occuerd")
				}
				var enc string
				exts := (block.Block.Extrinsics)
				// fmt.Print(ext)
				for i, j := range exts {
					if j.IsSigned() && j.Method.CallIndex.SectionIndex == 6 {
						enc, _ = EncodeToHex(j)

						fmt.Printf("\n the Transfer extrinsic Index is %d", i+1)
						signer := j.Signature.Signer
						fmt.Printf("\nfrom address hex %x", signer.AsID)
						address := fmt.Sprintf("\n%x", j.Method.Args[1:])
						fmt.Println("to address hex:", address)
					}
				}

				key, err := types.CreateStorageKey(meta, "System", "Events", nil, nil)
				if err != nil {
					log.Fatalf("Failed to create storage key: %v", err)
				}
				rawEvents, err := api.RPC.State.GetStorageRaw(key, h)
				if err != nil {
					log.Fatalf("Failed to fetch events: %v", err)
				}
				events := types.EventRecords{}
				err = types.EventRecordsRaw(*rawEvents).DecodeEventRecords(meta, &events)
				if err != nil {
					log.Fatalf("Failed to decode events: %v", err)
				}

				if rawEvents != nil && len(*rawEvents) > 0 {
					err = types.EventRecordsRaw(*rawEvents).DecodeEventRecords(meta, &events)
					if err != nil {
						log.Fatalf("Failed to decode events: %v", err)
					}

					for _, e := range events.Balances_Transfer {

						vals := convInt(fmt.Sprintf("%v", e.Value))
						fmt.Printf("Transfer event: %v\n", vals)

					}
					f := events.TransactionPayment_TransactionFeePaid
					for _, i := range f {
						fee := convInt(i.ActualFee.String())
						fmt.Printf("Fee Paid %v", fee)
					}

				} else {
					fmt.Println("No events found in the block")
				}
				var inclusionFee InclusionFee = InclusionFee{
					BaseFee:           BigInt{big.NewInt(0)},
					LenFee:            BigInt{big.NewInt(0)},
					AdjustedWeightFee: BigInt{big.NewInt(0)},
				}

				var feeDetails FeeDetails = FeeDetails{
					InclusionFee: &inclusionFee,
				}

				err = api.Client.Call(&feeDetails, "payment_queryFeeDetails", enc, h)
				if err != nil {
					panic(fmt.Sprintf("%v\n", err))
				}

				fmt.Println("Formatted Inclusion Fee:")
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
func formatFee(fee BigInt, isLenFee bool) string {
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
