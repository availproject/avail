# RPC and Fee Details

#### author.rotateKeys

```ts
{{#include ./index.ts:author_rotateKeys}}
```

```ts
{
  babe: '0x9cea2fedecce74fd747105e85715d27e68e8977876dd8b8fd3f2e365bbd2c47a',
  grandpa: '0x3c6bcf411ff0e085c5d93b288a5d2b5dd0728e8d78792b60f5ed199788f88f98',
  imOnline: '0x78ff6f1d66252f366fc52cd1bbe9c08d9322cb018e64c115b79a5ce43ae8a241',
  authorityDiscover: '0x2080eb2c5c091c50a92bd908079c9fadac46ec59f9f44b4114858f1b5df6971c'
}
```

#### chain.getBlock

```ts
{{#include ./index.ts:chain_getBlock}}
```

```ts
{
  block: {
    header: {
      parentHash: '0x625b1b723ccad3eabed97db767a9bd479229d78ca794240f310d52ddfe0cde61',
      number: 1,
      stateRoot: '0x27a3085503538f17bcfe60c3afaf52819ed372b67694f9df8148ebe483923052',
      extrinsicsRoot: '0xa28c0eb3c3594c4ccacc51afb10cb1fb9e2dd7c79a3034493fe848741dec1423',
      digest: [Object],
      extension: [Object]
    },
    extrinsics: [ '0x280403000b702077729301', '0x1004270b00' ]
  },
  justifications: null
}
```

#### chain.getBlockHash

```ts
{{#include ./index.ts:chain_getBlockHash}}
```

```ts
0x004946b993c78d78af699bf32a2a50eafb041e0cdfebbe13daa2e34ecc03b9d1
```

#### chain.getFinalizedHead

```ts
{{#include ./index.ts:chain_getFinalizedHead}}
```

```ts
0x625b1b723ccad3eabed97db767a9bd479229d78ca794240f310d52ddfe0cde61
```

#### chain.getHeader

```ts
{{#include ./index.ts:chain_getHeader}}
```

```ts
{
  parentHash: '0x625b1b723ccad3eabed97db767a9bd479229d78ca794240f310d52ddfe0cde61',
  number: 1,
  stateRoot: '0x27a3085503538f17bcfe60c3afaf52819ed372b67694f9df8148ebe483923052',
  extrinsicsRoot: '0xa28c0eb3c3594c4ccacc51afb10cb1fb9e2dd7c79a3034493fe848741dec1423',
  digest: { logs: [ [Object], [Object], [Object] ] },
  extension: { v3: { appLookup: [Object], commitment: [Object] } }
}

```

#### system.accountNextIndex

```ts
{{#include ./index.ts:system_accountNextIndex}}
```

```ts
0
```

#### system.chain

```ts
{{#include ./index.ts:system_chain}}
```

```ts
Avail Development Network
```

#### system.chainType

```ts
{{#include ./index.ts:system_chainType}}
```

```ts
Development
```

#### system.health

```ts
{{#include ./index.ts:system_health}}
```

```ts
0
false
false
```

#### system.localListenAddresses

```ts
{{#include ./index.ts:system_localListenAddresses}}
```

```ts
/ip6/fe80::a333:1e13:2097:7c0a/tcp/30333/p2p/12D3KooWCSqjjJB2q7ZeEbsEB9atMeSpD9upgq9GGvjHBTv4DRgU
/ip6/::1/tcp/30333/p2p/12D3KooWCSqjjJB2q7ZeEbsEB9atMeSpD9upgq9GGvjHBTv4DRgU
/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWCSqjjJB2q7ZeEbsEB9atMeSpD9upgq9GGvjHBTv4DRgU
/ip4/192.168.1.103/tcp/30333/p2p/12D3KooWCSqjjJB2q7ZeEbsEB9atMeSpD9upgq9GGvjHBTv4DRgU
```

#### system.localPeerId

```ts
{{#include ./index.ts:system_localPeerId}}
```

```ts
12D3KooWCSqjjJB2q7ZeEbsEB9atMeSpD9upgq9GGvjHBTv4DRgU

```

#### system.name

```ts
{{#include ./index.ts:system_name}}
```

```ts
Avail Node
```

#### system.nodeRoles

```ts
{{#include ./index.ts:system_nodeRoles}}
```

```ts
Authority
```

#### system.peers

```ts
{{#include ./index.ts:system_peers}}
```

```ts

```

#### system.properties

```ts
{{#include ./index.ts:system_properties}}
```

```ts
42
18
AVAIL
```

#### system.syncState

```ts
{{#include ./index.ts:system_syncState}}
```

```ts
0
27
27
```

#### system.version

```ts
{{#include ./index.ts:system_version}}
```

```ts
2.2.1-7926ab79cbc
```

#### payment.queryInfo

```ts
{{#include ./index.ts:payment_queryInfo}}
```

```ts
2.2.1-7926ab79cbc
```

#### payment.queryFeeDetails

```ts
{{#include ./index.ts:payment_queryFeeDetails}}
```

```ts
124414000000000000
0
1960461890972725
```

#### kate.blockLength

```ts
{{#include ./index.ts:kate_blockLength}}
```

```ts
2097152
2097152
2097152
256
256
32
```

#### kate.queryDataProof

```ts
{{#include ./index.ts:kate_queryDataProof}}
```

```ts
0xd6e516bbf0b0d964a6a6a41a18c58a2eac4757001c2338a8601c4cc961332fda
0x29c73490baca9fe2b11095a69294de4b4a86bcb3a2eb3cd04b51dfdd0b4030f9
0x0000000000000000000000000000000000000000000000000000000000000000
1
0
0x47a59a7805e0bfe350ee0395d426c15770edc03fee72aa6532b5bbcffaf28030
```

#### kate.queryProof

```ts
{{#include ./index.ts:kate_queryProof}}
```

```ts
2178534751726990040338027377623275511556638494274780568875624948149315822336
0x89b49ab7bb1f3cbe8d76b2a2c62439dc50ae675bcbeaf18356cb7213772576761495145a8aff45a3e2f798d169a3efc0
```

#### kate.queryRows

```ts
{{#include ./index.ts:kate_queryRows}}
```

```ts
2178534751726990040338027377623275511556638494274780568875624948149315822336
69809044805081050561186983554854622074947956385056036930418766855605578529536
10405497027851839075564710420280846593395090753174299025377453497505735898368
53390067642421534589546003666495070348261001112389004194003449569247590612992
```

## Source Code

```ts
{{#include ./index.ts}}
```
