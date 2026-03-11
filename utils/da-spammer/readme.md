# DA Spammer

`da-spammer` is a single Rust CLI for stressing Avail DA blob submission.

It supports:
- single-account mode
- sybil mode with many derived accounts
- optional precomputation of blobs before submission
- bounded in-flight submission concurrency

## Build

```bash
cargo build --release -p da-spammer
```

Binary:

```bash
./target/release/da-spammer
```

## Main Flags

- `--account <alice|bob|charlie|dave|eve|ferdie|one|two>`: sender in normal mode, funder/root in sybil mode
- `--endpoint <url>`: RPC endpoint, default `http://127.0.0.1:8546`
- `--size-mb <n>`: blob size in MiB, `1..=31`
- `--file <path>`: use file contents as the blob payload template (exclusive to size-mb)
- `--count <n>`: number of submissions, `1..=1000`
- `--prepare <n>`: how many transactions to prepare ahead; `0` means on-the-fly
- `--in-flight <n>`: max concurrent submissions
- `--sybil <n>`: number of deterministic sender accounts; `1` means normal mode

## Modes

### Single account

Uses one dev account and submits `count` blobs with nonce ordering preserved.

```bash
./target/release/da-spammer \
  --account alice \
  --endpoint http://127.0.0.1:9944 \
  --size-mb 16 \
  --count 200 \
  --prepare 32 \
  --in-flight 8
```

### Sybil mode

Derives `n` deterministic accounts from the chosen root account, funds them if needed, then submits round-robin across them.

```bash
./target/release/da-spammer \
  --account alice \
  --sybil 10 \
  --endpoint http://127.0.0.1:8546 \
  --size-mb 8 \
  --count 200 \
  --prepare 50 \
  --in-flight 10
```

## File Payloads

Use `--file` to submit a real file as the blob payload. The file is loaded once, trimmed to the 31 MiB maximum if needed, and the first bytes are changed for each submission so every blob is unique.

- `--file` and `--size-mb` are mutually exclusive.
- If neither is provided, `da-spammer` falls back to a generated `31 MiB` payload.

```bash
./target/release/da-spammer \
  --account alice \
  --endpoint http://127.0.0.1:9944 \
  --file ./blob \
  --count 50
```

## Notes

- Sybil accounts are derived from `//<Root>//da-spammer//<index>`.
- In sybil mode, accounts below the minimum balance are topped up automatically.
- The tool prints per-tx results and a final summary with success, failures, bytes submitted, and average submit time.
