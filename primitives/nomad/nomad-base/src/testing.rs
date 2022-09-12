use ethers_signers::LocalWallet;
use nomad_core::test_utils::Updater;
use once_cell::sync::Lazy;

use crate::NomadBase;

pub const TEST_LOCAL_DOMAIN: u32 = 1111;
pub const TEST_UPDATER_PRIVKEY: &str =
	"1111111111111111111111111111111111111111111111111111111111111111";

pub static TEST_UPDATER: Lazy<Updater> = Lazy::new(|| {
	let signer: LocalWallet = TEST_UPDATER_PRIVKEY.parse().unwrap();

	Updater::new(TEST_LOCAL_DOMAIN, signer)
});

pub const FAKE_UPDATER_PRIVKEY: &str =
	"2222222222222222222222222222222222222222222222222222222222222222";

pub static FAKE_UPDATER: Lazy<Updater> = Lazy::new(|| {
	let signer: LocalWallet = FAKE_UPDATER_PRIVKEY.parse().unwrap();

	Updater::new(TEST_LOCAL_DOMAIN, signer)
});

pub static TEST_NOMAD_BASE: Lazy<NomadBase> = Lazy::new(|| {
	NomadBase::new(
		TEST_LOCAL_DOMAIN,
		Default::default(),
		TEST_UPDATER.address(),
	)
});
