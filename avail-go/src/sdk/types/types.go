package types

import (
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

type MultiAddress = types.MultiAddress
type Call = types.Call
type Hash = types.Hash
type U32 = types.U32
type U264 = types.U64
type U256 = types.U256
type U128 = types.U128
type ChainProperties struct {
	IsEthereum    bool
	SS58Format    types.U32
	TokenDecimals types.U32
	TokenSymbol   types.Text
}
