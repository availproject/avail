use crate::{
	constants, mmr, version::VERSION, AccountId, AuthorityDiscovery, Babe, Block, BlockNumber,
	EpochDuration, Executive, Grandpa, Historical, Index, InherentDataExt, Mmr, NominationPools,
	OpaqueMetadata, Runtime, RuntimeCall, RuntimeGenesisConfig, SessionKeys, System,
	TransactionPayment, LOG_TARGET,
};
use avail_base::ProvidePostInherent;
use avail_core::{
	currency::Balance,
	data_proof::{DataProof, ProofResponse, SubTrie},
	header::HeaderExtension,
	AppId, OpaqueExtrinsic,
};
use da_control::kate::{Error as RTKateError, GDataProof, GRow, RTKate};
use frame_system::{
	data_root::build_tx_data_from_opaque, header_builder::da::HeaderExtensionBuilder,
	limits::BlockLength, HeaderExtensionBuilder as _,
};

use frame_support::{
	genesis_builder_helper::{build_config, create_default_config},
	traits::KeyOwnerProofSystem,
	weights::Weight,
};
use pallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};
use sp_api::{decl_runtime_apis, impl_runtime_apis};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::KeyTypeId, H256, U256};
use sp_inherents::{CheckInherentsResult, InherentData};
use sp_runtime::{
	traits::{Block as BlockT, Extrinsic as ExtrinsicT, NumberFor},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult,
};
use sp_std::{borrow::Cow, vec::Vec};
use sp_version::RuntimeVersion;

type RTExtractor = <Runtime as frame_system::Config>::TxDataExtractor;
type RTExtrinsic = <Runtime as frame_system::Config>::Extrinsic;

decl_runtime_apis! {
	#[api_version(2)]
	pub trait DataAvailApi {
		fn block_length() -> BlockLength;
	}

	pub trait ExtensionBuilder {
		#[api_version(4)]
		fn build_extension(
			extrinsics: Vec<OpaqueExtrinsic>,
			data_root: H256,
			block_length: BlockLength,
			block_number: u32,
		) -> HeaderExtension;

		fn build_data_root(block: u32, extrinsics: Vec<OpaqueExtrinsic>) -> H256;
	}

	pub trait VectorApi {
		fn sync_committee_poseidons(slot: u64) -> U256;
		fn head() -> u64;
		fn headers(slot: u64) -> H256;
	}

	pub trait KateApi {
		fn data_proof(block_number: u32, extrinsics: Vec<OpaqueExtrinsic>, tx_idx: u32) -> Option<ProofResponse>;
		fn rows(block_number: u32, extrinsics: Vec<OpaqueExtrinsic>, block_len: BlockLength, rows: Vec<u32>) -> Result<Vec<GRow>, RTKateError >;
		fn app_data(block_number: u32, extrinsics: Vec<OpaqueExtrinsic>, block_len: BlockLength, id: AppId) -> Result<Vec<Option<GRow>>, RTKateError>;
		fn proof(block_number: u32, extrinsics: Vec<OpaqueExtrinsic>, block_len: BlockLength, cells: Vec<(u32,u32)> ) -> Result<Vec<GDataProof>, RTKateError>;
	}
}

pub(crate) const fn runtime_api_versions() -> Cow<'static, [([u8; 8], u32)]> {
	RUNTIME_API_VERSIONS
}

