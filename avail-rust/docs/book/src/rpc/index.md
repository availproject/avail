# RPC and Fee Details

#### author_rotate_keys
```rs
{{#include ./mod.rs:11:14}}
```

```rs
SessionKeys {
    babe: Public(...),
    grandpa: Public(...),
    im_online: Public(...),
    authority_discovery: Public(...),
}
```

#### chain_get_block
```rs
{{#include ./mod.rs:16:18}}
```

```rs
BlockDetails {
    block: Block {
        header: AvailHeader {
            parent_hash: 0x4753c70a0652f50ee24f19ea402c1377ce5ab08fc5e0f801123e8116e5e1fcf8,
            number: 495,
            state_root: 0x22470c3402bee3cd95c10b9303e61019aaec0603cbfc197eca646c94ba9332f1,
            extrinsics_root: 0x609ed0e14f3252c9f59ab59004ea458d7927a5bd81f241651634266b7098f415,
            digest: Digest {...},
            extension: V3(
                HeaderExtension {
                    app_lookup: CompactDataLookup {
                        size: 0,
                        index: [],
                    },
                    commitment: KateCommitment {
                        rows: 0,
                        cols: 0,
                        commitment: [],
                        data_root: 0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5,
                    },
                },
            ),
        },
        extrinsics: [...],
    },
    justifications: None,
}
```

#### chain_get_block_hash
```rs
{{#include ./mod.rs:20:22}}
```
```rs
0xc4e0a9a2ef80ddc1d70c9946d8a6f86ca4b15053b39ba56709222f01ddc64561
```

#### chain_get_finalized_head
```rs
{{#include ./mod.rs:24:26}}
```
```rs
0x2c896c9faae4e111f1fbeb955be5e999a328846969b59a7a7c64eadc4701122a
```

#### chain_get_header
```rs
{{#include ./mod.rs:28:30}}
```
```rs
AvailHeader {
    parent_hash: 0x4753c70a0652f50ee24f19ea402c1377ce5ab08fc5e0f801123e8116e5e1fcf8,
    number: 495,
    state_root: 0x22470c3402bee3cd95c10b9303e61019aaec0603cbfc197eca646c94ba9332f1,
    extrinsics_root: 0x609ed0e14f3252c9f59ab59004ea458d7927a5bd81f241651634266b7098f415,
    digest: Digest {...},
    extension: V3(
        HeaderExtension {
            app_lookup: CompactDataLookup {
                size: 0,
                index: [],
            },
            commitment: KateCommitment {
                rows: 0,
                cols: 0,
                commitment: [],
                data_root: 0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5,
            },
        },
    ),
}
```

#### system_account_next_index
```rs
{{#include ./mod.rs:32:35}}
```
```rs
2
```

#### system_chain
```rs
{{#include ./mod.rs:37:39}}
```
```rs
"Avail Development Network"
```


#### system_chain_type
```rs
{{#include ./mod.rs:41:43}}
```
```rs
"Development"
```

#### system_health
```rs
{{#include ./mod.rs:45:47}}
```
```rs
SystemHealth {
    peers: 0,
    is_syncing: false,
    should_have_peers: false,
}
```

#### system_local_listen_addresses
```rs
{{#include ./mod.rs:49:51}}
```
```rs
value = [
    "/ip6/fe81::a234:6e32:1034:3c3b/tcp/30333/p2p/12D3KooWRajsCfp1NR15iN7PcwcFAG3LB7iGDKUBosHkevNRQLYs",
    "/ip4/192.168.1.103/tcp/30333/p2p/12D3KooWRajsCfp1NR15iN7PcwcFAG3LB7iGDKUBosHkevNRQLYs",
    "/ip6/::1/tcp/30333/p2p/12D3KooWRajsCfp1NR15iN7PcwcFAG3LB7iGDKUBosHkevNRQLYs",
    "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWRajsCfp1NR15iN7PcwcFAG3LB7iGDKUBosHkevNRQLYs",
]
```

#### system_local_peer_id
```rs
{{#include ./mod.rs:53:55}}
```
```rs
"12D3KooWRajsCfp1NR15iN7PcwcFAG3LB7iGDKUBosHkevNRQLYs"
```

#### system_name
```rs
{{#include ./mod.rs:57:59}}
```
```rs
"Avail Node"
```

#### system_node_roles
```rs
{{#include ./mod.rs:61:63}}
```
```rs
[
    Authority,
]
```

#### system_peers
```rs
{{#include ./mod.rs:65:67}}
```
```rs
[]
```

#### system_properties
```rs
{{#include ./mod.rs:69:71}}
```
```rs
{
    "ss58Format": Number(42),
    "tokenDecimals": Number(18),
    "tokenSymbol": String("AVAIL"),
}
```

#### system_system_sync_state
```rs
{{#include ./mod.rs:73:75}}
```
```rs
SyncState {
    starting_block: 0,
    current_block: 495,
    highest_block: 495,
}
```

#### system_version
```rs
{{#include ./mod.rs:77:79}}
```
```rs
"2.2.1-55da578d34b"
```

#### TransactionPaymentApi_query_info
```rs
{{#include ./mod.rs:81:95}}
```
```rs
124684322202721409
```

#### TransactionPaymentApi_query_fee_details
```rs
{{#include ./mod.rs:97:106}}
```
```rs
FeeDetails {
    inclusion_fee: Some(
        InclusionFee {
            base_fee: 124414000000000000,
            len_fee: 11400000000000,
            adjusted_weight_fee: 259321813738397,
        },
    ),
    tip: 0,
}
```

#### kate_block_length
```rs
{{#include ./mod.rs:108:110}}
```
```rs
BlockLength {
    max: PerDispatchClass {
        normal: 2097152,
        operational: 2097152,
        mandatory: 2097152,
    },
    cols: BlockLengthColumns(
        256,
    ),
    rows: BlockLengthRows(
        256,
    ),
    chunk_size: 32,
}
```

#### kate_query_data_proof
```rs
{{#include ./mod.rs:112:118}}
```
```rs
ProofResponse {
    data_proof: DataProof {
        roots: TxDataRoots {
            data_root: 0xd6e516bbf0b0d964a6a6a41a18c58a2eac4757001c2338a8601c4cc961332fda,
            blob_root: 0x29c73490baca9fe2b11095a69294de4b4a86bcb3a2eb3cd04b51dfdd0b4030f9,
            bridge_root: 0x0000000000000000000000000000000000000000000000000000000000000000,
        },
        proof: [],
        number_of_leaves: 1,
        leaf_index: 0,
        leaf: 0x47a59a7805e0bfe350ee0395d426c15770edc03fee72aa6532b5bbcffaf28030,
    },
    message: None,
}
```

#### kate_query_proof
```rs
{{#include ./mod.rs:120:123}}
```
```rs
[
    (
        2178534751726990040338027377623275511556638494274780568875624948149315822336,
        GProof(
            [...],
        ),
    ),
]
```

#### kate_query_rows
```rs
{{#include ./mod.rs:125:128}}
```
```rs
[
    [
        2178534751726990040338027377623275511556638494274780568875624948149315822336,
        69809044805081050561201039752112594468796256047454289799440609083602104564736,
        26941852917393734161602180963833199552029986735939578666038548832600818441216,
        14351520191331507525755130937317610561547699892218140156652644610507664261120,
    ],
]
```

## Source Code
```rs
{{#include ./mod.rs}}
```
