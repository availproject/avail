// use frame_support::{
//     derive_impl,
//     pallet_prelude::Weight,
//     parameter_types,
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
//     type BaseCallFilter = frame_support::traits::Everything;
//     type Block = Block;
//     type BlockHashCount = BlockHashCount;
//     type HeaderExtensionBuilder = frame_system::header_builder::da::HeaderExtensionBuilder<Test>;
//     type OnSetCode = ();
//     type PalletInfo = PalletInfo;
//     type Randomness = frame_system::test_utils::TestRandomness<Test>;
//     type RuntimeCall = RuntimeCall;
//     type RuntimeEvent = RuntimeEvent;
//     type RuntimeOrigin = RuntimeOrigin;
// }
//
// parameter_types! {
// 	pub const MinSyncCommitteeParticipants: u16=10;
// 	pub const SyncCommitteeSize: u32=512;
// 	pub const FinalizedRootIndex: u32=105;
// 	pub const NextSyncCommitteeIndex: u32= 55;
// 	pub const ExecutionStateRootIndex: u32= 402;
// 	pub const MaxPublicInputsLength: u32 = 9;
// 	pub const MaxVerificationKeyLength: u32 = 4143;
// 	pub const MaxProofLength: u32 = 1133;
// }
//
//
// frame_support::construct_runtime!(
// 	pub struct Test {
//         System: frame_system,
//         bridge: pallet_succinct
// 	}
// );
//
//
// impl pallet_succinct::Config for Test {
//     type RuntimeCall = RuntimeCall;
//     type RuntimeEvent = RuntimeEvent;
//     type WeightInfo = ();
//     type TimeProvider = ();
//     type MaxPublicInputsLength = MaxPublicInputsLength;
//     type MaxProofLength = MaxProofLength;
//     type MaxVerificationKeyLength = MaxVerificationKeyLength;
//     type MinSyncCommitteeParticipants = MinSyncCommitteeParticipants;
//     type SyncCommitteeSize = SyncCommitteeSize;
//     type FinalizedRootIndex = FinalizedRootIndex;
//     type NextSyncCommitteeIndex = NextSyncCommitteeIndex;
//     type ExecutionStateRootIndex = ExecutionStateRootIndex;
// }
//
//
// /// Create new externalities for `Succinct` module tests.
// pub fn new_test_ext() -> sp_io::TestExternalities {
//     let t = frame_system::GenesisConfig::<Test>::default()
//         .build_storage()
//         .unwrap();
//     let mut ext = sp_io::TestExternalities::new(t);
//     ext.execute_with(|| System::set_block_number(1));
//     ext
// }
