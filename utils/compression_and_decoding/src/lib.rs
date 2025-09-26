mod from_substrate;

use std::io::{Read, Write};

pub fn zstd_encode(data: &[u8], level: i32) -> Vec<u8> {
	let mut out = Vec::with_capacity(0);
	let mut encoder = zstd::Encoder::new(&mut out, level).unwrap();

	// Improves performance
	let data_size = Some(data.len() as u64);
	encoder.set_pledged_src_size(data_size).unwrap();
	encoder.write_all(data).unwrap();
	encoder.finish().unwrap();

	out
}

pub fn zstd_decode(data: &[u8]) {
	let _ = zstd::decode_all(data).unwrap();
}

pub fn xz_encode(data: &[u8], level: u32) -> Vec<u8> {
	use liblzma::read::XzEncoder;

	let mut encoder = XzEncoder::new_parallel(data, level);
	let mut out = Vec::new();
	encoder.read_to_end(&mut out).unwrap();

	out
}

pub fn xz_decode(data: &[u8]) {
	use liblzma::read::XzDecoder;

	let mut decoder = XzDecoder::new_parallel(data);
	let mut out = Vec::new();
	decoder.read_to_end(&mut out).unwrap();
}

pub fn bzip_2_encode(data: &[u8], level: u32) -> Vec<u8> {
	use bzip2::Compression;
	use bzip2::read::BzEncoder;

	let mut encoder = BzEncoder::new(data, Compression::new(level));
	let mut out = Vec::new();
	encoder.read_to_end(&mut out).unwrap();

	out
}

pub fn bzip_2_decode(data: &[u8]) {
	use bzip2::read::BzDecoder;

	let mut decoder = BzDecoder::new(data);
	let mut out = Vec::new();
	decoder.read_to_end(&mut out).unwrap();
}

pub fn const_hex_encode(data: &[u8]) -> String {
	const_hex::encode_prefixed(data)
}

pub fn const_hex_decode(data: &str) -> Vec<u8> {
	const_hex::decode(data).unwrap()
}

pub fn base_64_encode(data: &[u8]) -> String {
	use base64::Engine;
	base64::engine::general_purpose::STANDARD.encode(data)
}

pub fn base_64_decode(data: &str) -> Vec<u8> {
	use base64::Engine;
	base64::engine::general_purpose::STANDARD
		.decode(data)
		.unwrap()
}

pub fn r_base_64_encode(data: &[u8]) -> String {
	rbase64::encode(data)
}

pub fn r_base_64_decode(data: &str) -> Vec<u8> {
	rbase64::decode(data).unwrap()
}

pub fn sp_core_bytes_encode(data: &[u8]) -> String {
	from_substrate::bytes::to_hex(data, false)
}

pub fn sp_core_bytes_decode(data: &str) -> Vec<u8> {
	from_substrate::bytes::from_hex(data).unwrap()
}
