use da_types::{AppExtrinsic, DataLookup};
use dusk_bytes::Serializable;
use dusk_plonk::prelude::PublicParameters;
use kate_grid::Grid;
use kate_recovery::{index::AppDataIndex, data::DataCell};
use proptest::{collection, prelude::*, sample::size_range};
use rand::{distributions::Uniform, prelude::Distribution, SeedableRng};
use rand_chacha::ChaChaRng;

use crate::testnet;

use super::EvaluationGrid;

mod commitments;
mod formatting;
mod reconstruction;

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

fn sample_unique(rng: &mut impl Rng, n_samples: usize, n: usize) -> Vec<usize> {
	let mut sampled = vec![];
	let u = Uniform::from(0..n);
	while sampled.len() < n_samples || sampled.len() < n {
		let t = u.sample(rng);
		if !sampled.contains(&t) {
			sampled.push(t)
		}
	}
	sampled
}

fn sample_cells(grid: &EvaluationGrid, columns: Option<&[usize]>) -> Vec<DataCell> {
	let mut rng = ChaChaRng::from_seed([42u8; 32]);
	let cols: Vec<usize> = match columns {
		Some(cols) => cols.to_vec(),
		None => (0..grid.dims.width()).into_iter().collect(),
	};
	cols.iter()
		.flat_map(|x| {
			sample_unique(&mut rng, grid.dims.height() / 2, grid.dims.height())
				.into_iter()
				.map(move |y| kate_recovery::data::DataCell {
					position: kate_recovery::matrix::Position {
						row: y as u32,
						col: *x as u16,
					},
					data: grid.evals.get(*x, y).unwrap().to_bytes(),
				})
		})
		.collect::<Vec<_>>()
}

