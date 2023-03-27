use da_types::{AppExtrinsic, DataLookup};
use dusk_plonk::prelude::PublicParameters;
use kate_recovery::index::AppDataIndex;
use proptest::{collection, prelude::*, sample::size_range};

use crate::testnet;

mod commitments;

pub(crate) fn pp() -> PublicParameters {
	testnet::public_params(da_types::BlockLengthColumns(256))
}

fn app_extrinsic_strategy() -> impl Strategy<Value = AppExtrinsic> {
	(
		any::<u32>(),
		any_with::<Vec<u8>>(size_range(1..2048).lift()),
	)
		.prop_map(|(app_id, data)| AppExtrinsic {
			app_id: app_id.into(),
			data,
		})
}

fn app_extrinsics_strategy() -> impl Strategy<Value = Vec<AppExtrinsic>> {
	collection::vec(app_extrinsic_strategy(), size_range(1..16)).prop_map(|xts| {
		let mut new_xts = xts;
		new_xts.sort_by(|a1, a2| a1.app_id.cmp(&a2.app_id));
		new_xts
	})
}

fn app_data_index_from_lookup(lookup: &DataLookup) -> AppDataIndex {
	AppDataIndex {
		size: lookup.size,
		index: lookup.index.iter().map(|e| (e.app_id.0, e.start)).collect(),
	}
}
