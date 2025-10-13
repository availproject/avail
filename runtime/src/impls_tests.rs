#[cfg(test)]
mod multiplier_tests {
	use crate::impls::*;
	use crate::*;
	use avail_core::currency::{CENTS, MICRO_AVAIL, MILLICENTS};
	use frame_support::{
		dispatch::{DispatchClass, DispatchInfo, Pays},
		traits::OnFinalize,
		weights::{Weight, WeightToFee},
	};
	use pallet_transaction_payment::{Multiplier, TargetedFeeAdjustment};
	use sp_runtime::{
		assert_eq_error_rate,
		traits::{Convert, One, Zero},
		BuildStorage, FixedPointNumber,
	};

	use crate::{
		constants::{currency::*, system::RuntimeBlockWeights as BlockWeights, time::*},
		Runtime, System, TransactionPayment,
	};

	fn max_normal() -> Weight {
		BlockWeights::get()
			.get(DispatchClass::Normal)
			.max_total
			.unwrap_or_else(|| BlockWeights::get().max_block)
	}

	fn min_multiplier() -> Multiplier {
		MinimumMultiplier::get()
	}

	fn target() -> Weight {
		TargetBlockFullness::get() * max_normal()
	}

	// update based on runtime impl.
	fn runtime_multiplier_update(fm: Multiplier) -> Multiplier {
		TargetedFeeAdjustment::<
			Runtime,
			TargetBlockFullness,
			AdjustmentVariable,
			MinimumMultiplier,
			MaximumMultiplier,
		>::convert(fm)
	}

	// update based on reference impl.
	fn truth_value_update(block_weight: Weight, previous: Multiplier) -> Multiplier {
		let accuracy = Multiplier::accuracy() as f64;
		let previous_float = previous.into_inner() as f64 / accuracy;
		// bump if it is zero.
		let previous_float = previous_float.max(min_multiplier().into_inner() as f64 / accuracy);

		// maximum tx weight
		let m = max_normal().ref_time() as f64;
		// block weight always truncated to max weight
		let block_weight = (block_weight.ref_time() as f64).min(m);
		let v: f64 = AdjustmentVariable::get().to_float();

		// Ideal saturation in terms of weight
		let ss = target().ref_time() as f64;
		// Current saturation in terms of weight
		let s = block_weight;

		let t1 = v * (s / m - ss / m);
		let t2 = v.powi(2) * (s / m - ss / m).powi(2) / 2.0;
		let next_float = previous_float * (1.0 + t1 + t2);
		Multiplier::from_float(next_float)
	}

	fn run_with_system_weight<F>(w: Weight, assertions: F)
	where
		F: Fn(),
	{
		let mut t: sp_io::TestExternalities = frame_system::GenesisConfig::<Runtime>::default()
			.build_storage()
			.unwrap()
			.into();
		t.execute_with(|| {
			System::set_block_consumed_resources(w, 0);
			assertions()
		});
	}

	#[test]
	#[ignore]
	fn truth_value_update_poc_works() {
		let fm = Multiplier::saturating_from_rational(1, 2);
		let test_set = vec![
			(Weight::zero(), fm),
			(Weight::from_parts(100, 0), fm),
			(Weight::from_parts(1000, 0), fm),
			(target(), fm),
			(max_normal() / 2, fm),
			(max_normal(), fm),
		];
		test_set.into_iter().for_each(|(w, fm)| {
			run_with_system_weight(w, || {
				assert_eq_error_rate!(
					truth_value_update(w, fm),
					runtime_multiplier_update(fm),
					// Error is only 1 in 100^18
					Multiplier::from_inner(100),
				);
			})
		})
	}

	#[test]
	fn multiplier_can_grow_from_zero() {
		// if the min is too small, then this will not change, and we are doomed forever.
		// the weight is 1/100th bigger than target.
		run_with_system_weight(
			target().set_ref_time(target().ref_time() * 101 / 100),
			|| {
				let next = runtime_multiplier_update(min_multiplier());
				assert!(
					next > min_multiplier(),
					"{:?} !>= {:?}",
					next,
					min_multiplier()
				);
			},
		)
	}