pub static NATIVE_VERSION: &RuntimeVersion = &VERSION;

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}

		fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
			Runtime::metadata_at_version(version)
		}

		fn metadata_versions() -> sp_std::vec::Vec<u32> {
			Runtime::metadata_versions()
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime
	{
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(block: Block, data: InherentData) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn current_set_id() -> sp_consensus_grandpa::SetId {
			Grandpa::current_set_id()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: sp_consensus_grandpa::EquivocationProof<
				<Block as BlockT>::Hash,
				NumberFor<Block>,
			>,
			key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Grandpa::submit_unsigned_equivocation_report(
				equivocation_proof,
				key_owner_proof,
			)
		}

		fn generate_key_ownership_proof(
			_set_id: sp_consensus_grandpa::SetId,
			authority_id: GrandpaId,
		) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
			use codec::Encode;

			Historical::prove((sp_consensus_grandpa::KEY_TYPE, authority_id))
				.map(|p| p.encode())
				.map(sp_consensus_grandpa::OpaqueKeyOwnershipProof::new)
		}
	}

	impl sp_consensus_babe::BabeApi<Block> for Runtime {
		fn configuration() -> sp_consensus_babe::BabeConfiguration {
			let epoch_config = Babe::epoch_config().unwrap_or(constants::babe::GENESIS_EPOCH_CONFIG);
			sp_consensus_babe::BabeConfiguration {
				slot_duration: Babe::slot_duration(),
				epoch_length: EpochDuration::get() as u64,
				c: epoch_config.c,
				authorities: Babe::authorities().to_vec(),
				randomness: Babe::randomness(),
				allowed_slots: epoch_config.allowed_slots,
			}
		}

		fn current_epoch_start() -> sp_consensus_babe::Slot {
			Babe::current_epoch_start()
		}

		fn current_epoch() -> sp_consensus_babe::Epoch {
			Babe::current_epoch()
		}

		fn next_epoch() -> sp_consensus_babe::Epoch {
			Babe::next_epoch()
		}

		fn generate_key_ownership_proof(
			_slot: sp_consensus_babe::Slot,
			authority_id: sp_consensus_babe::AuthorityId,
		) -> Option<sp_consensus_babe::OpaqueKeyOwnershipProof> {
			use codec::Encode;

			Historical::prove((sp_consensus_babe::KEY_TYPE, authority_id))
				.map(|p| p.encode())
				.map(sp_consensus_babe::OpaqueKeyOwnershipProof::new)
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: sp_consensus_babe::EquivocationProof<<Block as BlockT>::Header>,
			key_owner_proof: sp_consensus_babe::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Babe::submit_unsigned_equivocation_report(
				equivocation_proof,
				key_owner_proof,
			)
		}
	}

	impl sp_authority_discovery::AuthorityDiscoveryApi<Block> for Runtime {
		fn authorities() -> Vec<AuthorityDiscoveryId> {
			AuthorityDiscovery::authorities()
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}


	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
		Block,
		Balance,
	> for Runtime {
		fn query_info(uxt: <Block as BlockT>::Extrinsic, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}

		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}

		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
		for Runtime
	{
		fn query_call_info(call: RuntimeCall, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_call_info(call, len)
		}
		fn query_call_fee_details(call: RuntimeCall, len: u32) -> FeeDetails<Balance> {
			TransactionPayment::query_call_fee_details(call, len)
		}

		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}

		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl pallet_mmr::primitives::MmrApi<
		Block,
		mmr::Hash,
		BlockNumber,
	> for Runtime {
		fn mmr_root() -> Result<mmr::Hash, mmr::Error> {
			Ok(Mmr::mmr_root())
		}

		fn mmr_leaf_count() -> Result<mmr::LeafIndex, mmr::Error> {
			Ok(Mmr::mmr_leaves())
		}

		fn generate_proof(
			block_numbers: Vec<BlockNumber>,
			best_known_block_number: Option<BlockNumber>,
		) -> Result<(Vec<mmr::EncodableOpaqueLeaf>, mmr::Proof<mmr::Hash>), mmr::Error> {
			Mmr::generate_proof(block_numbers, best_known_block_number).map(
				|(leaves, proof)| {
					(
						leaves
							.into_iter()
							.map(|leaf| mmr::EncodableOpaqueLeaf::from_leaf(&leaf))
							.collect(),
						proof,
					)
				},
			)
		}

		fn verify_proof(leaves: Vec<mmr::EncodableOpaqueLeaf>, proof: mmr::Proof<mmr::Hash>)
			-> Result<(), mmr::Error>
		{
			let leaves = leaves.into_iter().map(|leaf|
				leaf.into_opaque_leaf()
				.try_decode()
				.ok_or(mmr::Error::Verify)).collect::<Result<Vec<mmr::Leaf>, mmr::Error>>()?;
			Mmr::verify_leaves(leaves, proof)
		}

		fn verify_proof_stateless(
			root: mmr::Hash,
			leaves: Vec<mmr::EncodableOpaqueLeaf>,
			proof: mmr::Proof<mmr::Hash>
		) -> Result<(), mmr::Error> {
			let nodes = leaves.into_iter().map(|leaf|mmr::DataOrHash::Data(leaf.into_opaque_leaf())).collect();
			pallet_mmr::verify_leaves_proof::<mmr::Hashing, _>(root, nodes, proof)
		}
	}

	impl pallet_nomination_pools_runtime_api::NominationPoolsApi<
		Block,
		AccountId,
		Balance,
	> for Runtime {
		fn pending_rewards(member: AccountId) -> Balance {
			NominationPools::api_pending_rewards(member).unwrap_or_default()
		}

		fn points_to_balance(pool_id: pallet_nomination_pools::PoolId, points: Balance) -> Balance {
			NominationPools::api_points_to_balance(pool_id, points)
		}

		fn balance_to_points(pool_id: pallet_nomination_pools::PoolId, new_funds: Balance) -> Balance {
			NominationPools::api_balance_to_points(pool_id, new_funds)
		}
	}

	impl crate::apis::DataAvailApi<Block> for Runtime {
		fn block_length() -> frame_system::limits::BlockLength {
			frame_system::Pallet::<Runtime>::block_length()
		}
	}

	#[api_version(4)]
	impl crate::apis::ExtensionBuilder<Block> for Runtime {
		fn build_data_root(block: u32, extrinsics: Vec<OpaqueExtrinsic>) -> H256  {
			let tx_data = build_tx_data_from_opaque::<RTExtractor, RTExtrinsic, _, _>(block, extrinsics);
			tx_data.root()
		}

		fn build_extension(
			extrinsics: Vec<OpaqueExtrinsic>,
			data_root: H256,
			block_length: BlockLength,
			block_number: u32,
		) -> HeaderExtension {
			let tx_data = build_tx_data_from_opaque::<RTExtractor, RTExtrinsic, _, _>(block_number, extrinsics);
			let submitted = tx_data.to_app_extrinsics();
			HeaderExtensionBuilder::<Runtime>::build( submitted, data_root, block_length, block_number)
		}
	}

	impl crate::apis::VectorApi<Block> for Runtime {
		fn sync_committee_poseidons(slot: u64) -> U256 {
			pallet_vector::Pallet::<Runtime>::sync_committee_poseidons(slot)
		}

		fn head() -> u64 {
			pallet_vector::Pallet::<Runtime>::head()
		}

		fn headers(slot: u64) -> H256 {
			pallet_vector::Pallet::<Runtime>::headers(slot)
		}
	}

	impl crate::apis::KateApi<Block> for Runtime {
		fn data_proof(block_number: u32, extrinsics: Vec<OpaqueExtrinsic>, tx_idx: u32) -> Option<ProofResponse> {

			let tx_data = build_tx_data_from_opaque::<RTExtractor, RTExtrinsic, _, _>(block_number, extrinsics);
			let (leaf_idx, sub_trie) = tx_data.leaf_idx(tx_idx)?;
			log::trace!(
				target: LOG_TARGET,
				"KateApi::data_proof: tx_idx={tx_idx:?} leaf_idx={leaf_idx:?}, sub_trie:{sub_trie:?}");

			let (sub_proof, message) = match sub_trie {
				SubTrie::DataSubmit => {
					let proof = tx_data.submitted_proof_of(leaf_idx)?;
					(proof, None)
				},
				SubTrie::Bridge => {
					let message = tx_data.bridged.get(leaf_idx).map(|b| b.addr_msg.message.clone());
					let proof = tx_data.bridged_proof_of(leaf_idx)?;
					(proof, message)
				},
			};

			let roots = tx_data.roots();
			let data_proof = DataProof::new(roots, sub_proof);
			let proof = ProofResponse::new(data_proof, message);
			log::trace!(
				target: LOG_TARGET,
				"KateApi::data_proof: proof={proof:#?}");

			Some(proof)
		}

		fn rows(block_number: u32, extrinsics: Vec<OpaqueExtrinsic>, block_len: BlockLength, rows: Vec<u32>) -> Result<Vec<GRow>, RTKateError> {
			let app_exts = build_tx_data_from_opaque::<RTExtractor, RTExtrinsic, _, _>(block_number, extrinsics).to_app_extrinsics();
			let grid_rows = RTKate::<Runtime>::grid(app_exts, block_len, rows)?;
			log::trace!(target: LOG_TARGET, "KateApi::rows: rows={grid_rows:#?}");
			Ok(grid_rows)
		}

		fn app_data(block_number: u32, extrinsic: Vec<OpaqueExtrinsic>, block_len: BlockLength, id: AppId) -> Result<Vec<Option<GRow>>, RTKateError> {
			let app_exts = build_tx_data_from_opaque::<RTExtractor, RTExtrinsic, _, _>(block_number, extrinsic).to_app_extrinsics();
			let grid_rows = RTKate::<Runtime>::app_data(app_exts, block_len, id)?;
			log::trace!(target: LOG_TARGET, "KateApi::app_data: rows={grid_rows:#?}");
			Ok(grid_rows)
		}

		fn proof(block_number: u32, extrinsics: Vec<OpaqueExtrinsic>, block_len: BlockLength, cells: Vec<(u32,u32)> ) -> Result<Vec<GDataProof>, RTKateError> {
			let app_exts = build_tx_data_from_opaque::<RTExtractor, RTExtrinsic, _, _>(block_number, extrinsics).to_app_extrinsics();
			let data_proofs = RTKate::<Runtime>::proof(app_exts, block_len, cells)?;
			log::trace!(target: LOG_TARGET, "KateApi::proof: data_proofs={data_proofs:#?}");
			Ok(data_proofs)
		}
	}

	impl avail_base::PostInherentsProvider<Block> for Runtime {
		fn create_post_inherent_extrinsics(data: avail_base::StorageMap) -> Vec<<Block as BlockT>::Extrinsic> {
			pallet_vector::Pallet::<Runtime>::create_inherent(&data)
				.into_iter()
				.filter_map(|inherent| {
					<Block as BlockT>::Extrinsic::new(inherent.into(), None)
				})
				.collect()
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here. If any of the pre/post migration checks fail, we shall stop
			// right here and right now.
			let weight = Executive::try_runtime_upgrade(checks).unwrap();
			(weight, constants::system::RuntimeBlockWeights::get().max_block)
		}

		fn execute_block(
			block: Block,
			state_root_check: bool,
			signature_check: bool,
			select: frame_try_runtime::TryStateSelect
		) -> Weight {
			log::info!(
				target: LOG_TARGET,
				"try-runtime: executing block {:?} / root checks: {:?} / try-state-select: {:?}",
				block.header.hash(),
				state_root_check,
				select,
			);
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here.
			Executive::try_execute_block(block, state_root_check, signature_check, select).unwrap()
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use crate::{AllPalletsWithSystem, list_benchmarks};

			// Trying to add benchmarks directly to the Session Pallet caused cyclic dependency
			// issues. To get around that, we separated the Session benchmarks into its own crate,
			// which is why we need these two lines below.
			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();

			(list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch};
			use sp_storage::TrackedStorageKey;
			use crate::{add_benchmarks, Treasury, AllPalletsWithSystem};

			// Trying to add benchmarks directly to the Session Pallet caused cyclic dependency
			// issues. To get around that, we separated the Session benchmarks into its own crate,
			// which is why we need these two lines below.
			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			impl frame_system_benchmarking::Config for Runtime {}
			impl baseline::Config for Runtime {}

			use frame_support::traits::WhitelistedStorageKeys;
			let mut whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();

			// Treasury Account
			let treasury_key = frame_system::Account::<Runtime>::hashed_key_for(Treasury::account_id());
			whitelist.push(treasury_key.to_vec().into());

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);
			add_benchmarks!(params, batches);
			Ok(batches)
		}
	}

	impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
		fn create_default_config() -> Vec<u8> {
			create_default_config::<RuntimeGenesisConfig>()
		}

		fn build_config(config: Vec<u8>) -> sp_genesis_builder::Result {
			build_config::<RuntimeGenesisConfig>(config)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use avail_core::data_proof::{AddressedMessage, BoundedData, Message};
	use frame_system::data_root::{BridgedData, TxData};
	use hex_literal::hex;
	use sp_std::{vec, vec::Vec};
	use test_case::test_case;

	const SEND_ARBITRARY_DATA: &[u8] = &hex!("8400d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01e6052f8eddddbd425c2794829ca84c3382ea8b14f3e193824900523650149e2a9dea3cf9e417068ddc04d9ac9563e6b22ea11be1505e4a6a82f70864b01af38214000800002703000c313233000000000000000000000000000000000000000000000000000000000000003254");
	const FAILED_TXS: &[u8] = &hex!("04270b080c10");

	fn expected_send_arbitrary_data() -> TxData {
		let message = Message::ArbitraryMessage(BoundedData::truncate_from(b"123".to_vec()));
		let addr_msg = AddressedMessage::new(
			message,
			hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").into(),
			hex!("0000000000000000000000000000000000000000000000000000000000000032").into(),
			1,
			21,
			0,
		);
		BridgedData::new(0, addr_msg).into()
	}

	fn expected_failed_send_txs() -> TxData {
		TxData::failed_send_msg_txs(vec![3, 4])
	}

	fn expected_all() -> TxData {
		[expected_send_arbitrary_data(), expected_failed_send_txs()]
			.into_iter()
			.collect()
	}

	#[test_case( vec![SEND_ARBITRARY_DATA.to_vec()] => expected_send_arbitrary_data(); "Vector Send Arbitrary")]
	#[test_case( vec![FAILED_TXS.to_vec()] => expected_failed_send_txs(); "Post Inherent failed tx")]
	#[test_case( vec![SEND_ARBITRARY_DATA.to_vec(), FAILED_TXS.to_vec()] => expected_all(); "all")]
	fn kate_data_proof(raw_extrinsics: Vec<Vec<u8>>) -> TxData {
		let extrinsics = raw_extrinsics
			.into_iter()
			.map(|raw| OpaqueExtrinsic(raw))
			.collect::<Vec<OpaqueExtrinsic>>();
		build_tx_data_from_opaque::<RTExtractor, RTExtrinsic, _, _>(0, extrinsics)
	}
}
