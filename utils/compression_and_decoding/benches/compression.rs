use compression_and_decoding::*;
use divan::Bencher;

fn main() {
	divan::main();
}

mod size_32_mib {
	use super::*;

	mod encode {
		use super::*;

		#[divan::bench(max_time = 3)]
		fn xz_01(bencher: Bencher) {
			let data = std::fs::read("./32MiB").unwrap();
			bencher
				.with_inputs(|| data.as_slice())
				.bench_local_refs(|data| {
					xz_encode(data, 1);
				});
		}

		#[divan::bench(max_time = 3)]
		fn xz_03(bencher: Bencher) {
			let data = std::fs::read("./32MiB").unwrap();
			bencher
				.with_inputs(|| data.as_slice())
				.bench_local_refs(|data| {
					xz_encode(data, 3);
				});
		}

		#[divan::bench(max_time = 3)]
		fn zstd_03(bencher: Bencher) {
			let data = std::fs::read("./32MiB").unwrap();
			bencher
				.with_inputs(|| data.as_slice())
				.bench_local_refs(|data| {
					zstd_encode(data, 3);
				});
		}

		#[divan::bench(max_time = 3)]
		fn bzip2_01(bencher: Bencher) {
			let data = std::fs::read("./32MiB").unwrap();
			bencher
				.with_inputs(|| data.as_slice())
				.bench_local_refs(|data| {
					bzip_2_encode(data, 1);
				});
		}
	}

	mod decode {
		use super::*;

		#[divan::bench(max_time = 3)]
		fn xz_01(bencher: Bencher) {
			let data = std::fs::read("./32MiB").unwrap();
			let encoded = xz_encode(&data, 1);
			bencher
				.with_inputs(|| encoded.as_slice())
				.bench_local_refs(|data| {
					xz_decode(data);
				});
		}

		#[divan::bench(max_time = 3)]
		fn xz_03(bencher: Bencher) {
			let data = std::fs::read("./32MiB").unwrap();
			let encoded = xz_encode(&data, 3);
			bencher
				.with_inputs(|| encoded.as_slice())
				.bench_local_refs(|data| {
					xz_decode(data);
				});
		}

		#[divan::bench(max_time = 3)]
		fn zstd_03(bencher: Bencher) {
			let data = std::fs::read("./32MiB").unwrap();
			let encoded = zstd_encode(&data, 3);
			bencher
				.with_inputs(|| encoded.as_slice())
				.bench_local_refs(|data| {
					zstd_decode(data);
				});
		}

		#[divan::bench(max_time = 3)]
		fn bzip2_01(bencher: Bencher) {
			let data = std::fs::read("./32MiB").unwrap();
			let encoded = bzip_2_encode(&data, 1);
			bencher
				.with_inputs(|| encoded.as_slice())
				.bench_local_refs(|data| {
					bzip_2_decode(data);
				});
		}
	}
}