	#[test]
	fn multiplier_cannot_go_below_limit() {
		// will not go any further below even if block is empty.
		run_with_system_weight(Weight::zero(), || {
			let next = runtime_multiplier_update(min_multiplier());
			assert_eq!(next, min_multiplier());
		})
	}

	// Note: With sensitivity = 0.000001, this test is going to take a long time. We may consider commenting this out
	#[test]
	#[ignore]
	fn time_to_reach_zero() {
		// blocks per 24h in substrate-node: 28,800 (k)
		// s* = 0.1875
		// The bound from the research in an empty chain is:
		// v <~ (p / k(0 - s*))
		// p > v * k * -0.1875
		// to get p == -1 we'd need
		// -1 > 0.00001 * k * -0.1875
		// 1 < 0.00001 * k * 0.1875
		// 10^9 / 1875 < k
		// k > 533_333 ~ 18,5 days.
		run_with_system_weight(Weight::zero(), || {
			// start from 1, the default.
			let mut fm = Multiplier::one();
			let mut iterations: u64 = 0;
			loop {
				let next = runtime_multiplier_update(fm);
				fm = next;
				if fm == min_multiplier() {
					break;
				}
				iterations += 1;
			}
			assert!(iterations > 533_333);
		})
	}

	#[test]
	fn min_change_per_day() {
		run_with_system_weight(max_normal(), || {
			let mut fm = Multiplier::one();
			// We expect a daily multiplier increase of 0.2% if we sustain the congested network on Avail
			for _ in 0..DAYS {
				let next = runtime_multiplier_update(fm);
				fm = next;
			}
			assert!(
				fm > Multiplier::saturating_from_rational(1002, 1000),
				"Invalid fm ={}",
				fm
			);
		})
	}

	#[test]
	#[ignore]
	fn congested_chain_simulation() {
		// `cargo test congested_chain_simulation -- --nocapture` to get some insight.

		// almost full. The entire quota of normal transactions is taken.
		let block_weight = BlockWeights::get()
			.get(DispatchClass::Normal)
			.max_total
			.unwrap() - Weight::from_parts(100, 0);

		// Default substrate weight.
		let tx_weight = frame_support::weights::constants::ExtrinsicBaseWeight::get();

		run_with_system_weight(block_weight, || {
			// initial value configured on module
			let mut fm = Multiplier::one();
			assert_eq!(fm, TransactionPayment::next_fee_multiplier());

			let mut iterations: u64 = 0;
			loop {
				let next = runtime_multiplier_update(fm);
				// if no change, panic. This should never happen in this case.
				if fm == next {
					panic!("The fee should ever increase");
				}
				fm = next;
				iterations += 1;
				let fee =
					<Runtime as pallet_transaction_payment::Config>::WeightToFee::weight_to_fee(
						&tx_weight,
					);
				let adjusted_fee = fm.saturating_mul_acc_int(fee);
				println!(
					"iteration {}, new fm = {:?}. Fee at this point is: {} units / {} millicents, \
					{} cents, {} dollars",
					iterations,
					fm,
					adjusted_fee,
					adjusted_fee / MILLICENTS,
					adjusted_fee / CENTS,
					adjusted_fee / DOLLARS,
				);
			}
		});
	}

