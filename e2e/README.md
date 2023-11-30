Build and run a Node with `--features fast-runtime` enabled
```bash
cargo build --release --features fast-runtime
 ./target/release/data-avail --dev
```

After that in a new terminal run the following command:
``` bash
cd e2e
cargo test -- --test-threads 1
```