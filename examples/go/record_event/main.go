package main

import (
	"fmt"
	"log"
	"math/big"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry"
	. "github.com/centrifuge/go-substrate-rpc-client/v4/registry/retriever"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry/state"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

func main() {

	api, err := gsrpc.NewSubstrateAPI("wss://turing-rpc.avail.so/")
	if err != nil {
		panic(fmt.Sprintf("cannot create api client:%v", err))
	}
	retriever, err := NewDefaultEventRetriever(state.NewEventProvider(api.RPC.State), api.RPC.State)

	if err != nil {
		fmt.Printf("Couldn't create event retriever: %s", err)
		return
	}
	// meta, err := api.RPC.State.GetMetadataLatest()
	// if err != nil {
	// 	fmt.Errorf("cannot get metadata:%w", err)
	// }
	// fmt.Println(meta)
	// prop, _ := api.RPC.System.Properties()
	// fmt.Println(prop)

	h, _ := types.NewHashFromHexString("0x31c31825a80ac53544208e6135a583a5dfa1aef7bf50f79b363d82d3b6b3392a")
	// key, err := types.CreateStorageKey(meta, "System", "Events", nil, nil)
	// if err != nil {
	// 	log.Fatalf("Failed to create storage key: %v", err)
	// }
	// rawEvents, err := api.RPC.State.GetStorageRaw(key, h)
	// if err != nil {
	// 	log.Fatalf("Failed to fetch events: %v", err)
	// }
	// events := types.EventRecords{}
	// err = types.EventRecordsRaw(*rawEvents).DecodeEventRecords(meta, &events)
	// if err != nil {
	// 	log.Fatalf("Failed to decode events: %v", err)
	// }
	// // fmt.Println(events)

	// if rawEvents != nil && len(*rawEvents) > 0 {
	// 	err = types.EventRecordsRaw(*rawEvents).DecodeEventRecords(meta, &events)
	// 	if err != nil {
	// 		log.Fatalf("Failed to decode events: %v", err)
	// 	}

	// 	for _, e := range events.Utility_ItemCompleted {

	// 		fmt.Printf("Transfer event: %v\n", e)

	// 	}

	// } else {
	// 	fmt.Println("No events found in the block")
	// }
	events, err := retriever.GetEvents(h)

	if err != nil {
		log.Printf("Couldn't retrieve events")
		return
	}

	// log.Printf("Found %d events for '%s', at block number %d.\n", len(events), testURL, header.Number)

	// Example of the events returned structure
	for _, event := range events {
		if event.Name == "Balances.Transfer" {
			// log.Printf("Event ID: %x \n", event.EventID)
			// log.Printf("Event Name: %s \n", event.Name)
			// log.Printf("Event Fields Count: %d \n", len(event.Fields))
			// for k, v := range event.Fields {
			// 	log.Printf("Field Name: %d \n", k)
			// 	log.Printf("Field Type: %v \n", reflect.TypeOf(v))
			// 	log.Printf("Field Value: %v \n", v)
			// 	log.Println("testing registery events")
			// 	if k != 2 {
			// 		w, err := registry.GetDecodedFieldAsType[registry.DecodedFields](
			// 			event.Fields,
			// 			func(fieldIndex int, field *registry.DecodedField) bool {
			// 				return fieldIndex == k
			// 			},
			// 		)
			// 		if err != nil {
			// 			fmt.Printf("Balances.Deposit.Who: %s\n", err.Error())
			// 		}
			// 		fmt.Println(w)
			// 	} else {
			// 		w, err := registry.GetDecodedFieldAsType[types.U128](
			// 			event.Fields,
			// 			func(fieldIndex int, field *registry.DecodedField) bool {
			// 				return fieldIndex == k
			// 			},
			// 		)
			// 		if err != nil {
			// 			fmt.Printf("Balances.Deposit.Who: %s\n", err.Error())
			// 		}
			// 		fmt.Println(w)
			// 	}
			// }
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
			fmt.Println(a)

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
			fmt.Println(to.ToHexString())
			amount, err := registry.GetDecodedFieldAsType[types.U128](
				event.Fields,
				func(fieldIndex int, field *registry.DecodedField) bool {
					return fieldIndex == 2
				},
			)
			if err != nil {
				fmt.Printf("Amount parsing err: %s\n", err.Error())
			}
			fmt.Printf("Amount: %s \n", convInt(amount.String()))
			if err != nil {
				fmt.Printf("Balances.Deposit.Who: %s\n", err.Error())
			}
		}
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
			fmt.Printf("Amount: %s \n", convInt(amount.String()))
			if err != nil {
				fmt.Printf("Balances.Deposit.Who: %s\n", err.Error())
			}
		}
	}
}

func convInt(val string) string {
	bigIntValue := new(big.Int)
	bigIntValue.SetString(val, 10)

	divisor := new(big.Int)
	divisor.SetString("1000000000000000000", 10)

	bigFloatValue := new(big.Float).SetInt(bigIntValue)
	divisorFloat := new(big.Float).SetInt(divisor)
	result := new(big.Float).Quo(bigFloatValue, divisorFloat)

	x := (result.Text('f', 18))
	return x
}
