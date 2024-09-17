package sdk

import (
	"encoding/hex"
	"fmt"

	"log"
	"math/big"

	"github.com/centrifuge/go-substrate-rpc-client/v4/registry"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry/parser"
	retriever "github.com/centrifuge/go-substrate-rpc-client/v4/registry/retriever"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry/state"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/vedhavyas/go-subkey"
)

func EventParser(api *SubstrateAPI, h types.Hash, eventParse string) {
	retriever, err := retriever.NewDefaultEventRetriever(state.NewEventProvider(api.RPC.State), api.RPC.State)
	if err != nil {
		log.Printf("Couldn't create event retriever")
	}

	events, err := retriever.GetEvents(h)

	if err != nil {
		log.Printf("Couldn't retrieve events")
	}

	switch eventParse {
	case "DataSubmitted":
		ParseDataSubmitted(events)
	case "ApplicationKeyCreated":
		parseApplicationKeyCreated(events)
	case "ApplicationKeySet":
		parseApplicationKeySet(events)
	case "BalanceTransfer":
		parseBalanceTransfer(events)
	case "Bond":
		parseBond(events)
	}
}
func ParseDataSubmitted(events []*parser.Event) {
	for _, event := range events {
		if event.Name == "DataAvailability.DataSubmitted" {
			from, _ := registry.ProcessDecodedFieldValue[*types.AccountID](
				event.Fields,
				func(fieldIndex int, field *registry.DecodedField) bool {

					return field.Name == "sp_core.crypto.AccountId32.who"
				},
				func(value any) (*types.AccountID, error) {
					fields, ok := value.(registry.DecodedFields)

					if !ok {
						return nil, fmt.Errorf("unexpected value: %v", value)
					}

					accByteSlice, err := registry.GetDecodedFieldAsSliceOfType[types.U8](fields, func(fieldIndex int, field *registry.DecodedField) bool {
						return fieldIndex == 0
					})

					if err != nil {
						return nil, err
					}

					var accBytes []byte

					for _, accByte := range accByteSlice {
						accBytes = append(accBytes, byte(accByte))
					}

					return types.NewAccountID(accBytes)
				},
			)
			// a := from.ToHexString()

			// // add, _ := types.NewAddressFromHexAccountID(a)
			// fmt.Println(from)
			// fmt.Printf("from address read from event: %s \n", a)
			has := subkey.SS58Encode(from.ToBytes(), 42)
			fmt.Printf("from address read from event: %s \n", has)
			dataHash, err := registry.ProcessDecodedFieldValue[*types.Hash](
				event.Fields,
				func(fieldIndex int, field *registry.DecodedField) bool {
					return fieldIndex == 1
				},
				func(value any) (*types.Hash, error) {
					fields, ok := value.(registry.DecodedFields)
					if !ok {
						return nil, fmt.Errorf("unexpected value: %v", value)
					}

					hashByteSlice, err := registry.GetDecodedFieldAsSliceOfType[types.U8](fields, func(fieldIndex int, field *registry.DecodedField) bool {
						return fieldIndex == 0
					})

					if err != nil {
						return nil, err
					}

					var hashBytes []byte
					for _, hashByte := range hashByteSlice {
						hashBytes = append(hashBytes, byte(hashByte))
					}

					hash := types.NewHash(hashBytes)
					return &hash, nil
				},
			)
			if err != nil {
				fmt.Printf("DataHash parsing err: %s\n", err.Error())
			} else if dataHash == nil {
				fmt.Println("DataHash is nil")
			} else {
				fmt.Printf("DataHash read from event: %s \n", dataHash.Hex())
			}

		}
		parseTransactionFee(event)
	}
}

func parseTransactionFee(event *parser.Event) {
	if event.Name == "TransactionPayment.TransactionFeePaid" {

		amount, err := registry.GetDecodedFieldAsType[types.U128](
			event.Fields,
			func(fieldIndex int, field *registry.DecodedField) bool {
				return fieldIndex == 1
			},
		)
		if err != nil {
			fmt.Printf("Amount parsing err: %s\n", err.Error())
		}
		fmt.Printf("Actual Fee from TransactionPayment.TransactionFeePaid event: %s \n", convInt(amount.String()))
		if err != nil {
			fmt.Printf("Balances.Deposit.Who: %s\n", err.Error())
		}
	}

}

