#![no_main]

use anyhow::{anyhow, Result};
use libfuzzer_sys::fuzz_target;
use std::{panic, cmp::max};

use avail_core::header::HeaderExtension;
use da_control::{pallet::Call as DaControlCall, AppDataFor, CheckAppId};
use da_runtime::{
	AppId, Executive, Header, Runtime, RuntimeCall, RuntimeGenesisConfig, SignedExtra,
	SignedPayload, UncheckedExtrinsic, AVL
};

use codec::Encode;
use frame_support::dispatch::GetDispatchInfo;
use frame_system::{
	CheckEra, CheckGenesis, CheckNonZeroSender, CheckNonce, CheckSpecVersion, CheckTxVersion,
	CheckWeight,
};
use pallet_transaction_payment::ChargeTransactionPayment;
use sp_core::{Pair, H256};
use sp_io::TestExternalities;
use sp_keyring::AccountKeyring::Alice;
use sp_runtime::{
	generic::Era, traits::SignedExtension as _, BuildStorage, Digest, MultiAddress, MultiSignature,
};

fn runtime_ext(total_app_ids: u16) -> TestExternalities {
	let alice = Alice.to_account_id();

	let mut t = RuntimeGenesisConfig::default().system.build_storage().unwrap();

	pallet_babe::GenesisConfig::<Runtime> {
		epoch_config: Some(da_runtime::constants::babe::GENESIS_EPOCH_CONFIG),
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();

	// Alice has all the money
	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![(alice.clone(), 100_000_000 * AVL)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	// Generate valid app IDs
	da_control::GenesisConfig::<Runtime> {
		app_keys: (0..=max(total_app_ids,1))
			.map(|i| (b"".to_vec(), (alice.clone(), i as u32)))
			.collect::<Vec<_>>(),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	t.into()
}

fn submit_data(
	nonce: u32,
	app_id: AppId,
	data: Vec<u8>,
) -> Result<(UncheckedExtrinsic, RuntimeCall)> {
	let data = AppDataFor::<Runtime>::try_from(data).map_err(|_| anyhow!("Data too long"))?;
	let call = RuntimeCall::DataAvailability(DaControlCall::<Runtime>::submit_data { data });

	let extra: SignedExtra = (
		CheckNonZeroSender::<Runtime>::new(),
		CheckSpecVersion::<Runtime>::new(),
		CheckTxVersion::<Runtime>::new(),
		CheckGenesis::<Runtime>::new(),
		CheckEra::<Runtime>::from(Era::Mortal(32, 2)),
		CheckNonce::<Runtime>::from(nonce),
		CheckWeight::<Runtime>::new(),
		ChargeTransactionPayment::<Runtime>::from(0),
		CheckAppId::<Runtime>::from(app_id),
	);
	let payload =
		SignedPayload::new(call, extra).map_err(|_| anyhow!("Failed to create payload"))?;
	let enc_payload = payload.encode();
	let signature = MultiSignature::from(Alice.pair().sign(&enc_payload));
	let (call, extra, _signed) = payload.deconstruct();
	let signer = MultiAddress::from(Alice.to_account_id());

	let tx = UncheckedExtrinsic::new_signed(call.clone(), signer, signature, extra);
	Ok((tx, call))
}

fn header() -> Header {
	let extrinsic_root = H256::default();
	let state_root = H256::default();
	let parent_hash = [69u8; 32].into();
	let digest = Digest::default();

	let ext = HeaderExtension::default();
	Header::new(1, extrinsic_root, state_root, parent_hash, digest, ext)
}

fuzz_target!(|input: (u16, Vec<Vec<u8>>)| {
	let who = Alice.to_account_id();
	let (total_app_ids, tx_data) = input;

	runtime_ext(total_app_ids).execute_with(|| {
		let txs = tx_data
			.into_iter()
			.enumerate()
			.map(|(i, data)| {
				let nonce = i as u32;
				let app_id = AppId(nonce % total_app_ids as u32);
				submit_data(nonce, app_id, data)
			})
			.collect::<Result<Vec<_>>>();

		if let Ok(txs) = txs {
			Executive::initialize_block(&header());
			for (i, (tx, call)) in txs.into_iter().enumerate() {
				let (_address, _signature, extra) = tx.signature.as_ref().unwrap();
				let (_, _, _, _, _, _, check_weight, _, check_app_id) = extra;
				let len = tx.encode().len();
				let info = call.get_dispatch_info();

				if let Err(e) = check_weight.validate(&who, &call, &info, len) {
					println!("Block weight consumed at index {i}: {e:?}");
					break;
				}
				if let Err(e) = check_app_id.validate(&who, &call, &info, len) {
					println!("Block Kate Grid consumed at index {i}: {e:?}");
					break;
				}
				if let Err(e) = Executive::apply_extrinsic(tx) {
					panic!("Failed to apply extrinsic at index {i}: {e:?}");
				}
			}
			Executive::finalize_block();
		}
	});
});
