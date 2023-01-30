// You can edit this code!
// Click here and start typing.
package main

import (
	"fmt"
	"math/big"

	. "github.com/centrifuge/go-substrate-rpc-client/v4/types"
	. "github.com/centrifuge/go-substrate-rpc-client/v4/types/codec"
)

func main() {
	fmt.Println("Hello, 世界")

	var exampleHeader = Header{
		ParentHash:     Hash{1, 2, 3, 4, 5},
		Number:         42,
		StateRoot:      Hash{2, 3, 4, 5, 6},
		ExtrinsicsRoot: Hash{3, 4, 5, 6, 7},
		Digest: Digest{
			{IsOther: true, AsOther: Bytes{4, 5}},
			{IsChangesTrieRoot: true, AsChangesTrieRoot: Hash{6, 7}},
			{IsConsensus: true, AsConsensus: Consensus{ConsensusEngineID: 9, Bytes: Bytes{10, 11, 12}}},
			{IsSeal: true, AsSeal: Seal{ConsensusEngineID: 11, Bytes: Bytes{12, 13, 14}}},
			{IsPreRuntime: true, AsPreRuntime: PreRuntime{ConsensusEngineID: 13, Bytes: Bytes{14, 15, 16}}},
		},
		Extension: HeaderExtension{
			Enum: HeaderExtensionEnum{
				V1: V1HeaderExtension{
					Commitment: KateCommitment{
						Rows:       NewUCompactFromUInt(4),
						Cols:       NewUCompactFromUInt(1),
						DataRoot:   Hash{8, 9, 10, 11, 12},
						Commitment: []U8{1, 2, 3, 4},
					},
					AppLookup: DataLookup{
						Size: NewUCompactFromUInt(1),
						Index: []DataLookupIndexItem{
							{
								AppId: AppId(NewUCompactFromUInt(1)),
								Start: NewUCompactFromUInt(1),
							},
						},
					},
				},
				VTest: VTHeaderExtension{
					NewField: []U8{1, 2, 3, 4, 5},
					Commitment: KateCommitment{
						Rows:       NewUCompactFromUInt(8),
						Cols:       NewUCompactFromUInt(2),
						DataRoot:   Hash{13, 14, 15, 16, 17},
						Commitment: []U8{5, 6, 7, 8},
					},
					AppLookup: DataLookup{
						Size: NewUCompact(big.NewInt(2)),
						Index: []DataLookupIndexItem{
							{
								AppId: AppId(NewUCompactFromUInt(2)),
								Start: NewUCompactFromUInt(10),
							},
						},
					},
				},
			},
		},
	}
	x, err := EncodeToHex(exampleHeader)
	if err != nil {
		panic(err)
	}
	fmt.Println("head: ", x)
}
