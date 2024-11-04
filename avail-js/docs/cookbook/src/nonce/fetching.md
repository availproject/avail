# Fetching Nonce

## Fetching Nonce From Chain State

There are multiple methods to retrieve the nonce from the chain state:

#### Chain state query

Directly querying system.account from the chain state provides an object that includes the account's nonce.

```ts
{{#include ./fetching.ts:8:12}}
```

#### SDK

The SDK offers a utility function for fetching the nonce, avoiding the need to manually query.

```ts
{{#include ./fetching.ts:14:16}}
```

#### Free function

The SDK's nonce helper function is also accessible as a free-standing function.

```ts
{{#include ./fetching.ts:18:20}}
```

#### Account

The nonce can also be retrieved directly from an Account instance.

```ts
{{#include ./fetching.ts:22:24}}
```

## Fetching Nonce From Node

The nonce can also be retrieved via an RPC call, which queries the chain state and adjusts the nonce if there are pending transactions in the memory queue.

#### RPC call

The `system.accountNextIndex` RPC call is a direct way to fetch the nonce.

```ts
{{#include ./fetching.ts:26:29}}
```

#### SDK

The SDK provides a helper function to perform the RPC call automatically.

```ts
{{#include ./fetching.ts:31:33}}
```

#### Free function

The SDK's nonce helper function is also accessible as a free-standing function.

```ts
{{#include ./fetching.ts:35:37}}
```

#### Account

The nonce can also be retrieved directly from an Account instance.

```ts
{{#include ./fetching.ts:39:40}}
```

## Source Code

```ts
{{#include ./fetching.ts}}
```
