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
- Uses a chosen dev account (`Alice`, `Bob`, `Charlie`, `Dave`, `Eve`, `Ferdie`, `One`, `Two`)
- Configurable blob size (1-31 MiB), count, and repeated content character
- Optional staged delays between submissions (`warmup`, `subsequent`)
- Payload can be generated from a repeated char or loaded from disk (`--file`); sprinkles random bytes by default, disable via `--randomize-disabled`
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
cargo run --release --bin da-spammer -- --account alice
```

**Flags**
- `-a, --account <alice|bob|charlie|dave|eve|ferdie|one|two>` (required)
- `-s, --size-mb <1..31>`  (default: `31`)
- `--count <1..1000>`  (default: `50`)
- `--ch <char>`        (optional; default is first letter of `--account`)
- `-e, --endpoint <URL>`   (default: `http://127.0.0.1:8546`)
- `-w, --warmup-delay <ms>`      (default: `0`; additional sleep before the second submit)
- `--subsequent-delay <ms>`  (default: `0`; additional sleep before every submit after the second)
- `-r, --randomize-disabled` (optional; disable random byte sprinkling, which is enabled by default)
- `-f, --file <path>`      (optional; use contents of `path` as the payload source instead of a repeated character)

**Full explicit example**
```bash
./target/release/da-spammer \
  --account alice \
  --size-mb 16 \
  --count 10 \
  --ch Z \
  --endpoint http://127.0.0.1:8546
```
- Account: Alice
- Blob size: 16 MiB
- Transactions: 10
- Blob content: repeated `Z`
- RPC endpoint: local node

**Small / default example**
```bash
./target/release/da-spammer --account bob
```
- Account: Bob
- Blob size: 31 MiB (default)
- Transactions: 50 (default)
- Blob content: repeated `b`
- RPC endpoint: `http://127.0.0.1:8546`
- Delays: no warmup/subsequent delay (all default to `0`)

---

## 📝 Notes & Tips

- **Random bytes**: By default the payload sprinkles random bytes into the repeated character; use `--randomize-disabled` to turn that off.
- **Timing controls**: Combine`--warmup-delay`, and `--subsequent-delay` to pace transactions.
- **External payloads**: If you pass `--file`, make sure the file has at least `size_mb * 1024 * 1024` bytes; only the first chunk of that size is used (and may be randomized).
- **Client pool**: The binary instantiates multiple RPC clients to keep submissions flowing; adjust node-side rate limits accordingly.

---

## 📜 License

MIT (or your preferred license)
