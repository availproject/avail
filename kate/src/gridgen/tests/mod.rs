use avail_core::{AppExtrinsic, AppId, DataLookup};
use kate_recovery::{data::DataCell, index::AppDataIndex, matrix::Position};
use once_cell::sync::Lazy;
use poly_multiproof::{m1_blst::M1NoPrecomp, traits::AsBytes};
use proptest::{collection, prelude::*, sample::size_range};
use rand::{distributions::Uniform, prelude::Distribution, SeedableRng};
use rand_chacha::ChaChaRng;

use crate::{gridgen::ArkScalar, testnet};

use super::EvaluationGrid;

mod commitments;
mod formatting;
mod reconstruction;

pub static PMP: Lazy<M1NoPrecomp> = Lazy::new(|| testnet::multiproof_params(256, 256));

fn app_extrinsic_strategy() -> impl Strategy<Value = AppExtrinsic> {
	(
		any::<u32>(),
		any_with::<Vec<u8>>(size_range(1..2048).lift()),
	)
		.prop_map(|(app_id, data)| AppExtrinsic {
			app_id: AppId(app_id),
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
		size: lookup.len(),
		index: lookup
			.index()
			.iter()
			.map(|e| (e.app_id.0, e.start))
			.collect(),
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

fn sample_cells(grid: &EvaluationGrid, columns: Option<Vec<usize>>) -> Vec<DataCell> {
	let mut rng = ChaChaRng::from_seed([42u8; 32]);
	let (g_rows, g_cols): (usize, usize) = grid.dims().into();
	let cols = columns.unwrap_or_else(|| (0..g_cols).into_iter().collect());

	cols.iter()
		.flat_map(|x| {
			debug_assert!(*x < g_cols);
			sample_unique(&mut rng, g_rows / 2, g_rows)
				.into_iter()
				.map(move |y| {
					let data = grid
						.evals
						.get((y, *x))
						.and_then(|s: &ArkScalar| s.to_bytes().ok())
						.unwrap();
					// SAFETY: `y` and `x` can be casted safetly becasue `x < g_cols (u16)` and `y
					// < g_rows(u16)`
					let position = Position::from((y as u32, *x as u16));

					DataCell::new(position, data)
				})
		})
		.collect::<Vec<_>>()
}
