## Avail typescript examples

Sample examples written in typescript that demonstrate interaction with Avail network.
The following scripts have been tested with `ts-node v10.9.1 node v16.19.0 (npm v8.19.3)`.

- Go to the `examples` folder.

    ```
    cd examples/ts/src
    ```

- Install dependencies.

    ```
    npm i
    ```

- Install ts-node
  You can install globally or use `npx`.
    ```
    npm i -g ts-node
    ```

- Make sure to populate basic configuration in `config.ts` and also, take a look at the scripts for more details about
  the configuration.

```typescript
export default {
    mnemonic: "", // The secret seed value for account used to sign transactions 
    ApiURL: "",   // Api url
    app_id: 0,    // Application id 
    amount: 0,    // Amount of tokens to transfer
    receiver: ""  // Receiver address
}
```

- Connect to a node and display some basic information.
    ```
    ts-node connect.ts 
    ```
- Subscribes to new blocks and displays block number.
    ```
    ts-node listen_new_blocks.ts 
    ```
- Transferring tokens to some account (script sends from the account whose mnemonic is mentioned on config to the
  receiver mentioned on config).
    ```
    ts-node transfer.ts
    ```
- Transferring tokens to some account with status tracking until block is finalized.
    ```
    ts-node transfer_status.ts
    ```
- For submitting blob of data (make sure to populate data variable in the script):
    ``` 
    ts-node data_submit.ts
    ```
- Dispatching data root, more details can be found in the documentation about [Validiums](..%2Fvalidium%2FReadme.md) .
    ``` 
    ts-node dispatch_data_root.ts  
    ```
- Query data proof returns a Merkle proof for the particular block and data index.
    ```
    ts-node query_proof_data.ts
    ```
- Query proof for the given row/col and block hash.
    ```
    ts-node query_proof.ts
    ```
- Query application data returns data fot the given application id and block hash.
    ```
    ts-node ts-node query_app_data.ts
    ```
- Creation of app_id
    ```
    ts-node app_id.ts -i "10"
    ```
  Here `i` is the app_id that we would like to create(default is 1).
- Submit a proposal using the council.
    ```
    ts-node ts-node submit_proposal.ts
    ```
You can also use yarn commands to run the scripts. For example 
- Start by building the project:
    ```
    npm run build
    ```

- Then run the commands:
    ```
    npm run app_id -i "test"
    npm run connect
    npm run data_submit
    npm run dispatch_root
    npm run listen
    npm run query_app
    npm run query_proof
    npm run submit_proposal
    npm run transfer
    npm run transfer:status
    ```
