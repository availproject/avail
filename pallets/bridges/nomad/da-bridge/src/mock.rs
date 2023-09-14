use frame_support::{derive_impl, parameter_types};
use frame_system::{self as system, header_builder::da, test_utils::TestRandomness};
use nomad_base::NomadBase;
use sp_core::{H160, H256};
use sp_runtime::{traits::IdentityLookup, AccountId32, BuildStorage};

use crate::{self as da_bridge};

// type TestXt = sp_runtime::testing::TestXt<Call, SignedExtra>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockDaBlock<Test>;

// TODO: add proper config once frame executive mocking has been demonstrated
// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub struct Test {
		System: frame_system,
		UpdaterManager: nomad_updater_manager,
		Home: nomad_home,
		DABridge: da_bridge,
	}
);

parameter_types! {
	pub const BlockHashCount: u32 = 250;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl system::Config for Test {
	type AccountId = AccountId32;
	type BaseCallFilter = frame_support::traits::Everything;
	type Block = Block;
	type BlockHashCount = BlockHashCount;
	type HeaderExtensionBuilder = da::HeaderExtensionBuilder<Test>;
	type Lookup = IdentityLookup<Self::AccountId>;
	type OnSetCode = ();
	type PalletInfo = PalletInfo;
	type Randomness = TestRandomness<Test>;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SubmittedDataExtractor = ();
	type UncheckedExtrinsic = UncheckedExtrinsic;
}

impl nomad_updater_manager::Config for Test {
	type RuntimeEvent = RuntimeEvent;
}

#[derive_impl(nomad_home::config_preludes::TestDefaultConfig as nomad_home::DefaultConfig)]
impl nomad_home::Config for Test {
	type RuntimeEvent = RuntimeEvent;
}

#[derive_impl(da_bridge::config_preludes::TestDefaultConfig as da_bridge::DefaultConfig)]
impl da_bridge::Config for Test {
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
			.expect("Genesis build should work");

		nomad_home::GenesisConfig::<Test> {
			updater: self.updater,
			local_domain: self.local_domain,
			committed_root: self.committed_root,
			_phantom: Default::default(),
		}
		.assimilate_storage(&mut t)
		.expect("Pallet base storage can be assimilated");
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

pub(crate) fn _events() -> Vec<super::Event<Test>> {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| {
			if let RuntimeEvent::DABridge(inner) = e {
				Some(inner)
			} else {
				None
			}
		})
		.collect::<Vec<_>>()
}

pub(crate) fn fill_block_hash_mapping_up_to_n(n: u8) {
	for i in 0..=n {
		frame_system::BlockHash::<Test>::insert::<u32, <Test as system::Config>::Hash>(
			n as u32,
			H256::repeat_byte(i),
		);
	}
}
