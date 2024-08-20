package rpc

import (
	"encoding/json"
	"fmt"
	"math/big"
	"strings"
)

const MaxCells = 10000

type Cell struct {
	Row uint64 `json:"row"`
	Col uint64 `json:"col"`
}

func NewCell(row, col uint64) Cell {
	return Cell{Row: row, Col: col}
}

type GDataProof struct {
	RawScalar big.Int `json:"RawScalar"` // For U256
	Proof     GProof  `json:"Proof"`
}

type GProof struct {
	Data [48]byte `json:"Data"`
}

func (g *GDataProof) UnmarshalJSON(data []byte) error {
	var tupleData [2]json.RawMessage

	if err := json.Unmarshal(data, &tupleData); err != nil {
		return err
	}

	// Unmarshal RawScalar and remove the '0x' prefix
	var rawScalarString string
	if err := json.Unmarshal(tupleData[0], &rawScalarString); err != nil {
		return err
	}

	fmt.Println("RawScalarString:", rawScalarString)

	// Strip '0x' prefix and convert to big.Int
	trimmedScalarString := strings.TrimPrefix(rawScalarString, "0x")
	rawScalar, ok := new(big.Int).SetString(trimmedScalarString, 16)
	if !ok {
		fmt.Printf("Failed to convert RawScalar to big.Int, string was: %s\n", trimmedScalarString)
		return fmt.Errorf("invalid RawScalar format")
	}
	g.RawScalar = *rawScalar

	// Unmarshal Proof
	var proof [48]byte
	if err := json.Unmarshal(tupleData[1], &proof); err != nil {
		return err
	}
	g.Proof = GProof{proof}

	return nil
}

func formatBigIntWithCommas(b *big.Int) string {
	if b == nil {
		return ""
	}

	numStr := b.String()

	// Starting index for inserting commas
	startOffset := 0
	if numStr[0] == '-' {
		startOffset = 1 // Keep the negative sign intact
	}

	// Slice to hold the parts of the number
	var parts []string

	// Iterate over the string in reverse, collecting slices of 3 digits
	for i := len(numStr); i > startOffset; i -= 3 {
		end := i
		start := i - 3
		if start < startOffset {
			start = startOffset
		}
		parts = append([]string{numStr[start:end]}, parts...)
	}

	// Join the parts with commas
	return strings.Join(parts, ",")
}

func (g *GDataProof) MarshalJSON() ([]byte, error) {
	rawScalarStr := formatBigIntWithCommas(&g.RawScalar)
	proofHex := fmt.Sprintf("0x%x", g.Proof.Data)
	return json.Marshal([]interface{}{rawScalarStr, proofHex})
}
