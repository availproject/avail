# DA Spammer

Two Rust CLIs to stress-test data availability on an [Avail](https://www.availproject.org/) node:

- **`da-spammer`** - single-account spammer: prepares blobs, computes commitments, and submits `submit_blob_metadata + blob` as signed extrinsics.
- **`da-sybil-spammer`** - multi-account spammer: generates many ephemeral accounts, batch-funds them using `utility.batchAll(balances.transfer_keep_alive(...))`, then round-robins blob submissions across them.

Both binaries connect to an Avail HTTP RPC endpoint and log per-tx progress.

---

## ✨ Features

Shared
- Connects to an Avail node via RPC (default: `http://127.0.0.1:8546`)
- Computes KZG commitments for each blob
- Rotates `app_id` (`i % 5`) for submissions
- Verbose logging (nonce, app_id, tx size, etc.)

**`da-spammer` (single account)**
- Uses a chosen dev account (`Alice`, `Bob`, `Charlie`, `Dave`, `Eve`, `Ferdie`, `One`, `Two`)
- Configurable blob size (1-64 MiB), count, and repeated content character

**`da-sybil-spammer` (multi account)**
- Generates *N* fresh SR25519 accounts (not persisted by default)
- Funds them from a dev account via `utility.batchAll + balances.transfer_keep_alive`
- Loops a given number of times and sends blobs round-robin: at loop *i*, use account `i % N`
- Optional delay between txs to smooth load

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
- `./target/release/da-sybil-spammer`

---

## 🚀 Usage

### 1) `da-spammer` (single account)

**Flags**
- `--account <alice|bob|charlie|dave|eve|ferdie|one|two>` (required)
- `--size-mb <1..64>`  (default: `32`)
- `--count <1..100>`   (default: `50`)
- `--ch <char>`        (optional; default is first letter of `--account`)
- `--endpoint <URL>`   (default: `http://127.0.0.1:8546`)

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
- Blob size: 32 MiB (default)
- Transactions: 50 (default)
- Blob content: repeated `b`
- RPC endpoint: `http://127.0.0.1:8546`

---

### 2) `da-sybil-spammer` (multi account)

**What it does**
1. Generates `--accounts` ephemeral keypairs (mnemonics are printed *only in-memory*; one sample SS58 is logged).
2. Funds each with `--fund-each` AVAIL using `utility.batchAll(balances.transfer_keep_alive(...))` from `--funder`.
3. Performs `--loops` submissions, using account `i % --accounts` on each iteration.
   - Blob size per tx is `--size-mb` MiB; content char is fixed via `--ch` or derived from account index.

**Flags**
- `--endpoint <URL>`            (default: `http://127.0.0.1:8546`)
- `--funder <dev-account>`      (default: `alice`; one of: `alice|bob|charlie|dave|eve|ferdie|one|two`)
- `--accounts <N>`              (default: `100`)
- `--fund-each <AVAIL>`         (default: `10`; amount in AVAIL, multiplied internally by chain `ONE_AVAIL` constant)
- `--batch-size <N>`            (default: `100`; number of transfers per `batchAll`)
- `--size-mb <1..64>`           (default: `32`)
- `--loops <N>`                 (default: `1000`)
- `--sleep-ms <milliseconds>`   (default: `0`; delay between submissions)
- `--ch <char>`                 (optional; fixed blob character)

**Default run**
```bash
./target/release/da-sybil-spammer \
  --endpoint http://127.0.0.1:8546 \
  --funder alice
```
- Generates 100 accounts
- Funds 10 AVAIL each (using chain's `ONE_AVAIL` base units)
- Batches transfers in groups of 100
- Submits 1000 blobs, 32 MiB each, round-robin over accounts

**Custom run**
```bash
./target/release/da-sybil-spammer \
  --endpoint http://127.0.0.1:8546 \
  --funder bob \
  --accounts 200 \
  --fund-each 5 \
  --batch-size 50 \
  --size-mb 16 \
  --loops 500 \
  --sleep-ms 10 \
  --ch X
```
- 200 accounts, fund 5 AVAIL each, batches of 50
- 500 blobs of 16 MiB, alternating through accounts
- 10 ms delay between submissions
- Blob content: repeated `X`

---

## 📝 Notes & Tips

- **Funding units**: `--fund-each` is interpreted as whole AVAIL and multiplied by the runtime's `ONE_AVAIL` base unit constant.
- **Batch size**: `utility.batchAll` can be large; if you hit call size/weight limits, reduce `--batch-size`.
- **Nonces**: `da-sybil-spammer` snapshots starting nonces and increments locally on success. (No retry/backoff logic by default.)
- **Blob length variance**: The multi-account script reduces the blob length slightly each iteration (`len_bytes - i`) to keep content unique; ensure `--loops <= blob_size_in_bytes`.

---

## 📜 License

MIT (or your preferred license)
