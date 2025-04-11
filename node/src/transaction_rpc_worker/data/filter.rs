use std::sync::Arc;

use super::cache::{CachedEvents, SharedCache};
use crate::transaction_rpc_worker::read_pallet_call_index;
use avail_core::OpaqueExtrinsic;
use codec::Encode;
use da_runtime::UncheckedExtrinsic;
use sc_telemetry::log;
use sp_core::H256;
use sp_core::{Blake2Hasher, Hasher};
use sp_runtime::MultiAddress;
use transaction_rpc::{block_overview, HashIndex};

pub(crate) fn filter_pallet_call_id(
	ext: &UncheckedExtrinsic,
	filter: &block_overview::Filter,
) -> Option<(u8, u8)> {
	let Some((pallet_id, call_id)) = read_pallet_call_index(&ext) else {
		return None;
	};

	if filter.pallet_id.is_some_and(|x| x != pallet_id) {
		return None;
	};

	if filter.call_id.is_some_and(|x| x != call_id) {
		return None;
	};

	Some((pallet_id, call_id))
}

pub(crate) fn filter_signature(
	ext: &UncheckedExtrinsic,
	filter: &block_overview::Filter,
) -> Option<Option<block_overview::TransactionDataSigned>> {
	let requires_signed =
		filter.app_id.is_some() || filter.nonce.is_some() || filter.ss58_address.is_some();

	let Some(sig) = &ext.signature else {
		if requires_signed {
			return None;
		}
		return Some(None);
	};

	let mut signed = block_overview::TransactionDataSigned::default();

	if let MultiAddress::Id(id) = &sig.0 {
		signed.ss58_address = Some(std::format!("{}", id))
	};

	signed.nonce = sig.2 .5 .0;
	signed.app_id = sig.2 .8 .0 .0;
	match sig.2 .4 .0 {
		sp_runtime::generic::Era::Immortal => signed.mortality = None,
		sp_runtime::generic::Era::Mortal(x, y) => signed.mortality = Some((x, y)),
	};

	if filter.app_id.is_some_and(|x| x != signed.app_id) {
		return None;
	}

	if filter.nonce.is_some_and(|x| x != signed.nonce) {
		return None;
	}

	if filter.ss58_address.is_some() && filter.ss58_address != signed.ss58_address {
		return None;
	}

	Some(Some(signed))
}

pub(crate) fn filter_extrinsic(
	block_hash: H256,
	tx_index: u32,
	opaq: &OpaqueExtrinsic,
	filter: &block_overview::Filter,
	extension: &block_overview::RPCParamsExtension,
	cache: SharedCache,
	events: Arc<CachedEvents>,
) -> Option<block_overview::TransactionData> {
	if let Some(HashIndex::Index(target_index)) = &filter.tx_id {
		if *target_index != tx_index as u32 {
			return None;
		}
	};

	let cached_tx_hash = {
		let Ok(lock) = cache.read() else {
			return None;
		};

		if let Some(cached) = lock.tx_hash.get(&(block_hash, tx_index)) {
			Some(cached.clone())
		} else {
			None
		}
	};

	if let Some(HashIndex::Hash(target_hash)) = &filter.tx_id {
		if let Some(cached) = &cached_tx_hash {
			if target_hash != cached {
				return None;
			}
		}
	};

	let ext = UncheckedExtrinsic::decode_no_vec_prefix(&mut opaq.0.as_slice());
	let Ok(ext) = ext else {
		let msg = std::format!(
			"Failed to fetch transaction. tx index: {}, block hash: {:?}",
			tx_index,
			block_hash
		);
		log::warn!("{}", msg);
		return None;
	};

	let Some((pallet_id, call_id)) = filter_pallet_call_id(&ext, &filter) else {
		let msg = std::format!(
			"Failed to read pallet and call id. Tx index: {}, block hash: {:?}",
			tx_index,
			block_hash
		);
		log::warn!("{}", msg);
		return None;
	};

	let Some(signed) = filter_signature(&ext, &filter) else {
		return None;
	};

	let tx_hash = if let Some(tx_hash) = cached_tx_hash {
		tx_hash
	} else {
		let tx_hash = Blake2Hasher::hash(&ext.encode());

		let Ok(mut lock) = cache.write() else {
			return None;
		};
		lock.tx_hash.insert((block_hash, tx_index), tx_hash);
		tx_hash
	};

	if let Some(HashIndex::Hash(target_hash)) = &filter.tx_id {
		if tx_hash != *target_hash {
			return None;
		}
	};

	/* 	{
		let Ok(lock) = cache.read() else {
			return None;
		};

		if let Some(value) = lock.calls.get(&(block_hash, tx_index)) {
			tx_extension.encoded_call = Some(std::format!("0x{}", hex::encode(value)));
		} else {
			drop(lock);

			let Ok(mut lock) = cache.write() else {
				return None;
			};
			let encoded = std::format!("0x{}", hex::encode(ext.function.encode()));
			lock.calls.insert((block_hash, tx_index), encoded);
		}
	} */

	let mut tx_events = None;
	if extension.fetch_events {
		let phase = frame_system::Phase::ApplyExtrinsic(tx_index);
		if let Some(cached_event) = events.0.iter().find(|x| x.phase == phase) {
			use block_overview::Event;
			let mut rpc_events: Vec<Event> = Vec::with_capacity(cached_event.events.len());

			for ev in &cached_event.events {
				let event = Event {
					index: ev.index,
					pallet_id: ev.pallet_id,
					event_id: ev.event_id,
					decoded: ev.decoded.clone(),
				};
				rpc_events.push(event);
			}

			tx_events = Some(rpc_events);
		};
	}

	let decoded = None;
	let tx = block_overview::TransactionData {
		tx_hash,
		tx_index,
		pallet_id,
		call_id,
		signed,
		decoded,
		events: tx_events,
	};

	Some(tx)
}
