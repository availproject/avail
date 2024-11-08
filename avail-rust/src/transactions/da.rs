use crate::api_dev::api::data_availability::calls::types::create_application_key::Key;
use crate::api_dev::api::data_availability::calls::types::submit_data::Data;
use crate::api_dev::api::runtime_types::frame_support::dispatch::DispatchFeeModifier;
use crate::api_dev::api::Call;
use crate::sdk::WaitFor;
use crate::{avail, ABlocksClient, ATxClient};

use avail::data_availability::calls::types as DataAvailabilityCalls;
use avail::data_availability::events as DataAvailabilityEvents;
use avail::sudo::events as SudoEvents;
use subxt::backend::rpc::RpcClient;
use subxt_signer::sr25519::Keypair;

use super::{options::Options, sign_and_submit_and_progress_transaction, TxResultDetails};

#[derive(Debug)]
pub struct SubmitDataTx {
	pub event: DataAvailabilityEvents::DataSubmitted,
	pub data: DataAvailabilityCalls::SubmitData,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct CreateApplicationKeyTx {
	pub event: DataAvailabilityEvents::ApplicationKeyCreated,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct SetApplicationKeyTx {
	pub event: DataAvailabilityEvents::ApplicationKeySet,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct SubmitBlockLengthProposalTx {
	pub event: DataAvailabilityEvents::BlockLengthProposalSubmitted,
	pub details: TxResultDetails,
}

#[derive(Debug)]
pub struct SetSubmitDataFeeModifierTx {
	pub event: DataAvailabilityEvents::SubmitDataFeeModifierSet,
	pub details: TxResultDetails,
}

#[derive(Clone)]
pub struct DataAvailability {
	tx_client: ATxClient,
	blocks_client: ABlocksClient,
	rpc_client: RpcClient,
}

impl DataAvailability {
	pub fn new(tx_client: ATxClient, rpc_client: RpcClient, blocks_client: ABlocksClient) -> Self {
		Self {
			tx_client,
			blocks_client,
			rpc_client,
		}
	}

	pub async fn submit_data(
		&self,
		data: Data,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SubmitDataTx, String> {
		let call = avail::tx().data_availability().submit_data(data);
		let details = sign_and_submit_and_progress_transaction(
			&self.tx_client,
			&self.blocks_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = details
			.events
			.find_first::<DataAvailabilityEvents::DataSubmitted>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find DataSubmitted event"));
		};

		let block = details
			.fetch_block(&self.blocks_client)
			.await
			.map_err(|e| e.to_string())?;
		let mut data =
			block.transaction_by_hash_static::<DataAvailabilityCalls::SubmitData>(details.tx_hash);
		let data = data
			.pop()
			.ok_or(String::from("Failed to find transaction data"))?
			.value;

		Ok(SubmitDataTx {
			event,
			data,
			details,
		})
	}

	pub async fn create_application_key(
		&self,
		key: Key,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<CreateApplicationKeyTx, String> {
		let call = avail::tx().data_availability().create_application_key(key);
		let details = sign_and_submit_and_progress_transaction(
			&self.tx_client,
			&self.blocks_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = details
			.events
			.find_first::<DataAvailabilityEvents::ApplicationKeyCreated>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find ApplicationKeyCreated event"));
		};

		Ok(CreateApplicationKeyTx { event, details })
	}

	pub async fn set_application_key(
		&self,
		old_key: Key,
		new_key: Key,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SetApplicationKeyTx, String> {
		let call = Call::DataAvailability(
			avail::runtime_types::da_control::pallet::Call::set_application_key {
				old_key,
				new_key,
			},
		);
		let sudo = avail::tx().sudo().sudo(call);
		let details = sign_and_submit_and_progress_transaction(
			&self.tx_client,
			&self.blocks_client,
			&self.rpc_client,
			account,
			sudo,
			wait_for,
			options,
		)
		.await?;

		let sudo_event = details.events.find_first::<SudoEvents::Sudid>();
		let Some(sudo_event) = sudo_event.ok().flatten() else {
			return Err(String::from("Failed to find Sudid event"));
		};

		if let Err(error) = sudo_event.sudo_result {
			return Err(std::format!("{:?}", error));
		}

		let event = details
			.events
			.find_first::<DataAvailabilityEvents::ApplicationKeySet>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from("Failed to find ApplicationKeySet event"));
		};

		Ok(SetApplicationKeyTx { event, details })
	}

	pub async fn submit_block_length_proposal(
		&self,
		rows: u32,
		cols: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SubmitBlockLengthProposalTx, String> {
		let call = Call::DataAvailability(
			avail::runtime_types::da_control::pallet::Call::submit_block_length_proposal {
				rows,
				cols,
			},
		);
		let sudo = avail::tx().sudo().sudo(call);
		let details = sign_and_submit_and_progress_transaction(
			&self.tx_client,
			&self.blocks_client,
			&self.rpc_client,
			account,
			sudo,
			wait_for,
			options,
		)
		.await?;

		let sudo_event = details.events.find_first::<SudoEvents::Sudid>();
		let Some(sudo_event) = sudo_event.ok().flatten() else {
			return Err(String::from("Failed to find Sudid event"));
		};

		if let Err(error) = sudo_event.sudo_result {
			return Err(std::format!("{:?}", error));
		}

		let event = details
			.events
			.find_first::<DataAvailabilityEvents::BlockLengthProposalSubmitted>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from(
				"Failed to find BlockLengthProposalSubmitted event",
			));
		};

		Ok(SubmitBlockLengthProposalTx { event, details })
	}

	pub async fn set_submit_data_fee_modifier(
		&self,
		modifier: DispatchFeeModifier,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SetSubmitDataFeeModifierTx, String> {
		let call = Call::DataAvailability(
			avail::runtime_types::da_control::pallet::Call::set_submit_data_fee_modifier {
				modifier,
			},
		);
		let sudo = avail::tx().sudo().sudo(call);
		let details = sign_and_submit_and_progress_transaction(
			&self.tx_client,
			&self.blocks_client,
			&self.rpc_client,
			account,
			sudo,
			wait_for,
			options,
		)
		.await?;

		let sudo_event = details.events.find_first::<SudoEvents::Sudid>();
		let Some(sudo_event) = sudo_event.ok().flatten() else {
			return Err(String::from("Failed to find Sudid event"));
		};

		if let Err(error) = sudo_event.sudo_result {
			return Err(std::format!("{:?}", error));
		}

		let event = details
			.events
			.find_first::<DataAvailabilityEvents::SubmitDataFeeModifierSet>();
		let Some(event) = event.ok().flatten() else {
			return Err(String::from(
				"Failed to find SubmitDataFeeModifierSet event",
			));
		};

		Ok(SetSubmitDataFeeModifierTx { event, details })
	}
}
