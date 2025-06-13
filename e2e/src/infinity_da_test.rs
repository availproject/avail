use crate::ClientError;
use avail_base::build_da_commitments::build_da_commitments;
use kate::Seed;
use sp_core::{blake2_256, H256};

pub async fn run() -> Result<(), ClientError> {
	println!("---------- START ---------- ");
	let string = "aaaaa";
	let blob = string.as_bytes().to_vec();
	let blob_hash = H256::from(blake2_256(&blob));
	let commitments = build_da_commitments(blob.clone(), 512, 512, Seed::default()).unwrap();
	let commitments_hex = hex::encode(&commitments);
	println!("string = {}", string);
	println!("blob = {:?}", blob);
	println!("blob_hash = {:?}", blob_hash);
	println!("commitments = {:?}", commitments);
	println!("commitments_hex = {:?}", commitments_hex);
	// string = aaaaa
	// blob = [97, 97, 97, 97, 97]
	// blob_hash = 0xbd9a50fa60ab6ecedb81dee4d80e715156dd2abde11c2fa27ea69e55a8ef1779
	// commitments = [142, 4, 47, 187, 186, 99, 201, 75, 100, 253, 25, 200, 97, 67, 225, 130, 133, 10, 11, 159, 92, 35, 246, 195, 127, 37, 45, 120, 118, 161, 15, 231, 73, 221, 229, 78, 228, 132, 77, 69, 127, 58, 149, 36, 45, 234, 44, 142]
	// commitments_hex = "8e042fbbba63c94b64fd19c86143e182850a0b9f5c23f6c37f252d7876a10fe749dde54ee4844d457f3a95242dea2c8e"
	Ok(())
}
