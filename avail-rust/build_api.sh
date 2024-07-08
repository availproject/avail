#!/bin/sh
echo "⛓ Installing SubXt..."
cargo install --git https://github.com/paritytech/subxt --tag v0.37.0 subxt-cli || true
echo "🔨 Generating Avail-SubXt API from localhost..."
subxt codegen --version 14 \
--derive Clone \
--derive PartialEq \
--derive Eq \
--derive-for-type avail_core::header::extension::HeaderExtension=serde::Serialize \
--derive-for-type avail_core::header::extension::HeaderExtension=serde::Deserialize  \
--derive-for-type avail_core::header::extension::v3::HeaderExtension=serde::Serialize  \
--derive-for-type avail_core::header::extension::v3::HeaderExtension=serde::Deserialize  \
--derive-for-type avail_core::header::extension::v3::HeaderExtension=Default \
--derive-for-type avail_core::kate_commitment::v3::KateCommitment=serde::Serialize \
--derive-for-type avail_core::kate_commitment::v3::KateCommitment=serde::Deserialize \
--derive-for-type avail_core::kate_commitment::v3::KateCommitment=Default \
--derive-for-type avail_core::data_lookup::compact::CompactDataLookup=Default \
--derive-for-type avail_core::data_lookup::compact::CompactDataLookup=serde::Serialize \
--derive-for-type avail_core::data_lookup::compact::CompactDataLookup=serde::Deserialize \
--derive-for-type avail_core::data_lookup::compact::DataLookupItem=serde::Serialize \
--derive-for-type avail_core::data_lookup::compact::DataLookupItem=serde::Deserialize \
--derive-for-type avail_core::AppId=serde::Serialize \
--derive-for-type avail_core::AppId=serde::Deserialize \
--derive-for-type avail_core::AppId=Default \
--derive-for-type avail_core::AppId=Copy \
--derive-for-type avail_core::AppId=derive_more::From \
--derive-for-type avail_core::BlockLengthColumns=serde::Serialize \
--derive-for-type avail_core::BlockLengthColumns=serde::Deserialize \
--derive-for-type avail_core::BlockLengthRows=serde::Serialize \
--derive-for-type avail_core::BlockLengthRows=serde::Deserialize \
--derive-for-type frame_system::limits::BlockLength=serde::Serialize \
--derive-for-type frame_system::limits::BlockLength=serde::Deserialize \
--derive-for-type frame_support::dispatch::PerDispatchClass=serde::Serialize \
--derive-for-type frame_support::dispatch::PerDispatchClass=serde::Deserialize \
--url http://localhost:9944 \
| sed -En "s/pub struct KateCommitment/#\[serde\(rename_all = \"camelCase\"\)\] &/gp" \
| sed -En "s/pub struct HeaderExtension/#\[serde\(rename_all = \"camelCase\"\)\] &/gp" \
| sed -En "s/pub struct DataLookupItem/#\[serde\(rename_all = \"camelCase\"\)\] \0/p" \
| sed -En "s/pub struct BlockLength\b/#\[serde\(rename_all = \"camelCase\"\)\] \0/p" \
| sed -En "s/pub enum RuntimeCall\b/#\[derive\(derive_more::From\)\] \0/p" \
| sed -E '1i \#\[allow(clippy::all)]' \
| rustfmt --edition=2021 --emit=stdout > ./src/api_dev.rs
echo "🎁 Avail-SubXt API generated in './src/api_dev.rs'"
