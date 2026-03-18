# Avail Try-Runtime CLI

Avail's custom `try-runtime` tool, built on top of Substrate's `try-runtime-cli` and embedded in the Avail workspace.

This copy exists inside the Avail repo so it can build against Avail's runtime, custom `frame-system`, block type, and hosted runtime interfaces without fighting a split dependency graph.

## What Is Custom Here

Compared with upstream Substrate `try-runtime-cli`, this embedded version is Avail-specific:

- it is built inside the Avail workspace
- it uses Avail's runtime block type
- it uses Avail hosted functions needed by the runtime
- it is intended for Avail networks such as Infinity

This is the right binary to use when testing Avail runtime upgrades against live Avail state.

## Location

The tool lives under:

```text
utils/try-runtime-cli/
```

The workspace members are:

- `utils/try-runtime-cli/cli`
- `utils/try-runtime-cli/core`

## Build

Run from the Avail repo root:

```bash
cargo build -p try-runtime-cli --release
```

The binary will be produced at:

```text
target/release/try-runtime
```

## Usage

For runtime-upgrade testing, pass an Avail runtime wasm compiled with `try-runtime` enabled.
Using `--runtime existing` against a production chain will fail because the on-chain wasm usually does not expose the `try-runtime` runtime API.

Example shape:

```bash
./target/release/try-runtime \
  --runtime /absolute/path/to/da_runtime.compact.compressed.wasm \
  on-runtime-upgrade \
  --blocktime 6000 \
  live \
  --uri wss://infinity-testnet-rpc.avail.so/ws
```

If you need migration hooks to run synchronously during testing, add:

```bash
--disable-mbm-checks
```

## Testing

Minimum checks before using or changing this tool:

```bash
cargo check -p try-runtime-cli --offline
cargo build -p try-runtime-cli --release
```

If you are changing execution behavior, also validate with a live Avail endpoint and a `try-runtime`-enabled Avail runtime wasm.

Example:

```bash
./target/release/try-runtime \
  --runtime /absolute/path/to/da_runtime.compact.compressed.wasm \
  on-runtime-upgrade \
  --blocktime 6000 \
  live \
  --uri wss://infinity-testnet-rpc.avail.so/ws
```

## Notes

- This tool is maintained as an Avail workspace utility, not as a standalone repo inside this tree.
- The README here documents the embedded Avail version, not the upstream Substrate project.
- If upstream `try-runtime-cli` changes are needed, port them deliberately instead of assuming the embedded copy should stay identical.
