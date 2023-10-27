use anyhow::{Result, Context};
use avail_core::DataProof;
use avail_subxt::{
	api::{
		self,
		runtime_types::{
			bounded_collections::bounded_vec::BoundedVec, da_control::pallet::Call as DaCall,
		},
	},
	avail::AppUncheckedExtrinsic,
	build_client,
	primitives::AvailExtrinsicParams,
	rpc::RpcParams,
	Call, Opts,
};
use sp_keyring::AccountKeyring;
use structopt::StructOpt;
use subxt::tx::PairSigner;
use binary_merkle_tree::{merkle_proof};
use avail_core::Keccak256;
/// This example submits an Avail data extrinsic, then retrieves the block containing the
/// extrinsic and matches the data.
#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let client = build_client(args.ws, args.validate_codegen).await?;

	let signer = PairSigner::new(AccountKeyring::Alice.pair());
	let mut example_data = [0u8; 12_500];
	example_data[..7].copy_from_slice(b"example");
	let example_data = example_data.to_vec();
	let data_transfer = api::tx()
		.data_availability()
		.submit_data(BoundedVec(example_data.clone()));
	let extrinsic_params = AvailExtrinsicParams::new_with_app_id(1.into());
	println!("Sending example data...");
	let h = client
		.tx()
		.sign_and_submit_then_watch(&data_transfer, &signer, extrinsic_params)
		.await?
		.wait_for_finalized_success()
		.await?;
	let mut index :usize= 0;
	let submitted_block = client.rpc().block(Some(h.block_hash())).await?.unwrap();
	let matched_xt_index = submitted_block
    .block
    .extrinsics
    .into_iter()
    .enumerate()
    .filter_map(|(index, chain_block_ext)| {
        AppUncheckedExtrinsic::try_from(chain_block_ext)
            .map(|ext| (index, ext.function))
            .ok()
    })
    .find_map(|(index, call)| match call {
        Call::DataAvailability(da_call) => match da_call {
            DaCall::submit_data { data } if data.0 == example_data => Some(index),
            _ => None,
        },
        _ => None,
    });
	let mut params = RpcParams::new();
	
	if let Some(i) = matched_xt_index {
		index = i-1;
		params.push(i.clone())?;
		params.push(h.block_hash().clone())?;
	}
	let res = client.rpc().request::<DataProof>("kate_queryDataProof", params)
	.await.context("DataProof failed")?;
	let data:Vec<Vec<u8>> = vec![example_data.clone()];
	// assert!(res.is_ok());
	println!("DataProof: {:?}", res);
	let proof = merkle_proof::<Keccak256, _, _>(data, index);
	// println!("proof: {:?}", proof);
	assert_eq!(res.root,proof.root);
	Ok(())

}
