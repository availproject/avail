module avail-gsrpc-examples

go 1.16

require (
	github.com/centrifuge/go-substrate-rpc-client v2.0.0+incompatible
	github.com/centrifuge/go-substrate-rpc-client/v4 v4.0.12
	github.com/vedhavyas/go-subkey/v2 v2.0.0
	go.uber.org/ratelimit v0.2.0
)

replace github.com/centrifuge/go-substrate-rpc-client/v4 => github.com/availproject/go-substrate-rpc-client/v4 v4.0.12-avail-1.4.0-rc1-5e286e3
