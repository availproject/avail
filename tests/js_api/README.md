## Avail polkadot-js script
Sample scripts to send transactions to Avail using Polkadot JS. The following has been tested on `node v16.16.0 (npm v8.11.0)`. 

- Go to the js_api folder in avail/tests

    ```
    cd avail/tests/js_api/src
    ```

- Install dependencies 

    ```
    npm i
    ```

- Install ts-node
    You can install globally or use `npx`. 
    ```
    npm i -g ts-node
    ```
- Create a `config.json` file
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



- Creation of app_id
    ```
    ts-node app_id.ts -i "10"
    ```
    Here `i` is the app_id that we would like to create(default is 1). App Id needs to be created before mentioning it in config file. But the app_id which is mentioned in the config is the index of the app_id which is stored. You can check the app_id by checking in the explorer.
    
- For submitting random blobs of data, use:
    ``` 
    ts-node data_submit.ts
    ```

- For subsribing to new blocks
    ```
    ts-node subscribe.ts 
    ```

- For balance transfer from one account to another use: 

    ```
    ts-node transfer.ts
    ```
    Script sends from the account whose mnemonic is mentioned on config to the receiver mentioned on config. 
