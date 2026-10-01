use crate::{impls::HardCutoverCallFilter, *};
use frame_support::{assert_ok, dispatch::GetDispatchInfo, traits::Contains};
use pallet_transaction_payment::OnChargeTransaction;
use sp_runtime::{traits::Dispatchable, BuildStorage};

fn ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Runtime>::default()
		.build_storage()
		.unwrap()
		.into()
}

fn transfer() -> RuntimeCall {
	RuntimeCall::Balances(pallet_balances::Call::transfer_keep_alive {
		dest: sp_runtime::MultiAddress::Id(AccountId::new([2; 32])),
		value: AVAIL,
	})
}

#[test]
fn cutover_blocks_user_calls_and_proxy_bypasses() {
	let blocked = vec![
		transfer(),
		RuntimeCall::System(frame_system::Call::remark { remark: vec![] }),
		RuntimeCall::Staking(pallet_staking::Call::chill {}),
		RuntimeCall::NominationPools(pallet_nomination_pools::Call::join {
			amount: AVAIL,
			pool_id: 1,
		}),
		RuntimeCall::DataAvailability(da_control::Call::create_application_key {
			key: b"blocked".to_vec().try_into().unwrap(),
		}),
		RuntimeCall::Utility(pallet_utility::Call::batch {
			calls: vec![transfer()],
		}),
		RuntimeCall::Proxy(pallet_proxy::Call::proxy {
			real: sp_runtime::MultiAddress::Id(AccountId::new([1; 32])),
			force_proxy_type: None,
			call: Box::new(transfer()),
		}),
	];
	ext().execute_with(|| {
		for call in blocked {
			assert!(!HardCutoverCallFilter::contains(&call), "{call:?}");
			let result = call.dispatch(RuntimeOrigin::signed(AccountId::new([1; 32])));
			assert_eq!(
				result.unwrap_err().error,
				frame_system::Error::<Runtime>::CallFiltered.into()
			);
		}
	});
}

#[test]
fn cutover_keeps_governance_and_block_inherents_available() {
	let calls = vec![
		RuntimeCall::Timestamp(pallet_timestamp::Call::set { now: 1 }),
		RuntimeCall::Vector(pallet_vector::Call::failed_send_message_txs { failed_txs: vec![] }),
		RuntimeCall::Sudo(pallet_sudo::Call::sudo {
			call: Box::new(transfer()),
		}),
		RuntimeCall::Mandate(pallet_mandate::Call::mandate {
			call: Box::new(transfer()),
		}),
		RuntimeCall::TechnicalCommittee(pallet_collective::Call::vote {
			proposal: Hash::zero(),
			index: 0,
			approve: true,
		}),
		RuntimeCall::TreasuryCommittee(pallet_collective::Call::vote {
			proposal: Hash::zero(),
			index: 0,
			approve: true,
		}),
		RuntimeCall::Scheduler(pallet_scheduler::Call::cancel { when: 10, index: 0 }),
		RuntimeCall::System(frame_system::Call::set_code { code: vec![] }),
		RuntimeCall::System(frame_system::Call::authorize_upgrade {
			code_hash: Hash::zero(),
		}),
		RuntimeCall::System(frame_system::Call::apply_authorized_upgrade { code: vec![] }),
	];
	for call in calls {
		assert!(HardCutoverCallFilter::contains(&call), "{call:?}");
	}
	ext().execute_with(|| {
		assert_ok!(
			RuntimeCall::Vector(pallet_vector::Call::failed_send_message_txs {
				failed_txs: vec![],
			})
			.dispatch(RuntimeOrigin::none())
		);
	});
}

#[test]
fn cutover_respects_tx_pause() {
	ext().execute_with(|| {
		let call = RuntimeCall::DataAvailability(da_control::Call::submit_data {
			data: vec![1].try_into().unwrap(),
		});
		let name = (
			b"DataAvailability".to_vec().try_into().unwrap(),
			b"submit_data".to_vec().try_into().unwrap(),
		);
		assert!(<Runtime as frame_system::Config>::BaseCallFilter::contains(
			&call
		));
		assert_ok!(TxPause::pause(RuntimeOrigin::root(), name));
		assert!(!<Runtime as frame_system::Config>::BaseCallFilter::contains(&call));
	});
}