func parseApplicationKeyCreated(events []*parser.Event) {
	for _, event := range events {
		if event.Name == "DataAvailability.ApplicationKeyCreated" {
			owner, _ := registry.ProcessDecodedFieldValue[*types.AccountID](
				event.Fields,
				func(fieldIndex int, field *registry.DecodedField) bool {

					return field.Name == "sp_core.crypto.AccountId32.owner"
				},
				func(value any) (*types.AccountID, error) {
					fields, ok := value.(registry.DecodedFields)

					if !ok {
						return nil, fmt.Errorf("unexpected value: %v", value)
					}

					accByteSlice, err := registry.GetDecodedFieldAsSliceOfType[types.U8](fields, func(fieldIndex int, field *registry.DecodedField) bool {
						return fieldIndex == 0
					})

					if err != nil {
						return nil, err
					}

					var accBytes []byte

					for _, accByte := range accByteSlice {
						accBytes = append(accBytes, byte(accByte))
					}

					return types.NewAccountID(accBytes)
				},
			)
			// a := from.ToHexString()

			// // add, _ := types.NewAddressFromHexAccountID(a)
			// fmt.Println(from)
			// fmt.Printf("from address read from event: %s \n", a)
			has := subkey.SS58Encode(owner.ToBytes(), 42)
			fmt.Printf("from address read from event: %s \n", has)

			id, err := registry.ProcessDecodedFieldValue[types.UCompact](
				event.Fields,
				func(fieldIndex int, field *registry.DecodedField) bool {
					return fieldIndex == 2
				},
				func(value any) (types.UCompact, error) {
					fields, ok := value.(registry.DecodedFields)
					if !ok {
						return types.NewUCompact(big.NewInt(0)), fmt.Errorf("unexpected value type: %T", value)
					}

					// Assuming the UCompact value is the first (and only) field in this struct
					ucompact, err := registry.GetDecodedFieldAsType[types.UCompact](fields, func(fieldIndex int, field *registry.DecodedField) bool {
						return fieldIndex == 0
					})

					if err != nil {
						return types.NewUCompact(big.NewInt(0)), fmt.Errorf("error getting UCompact field: %w", err)
					}

					return ucompact, nil
				},
			)
			if err != nil {
				fmt.Printf("AppId parsing err: %s\n", err.Error())
			} else {
				fmt.Println(id.Int64())
			}

			appKey, err := registry.ProcessDecodedFieldValue[types.Bytes](
				event.Fields,
				func(fieldIndex int, field *registry.DecodedField) bool {
					return fieldIndex == 0 // Adjust this index if needed
				},
				func(value any) (types.Bytes, error) {
					fields, ok := value.(registry.DecodedFields)
					if !ok {
						return nil, fmt.Errorf("unexpected value type: %T", value)
					}

					// The BoundedVec is likely represented as a slice of U8
					byteSlice, err := registry.GetDecodedFieldAsSliceOfType[types.U8](fields, func(fieldIndex int, field *registry.DecodedField) bool {
						return fieldIndex == 0
					})

					if err != nil {
						return nil, fmt.Errorf("error getting byte slice: %w", err)
					}

					// Convert the slice of U8 to types.Bytes
					bytes := make(types.Bytes, len(byteSlice))
					for i, b := range byteSlice {
						bytes[i] = byte(b)
					}

					return bytes, nil
				},
			)

			if err != nil {
				fmt.Printf("AppKey parsing err: %s\n", err.Error())
			} else {
				// Convert Bytes to string for printing
				stringValue := string(appKey)
				fmt.Printf("AppKey from event (as string): %s\n", stringValue)

				// If you also want to see the hex representation:
				hexString := hex.EncodeToString(appKey)
				fmt.Printf("AppKey from event (as hex): 0x%s\n", hexString)
			}

		}
		parseTransactionFee(event)
	}
}

