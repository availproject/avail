use anyhow::Result;
use avail_subxt::{
	api::{self, runtime_types::pallet_staking::pallet::pallet::Call as StakingCall},
	build_client, Call, Opts,
};
use sp_keyring::AccountKeyring;
use structopt::StructOpt;
use subxt::tx::PairSigner;

#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = build_client(args.ws).await?;
	let signer = PairSigner::new(AccountKeyring::Alice.pair());

	// Send a proposal to the council.
	let increase_validator_call =
		Call::Staking(StakingCall::increase_validator_count { additional: 1 });
	let proposal = api::tx().council().propose(5, increase_validator_call, 36);

	let tx = client
		.tx()
		.sign_and_submit_then_watch(&proposal, &signer, Default::default())
		.await?
		.wait_for_finalized_success()
		.await?;

	println!("Proposal has been inclided into {}", tx.block_hash());
	Ok(())
}
