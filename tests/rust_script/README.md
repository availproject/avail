# Avail subxt based script 

Before we run the script you should take a look at the subxt steps for installing `subxt-cli` and to get `metadata`. 

```
cargo install subxt-cli
```

```
subxt metadata -f bytes > metadata.scale
```
This defaults to querying the metadata of a locally running node on the default http://localhost:9933/. If querying a different node then the metadata command accepts a --url argument.

#### Run the command 

```
cargo run --[OPTIONS] --mode <MODE>
```

```
OPTIONS:
    -h, --help           Print help information
    -m, --mode <MODE>
    -n, --num <NUM>      number of transactions to be sent in bulk transaction mode
    -s, --size <SIZE>    size of each transaction
```

`submit_data` and `submit_bulk_data` are difference mode

