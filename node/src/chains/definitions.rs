use da_runtime::{AccountId, Block, RuntimeGenesisConfig, SessionKeys, Signature};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::Properties;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

type AccountPublic = <Signature as Verify>::Signer;

#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig, Extensions>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Common properties for chains
pub fn chain_properties() -> Option<Properties> {
	serde_json::json!({ "tokenDecimals": 18, "tokenSymbol": "AVL" })
		.as_object()
		.cloned()
}

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

#[derive(Clone)]
pub struct AuthorityKeys {
	pub controller: AccountId,
	pub stash: AccountId,
	pub session_keys: SessionKeys,
}

impl AuthorityKeys {
	/// Helper function to generate stash, controller and session key from seed
	pub fn from_seed(seed: &str) -> Self {
		let controller = get_account_id_from_seed::<sr25519::Public>(seed);
		let stash = get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed));
		let session_keys = SessionKeys {
			babe: get_from_seed::<BabeId>(seed),
			grandpa: get_from_seed::<GrandpaId>(seed),
			im_online: get_from_seed::<ImOnlineId>(seed),
			authority_discovery: get_from_seed::<AuthorityDiscoveryId>(seed),
		};

		Self {
			controller,
			stash,
			session_keys,
		}
	}

	pub fn from_accounts(controller: AccountId, grandpa: GrandpaId) -> Self {
		let session_keys = session_keys(controller.clone(), grandpa);
		let stash = controller.clone();

		Self {
			controller,
			stash,
			session_keys,
		}
	}
}

impl From<AuthorityKeys> for (AccountId, AccountId, SessionKeys) {
	fn from(val: AuthorityKeys) -> (AccountId, AccountId, SessionKeys) {
		(val.stash.clone(), val.stash, val.session_keys)
	}
}

fn session_keys(common: AccountId, grandpa: GrandpaId) -> SessionKeys {
	let raw: [u8; 32] = common.into();
	SessionKeys {
		babe: raw.unchecked_into(),
		grandpa,
		im_online: raw.unchecked_into(),
		authority_discovery: raw.unchecked_into(),
	}
}
