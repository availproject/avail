package rpc

import (
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

// ProofResponse struct represents the response from the queryDataProof2 RPC call
type ProofResponse struct {
	DataProof DataProof
	Message   Message // Interface to capture different message types
}

type TxDataRoot struct {
	DataRoot   types.Hash
	BlobRoot   types.Hash
	BridgeRoot types.Hash
}

// DataProof struct represents the data proof response
type DataProof struct {
	Roots          TxDataRoot
	Proof          []types.Hash
	NumberOfLeaves uint32 // Change to uint32 to match Rust u32
	LeafIndex      uint32 // Change to uint32 to match Rust u32
	Leaf           types.Hash
}

// Message interface represents the enum variants
type Message interface {
	isMessage()
}

type BoundedData struct {
	Data []byte
}

// BoundedDataMaxLen is the maximum length for the bounded data
const BoundedDataMaxLen = 32 // Adjust the maximum length as needed

// ArbitraryMessage struct represents the ArbitraryMessage variant
type ArbitraryMessage struct {
	BoundedData
}

func (a *ArbitraryMessage) isMessage() {}

// FungibleToken struct represents the FungibleToken variant
type FungibleToken struct {
	AssetID types.Hash
	Amount  uint128 // Define uint128 type as needed
}

func (f *FungibleToken) isMessage() {}

type uint128 uint64
