## Examples

Before running examples or benchmarks make sure that you have a running local node.

```bash
# To run the Connect example
deno run -A ./examples/deno/connect.ts

# To run the Data Submit example
deno run -A ./examples/deno/data_submit.ts

# To run the Query Proof example
deno run -A ./examples/deno/query_proof.ts

# To run the Header Subscription example
deno run -A ./examples/deno/subscribe_to_header.ts

# To run the Transfer example
deno run -A ./examples/deno/transfer.ts
```

## Benchmarks
Make sure that you have a running local node with `--features kate-rpc-metrics` enabled.

```bash
cargo build --release --features kate-rpc-metrics
```

Make sure to populate basic configuration in `benchmarks/config.ts` or leave the default if using local node. Take a look at the scripts for more details about the configuration.

```typescript
export default {
    seed: "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice",
    endpoint: "ws://127.0.0.1:9944",
    txCount: 100,
    jobCount: 10,
}
```

```bash
# To benchmark Query Proof RPC
deno run -A ./examples/deno/benchmarks/query_proof.ts

# To benchmark Query Data Proof RPC
deno run -A ./examples/deno/benchmarks/query_data_proof.ts

# To benchmark Query Data Proof V2 RPC
deno run -A ./examples/deno/benchmarks/query_data_proof_v2.ts

# To benchmark Query Rows RPC
deno run -A ./examples/deno/benchmarks/query_rows.ts

# To benchmark Query Block Length RPC
deno run -A ./examples/deno/benchmarks/query_block_length.ts

# To benchmark Query App Data RPC
deno run -A ./examples/deno/benchmarks/query_app_data.ts
```