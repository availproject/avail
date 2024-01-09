use avail_subxt::{api, avail::PairSigner, build_client, tx_send_in_finalized, Opts};
use sp_keyring::AccountKeyring;
use structopt::StructOpt;

const UPDATER: &str = "0x14dC79964da2C08b23698B3D3cc7Ca32193d9955";
/// This example submits an Avail data extrinsic, then retrieves the block containing the
/// extrinsic and matches the data.
#[async_std::main]
async fn main() -> anyhow::Result<()> {
	let args = Opts::from_args();
	let (client, _) = build_client(args.ws, args.validate_codegen).await?;
	let signer = PairSigner::new(AccountKeyring::Alice.pair());

	// crate a new transaction with new updater
	let set_updater = api::tx().nomad_home().set_updater(UPDATER.parse().unwrap());

	tx_send_in_finalized!(&client, &set_updater, &signer).await?;
	println!("Updating done.");
	Ok(())
}
