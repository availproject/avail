# Introduction
Label Legend:
- TRA: Transaction/Extrinsic
- RPC: RPC
- TYP: Types

# v2.0.0.0 Changelog
## Native
- Binary name changed from `data-avail` to `avail-node`
- Added chain specification for new testnet and mainnet.
- Updated Polkadot-SDK dependency to v1.7.1
- **Batch calls that contain DataSubmit txs or Bridge txs are now banned from usage.**
- **Data root in the header extension now contains two hashes**
    - `blob data` hash - hashed data submit calls
    - `bridge data` hash - hashed bridge calls
## Runtime
- The following pallets were renamed:
    - `succinct` -> `vector`
- **Only DataSumit txs now go into the matrix.**
    - previously it was all transactions.
- Removed Nomad Bridge.
- Removed Bounties.
- Removed Technical Membership.
- Added Treasury Committee.

## SDKs
- Updated Avail-subxt subxt dependency from v0.29 to v0.34

## Examples
- The following deno example were updated or added:
    - `balance_transfer_basic.ts`
    - `balance_transfer_advanced.ts`
    - `da_submitdata_basic.ts`
    - `da_submitdata_advanced.ts`
    - `kate_query_app_data.ts`
    - `kate_query_block_length.ts`
    - `kate_query_data_proof.ts`
    - `kate_query_proof.ts`
    - `kate_query_rows.ts`
    - `register_validator.ts`
    - `staking_bond_basic.ts`
    - `staking_bond_advanced.ts`

# Migration Guide: 1.11.0.0 to 2.0.0.0

## Removed Balances::transfer transaction [TRA]
`Balances::transfer` transaction was removed and replaced by `Balances::transferAll`, `Balances::transferAllowDeath` and `Balances::transferKeepAlive`.

```js
// JS/Deno Example
const alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const bobAddress = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
const oneAvl = new BN("1000000000000000000");

// v1.11.0.0
const hash = await api.tx.balances.transfer(bobAddress, oneAvl).signAndSend(alice);

// V2.0.0.0
const hash = await api.tx.balances.transferKeepAlive(bobAddress, oneAvl).signAndSend(alice);
```

## New types introduced to Kate RPC [TYP]
```rust
pub type GRawScalar = U256;
pub type GRow = Vec<GRawScalar>;
pub type GProof = [u8; 48];
pub type GDataProof = (GRawScalar, GProof);

pub type MaxCells = ConstU32<10_000>;
pub type Cells = BoundedVec<Cell, MaxCells>;
```

## Changed Kate::queryRows output from `Vec<Vec<u8>>` to `Vec<Grow>` [RPC, TYP]
```rust
// Subxt example
let rows = &[2];
let block_hash: H256 = // Hash

// v1.11.0.0
let mut params = RpcParams::new();
params.push(rows)?;
params.push(Some(block_hash))?;
let rows: Vec<Vec<u8>> = rpc.request("kate_queryRows", params).await?;

// v2.0.0.0
let rows: Vec<Grow> = client.rpc_methods().query_rows(rows, block_hash).await?;

```

## Changed Kate::queryProof output from `Vec<u8>` to `Vec<GDataProof>` [RPC, TYP]
```rust
// Subxt example
let block_hash: H256 = // Hash
let cell = Cell { row: BlockLengthRows(0), col: BlockLengthColumns(0) };
let cells: Vec<Cell> = vec![cell];

// v1.11.0.0
let mut params = RpcParams::new();
params.push(cells)?;
params.push(Some(block_hash))?;
let proof: Vec<u8> = rpc.request("kate_queryProof", params).await?;

// v2.0.0.0
let cells = Cells::try_from(cells)?;
let proof: Vec<GDataProof> = client.rpc_methods().query_proof(Cells::try_from(cells).unwrap(), block_hash).await?;
```

## Changed Kate::queryAppData output from `Vec<Option<Vec<u8>>>` to `Vec<Option<Grow>>` [RPC, TYP]
```rust
// Subxt example
let app_id = AppId(0);
let block_hash: H256 = // Hash

// v1.11.0.0
let mut params = RpcParams::new();
params.push(app_id)?;
params.push(Some(block_hash))?;
let rows: Vec<Option<Vec<u8>>> = rpc.request("kate_queryAppData", params).await?;

// v2.0.0.0
let rows: Vec<Option<Grow>> = client.rpc_methods().query_app_data(app_id, block_hash).await?;
```