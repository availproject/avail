# Data-Availability SubXt Library 

Facilities to use `subxt` to connect with a Data-Availability Node.

## Generate/Update `src/api_dev.rs`

Once node is updated, we can re-generate the `src/api_dev.rs` file using the following script:

**NOTE:**  The script requires a running node at `localhost:9944`

```Bash
#> ./build_api.sh 
```

### Troubleshooting

1. If you encounter any warnings or errors related to 'sed' while running the script on macOS, please ensure that you have [gnu-sed](https://medium.com/@bramblexu/install-gnu-sed-on-mac-os-and-set-it-as-default-7c17ef1b8f64) installed.

2. If you're not receiving any error messages, but the code generation process is not producing any output, consider running the individual code generation commands separately to check for any error messages.

# E2E Binary

This binary executes each example on a Data-Availability Node. It launches the node before running any example.

The following command uses the node located in `./target/release/data-avail` to run each example

```Bash
$> cargo run -- --avail-path ../target/release/data-avail
```
