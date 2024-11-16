use crate::rpcs::{get_best_block_hash, get_finalized_head};
use crate::{
	avail::data_availability::calls::types as DataAvailabilityCalls,
	primitives::block::extrinsics_params::CheckAppId,
};
use crate::{ABlock, AExtrinsicDetails, AExtrinsics, AFoundExtrinsic, AOnlineClient};

use primitive_types::H256;
use subxt::backend::rpc::RpcClient;
use subxt::backend::StreamOfResults;
use subxt::blocks::StaticExtrinsic;
use subxt::storage::StorageKeyValuePair;
use subxt::utils::Yes;

pub struct Block {
	pub block: ABlock,
	pub transactions: AExtrinsics,
}

impl Block {
	pub async fn new(client: &AOnlineClient, block_hash: H256) -> Result<Self, subxt::Error> {
		let (block, transactions) = transactions(client, block_hash).await?;
		Ok(Self {
			block,
			transactions,
		})
	}

	pub async fn new_best_block(
		online_client: &AOnlineClient,
		rpc_client: &RpcClient,
	) -> Result<Self, subxt::Error> {
		let best_hash = get_best_block_hash(rpc_client).await?;
		Self::new(online_client, best_hash).await
	}

	pub async fn new_finalized_block(
		online_client: &AOnlineClient,
		rpc_client: &RpcClient,
	) -> Result<Self, subxt::Error> {
		let best_hash = get_finalized_head(rpc_client).await?;
		Self::new(online_client, best_hash).await
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

	pub fn transaction_by_index(&self, tx_index: u32) -> Option<AExtrinsicDetails> {
		return transaction_by_index(&self.transactions, tx_index);
	}

	pub fn transaction_by_index_static<T: StaticExtrinsic>(
		&self,
		tx_index: u32,
	) -> Option<AFoundExtrinsic<T>> {
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

	pub fn transaction_by_app_id(&self, app_id: u32) -> Vec<AExtrinsicDetails> {
		transaction_by_app_id(&self.transactions, app_id)
	}

	pub fn transaction_by_app_id_static<T: StaticExtrinsic>(
		&self,
		app_id: u32,
	) -> Vec<AFoundExtrinsic<T>> {
		transaction_by_app_id_static(&self.transactions, app_id)
	}

	pub fn submit_data_all(&self) -> Vec<DataSubmission> {
		submit_data_all(&self.transactions)
	}

	pub fn submit_data_by_signer(&self, signer: &str) -> Vec<DataSubmission> {
		submit_data_by_signer(&self.transactions, signer)
	}

	pub fn submit_data_by_index(&self, tx_index: u32) -> Option<DataSubmission> {
		submit_data_by_index(&self.transactions, tx_index)
	}

	pub fn submit_data_by_hash(&self, tx_hash: H256) -> Option<DataSubmission> {
		submit_data_by_hash(&self.transactions, tx_hash)
	}

	pub fn submit_data_by_app_id(&self, app_id: u32) -> Option<DataSubmission> {
		submit_data_by_app_id(&self.transactions, app_id)
	}

	pub async fn storage_fetch<'address, T>(
		&self,
		address: &'address T,
	) -> Result<Option<<T as subxt::storage::Address>::Target>, subxt::Error>
	where
		T: subxt::storage::Address<IsFetchable = Yes> + 'address,
	{
		self.block.storage().fetch(address).await
	}

	pub async fn storage_fetch_or_default<'address, T>(
		&self,
		address: &'address T,
	) -> Result<<T as subxt::storage::Address>::Target, subxt::Error>
	where
		T: subxt::storage::Address<IsFetchable = Yes, IsDefaultable = Yes> + 'address,
	{
		self.block.storage().fetch_or_default(address).await
	}

	pub async fn storage_iter<T>(
		&self,
		address: T,
	) -> Result<StreamOfResults<StorageKeyValuePair<T>>, subxt::Error>
	where
		T: subxt::storage::Address<IsIterable = Yes> + 'static,
		T::Keys: 'static + Sized,
	{
		self.block.storage().iter(address).await
	}
}

pub async fn transactions(
	client: &AOnlineClient,
	block_hash: H256,
) -> Result<(ABlock, AExtrinsics), subxt::Error> {
	let block = client.blocks().at(block_hash).await?;
	let transactions = block.extrinsics().await?;
	Ok((block, transactions))
}

pub fn transaction_count(transactions: &AExtrinsics) -> usize {
	return transactions.len();
}

pub fn transaction_all_static<T: StaticExtrinsic>(
	transactions: &AExtrinsics,
) -> Vec<AFoundExtrinsic<T>> {
	transactions.find::<T>().flatten().collect()
}

pub fn submit_data_all(transactions: &AExtrinsics) -> Vec<DataSubmission> {
	transaction_all_static::<DataAvailabilityCalls::SubmitData>(transactions)
		.into_iter()
		.map(|tx| DataSubmission::from_static(tx))
		.collect()
}

