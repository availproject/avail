package rpc

import (
	"avail-go-sdk/src/extrinsic"
	"context"
	"fmt"
	"sync"

	"github.com/centrifuge/go-substrate-rpc-client/v4/client"
	"github.com/centrifuge/go-substrate-rpc-client/v4/config"
	gethrpc "github.com/centrifuge/go-substrate-rpc-client/v4/gethrpc"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types/codec"
)

type ExtrinsicStatusSubscription struct {
	sub      *gethrpc.ClientSubscription
	channel  chan types.ExtrinsicStatus
	quitOnce sync.Once // ensures quit is closed once
}

// Chan returns the subscription channel.
//
// The channel is closed when Unsubscribe is called on the subscription.
func (s *ExtrinsicStatusSubscription) Chan() <-chan types.ExtrinsicStatus {
	return s.channel
}

// Err returns the subscription error channel. The intended use of Err is to schedule
// resubscription when the client connection is closed unexpectedly.
//
// The error channel receives a value when the subscription has ended due
// to an error. The received error is nil if Close has been called
// on the underlying client and no other error has occurred.
//
// The error channel is closed when Unsubscribe is called on the subscription.
func (s *ExtrinsicStatusSubscription) Err() <-chan error {
	return s.sub.Err()
}

// Unsubscribe unsubscribes the notification and closes the error channel.
// It can safely be called more than once.
func (s *ExtrinsicStatusSubscription) Unsubscribe() {
	s.sub.Unsubscribe()
	s.quitOnce.Do(func() {
		close(s.channel)
	})
}

// SubmitAndWatchExtrinsic will submit and subscribe to watch an extrinsic until unsubscribed, returning a subscription
// that will receive server notifications containing the extrinsic status updates.
func SubmitAndWatchExtrinsic(xt extrinsic.Extrinsic, client client.Client) (*ExtrinsicStatusSubscription, error) { //nolint:lll
	ctx, cancel := context.WithTimeout(context.Background(), config.Default().SubscribeTimeout)
	defer cancel()

	c := make(chan types.ExtrinsicStatus)

	enc, err := codec.EncodeToHex(xt)
	if err != nil {
		return nil, err
	}

	sub, err := client.Subscribe(ctx, "author", "submitAndWatchExtrinsic", "unwatchExtrinsic", "extrinsicUpdate",
		c, enc)
	if err != nil {
		return nil, err
	}

	return &ExtrinsicStatusSubscription{sub: sub, channel: c}, nil
}

func SubmitExtrinsic(xt extrinsic.Extrinsic, client client.Client) (types.Hash, error) {
	enc, err := codec.EncodeToHex(xt)

	if err != nil {
		return types.Hash{}, err
	}
	fmt.Println(enc)

	var res string
	err = client.Call(&res, "author_submitExtrinsic", enc)
	if err != nil {
		return types.Hash{}, err
	}

	return types.NewHashFromHexString(res)
}
