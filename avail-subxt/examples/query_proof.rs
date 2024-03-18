use avail_subxt::{avail::Cells, submit::submit_data, tx, AvailClient, Cell, Opts, primitives::GDataProof};
use structopt::StructOpt;
use subxt::backend::rpc::RpcParams;
use subxt_signer::sr25519::dev;

const DATA: &[u8] = b"Hello World";

// Submit data (i.e. "Hello World") and fetch query proof of cell {0,0}.
#[async_std::main]
async fn main() -> anyhow::Result<()> {
	let args = Opts::from_args();
	let signer = dev::alice();
	let client = AvailClient::new(args.ws).await?;

	let block_hash = tx::then_in_finalized_block(submit_data(&client, &signer, DATA, 1).await?)
		.await?
		.block_hash();
	let cells = Cells::try_from(vec![Cell::new(0, 0)]).expect("Valid bounds .qed");

	let mut params = RpcParams::new();
	params.push(cells)?;
	params.push(Some(block_hash))?;

	println!("hash: {block_hash:?}");
	let proof: Vec<GDataProof> = client.rpc().request("kate_queryProof", params).await.unwrap();

	println!("Submitted data in block {block_hash:?} and got proof {proof:?}");
	Ok(())
}
