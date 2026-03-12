# Manual DA Benchmark Guide

## Goal

Use this guide to run a manual DA benchmark across the validators and full node, record what happened, and decide how close the network gets to sustained large blocks.

The target exercise is simple:

- run one or more `da-spammer` processes from the available machines
- push enough load to produce a sequence of very large finalized blocks
- record the exact block numbers, commands, logs, and observations

This file is meant to be edited after each run. Keep the commands you used and the block ranges you care about.

## Machines

Fill in the SSH user if it differs from `root`.

| Role        | IP            | SSH user | Local RPC               |
| ----------- | ------------- | -------- | ----------------------- |
| Validator 1 | `xx.xx.xx.xx` | `root`   | `http://127.0.0.1:8546` |
| Validator 2 | `xx.xx.xx.xx` | `root`   | `http://127.0.0.1:8546` |
| Validator 3 | `xx.xx.xx.xx` | `root`   | `http://127.0.0.1:8546` |
| Validator 4 | `xx.xx.xx.xx` | `root`   | `http://127.0.0.1:8546` |
| Full node   | `xx.xx.xx.xx` | `root`   | `http://127.0.0.1:8546` |

## SSH Access

If your key is called `devnet-key`:

```bash
ssh -i /root/keys/devnet-key root@<ip>
```

Useful one-shot SSH checks before the benchmark:

```bash
hostname
git --version
curl -H 'Content-Type: application/json' -d '{"id":1,"jsonrpc":"2.0","method":"system_health","params":[]}' http://127.0.0.1:8546
```

## Repo Setup On Every Machine

Do this once on each machine.

```bash
git clone https://github.com/availproject/avail.git
git fetch
git checkout ghali/infinity-da
cd avail/utils/da-spammer
cargo build --release
```

Before a real run, also check:

- enough free disk for logs and build artifacts: `df -h`
- enough free memory: `free -h`
- node logs are accessible
- explorer access is ready from your workstation

## What To Check Before Starting A Benchmark

On every machine that will send traffic:

- the node is healthy and synced
- the node is listening on `127.0.0.1:8546`

## How `da-spammer` Actually Works

Important behavior to keep in mind:

- `--count` is the total number of submissions for the process
- the process exits when those submissions are done, or on the first hard failure
- `--prepare` controls how many blobs are prepared ahead of submission (When a preparation is over, we start the next part if any)
- `--in-flight` bounds concurrent submissions across accounts
- `--sybil` enables round-robin sending across deterministic derived accounts
- each account still submits in nonce order

Operational consequences:

- if `--count` is `200`, the process attempts at most 200 submissions total
- `--prepare 0` means prepare and submit on the fly
- higher `--prepare`, `--in-flight`, `--count`, and `--size-mb` increase sender CPU and memory pressure
- in sybil mode, the chosen root account funds derived accounts if needed before submission starts

## Caveats

1. The sender computes FRI commitments locally before submission.
   If sender CPU becomes the bottleneck, the benchmark may measure client preparation instead of chain capacity.

## Baseline Commands

### Single account

```bash
./target/release/da-spammer \
  --account alice \
  --size-mb 31 \
  --count 20 \
  --prepare 5 \
  --in-flight 10 \
  --endpoint http://127.0.0.1:8546 | tee spammer-alice.log
```

### Sybil mode

```bash
./target/release/da-spammer \
  --account alice \
  --sybil 10 \
  --size-mb 31 \
  --count 20 \
  --prepare 5 \
  --in-flight 10 \
  --endpoint http://127.0.0.1:8546 | tee spammer-sybil.log
```

## Suggested Host Allocation

If you run single-account mode on several machines at once, use different funded accounts:

| Machine     | Recommended account |
| ----------- | ------------------- |
| Validator 1 | `alice`             |
| Validator 2 | `bob`               |
| Validator 3 | `charlie`           |
| Validator 4 | `dave`              |
| Full node   | `eve`               |

Do not run two single-account spammers with the same account at the same time.

If you use sybil mode, keep one root account per machine.

## Suggested Manual Run Order

Start simple and keep the runs easy to compare.

1. One machine only, `16 MB`, confirm the command is stable.
2. One machine only, `31 MB`, confirm large blobs are stable.
3. Two machines with the same blob size and same flags.
4. Three machines.
5. Four machines.
6. Five machines.
7. Repeat with the next blob size.

Recommended blob sizes:

- `1 MB`
- `2 MB`
- `4 MB`
- `8 MB`
- `16 MB`
- `31 MB`

## What To Watch During The Run

From node logs and explorer:

- `success`
- `failed`
- average submit latency
- whether failures are nonce-related, already-imported, seed-mismatch, or transport-related
- block production time
- import and finalization lag
- RPC errors
- sender CPU pressure from commitment generation
- exact finalized block numbers
- consecutive block sizes
- the post-inherent DA summary extrinsic
- whether you actually got a sequence of large blocks in a row
