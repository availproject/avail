# Avail Changelog

All notable changes to this project will be documented in this file.

# Next release
- [Compare](https://github.com/availproject/avail/compare/v1.7.1...HEAD)

## Added
## Changed
## Fixed

# v1.7.1
- [Compare](https://github.com/availproject/avail/compare/v1.7.1...v1.6.3)
### Binary Upgrade Priority 
❗️❗️High: This is a **high-priority** release, and you must upgrade **as soon as possible**.

Runtime version: **12**

## Changes
### Runtime
#### API breaking changes
- 🗑️ Removed Democracy module - part of new Governance change
- 🗑️ Removed Council module - part of new Governance change
- 🗑️ Removed Elections module - part of new Governance change
- 🗑️ Removed Uncles from Authorship module - part of Substrate v1.0.0 upgrade
- ✅ HTTP and WS now use the same port (9944)
- 👓 Error changes: https://pastebin.com/raw/4MAifbNU
- 👓 Event changes: https://pastebin.com/raw/kEWJCnHv
- 👓 Call changes: https://pastebin.com/raw/B6SZChd0

#### API non-breaking changes
- ✅ Updated our Substrate dependencies to version v1.0.0 - part of Substrate v1.0.0 upgrade
- ✅ Updated transaction weights with more realistic values
- ✅ Updated weight ratio for operational class
- ✅ Improved the performance of commitment generation
- 🆕 Added support for data submission TX to be executed via utility module TXs.
- 🆕 Increased data submission transaction max size to 512kb
- 🆕 Added Mandate module - part of new Governance change

### Binary
#### Breaking changes
- ✅ Updated binary code to be compatible with Substrate v1.0.0 - part of Substrate v1.0.0 upgrade  
- 🆕 Added Kate, Biryani and Dymension  chain specification to the node itself. You can access them via the following flags: `--chain kate`; `--chain biryani` and `--chain dymension`
- 🆕 Import process block verification ignores "Own" blocks.

#### Non-breaking changes
- 🆕 ❗️Added a new flag `--unsafe-da-sync` to sync without building commitments

### Misc
- 🗑️ Removed code related to `rs-merkle` library
- ✅ Updated benchmarks to V2 syntax
- 🆕 Added CI scripts for try-runtime and benchmarking 
- 🆕 Added changelog, issue template, contributing guide, security policy, release template.