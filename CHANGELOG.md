# Avail Changelog

All notable changes to this project will be documented in this file.

## Next release
[Compare](https://github.com/availproject/avail/compare/v1.6.3...HEAD)

#### Added
#### Changed
#### Fixed

## v1.6.4
[Compare](https://github.com/availproject/avail/compare/v1.6.4...v1.6.3)

#### Added
- Switch to new api for commitment generation, improved performances.
- Substrate upgraded to [1.0.0](https://github.com/paritytech/polkadot/releases/tag/v1.0.0).
- Import process block verification ignores "Own" blocks.
- Support utility pallet for data submission.
- Added script to benchmark.
- Added changelog, issue template, contributing guide, security policy, release template.
- Added new flag `--unsafe-da-sync` to sync without building commitment.
- Added CI for try-runtime / benchmarking.

#### Changed
- Improved benchmarking.
- Increase data submission transaction max size to 512kb.
- Now only one port is specified for http and ws endpoints.
- Improved code organization.
- Updated chainspecs.
- Added defaults to pallets.
- Adjusted weight ratio for operational class.
- Updated weights to v2 syntax.
- Improved RPC casing.

#### Fixed
- Fixed CI memory issues.
- Fixed CodeCov.

#### Removed
- Removed `rs-merkle` and use only `binary-merkle-tree`
