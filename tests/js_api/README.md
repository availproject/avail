## Avail polkadot-js script

go to the js_api folder in avail/tests

```
cd avail/tests/js_api/src
```

install dependencies 

```
npm i
```

install ts-node

```
npm i ts-node
```

>Creation of app_id

```
 ts-node app_id.ts -i 10
 ```
 Here `i` is the app_id that we would like to create(default is 1)

then run the script 

 ``` 
 ts-node full.ts [OPTIONS]
 ```
 where otpions are :

 ```
  -e, --endpoint  WSS endpoint                          [string] [default: "wss://testnet.polygonavail.net/ws"]
  -s, --payload   payload to be given in bytes          [number] [default: 100]
  -b, --batch     batch size of transactions            [number] [default: 3]
  -n, --function  function name                         [string] [default: "submit_data"]
  -i, --app_id    app id to be given                    [number] [default: 0]
  ```
  
  or you can run 
```
ts-node full.ts --help
```

`submit_data` and `bulk_tx` are the params for `-n`

