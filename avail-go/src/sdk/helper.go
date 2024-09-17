package sdk

import (
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"log"
	"math/big"
	"strings"

	"avail-go-sdk/src/rpc"

	"github.com/centrifuge/go-substrate-rpc-client/v4/scale"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types/codec"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/vedhavyas/go-subkey"
	"golang.org/x/crypto/blake2b"
)

func ConvertMultiAddress(receiver string) (types.MultiAddress, error) {
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

func ConvertToBondAmount(input int64) *big.Int {
	// Create a new big.Int for the input
	inputBig := big.NewInt(input)

	// Create a big.Int for 10^18
	multiplier := new(big.Int).Exp(big.NewInt(10), big.NewInt(18), nil)

	// Multiply input by 10^18
	result := new(big.Int).Mul(inputBig, multiplier)

	return result
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

func FormatBN(n BigInt) string {
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
func RandToken(n int) (string, error) {
	bytes := make([]byte, n)
	if _, err := rand.Read(bytes); err != nil {
		return "", err
	}
	return hex.EncodeToString(bytes), nil
}

func KeyringFromSeed(seed string) (signature.KeyringPair, error) {
	return signature.KeyringPairFromSecret(seed, 42)
}

type Bytes []byte

func NewBytes(b []byte) Bytes {
	return Bytes(b)
}

func EncodeToHex(hash interface{}) (string, error) {
	return codec.EncodeToHex(hash)
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

func CreateChannel() chan types.Hash {
	c := make(chan types.Hash, 1)
	return c
}

func ExistentialDeposit(api *SubstrateAPI) {

	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		panic(fmt.Sprintf("cannot get metadata: %v", err))
	}
	var existentialDeposit types.U128

	for _, mod := range meta.AsMetadataV14.Pallets {
		if string(mod.Name) == "Balances" {
			for _, constant := range mod.Constants {
				if string(constant.Name) == "ExistentialDeposit" {
					err = codec.Decode(constant.Value, &existentialDeposit)
					if err != nil {
						log.Fatalf("Failed to decode ExistentialDeposit: %v", err)
					}
					fmt.Printf("Existential Deposit: %d\n", existentialDeposit)
					return
				}
			}
		}
	}

	fmt.Printf("Existential Deposit: %d\n", existentialDeposit)
}

func NewCall(api *SubstrateAPI, callName string, args ...interface{}) (types.Call, error) {
	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		return types.Call{}, err
	}
	return types.NewCall(meta, callName, args...)
}

func NewHashFromHexString(hexStr string) (types.Hash, error) {
	return types.NewHashFromHexString(hexStr)
}

func NewU256(value *big.Int) types.U256 {
	return types.NewU256(*value)
}

func NewMultiAddressFromHexAccountID(hexStr string) (types.MultiAddress, error) {
	return types.NewMultiAddressFromHexAccountID(hexStr)
}

func NewMultiAddressFromAccountID(accountID []byte) (types.MultiAddress, error) {
	return types.NewMultiAddressFromAccountID(accountID)
}

func NewU8(value uint8) types.U8 {
	return types.U8(value)
}

func NewU32(value uint32) types.U32 {
	return types.U32(value)
}

func NewU64(value uint64) types.U64 {
	return types.U64(value)
}

func NewU128(value *big.Int) types.U128 {
	return types.NewU128(*value)
}

func KeyringPairFromSecret(seed string, keyType uint16) (signature.KeyringPair, error) {
	return signature.KeyringPairFromSecret(seed, keyType)
}

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

type Payee uint8

const (
	Staked WaitFor = iota
	Stash
	Controller
	Account
	None
)

func (w Payee) String() string {
	return [...]string{"Staked", "Stash", "Controller", "Account", "None"}[w]
}

// EnumIndex - Creating common behavior - give the type a EnumIndex function
func (w Payee) EnumIndex() uint8 {
	return uint8(w)
}

func GetData(hash types.Hash, api *SubstrateAPI, txHash types.Hash) error {
	block, err := rpc.GetAvailBlock(hash, api.Client)
	if err != nil {
		return fmt.Errorf("cannot get block by hash:%w", err)
	}
	for _, ext := range block.Block.Extrinsics {

		// these values below are specific indexes only for data submission, differs with each extrinsic
		if ext.IsSigned() && ext.Method.CallIndex.SectionIndex == 29 && ext.Method.CallIndex.MethodIndex == 1 {
			enc, _ := EncodeToHex(ext)
			cleanedHexString := strings.TrimPrefix(enc, "0x")
			bytes, err := hex.DecodeString(cleanedHexString)
			if err != nil {
				log.Fatal(err)
			}
			hash := blake2b.Sum256(bytes)
			txHashDecoded := hexutil.Encode(hash[:])
			if txHashDecoded == txHash.Hex() {
				arg := ext.Method.Args
				str := string(arg)
				slice := str[1:]

				fmt.Println("Data retrieved:", slice)
			}
		}
	}
	return nil
}

type DispatchFeeModifier struct {
	WeightMaximumFee    types.U128
	WeightFeeDivider    types.U32
	WeightFeeMultiplier types.U32
}

// Encode implements scale.Encodeable
func (d DispatchFeeModifier) Encode(encoder scale.Encoder) error {
	// Encode WeightMaximumFee
	if err := encoder.PushByte(1); err != nil { // 1 for Some
		return err
	}
	if err := encoder.Encode(d.WeightMaximumFee); err != nil {
		return err
	}

	// Encode WeightFeeDivider
	if err := encoder.PushByte(1); err != nil { // 1 for Some
		return err
	}
	if err := encoder.Encode(d.WeightFeeDivider); err != nil {
		return err
	}

	// Encode WeightFeeMultiplier
	if err := encoder.PushByte(1); err != nil { // 1 for Some
		return err
	}
	if err := encoder.Encode(d.WeightFeeMultiplier); err != nil {
		return err
	}

	return nil
}

// Decode implements scale.Decodeable
func (d *DispatchFeeModifier) Decode(decoder scale.Decoder) error {
	// Decode WeightMaximumFee
	optionByte, err := decoder.ReadOneByte()
	if err != nil {
		return err
	}
	if optionByte == 1 { // Some
		if err := decoder.Decode(&d.WeightMaximumFee); err != nil {
			return err
		}
	} else if optionByte != 0 { // Not None (0) or Some (1)
		return fmt.Errorf("invalid option byte for WeightMaximumFee: %d", optionByte)
	}

	// Decode WeightFeeDivider
	optionByte, err = decoder.ReadOneByte()
	if err != nil {
		return err
	}
	if optionByte == 1 { // Some
		if err := decoder.Decode(&d.WeightFeeDivider); err != nil {
			return err
		}
	} else if optionByte != 0 { // Not None (0) or Some (1)
		return fmt.Errorf("invalid option byte for WeightFeeDivider: %d", optionByte)
	}

	// Decode WeightFeeMultiplier
	optionByte, err = decoder.ReadOneByte()
	if err != nil {
		return err
	}
	if optionByte == 1 { // Some
		if err := decoder.Decode(&d.WeightFeeMultiplier); err != nil {
			return err
		}
	} else if optionByte != 0 { // Not None (0) or Some (1)
		return fmt.Errorf("invalid option byte for WeightFeeMultiplier: %d", optionByte)
	}

	return nil
}
