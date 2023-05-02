# Avail-GSRPC scripts for data submission

PS: Do keep in mind that the gsprc is not that stable like polakdot-js and subxt libs

### How to: 

1. create a json file and copy it into blockListener and loadTesting folder 

```json
{
    "seed":"bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice",
    "api_url":"ws://127.0.0.1:9944",
    "size":1000,
    "app_id" : 0,
    "dest" : "5H3qehpRTFiB3XwyTzYU43SpG7e8jW87vFug95fxdew76Vyi",
    "amount" : 10
}
```

2. There are multiple scripts you have in gsrpc

go to the gsrpc folder in tests 

```
cd avail/tests/gsrpc
```

> Using simple data submission 

``` 
 go run extrinsics/dataSubmission/dataSubmission.go -config config.json
```
here the size is amount of random amount of data to sent in bytes which is optional(default is 100). The default APP_ID is 0

>Using the blockListener

```
 go run extrinsics/blockListener/blockListener.go -config config.json
 ```
The same procedure of data submission is done here and also listens to the blocks using subscription for 10sec(can be modified in script)

>Using SubmitandWatch

```
go run extrinsics/dataSubmitAndWatch/dataSubmitAndWatch.go -config config.json
```
The same procedure of data submission is done here. But checks/logs if the data is included in the block. The default APP_ID is 0

>Balance Transfer

```
go run extrinsics/balance/balance.go -config config.json
```

Amount is transferred from the seed account to the dest account with the amount mentioned in config file. You can refer [here](https://docs.substrate.io/tutorials/build-a-blockchain/add-trusted-nodes/#generate-your-account-and-keys) on how to create or inspect account 

>LoadTesting(only for testing)

```
go run extrinsics/loadTestingTool/loadTestingTool.go -config config.json
```
Load testing script to sent randomly generated data for 10 seconds.

