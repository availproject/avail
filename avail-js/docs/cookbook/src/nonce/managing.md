# Managing Nonce

## Using Nonce in Transactions

To utilize a nonce in a transaction, it should be included in the last argument of the transaction call.

```ts
{{#include ./managing.ts:10:12}}
```

The `Account` instance provides an interface to set the nonce for all subsequent calls.

```ts
{{#include ./managing.ts:14:18}}
```

## Examples

#### Setting Nonce for Individual Transactions

The following example shows how to set the nonce separately for each transaction.

```ts
{{#include ./managing.ts:20:30}}
```

#### Non-Waiting Transactions

The same approach applies to transactions that do not wait for execution.

```ts
{{#include ./managing.ts:32:37}}
```

#### Automatically Incremented Nonce

In this scenario, instead of managing the nonce manually, it is fetched anew for each transaction. This works because the nonce is retrieved from the node rather than from the chain state, allowing it to increment as each transaction is queued in the memory pool.

```ts
{{#include ./managing.ts:39:43}}
```

## Source Code

```ts
{{#include ./managing.ts}}
```
