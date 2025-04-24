use super::{
	cache::{CachedEntryEvents, CachedEvent, CachedEvents},
	chain_api,
};
use codec::Encode;
use da_runtime::UncheckedExtrinsic;
use frame_system_rpc_runtime_api::SystemFetchEventsParams;
use sc_service::RpcHandlers;
use sp_core::H256;
use sp_runtime::AccountId32;
use std::time::{Duration, Instant};
use transaction_rpc::common::events::DecodedEventData;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct TxIdentifier {
	pub block_hash: H256,
	pub tx_index: u32,
}

impl From<(H256, u32)> for TxIdentifier {
	fn from(value: (H256, u32)) -> Self {
		Self {
			block_hash: value.0,
			tx_index: value.1,
		}
	}
}

pub(crate) fn read_pallet_call_index(ext: &UncheckedExtrinsic) -> Option<(u8, u8)> {
	let ext = ext.function.encode();
	if ext.len() < 2 {
		return None;
	}
	let pallet_index = ext[0];
	let call_index = ext[1];

	Some((pallet_index, call_index))
}

pub(crate) struct Timer {
	now: Instant,
	// In sec
	duration: u64,
}

impl Timer {
	pub fn new(duration: u64) -> Self {
		Self {
			now: Instant::now(),
			duration,
		}
	}

	pub fn restart(&mut self) -> Instant {
		self.now = Instant::now();
		self.now
	}

	pub fn elapsed(&self) -> Duration {
		self.now.elapsed()
	}

	pub fn expired(&self) -> bool {
		self.elapsed().as_secs() > self.duration
	}

	pub fn duration(&self) -> u64 {
		self.duration
	}
}

pub(crate) async fn fetch_events(
	handlers: &RpcHandlers,
	block_hash: H256,
	params: SystemFetchEventsParams,
) -> Option<CachedEvents> {
	let rpc_events = chain_api::system_fetch_events(handlers, params, &block_hash).await?;

	if rpc_events.error != 0 {
		return None;
	}

	let entries = rpc_events.entries;

	let mut cached_events = Vec::<CachedEntryEvents>::new();
	for enc in &entries {
		let mut cached_entry = CachedEntryEvents {
			phase: enc.phase.clone(),
			events: Vec::with_capacity(enc.events.len()),
		};

		for enc_event in &enc.events {
			let data = CachedEvent::from_runtime_event(enc_event);
			cached_entry.events.push(data);
		}

		cached_events.push(cached_entry);
	}

	Some(CachedEvents(cached_events))
}

pub mod decoding {
	use frame_system_rpc_runtime_api::events::RuntimeEvent;

	use super::*;

	#[derive(codec::Decode)]
	struct DataSubmitted {
		pub who: AccountId32,
		pub data_hash: H256,
	}

	#[derive(codec::Decode)]
	struct MultisigExecuted {
		pub multisig: AccountId32,
		pub call_hash: H256,
		pub result: bool,
	}

	pub fn parse_decoded_event(ev: &RuntimeEvent) -> Option<DecodedEventData> {
		use codec::{decode_from_bytes, DecodeAll};
		use frame_system_rpc_runtime_api::events::event_id::*;

		let Some(decoded) = &ev.decoded else {
			return None;
		};

		let (pallet_id, event_id) = ev.emitted_index;

		match pallet_id {
			system::PALLET_ID => {
				if event_id == system::EXTRINSIC_SUCCESS {
					return Some(DecodedEventData::SystemExtrinsicSuccess);
				} else if event_id == system::EXTRINSIC_FAILED {
					return Some(DecodedEventData::SystemExtrinsicFailed);
				}
			},
			sudo::PALLET_ID => {
				if event_id == sudo::SUDID {
					let data = decode_from_bytes::<bool>(decoded.clone().into()).ok()?;
					return Some(DecodedEventData::SudoSudid(data));
				} else if event_id == sudo::SUDO_AS_DONE {
					let data = decode_from_bytes::<bool>(decoded.clone().into()).ok()?;
					return Some(DecodedEventData::SudoSudoAsDone(data));
				}
			},
			data_availability::PALLET_ID => {
				if event_id == data_availability::DATA_SUBMITTED {
					let value = DataSubmitted::decode_all(&mut decoded.as_slice()).ok()?;
					let data = transaction_rpc::common::events::DataSubmitted {
						who: std::format!("{}", value.who),
						data_hash: std::format!("{:?}", value.data_hash),
					};

					return Some(DecodedEventData::DataAvailabilityDataSubmitted(data));
				}
			},
			multisig::PALLET_ID => {
				if event_id == multisig::MULTISIG_EXECUTED {
					let data = MultisigExecuted::decode_all(&mut decoded.as_slice()).ok()?;
					let data = transaction_rpc::common::events::MultisigExecuted {
						multisig: std::format!("{}", data.multisig),
						call_hash: std::format!("{:?}", data.call_hash),
						result: data.result,
					};
					return Some(DecodedEventData::MultisigMultisigExecuted(data));
				}
			},
			proxy::PALLET_ID => {
				if event_id == proxy::PROXY_EXECUTED {
					let data = decode_from_bytes::<bool>(decoded.clone().into()).ok()?;
					return Some(DecodedEventData::ProxyProxyExecuted(data));
				}
			},
			_ => (),
		}

		None
	}
}