func parseApplicationKeySet(events []*parser.Event) {
	for _, event := range events {
		if event.Name == "DataAvailability.ApplicationKeySet" {
			owner, _ := registry.ProcessDecodedFieldValue[*types.AccountID](
				event.Fields,
				func(fieldIndex int, field *registry.DecodedField) bool {

					return field.Name == "sp_core.crypto.AccountId32.owner"
				},
				func(value any) (*types.AccountID, error) {
					fields, ok := value.(registry.DecodedFields)

					if !ok {
						return nil, fmt.Errorf("unexpected value: %v", value)
					}

					accByteSlice, err := registry.GetDecodedFieldAsSliceOfType[types.U8](fields, func(fieldIndex int, field *registry.DecodedField) bool {
						return fieldIndex == 0
					})

					if err != nil {
						return nil, err
					}

					var accBytes []byte

					for _, accByte := range accByteSlice {
						accBytes = append(accBytes, byte(accByte))
					}

					return types.NewAccountID(accBytes)
				},
			)
			// a := from.ToHexString()

			// // add, _ := types.NewAddressFromHexAccountID(a)
			// fmt.Println(from)
			// fmt.Printf("from address read from event: %s \n", a)
			has := subkey.SS58Encode(owner.ToBytes(), 42)
			fmt.Printf("from address read from event: %s \n", has)

			id, err := registry.ProcessDecodedFieldValue[types.UCompact](
				event.Fields,
				func(fieldIndex int, field *registry.DecodedField) bool {
					return fieldIndex == 2
				},
				func(value any) (types.UCompact, error) {
					fields, ok := value.(registry.DecodedFields)
					if !ok {
						return types.NewUCompact(big.NewInt(0)), fmt.Errorf("unexpected value type: %T", value)
					}

					// Assuming the UCompact value is the first (and only) field in this struct
					ucompact, err := registry.GetDecodedFieldAsType[types.UCompact](fields, func(fieldIndex int, field *registry.DecodedField) bool {
						return fieldIndex == 0
					})

					if err != nil {
						return types.NewUCompact(big.NewInt(0)), fmt.Errorf("error getting UCompact field: %w", err)
					}

					return ucompact, nil
				},
			)
			if err != nil {
				fmt.Printf("AppId parsing err: %s\n", err.Error())
			} else {
				fmt.Println(id.Int64())
			}

			appKey, err := registry.ProcessDecodedFieldValue[types.Bytes](
				event.Fields,
				func(fieldIndex int, field *registry.DecodedField) bool {
					return fieldIndex == 0 // Adjust this index if needed
				},
				func(value any) (types.Bytes, error) {
					fields, ok := value.(registry.DecodedFields)
					if !ok {
						return nil, fmt.Errorf("unexpected value type: %T", value)
					}

					// The BoundedVec is likely represented as a slice of U8
					byteSlice, err := registry.GetDecodedFieldAsSliceOfType[types.U8](fields, func(fieldIndex int, field *registry.DecodedField) bool {
						return fieldIndex == 0
					})

					if err != nil {
						return nil, fmt.Errorf("error getting byte slice: %w", err)
					}

					// Convert the slice of U8 to types.Bytes
					bytes := make(types.Bytes, len(byteSlice))
					for i, b := range byteSlice {
						bytes[i] = byte(b)
					}

					return bytes, nil
				},
			)

			if err != nil {
				fmt.Printf("AppKey parsing err: %s\n", err.Error())
			} else {
				// Convert Bytes to string for printing
				stringValue := string(appKey)
				fmt.Printf("AppKey from event (as string): %s\n", stringValue)

				// If you also want to see the hex representation:
				hexString := hex.EncodeToString(appKey)
				fmt.Printf("AppKey from event (as hex): 0x%s\n", hexString)
			}

		}
		parseTransactionFee(event)
	}
}

