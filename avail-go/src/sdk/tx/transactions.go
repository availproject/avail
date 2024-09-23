package tx

import (
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/types"
	"errors"
	"fmt"
	"math/big"
	"strconv"

	"github.com/vedhavyas/go-subkey/v2"
)

type WaitFor int

const (
	BlockInclusion WaitFor = iota + 1
	BlockFinalization
)

func (w WaitFor) String() string {
	return [...]string{"BlockInclusion", "BlockFinalization"}[w-1]
}

// EnumIndex - Creating common behavior - give the type a EnumIndex function
func (w WaitFor) EnumIndex() int {
	return int(w)
}

// SubmitData submits data to the chain
func SubmitData(api *sdk.SubstrateAPI, seed string, AppID int, data string, WaitForInclusion sdk.WaitFor) (types.Hash, types.Hash, error) {
	keyringPair, err := sdk.KeyringFromSeed(seed)
	if err != nil {
		panic(fmt.Sprintf("cannot create KeyPair:%v", err))
	}
	BlockHashCh2 := make(chan types.Hash)
	txHashCh2 := make(chan types.Hash)
	go func() {
		err := sdk.NewExtrinsicWatch(api, "DataAvailability.submit_data", keyringPair, BlockHashCh2, txHashCh2, AppID, WaitForInclusion, sdk.NewBytes([]byte(data)))
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
			close(BlockHashCh2)
			close(txHashCh2)
			return
		}
		fmt.Println("Data submitted successfully")
	}()
	blockHash := <-BlockHashCh2
	txHash := <-txHashCh2
	return blockHash, txHash, nil
}

func CreateApplicationKey(api *sdk.SubstrateAPI, seed string, data string, WaitForInclusion sdk.WaitFor) (types.Hash, types.Hash, error) {
	keyringPair, err := sdk.KeyringFromSeed(seed)
	if err != nil {
		panic(fmt.Sprintf("cannot create KeyPair:%v", err))
	}
	BlockHashCh2 := make(chan types.Hash)
	txHashCh2 := make(chan types.Hash)
	go func() {
		err := sdk.NewExtrinsicWatch(api, "DataAvailability.create_application_key", keyringPair, BlockHashCh2, txHashCh2, 0, WaitForInclusion, sdk.NewBytes([]byte("22222")))
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
			close(BlockHashCh2)
			close(txHashCh2)
			return
		}
		fmt.Println("Data submitted successfully")
	}()
	blockHash := <-BlockHashCh2
	txHash := <-txHashCh2
	return blockHash, txHash, nil

}

func SetApplicationKey(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, oldKey string, newKey string) (types.Hash, types.Hash, error) {
	keyringPair, err := sdk.KeyringFromSeed(seed)
	if err != nil {
		panic(fmt.Sprintf("cannot create KeyPair:%v", err))
	}
	BlockHashCh2 := make(chan types.Hash)
	txHashCh2 := make(chan types.Hash)
	go func() {
		call, err := sdk.NewCall(api, "DataAvailability.set_application_key", sdk.NewBytes([]byte(oldKey)), sdk.NewBytes([]byte(newKey)))
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
		}
		err = sdk.NewExtrinsicWatch(api, "Sudo.sudo", keyringPair, BlockHashCh2, txHashCh2, 0, WaitForInclusion, call)
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
			close(BlockHashCh2)
			close(txHashCh2)
			return
		}
		fmt.Println("Transaction submitted successfully")
	}()
	blockHash := <-BlockHashCh2
	txHash := <-txHashCh2
	return blockHash, txHash, nil

}

func SetSubmitDataFeeModifier(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, modifier sdk.DispatchFeeModifier) (types.Hash, types.Hash, error) {
	keyringPair, err := sdk.KeyringFromSeed(seed)
	if err != nil {
		panic(fmt.Sprintf("cannot create KeyPair:%v", err))
	}
	BlockHashCh2 := make(chan types.Hash)
	txHashCh2 := make(chan types.Hash)

	go func() {
		call, err := sdk.NewCall(api, "DataAvailability.set_submit_data_fee_modifier", modifier)
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
		}
		err = sdk.NewExtrinsicWatch(api, "Sudo.sudo", keyringPair, BlockHashCh2, txHashCh2, 0, WaitForInclusion, call)
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
			close(BlockHashCh2)
			close(txHashCh2)
			return
		}
		fmt.Println("Transaction submitted successfully")
	}()
	blockHash := <-BlockHashCh2
	txHash := <-txHashCh2
	return blockHash, txHash, nil

}