pub fn transaction_by_signer(transactions: &AExtrinsics, signer: &str) -> Vec<AExtrinsicDetails> {
	transactions
		.iter()
		.filter(|tx| tx.signature_bytes() == Some(signer.as_bytes()))
		.collect()
}

pub fn transaction_by_signer_static<T: StaticExtrinsic>(
	transactions: &AExtrinsics,
	signer: &str,
) -> Vec<AFoundExtrinsic<T>> {
	transactions
		.find::<T>()
		.flatten()
		.filter(|tx| tx.details.signature_bytes() == Some(signer.as_bytes()))
		.collect()
}

pub fn submit_data_by_signer(transactions: &AExtrinsics, signer: &str) -> Vec<DataSubmission> {
	transaction_by_signer_static::<DataAvailabilityCalls::SubmitData>(transactions, signer)
		.into_iter()
		.map(|tx| DataSubmission::from_static(tx))
		.collect()
}

pub fn transaction_by_index(
	transactions: &AExtrinsics,
	tx_index: u32,
) -> Option<AExtrinsicDetails> {
	transactions.iter().find(|tx| tx.index() == tx_index)
}

pub fn transaction_by_index_static<T: StaticExtrinsic>(
	transactions: &AExtrinsics,
	tx_index: u32,
) -> Option<AFoundExtrinsic<T>> {
	let details = transactions.iter().find(|tx| tx.index() == tx_index)?;
	let value = details.as_extrinsic::<T>().ok()??;

	Some(AFoundExtrinsic { details, value })
}

pub fn submit_data_by_index(transactions: &AExtrinsics, tx_index: u32) -> Option<DataSubmission> {
	transaction_by_index_static::<DataAvailabilityCalls::SubmitData>(transactions, tx_index)
		.map(|tx| DataSubmission::from_static(tx))
}

pub fn transaction_by_hash(transactions: &AExtrinsics, tx_hash: H256) -> Vec<AExtrinsicDetails> {
	transactions
		.iter()
		.filter(|tx| tx.hash() == tx_hash)
		.collect()
}

pub fn transaction_by_hash_static<T: StaticExtrinsic>(
	transactions: &AExtrinsics,
	tx_hash: H256,
) -> Vec<AFoundExtrinsic<T>> {
	transactions
		.find::<T>()
		.flatten()
		.filter(|tx| tx.details.hash() == tx_hash)
		.collect()
}

pub fn submit_data_by_hash(transactions: &AExtrinsics, tx_hash: H256) -> Option<DataSubmission> {
	let all_submissions: Vec<DataSubmission> =
		transaction_by_hash_static::<DataAvailabilityCalls::SubmitData>(transactions, tx_hash)
			.into_iter()
			.map(|tx| DataSubmission::from_static(tx))
			.collect();

	all_submissions.into_iter().next()
}

pub fn transaction_by_app_id(transactions: &AExtrinsics, app_id: u32) -> Vec<AExtrinsicDetails> {
	transactions
		.iter()
		.filter(|tx| read_app_id(tx) == Some(app_id))
		.collect()
}

pub fn transaction_by_app_id_static<T: StaticExtrinsic>(
	transactions: &AExtrinsics,
	app_id: u32,
) -> Vec<AFoundExtrinsic<T>> {
	transactions
		.find::<T>()
		.flatten()
		.filter(|tx| read_app_id(&tx.details) == Some(app_id))
		.collect()
}

pub fn submit_data_by_app_id(transactions: &AExtrinsics, app_id: u32) -> Option<DataSubmission> {
	let all_submissions: Vec<DataSubmission> =
		transaction_by_app_id_static::<DataAvailabilityCalls::SubmitData>(transactions, app_id)
			.into_iter()
			.map(|tx| DataSubmission::from_static(tx))
			.collect();

	all_submissions.into_iter().next()
}

pub fn read_app_id(transaction: &AExtrinsicDetails) -> Option<u32> {
	transaction
		.signed_extensions()?
		.find::<CheckAppId>()
		.ok()?
		.and_then(|e| Some(e.0))
}

#[derive(Debug, Clone)]
pub struct DataSubmission {
	pub tx_hash: H256,
	pub tx_index: u32,
	pub data: Vec<u8>,
	pub tx_signer: Vec<u8>,
	pub app_id: u32,
}

impl DataSubmission {
	pub fn from_static(tx: AFoundExtrinsic<DataAvailabilityCalls::SubmitData>) -> Self {
		let tx_hash = tx.details.hash();
		let tx_index = tx.details.index();
		let tx_signer = tx
			.details
			.signature_bytes()
			.expect("DA can only be executed signed")
			.to_vec();
		let app_id = read_app_id(&tx.details).expect("There must be an app id");
		let data = tx.value.data.0.clone();
		Self {
			tx_hash,
			tx_index,
			data,
			tx_signer,
			app_id,
		}
	}
}
