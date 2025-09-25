use da_commitment::build_da_commitments::{
	build_commitments_from_polynomal_grid, build_polynomal_grid,
};
use divan::Bencher;

fn main() {
	divan::main();
}

#[divan::bench(max_time = 5)]
fn bench_build_polynomal_grid_real_data(bencher: Bencher) {
	let blob_m = std::fs::read("./50mb").unwrap();
	bencher.bench(|| {
		build_polynomal_grid(&blob_m, 1024, 4096, Default::default());
	});
}

#[divan::bench(max_time = 5)]
fn bench_build_polynomal_grid_fake_data(bencher: Bencher) {
	let blob_m = vec![1u8; 50157027];
	bencher.bench(|| {
		build_polynomal_grid(&blob_m, 1024, 4096, Default::default());
	});
}

/*
Timer precision: 15 ns
commitment                     fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ bench_build_polynomal_grid  370.3 ms      │ 377.4 ms      │ 374.8 ms      │ 374.8 ms      │ 14      │ 14
*/

#[divan::bench(max_time = 10)]
fn bench_build_commitments_from_polynomal_grid_real_data(bencher: Bencher) {
	let blob_m = std::fs::read("./50mb").unwrap();
	let grid = build_polynomal_grid(&blob_m, 1024, 4096, Default::default());
	bencher
		.with_inputs(|| grid.clone())
		.bench_local_values(|g| {
			build_commitments_from_polynomal_grid(g);
		});
}

#[divan::bench(max_time = 10)]
fn bench_build_commitments_from_polynomal_grid_fake_data(bencher: Bencher) {
	let blob_m = vec![1u8; 50157027];
	let grid = build_polynomal_grid(&blob_m, 1024, 4096, Default::default());
	bencher
		.with_inputs(|| grid.clone())
		.bench_local_values(|g| {
			build_commitments_from_polynomal_grid(g);
		});
}
