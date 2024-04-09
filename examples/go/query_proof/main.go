package main

import (
	"avail-gsrpc-examples/internal/config"
	"avail-gsrpc-examples/internal/extrinsics"
	"flag"
	"fmt"
	"log"
	"math/big"
	"os"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

// Tuple represents the Rust tuple (U256, [u8; 48]).
const MaxCells = 10000

// Cell represents a row and column.
type Cell struct {
	Row uint32
	Col uint32
}

type CellsWithMax struct {
	Cells   []Cell
	MaxCell uint32
}

// BlockLengthRows represents the row.
type BlockLengthRows struct {
	Value types.U32
}

// BlockLengthColumns represents the column.
type BlockLengthColumns struct {
	Value types.U32
}

// GDataProof represents the proof data.
type GDataProof struct {
	RawScalar GRawScalar
	Proof     GProof
}

// GProof represents the proof.
type GProof struct {
	Data [48]byte
}

// GRawScalar represents a 256-bit number.
type GRawScalar big.Int

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
	api, err := gsrpc.NewSubstrateAPI(config.ApiURL)
	if err != nil {
		panic(fmt.Sprintf("cannot create api client:%v", err))
	}
	if api == nil || api.Client == nil {
		log.Fatal("API client is not properly initialized")
	}

	var finalizedBlockCh = make(chan types.Hash)
	go func() {
		err = extrinsics.SubmitData(api, "data", config.Seed, 1, finalizedBlockCh)
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

	// defer func() {
	// 	if r := recover(); r != nil {
	// 		fmt.Println("Recovered in main", r)
	// 	}
	// }()
	var result []GDataProof

	// Populate the Celled slice
	// cellsWithMax := CellsWithMax{
	// 	Cells:   []Cell{{Row: 0, Col: 0}, {Row: 0, Col: 1}},
	// 	MaxCell: 64,
	// }
	cells := []Cell{{Row: 0, Col: 0}}

	fmt.Println(cells)
	err = api.Client.Call(&result, "kate_queryProof", cells, h)
	if err != nil {
		fmt.Println("Error calling api.Client.Call:", err)
	}
	fmt.Println(result)

}
