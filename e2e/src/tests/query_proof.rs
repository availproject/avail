use super::local_connection;

use avail_core::AppId;
use avail_subxt::{
	avail::{Cells, GDataProof},
	submit::submit_data,
	tx, Cell,
};

use subxt::backend::rpc::RpcParams;
use subxt_signer::sr25519::dev;

const DATA: &[u8] = b"Hello World";

// Submit data (i.e. "Hello World") and fetch query proof of cell {0,0}.
#[async_std::test]
async fn query_proof() -> anyhow::Result<()> {
	let client = local_connection().await?;
	let signer = dev::alice();

	let block_hash =
		tx::then_in_finalized_block(submit_data(&client, &signer, DATA, AppId(1)).await?)
			.await?
			.block_hash();
	let cells = Cells::try_from(vec![Cell::new(0, 0)]).expect("Valid bounds .qed");

	let mut params = RpcParams::new();
	params.push(cells)?;
	params.push(Some(block_hash))?;

	println!("hash: {block_hash:?}");
	let proof: Vec<GDataProof> = client
		.rpc()
		.request("kate_queryProof", params)
		.await
		.unwrap();

	println!("Submitted data in block {block_hash:?} and got proof {proof:?}");
	Ok(())
}