func SubmitBlockLength(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, rows uint32, cols uint32) (types.Hash, types.Hash, error) {
	keyringPair, err := sdk.KeyringFromSeed(seed)
	if err != nil {
		panic(fmt.Sprintf("cannot create KeyPair:%v", err))
	}
	BlockHashCh2 := make(chan types.Hash)
	txHashCh2 := make(chan types.Hash)

	go func() {
		call, err := sdk.NewCall(api, "DataAvailability.submit_block_length_proposal", sdk.NewU32(rows), sdk.NewU32(cols))
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
		}
		err = sdk.NewExtrinsicWatch(api, "Sudo.sudo", keyringPair, BlockHashCh2, txHashCh2, 0, WaitForInclusion, call)
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
			close(BlockHashCh2)
			close(txHashCh2)
			return
		}
		fmt.Println("Data submitted successfully")
	}()
	blockHash := <-BlockHashCh2
	txHash := <-txHashCh2
	return blockHash, txHash, nil

}

func Bond(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, amount int64, Payee sdk.Payee) (types.Hash, types.Hash, error) {
	keyringPair, err := sdk.KeyringFromSeed(seed)
	if err != nil {
		panic(fmt.Sprintf("cannot create KeyPair:%v", err))
	}
	bondAmount := sdk.ConvertToBondAmount(amount)

	newBondAmount := new(big.Int)
	newBondAmount.Set(bondAmount)

	BlockHashCh2 := make(chan types.Hash)
	txHashCh2 := make(chan types.Hash)
	bondAmountUCompact := types.NewUCompact(newBondAmount)
	go func() {
		err = sdk.NewExtrinsicWatch(api, "Staking.bond", keyringPair, BlockHashCh2, txHashCh2, 0, WaitForInclusion, bondAmountUCompact, sdk.NewU8(Payee.EnumIndex()))
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
			close(BlockHashCh2)
			close(txHashCh2)
			return
		}
		fmt.Println("Data submitted successfully")
	}()
	blockHash := <-BlockHashCh2
	txHash := <-txHashCh2
	return blockHash, txHash, nil
}

func BondExtra(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, amount int64) (types.Hash, types.Hash, error) {
	keyringPair, err := sdk.KeyringFromSeed(seed)
	if err != nil {
		panic(fmt.Sprintf("cannot create KeyPair:%v", err))
	}
	BlockHashCh2 := make(chan types.Hash)
	txHashCh2 := make(chan types.Hash)
	bondAmount := sdk.ConvertToBondAmount(amount)

	newBondAmount := new(big.Int)
	newBondAmount.Set(bondAmount)
	bondAmountUCompact := types.NewUCompact(newBondAmount)

	go func() {
		err = sdk.NewExtrinsicWatch(api, "Staking.bond_extra", keyringPair, BlockHashCh2, txHashCh2, 0, WaitForInclusion, bondAmountUCompact)
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
			close(BlockHashCh2)
			close(txHashCh2)
			return
		}
		fmt.Println("Data submitted successfully")
	}()
	blockHash := <-BlockHashCh2
	txHash := <-txHashCh2
	return blockHash, txHash, nil
}

func Chill(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor) (types.Hash, types.Hash, error) {
	keyringPair, err := sdk.KeyringFromSeed(seed)
	if err != nil {
		panic(fmt.Sprintf("cannot create KeyPair:%v", err))
	}
	BlockHashCh2 := make(chan types.Hash)
	txHashCh2 := make(chan types.Hash)

	go func() {
		err = sdk.NewExtrinsicWatch(api, "Staking.chill", keyringPair, BlockHashCh2, txHashCh2, 0, WaitForInclusion)
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
			close(BlockHashCh2)
			close(txHashCh2)
			return
		}
		fmt.Println("Data submitted successfully")
	}()
	blockHash := <-BlockHashCh2
	txHash := <-txHashCh2
	return blockHash, txHash, nil
}

