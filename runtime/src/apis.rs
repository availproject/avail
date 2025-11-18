// use super::kate::{Error as RTKateError, GDataProof, GRow};
use crate::{
	constants,
	// kate::{GCellBlock, GMultiProof},
	mmr,
	version::VERSION,
	AccountId,
	AuthorityDiscovery,
	Babe,
	Block,
	BlockNumber,
	EpochDuration,
	Executive,
	Grandpa,
	Historical,
	Index,
	InherentDataExt,
	Mmr,
	NominationPools,
	OpaqueMetadata,
	Runtime,
	RuntimeCall,
	RuntimeGenesisConfig,
	SessionKeys,
	Staking,
	System,
	TransactionPayment,
};
use avail_base::{HeaderExtensionBuilderData, ProvidePostInherent};
use avail_core::{currency::Balance, header::HeaderExtension, DataProof, OpaqueExtrinsic};

use frame_system::limits::BlockLength;

use frame_support::{
	genesis_builder_helper::{build_config, create_default_config},
	traits::KeyOwnerProofSystem,
	weights::Weight,
};
use pallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};
use scale_info::prelude::string::String;
use sp_api::{decl_runtime_apis, impl_runtime_apis};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::KeyTypeId, H256, U256};
use sp_inherents::{CheckInherentsResult, InherentData};
use sp_runtime::{
	traits::{Block as BlockT, Extrinsic as ExtrinsicT, NumberFor},
	transaction_validity::{TransactionSource, TransactionValidity},
	AccountId32, ApplyExtrinsicResult, SaturatedConversion,
};
use sp_std::{borrow::Cow, vec::Vec};
use sp_version::RuntimeVersion;

