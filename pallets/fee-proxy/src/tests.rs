use frame_support::{assert_noop, assert_ok, error::BadOrigin};
use frame_system::RawOrigin;

use crate::{
	mock::{
		new_test_ext, Balances, FeeProxy, RuntimeCall, RuntimeOrigin, System, Test, PROXY_ACCOUNT,
	},
	*,
};

type Error = crate::Error<Test>;

const ALICE: u64 = 1u64;
const BOB: u64 = 2u64;

mod set_proxy_account {

	use super::*;

	#[test]
	fn set_proxy_account() {
		new_test_ext().execute_with(|| {
			let root: RuntimeOrigin = RawOrigin::Root.into();

			// Set proxy account
			assert_ok!(FeeProxy::set_proxy_account(root, Some(PROXY_ACCOUNT)));

			// Checking that the storage has changed
			assert_eq!(FeeProxy::fee_proxy_account(), Some(PROXY_ACCOUNT));

			// Checking Events
			System::assert_last_event(
				Event::ProxyAccountSet {
					account: Some(PROXY_ACCOUNT),
				}
				.into(),
			);
		});
	}

	#[test]
	fn unset_proxy_account() {
		new_test_ext().execute_with(|| {
			let root: RuntimeOrigin = RawOrigin::Root.into();

			// Set proxy account
			assert_ok!(FeeProxy::set_proxy_account(root, None));

			// Checking that the storage has changed
			assert_eq!(FeeProxy::fee_proxy_account(), None);

			// Checking Events
			System::assert_last_event(Event::ProxyAccountSet { account: None }.into());
		});
	}

	#[test]
	fn bad_origin() {
		new_test_ext().execute_with(|| {
			let alice: RuntimeOrigin = RawOrigin::Signed(ALICE).into();

			let err = FeeProxy::set_proxy_account(alice, Some(PROXY_ACCOUNT));
			assert_noop!(err, BadOrigin);
		});
	}
}

mod wrap {

	use super::*;

	#[test]
	fn wrap() {
		new_test_ext().execute_with(|| {
			let root: RuntimeOrigin = RawOrigin::Root.into();
			let alice: RuntimeOrigin = RawOrigin::Signed(ALICE).into();

			assert_ok!(FeeProxy::set_proxy_account(root, Some(PROXY_ACCOUNT)));

			let proxy_initial_balance = Balances::free_balance(PROXY_ACCOUNT);
			let alice_initial_balance = Balances::free_balance(ALICE);

			let call = frame_system::Call::remark {
				remark: "aaa".into(),
			};
			let inner_call = Box::new(RuntimeCall::System(call));
			let outer_call = FeeProxy::wrap(alice, inner_call);

			assert_ok!(outer_call);
			assert!(Balances::free_balance(PROXY_ACCOUNT) < proxy_initial_balance);
			assert!(Balances::free_balance(ALICE) == alice_initial_balance);
		});
	}

	#[test]
	fn proxy_account_not_set() {
		new_test_ext().execute_with(|| {
			let alice: RuntimeOrigin = RawOrigin::Signed(ALICE).into();

			let call = frame_system::Call::remark {
				remark: "aaa".into(),
			};
			let inner_call = Box::new(RuntimeCall::System(call));
			let outer_call = FeeProxy::wrap(alice, inner_call);

			assert_noop!(outer_call, Error::ProxyAccountNotSet);
		});
	}

	#[test]
	fn insufficient_balance_in_proxy_account() {
		new_test_ext().execute_with(|| {
			let alice: RuntimeOrigin = RawOrigin::Signed(ALICE).into();
			let root: RuntimeOrigin = RawOrigin::Root.into();

			assert_ok!(FeeProxy::set_proxy_account(root, Some(BOB)));

			let call = frame_system::Call::remark {
				remark: "aaa".into(),
			};
			let inner_call = Box::new(RuntimeCall::System(call));
			let outer_call = FeeProxy::wrap(alice, inner_call);

			assert_noop!(outer_call, Error::InsufficientBalanceInProxyAccount);
		});
	}
}
