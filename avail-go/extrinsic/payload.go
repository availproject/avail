package extrinsic

import (
	"fmt"

	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types/codec"
	"github.com/vedhavyas/go-subkey/scale"
)

type ExtrinsicPayloadV3 struct {
	Method             types.BytesBare
	Era                ExtrinsicEra   // extra via system::CheckEra
	Nonce              types.UCompact // extra via system::CheckNonce (Compact<Index> where Index is u32)
	Tip                types.UCompact // extra via balances::TakeFees (Compact<Balance> where Balance is u128)
	AppID              types.UCompact
	SpecVersion        types.U32 // additional via system::CheckVersion
	TransactionVersion types.U32
	GenesisHash        types.Hash // additional via system::CheckGenesis
	BlockHash          types.Hash // additional via system::CheckEra
}

// Sign the extrinsic payload with the given derivation path
func (e ExtrinsicPayloadV3) Sign(signer signature.KeyringPair) (types.Signature, error) {
	b, err := codec.Encode(e)
	if err != nil {
		return types.Signature{}, err
	}

	sig, err := signature.Sign(b, signer.URI)
	return types.NewSignature(sig), err
}

// Encode implements encoding for ExtrinsicPayloadV3, which just unwraps the bytes of ExtrinsicPayloadV3 without
// adding a compact length prefix
func (e ExtrinsicPayloadV3) Encode(encoder scale.Encoder) error {
	err := encoder.Encode(e.Method)
	if err != nil {
		return err
	}

	err = encoder.Encode(e.Era)
	if err != nil {
		return err
	}

	err = encoder.Encode(e.Nonce)
	if err != nil {
		return err
	}

	err = encoder.Encode(e.Tip)
	if err != nil {
		return err
	}

	err = encoder.Encode(e.SpecVersion)
	if err != nil {
		return err
	}

	err = encoder.Encode(e.GenesisHash)
	if err != nil {
		return err
	}

	err = encoder.Encode(e.BlockHash)
	if err != nil {
		return err
	}
	err = encoder.Encode(e.AppID)
	if err != nil {
		return err
	}
	err = encoder.Encode(e.TransactionVersion)
	if err != nil {
		return err
	}

	return nil
}

// Decode does nothing and always returns an error. ExtrinsicPayloadV3 is only used for encoding, not for decoding
func (e *ExtrinsicPayloadV3) Decode(decoder scale.Decoder) error {
	return fmt.Errorf("decoding of ExtrinsicPayloadV3 is not supported")
}

type ExtrinsicPayloadV4 struct {
	ExtrinsicPayloadV3
	TransactionVersion types.U32
	AppID              types.UCompact
}

// Sign the extrinsic payload with the given derivation path
func (e ExtrinsicPayloadV4) Sign(signer signature.KeyringPair) (types.Signature, error) {
	b, err := codec.Encode(e)
	if err != nil {
		return types.Signature{}, err
	}

	sig, err := signature.Sign(b, signer.URI)
	return types.NewSignature(sig), err
}

func (e ExtrinsicPayloadV4) Encode(encoder scale.Encoder) error {
	err := encoder.Encode(e.Method)
	if err != nil {
		return err
	}

	err = encoder.Encode(e.Era)
	if err != nil {
		return err
	}

	err = encoder.Encode(e.Nonce)
	if err != nil {
		return err
	}

	err = encoder.Encode(e.Tip)
	if err != nil {
		return err
	}

	err = encoder.Encode(e.AppID)
	if err != nil {
		return err
	}

	err = encoder.Encode(e.SpecVersion)
	if err != nil {
		return err
	}

	err = encoder.Encode(e.TransactionVersion)
	if err != nil {
		return err
	}

	err = encoder.Encode(e.GenesisHash)
	if err != nil {
		return err
	}

	err = encoder.Encode(e.BlockHash)
	if err != nil {
		return err
	}

	return nil
}

// Decode does nothing and always returns an error. ExtrinsicPayloadV4 is only used for encoding, not for decoding
func (e *ExtrinsicPayloadV4) Decode(decoder scale.Decoder) error {
	return fmt.Errorf("decoding of ExtrinsicPayloadV4 is not supported")
}
