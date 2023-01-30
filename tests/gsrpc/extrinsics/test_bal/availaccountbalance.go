package main

import (
	"fmt"
	"math/big"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/tyler-smith/go-bip39"
)

func NewAccount() (signature.KeyringPair, error) {
	entropy, err := bip39.NewEntropy(128)
	if err != nil {
		return signature.KeyringPair{}, err
	}

	mnemonic, err := bip39.NewMnemonic(entropy)
	if err != nil {
		return signature.KeyringPair{}, err
	}

	keyPair, err := signature.KeyringPairFromSecret(mnemonic, 42)
	if err != nil {
		return signature.KeyringPair{}, err
	}

	return keyPair, nil
}

func DepositBalance(api *gsrpc.SubstrateAPI, account signature.KeyringPair, amount *big.Int) error {
	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		return err
	}

	addr, _ := types.NewMultiAddressFromAccountID(account.PublicKey)

	c, err := types.NewCall(meta, "Balances.transfer", addr, types.NewUCompact(amount)) //types.NewU128(*amount))
	if err != nil {
		return err
	}

	// Create the extrinsic
	ext := types.NewExtrinsic(c)

	genesisHash, err := api.RPC.Chain.GetBlockHash(0)
	if err != nil {
		return err
	}

	rv, err := api.RPC.State.GetRuntimeVersionLatest()
	if err != nil {
		return err
	}

	key, err := types.CreateStorageKey(meta, "System", "Account", signature.TestKeyringPairAlice.PublicKey, nil)
	if err != nil {
		return err
	}

	var accountInfo types.AccountInfo
	ok, err := api.RPC.State.GetStorageLatest(key, &accountInfo)
	if err != nil || !ok {
		return err
	}

	nonce := uint32(accountInfo.Nonce)

	o := types.SignatureOptions{
		BlockHash:          genesisHash,
		Era:                types.ExtrinsicEra{IsMortalEra: false},
		GenesisHash:        genesisHash,
		Nonce:              types.NewUCompactFromUInt(uint64(nonce)),
		SpecVersion:        rv.SpecVersion,
		Tip:                types.NewUCompactFromUInt(0),
		AppID:              types.NewUCompactFromUInt(0),
		TransactionVersion: rv.TransactionVersion,
	}

	// Sign the transaction using Alice's default account
	err = ext.Sign(signature.TestKeyringPairAlice, o)
	if err != nil {
		return err
	}

	// Send the extrinsic
	sub, err := api.RPC.Author.SubmitAndWatchExtrinsic(ext)
	if err != nil {
		return err
	}

	defer sub.Unsubscribe()

	for {
		select {
		case status := <-sub.Chan():
			// NOTE: See first line of this function for supported extrinsic status expectations.
			switch {
			case status.IsFinalized:
				return nil
			case status.IsInBlock:
				return nil
			default:
				if status.IsDropped || status.IsInvalid {
					return fmt.Errorf("unexpected extrinsic status from Avail: %#v", status)
				}
			}
		case err := <-sub.Err():
			// TODO: Consider re-connecting subscription channel on error?
			return err
		}
	}
}

// func NewAppKey(api *gsrpc.SubstrateAPI, account signature.KeyringPair) error {
// 	meta, err := api.RPC.State.GetMetadataLatest()
// 	if err != nil {
// 		return err
// 	}
// 	key, err := types.CreateStorageKey(meta, "System", "Account", account.PublicKey)
// 	if err != nil {
// 		return err
// 	}

// 	var accountInfo types.AccountInfo

// 	ok, err := api.RPC.State.GetStorageLatest(key, &accountInfo)
// 	if err != nil {
// 		return fmt.Errorf("couldn't fetch latest account storage info: %w", err)
// 	}

// 	if !ok {
// 		accountInfo.Nonce = 1
// 	}

// 	meta, err = api.RPC.State.GetMetadataLatest()
// 	if err != nil {
// 		return err
// 	}

// 	encodedBytes, err := types.EncodeToBytes("avail-settlement")
// 	call, err := types.NewCall(meta, "DataAvailability.create_application_key", []byte("avail-settlement"))
// 	if err != nil {
// 		return err
// 	}

// 	ext := types.NewExtrinsic(call)

