use avail_subxt::{avail::PairSigner, build_client, submit_data_in_block, Opts};
use sp_keyring::AccountKeyring;
use structopt::StructOpt;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
	let args = Opts::from_args();
	let alice = PairSigner::new(AccountKeyring::Alice.pair());
	let client = build_client(args.ws, args.validate_codegen).await?;

	let in_block = submit_data_in_block(&client, &alice, *b"Hello World", 1).await?;

	println!("Submitted data in block {:?}", in_block.block_hash());
	Ok(())
}
