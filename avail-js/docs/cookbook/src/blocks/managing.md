# Managing Blocks

There is a lot of things that can be done with a block. Because [Polkadot JS Docs](https://polkadot.js.org/docs/api/cookbook/blocks/) covers most of it here we are just going to tackle the additional interface that we have over the SignedBlock object.

## Block Abstractions

### SDK

#### Getting transaction count

```ts
{{#include ./managing.ts:17:19}}
```

#### Getting all transactions

```ts
{{#include ./managing.ts:21:22}}
```

#### Getting transaction by transaction hash

```ts
{{#include ./managing.ts:24:25}}
```

#### Getting transaction by transaction index

```ts
{{#include ./managing.ts:27:30}}
```

#### Getting transaction by transaction signer(author)

```ts
{{#include ./managing.ts:32:33}}
```

#### Getting data-submission transaction count

```ts
{{#include ./managing.ts:35:36}}
```

#### Getting all data-submission instances

```ts
{{#include ./managing.ts:38:39}}
```

#### Getting data-submission data by transaction hash

```ts
{{#include ./managing.ts:41:44}}
```

#### Getting data-submission data by transaction index

```ts
{{#include ./managing.ts:46:49}}
```

#### Getting data-submission data by transaction signer(author)

```ts
{{#include ./managing.ts:51:52}}
```

### Free Functions

#### Getting transaction count

```ts
{{#include ./managing.ts:56:58}}
```

#### Getting all transactions

```ts
{{#include ./managing.ts:60:61}}
```

#### Getting transaction by transaction hash

```ts
{{#include ./managing.ts:63:64}}
```

#### Getting transaction by transaction index

```ts
{{#include ./managing.ts:66:69}}
```

#### Getting transaction by transaction signer(author)

```ts
{{#include ./managing.ts:71:72}}
```

#### Getting data-submission transaction count

```ts
{{#include ./managing.ts:74:75}}
```

#### Getting all data-submission instances

```ts
{{#include ./managing.ts:77:78}}
```

#### Getting data-submission data by transaction hash

```ts
{{#include ./managing.ts:80:83}}
```

#### Getting data-submission data by transaction index

```ts
{{#include ./managing.ts:85:88}}
```

#### Getting data-submission data by transaction signer(author)

```ts
{{#include ./managing.ts:90:91}}
```

## Source Code

```ts
{{#include ./managing.ts}}
```
