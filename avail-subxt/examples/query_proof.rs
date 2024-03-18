use avail_subxt::api;
use avail_subxt::{avail::Cells, tx, AvailClient, BoundedVec, Cell, Opts};
use structopt::StructOpt;
use subxt::backend::rpc::RpcParams;
use subxt_signer::sr25519::dev;

const DATA: &[u8] = b"Hello World";

// Submit data (i.e. "Hello World") and fetch query proof of cell {0,0}.
#[async_std::main]
async fn main() -> anyhow::Result<()> {
	let args = Opts::from_args();
	let client = AvailClient::new(args.ws).await?;

	// Account
	let alice = dev::alice();

	let call = api::tx()
		.data_availability()
		.submit_data(BoundedVec(DATA.into()));
	let hash = tx::send_then_finalized(&client, &call, &alice, 1)
		.await?
		.block_hash();
	let cells = Cells::try_from(vec![Cell::new(0, 0)]).expect("Valid bounds .qed");
	let mut params = RpcParams::new();
	params.push(cells)?;
	params.push(Some(hash))?;

	let _proof = client.rpc().request("kate_queryProof", params).await?;

	// let proof = hex::encode(rpc.kate_query_proof(cells, block_hash).await?);

	println!("Submitted data in block {hash:?} and got proof");
	Ok(())
}
