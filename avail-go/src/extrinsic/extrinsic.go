package extrinsic

import (
	"bytes"
	"encoding/json"
	"fmt"
	"math/big"
	"strings"

	"github.com/centrifuge/go-substrate-rpc-client/v4/scale"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types/codec"
)

type SignatureOptions struct {
	Era                ExtrinsicEra
	Nonce              types.UCompact
	Tip                types.UCompact
	SpecVersion        types.U32
	GenesisHash        types.Hash
	BlockHash          types.Hash
	AppID              types.UCompact
	TransactionVersion types.U32
}
type ExtrinsicSignatureV4 struct {
	Signer    types.MultiAddress
	Signature types.MultiSignature
	Era       ExtrinsicEra   // extra via system::CheckEra
	Nonce     types.UCompact // extra via system::CheckNonce (Compact<Index> where Index is u32))
	Tip       types.UCompact // extra via balances::TakeFees (Compact<Balance> where Balance is u128))
	AppID     types.UCompact // Avail specific AppID
}

const (
	ExtrinsicBitSigned      = 0x80
	ExtrinsicBitUnsigned    = 0
	ExtrinsicUnmaskVersion  = 0x7f
	ExtrinsicDefaultVersion = 1
	ExtrinsicVersionUnknown = 0 // v0 is unknown
	ExtrinsicVersion1       = 1
	ExtrinsicVersion2       = 2
	ExtrinsicVersion3       = 3
	ExtrinsicVersion4       = 4
)

// Extrinsic is a piece of Args bundled into a block that expresses something from the "external" (i.e. off-chain)
// world. There are, broadly speaking, two types of extrinsic: transactions (which tend to be signed) and
// inherents (which don't).
type Extrinsic struct {
	// Version is the encoded version flag (which encodes the raw transaction version and signing information in one byte)
	Version byte
	// Signature is the ExtrinsicSignatureV4, it's presence depends on the Version flag
	Signature ExtrinsicSignatureV4
	// Method is the call this extrinsic wraps
	Method types.Call
}

// NewExtrinsic creates a new Extrinsic from the provided Call
func NewExtrinsic(c types.Call) Extrinsic {
	return Extrinsic{
		Version: ExtrinsicVersion4,
		Method:  c,
	}
}

// UnmarshalJSON fills Extrinsic with the JSON encoded byte array given by bz
func (e *Extrinsic) UnmarshalJSON(bz []byte) error {
	var tmp string
	if err := json.Unmarshal(bz, &tmp); err != nil {
		return err
	}

	// HACK 11 Jan 2019 - before https://github.com/paritytech/substrate/pull/1388
	// extrinsics didn't have the length, cater for both approaches. This is very
	// inconsistent with any other `Vec<u8>` implementation
	var l types.UCompact
	err := codec.DecodeFromHex(tmp, &l)
	if err != nil {
		return err
	}

	prefix, err := codec.EncodeToHex(l)
	if err != nil {
		return err
	}

	// determine whether length prefix is there
	if strings.HasPrefix(tmp, prefix) {
		return codec.DecodeFromHex(tmp, e)
	}

	// not there, prepend with compact encoded length prefix
	dec, err := codec.HexDecodeString(tmp)
	if err != nil {
		return err
	}
	length := types.NewUCompactFromUInt(uint64(len(dec)))
	bprefix, err := codec.Encode(length)
	if err != nil {
		return err
	}
	bprefix = append(bprefix, dec...)
	return codec.Decode(bprefix, e)
}

// MarshalJSON returns a JSON encoded byte array of Extrinsic
func (e Extrinsic) MarshalJSON() ([]byte, error) {
	s, err := codec.EncodeToHex(e)
	if err != nil {
		return nil, err
	}
	return json.Marshal(s)
}

// IsSigned returns true if the extrinsic is signed
func (e Extrinsic) IsSigned() bool {
	return e.Version&ExtrinsicBitSigned == ExtrinsicBitSigned
}