func parseBalanceTransfer(events []*parser.Event) {
	for _, event := range events {
		if event.Name == "Balances.Transfer" {
			from, err := registry.ProcessDecodedFieldValue[*types.AccountID](
				event.Fields,
				func(fieldIndex int, field *registry.DecodedField) bool {

					return field.Name == "sp_core.crypto.AccountId32.from"
				},
				func(value any) (*types.AccountID, error) {
					fields, ok := value.(registry.DecodedFields)

					if !ok {
						return nil, fmt.Errorf("unexpected value: %v", value)
					}

					accByteSlice, err := registry.GetDecodedFieldAsSliceOfType[types.U8](fields, func(fieldIndex int, field *registry.DecodedField) bool {
						return fieldIndex == 0
					})

					if err != nil {
						return nil, err
					}

					var accBytes []byte

					for _, accByte := range accByteSlice {
						accBytes = append(accBytes, byte(accByte))
					}

					return types.NewAccountID(accBytes)
				},
			)
			a := from.ToHexString()

			// // add, _ := types.NewAddressFromHexAccountID(a)
			// fmt.Println(from)
			fmt.Printf("from address read from event: %s \n", a)

			to, err := registry.ProcessDecodedFieldValue[*types.AccountID](
				event.Fields,

				func(fieldIndex int, field *registry.DecodedField) bool {

					return field.Name == "sp_core.crypto.AccountId32.to"
				},
				func(value any) (*types.AccountID, error) {
					fields, ok := value.(registry.DecodedFields)

					if !ok {
						return nil, fmt.Errorf("unexpected value: %v", value)
					}

					accByteSlice, err := registry.GetDecodedFieldAsSliceOfType[types.U8](fields, func(fieldIndex int, field *registry.DecodedField) bool {
						return fieldIndex == 0
					})

					if err != nil {
						return nil, err
					}

					var accBytes []byte

					for _, accByte := range accByteSlice {
						accBytes = append(accBytes, byte(accByte))
					}

					return types.NewAccountID(accBytes)
				},
			)
			if err != nil {
				fmt.Printf("TO parsing err: %s\n", err.Error())
			}
			fmt.Printf("To address read from event: %s \n", to.ToHexString())
			amount, err := registry.GetDecodedFieldAsType[types.U128](
				event.Fields,
				func(fieldIndex int, field *registry.DecodedField) bool {
					return fieldIndex == 2
				},
			)
			if err != nil {
				fmt.Printf("Amount parsing err: %s\n", err.Error())
			}
			fmt.Printf("Amount transferred : %s \n", convInt(amount.String()))
			if err != nil {
				fmt.Printf("Balances.Deposit.Who: %s\n", err.Error())
			}
		}
		parseTransactionFee(event)
	}
}

func parseBond(events []*parser.Event) {
	for _, event := range events {
		if event.Name == "Staking.Bonded" {
			from, err := registry.ProcessDecodedFieldValue[*types.AccountID](
				event.Fields,
				func(fieldIndex int, field *registry.DecodedField) bool {

					return field.Name == "sp_core.crypto.AccountId32.stash"
				},
				func(value any) (*types.AccountID, error) {
					fields, ok := value.(registry.DecodedFields)

					if !ok {
						return nil, fmt.Errorf("unexpected value: %v", value)
					}

					accByteSlice, err := registry.GetDecodedFieldAsSliceOfType[types.U8](fields, func(fieldIndex int, field *registry.DecodedField) bool {
						return fieldIndex == 0
					})

					if err != nil {
						return nil, err
					}

					var accBytes []byte

					for _, accByte := range accByteSlice {
						accBytes = append(accBytes, byte(accByte))
					}

					return types.NewAccountID(accBytes)
				},
			)
			a := from.ToHexString()

			// // add, _ := types.NewAddressFromHexAccountID(a)
			// fmt.Println(from)
			fmt.Printf("stash address read from event: %s \n", a)

			amount, err := registry.GetDecodedFieldAsType[types.U128](
				event.Fields,
				func(fieldIndex int, field *registry.DecodedField) bool {
					return fieldIndex == 1
				},
			)
			if err != nil {
				fmt.Printf("Amount parsing err: %s\n", err.Error())
			}
			fmt.Printf("Amount staked : %s \n", convInt(amount.String()))
			if err != nil {
				fmt.Printf("Amount staked: %s\n", err.Error())
			}
		}
		parseTransactionFee(event)
	}
}
