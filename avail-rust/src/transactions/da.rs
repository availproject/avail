use crate::api_dev::api::data_availability::calls::types::create_application_key::Key;
use crate::api_dev::api::data_availability::calls::types::submit_data::Data;
use crate::api_dev::api::runtime_types::frame_support::dispatch::DispatchFeeModifier;
use crate::api_dev::api::Call;
use crate::rpcs::Rpc;
use crate::sdk::WaitFor;
use crate::utils_raw::{fetch_transaction, progress_transaction};
use crate::{avail, AvailBlocksClient, AvailConfig, BlockHash, TxApi};

use subxt::blocks::ExtrinsicEvents;
use subxt_signer::sr25519::Keypair;

use avail::data_availability::calls::types as DataAvailabilityCalls;
use avail::data_availability::events as DataAvailabilityEvents;
use avail::sudo::events as SudoEvents;

use super::options::{from_options_to_params, Options};
use super::{block_and_tx_hash, progress_transaction_ex};

#[derive(Debug)]
pub struct SubmitDataTxSuccess {
	pub event: DataAvailabilityEvents::DataSubmitted,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_data: DataAvailabilityCalls::SubmitData,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct CreateApplicationKeyTxSuccess {
	pub event: DataAvailabilityEvents::ApplicationKeyCreated,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct SetApplicationKeyTxSuccess {
	pub event: DataAvailabilityEvents::ApplicationKeySet,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct SubmitBlockLengthProposalTxSuccess {
	pub event: DataAvailabilityEvents::BlockLengthProposalSubmitted,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Debug)]
pub struct SetSubmitDataFeeModifierTxSuccess {
	pub event: DataAvailabilityEvents::SubmitDataFeeModifierSet,
	pub events: ExtrinsicEvents<AvailConfig>,
	pub tx_hash: BlockHash,
	pub tx_index: u32,
	pub block_hash: BlockHash,
	pub block_number: u32,
}

#[derive(Clone)]
pub struct DataAvailability {
	api: TxApi,
	rpc_client: Rpc,
	blocks: AvailBlocksClient,
}

impl DataAvailability {
	pub fn new(api: TxApi, rpc_client: Rpc, blocks: AvailBlocksClient) -> Self {
		Self {
			api,
			rpc_client,
			blocks,
		}
	}

	pub async fn submit_data(
		&self,
		data: Data,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SubmitDataTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().data_availability().submit_data(data);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<DataAvailabilityEvents::DataSubmitted>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find DataSubmitted event"));
		};

		let tx_data = tx_data_da_submit_data(block_hash, tx_hash, &self.blocks).await?;

		Ok(SubmitDataTxSuccess {
			event,
			events,
			tx_data,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn create_application_key(
		&self,
		key: Key,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<CreateApplicationKeyTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = avail::tx().data_availability().create_application_key(key);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&call, account, params)
			.await;

		let (events, data) =
			progress_transaction_ex(maybe_tx_progress, wait_for, &self.blocks).await?;
		let (block_hash, block_number, tx_hash, tx_index) = data;

		let event = events.find_first::<DataAvailabilityEvents::ApplicationKeyCreated>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find ApplicationKeyCreated event"));
		};

		Ok(CreateApplicationKeyTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn set_application_key(
		&self,
		old_key: Key,
		new_key: Key,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SetApplicationKeyTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = Call::DataAvailability(
			avail::runtime_types::da_control::pallet::Call::set_application_key {
				old_key,
				new_key,
			},
		);
		let sudo = avail::tx().sudo().sudo(call);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&sudo, account, params)
			.await;

		let transaction = progress_transaction(maybe_tx_progress, wait_for).await;
		let tx_in_block = match transaction {
			Ok(tx_in_block) => tx_in_block,
			Err(message) => return Err(message),
		};

		let events = match tx_in_block.wait_for_success().await {
			Ok(e) => e,
			Err(error) => return Err(error.to_string()),
		};

		let sudo_event = events.find_first::<SudoEvents::Sudid>();
		let Some(sudo_event) = sudo_event.ok().flatten() else {
			return Err(String::from("Failed to find Sudid event"));
		};

		if let Err(error) = sudo_event.sudo_result {
			return Err(std::format!("{:?}", error));
		}

		let event = events.find_first::<DataAvailabilityEvents::ApplicationKeySet>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find ApplicationKeySet event"));
		};

		let (block_hash, block_number, tx_hash, tx_index) =
			block_and_tx_hash(&tx_in_block, &events, &self.blocks).await?;

		Ok(SetApplicationKeyTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn submit_block_length_proposal(
		&self,
		rows: u32,
		cols: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SubmitBlockLengthProposalTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = Call::DataAvailability(
			avail::runtime_types::da_control::pallet::Call::submit_block_length_proposal {
				rows,
				cols,
			},
		);
		let sudo = avail::tx().sudo().sudo(call);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&sudo, account, params)
			.await;

		let transaction = progress_transaction(maybe_tx_progress, wait_for).await;
		let tx_in_block = match transaction {
			Ok(tx_in_block) => tx_in_block,
			Err(message) => return Err(message),
		};

		let events = match tx_in_block.wait_for_success().await {
			Ok(e) => e,
			Err(error) => return Err(error.to_string()),
		};

		let sudo_event = events.find_first::<SudoEvents::Sudid>();
		let Some(sudo_event) = sudo_event.ok().flatten() else {
			return Err(String::from("Failed to find Sudid event"));
		};

		if let Err(error) = sudo_event.sudo_result {
			return Err(std::format!("{:?}", error));
		}

		let event = events.find_first::<DataAvailabilityEvents::BlockLengthProposalSubmitted>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from(
				"Failed to find BlockLengthProposalSubmitted event",
			));
		};

		let (block_hash, block_number, tx_hash, tx_index) =
			block_and_tx_hash(&tx_in_block, &events, &self.blocks).await?;

		Ok(SubmitBlockLengthProposalTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}

	pub async fn set_submit_data_fee_modifier(
		&self,
		modifier: DispatchFeeModifier,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SetSubmitDataFeeModifierTxSuccess, String> {
		let account_id = account.public_key().to_account_id();
		let params =
			from_options_to_params(options, &self.rpc_client, account_id, &self.blocks).await?;
		let call = Call::DataAvailability(
			avail::runtime_types::da_control::pallet::Call::set_submit_data_fee_modifier {
				modifier,
			},
		);
		let sudo = avail::tx().sudo().sudo(call);

		let maybe_tx_progress = self
			.api
			.sign_and_submit_then_watch(&sudo, account, params)
			.await;

		let transaction = progress_transaction(maybe_tx_progress, wait_for).await;
		let tx_in_block = match transaction {
			Ok(tx_in_block) => tx_in_block,
			Err(message) => return Err(message),
		};

		let events = match tx_in_block.wait_for_success().await {
			Ok(e) => e,
			Err(error) => return Err(error.to_string()),
		};

		let sudo_event = events.find_first::<SudoEvents::Sudid>();
		let Some(sudo_event) = sudo_event.ok().flatten() else {
			return Err(String::from("Failed to find Sudid event"));
		};

		if let Err(error) = sudo_event.sudo_result {
			return Err(std::format!("{:?}", error));
		}

		let event = events.find_first::<DataAvailabilityEvents::SubmitDataFeeModifierSet>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from(
				"Failed to find SubmitDataFeeModifierSet event",
			));
		};

		let (block_hash, block_number, tx_hash, tx_index) =
			block_and_tx_hash(&tx_in_block, &events, &self.blocks).await?;

		Ok(SetSubmitDataFeeModifierTxSuccess {
			event,
			events,
			tx_hash,
			tx_index,
			block_hash,
			block_number,
		})
	}
}

pub async fn tx_data_da_submit_data(
	block_hash: BlockHash,
	tx_hash: BlockHash,
	blocks: &AvailBlocksClient,
) -> Result<DataAvailabilityCalls::SubmitData, String> {
	let transaction =
		fetch_transaction::<DataAvailabilityCalls::SubmitData>(block_hash, tx_hash, blocks).await;
	let transaction = transaction.map_err(|err| err.to_string())?;
	Ok(transaction.value)
}