	#[test]
	#[ignore]
	fn weight_congested_chain_simulation() {
		// `cargo test weight_congested_chain_simulation -- --nocapture` to get some insight.
		sp_io::TestExternalities::default().execute_with(|| {
			// By default weight multiplier will be 1
			let wm = TransactionPayment::next_fee_multiplier();
			assert_eq!(wm, Multiplier::one());
			let block_weight = BlockWeights::get()
				.get(DispatchClass::Normal)
				.max_total
				.unwrap() - Weight::from_parts(100, 0);

			let tx_len: usize = 512 * 1024; // 512 Kb data
			let da_submission_weight = da_control::weight_helper::submit_data::<Runtime>(tx_len);
			let dispatch_info = DispatchInfo {
				weight: da_submission_weight,
				pays_fee: Pays::Yes,
				..Default::default()
			};
			let tx_fee = TransactionPayment::compute_fee(tx_len as u32, &dispatch_info, 0);
			println!(
				"Iteration: {}, wm: {:?},  Fee: {} units / {} MICRO_AVAIL",
				0,
				wm,
				tx_fee,
				tx_fee / MICRO_AVAIL,
			);
			run_with_system_weight(block_weight, || {
				let mut iterations: u32 = 0;
				let mut day_count: u32 = 0;
				loop {
					iterations += 1;
					TransactionPayment::on_finalize(System::block_number());
					let wm = TransactionPayment::next_fee_multiplier();
					let tx_fee = TransactionPayment::compute_fee(tx_len as u32, &dispatch_info, 0);
					if iterations % EPOCH_DURATION_IN_SLOTS == 0 {
						day_count += 1;
						println!(
							"Iteration: {}, wm: {:?},  Fee: {} units / {} MICRO_AVAIL",
							day_count,
							wm,
							tx_fee,
							tx_fee / MICRO_AVAIL,
						);
					}
					if day_count == 7u32 {
						break;
					}
				}
			});
		});
	}

	#[test]
	fn stateless_weight_mul() {
		let fm = Multiplier::saturating_from_rational(1, 2);
		run_with_system_weight(target() / 4, || {
			let next = runtime_multiplier_update(fm);
			assert_eq_error_rate!(
				next,
				truth_value_update(target() / 4, fm),
				Multiplier::from_inner(100),
			);

			// Light block. Multiplier is reduced a little.
			assert!(next < fm);
		});

		run_with_system_weight(target() / 2, || {
			let next = runtime_multiplier_update(fm);
			assert_eq_error_rate!(
				next,
				truth_value_update(target() / 2, fm),
				Multiplier::from_inner(100),
			);
			// Light block. Multiplier is reduced a little.
			assert!(next < fm);
		});
		run_with_system_weight(target(), || {
			let next = runtime_multiplier_update(fm);
			assert_eq_error_rate!(
				next,
				truth_value_update(target(), fm),
				Multiplier::from_inner(100),
			);
			// ideal. No changes.
			assert_eq!(next, fm)
		});
		run_with_system_weight(target() * 2, || {
			// More than ideal. Fee is increased.
			let next = runtime_multiplier_update(fm);
			assert_eq_error_rate!(
				next,
				truth_value_update(target() * 2, fm),
				Multiplier::from_inner(100),
			);

			// Heavy block. Fee is increased a little.
			assert!(next > fm);
		});
	}

	#[test]
	fn weight_mul_grow_on_big_block() {
		run_with_system_weight(target() * 2, || {
			let mut original = Multiplier::zero();
			let mut next = Multiplier::default();

			(0..500).for_each(|_| {
				next = runtime_multiplier_update(original);
				assert_eq_error_rate!(
					next,
					truth_value_update(target() * 2, original),
					Multiplier::from_inner(100),
				);
				// must always increase
				assert!(next > original, "{:?} !>= {:?}", next, original);
				original = next;
			});
		});
	}

	#[test]
	fn weight_mul_decrease_on_small_block() {
		run_with_system_weight(target() / 2, || {
			let mut original = Multiplier::saturating_from_rational(1, 2);
			let mut next;

			for _ in 0..100 {
				// decreases
				next = runtime_multiplier_update(original);
				assert!(next < original, "{:?} !<= {:?}", next, original);
				original = next;
			}
		})
	}

