// use frame_benchmarking::baseline::mock::{RuntimeCall, Test};
// use frame_support::{
// 	derive_impl,
// 	pallet_prelude::Weight,
// 	parameter_types,
// };
// use frame_system::EnsureRoot;
// use sp_runtime::BuildStorage;
//
// type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
// type Block = frame_system::mocking::MockDaBlock<Test>;
// type BlockNumber = u32;
// type AccountId = u64;
//
// use crate as pallet_succinct;
//
//
// parameter_types! {
// 	pub const BlockHashCount: u32 = 250;
// 	pub static ExistentialDeposit: u64 = 0;
// }
//
// #[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
// impl frame_system::Config for Test {
// 	type BaseCallFilter = frame_support::traits::Everything;
// 	type Block = Block;
// 	type BlockHashCount = BlockHashCount;
// 	type HeaderExtensionBuilder = frame_system::header_builder::da::HeaderExtensionBuilder<Test>;
// 	type OnSetCode = ();
// 	type PalletInfo = PalletInfo;
// 	type Randomness = frame_system::test_utils::TestRandomness<Test>;
// 	type RuntimeCall = RuntimeCall;
// 	type RuntimeEvent = RuntimeEvent;
// 	type RuntimeOrigin = RuntimeOrigin;
// 	type SubmittedDataExtractor = ();
// 	type UncheckedExtrinsic = UncheckedExtrinsic;
// }
//
// parameter_types! {
// 	pub const CouncilMotionDuration: BlockNumber = 10;
// 	pub const CouncilMaxProposals: u32 = 100;
// 	pub const CouncilMaxMembers: u32 = 100;
// 	pub const MaxProposalWeight: Weight = Weight::MAX;
// }
//
//
// frame_support::construct_runtime!(
// 	pub struct Test {
//
// 	}
// );
//
//
// impl pallet_succinct::Config for Test {
// 	type RuntimeCall = RuntimeCall;
// 	type RuntimeEvent = RuntimeEvent;
// 	type WeightInfo = ();
// }
//
//
// /// Create new externalities for `Succinct` module tests.
// pub fn new_test_ext() -> sp_io::TestExternalities {
// 	let t = frame_system::GenesisConfig::<Test>::default()
// 		.build_storage()
// 		.unwrap();
// 	let mut ext = sp_io::TestExternalities::new(t);
// 	ext.execute_with(|| System::set_block_number(1));
// 	ext
// }
