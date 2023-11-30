# Fee Proxy Pallet
## Storage Units
```rust
pub type FeeProxyAccount:  StorageValue<Option<AccountId>>
```
## Extrinsic
```rust
interface {
	wrap(call: Box<RuntimeCall>);

    // Root
	set_proxy_account(account: Option<AccountId>);
}
```
## Events
```rust
pub enum Event {
  WrappedOp{
     result: DispatchResult,
  },
  ProxyAccountSet{
    account: Option<AccountId>
  }
}
```
## Errors
```rust
pub enum Error {
  InsufficientBalanceInProxyAccount,
  ProxyAccountNotSet,
  TipIsNotAllowed
}
```