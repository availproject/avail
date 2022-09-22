#![cfg(feature = "runtime-benchmarks")]

use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_core::H256;
use nomad_core::Updater;
use ethers_signers::LocalWallet;
use crate::*;


fn updater() -> Updater {
	let wallet: LocalWallet = "1111111111111111111111111111111111111111111111111111111111111111".parse().unwrap();
	Updater::new( 1111, wallet)
}

benchmarks! {

	improper_update {
		let origin = RawOrigin::Signed(whitelisted_caller::<T::AccountId>());

		let committed_root = Pallet::<T>::base().committed_root;
		let fake_root = H256::repeat_byte(1);
		let update = UPDATER.sign_update(committed_root, fake_root);

	}: _(origin, update)
	verify {
	}
}