	#[test]
	fn weight_to_fee_should_not_overflow_on_large_weights() {
		let kb = Weight::from_parts(1024, 0);
		let mb = 1024u64 * kb;
		let max_fm = Multiplier::saturating_from_integer(i128::MAX);

		// check that for all values it can compute, correctly.
		vec![
			Weight::zero(),
			Weight::from_parts(1, 0),
			Weight::from_parts(10, 0),
			Weight::from_parts(1000, 0),
			kb,
			10u64 * kb,
			100u64 * kb,
			mb,
			10u64 * mb,
			Weight::from_parts(2147483647, 0),
			Weight::from_parts(4294967295, 0),
			BlockWeights::get().max_block / 2,
			BlockWeights::get().max_block,
			Weight::MAX / 2,
			Weight::MAX,
		]
		.into_iter()
		.for_each(|i| {
			run_with_system_weight(i, || {
				let next = runtime_multiplier_update(Multiplier::one());
				let truth = truth_value_update(i, Multiplier::one());
				assert_eq_error_rate!(truth, next, Multiplier::from_inner(50_000_000));
			});
		});

		// Some values that are all above the target and will cause an increase.
		let t = target();
		vec![t + Weight::from_parts(100, 0), t * 2]
			.into_iter()
			.for_each(|i| {
				run_with_system_weight(i, || {
					let fm = runtime_multiplier_update(max_fm);
					// won't grow. The convert saturates everything.
					assert_eq!(fm, max_fm);
				})
			});
	}
}

#[cfg(test)]
mod tests {
	use crate::{impls::DealWithFees, AccountId, Balance, BlockNumber, Header};
	use frame_support::{
		derive_impl, parameter_types,
		traits::{
			tokens::{PayFromAccount, UnityAssetBalanceConversion},
			ConstU32, FindAuthor, OnUnbalanced,
		},
		PalletId,
	};
	use frame_system::{
		mocking::MockUncheckedExtrinsic, native::hosted_header_builder::da::HeaderExtensionBuilder,
		test_utils::TestRandomness,
	};
	use sp_runtime::{traits::IdentityLookup, BuildStorage, Perquintill};

	/// An unchecked extrinsic type to be used in tests.
	type Extrinsic = MockUncheckedExtrinsic<Test>;

	/// An implementation of `sp_runtime::traits::Block` to be used in tests.
	type Block = frame_system::mocking::MockDaBlock<Test>;
	const TEST_ACCOUNT: AccountId = AccountId::new([1; 32]);

	frame_support::construct_runtime!(
		pub struct Test {
			System: frame_system,
			Authorship: pallet_authorship,
			Balances: pallet_balances,
			Treasury: pallet_treasury,
		}
	);

	parameter_types! {
		pub const BlockHashCount: BlockNumber = 250;
		pub static ExistentialDeposit: u64 = 1;
	}

	#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
	impl frame_system::Config for Test {
		type AccountId = AccountId;
		type Lookup = IdentityLookup<Self::AccountId>;
		type AccountData = pallet_balances::AccountData<Balance>;
		type BaseCallFilter = frame_support::traits::Everything;
		type Block = Block;
		type BlockHashCount = BlockHashCount;
		type HeaderExtensionBuilder = HeaderExtensionBuilder<Test>;
		type OnSetCode = ();
		type PalletInfo = PalletInfo;
		type Randomness = TestRandomness<Test>;
		type RuntimeCall = RuntimeCall;
		type RuntimeEvent = RuntimeEvent;
		type RuntimeOrigin = RuntimeOrigin;
		type HeaderExtensionDataFilter = ();

		type Header = Header;
		type Extrinsic = Extrinsic;
		type MaxDiffAppIdPerBlock = ConstU32<1_024>;
		type MaxTxPerAppIdPerBlock = ConstU32<8_192>;
	}

	parameter_types! {
		pub const MaxReserves: u32 = 2;
	}

	impl pallet_balances::Config for Test {
		type AccountStore = System;
		type Balance = Balance;
		type DustRemoval = ();
		type ExistentialDeposit = ExistentialDeposit;
		type FreezeIdentifier = [u8; 8];
		type MaxFreezes = ConstU32<2>;
		type MaxLocks = ();
		type MaxReserves = MaxReserves;
		type ReserveIdentifier = [u8; 8];
		type RuntimeEvent = RuntimeEvent;
		type RuntimeHoldReason = ();
		type RuntimeFreezeReason = ();
		type WeightInfo = ();
	}

