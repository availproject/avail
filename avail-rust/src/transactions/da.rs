use crate::api_dev::api::data_availability::calls::types::create_application_key::Key;
use crate::api_dev::api::data_availability::calls::types::submit_data::Data;
use crate::api_dev::api::runtime_types::frame_support::dispatch::DispatchFeeModifier;
use crate::api_dev::api::Call;
use crate::sdk::WaitFor;
use crate::{avail, AOnlineClient};

use avail::data_availability::calls::types as DataAvailabilityCalls;
use avail::data_availability::events as DataAvailabilityEvents;
use avail::sudo::events as SudoEvents;
use subxt::backend::rpc::RpcClient;
use subxt_signer::sr25519::Keypair;

use super::{find_event_or_return_error, TransactionFailed};
use super::{options::Options, progress_and_parse_transaction, TransactionDetails};

#[derive(Debug)]
pub struct SubmitDataTx {
	pub event: DataAvailabilityEvents::DataSubmitted,
	pub data: DataAvailabilityCalls::SubmitData,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct CreateApplicationKeyTx {
	pub event: DataAvailabilityEvents::ApplicationKeyCreated,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct SetApplicationKeyTx {
	pub event: DataAvailabilityEvents::ApplicationKeySet,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct SubmitBlockLengthProposalTx {
	pub event: DataAvailabilityEvents::BlockLengthProposalSubmitted,
	pub details: TransactionDetails,
}

#[derive(Debug)]
pub struct SetSubmitDataFeeModifierTx {
	pub event: DataAvailabilityEvents::SubmitDataFeeModifierSet,
	pub details: TransactionDetails,
}

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

	pub async fn submit_data(
		&self,
		data: Data,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SubmitDataTx, TransactionFailed> {
		let call = avail::tx().data_availability().submit_data(data);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = find_event_or_return_error::<DataAvailabilityEvents::DataSubmitted>(
			"Failed to find DataSubmitted event",
			&details,
		)?;

		let block = details.fetch_block(&self.online_client).await;
		let block = block.map_err(|e| TransactionFailed {
			reason: e.into(),
			details: Some(details.clone()),
		})?;

		let data = block
			.transaction_by_index_static::<DataAvailabilityCalls::SubmitData>(details.tx_index);
		let data = data
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
	) -> Result<CreateApplicationKeyTx, TransactionFailed> {
		let call = avail::tx().data_availability().create_application_key(key);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			call,
			wait_for,
			options,
		)
		.await?;

		let event = find_event_or_return_error::<DataAvailabilityEvents::ApplicationKeyCreated>(
			"Failed to find ApplicationKeyCreated event",
			&details,
		)?;

		Ok(CreateApplicationKeyTx { event, details })
	}

	pub async fn set_application_key(
		&self,
		old_key: Key,
		new_key: Key,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SetApplicationKeyTx, TransactionFailed> {
		let call = Call::DataAvailability(
			avail::runtime_types::da_control::pallet::Call::set_application_key {
				old_key,
				new_key,
			},
		);
		let sudo = avail::tx().sudo().sudo(call);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			sudo,
			wait_for,
			options,
		)
		.await?;

		let sudo_event = find_event_or_return_error::<SudoEvents::Sudid>(
			"Failed to find Sudid event",
			&details,
		)?;

		if let Err(error) = sudo_event.sudo_result {
			return Err(TransactionFailed::from((
				std::format!("{:?}", error),
				details.clone(),
			)));
		}

		let event = find_event_or_return_error::<DataAvailabilityEvents::ApplicationKeySet>(
			"Failed to find ApplicationKeySet event",
			&details,
		)?;

		Ok(SetApplicationKeyTx { event, details })
	}

	pub async fn submit_block_length_proposal(
		&self,
		rows: u32,
		cols: u32,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SubmitBlockLengthProposalTx, TransactionFailed> {
		let call = Call::DataAvailability(
			avail::runtime_types::da_control::pallet::Call::submit_block_length_proposal {
				rows,
				cols,
			},
		);
		let sudo = avail::tx().sudo().sudo(call);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			sudo,
			wait_for,
			options,
		)
		.await?;

		let sudo_event = find_event_or_return_error::<SudoEvents::Sudid>(
			"Failed to find Sudid event",
			&details,
		)?;

		if let Err(error) = sudo_event.sudo_result {
			return Err(TransactionFailed::from((
				std::format!("{:?}", error),
				details.clone(),
			)));
		}

		let event =
			find_event_or_return_error::<DataAvailabilityEvents::BlockLengthProposalSubmitted>(
				"Failed to find BlockLengthProposalSubmitted event",
				&details,
			)?;

		Ok(SubmitBlockLengthProposalTx { event, details })
	}

	pub async fn set_submit_data_fee_modifier(
		&self,
		modifier: DispatchFeeModifier,
		wait_for: WaitFor,
		account: &Keypair,
		options: Option<Options>,
	) -> Result<SetSubmitDataFeeModifierTx, TransactionFailed> {
		let call = Call::DataAvailability(
			avail::runtime_types::da_control::pallet::Call::set_submit_data_fee_modifier {
				modifier,
			},
		);
		let sudo = avail::tx().sudo().sudo(call);
		let details = progress_and_parse_transaction(
			&self.online_client,
			&self.rpc_client,
			account,
			sudo,
			wait_for,
			options,
		)
		.await?;

		let sudo_event = find_event_or_return_error::<SudoEvents::Sudid>(
			"Failed to find Sudid event",
			&details,
		)?;

		if let Err(error) = sudo_event.sudo_result {
			return Err(TransactionFailed::from((
				std::format!("{:?}", error),
				details.clone(),
			)));
		}

		let event = find_event_or_return_error::<DataAvailabilityEvents::SubmitDataFeeModifierSet>(
			"Failed to find SubmitDataFeeModifierSet event",
			&details,
		)?;

		Ok(SetSubmitDataFeeModifierTx { event, details })
	}
}