func ChillOther(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, stash string) (types.Hash, types.Hash, error) {
	keyringPair, err := sdk.KeyringFromSeed(seed)
	if err != nil {
		panic(fmt.Sprintf("cannot create KeyPair:%v", err))
	}
	fmt.Println(keyringPair)
	_, pubkeyBytes, _ := subkey.SS58Decode(stash)
	hexString := subkey.EncodeHex(pubkeyBytes)

	fmt.Println(hexString)
	dest, err := sdk.NewMultiAddressFromHexAccountID(hexString)
	if err != nil {
		return types.Hash{}, types.Hash{}, err
	}
	fmt.Println(dest)
	BlockHashCh2 := make(chan types.Hash)
	txHashCh2 := make(chan types.Hash)

	go func() {
		err = sdk.NewExtrinsicWatch(api, "Staking.chill_other", keyringPair, BlockHashCh2, txHashCh2, 0, WaitForInclusion, dest.AsID)
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
			close(BlockHashCh2)
			close(txHashCh2)
			return
		}
		fmt.Println("Data submitted successfully")
	}()
	blockHash := <-BlockHashCh2
	txHash := <-txHashCh2
	return blockHash, txHash, nil
}

func Nominate(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, stash []string) (types.Hash, types.Hash, error) {
	keyringPair, err := sdk.KeyringFromSeed(seed)
	if err != nil {
		panic(fmt.Sprintf("cannot create KeyPair:%v", err))
	}
	var dest []types.MultiAddress
	for _, v := range stash {
		_, pubkeyBytes, _ := subkey.SS58Decode(v)
		hexString := subkey.EncodeHex(pubkeyBytes)
		d, err := sdk.NewMultiAddressFromHexAccountID(hexString)
		if err != nil {
			return types.Hash{}, types.Hash{}, err
		}
		dest = append(dest, d)
	}

	BlockHashCh2 := make(chan types.Hash)
	txHashCh2 := make(chan types.Hash)

	go func() {
		err = sdk.NewExtrinsicWatch(api, "Staking.nominate", keyringPair, BlockHashCh2, txHashCh2, 0, WaitForInclusion, dest)
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
			close(BlockHashCh2)
			close(txHashCh2)
			return
		}
		fmt.Println("Data submitted successfully")
	}()
	blockHash := <-BlockHashCh2
	txHash := <-txHashCh2
	return blockHash, txHash, nil
}

func Unbond(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, amount types.UCompact) (types.Hash, types.Hash, error) {
	keyringPair, err := sdk.KeyringFromSeed(seed)
	if err != nil {
		panic(fmt.Sprintf("cannot create KeyPair:%v", err))
	}
	BlockHashCh2 := make(chan types.Hash)
	txHashCh2 := make(chan types.Hash)

	go func() {
		err = sdk.NewExtrinsicWatch(api, "Staking.unbond", keyringPair, BlockHashCh2, txHashCh2, 0, WaitForInclusion, amount)
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
			close(BlockHashCh2)
			close(txHashCh2)
			return
		}
		fmt.Println("Data submitted successfully")
	}()
	blockHash := <-BlockHashCh2
	txHash := <-txHashCh2
	return blockHash, txHash, nil
}

func Validate(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, commissionNum int) (types.Hash, types.Hash, error) {
	keyringPair, err := sdk.KeyringFromSeed(seed)
	if err != nil {
		panic(fmt.Sprintf("cannot create KeyPair:%v", err))
	}
	BlockHashCh2 := make(chan types.Hash)
	txHashCh2 := make(chan types.Hash)
	commissionStr, err := commissionNumberToPerbill(commissionNum)
	if err != nil {
		fmt.Errorf("failed to convert commission to Perbill: %v", err)
	}

	commission, err := strconv.ParseUint(commissionStr, 10, 64)
	if err != nil {
		fmt.Errorf("failed to parse commission string: %v", err)
	}

	// Define the ValidatorPrefs struct manually
	type ValidatorPrefs struct {
		Commission types.UCompact
		Blocked    bool
	}

	// Create the validator preferences struct
	prefs := ValidatorPrefs{
		Commission: types.NewUCompactFromUInt(commission),
		Blocked:    false,
	}
	go func() {
		err = sdk.NewExtrinsicWatch(api, "Staking.validate", keyringPair, BlockHashCh2, txHashCh2, 0, WaitForInclusion, prefs)
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
			close(BlockHashCh2)
			close(txHashCh2)
			return
		}
		fmt.Println("Data submitted successfully")
	}()
	blockHash := <-BlockHashCh2
	txHash := <-txHashCh2
	return blockHash, txHash, nil
}

