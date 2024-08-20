package chain

import (
	"avail-go-sdk/extrinsic"
	"avail-go-sdk/header"

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