type RTExtractor = <Runtime as frame_system::Config>::HeaderExtensionDataFilter;
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
		fn check_if_extrinsic_is_vector_post_inherent(uxt: &<Block as BlockT>::Extrinsic) -> bool;
		fn check_if_extrinsic_is_da_post_inherent(uxt: &<Block as BlockT>::Extrinsic) -> bool;
		fn extract_post_inherent_summaries(uxt: &<Block as BlockT>::Extrinsic) -> Option<Vec<da_control::BlobTxSummaryRuntime>>;
	}

	pub trait VectorApi {
		fn sync_committee_poseidons(slot: u64) -> U256;
		fn head() -> u64;
		fn headers(slot: u64) -> H256;
	}

	pub trait KateApi {
		// fn data_proof(block_number: u32, extrinsics: Vec<OpaqueExtrinsic>, tx_idx: u32) -> Option<ProofResponse>;
		// fn rows(block_number: u32, extrinsics: Vec<OpaqueExtrinsic>, block_len: BlockLength, rows: Vec<u32>) -> Result<Vec<GRow>, RTKateError >;
		// fn proof(block_number: u32, extrinsics: Vec<OpaqueExtrinsic>, block_len: BlockLength, cells: Vec<(u32,u32)> ) -> Result<Vec<GDataProof>, RTKateError>;
		// fn multiproof(block_number: u32, extrinsics: Vec<OpaqueExtrinsic>, block_len: BlockLength, cells: Vec<(u32,u32)> ) -> Result<Vec<(GMultiProof, GCellBlock)>, RTKateError>;
		fn inclusion_proof(extrinsics: Vec<OpaqueExtrinsic>, blob_hash: H256) -> Option<DataProof>;
	}

	pub trait BlobApi {
		/// Map (key_type, raw key bytes) -> owner AccountId (if registered in Session)
		fn get_validator_from_key(id: KeyTypeId, key_data: Vec<u8>) -> Option<AccountId>;

		/// Return the ACTIVE validators for the current session (AccountIds).
		fn get_active_validators() -> Vec<AccountId>;

		/// Return blob runtime parameters.
		fn get_blob_runtime_parameters() -> da_control::BlobRuntimeParameters;

		/// Expose accounts nonce
		fn account_nonce(who: AccountId32) -> u32;

		/// Get the blob vouch fee reserve amount
		fn get_blob_vouch_fee_reserve() -> u128;
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

	impl frame_system_rpc_runtime_api::SystemEventsApi<Block> for Runtime {
		fn fetch_events_v1(options: frame_system_rpc_runtime_api::system_events_api::fetch_events_v1::Options) -> frame_system_rpc_runtime_api::system_events_api::fetch_events_v1::ApiResult {
			use sp_std::vec;
			use frame_system_rpc_runtime_api::system_events_api::fetch_events_v1::{RuntimeEvent, GroupedRuntimeEvents, ERROR_INVALID_INPUTS};
			use codec::Encode;

			let filter = options.filter.unwrap_or_default();
			if !filter.is_valid() {
				return Err(ERROR_INVALID_INPUTS);
			}

			let enable_encoding = options.enable_encoding.unwrap_or(false);
			let enable_decoding = options.enable_decoding.unwrap_or(false);

			let mut result: Vec<GroupedRuntimeEvents> = Vec::new();
			let all_events = System::read_events_no_consensus();
			for (position, event) in all_events.enumerate() {
				if !filter.should_allow(event.phase) {
					continue
				}

				let encoded = event.event.encode();
				if encoded.len() <2 {
					continue
				}

				let emitted_index: (u8, u8) = (encoded[0], encoded[1]);
				let encoded = enable_encoding.then_some(encoded);
				let decoded = enable_decoding.then(|| decode_runtime_event_v1(&event.event)).flatten();

				let ev = RuntimeEvent::new(position as u32, emitted_index, encoded, decoded);
				if let Some(entry) = result.iter_mut().find(|x| x.phase == event.phase) {
					entry.events.push(ev);
				} else {
					result.push(GroupedRuntimeEvents {phase: event.phase, events: vec![ev]});
				};
			}

			Ok(result)
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
			let bl = frame_system::Pallet::<Runtime>::block_length();
			let cols = bl.cols.0;
			let rows = bl.rows.0;
			HeaderExtensionBuilderData::from_opaque_extrinsics::<RTExtractor>(block, &extrinsics, cols, rows).data_root()
		}

		fn build_extension(
			_extrinsics: Vec<OpaqueExtrinsic>,
			_data_root: H256,
			_block_length: BlockLength,
			_block_number: u32,
		) -> HeaderExtension {
			// use frame_system::native::hosted_header_builder::da::HeaderExtensionBuilder;
			// use frame_system::HeaderExtensionBuilder as _;

			// let bl = frame_system::Pallet::<Runtime>::block_length();
			// let cols = bl.cols.0;
			// let rows = bl.rows.0;
			// let app_extrinsics = HeaderExtensionBuilderData::from_opaque_extrinsics::<RTExtractor>(block_number, &extrinsics, cols, rows).to_app_extrinsics();
			// HeaderExtensionBuilder::<Runtime>::build(app_extrinsics, data_root, block_length, block_number)
			// Currentlt this API is used for V3 header generaton, which we no longer support for time being
			todo!()
		}

		fn check_if_extrinsic_is_vector_post_inherent(uxt: &<Block as BlockT>::Extrinsic) -> bool {
			use frame_support::traits::ExtrinsicCall;

			let Ok(xt) =  TryInto::<&RTExtrinsic>::try_into(uxt);

			let vector_pallet_call = match xt.call() {
				RuntimeCall::Vector(call) => call,
				_ => return false
			};


			matches!(vector_pallet_call, pallet_vector::Call::failed_send_message_txs {failed_txs: _})
		}

		fn check_if_extrinsic_is_da_post_inherent(uxt: &<Block as BlockT>::Extrinsic) -> bool {
			use frame_support::traits::ExtrinsicCall;

			let Ok(xt) =  TryInto::<&RTExtrinsic>::try_into(uxt);

			let da_pallet_call = match xt.call() {
				RuntimeCall::DataAvailability(call) => call,
				_ => return false
			};


			matches!(da_pallet_call, da_control::Call::submit_blob_txs_summary { total_blob_size: _, nb_blobs: _, blob_txs_summary: _})
		}

		fn extract_post_inherent_summaries(uxt: &<Block as BlockT>::Extrinsic) -> Option<Vec<da_control::BlobTxSummaryRuntime>> {
			use frame_support::traits::ExtrinsicCall;

			let Ok(xt) =  TryInto::<&RTExtrinsic>::try_into(uxt);

			let da_pallet_call = match xt.call() {
				RuntimeCall::DataAvailability(call) => call,
				_ => return None
			};

			if let da_control::Call::submit_blob_txs_summary { total_blob_size: _, nb_blobs: _, blob_txs_summary } = da_pallet_call {
				Some(blob_txs_summary.clone())
			} else {
				None
			}
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

	impl crate::apis::BlobApi<Block> for Runtime {
		fn get_validator_from_key(id: KeyTypeId, key_data: Vec<u8>) -> Option<AccountId> {
			pallet_session::Pallet::<Runtime>::key_owner(id, &key_data)
		}

		fn get_active_validators() -> Vec<AccountId> {
			pallet_session::Pallet::<Runtime>::validators()
		}

		fn get_blob_runtime_parameters() -> da_control::BlobRuntimeParameters {
			da_control::Pallet::<Runtime>::blob_runtime_parameters()
		}

		fn account_nonce(who: AccountId32) -> u32 {
			frame_system::Pallet::<Self>::account_nonce(who)
		}

		fn get_blob_vouch_fee_reserve() -> u128 {
			crate::constants::da::BlobVouchFeeReserve::get()
		}
	}

	impl crate::apis::KateApi<Block> for Runtime {
	// 	fn data_proof(block_number: u32, extrinsics: Vec<OpaqueExtrinsic>, tx_idx: u32) -> Option<ProofResponse> {
	// 		let bl = frame_system::Pallet::<Runtime>::block_length();
	// 		let cols = bl.cols.0;
	// 		let rows = bl.rows.0;
	// 		let data = HeaderExtensionBuilderData::from_opaque_extrinsics::<RTExtractor>(block_number, &extrinsics, cols, rows);
	// 		let (leaf_idx, sub_trie) = data.leaf_idx(tx_idx)?;
	// 		log::trace!(
	// 			target: LOG_TARGET,
	// 			"KateApi::data_proof: tx_idx={tx_idx:?} leaf_idx={leaf_idx:?}, sub_trie:{sub_trie:?}");

	// 		let (sub_proof, message) = match sub_trie {
	// 			SubTrie::DataSubmit => {
	// 				let proof = data.submitted_proof_of(leaf_idx)?;
	// 				(proof, None)
	// 			},
	// 			SubTrie::Bridge => {
	// 				let message = data.bridge_messages.get(leaf_idx).map(|b| b.addr_msg.clone());
	// 				let proof = data.bridged_proof_of(leaf_idx)?;
	// 				(proof, message)
	// 			},
	// 		};

	// 		let roots = data.roots();
	// 		let data_proof = DataProof::new(sub_trie, roots, sub_proof);
	// 		let proof = ProofResponse::new(data_proof, message);
	// 		log::trace!(
	// 			target: LOG_TARGET,
	// 			"KateApi::data_proof: proof={proof:#?}");

	// 		Some(proof)
	// 	}

		fn inclusion_proof(extrinsics: Vec<OpaqueExtrinsic>, blob_hash: H256) -> Option<DataProof> {
			// TODO: block_number, rows & cols has no significance in this case, should be refactored later
			let builder_data = HeaderExtensionBuilderData::from_opaque_extrinsics::<RTExtractor>(0, &extrinsics, 0, 0);

			let (leaf_idx, sub_trie) = builder_data.leaf_idx_by_hash(blob_hash)?;

			let sub_proof = builder_data.submitted_proof_of(leaf_idx)?;
			let roots = builder_data.roots();
			let data_proof = DataProof::new(sub_trie, roots, sub_proof);
			Some(data_proof)
		}

		// fn rows(block_number: u32, extrinsics: Vec<OpaqueExtrinsic>, block_len: BlockLength, rows: Vec<u32>) -> Result<Vec<GRow>, RTKateError> {
		// 	let bl = frame_system::Pallet::<Runtime>::block_length();
		// 	let cols = bl.cols.0;
		// 	let rows2 = bl.rows.0;
		// 	let app_extrinsics = HeaderExtensionBuilderData::from_opaque_extrinsics::<RTExtractor>(block_number, &extrinsics, cols, rows2).to_app_extrinsics();
		// 	let grid_rows = super::kate::grid::<Runtime>(app_extrinsics, block_len, rows)?;
		// 	log::trace!(target: LOG_TARGET, "KateApi::rows: rows={grid_rows:#?}");
		// 	Ok(grid_rows)
		// }

	// 	fn proof(block_number: u32, extrinsics: Vec<OpaqueExtrinsic>, block_len: BlockLength, cells: Vec<(u32,u32)> ) -> Result<Vec<GDataProof>, RTKateError> {
	// 		let bl = frame_system::Pallet::<Runtime>::block_length();
	// 		let cols = bl.cols.0;
	// 		let rows = bl.rows.0;
	// 		let app_extrinsics = HeaderExtensionBuilderData::from_opaque_extrinsics::<RTExtractor>(block_number, &extrinsics, cols, rows).to_app_extrinsics();
	// 		let data_proofs = super::kate::proof::<Runtime>(app_extrinsics, block_len, cells)?;
	// 		log::trace!(target: LOG_TARGET, "KateApi::proof: data_proofs={data_proofs:#?}");
	// 		Ok(data_proofs)
	// 	}

	// 	fn multiproof(block_number: u32, extrinsics: Vec<OpaqueExtrinsic>, block_len: BlockLength, cells: Vec<(u32,u32)> ) -> Result<Vec<(GMultiProof, GCellBlock)>, RTKateError> {
	// 		let app_extrinsics = HeaderExtensionBuilderData::from_opaque_extrinsics::<RTExtractor>(block_number, &extrinsics, block_len.cols.0, block_len.rows.0).to_app_extrinsics();
	// 		let data_proofs = super::kate::multiproof::<Runtime>(app_extrinsics, block_len, cells)?;
	// 		log::trace!(target: LOG_TARGET, "KateApi::proof: data_proofs={data_proofs:#?}");
	// 		Ok(data_proofs)
	// 	}
	}

	impl avail_base::PostInherentsProvider<Block> for Runtime {
		fn create_post_inherent_extrinsics(data: avail_base::StorageMap, blob_txs_summary: Vec<(
			H256,
			u32,
			bool,
			Option<String>,
			Vec<(AccountId32, AuthorityDiscoveryId, String, Vec<u8>)>,
		)>, total_blob_size: u64) -> Vec<<Block as BlockT>::Extrinsic> {
			let mut post_inherent_extrinsics: Vec<<Block as BlockT>::Extrinsic> = pallet_vector::Pallet::<Runtime>::create_inherent(&data)
				.into_iter()
				.filter_map(|inherent| {
					<Block as BlockT>::Extrinsic::new(inherent.into(), None)
				})
				.collect();

			let blob_txs_summary = da_control::BlobTxSummaryRuntime::convert_into(blob_txs_summary);
			let da_inherent_call: da_control::Call<Runtime> = da_control::Call::submit_blob_txs_summary { total_blob_size, nb_blobs:blob_txs_summary.len().saturated_into(),  blob_txs_summary };
			if let Some(da_inherent_extrinsic) = <Block as BlockT>::Extrinsic::new(da_inherent_call.into(), None) {
				post_inherent_extrinsics.insert(0, da_inherent_extrinsic);
			};

			post_inherent_extrinsics
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

	impl pallet_staking_runtime_api::StakingApi<Block, Balance, AccountId> for Runtime {
		fn nominations_quota(balance: Balance) -> u32 {
			Staking::api_nominations_quota(balance)
		}

		fn eras_stakers_page_count(era: sp_staking::EraIndex, account: AccountId) -> sp_staking::Page {
			Staking::api_eras_stakers_page_count(era, account)
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

fn decode_runtime_event_v1(event: &super::RuntimeEvent) -> Option<Vec<u8>> {
	use super::*;
	use codec::Encode;

	match event {
		RuntimeEvent::Sudo(e) => match e {
			pallet_sudo::Event::<Runtime>::Sudid { sudo_result } => {
				let mut event_data = Vec::<u8>::new();
				sudo_result.is_ok().encode_to(&mut event_data);

				return Some(event_data);
			},
			pallet_sudo::Event::<Runtime>::SudoAsDone { sudo_result } => {
				let mut event_data = Vec::<u8>::new();
				sudo_result.is_ok().encode_to(&mut event_data);

				return Some(event_data);
			},
			_ => (),
		},
		RuntimeEvent::Multisig(e) => {
			if let pallet_multisig::Event::<Runtime>::MultisigExecuted {
				multisig,
				call_hash,
				result: x,
				..
			} = e
			{
				let mut event_data = Vec::<u8>::new();
				multisig.encode_to(&mut event_data);
				call_hash.encode_to(&mut event_data);
				x.is_ok().encode_to(&mut event_data);

				return Some(event_data);
			}
		},
		RuntimeEvent::Proxy(e) => {
			if let pallet_proxy::Event::<Runtime>::ProxyExecuted { result, .. } = e {
				let mut event_data = Vec::<u8>::new();
				result.is_ok().encode_to(&mut event_data);

				return Some(event_data);
			}
		},
		RuntimeEvent::Scheduler(e) => {
			if let pallet_scheduler::Event::<Runtime>::Dispatched { result, .. } = e {
				let mut event_data = Vec::<u8>::new();
				result.is_ok().encode_to(&mut event_data);

				return Some(event_data);
			}
		},
		RuntimeEvent::DataAvailability(e) => {
			if let da_control::Event::<Runtime>::DataSubmitted { who, data_hash } = e {
				let mut event_data = Vec::<u8>::new();
				who.encode_to(&mut event_data);
				data_hash.encode_to(&mut event_data);

				return Some(event_data);
			}
		},
		_ => (),
	};

	None
}