	parameter_types! {
		pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
		pub const MaxApprovals: u32 = 100;
		pub TreasuryAccount: AccountId = Treasury::account_id();
	}

	impl pallet_treasury::Config for Test {
		type Currency = pallet_balances::Pallet<Test>;
		type ApproveOrigin = frame_system::EnsureRoot<AccountId>;
		type RejectOrigin = frame_system::EnsureRoot<AccountId>;
		type RuntimeEvent = RuntimeEvent;
		type OnSlash = ();
		type ProposalBond = ();
		type ProposalBondMinimum = ();
		type ProposalBondMaximum = ();
		type SpendPeriod = ();
		type Burn = ();
		type BurnDestination = ();
		type PalletId = TreasuryPalletId;
		type SpendFunds = ();
		type MaxApprovals = MaxApprovals;
		type SpendOrigin = frame_support::traits::NeverEnsureOrigin<u128>;
		type WeightInfo = ();

		type AssetKind = ();
		type Beneficiary = Self::AccountId;
		type BeneficiaryLookup = IdentityLookup<Self::AccountId>;
		type Paymaster = PayFromAccount<Balances, TreasuryAccount>;
		type BalanceConverter = UnityAssetBalanceConversion;
		type PayoutPeriod = ConstU32<0>;
		#[cfg(feature = "runtime-benchmarks")]
		type BenchmarkHelper = ();
	}

	pub struct OneAuthor;
	impl FindAuthor<AccountId> for OneAuthor {
		fn find_author<'a, I>(_: I) -> Option<AccountId>
		where
			I: 'a,
		{
			Some(TEST_ACCOUNT)
		}
	}
	impl pallet_authorship::Config for Test {
		type FindAuthor = OneAuthor;
		type EventHandler = ();
	}

	pub fn new_test_ext() -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::<Test>::default()
			.build_storage()
			.unwrap();
		// We use default for brevity, but you can configure as desired if needed.
		pallet_balances::GenesisConfig::<Test>::default()
			.assimilate_storage(&mut t)
			.unwrap();
		t.into()
	}

	#[test]
	fn test_fees_and_tip_split() {
		new_test_ext().execute_with(|| {
			let fee =
				<pallet_balances::Pallet<Test> as frame_support::traits::fungible::Balanced<
					AccountId,
				>>::issue(10);

			let tip =
				<pallet_balances::Pallet<Test> as frame_support::traits::fungible::Balanced<
					AccountId,
				>>::issue(20);

			assert_eq!(Balances::free_balance(Treasury::account_id()), 0);
			assert_eq!(Balances::free_balance(TEST_ACCOUNT), 0);

			DealWithFees::on_unbalanceds(vec![fee, tip].into_iter());

			// Author gets 100% of tip and 20% of fee = 22
			assert_eq!(Balances::free_balance(TEST_ACCOUNT), 22);
			// Treasury gets 80% of fee = 8
			assert_eq!(Balances::free_balance(Treasury::account_id()), 8);
		});
	}

	#[test]
	fn compute_inflation_should_give_sensible_results() {
		assert_eq!(
			pallet_staking_reward_fn::compute_inflation(
				Perquintill::from_percent(50),
				Perquintill::from_percent(50),
				Perquintill::from_percent(5),
			),
			Perquintill::one()
		);
		assert_eq!(
			pallet_staking_reward_fn::compute_inflation(
				Perquintill::from_percent(25),
				Perquintill::from_percent(50),
				Perquintill::from_percent(5),
			),
			Perquintill::from_rational(1u64, 2u64)
		);
		assert_eq!(
			pallet_staking_reward_fn::compute_inflation(
				Perquintill::from_percent(55),
				Perquintill::from_percent(50),
				Perquintill::from_percent(5),
			),
			Perquintill::from_rational(1u64, 2u64)
		);
		assert_eq!(
			pallet_staking_reward_fn::compute_inflation(
				Perquintill::from_percent(60),
				Perquintill::from_percent(50),
				Perquintill::from_percent(5),
			),
			Perquintill::from_rational(1u64, 4u64)
		);
		assert_eq!(
			pallet_staking_reward_fn::compute_inflation(
				Perquintill::from_percent(75),
				Perquintill::from_percent(50),
				Perquintill::from_percent(5),
			),
			Perquintill::from_rational(1u64, 32u64)
		);
	}
}

