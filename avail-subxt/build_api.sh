#!/bin/sh
echo "⛓ Installing SubXt..."
cargo install --git https://github.com/paritytech/subxt --tag v0.27.1 || true 
echo "🔨 Generating Avail-SubXt API from localhost..."
subxt codegen \
	--derive Clone \
	--derive PartialEq \
	--derive Eq \
	--derive-for-type da_primitives::header::extension::HeaderExtension=serde::Serialize \
	--derive-for-type da_primitives::header::extension::HeaderExtension=serde::Deserialize  \
	--derive-for-type da_primitives::header::extension::v1::HeaderExtension=serde::Serialize  \
	--derive-for-type da_primitives::header::extension::v1::HeaderExtension=serde::Deserialize  \
	--derive-for-type da_primitives::header::extension::v1::HeaderExtension=Default \
	--derive-for-type da_primitives::header::extension::v2::HeaderExtension=serde::Serialize  \
	--derive-for-type da_primitives::header::extension::v2::HeaderExtension=serde::Deserialize  \
	--derive-for-type da_primitives::header::extension::v2::HeaderExtension=Default \
	--derive-for-type da_primitives::kate_commitment::v1::KateCommitment=serde::Serialize \
	--derive-for-type da_primitives::kate_commitment::v1::KateCommitment=serde::Deserialize \
	--derive-for-type da_primitives::kate_commitment::v1::KateCommitment=Default \
	--derive-for-type da_primitives::kate_commitment::v2::KateCommitment=serde::Serialize \
	--derive-for-type da_primitives::kate_commitment::v2::KateCommitment=serde::Deserialize \
	--derive-for-type da_primitives::kate_commitment::v2::KateCommitment=Default \
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
	| sed -En "s/pub struct KateCommitment/#\[serde\(rename_all = \"camelCase\"\)\] \0/gp" \
	| sed -E '1i \#\[allow(clippy::all)]' \
	| rustfmt --edition=2021 --emit=stdout > src/api_dev.rs
echo "🎁 Avail-SubXt API generated in 'src/api_dev.rs'"
