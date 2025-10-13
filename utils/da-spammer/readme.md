# DA Spammer

Rust CLI to stress-test data availability on an [Avail](https://www.availproject.org/) node by preparing blobs, building commitments, and submitting `submit_blob_metadata + blob` as signed extrinsics. The tool connects to an Avail HTTP RPC endpoint and logs per-transaction progress.

---

## ✨ Features

Shared

- Connects to an Avail node via RPC (default: `http://127.0.0.1:8546`)
- Computes KZG commitments for each blob
- Rotates `app_id` (`i % 5`) for submissions
- Verbose logging (nonce, app_id, tx size, etc.)
- Uses a pool of RPC clients to overlap metadata generation and submissions

**`da-spammer`**

- Uses one or more dev accounts (`Alice`, `Bob`, `Charlie`, `Dave`, `Eve`, `Ferdie`, `One`, `Two`)
- Configurable blob size (1-31 MiB) and submission count
- Optional staged delays between submissions (`warmup`, `subsequent`)
- Payload can be generated in-memory or loaded from disk (`--file`); sprinkles random bytes by default, disable via `--randomize-disabled`

---

## ⚡ Requirements

- Rust (>= 1.70 recommended)
- A running Avail node exposing HTTP RPC at the endpoint you plan to use  
  (for local testing: `http://127.0.0.1:8546`)

---

## 🔧 Build

```bash
cargo build --release
```

Artifacts:

- `./target/release/da-spammer`

---

## 🚀 Usage

### 1) `da-spammer` (single account)

```bash
cargo run --release --bin da-spammer -- --accounts alice
```

**Flags**

- `-a, --accounts <alice|bob|charlie|dave|eve|ferdie|one|two>[,...]` (optional; comma-separated, defaults to `alice` if omitted)
- `-s, --size-mb <1..31>` (default: `31`)
- `--count <1..1000>` (default: `50`)
- `-e, --endpoint <URL>` (default: `http://127.0.0.1:8546`)
- `-w, --warmup-delay <ms>` (default: `0`; additional sleep before the second submit)
- `--subsequent-delay <ms>` (default: `750`; additional sleep before every submit after the second; 750ms is the probable optimal delay for ONE account)
- `-r, --randomize-disabled` (flag; disable the default random byte sprinkling)
- `-f, --file <path>` (optional; use contents of `path` as the payload source)

**Full explicit example**

```bash
./target/release/da-spammer \
  --accounts alice,bob \
  --size-mb 16 \
  --count 10 \
  --endpoint http://127.0.0.1:8546 \
  --warmup-delay 1000 \
  --subsequent-delay 250 \
  --file ./payload.bin \
  --randomize-disabled
```

- Accounts: Alice, Bob (round-robin signer selection)
- Blob size: 16 MiB
- Transactions: 10
- Blob content: first 16 MiB of `./payload.bin`
- RPC endpoint: local node
- Randomization disabled; delays applied after first and subsequent submissions

**Small / default example**

```bash
./target/release/da-spammer
```

- Account: Alice (default)
- Blob size: 31 MiB (default)
- Transactions: 50 (default)
- Blob content: zeroed buffer with sprinkled random bytes
- RPC endpoint: `http://127.0.0.1:8546`
- Delays: no warmup/subsequent delay (all default to `0`)

---

## 📝 Notes & Tips

- **Random bytes**: By default the payload is zeroed and a handful of positions are randomized; use `--randomize-disabled` to keep the payload untouched.
- **Timing controls**: Combine `--warmup-delay` and `--subsequent-delay` to pace transactions.
- **External payloads**: If you pass `--file`, make sure the file has at least `size_mb * 1024 * 1024` bytes; only the first chunk of that size is used (and may be randomized).
- **Client pool**: The binary instantiates multiple RPC clients to keep submissions flowing; adjust node-side rate limits accordingly.

---

## 📜 License

MIT (or your preferred license)
