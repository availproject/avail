# Data-Availability SubXt Library 

Facilities to use `subxt` to connect with a Data-Availability Node.

# E2E Binary

This binary executes each example on a Data-Availability Node. It launches the node before run any
example.

The following command uses the node located in `./target/release/data-avail` to run each example

```Bash
$> cargo run -- --avail-path ../target/release/data-avail
```
