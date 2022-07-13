#![cfg(test)]

use crate::NomadProof;
use primitive_types::H256;
use std::{fs::File, io::Read};

/// Struct representing a single merkle test case
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct MerkleTestCase {
	/// Test case name
	pub test_name: String,
	/// Leaves of merkle tree
	pub leaves: Vec<String>,
	/// Proofs for leaves in tree
	pub proofs: Vec<NomadProof>,
	/// Root of tree
	pub expected_root: H256,
}

/// Reads merkle test case json file and returns a vector of `MerkleTestCase`s
pub fn load_merkle_test_json() -> Vec<MerkleTestCase> {
	let mut file = File::open("fixtures/merkle.json").unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();
	serde_json::from_str(&data).unwrap()
}