#[test]
fn cutover_charges_neither_fees_nor_tips_and_mints_no_era_reward() {
	ext().execute_with(|| {
		let who = AccountId::new([1; 32]);
		let call = RuntimeCall::DataAvailability(da_control::Call::submit_data {
			data: vec![1].try_into().unwrap(),
		});
		let info = call.get_dispatch_info();
		let issuance = Balances::total_issuance();
		type Charger = <Runtime as pallet_transaction_payment::Config>::OnChargeTransaction;
		let liquidity = Charger::withdraw_fee(&who, &call, &info, AVAIL, AVAIL).unwrap();
		assert_ok!(Charger::correct_and_deposit_fee(
			&who,
			&info,
			&Default::default(),
			AVAIL,
			AVAIL,
			liquidity
		));
		assert_eq!(Balances::free_balance(&who), 0);
		assert_eq!(Balances::total_issuance(), issuance);
		assert_eq!(
			<<Runtime as pallet_staking::Config>::EraPayout as pallet_staking::EraPayout<
				Balance,
			>>::era_payout(AVAIL, 2 * AVAIL, 86_400_000,),
			(0, 0)
		);
	});
}

#[test]
fn cutover_whitelist_is_managed_by_root_and_enforced_at_dispatch() {
	ext().execute_with(|| {
		System::set_block_number(1);
		let who = AccountId::new([1; 32]);
		let submit = || {
			RuntimeCall::DataAvailability(da_control::Call::submit_data {
				data: vec![1].try_into().unwrap(),
			})
		};
		let manage = |allowed| {
			RuntimeCall::DataAvailability(da_control::Call::set_submit_data_whitelist {
				account: who.clone(),
				allowed,
			})
		};
		assert!(HardCutoverCallFilter::contains(&manage(true)));
		assert!(manage(true)
			.dispatch(RuntimeOrigin::signed(who.clone()))
			.is_err());
		assert_eq!(
			submit()
				.dispatch(RuntimeOrigin::signed(who.clone()))
				.unwrap_err()
				.error,
			da_control::Error::<Runtime>::SubmitDataSignerNotWhitelisted.into()
		);
		// The same Root path is available through Sudo or a committee Mandate.
		assert_ok!(manage(true).dispatch(RuntimeOrigin::root()));
		assert_ok!(submit().dispatch(RuntimeOrigin::signed(who.clone())));
		assert_ok!(manage(false).dispatch(RuntimeOrigin::root()));
		assert_eq!(
			submit()
				.dispatch(RuntimeOrigin::signed(who))
				.unwrap_err()
				.error,
			da_control::Error::<Runtime>::SubmitDataSignerNotWhitelisted.into()
		);
	});
}

#[test]
fn cutover_proxy_cannot_use_delegate_whitelist_for_an_unlisted_origin() {
	ext().execute_with(|| {
		System::set_block_number(1);
		let real = AccountId::new([1; 32]);
		let delegate = AccountId::new([2; 32]);
		let proxies: frame_support::BoundedVec<_, <Runtime as pallet_proxy::Config>::MaxProxies> =
			vec![pallet_proxy::ProxyDefinition {
				delegate: delegate.clone(),
				proxy_type: impls::ProxyType::Any,
				delay: 0u32,
			}]
			.try_into()
			.unwrap();
		pallet_proxy::Proxies::<Runtime>::insert(&real, (proxies, 0u128));
		assert_ok!(DataAvailability::set_submit_data_whitelist(
			RuntimeOrigin::root(),
			delegate.clone(),
			true
		));
		let call = || {
			RuntimeCall::Proxy(pallet_proxy::Call::proxy {
				real: sp_runtime::MultiAddress::Id(real.clone()),
				force_proxy_type: None,
				call: Box::new(RuntimeCall::DataAvailability(
					da_control::Call::submit_data {
						data: vec![1].try_into().unwrap(),
					},
				)),
			})
		};
		assert_ok!(call().dispatch(RuntimeOrigin::signed(delegate.clone())));
		System::assert_last_event(RuntimeEvent::Proxy(pallet_proxy::Event::ProxyExecuted {
			result: Err(da_control::Error::<Runtime>::SubmitDataSignerNotWhitelisted.into()),
		}));
		assert_ok!(DataAvailability::set_submit_data_whitelist(
			RuntimeOrigin::root(),
			real.clone(),
			true
		));
		assert_ok!(call().dispatch(RuntimeOrigin::signed(delegate)));
		System::assert_last_event(RuntimeEvent::Proxy(pallet_proxy::Event::ProxyExecuted {
			result: Ok(()),
		}));
	});
}