mod measure_full_block_size {
	use crate::{
		impls_tests::tests::RuntimeGenesisConfig, Block, DataAvailability, Executive, Header,
		Runtime, RuntimeCall, SignedExtra, SignedPayload, System, Timestamp, UncheckedExtrinsic,
	};
	use avail_core::{currency::AVAIL, from_substrate::keccak_256, AppId};
	use codec::Encode;
	use da_control::{
		extensions::native::hosted_commitment_builder::build_da_commitments, BlobTxSummaryRuntime,
		CheckAppId,
	};
	use frame_support::{
		// dispatch::GetDispatchInfo,
		pallet_prelude::{InvalidTransaction, TransactionValidityError},
	};
	use frame_system::{
		CheckEra, CheckGenesis, CheckNonZeroSender, CheckNonce, CheckSpecVersion, CheckTxVersion,
		CheckWeight,
	};
	use pallet_transaction_payment::ChargeTransactionPayment;
	use sp_authority_discovery::AuthorityId;
	use sp_core::{sr25519, Pair, H256};
	use sp_io::TestExternalities;
	use sp_keyring::Ed25519Keyring::Alice;
	use sp_runtime::{generic::Era, AccountId32, BuildStorage, MultiAddress, MultiSignature};

	fn runtime_ext() -> TestExternalities {
		let alice = Alice.to_account_id();
		let mut t = RuntimeGenesisConfig::default()
			.system
			.build_storage()
			.unwrap();

		pallet_babe::GenesisConfig::<Runtime> {
			epoch_config: Some(crate::constants::babe::GENESIS_EPOCH_CONFIG),
			..Default::default()
		}
		.assimilate_storage(&mut t)
		.unwrap();

		pallet_balances::GenesisConfig::<Runtime> {
			balances: vec![(alice.clone(), 100_000_000 * AVAIL)],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		da_control::GenesisConfig::<Runtime> {
			app_keys: (0..=1024)
				.map(|i| (b"".to_vec(), (alice.clone(), i as u32)))
				.collect::<Vec<_>>(),
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}

	fn sample_ownerships() -> Vec<(AccountId32, AuthorityId, String, Vec<u8>)> {
		(0..30)
			.map(|i| {
				let pair = sr25519::Pair::from_seed(&[i as u8; 32]);
				let babe_pair = sr25519::Pair::from_seed(&[i as u8 + 100; 32]);

				let account = AccountId32::from(pair.public());
				let babe_key = AuthorityId::from(babe_pair.public());
				let encoded_peer_id = format!("12D3KooWTestPeerId{:02}", i);
				let signature = pair.sign(b"blob_hash-address-stored").0.to_vec();

				(account, babe_key, encoded_peer_id, signature)
			})
			.collect()
	}

	#[test]
	fn measure_full_block_size() {
		runtime_ext().execute_with(|| {
			let parent_number = 0u32;
			Executive::initialize_block(&Header::new(
				parent_number + 1,
				Default::default(),
				Default::default(),
				Default::default(),
				Default::default(),
				Default::default(),
			));
			Timestamp::set_timestamp(0);

			// We don't care about the real size, we want the maximum number of tx, this will trigger the minimum weight so the maximum number of transaction
			// The total number is bound by weight and length (also since the base_fee is high, it impacts the weight of the tx)
			let tx_size: u64 = 1 * 1024;
			let mut nonce: u32 = 0;
			let mut extrinsics = Vec::new();

			let block_length = System::block_length();
			let blob_runtime_parameters = DataAvailability::blob_runtime_parameters();
			let max_blob_size = blob_runtime_parameters.max_blob_size;
			let blob = vec![b'a'; max_blob_size as usize];
			let cols = block_length.cols.0;
			let rows = block_length.rows.0;
			let seed = kate::Seed::default();
			let blob_hash = H256(keccak_256(&blob));
			let commitment = build_da_commitments(&blob, cols, rows, seed);

			let mut blob_txs_summary: Vec<BlobTxSummaryRuntime> = vec![];
			let ownership = sample_ownerships();
			let missing_validators: Vec<AccountId32> =
				ownership.iter().map(|o| o.0.clone()).collect();

			loop {
				let call = RuntimeCall::DataAvailability(
					da_control::Call::<Runtime>::submit_blob_metadata {
						blob_hash,
						size: tx_size,
						commitment: commitment.clone(),
					},
				);

				// let info = call.get_dispatch_info();
				// println!(
				// 	"predicted: class={:?}, weight(ref_time)={}, proof_size={}",
				// 	info.class,
				// 	info.weight.ref_time(),
				// 	info.weight.proof_size()
				// );

				let extra: SignedExtra = (
					CheckNonZeroSender::<Runtime>::new(),
					CheckSpecVersion::<Runtime>::new(),
					CheckTxVersion::<Runtime>::new(),
					CheckGenesis::<Runtime>::new(),
					CheckEra::<Runtime>::from(Era::Immortal),
					CheckNonce::<Runtime>::from(nonce),
					CheckWeight::<Runtime>::new(),
					ChargeTransactionPayment::<Runtime>::from(0),
					CheckAppId::<Runtime>::from(AppId(nonce % 1024)),
				);
				let payload = SignedPayload::new(call, extra).unwrap();
				let enc_payload = payload.encode();
				let signature = MultiSignature::from(Alice.pair().sign(&enc_payload));
				let (call, extra, _signed) = payload.deconstruct();
				let signer = MultiAddress::from(Alice.to_account_id());
				let tx = UncheckedExtrinsic::new_signed(call.clone(), signer, signature, extra);

				match Executive::apply_extrinsic(tx.clone()) {
					Ok(_) => {
						extrinsics.push(tx);
						nonce += 1;
						blob_txs_summary.push(BlobTxSummaryRuntime {
							hash: H256::zero(),
							finalized_block_hash_checkpoint: H256::zero(),
							tx_index: nonce,
							success: true,
							reason: None,
							missing_validators: missing_validators.clone(),
							ownership: ownership.clone(),
						});
					},
					Err(e) => match e {
						TransactionValidityError::Invalid(
							InvalidTransaction::ExhaustsResources,
						) => {
							println!("Found InvalidTransaction::ExhaustsResources, exiting loop");
							break;
						},
						_ => panic!("Failed to apply extrinsic at index {}: {:?}", nonce - 1, e),
					},
				}
			}

			let post_inherent_call = RuntimeCall::DataAvailability(
				da_control::Call::<Runtime>::submit_blob_txs_summary {
					total_blob_size: tx_size * ((nonce + 1) as u64),
					nb_blobs: (nonce + 1) as u32,
					blob_txs_summary,
				},
			);
			let post_inherent_tx = UncheckedExtrinsic::new_unsigned(post_inherent_call);
			Executive::apply_extrinsic(post_inherent_tx)
				.unwrap()
				.unwrap();

			println!("Extrinsics count = {}", nonce);

			let header = Executive::finalize_block();

			// 12.001 MiB for 6 sec blocktime  and 2 sec for extrinsics
			// ≃ 40 MiB for 20 sec blocktime
			println!("Encoded header size = {} bytes", header.encode().len());

			let block = Block { header, extrinsics };
			// 18.402 MiB for 6 sec blocktime and 2 sec for extrinsics
			// ≃ 62 MiB for 20 sec blocktime
			println!("Encoded block size = {} bytes", block.encode().len()); // 18.402 MiB
		})
	}
}
