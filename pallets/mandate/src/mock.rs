use frame_support::{
	pallet_prelude::Weight,
	parameter_types,
	traits::{EitherOf, EnsureOrigin},
};
use frame_system::EnsureRoot;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, ConstU32, IdentityLookup},
	BuildStorage,
};

use crate::{self as pallet_mandate};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockDaBlock<Test>;
type BlockNumber = u32;
type AccountId = u32;

frame_support::construct_runtime!(
	pub struct Test {
		System: frame_system,
		TechnicalCommittee: pallet_collective,
		Mandate: pallet_mandate,
	}
);

parameter_types! {
	pub const BlockHashCount: u32 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(Weight::from_parts(1_024, 0));
	pub static ExistentialDeposit: u64 = 0;
}

impl frame_system::Config for Test {
	type AccountData = ();
	type AccountId = AccountId;
	type BaseCallFilter = frame_support::traits::Everything;
	type Block = Block;
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockWeights = ();
	type DbWeight = ();
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type HeaderExtensionBuilder = frame_system::header_builder::da::HeaderExtensionBuilder<Test>;
	type Lookup = IdentityLookup<Self::AccountId>;
	type MaxConsumers = ConstU32<16>;
	type Nonce = u64;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type PalletInfo = PalletInfo;
	type Randomness = frame_system::test_utils::TestRandomness<Test>;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SS58Prefix = ();
	type SubmittedDataExtractor = ();
	type SystemWeightInfo = ();
	type UncheckedExtrinsic = UncheckedExtrinsic;
	type Version = ();
}

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 10;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
	pub const MaxProposalWeight: Weight = Weight::MAX;
}

impl pallet_collective::Config for Test {
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type MaxMembers = CouncilMaxMembers;
	type MaxProposalWeight = MaxProposalWeight;
	type MaxProposals = CouncilMaxProposals;
	type MotionDuration = CouncilMotionDuration;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SetMembersOrigin = EnsureRoot<Self::AccountId>;
	type WeightInfo = ();
}

impl pallet_mandate::Config for Test {
	type ApprovedOrigin = EitherOf<EnsureRoot<AccountId>, HalfOfTechnicalCommittee>;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

pub struct HalfOfTechnicalCommittee;
impl<OuterOrigin> EnsureOrigin<OuterOrigin> for HalfOfTechnicalCommittee
where
	OuterOrigin: Into<Result<pallet_collective::RawOrigin<AccountId, ()>, OuterOrigin>>
		+ From<pallet_collective::RawOrigin<AccountId, ()>>,
{
	type Success = ();

	fn try_origin(o: OuterOrigin) -> Result<Self::Success, OuterOrigin> {
		o.into().and_then(|o| match o {
			pallet_collective::RawOrigin::Members(n, m) if n * 2u32 >= 1u32 * m => Ok(()),
			r => Err(OuterOrigin::from(r)),
		})
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<OuterOrigin, ()> {
		unimplemented!()
	}
}

/// Create new externalities for `Mandate` module tests.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
