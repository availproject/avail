package call

import (
	"avail-go-sdk/src/rpc"
	"avail-go-sdk/src/sdk"
	"avail-go-sdk/src/sdk/types"
)

func Query_proof(api *sdk.SubstrateAPI, testCell []rpc.Cell, blockHash types.Hash) ([]rpc.GDataProof, error) {
	var result []rpc.GDataProof
	err := api.Client.Call(&result, "kate_queryProof", testCell, blockHash)
	if err != nil {
		return nil, err
	}
	return result, nil
}

func Query_rows(api *sdk.SubstrateAPI, arr []uint32, h types.Hash) ([][]sdk.BigInt, error) {
	var response [][]sdk.BigInt
	err := api.Client.Call(&response, "kate_queryRows", arr, h)
	if err != nil {
		return nil, err
	}
	return response, nil
}

func Query_dataproof(api *sdk.SubstrateAPI, transactionIndex types.U32, h types.Hash) (rpc.ProofResponse, error) {
	var response rpc.ProofResponse
	err := api.Client.Call(&response, "kate_queryDataProof", transactionIndex, h)
	if err != nil {
		return rpc.ProofResponse{}, err
	}
	return response, nil
}
