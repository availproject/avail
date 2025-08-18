use avail_rust_client::{avail_rust_core::rpc::blob::submit_blob, prelude::*};
use da_commitment::build_da_commitments::build_da_commitments;
use kate::Seed;
use sp_core::keccak_256;
use sp_std::iter::repeat;

pub async fn run() -> Result<(), ClientError> {
	// println!("---------- START ---------- ");
	// let string = "aaaaa";
	// let blob = string.as_bytes().to_vec();
	// let blob_hash = H256::from(keccak_256(&blob));
	// let commitments = build_da_commitments(blob.clone(), 1024, 4096, Seed::default()).unwrap();
	// let commitments_hex = hex::encode(&commitments);

	// println!("string = {}", string);
	// println!("blob = {:?}", blob);
	// println!("blob_hash = {:?}", blob_hash);
	// println!("commitments = {:?}", commitments);
	// println!("commitments_hex = {:?}", commitments_hex);

	// string = aaaaa
	// blob = [97, 97, 97, 97, 97]
	// blob_hash = 0xbd9a50fa60ab6ecedb81dee4d80e715156dd2abde11c2fa27ea69e55a8ef1779
	// commitments = [142, 4, 47, 187, 186, 99, 201, 75, 100, 253, 25, 200, 97, 67, 225, 130, 133, 10, 11, 159, 92, 35, 246, 195, 127, 37, 45, 120, 118, 161, 15, 231, 73, 221, 229, 78, 228, 132, 77, 69, 127, 58, 149, 36, 45, 234, 44, 142]
	// commitments_hex = "0xa8ca453fd5fcac83d7addc8f8708c429ad3e9ca9a2169fda57b2a2641539ec7a87e35f3dfc5e715439256c8a1b420c59"

	// string = bbbbb
	// blob = [98, 98, 98, 98, 98]
	// blob_hash = 0x13e7b7890d9373a5cc635e72cb24d40ed550fd0a6fbfe2981e6fa4fa0bc15fff
	// commitments = [149, 2, 235, 141, 174, 11, 94, 48, 177, 83, 188, 198, 207, 56, 159, 89, 149, 194, 88, 235, 157, 240, 203, 183, 133, 27, 117, 120, 180, 46, 94, 3, 167, 232, 85, 240, 156, 219, 223, 148, 221, 74, 71, 155, 140, 66, 223, 248]
	// commitments_hex = "0x9669dc7b3f8c9f4c572813bd0d97050cba709e13d68b77d2f01f50b41d24ffec26b1179717d5600324a961520ac679db"

	// string = cccccccccc
	// blob = [99, 99, 99, 99, 99, 99, 99, 99, 99, 99]
	// blob_hash = 0x578fcc5386274711e9ab926c82858de3179dd8afffa0e33f7689c37c6d9aa986
	// commitments = [142, 209, 13, 238, 213, 85, 180, 162, 122, 102, 82, 148, 209, 225, 154, 151, 54, 42, 215, 48, 56, 11, 52, 118, 103, 150, 25, 207, 1, 38, 104, 208, 27, 43, 213, 192, 160, 236, 107, 149, 141, 223, 177, 63, 142, 227, 8, 241]
	// commitments_hex = "0xa423bb984e0361893d6146d567c4a690a0d13704a1d5aac25a123970d7935646df63e518a709e0fe69ee7ca8617aebd6"

	println!("---------- START process ---------- ");
	let len = 5 * 1024 * 1024 - 1;
	let mode = 1;

	// let local_endpoint: &str = if mode == 1 {
	// 	"http://127.0.0.1:9944"
	// } else if mode == 2 {
	// 	"http://127.0.0.1:9945"
	// } else if mode == 3 {
	// 	"http://127.0.0.1:9946"
	// } else {
	// 	"http://127.0.0.1:9947"
	// };

	let local_endpoint: &str = if mode == 1 {
		"http://167.99.54.199:8546"
	} else if mode == 2 {
		"https://infinity-devnet2-rpc.avail.tools/rpc"
	} else if mode == 3 {
		"https://infinity-devnet3-rpc.avail.tools/rpc"
	} else {
		"https://infinity-devnet4-rpc.avail.tools/rpc"
	};

	
	let client = Client::new(local_endpoint).await?;
	let signer = if mode == 1 {
		alice()
	} else if mode == 2 {
		bob()
	} else if mode == 3 {
		charlie()
	} else {
		dave()
	};
	let byte = if mode == 1 {
		b'1'
	} else if mode == 2 {
		b'B'
	} else if mode == 3 {
		b'C'
	} else {
		b'D'
	};

	let nonce = client.nonce(&signer.account_id()).await?;
	println!("Nonce: {nonce}");

	let mut blobs: Vec<(Vec<u8>, H256, Vec<u8>)> = Vec::new();
	println!("---------- START Commitments generation ---------- ");
	for i in 0..1 {
		println!("---------- START Commitment generation {i} ---------- ");
		let blob: Vec<u8> = repeat(byte).take(len - i).collect::<Vec<u8>>();
		let blob_hash = H256::from(keccak_256(&blob));
		// TODO Blob take values from the runtime
		let commitments = build_da_commitments(blob.clone(), 1024, 4096, Seed::default());
		println!("blob len = {:?}", blob.len());
		println!("blob_hash = {:?}", blob_hash);
		println!("commitments len = {:?}", commitments.len());
		blobs.push((blob, blob_hash, commitments));
	}
	for (i, (blob, hash, commitments)) in blobs.into_iter().enumerate() {
		println!("---------- START Submission {i} ---------- ");
		let options = Options::new()
			.app_id((i % 5) as u32)
			.nonce(nonce + i as u32);
		let unsigned_tx = client.tx().data_availability().submit_blob_metadata(
			hash,
			blob.len() as u64,
			commitments,
		);

		let tx = unsigned_tx.sign(&signer, options).await.unwrap().0.encode();

		// println!("TX: {:?}", hex::encode(&tx));
		if let Err(e) = submit_blob(&client.rpc_client, tx, blob).await {
			println!("An error has occured: {e}");
		}
		println!("---------- END Submission {i} ---------- ");
	}

	Ok(())
}