func commissionNumberToPerbill(value int) (string, error) {
	if value < 0 || value > 100 {
		return "", errors.New("Commission is limited to the following range: 0 - 100. It cannot be less than 0 or more than 100.")
	}

	commission := strconv.Itoa(value) + "0000000"
	// For some reason 0 commission is not defined as "0" but as "1".
	if commission == "00000000" {
		commission = "1"
	}

	return commission, nil
}

func TransferKeepAlive(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, dest string, amount types.UCompact) (types.Hash, types.Hash, error) {
	keyringPair, err := sdk.KeyringFromSeed(seed)
	if err != nil {
		panic(fmt.Sprintf("cannot create KeyPair:%v", err))
	}
	_, pubkeyBytes, _ := subkey.SS58Decode(dest)
	hexString := subkey.EncodeHex(pubkeyBytes)
	destAddr, err := sdk.NewMultiAddressFromHexAccountID(hexString)
	if err != nil {
		return types.Hash{}, types.Hash{}, err
	}
	BlockHashCh2 := make(chan types.Hash)
	txHashCh2 := make(chan types.Hash)

	go func() {
		err = sdk.NewExtrinsicWatch(api, "Balances.transfer_keep_alive", keyringPair, BlockHashCh2, txHashCh2, 0, WaitForInclusion, destAddr, amount)
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
			close(BlockHashCh2)
			close(txHashCh2)
			return
		}
		fmt.Println("Transaction submitted successfully")
	}()
	blockHash := <-BlockHashCh2
	txHash := <-txHashCh2
	return blockHash, txHash, nil
}

func TransferAllowDeath(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, dest string, amount types.UCompact) (types.Hash, types.Hash, error) {
	keyringPair, err := sdk.KeyringFromSeed(seed)
	if err != nil {
		panic(fmt.Sprintf("cannot create KeyPair:%v", err))
	}
	_, pubkeyBytes, _ := subkey.SS58Decode(dest)
	hexString := subkey.EncodeHex(pubkeyBytes)
	destAddr, err := sdk.NewMultiAddressFromHexAccountID(hexString)
	if err != nil {
		return types.Hash{}, types.Hash{}, err
	}
	BlockHashCh2 := make(chan types.Hash)
	txHashCh2 := make(chan types.Hash)

	go func() {
		err = sdk.NewExtrinsicWatch(api, "Balances.transfer_allow_death", keyringPair, BlockHashCh2, txHashCh2, 0, WaitForInclusion, destAddr, amount)
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
			close(BlockHashCh2)
			close(txHashCh2)
			return
		}
		fmt.Println("Transaction submitted successfully")
	}()
	blockHash := <-BlockHashCh2
	txHash := <-txHashCh2
	return blockHash, txHash, nil
}

func TransferAll(api *sdk.SubstrateAPI, seed string, WaitForInclusion sdk.WaitFor, dest string) (types.Hash, types.Hash, error) {
	keyringPair, err := sdk.KeyringFromSeed(seed)
	if err != nil {
		panic(fmt.Sprintf("cannot create KeyPair:%v", err))
	}
	_, pubkeyBytes, _ := subkey.SS58Decode(dest)
	hexString := subkey.EncodeHex(pubkeyBytes)
	destAddr, err := sdk.NewMultiAddressFromHexAccountID(hexString)
	if err != nil {
		return types.Hash{}, types.Hash{}, err
	}
	BlockHashCh2 := make(chan types.Hash)
	txHashCh2 := make(chan types.Hash)

	go func() {
		err = sdk.NewExtrinsicWatch(api, "Balances.transfer_all", keyringPair, BlockHashCh2, txHashCh2, 0, WaitForInclusion, destAddr, sdk.NewU8(1)) //KeepAlive:yes, change to 0 for no
		if err != nil {
			fmt.Printf("cannot create extrinsic: %v", err)
			close(BlockHashCh2)
			close(txHashCh2)
			return
		}
		fmt.Println("Transaction submitted successfully")
	}()
	blockHash := <-BlockHashCh2
	txHash := <-txHashCh2
	return blockHash, txHash, nil
}
