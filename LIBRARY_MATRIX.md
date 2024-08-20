#### Labels
- L1 - Available throught SDK as a function
- L2 - Can be done but requires extra work

## Extrinsics

#### Data Availability

| Lib        | Submit Data | Submit Block Length Proposal | Create Application Key | Set Application Key | Set Submit Data Fee Modifier |
| ---------- | ----------- | ---------------------------- | ---------------------- | ------------------- | ---------------------------- |
| Avail-Rust | L1          | L1                           | L1                     | L1                  | L1                           |
| Avail-JS   | L1          | L1                           | L1                     | L1                  | L1                           |
| Avail-Deno | L1          | L1                           | L1                     | L1                  | L1                           |
| Avail-Go   | -           | -                            | -                      | -                   | -                            |

#### Balances

| Lib        | Transfer All | Transfer Allow Death | Transfer Keep Alive |
| ---------- | ------------ | -------------------- | ------------------- |
| Avail-Rust | L1           | L1                   | L1                  |
| Avail-JS   | L1           | L1                   | L1                  |
| Avail-Deno | L1           | L1                   | L1                  |
| Avail-Go   | -            | -                    | -                   |

#### Staking

| Lib        | Bond | Bond Extra | Chill | Chill Extra | Nomiate | Unbound | Validate |
| ---------- | ---- | ---------- | ----- | ----------- | ------- | ------- | -------- |
| Avail-Rust | L1   | L1         | L1    | L1          | L1      | L1      | L1       |
| Avail-JS   | L1   | L1         | L1    | L1          | L1      | L1      | L1       |
| Avail-Deno | L1   | L1         | L1    | L1          | L1      | L1      | L1       |
| Avail-Go   | -    | -          | -     | -           | -       | -       | -        |

## RPCs

#### Kate

| Lib        | Block Length | Query Data Proof | Query Proof | Query Rows |
| ---------- | ------------ | ---------------- | ----------- | ---------- |
| Avail-Rust | L1           | L1               | L1          | L1         |
| Avail-JS   | -            | -                | -           | -          |
| Avail-Deno | -            | -                | -           | -          |
| Avail-Go   | -            | -                | -           | -          |

#### Author

| Lib        | Rotate Keys | Has Key | Has Session Key | Insert Key | Pending Extrinsics | Remove Extrinsic | Submit Extrinsic |
| ---------- | ----------- | ------- | --------------- | ---------- | ------------------ | ---------------- | ---------------- |
| Avail-Rust | L1          | -       | -               | -          | -                  | -                | -                |
| Avail-JS   | -           | -       | -               | -          | -                  | -                | -                |
| Avail-Deno | -           | -       | -               | -          | -                  | -                | -                |
| Avail-Go   | -           | -       | -               | -          | -                  | -                | -                |

#### Chain

| Lib        | Get Block | Get Block Hash | Get Finalized Head | Get Header |
| ---------- | --------- | -------------- | ------------------ | ---------- |
| Avail-Rust | L1        | L1             | L1                 | L1         |
| Avail-JS   | -         | -              | -                  | -          |
| Avail-Deno | -         | -              | -                  | -          |
| Avail-Go   | -         | -              | -                  | -          |

#### Payment

| Lib        | Query Fee Details | Query Info |
| ---------- | ----------------- | ---------- |
| Avail-Rust | L1*               | L1*        |
| Avail-JS   | -                 | -          |
| Avail-Deno | -                 | -          |
| Avail-Go   | -                 | -          |

#### System

| Lib        | Account Next Index | Add Log Filter | Add Reserved Peer | Chain | Chain Type | Dry Run | Health | Local Listen Addresses | Local Peer Id | Name | Node Roles | Peers | Properties | Removed Reserved Peer | Reserved Peers | Reset Log Filter | Sync State | Version |
| ---------- | ------------------ | -------------- | ----------------- | ----- | ---------- | ------- | ------ | ---------------------- | ------------- | ---- | ---------- | ----- | ---------- | --------------------- | -------------- | ---------------- | ---------- | ------- |
| Avail-Rust | L1                 | -              | -                 | L1    | L1         | -       | L1     | L1                     | L1            | L1   | L1         | L1    | L1         | -                     | -              | -                | L1         | L1      |
| Avail-JS   | -                  | -              | -                 | -     | -          | -       | -      | -                      | -             | -    | -          | -     | -          | -                     | -              | -                | -          | -       |
| Avail-Deno | -                  | -              | -                 | -     | -          | -       | -      | -                      | -             | -    | -          | -     | -          | -                     | -              | -                | -          | -       |
| Avail-Go   | -                  | -              | -                 | -     | -          | -       | -      | -                      | -             | -    | -          | -     | -          | -                     | -              | -                | -          | -       |
