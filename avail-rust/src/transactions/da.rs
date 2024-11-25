use crate::api_dev::api::data_availability::calls::types::create_application_key::Key;
use crate::api_dev::api::data_availability::calls::types::submit_data::Data;
use crate::{avail, AOnlineClient};

use super::Transaction;
use subxt::backend::rpc::RpcClient;

pub type SubmitDataCall = avail::data_availability::calls::types::SubmitData;
pub type CreateApplicationKeyCall = avail::data_availability::calls::types::CreateApplicationKey;

#[derive(Clone)]
pub struct DataAvailability {
	online_client: AOnlineClient,
	rpc_client: RpcClient,
}

impl DataAvailability {
	pub fn new(online_client: AOnlineClient, rpc_client: RpcClient) -> Self {
		Self {
			online_client,
			rpc_client,
		}
	}

	pub fn submit_data(&self, data: Vec<u8>) -> Transaction<SubmitDataCall> {
		let data = Data { 0: data };
		let payload = avail::tx().data_availability().submit_data(data);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}

	pub fn create_application_key(&self, key: Vec<u8>) -> Transaction<CreateApplicationKeyCall> {
		let key = Key { 0: key };
		let payload = avail::tx().data_availability().create_application_key(key);
		Transaction::new(self.online_client.clone(), self.rpc_client.clone(), payload)
	}
}