// 	rv, err := api.RPC.State.GetRuntimeVersionLatest()
// 	if err != nil {
// 		return err
// 	}

// 	genesisHash, err := api.RPC.Chain.GetBlockHash(0)
// 	if err != nil {
// 		return err
// 	}

// 	nonce := uint64(accountInfo.Nonce)
// 	o := types.SignatureOptions{
// 		// This transaction is Immortal (https://wiki.polkadot.network/docs/build-protocol-info#transaction-mortality)
// 		// Hence BlockHash: Genesis Hash.
// 		BlockHash:          genesisHash,
// 		Era:                types.ExtrinsicEra{IsMortalEra: false},
// 		GenesisHash:        genesisHash,
// 		Nonce:              types.NewUCompactFromUInt(nonce),
// 		SpecVersion:        rv.SpecVersion,
// 		Tip:                types.NewUCompactFromUInt(100),
// 		AppID:              types.NewU32(0),
// 		TransactionVersion: rv.TransactionVersion,
// 	}

// 	err = ext.Sign(account, o)
// 	if err != nil {
// 		return err
// 	}

// 	fmt.Println("submitting tx.")

// 	sub, err := api.RPC.Author.SubmitAndWatchExtrinsic(ext)
// 	if err != nil {
// 		return fmt.Errorf("failed to submit application key creation tx: %w", err)
// 	}

// 	defer sub.Unsubscribe()

// 	for {
// 		select {
// 		case status := <-sub.Chan():
// 			switch {
// 			case status.IsFinalized:
// 				fmt.Println("block with create application key tx was finalized.")
// 				return nil
// 			case status.IsInBlock:
// 				fmt.Println("block with create application key tx is in block.")

// 				key, err = types.CreateStorageKey(meta, "DataAvailability", "AppKeys", encodedBytes)
// 				if err != nil {
// 					return err
// 				}

// 				type AppKeyInfo struct {
// 					AccountID types.AccountID
// 					AppID     types.U32
// 				}

// 				var aki AppKeyInfo
// 				ok, err := api.RPC.State.GetStorageLatest(key, &aki)
// 				if err != nil {
// 					return fmt.Errorf("failed to read new appID: %w", err)
// 				}

// 				if ok {
// 					fmt.Printf("found appID: %d\n", aki.AppID)
// 				} else {
// 					fmt.Println("couldn't find appID :(")
// 				}

// 				continue
// 			default:
// 				if status.IsDropped || status.IsInvalid {
// 					return fmt.Errorf("unexpected extrinsic status from Avail: %#v", status)
// 				}
// 			}
// 		case err = <-sub.Err():
// 			return fmt.Errorf("error while watching for extrinsic status: %w", err)
// 		}
// 	}
// }

func GetBalance(api *gsrpc.SubstrateAPI, account signature.KeyringPair) (types.U128, error) {
	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		panic(err)
	}

	key, err := types.CreateStorageKey(meta, "System", "Account", account.PublicKey, nil)
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

func main() {
	api, err := gsrpc.NewSubstrateAPI("ws://127.0.0.1:9944/v1/json-rpc")
	if err != nil {
		panic(err)
	}

	account, err := NewAccount()
	if err != nil {
		panic(err)
	}

	balanceZero, err := GetBalance(api, account)
	if err != nil {
		panic(err)
	}

	fmt.Printf("New account balance before deposit: %v\n", balanceZero)

	balance := big.NewInt(300_000_000_000_000_000)

	fmt.Printf("created a new account: %q\n", account.URI)
	fmt.Printf("transferring %d units from %q to %q\n", balance, signature.TestKeyringPairAlice.Address, account.Address)

	err = DepositBalance(api, account, balance)
	if err != nil {
		panic(err)
	}

	fmt.Printf("transferred %d units from %q to %q\n", balance, signature.TestKeyringPairAlice.Address, account.Address)

	for {
		balance, err := GetBalance(api, account)
		if err != nil {
			panic(err)
		}

		fmt.Printf("account balance: %v\n", balance)

		moreFunds := big.NewInt(500_000_000_000)
		err = DepositBalance(api, account, moreFunds)
		if err != nil {
			panic(err)
		}

		fmt.Printf("transferred %d units from %q to %q\n", moreFunds, signature.TestKeyringPairAlice.Address, account.Address)
	}
}
