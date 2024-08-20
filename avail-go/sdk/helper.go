package sdk

import (
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"math/big"
	"strings"
	"time"

	"avail-go-sdk/extrinsic"
	"avail-go-sdk/rpc"

	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/vedhavyas/go-subkey"
)

func convertMultiAddress(receiver string) (types.MultiAddress, error) {
	_, pubkeyBytes, _ := subkey.SS58Decode(receiver)
	address := subkey.EncodeHex(pubkeyBytes)

	dest, err := types.NewMultiAddressFromHexAccountID(address)
	if err != nil {
		_ = fmt.Errorf("cannot create address from given hex:%w", err)
		return types.MultiAddress{}, err
	}
	return dest, nil
}

type BigInt struct {
	*big.Int
}

// UnmarshalJSON defines custom unmarshalling for BigInt.
func (bi *BigInt) UnmarshalJSON(data []byte) error {
	// Unmarshal as raw JSON string
	var rawString string
	if err := json.Unmarshal(data, &rawString); err != nil {
		return err
	}

	// Remove 0x prefix if present
	str := strings.TrimPrefix(rawString, "0x")

	// Initialize bi.Int if it's nil
	if bi.Int == nil {
		bi.Int = new(big.Int)
	}

	// If the string is empty, set bi to zero
	if str == "" {
		bi.SetInt64(0)
		return nil
	}

	// Parse the string in base 16
	_, success := bi.SetString(str, 16)
	if !success {
		return fmt.Errorf("invalid hex string")
	}

	return nil
}

func formatBigInt(n BigInt) string {
	s := n.String() // Convert number to string
	var result strings.Builder
	count := 0

	for i := len(s) - 1; i >= 0; i-- {
		if count == 3 {
			result.WriteString(",")
			count = 0
		}
		result.WriteByte(s[i])
		count++
	}

	// Reverse the string to get the correct order
	reversed := result.String()
	var formatted strings.Builder
	for i := len(reversed) - 1; i >= 0; i-- {
		formatted.WriteByte(reversed[i])
	}

	return formatted.String()
}

func SubmitData(api SubstrateAPI, data string, seed string, appID int, finalizedBlockCh chan types.Hash) error {

	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		return err
	}

	c, err := types.NewCall(meta, "DataAvailability.submit_data", types.NewBytes([]byte(data)))
	if err != nil {
		return fmt.Errorf("error creating new call: %s", err)
	}

	// Create the extrinsic
	ext := extrinsic.NewExtrinsic(c)

	genesisHash, err := api.RPC.Chain.GetBlockHash(0)
	if err != nil {
		return fmt.Errorf("error getting genesis hash: %s", err)
	}

	rv, err := api.RPC.State.GetRuntimeVersionLatest()
	if err != nil {
		return fmt.Errorf("error retrieveing runtime version: %s", err)
	}

	keyringPair, err := signature.KeyringPairFromSecret(seed, 42)
	if err != nil {
		return fmt.Errorf("error creating keyring pair: %s", err)
	}

	key, err := types.CreateStorageKey(meta, "System", "Account", keyringPair.PublicKey)
	if err != nil {
		return fmt.Errorf("cannot create storage key:%w", err)
	}

	var accountInfo types.AccountInfo
	ok, err := api.RPC.State.GetStorageLatest(key, &accountInfo)
	if err != nil || !ok {
		return fmt.Errorf("cannot get latest storage:%v", err)
	}

	nonce := uint32(accountInfo.Nonce)
	options := extrinsic.SignatureOptions{
		BlockHash:          genesisHash,
		Era:                extrinsic.ExtrinsicEra{IsMortalEra: false},
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
		return fmt.Errorf("cannot sign:%v", err)
	}

	// Send the extrinsic
	sub, err := rpc.SubmitAndWatchExtrinsic(ext, api.Client)
	if err != nil {
		return fmt.Errorf("cannot submit extrinsic:%v", err)
	}

	defer sub.Unsubscribe()
	timeout := time.After(100 * time.Second)
	for {
		select {
		case status := <-sub.Chan():
			if status.IsInBlock {
				fmt.Printf("Txn inside block %v\n", status.AsInBlock.Hex())
			} else if status.IsFinalized {
				fmt.Printf("Txn inside finalized block\n")
				finalizedBlockCh <- status.AsFinalized
				return nil
			}
		case <-timeout:
			fmt.Printf("timeout of 100 seconds reached without getting finalized status for extrinsic")
			return nil
		}
	}
}
func RandToken(n int) (string, error) {
	bytes := make([]byte, n)
	if _, err := rand.Read(bytes); err != nil {
		return "", err
	}
	return hex.EncodeToString(bytes), nil
}
