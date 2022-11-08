#!/bin/sh
cargo install --git https://github.com/fmiguelgarcia/subxt --branch cli_derives_for_type_2
subxt codegen \
	--derive Clone \
	--derive PartialEq \
	--derive Eq \
	--derive-for-type da_primitives::header::extension::HeaderExtension=serde::Serialize \
	--derive-for-type da_primitives::header::extension::HeaderExtension=serde::Deserialize  \
	--derive-for-type da_primitives::header::extension::v1::HeaderExtension=serde::Serialize  \
	--derive-for-type da_primitives::header::extension::v1::HeaderExtension=serde::Deserialize  \
	--derive-for-type da_primitives::header::extension::v1::HeaderExtension=Default \
	--derive-for-type da_primitives::kate_commitment::KateCommitment=serde::Serialize \
	--derive-for-type da_primitives::kate_commitment::KateCommitment=serde::Deserialize \
	--derive-for-type da_primitives::kate_commitment::KateCommitment=Default \
	--derive-for-type da_primitives::asdr::data_lookup::DataLookup=serde::Serialize \
	--derive-for-type da_primitives::asdr::data_lookup::DataLookup=serde::Deserialize \
	--derive-for-type da_primitives::asdr::data_lookup::DataLookup=Default \
	--derive-for-type da_primitives::asdr::data_lookup::DataLookupIndexItem=serde::Serialize \
	--derive-for-type da_primitives::asdr::data_lookup::DataLookupIndexItem=serde::Deserialize \
	--derive-for-type da_primitives::asdr::AppId=serde::Serialize \
	--derive-for-type da_primitives::asdr::AppId=serde::Deserialize \
	--derive-for-type da_primitives::asdr::AppId=Default \
	--derive-for-type da_primitives::asdr::AppId=Copy \
	--derive-for-type da_primitives::asdr::AppId=derive_more::From \
	--url http://localhost:9933 \
	| sed -En "s/pub struct KateCommitment/#\[serde\(rename_all = \"camelCase\"\)\] \0/p" \
	| sed '1i \#\[allow(clippy::all)]' \
	| rustfmt --edition=2021 --emit=stdout > src/api_dev.rs
