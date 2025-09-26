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
		fn const_hex(bencher: Bencher) {
			let data = std::fs::read("./32MiB").unwrap();
			bencher
				.with_inputs(|| data.as_slice())
				.bench_local_refs(|data| {
					const_hex_encode(data);
				});
		}

		#[divan::bench(max_time = 3)]
		fn base_64(bencher: Bencher) {
			let data = std::fs::read("./32MiB").unwrap();
			bencher
				.with_inputs(|| data.as_slice())
				.bench_local_refs(|data| {
					base_64_encode(data);
				});
		}

		#[divan::bench(max_time = 3)]
		fn r_base_64(bencher: Bencher) {
			let data = std::fs::read("./32MiB").unwrap();
			bencher
				.with_inputs(|| data.as_slice())
				.bench_local_refs(|data| {
					r_base_64_encode(data);
				});
		}

		#[divan::bench(max_time = 3)]
		fn sp_core_bytes(bencher: Bencher) {
			let data = std::fs::read("./32MiB").unwrap();
			bencher
				.with_inputs(|| data.as_slice())
				.bench_local_refs(|data| {
					sp_core_bytes_encode(data);
				});
		}
	}

	mod decode {
		use super::*;

		#[divan::bench(max_time = 3)]
		fn const_hex(bencher: Bencher) {
			let data = std::fs::read("./32MiB").unwrap();
			let data = const_hex_encode(&data);
			bencher
				.with_inputs(|| data.as_str())
				.bench_local_refs(|data| {
					const_hex_decode(data);
				});
		}

		#[divan::bench(max_time = 3)]
		fn base_64(bencher: Bencher) {
			let data = std::fs::read("./32MiB").unwrap();
			let data = base_64_encode(&data);
			bencher
				.with_inputs(|| data.as_str())
				.bench_local_refs(|data| {
					base_64_decode(data);
				});
		}

		#[divan::bench(max_time = 3)]
		fn r_base_64(bencher: Bencher) {
			let data = std::fs::read("./32MiB").unwrap();
			let data = r_base_64_encode(&data);
			bencher
				.with_inputs(|| data.as_str())
				.bench_local_refs(|data| {
					r_base_64_decode(data);
				});
		}

		#[divan::bench(max_time = 3)]
		fn sp_core_bytes(bencher: Bencher) {
			let data = std::fs::read("./32MiB").unwrap();
			let data = sp_core_bytes_encode(&data);
			bencher
				.with_inputs(|| data.as_str())
				.bench_local_refs(|data| {
					sp_core_bytes_decode(data);
				});
		}
	}
}
