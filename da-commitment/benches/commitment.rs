use da_commitment::build_da_commitments::{
	build_commitments_from_polynomial_grid, build_polynomial_grid,
};
use divan::Bencher;

fn main() {
	divan::main();
}

mod polynominal_grid_32_mib {
	use super::*;

	#[divan::bench(max_time = 5)]
	fn real_data(bencher: Bencher) {
		let blob_m = std::fs::read("./32MiB").unwrap();
		bencher.bench_local(|| {
			build_polynomial_grid(&blob_m, 1024, 4096, Default::default());
		});
	}
	#[divan::bench(max_time = 5)]
	fn fake_data(bencher: Bencher) {
		let blob_m = vec![1u8; 31 * 1024 * 1024];
		bencher.bench(|| {
			build_polynomial_grid(&blob_m, 1024, 4096, Default::default());
		});
	}
}

mod build_commitments_32_mib {
	use super::*;

	#[divan::bench(max_time = 10)]
	fn real_data(bencher: Bencher) {
		let blob_m = std::fs::read("./32MiB").unwrap();
		let grid = build_polynomial_grid(&blob_m, 1024, 4096, Default::default());
		bencher
			.with_inputs(|| grid.clone())
			.bench_local_values(|g| {
				build_commitments_from_polynomial_grid(g);
			});
	}

	#[divan::bench(max_time = 10)]
	fn fake_data(bencher: Bencher) {
		let blob_m = vec![1u8; 31 * 1024 * 1024];
		let grid = build_polynomial_grid(&blob_m, 1024, 4096, Default::default());
		bencher
			.with_inputs(|| grid.clone())
			.bench_local_values(|g| {
				build_commitments_from_polynomial_grid(g);
			});
	}
}
