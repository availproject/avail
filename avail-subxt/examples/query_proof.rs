use avail_subxt::{
	avail::{Cells, PairSigner},
	build_client,
	rpc::KateRpcClient as _,
	submit_data_finalized as submit, Cell, Opts,
};
use sp_keyring::AccountKeyring;
use structopt::StructOpt;

const DATA: &[u8] = b"Hello World";

// Submit data (i.e. "Hello World") and fetch query proof of cell {0,0}.
#[async_std::main]
async fn main() -> anyhow::Result<()> {
	let args = Opts::from_args();
	let alice = PairSigner::new(AccountKeyring::Alice.pair());
	let (client, rpc) = build_client(args.ws, args.validate_codegen).await?;

	let block = submit(&client, &alice, DATA, 1).await?.block_hash();
	let cells = Cells::try_from(vec![Cell::new(0, 0)]).expect("Valid bounds .qed");
	let proof = hex::encode(rpc.query_proof(cells, block).await?);

	println!("Submitted data in block {block:?} and got proof {proof:?}");
	Ok(())
}
