package sdk

import (
	"avail-go-sdk/src/extrinsic"
	"avail-go-sdk/src/rpc"
	"encoding/hex"
	"fmt"
	"log"
	"strings"
	"time"

	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"golang.org/x/crypto/blake2b"
)

func NewExtrinsic(api *SubstrateAPI, ext_call string, keyring signature.KeyringPair, AppID int, arg ...interface{}) (types.Hash, error) {
	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		return types.Hash{}, err
	}
	call, err := types.NewCall(meta, ext_call, arg...)
	if err != nil {
		return types.Hash{}, err
	}
	ext := extrinsic.NewExtrinsic(call)
	genesisHash, err := api.RPC.Chain.GetBlockHash(0)
	if err != nil {
		return types.Hash{}, err
	}
	rv, err := api.RPC.State.GetRuntimeVersionLatest()
	if err != nil {
		return types.Hash{}, err
	}
	key, err := types.CreateStorageKey(meta, "System", "Account", keyring.PublicKey)
	if err != nil {
		return types.Hash{}, err
	}

	var accountInfo types.AccountInfo
	ok, err := api.RPC.State.GetStorageLatest(key, &accountInfo)
	if err != nil || !ok {
		return types.Hash{}, err
	}
	nonce := uint32(accountInfo.Nonce)
	options := extrinsic.SignatureOptions{
		BlockHash:          genesisHash,
		Era:                extrinsic.ExtrinsicEra{IsMortalEra: false},
		GenesisHash:        genesisHash,
		Nonce:              types.NewUCompactFromUInt(uint64(nonce)),
		SpecVersion:        rv.SpecVersion,
		Tip:                types.NewUCompactFromUInt(100),
		AppID:              types.NewUCompactFromUInt(uint64(AppID)),
		TransactionVersion: rv.TransactionVersion,
	}
	err = ext.Sign(keyring, options)
	if err != nil {
		panic(fmt.Sprintf("cannot sign:%v", err))
	}
	hash, err := rpc.SubmitExtrinsic(ext, api.Client)
	if err != nil {
		panic(fmt.Sprintf("cannot submit extrinsic:%v", err))
	}

	fmt.Printf("Data submitted using APPID: %v \n", AppID)
	return hash, nil
}

func NewExtrinsicWatch(api *SubstrateAPI, ext_call string, keyring signature.KeyringPair, final chan types.Hash, txHash chan types.Hash, AppID int, WaitForInclusion WaitFor, arg ...interface{}) error {
	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		return err
	}
	call, err := types.NewCall(meta, ext_call, arg...)
	if err != nil {
		return err
	}
	ext := extrinsic.NewExtrinsic(call)
	genesisHash, err := api.RPC.Chain.GetBlockHash(0)
	if err != nil {
		return err
	}
	rv, err := api.RPC.State.GetRuntimeVersionLatest()
	if err != nil {
		return err
	}
	key, err := types.CreateStorageKey(meta, "System", "Account", keyring.PublicKey)
	if err != nil {
		return err
	}

	var accountInfo types.AccountInfo
	ok, err := api.RPC.State.GetStorageLatest(key, &accountInfo)
	if err != nil || !ok {
		return err
	}
	nonce := uint32(accountInfo.Nonce)
	options := extrinsic.SignatureOptions{
		BlockHash:          genesisHash,
		Era:                extrinsic.ExtrinsicEra{IsMortalEra: false},
		GenesisHash:        genesisHash,
		Nonce:              types.NewUCompactFromUInt(uint64(nonce)),
		SpecVersion:        rv.SpecVersion,
		Tip:                types.NewUCompactFromUInt(100),
		AppID:              types.NewUCompactFromUInt(uint64(AppID)),
		TransactionVersion: rv.TransactionVersion,
	}
	err = ext.Sign(keyring, options)
	if err != nil {
		panic(fmt.Sprintf("cannot sign:%v", err))
	}

	go func() {
		enc, _ := EncodeToHex(ext)

		cleanedHexString := strings.TrimPrefix(enc, "0x")
		bytes, err := hex.DecodeString(cleanedHexString)
		if err != nil {
			log.Fatal(err)
		}
		hash := blake2b.Sum256(bytes)
		ext_z := hexutil.Encode(hash[:])
		hash, err = NewHashFromHexString(ext_z)
		if err != nil {
			log.Fatal(err)
		}
		txHash <- hash
	}()

	sub, err := rpc.SubmitAndWatchExtrinsic(ext, api.Client)
	if err != nil {
		panic(fmt.Sprintf("cannot submit extrinsic:%v", err))
	}

	fmt.Printf("Transaction being submitted .... ⏳Waiting for block inclusion..")

	defer sub.Unsubscribe()
	timeout := time.After(200 * time.Second)
	for {
		select {
		case status := <-sub.Chan():
			switch WaitForInclusion {
			case BlockInclusion:
				if status.IsInBlock {
					final <- status.AsInBlock
					return err
				}
			case BlockFinalization:
				if status.IsFinalized {
					final <- status.AsFinalized
					return err
				}
			}
		case <-timeout:
			fmt.Printf("timeout of 200 seconds reached without getting finalized status for extrinsic")
			return err
		}
	}
}
