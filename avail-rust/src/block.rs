use primitive_types::H256;
use subxt::blocks::StaticExtrinsic;

use crate::{ABlock, ABlocksClient, AExtrinsicDetails, AExtrinsics, AFoundExtrinsic};

pub struct Block {
	pub block: ABlock,
	pub transactions: AExtrinsics,
}

impl Block {
	pub async fn new(client: &ABlocksClient, block_hash: H256) -> Result<Self, subxt::Error> {
		let (block, transactions) = block_transactions(client, block_hash).await?;
		Ok(Self {
			block,
			transactions,
		})
	}

	pub fn transaction_count(&self) -> usize {
		return transaction_count(&self.transactions);
	}

	pub fn transaction_by_signer(&self, signer: &str) -> Vec<AExtrinsicDetails> {
		return transaction_by_signer(&self.transactions, signer);
	}

	pub fn transaction_by_signer_static<T: StaticExtrinsic>(
		&self,
		signer: &str,
	) -> Vec<AFoundExtrinsic<T>> {
		transaction_by_signer_static::<T>(&self.transactions, signer)
	}

	pub fn transaction_by_index(&self, tx_index: u32) -> Result<AExtrinsicDetails, String> {
		return transaction_by_index(&self.transactions, tx_index);
	}

	pub fn transaction_by_index_static<T: StaticExtrinsic>(
		&self,
		tx_index: u32,
	) -> Result<AFoundExtrinsic<T>, String> {
		transaction_by_index_static::<T>(&self.transactions, tx_index)
	}

	pub fn transaction_by_hash(&self, tx_hash: H256) -> Vec<AExtrinsicDetails> {
		return transaction_by_hash(&self.transactions, tx_hash);
	}

	pub fn transaction_by_hash_static<T: StaticExtrinsic>(
		&self,
		tx_hash: H256,
	) -> Vec<AFoundExtrinsic<T>> {
		transaction_by_hash_static::<T>(&self.transactions, tx_hash)
	}
}

pub async fn block_transactions(
	client: &ABlocksClient,
	block_hash: H256,
) -> Result<(ABlock, AExtrinsics), subxt::Error> {
	let block = client.at(block_hash).await?;
	let transactions = block.extrinsics().await?;
	Ok((block, transactions))
}

pub fn transaction_count(transactions: &AExtrinsics) -> usize {
	return transactions.len();
}

pub fn transaction_by_signer(transactions: &AExtrinsics, signer: &str) -> Vec<AExtrinsicDetails> {
	let mut result: Vec<AExtrinsicDetails> = Vec::new();
	for tx in transactions.iter() {
		if tx.signature_bytes() == Some(signer.as_bytes()) {
			result.push(tx)
		}
	}

	result
}

pub fn transaction_by_signer_static<T: StaticExtrinsic>(
	transactions: &AExtrinsics,
	signer: &str,
) -> Vec<AFoundExtrinsic<T>> {
	let mut result: Vec<AFoundExtrinsic<T>> = Vec::new();
	for tx in transactions.find::<T>() {
		if let Ok(tx) = tx {
			if tx.details.signature_bytes() == Some(signer.as_bytes()) {
				result.push(tx)
			}
		}
	}

	result
}

pub fn transaction_by_index(
	transactions: &AExtrinsics,
	tx_index: u32,
) -> Result<AExtrinsicDetails, String> {
	transactions
		.iter()
		.skip(tx_index as usize)
		.next()
		.ok_or(String::from("Transaction not found"))
}

pub fn transaction_by_index_static<T: StaticExtrinsic>(
	transactions: &AExtrinsics,
	tx_index: u32,
) -> Result<AFoundExtrinsic<T>, String> {
	let tx = transactions
		.find::<T>()
		.skip(tx_index as usize)
		.next()
		.ok_or(String::from("Transaction not found"))?;

	tx.map_err(|_| String::from("Transaction has different type"))
}

pub fn transaction_by_hash(transactions: &AExtrinsics, tx_hash: H256) -> Vec<AExtrinsicDetails> {
	let mut result: Vec<AExtrinsicDetails> = Vec::new();
	for tx in transactions.iter() {
		if tx.hash() == tx_hash {
			result.push(tx)
		}
	}

	result
}

pub fn transaction_by_hash_static<T: StaticExtrinsic>(
	transactions: &AExtrinsics,
	tx_hash: H256,
) -> Vec<AFoundExtrinsic<T>> {
	let mut result: Vec<AFoundExtrinsic<T>> = Vec::new();
	for tx in transactions.find::<T>() {
		if let Ok(tx) = tx {
			if tx.details.hash() == tx_hash {
				result.push(tx);
			}
		}
	}

	result
}
