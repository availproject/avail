# Fetching Nonce

## Fetching Nonce from chain state

#### Fetching the state nonce via chain state query

Directly querying system.account from the chain state provides an object that includes the account's nonce.

```ts
{{#include ./fetching.ts:8:12}}
```

#### Fetching the state nonce via SDK

The SDK offers a utility function for fetching the nonce, avoiding the need to manually query.

```ts
{{#include ./fetching.ts:14:16}}
```

#### Fetching the state nonce via free function

The SDK's nonce helper function is also accessible as a free-standing function.

```ts
{{#include ./fetching.ts:18:20}}
```

#### Fetching the state nonce via Account instance

The nonce can also be retrieved directly from an Account instance.

```ts
{{#include ./fetching.ts:22:24}}
```

## Fetching Nonce from node

The nonce can also be retrieved via an RPC call, which queries the chain state and adjusts the nonce if there are pending transactions in the memory queue.

#### Fetching the state node nonce via RPC call

The `system.accountNextIndex` RPC call is a direct way to fetch the nonce.

```ts
{{#include ./fetching.ts:26:29}}
```

#### Fetching the state node nonce via SDK

The SDK provides a helper function to perform the RPC call automatically.

```ts
{{#include ./fetching.ts:31:33}}
```

#### Fetching the state node nonce via free function

The SDK's nonce helper function is also accessible as a free-standing function.

```ts
{{#include ./fetching.ts:35:37}}
```

#### Fetching the state node nonce via Account instance

The nonce can also be retrieved directly from an Account instance.

```ts
{{#include ./fetching.ts:39:40}}
```

## Source Code

```ts
{{#include ./fetching.ts}}
```
