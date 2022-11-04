# Data-Availability SubXt Library 

Facilities to use `subxt` to connect with a Data-Availability Node.

## Generate/Update `src/api_dev.rs`

Once node is updated, we can re-generate the `src/api_dev.rs` file using the following script:

**NOTE:**  The script requires a running node at `localhost:9933`

```Bash
#> ./build_api.sh 
```

# E2E Binary

This binary executes each example on a Data-Availability Node. It launches the node before run any
example.

The following command uses the node located in `./target/release/data-avail` to run each example

```Bash
$> cargo run -- --avail-path ../target/release/data-avail
```
