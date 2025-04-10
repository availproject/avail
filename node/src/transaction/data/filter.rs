use crate::transaction::read_pallet_call_index;
use avail_core::OpaqueExtrinsic;
use codec::Encode;
use da_runtime::UncheckedExtrinsic;
use sc_telemetry::log;
use sp_core::H256;
use sp_core::{Blake2Hasher, Hasher};
use sp_runtime::MultiAddress;
use transaction_rpc::data_types::{
	self, DecodedEvents, EncodedEvents, Filter, HashIndex, TransactionData,
	TransactionDataExtension, TransactionDataSigned,
};

use super::worker::{RPCEvent, SharedCache};

pub(crate) fn filter_pallet_call_id(ext: &UncheckedExtrinsic, filter: &Filter) -> Option<(u8, u8)> {
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
	filter: &Filter,
) -> Option<Option<TransactionDataSigned>> {
	let requires_signed =
		filter.app_id.is_some() || filter.nonce.is_some() || filter.ss58_address.is_some();

	let Some(sig) = &ext.signature else {
		if requires_signed {
			return None;
		}
		return Some(None);
	};

	let mut signed = TransactionDataSigned::default();

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
	filter: &Filter,
	extension: &data_types::RPCParamsExtension,
	cache: SharedCache,
) -> Option<TransactionData> {
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

	let mut tx_extension = TransactionDataExtension::default();
	if extension.fetch_call {
		if extension.enable_call_encoding {
			let Ok(lock) = cache.read() else {
				return None;
			};

			if let Some(value) = lock.encoded_call.get(&(block_hash, tx_index)) {
				tx_extension.encoded_call = Some(std::format!("0x{}", hex::encode(value)));
			} else {
				drop(lock);

				let Ok(mut lock) = cache.write() else {
					return None;
				};
				let encoded = ext.function.encode();
				lock.encoded_call
					.insert((block_hash, tx_index), encoded.clone());
				tx_extension.encoded_call = Some(std::format!("0x{}", hex::encode(encoded)));
			}
		}
	}

	let tx = TransactionData {
		tx_hash,
		tx_index,
		pallet_id,
		call_id,
		signed,
		extension: tx_extension,
	};

	Some(tx)
}

pub(crate) fn filter_events(
	tx_index: u32,
	enable_encoding: bool,
	enable_decoding: bool,
	events: &Vec<RPCEvent>,
) -> (Option<EncodedEvents>, Option<DecodedEvents>) {
	let Some(events) = events.iter().find(|x| x.tx_index == tx_index) else {
		return (None, None);
	};

	let mut encoded_result = None;
	let mut decoded_result = None;

	if enable_encoding {
		encoded_result = Some(events.encoded.clone())
	}

	if enable_decoding {
		decoded_result = Some(events.decoded.clone())
	}

	if !enable_encoding && !enable_encoding {
		let mut encoded = events.encoded.clone();
		encoded.iter_mut().for_each(|x| x.data = "".into());
		encoded_result = Some(encoded);
	}

	(encoded_result, decoded_result)
}
