package rpc

import (
	"avail-go-sdk/chain"

	"github.com/centrifuge/go-substrate-rpc-client/v4/client"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

// GetBlock returns the header and body of the relay chain block with the given hash
func GetAvailBlock(blockHash types.Hash, client1 client.Client) (*chain.SignedBlock, error) {
	return getBlock(&blockHash, client1)
}

func GetAvailBlockLatest(client1 client.Client) (*chain.SignedBlock, error) {
	return getBlock(nil, client1)
}

func getBlock(blockHash *types.Hash, client1 client.Client) (*chain.SignedBlock, error) {
	var SignedBlock chain.SignedBlock
	err := client.CallWithBlockHash(client1, &SignedBlock, "chain_getBlock", blockHash)
	if err != nil {
		return nil, err
	}
	return &SignedBlock, err
}