// Type returns the raw transaction version (not flagged with signing information)
func (e Extrinsic) Type() uint8 {
	return e.Version & ExtrinsicUnmaskVersion
}

// Sign adds a signature to the extrinsic
func (e *Extrinsic) Sign(signer signature.KeyringPair, o SignatureOptions) error {
	if e.Type() != ExtrinsicVersion4 {
		return fmt.Errorf("unsupported extrinsic version: %v (isSigned: %v, type: %v)", e.Version, e.IsSigned(), e.Type())
	}

	mb, err := codec.Encode(e.Method)
	if err != nil {
		return err
	}

	era := o.Era
	if !o.Era.IsMortalEra {
		era = ExtrinsicEra{IsImmortalEra: true}
	}

	payload := ExtrinsicPayloadV3{
		Method:             mb,
		Era:                era,
		Nonce:              o.Nonce,
		Tip:                o.Tip,
		AppID:              o.AppID,
		SpecVersion:        o.SpecVersion,
		TransactionVersion: o.TransactionVersion,
		GenesisHash:        o.GenesisHash,
		BlockHash:          o.BlockHash,
	}

	signerPubKey, err := types.NewMultiAddressFromAccountID(signer.PublicKey)

	if err != nil {
		return err
	}

	sig, err := payload.Sign(signer)
	if err != nil {
		return err
	}

	extSig := ExtrinsicSignatureV4{
		Signer:    signerPubKey,
		Signature: types.MultiSignature{IsSr25519: true, AsSr25519: sig},
		Era:       era,
		Nonce:     o.Nonce,
		Tip:       o.Tip,
		AppID:     o.AppID,
	}

	e.Signature = extSig

	// mark the extrinsic as signed
	e.Version |= ExtrinsicBitSigned

	return nil
}

func (e *Extrinsic) Decode(decoder scale.Decoder) error {
	// compact length encoding (1, 2, or 4 bytes) (may not be there for Extrinsics older than Jan 11 2019)
	_, err := decoder.DecodeUintCompact()
	if err != nil {
		return err
	}

	// version, signature bitmask (1 byte)
	err = decoder.Decode(&e.Version)
	if err != nil {
		return err
	}

	// signature
	if e.IsSigned() {
		if e.Type() != ExtrinsicVersion4 {
			return fmt.Errorf("unsupported extrinsic version: %v (isSigned: %v, type: %v)", e.Version, e.IsSigned(),
				e.Type())
		}

		err = decoder.Decode(&e.Signature)
		if err != nil {
			return err
		}
	}

	// call
	err = decoder.Decode(&e.Method)
	if err != nil {
		return err
	}

	return nil
}

func (e Extrinsic) Encode(encoder scale.Encoder) error {
	if e.Type() != ExtrinsicVersion4 {
		return fmt.Errorf("unsupported extrinsic version: %v (isSigned: %v, type: %v)", e.Version, e.IsSigned(),
			e.Type())
	}

	// create a temporary buffer that will receive the plain encoded transaction (version, signature (optional),
	// method/call)
	var bb = bytes.Buffer{}
	tempEnc := scale.NewEncoder(&bb)

	// encode the version of the extrinsic
	err := tempEnc.Encode(e.Version)
	if err != nil {
		return err
	}

	// encode the signature if signed
	if e.IsSigned() {
		err = tempEnc.Encode(e.Signature)
		if err != nil {
			return err
		}
	}

	// encode the method
	err = tempEnc.Encode(e.Method)
	if err != nil {
		return err
	}

	// take the temporary buffer to determine length, write that as prefix
	eb := bb.Bytes()
	err = encoder.EncodeUintCompact(*big.NewInt(0).SetUint64(uint64(len(eb))))
	if err != nil {
		return err
	}

	// write the actual encoded transaction
	err = encoder.Write(eb)
	if err != nil {
		return err
	}

	return nil
}
