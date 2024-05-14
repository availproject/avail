use super::{alice_nonce, allow_concurrency, local_connection};

use avail_core::AppId;
use avail_subxt::{
	avail::{Cells, GDataProof},
	submit::submit_data_with_nonce,
	tx, Cell,
};
use subxt::backend::rpc::RpcParams;
use subxt_signer::sr25519::dev;

use std::sync::atomic::Ordering::Relaxed;
use test_log::test;
use tracing::trace;

const DATA: &[u8] = b"Hello World";

// Submit data (i.e. "Hello World") and fetch query proof of cell {0,0}.
#[test(tokio::test)]
async fn query_proof() -> anyhow::Result<()> {
	let _gc = allow_concurrency("query_proof").await;
	let client = local_connection().await?;
	let alice = dev::alice();
	let nonce = alice_nonce().await.fetch_add(1, Relaxed);

	let tx = submit_data_with_nonce(&client, &alice, DATA, AppId(1), nonce).await?;
	let block_hash = tx::then_in_finalized_block(tx).await?.block_hash();
	let cells = Cells::try_from(vec![Cell::new(0, 0)]).expect("Valid bounds .qed");

	let mut params = RpcParams::new();
	params.push(cells)?;
	params.push(Some(block_hash))?;

	trace!("hash: {block_hash:?}");
	let proof: Vec<GDataProof> = client
		.rpc()
		.request("kate_queryProof", params)
		.await
		.unwrap();

	trace!("Submitted data in block {block_hash:?} and got proof {proof:?}");
	Ok(())
}
