#![warn(unused_extern_crates)]

fn main() {}

#[cfg(test)]
pub mod tests {
	use std::num::NonZeroU16;
	use std::time::Duration;

	use avail_core::DataProof;
	use avail_core::{AppExtrinsic, AppId, BlockLengthColumns, BlockLengthRows};
	use avail_subxt::api::babe;
	use avail_subxt::{
		api::{
			self,
			runtime_types::{
				avail_core::header::extension::HeaderExtension,
				bounded_collections::bounded_vec::BoundedVec, frame_system::limits::BlockLength,
			},
		},
		avail::{AppUncheckedExtrinsic, PairSigner},
		build_client,
		primitives::AvailExtrinsicParams,
		AvailConfig, Opts,
	};
	use binary_merkle_tree::merkle_proof;
	use codec::Encode;
	use kate::com::Cell;
	use kate::gridgen::{AsBytes, EvaluationGrid};
	use kate_recovery::matrix::Dimensions;
	use kate_recovery::proof::verify;
	use sp_keyring::AccountKeyring;
	use subxt::ext::sp_core::hexdisplay::AsBytesRef;
	use subxt::ext::sp_core::{blake2_256, U256};
	use subxt::ext::sp_runtime::traits::Keccak256;
	use subxt::tx::TxClient;
	use subxt::{
		ext::sp_core::H256,
		rpc::{types::ChainBlockResponse, Rpc, RpcParams},
		OnlineClient,
	};

	async fn establish_a_connection() -> anyhow::Result<OnlineClient<AvailConfig>> {
		let args = Opts {
			ws: String::from("ws://127.0.0.1:9944"),
			validate_codegen: false,
		};
		build_client(args.ws, args.validate_codegen).await
	}

	#[async_std::test]
	pub async fn rpc_query_app_data_test() {
		let client = establish_a_connection().await.unwrap();

		loop {
			let current_block_number = client.blocks().at_latest().await.unwrap().number();
			let slot_query = api::storage().babe().current_slot();
			let slot = client
				.storage()
				.at_latest()
				.await
				.unwrap()
				.fetch(&slot_query)
				.await
				.unwrap()
				.unwrap();

			let randomness_query = api::storage().babe().randomness();
			let randomness = client
				.storage()
				.at_latest()
				.await
				.unwrap()
				.fetch(&randomness_query)
				.await
				.unwrap()
				.unwrap_or([0u8; 32]);

			let slot_id = slot.0.clone();
			let rand = U256::from((randomness, slot).using_encoded(blake2_256));
			let authorities_len = U256::from(1);
			let idx = rand % 10;
			println!("Block number: {current_block_number}, id: {idx}, slot: {slot_id}");

			async_std::task::sleep(Duration::from_secs(5)).await;
		}
	}
}
