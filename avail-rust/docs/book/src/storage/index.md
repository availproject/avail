# Storage

### da_app_keys
```rs
{{#include ./mod.rs:9:24}}
```

```rs
AppKeyInfo {
    owner: AccountId32(...),
    id: AppId(
        1,
    ),
}
```

### da_app_keys_iter
```rs
{{#include ./mod.rs:26:45}}
```

```rs
Key: "Reserved-2"
Value: AppKeyInfo { owner: AccountId32(...), id: AppId(2) }
Key: "Reserved-8"
Value: AppKeyInfo { owner: AccountId32(...), id: AppId(8) }
Key: "Reserved-1"
Value: AppKeyInfo { owner: AccountId32(...) id: AppId(1) }
Key: "Reserved-9"
Value: AppKeyInfo { owner: AccountId32(...), id: AppId(9) }
Key: "Reserved-4"
Value: AppKeyInfo { owner: AccountId32(...), id: AppId(4) }
Key: "Reserved-5"
Value: AppKeyInfo { owner: AccountId32(...), id: AppId(5) }
Key: "Reserved-7"
Value: AppKeyInfo { owner: AccountId32(...), id: AppId(7) }
Key: "Avail"
Value: AppKeyInfo { owner: AccountId32(...), id: AppId(0) }
Key: "Reserved-3"
Value: AppKeyInfo { owner: AccountId32(...), id: AppId(3) }
Key: "Reserved-6"
Value: AppKeyInfo { owner: AccountId32(...), id: AppId(6) }
```

### da_next_app_id
```rs
{{#include ./mod.rs:47:60}}
```

```rs
AppId(10)
```

### staking_active_era
```rs
{{#include ./mod.rs:62:74}}
```

```rs
ActiveEraInfo {
    index: 13,
    start: Some(
        1732612788000,
    ),
}
```

### staking_bonded
```rs
{{#include ./mod.rs:76:90}}
```

```rs
 AccountId32(...)
```

### staking_bonded_iter
```rs
{{#include ./mod.rs:92:110}}
```

```rs
Key: "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"
Value: AccountId32(...)
```

### system_account_iter
```rs
{{#include ./mod.rs:112:130}}
```

```rs
Key: "5FCfAonRZgTFrTd9HREEyeJjDpT397KMzizE6T3DvebLFE7n"
Value: AccountInfo { nonce: 0, consumers: 0, providers: 1, sufficients: 0, data: AccountData { free: 10000000000000000000000000, reserved: 0, frozen: 0, flags: ExtraFlags(170141183460469231731687303715884105728) } }
Key: "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL"
Value: AccountInfo { nonce: 0, consumers: 0, providers: 1, sufficients: 0, data: AccountData { free: 10000000000000000000000000, reserved: 0, frozen: 0, flags: ExtraFlags(170141183460469231731687303715884105728) } }
Key: "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"
Value: AccountInfo { nonce: 0, consumers: 3, providers: 1, sufficients: 0, data: AccountData { free: 10000001075151923366255874, reserved: 0, frozen: 100000000000000000000000, flags: ExtraFlags(170141183460469231731687303715884105728) } }
Key: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
Value: AccountInfo { nonce: 0, consumers: 0, providers: 1, sufficients: 0, data: AccountData { free: 10000000000000000000000000, reserved: 0, frozen: 0, flags: ExtraFlags(170141183460469231731687303715884105728) } }
Key: "5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z"
Value: AccountInfo { nonce: 0, consumers: 0, providers: 1, sufficients: 0, data: AccountData { free: 625293930193641302084, reserved: 0, frozen: 0, flags: ExtraFlags(170141183460469231731687303715884105728) } }
Key: "5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"
Value: AccountInfo { nonce: 0, consumers: 0, providers: 1, sufficients: 0, data: AccountData { free: 10000000000000000000000000, reserved: 0, frozen: 0, flags: ExtraFlags(170141183460469231731687303715884105728) } }
Key: "5CRmqmsiNFExV6VbdmPJViVxrWmkaXXvBrSX8oqBT8R9vmWk"
Value: AccountInfo { nonce: 0, consumers: 0, providers: 1, sufficients: 0, data: AccountData { free: 10000000000000000000000000, reserved: 0, frozen: 0, flags: ExtraFlags(170141183460469231731687303715884105728) } }
Key: "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"
Value: AccountInfo { nonce: 0, consumers: 0, providers: 1, sufficients: 0, data: AccountData { free: 10000000000000000000000000, reserved: 0, frozen: 0, flags: ExtraFlags(170141183460469231731687303715884105728) } }
Key: "5Ck5SLSHYac6WFt5UZRSsdJjwmpSZq85fd5TRNAdZQVzEAPT"
Value: AccountInfo { nonce: 0, consumers: 0, providers: 1, sufficients: 0, data: AccountData { free: 10000000000000000000000000, reserved: 0, frozen: 0, flags: ExtraFlags(170141183460469231731687303715884105728) } }
Key: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
Value: AccountInfo { nonce: 3, consumers: 0, providers: 1, sufficients: 0, data: AccountData { free: 9999998624240383168720634, reserved: 0, frozen: 0, flags: ExtraFlags(170141183460469231731687303715884105728) } }
Key: "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
Value: AccountInfo { nonce: 0, consumers: 0, providers: 1, sufficients: 0, data: AccountData { free: 10000000000000000000000000, reserved: 0, frozen: 0, flags: ExtraFlags(170141183460469231731687303715884105728) } }
Key: "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"
Value: AccountInfo { nonce: 0, consumers: 0, providers: 1, sufficients: 0, data: AccountData { free: 10000000000000000000000000, reserved: 0, frozen: 0, flags: ExtraFlags(170141183460469231731687303715884105728) } }
Key: "5HKPmK9GYtE1PSLsS1qiYU9xQ9Si1NcEhdeCq9sw5bqu4ns8"
Value: AccountInfo { nonce: 0, consumers: 0, providers: 1, sufficients: 0, data: AccountData { free: 10000000000000000000000000, reserved: 0, frozen: 0, flags: ExtraFlags(170141183460469231731687303715884105728) } }
```

## Source Code
```rs
{{#include ./mod.rs}}
```