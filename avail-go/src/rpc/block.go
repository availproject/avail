package rpc

import (
	"avail-go-sdk/src/extrinsic"
	"avail-go-sdk/src/header"

	"github.com/centrifuge/go-substrate-rpc-client/v4/client"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

type SignedBlock struct {
	Block         Block               `json:"block"`
	Justification types.Justification `json:"justification"`
}

// Block encoded with header and extrinsics
type Block struct {
	Header     header.Header `json:"header"`
	Extrinsics []extrinsic.Extrinsic
}

// GetBlock returns the header and body of the relay chain block with the given hash
func GetAvailBlock(blockHash types.Hash, client1 client.Client) (*SignedBlock, error) {
	return getBlock(&blockHash, client1)
}

func GetAvailBlockLatest(client1 client.Client) (*SignedBlock, error) {
	return getBlock(nil, client1)
}

func getBlock(blockHash *types.Hash, client1 client.Client) (*SignedBlock, error) {
	var SignedBlock SignedBlock
	err := client.CallWithBlockHash(client1, &SignedBlock, "chain_getBlock", blockHash)
	if err != nil {
		return nil, err
	}
	return &SignedBlock, err
}
