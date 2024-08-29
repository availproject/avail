package main

import (
	"fmt"

	"avail-go-sdk/src/sdk"

	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

func main() {

	api, err := sdk.NewSDK("ws://127.0.0.1:9944")
	if err != nil {
		panic(fmt.Sprintf("cannot create api client:%v", err))
	}

	h, _ := types.NewHashFromHexString("0xffebb53bcb405a1069d5a63d8f6a015429aeb903bd0e7decb06723aeaacca0f2")
	sdk.EventParser(api, h)
}
