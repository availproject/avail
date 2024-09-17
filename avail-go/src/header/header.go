package header

import (
	avail "avail-go-sdk/src/sdk/types"

	"github.com/centrifuge/go-substrate-rpc-client/v4/scale"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

type DataLookupIndexItem struct {
	AppId avail.UCompact `json:"appId"`
	Start avail.UCompact `json:"start"`
}
type DataLookup struct {
	Size  avail.UCompact        `json:"size"`
	Index []DataLookupIndexItem `json:"index"`
}

type KateCommitment struct {
	Rows       avail.UCompact `json:"rows"`
	Cols       avail.UCompact `json:"cols"`
	Commitment []types.U8     `json:"commitment"`
	DataRoot   types.Hash     `json:"dataRoot"`
}

type V3HeaderExtension struct {
	AppLookup  DataLookup     `json:"appLookup"`
	Commitment KateCommitment `json:"commitment"`
}

type HeaderExtensionEnum struct {
	V3 V3HeaderExtension `json:"V3"`
}

type Header struct {
	ParentHash     types.Hash          `json:"parentHash"`
	Number         types.BlockNumber   `json:"number"`
	StateRoot      types.Hash          `json:"stateRoot"`
	ExtrinsicsRoot types.Hash          `json:"extrinsicsRoot"`
	Digest         types.Digest        `json:"digest"`
	Extension      HeaderExtensionEnum `json:"extension"`
}

func (m HeaderExtensionEnum) Encode(encoder scale.Encoder) error {
	var err, err1 error

	err = encoder.PushByte(2)

	if err != nil {
		return err
	}
	err1 = encoder.Encode(m.V3)
	if err1 != nil {
		return err1
	}
	return nil
}

func (m *HeaderExtensionEnum) Decode(decoder scale.Decoder) error {
	b, err := decoder.ReadOneByte()

	if err != nil {
		return err
	}

	if b == 2 {
		err = decoder.Decode(&m.V3)
	}
	if err != nil {
		return err
	}

	return nil
}
