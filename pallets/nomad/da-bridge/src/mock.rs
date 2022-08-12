use da_primitives::{Header, KateCommitment};
use frame_executive::Executive;
use frame_support::{pallet_prelude::Hooks, traits::GenesisBuild};
use frame_system as system;
use hex_literal::hex;
#[cfg(features = "testing")]
use nomad_base::testing::*;
use nomad_base::NomadBase;
use once_cell::sync::Lazy;
use primitive_types::{H160, H256};
use sp_runtime::{
	testing::Block,
	traits::{BlakeTwo256, IdentityLookup},
	AccountId32,
};

use crate as da_bridge;

type SignedExtra = (
	frame_system::CheckEra<Test>,
	frame_system::CheckNonce<Test>,
	frame_system::CheckWeight<Test>,
	pallet_transaction_payment::ChargeTransactionPayment<Test>,
	da_control::CheckAppId<Test>,
);
// type TestXt = sp_runtime::testing::TestXt<Call, SignedExtra>;
type TestHeader = Header<BlockNumber, BlakeTwo256>;
type TestBlock = Block<TestHeader>;
type TestUncheckedExtrinsic = sp_runtime::testing::TestXt<Call, SignedExtra>;

type BlockNumber = u32;

pub(crate) const TEST_REMOTE_DOMAIN: u32 = 2222;
pub(crate) const TEST_SENDER_VEC: [u8; 32] = [2u8; 32];
pub(crate) static TEST_SENDER_BYTES: Lazy<H256> = Lazy::new(|| H256::from(TEST_SENDER_VEC));
pub(crate) static TEST_SENDER_ACCOUNT: Lazy<AccountId32> =
	Lazy::new(|| AccountId32::new(TEST_SENDER_VEC));
static TEST_RECIPIENT: Lazy<H256> = Lazy::new(|| H256::repeat_byte(3));

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = TestBlock,
		NodeBlock = TestBlock,
		UncheckedExtrinsic = TestUncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		UpdaterManager: updater_manager::{Pallet, Call, Storage, Event<T>},
		Home: home::{Pallet, Call, Storage, Event<T>},
		DABridge: da_bridge::{Pallet, Call, Storage, Event<T>},
	}
);

frame_support::parameter_types! {
	pub const BlockHashCount: u32 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
	pub static ExistentialDeposit: u64 = 0;
}

impl system::Config for Test {
	type AccountData = ();
	type AccountId = AccountId32;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockNumber = BlockNumber;
	type BlockWeights = ();
	type Call = Call;
	type DbWeight = ();
	type Event = Event;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;
	type HeaderBuilder = frame_system::header_builder::da::HeaderBuilder<Test>;
	type Index = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type Origin = Origin;
	type PalletInfo = PalletInfo;
	type Randomness = frame_system::tests::TestRandomness<Test>;
	type SS58Prefix = ();
	type SystemWeightInfo = ();
	type Version = ();
}

impl updater_manager::Config for Test {
	type Event = Event;
}

frame_support::parameter_types! {
	pub const MaxMessageBodyBytes: u32 = 5_000;
}

impl home::Config for Test {
	type Event = Event;
	type MaxMessageBodyBytes = MaxMessageBodyBytes;
}

impl da_bridge::Config for Test {
	type Event = Event;
}

pub(crate) struct ExtBuilder {
	updater: H160,
	local_domain: u32,
	committed_root: H256,
}

impl Default for ExtBuilder {
	fn default() -> ExtBuilder {
		ExtBuilder {
			updater: Default::default(),
			local_domain: Default::default(),
			committed_root: Default::default(),
		}
	}
}

impl ExtBuilder {
	pub(crate) fn with_base(mut self, base: NomadBase) -> Self {
		self.updater = base.updater;
		self.local_domain = base.local_domain;
		self.committed_root = base.committed_root;
		self
	}

	pub(crate) fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default()
			.build_storage::<Test>()
			.expect("Frame system builds valid default genesis config");

		home::GenesisConfig::<Test> {
			updater: self.updater,
			local_domain: self.local_domain,
			committed_root: self.committed_root,
			_phantom: Default::default(),
		}
		.assimilate_storage(&mut t)
		.expect("Pallet base storage can be assimilated");
		updater_manager::GenesisConfig::<Test> {
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
			if let Event::DABridge(inner) = e {
				Some(inner)
			} else {
				None
			}
		})
		.collect::<Vec<_>>()
}

pub(crate) fn run_to_block_while_dispatching_random_messages(n: BlockNumber) {
	while System::block_number() < n {
		println!("Finalizing block {}.", System::block_number());
		DABridge::on_finalize(System::block_number());

		#[cfg(feature = "testing")]
		let msg: Vec<u8> = (0..8).map(|_| rand::random::<u8>()).collect();
		Home::dispatch(
			Origin::signed((*TEST_SENDER_ACCOUNT).clone()),
			TEST_REMOTE_DOMAIN,
			*TEST_RECIPIENT,
			msg.clone(),
		)
		.expect("!dispatched message at block interval");
		println!(
			"Dispatched message {:?} for block {}.",
			msg,
			System::block_number()
		);

		Executive::execute_block(Block {});

		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
	}
}
