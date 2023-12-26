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
- Make sure to populate basic configuration in `benchmarks/config.ts`
```typescript
export default {
    mnemonic: "", // The secret seed value for account used to sign transactions. //Alice is used by default
    ApiURL: "",   // Api url
    txCount: 100, // Count of submitting transactions
    jobCount: 10, // Count of tasks
}
```

```bash
# To benchmark Query Proof RPC
deno run -A ./examples/deno/benchmarks/query_proof.ts

# To benchmark Query Data Proof RPC
deno run -A ./examples/deno/benchmarks/query_data_proof.ts

# To benchmark Query Rows RPC
deno run -A ./examples/deno/benchmarks/query_rows.ts

# To benchmark Query Block Length RPC
deno run -A ./examples/deno/benchmarks/query_block_length.ts

# To benchmark Query App Data RPC
deno run -A ./examples/deno/benchmarks/query_app_data.ts
```
