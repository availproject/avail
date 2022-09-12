## Avail polkadot-js script
Sample scripts to send transactions to Avail using Polkadot JS. The following has been tested on `node v16.16.0 (npm v8.11.0)`. 

go to the js_api folder in avail/tests

```
cd avail/tests/js_api/src
```

install dependencies 

```
npm i
```

install ts-node
You can install globally or use `npx`. 

```
npm i -g ts-node
```
create a config.json file
```
touch config.json
```

```json

{
    "mnemonic" : "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice",
    "size" : 10,
    "ApiURL" : "ws://127.0.0.1:9944",
    "app_id" : 0,
    "batch" : 0, 
    "count" : 10,
    "receiver": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    "amount" : 100000
}
```
Keep in mind that the fields `batch`(default is 0 and if its specified then it switches to batch transaction mode), `count`(if not specified the subscription will continue infinitely) and `amount`(if not specified default is `12345`) are optional. If you do not want to use them, you can leave them empty.
Do Keep in mind that the receiver address should be specified when you are using transfer calls. The data will be send from the mnemonic address to the receiver. 



>Creation of app_id

```
 ts-node app_id.ts -i 10
 ```
 Here `i` is the app_id that we would like to create(default is 1). App Id needs to be created before mentioning it in config file. But the app_id which is mentioned in the config is the index of the app_id which is stored. You can check the app_id by checking in the explorer. 

then run the script 

 ``` 
 ts-node full.ts
 ```

>The following is the script for subsribing to new blocks
```
ts-node sub.ts 
```

>If you want to transer from a account to another use: (script default uses alice and bob accounts)

```
ts-node transfer.ts
```


