#[cfg(feature = "runtime-benchmarks")]
use frame_benchmarking::{benchmarks, whitelisted_caller};
#[cfg(feature = "runtime-benchmarks")]
use frame_support::{assert_ok, traits::Get as _, BoundedVec};
#[cfg(feature = "runtime-benchmarks")]
use frame_system::RawOrigin;
use hex_literal::hex;
#[cfg(feature = "runtime-benchmarks")]
use nomad_core::NomadState;
use nomad_core::{SignedUpdate, Update};
#[cfg(feature = "runtime-benchmarks")]
use nomad_merkle::Merkle;
use nomad_signature::Signature;
use sp_core::{H160, H256, U256};
#[cfg(feature = "runtime-benchmarks")]
use sp_std::{iter::repeat, vec::Vec};

#[cfg(feature = "runtime-benchmarks")]
use crate::*;

const ID: u32 = 1111;
const UPDATER_ADDRESS: H160 = H160(hex!("19e7e376e7c213b7e7e7e46cc70a5dd086daff2a"));

// Design of benchmark cases:
// - `improper_update`. The worst case is when updater is slashed and the state change to
// `NomdState::Failed`.
// - `dispatch` cost is affected by the length of the message.
// - `update`. The worst case is when `Tree` is full, having `TREE_DEPTH` as max index.

#[cfg(feature = "runtime-benchmarks")]
benchmarks! {
	where_clause { where [u8; 32]: From<<T as frame_system::Config>::AccountId> }

	improper_update {
		let _ = init_tree::<T>(0, 0);

		let origin = RawOrigin::Signed(whitelisted_caller::<T::AccountId>());
		let signed_update = expected_signed_update();
	}: _(origin, signed_update)
	verify {
		assert!(Base::<T>::get().state == NomadState::Failed);
	}

	dispatch {
		let b in 1.. T::MaxMessageBodyBytes::get();

		let _ = init_tree::<T>(0, 0);

		let origin = RawOrigin::Signed(whitelisted_caller::<T::AccountId>());
		let recipient_address = H256::zero();
		let body = random_message::<T>(b);

		let prev_nonce = Nonces::<T>::get(ID);
	}: _(origin, ID, recipient_address, body)
	verify {
		let new_nonce = Nonces::<T>::get(ID);
		assert_eq!( prev_nonce +1, new_nonce);
	}

	update {
		use nomad_merkle::TREE_DEPTH;
		use crate::common_tests_and_benches::expected_longest_tree_signed_update;

		let max_index = TREE_DEPTH as u32;
		let origin = RawOrigin::Signed(whitelisted_caller::<T::AccountId>());
		let new_root = init_tree::<T>(max_index, T::MaxMessageBodyBytes::get());
		let signed_update = expected_longest_tree_signed_update();
		assert_eq!(new_root, signed_update.update.new_root);

	}: _(origin, signed_update, max_index)
	verify {
		assert_eq!( Tree::<T>::get().root(), new_root);
		assert_eq!( Base::<T>::get().committed_root, new_root);
	}

	set_updater {
		let _ = init_tree::<T>(0, 0);

		let new_updater: H160 = H160(hex!("39dD11C243Ac4Ac250980FA3AEa016f73C509f37"));
		let origin = RawOrigin::Root;

	}: _(origin, new_updater)
	verify {
		//  check new updater
		assert_eq!(Base::<T>::get().updater, new_updater);
	}
}

#[cfg(test)]
mod tests {
	use nomad_core::test_utils::Updater;
	use sp_core::H256;

	use super::*;

	const TEST_UPDATER_PRIVKEY: &str =
		"1111111111111111111111111111111111111111111111111111111111111111";

	/// Test case used to generate the constant `SIGNED_UPDATE`.
	#[test]
	fn generate_sign_update() {
		let updater = Updater::new(ID, TEST_UPDATER_PRIVKEY.parse().unwrap());
		assert_eq!(updater.address(), UPDATER_ADDRESS);

		let previous_root = H256::repeat_byte(0);
		let new_root = H256::repeat_byte(1);
		let signed_update = updater.sign_update(previous_root, new_root);

		assert_eq!(signed_update, expected_signed_update());
	}
}

fn expected_signed_update() -> SignedUpdate {
	SignedUpdate {
		update: Update {
			home_domain: ID,
			previous_root: H256([0u8; 32]),
			new_root: H256([1u8; 32]),
		},
		signature: Signature {
			r: U256::from_dec_str(
				"108172166467498881923382587939104653020584515778413535909202974529351125581995",
			)
			.unwrap(),
			s: U256::from_dec_str(
				"24567415212865781861322727855523398821160274507883582359476963266144744361252",
			)
			.unwrap(),
			v: 27,
		},
	}
}

#[cfg(feature = "runtime-benchmarks")]
fn random_message<T: Config>(size: u32) -> BoundedVec<u8, T::MaxMessageBodyBytes> {
	repeat(3u8)
		.take(size as usize)
		.collect::<Vec<_>>()
		.try_into()
		.expect("`size` must be less than `T::MaxMessageBodyBytes`")
}

#[cfg(feature = "runtime-benchmarks")]
fn init_tree<T>(index: u32, message_size: u32) -> H256
where
	T: Config,
	[u8; 32]: From<<T as frame_system::Config>::AccountId>,
{
	let message = random_message::<T>(message_size);
	let recipient_address = H256::repeat_byte(3);
	let origin = RawOrigin::Signed(whitelisted_caller::<T::AccountId>());

	// Initial base:
	Base::<T>::put(nomad_base::NomadBase::new(
		ID,
		H256([0u8; 32]),
		UPDATER_ADDRESS,
	));

	for _ in 0..index {
		assert_ok!(Pallet::<T>::dispatch(
			origin.clone().into(),
			ID,
			recipient_address,
			message.clone()
		));
	}

	Tree::<T>::get().root()
}
