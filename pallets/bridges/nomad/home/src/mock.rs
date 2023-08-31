use frame_support::{parameter_types, traits::ConstU32, weights::Weight};
use frame_system::{self as system, header_builder::da, test_utils::TestRandomness};
use nomad_base::NomadBase;
use sp_core::{H160, H256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	AccountId32, BuildStorage,
};

use crate as home;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockDaBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub struct Test {
		System: frame_system,
		Home: home,
		UpdaterManager: nomad_updater_manager,
	}
);

parameter_types! {
	pub const BlockHashCount: u32 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(Weight::from_parts(1_024, 0));
	pub static ExistentialDeposit: u64 = 0;
}

impl system::Config for Test {
	type AccountData = ();
	type AccountId = AccountId32;
	type BaseCallFilter = frame_support::traits::Everything;
	type Block = Block;
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockWeights = ();
	type DbWeight = ();
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type HeaderExtensionBuilder = da::HeaderExtensionBuilder<Test>;
	type Lookup = IdentityLookup<Self::AccountId>;
	type MaxConsumers = ConstU32<16>;
	type Nonce = u64;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type PalletInfo = PalletInfo;
	type Randomness = TestRandomness<Test>;
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
	pub const MaxMessageBodyBytes: u32 = 2048;
}

impl home::Config for Test {
	type MaxMessageBodyBytes = MaxMessageBodyBytes;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

impl nomad_updater_manager::Config for Test {
	type RuntimeEvent = RuntimeEvent;
}

#[derive(Default)]
pub(crate) struct ExtBuilder {
	updater: H160,
	local_domain: u32,
	committed_root: H256,
}

impl ExtBuilder {
	pub(crate) fn with_base(mut self, base: NomadBase) -> Self {
		self.updater = base.updater;
		self.local_domain = base.local_domain;
		self.committed_root = base.committed_root;
		self
	}

	pub(crate) fn build(self) -> sp_io::TestExternalities {
		let mut t = RuntimeGenesisConfig::default()
			.system
			.build_storage()
			.unwrap();

		home::GenesisConfig::<Test> {
			updater: self.updater,
			local_domain: self.local_domain,
			committed_root: self.committed_root,
			_phantom: Default::default(),
		}
		.assimilate_storage(&mut t)
		.expect("Nomad base storage cannot be assimilated");
		nomad_updater_manager::GenesisConfig::<Test> {
			updater: self.updater,
			_phantom: Default::default(),
		}
		.assimilate_storage(&mut t)
		.expect("Updater manager storage cannot be assimilated");

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

pub(crate) fn events() -> Vec<super::Event<Test>> {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| {
			if let RuntimeEvent::Home(inner) = e {
				Some(inner)
			} else {
				None
			}
		})
		.collect::<Vec<_>>()
}
