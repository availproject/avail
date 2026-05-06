use da_commitment::build_fri_commitments::{build_fri_da_commitment, FriParamsVersion};
use divan::Bencher;

fn main() {
	divan::main();
}

mod fri_commitment_32_mib {
	use super::*;

	#[divan::bench(max_time = 10)]
	fn real_data(bencher: Bencher) {
		let blob_m = std::fs::read("./32MiB").unwrap();
		bencher.bench_local(|| {
			build_fri_da_commitment(&blob_m, FriParamsVersion::V0);
		});
	}

	#[divan::bench(max_time = 10)]
	fn fake_data(bencher: Bencher) {
		let blob_m = vec![1u8; 31 * 1024 * 1024];
		bencher.bench(|| {
			build_fri_da_commitment(&blob_m, FriParamsVersion::V0);
		});
	}
}
