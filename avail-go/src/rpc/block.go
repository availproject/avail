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

func GetFinalizedHead(client client.Client) (types.Hash, error) {
	var res string

	err := client.Call(&res, "chain_getFinalizedHead")
	if err != nil {
		return types.Hash{}, err
	}

	return types.NewHashFromHexString(res)
}

func getBlockHash(client client.Client, blockNumber *uint64) (types.Hash, error) {
	var res string
	var err error

	if blockNumber == nil {
		err = client.Call(&res, "chain_getBlockHash")
	} else {
		err = client.Call(&res, "chain_getBlockHash", *blockNumber)
	}

	if err != nil {
		return types.Hash{}, err
	}

	return types.NewHashFromHexString(res)
}

func GetBlockHash(client client.Client, blockNumber uint64) (types.Hash, error) {
	return getBlockHash(client, &blockNumber)
}

func GetBlockHashLatest(client client.Client) (types.Hash, error) {
	return getBlockHash(client, nil)
}

func getHeader(client1 client.Client, blockHash *types.Hash) (*header.Header, error) {
	var Header header.Header
	err := client.CallWithBlockHash(client1, &Header, "chain_getHeader", blockHash)
	if err != nil {
		return nil, err
	}
	return &Header, err
}

func GetHeaderLatest(client1 client.Client) (*header.Header, error) {
	return getHeader(client1, nil)
}

func GetHeader(client1 client.Client, blockHash types.Hash) (*header.Header, error) {
	return getHeader(client1, &blockHash)
}
