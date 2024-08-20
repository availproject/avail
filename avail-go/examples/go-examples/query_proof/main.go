package main

import (
	"avail-go-sdk/rpc"
	"avail-go-sdk/sdk"
	"avail-go-sdk-examples/internal/config"
	"encoding/json"
	"flag"
	"fmt"
	"log"
	"os"

	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

// const MaxCells = 10000

// type Cell struct {
// 	Row uint64 `json:"row"`
// 	Col uint64 `json:"col"`
// }

// func NewCell(row, col uint64) Cell {
// 	return Cell{Row: row, Col: col}
// }

// type GDataProof struct {
// 	RawScalar big.Int `json:"RawScalar"` // For U256
// 	Proof     GProof  `json:"Proof"`
// }

// type GProof struct {
// 	Data [48]byte `json:"Data"`
// }

// func (g *GDataProof) UnmarshalJSON(data []byte) error {
// 	var tupleData [2]json.RawMessage

// 	if err := json.Unmarshal(data, &tupleData); err != nil {
// 		return err
// 	}

// 	// Unmarshal RawScalar and remove the '0x' prefix
// 	var rawScalarString string
// 	if err := json.Unmarshal(tupleData[0], &rawScalarString); err != nil {
// 		return err
// 	}

// 	fmt.Println("RawScalarString:", rawScalarString)

// 	// Strip '0x' prefix and convert to big.Int
// 	trimmedScalarString := strings.TrimPrefix(rawScalarString, "0x")
// 	rawScalar, ok := new(big.Int).SetString(trimmedScalarString, 16)
// 	if !ok {
// 		fmt.Printf("Failed to convert RawScalar to big.Int, string was: %s\n", trimmedScalarString)
// 		return fmt.Errorf("invalid RawScalar format")
// 	}
// 	g.RawScalar = *rawScalar

// 	// Unmarshal Proof
// 	var proof [48]byte
// 	if err := json.Unmarshal(tupleData[1], &proof); err != nil {
// 		return err
// 	}
// 	g.Proof = GProof{proof}

// 	return nil
// }
// func formatBigIntWithCommas(b *big.Int) string {
// 	if b == nil {
// 		return ""
// 	}

// 	numStr := b.String()

// 	// Starting index for inserting commas
// 	startOffset := 0
// 	if numStr[0] == '-' {
// 		startOffset = 1 // Keep the negative sign intact
// 	}

// 	// Slice to hold the parts of the number
// 	var parts []string

// 	// Iterate over the string in reverse, collecting slices of 3 digits
// 	for i := len(numStr); i > startOffset; i -= 3 {
// 		end := i
// 		start := i - 3
// 		if start < startOffset {
// 			start = startOffset
// 		}
// 		parts = append([]string{numStr[start:end]}, parts...)
// 	}

// 	// Join the parts with commas
// 	return strings.Join(parts, ",")
// }

// func (g *GDataProof) MarshalJSON() ([]byte, error) {
// 	rawScalarStr := formatBigIntWithCommas(&g.RawScalar)
// 	proofHex := fmt.Sprintf("0x%x", g.Proof.Data)
// 	return json.Marshal([]interface{}{rawScalarStr, proofHex})
// }

func main() {
	var configJSON string
	var config config.Config
	flag.StringVar(&configJSON, "config", "", "config json file")
	flag.Parse()

	if configJSON == "" {
		log.Println("No config file provided. Exiting...")
		os.Exit(0)
	}

	err := config.GetConfig(configJSON)
	if err != nil {
		panic(fmt.Sprintf("cannot get config:%v", err))
	}
	api, err := sdk.NewSDK(config.ApiURL)
	if err != nil {
		panic(fmt.Sprintf("cannot create api client:%v", err))
	}
	if api == nil || api.Client == nil {
		log.Fatal("API client is not properly initialized")
	}

	var finalizedBlockCh = make(chan types.Hash)
	go func() {
		err = sdk.SubmitData(*api, "data", config.Seed, 1, finalizedBlockCh)
		if err != nil {
			panic(fmt.Sprintf("cannot submit data:%v", err))
		}
	}()

	// block hash to query proof
	blockHash := <-finalizedBlockCh
	fmt.Printf("Transaction included in finalized block: %v\n", blockHash.Hex())
	h, err := types.NewHashFromHexString(blockHash.Hex())
	if err != nil {
		panic(fmt.Sprintf("cannot create api client:%v", err))
	}

	defer func() {
		if r := recover(); r != nil {
			fmt.Println("Recovered in main", r)
		}
	}()
	var result []rpc.GDataProof
	testCell := []rpc.Cell{rpc.NewCell(0, 0), rpc.NewCell(0, 1)}

	testCellJSON, _ := json.Marshal(testCell)
	fmt.Println("Test Cell JSON:", string(testCellJSON))

	err = api.Client.Call(&result, "kate_queryProof", testCell, h)
	if err != nil {
		fmt.Println("Error calling api.Client.Call:", err)
	}

	resultJSON, err := json.Marshal(result)
	if err != nil {
		fmt.Println("Error marshalling result to JSON:", err)
		return
	}

	fmt.Println(string(resultJSON))
}
