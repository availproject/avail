#[allow(clippy::all)]
#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
	use super::api as root_mod;
	pub static PALLETS: [&str; 34usize] = [
		"System",
		"Utility",
		"Babe",
		"Timestamp",
		"Authorship",
		"Indices",
		"Balances",
		"TransactionPayment",
		"ElectionProviderMultiPhase",
		"Staking",
		"Session",
		"TechnicalCommittee",
		"TechnicalMembership",
		"Grandpa",
		"Treasury",
		"Sudo",
		"ImOnline",
		"AuthorityDiscovery",
		"Offences",
		"Historical",
		"Scheduler",
		"Bounties",
		"Tips",
		"Mmr",
		"DataAvailability",
		"NomadUpdaterManager",
		"NomadHome",
		"NomadDABridge",
		"Preimage",
		"Multisig",
		"VoterList",
		"NominationPools",
		"Identity",
		"Mandate",
	];
	#[derive(
		:: subxt :: ext :: codec :: Decode,
		:: subxt :: ext :: codec :: Encode,
		Clone,
		Debug,
		Eq,
		PartialEq,
	)]
	pub enum Event {
		#[codec(index = 0)]
		System(system::Event),
		#[codec(index = 1)]
		Utility(utility::Event),
		#[codec(index = 5)]
		Indices(indices::Event),
		#[codec(index = 6)]
		Balances(balances::Event),
		#[codec(index = 7)]
		TransactionPayment(transaction_payment::Event),
		#[codec(index = 9)]
		ElectionProviderMultiPhase(election_provider_multi_phase::Event),
		#[codec(index = 10)]
		Staking(staking::Event),
		#[codec(index = 11)]
		Session(session::Event),
		#[codec(index = 14)]
		TechnicalCommittee(technical_committee::Event),
		#[codec(index = 16)]
		TechnicalMembership(technical_membership::Event),
		#[codec(index = 17)]
		Grandpa(grandpa::Event),
		#[codec(index = 18)]
		Treasury(treasury::Event),
		#[codec(index = 19)]
		Sudo(sudo::Event),
		#[codec(index = 20)]
		ImOnline(im_online::Event),
		#[codec(index = 22)]
		Offences(offences::Event),
		#[codec(index = 24)]
		Scheduler(scheduler::Event),
		#[codec(index = 25)]
		Bounties(bounties::Event),
		#[codec(index = 26)]
		Tips(tips::Event),
		#[codec(index = 29)]
		DataAvailability(data_availability::Event),
		#[codec(index = 30)]
		NomadUpdaterManager(nomad_updater_manager::Event),
		#[codec(index = 31)]
		NomadHome(nomad_home::Event),
		#[codec(index = 32)]
		NomadDABridge(nomad_da_bridge::Event),
		#[codec(index = 33)]
		Preimage(preimage::Event),
		#[codec(index = 34)]
		Multisig(multisig::Event),
		#[codec(index = 35)]
		VoterList(voter_list::Event),
		#[codec(index = 36)]
		NominationPools(nomination_pools::Event),
		#[codec(index = 37)]
		Identity(identity::Event),
		#[codec(index = 38)]
		Mandate(mandate::Event),
	}
	pub mod system {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Remark {
				pub remark: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetHeapPages {
				pub pages: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetCode {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetCodeWithoutChecks {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetStorage {
				pub items: ::std::vec::Vec<(
					::std::vec::Vec<::core::primitive::u8>,
					::std::vec::Vec<::core::primitive::u8>,
				)>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct KillStorage {
				pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct KillPrefix {
				pub prefix: ::std::vec::Vec<::core::primitive::u8>,
				pub subkeys: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct RemarkWithEvent {
				pub remark: ::std::vec::Vec<::core::primitive::u8>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::remark`]."]
				pub fn remark(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<Remark> {
					::subxt::tx::StaticTxPayload::new("System", "remark", Remark { remark }, [
						101u8, 80u8, 195u8, 226u8, 224u8, 247u8, 60u8, 128u8, 3u8, 101u8, 51u8,
						147u8, 96u8, 126u8, 76u8, 230u8, 194u8, 227u8, 191u8, 73u8, 160u8, 146u8,
						87u8, 147u8, 243u8, 28u8, 228u8, 116u8, 224u8, 181u8, 129u8, 160u8,
					])
				}

				#[doc = "See [`Pallet::set_heap_pages`]."]
				pub fn set_heap_pages(
					&self,
					pages: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<SetHeapPages> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"set_heap_pages",
						SetHeapPages { pages },
						[
							43u8, 103u8, 128u8, 49u8, 156u8, 136u8, 11u8, 204u8, 80u8, 6u8, 244u8,
							86u8, 171u8, 44u8, 140u8, 225u8, 142u8, 198u8, 43u8, 87u8, 26u8, 45u8,
							125u8, 222u8, 165u8, 254u8, 172u8, 158u8, 39u8, 178u8, 86u8, 87u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_code`]."]
				pub fn set_code(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<SetCode> {
					::subxt::tx::StaticTxPayload::new("System", "set_code", SetCode { code }, [
						27u8, 104u8, 244u8, 205u8, 188u8, 254u8, 121u8, 13u8, 106u8, 120u8, 244u8,
						108u8, 97u8, 84u8, 100u8, 68u8, 26u8, 69u8, 93u8, 128u8, 107u8, 4u8, 3u8,
						142u8, 13u8, 134u8, 196u8, 62u8, 113u8, 181u8, 14u8, 40u8,
					])
				}

				#[doc = "See [`Pallet::set_code_without_checks`]."]
				pub fn set_code_without_checks(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<SetCodeWithoutChecks> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"set_code_without_checks",
						SetCodeWithoutChecks { code },
						[
							102u8, 160u8, 125u8, 235u8, 30u8, 23u8, 45u8, 239u8, 112u8, 148u8,
							159u8, 158u8, 42u8, 93u8, 206u8, 94u8, 80u8, 250u8, 66u8, 195u8, 60u8,
							40u8, 142u8, 169u8, 183u8, 80u8, 80u8, 96u8, 3u8, 231u8, 99u8, 216u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_storage`]."]
				pub fn set_storage(
					&self,
					items: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
				) -> ::subxt::tx::StaticTxPayload<SetStorage> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"set_storage",
						SetStorage { items },
						[
							74u8, 43u8, 106u8, 255u8, 50u8, 151u8, 192u8, 155u8, 14u8, 90u8, 19u8,
							45u8, 165u8, 16u8, 235u8, 242u8, 21u8, 131u8, 33u8, 172u8, 119u8, 78u8,
							140u8, 10u8, 107u8, 202u8, 122u8, 235u8, 181u8, 191u8, 22u8, 116u8,
						],
					)
				}

				#[doc = "See [`Pallet::kill_storage`]."]
				pub fn kill_storage(
					&self,
					keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				) -> ::subxt::tx::StaticTxPayload<KillStorage> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"kill_storage",
						KillStorage { keys },
						[
							174u8, 174u8, 13u8, 174u8, 75u8, 138u8, 128u8, 235u8, 222u8, 216u8,
							85u8, 18u8, 198u8, 1u8, 138u8, 70u8, 19u8, 108u8, 209u8, 41u8, 228u8,
							67u8, 130u8, 230u8, 160u8, 207u8, 11u8, 180u8, 139u8, 242u8, 41u8,
							15u8,
						],
					)
				}

				#[doc = "See [`Pallet::kill_prefix`]."]
				pub fn kill_prefix(
					&self,
					prefix: ::std::vec::Vec<::core::primitive::u8>,
					subkeys: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<KillPrefix> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"kill_prefix",
						KillPrefix { prefix, subkeys },
						[
							203u8, 116u8, 217u8, 42u8, 154u8, 215u8, 77u8, 217u8, 13u8, 22u8,
							193u8, 2u8, 128u8, 115u8, 179u8, 115u8, 187u8, 218u8, 129u8, 34u8,
							80u8, 4u8, 173u8, 120u8, 92u8, 35u8, 237u8, 112u8, 201u8, 207u8, 200u8,
							48u8,
						],
					)
				}

				#[doc = "See [`Pallet::remark_with_event`]."]
				pub fn remark_with_event(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<RemarkWithEvent> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"remark_with_event",
						RemarkWithEvent { remark },
						[
							123u8, 225u8, 180u8, 179u8, 144u8, 74u8, 27u8, 85u8, 101u8, 75u8,
							134u8, 44u8, 181u8, 25u8, 183u8, 158u8, 14u8, 213u8, 56u8, 225u8,
							136u8, 88u8, 26u8, 114u8, 178u8, 43u8, 176u8, 43u8, 240u8, 84u8, 116u8,
							46u8,
						],
					)
				}
			}
		}
		#[doc = "Event for the System pallet."]
		pub type Event = runtime_types::frame_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An extrinsic completed successfully."]
			pub struct ExtrinsicSuccess {
				pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
				const EVENT: &'static str = "ExtrinsicSuccess";
				const PALLET: &'static str = "System";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An extrinsic failed."]
			pub struct ExtrinsicFailed {
				pub dispatch_error: runtime_types::sp_runtime::DispatchError,
				pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicFailed {
				const EVENT: &'static str = "ExtrinsicFailed";
				const PALLET: &'static str = "System";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "`:code` was updated."]
			pub struct CodeUpdated;
			impl ::subxt::events::StaticEvent for CodeUpdated {
				const EVENT: &'static str = "CodeUpdated";
				const PALLET: &'static str = "System";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A new account was created."]
			pub struct NewAccount {
				pub account: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for NewAccount {
				const EVENT: &'static str = "NewAccount";
				const PALLET: &'static str = "System";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An account was reaped."]
			pub struct KilledAccount {
				pub account: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for KilledAccount {
				const EVENT: &'static str = "KilledAccount";
				const PALLET: &'static str = "System";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "On on-chain remark happened."]
			pub struct Remarked {
				pub sender: ::subxt::ext::sp_core::crypto::AccountId32,
				pub hash: ::subxt::ext::sp_core::H256,
			}
			impl ::subxt::events::StaticEvent for Remarked {
				const EVENT: &'static str = "Remarked";
				const PALLET: &'static str = "System";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "On on-chain remark happend called by Root."]
			pub struct RemarkedByRoot {
				pub hash: ::subxt::ext::sp_core::H256,
			}
			impl ::subxt::events::StaticEvent for RemarkedByRoot {
				const EVENT: &'static str = "RemarkedByRoot";
				const PALLET: &'static str = "System";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The full account information for a particular account ID."]
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::AccountInfo<
							::core::primitive::u32,
							runtime_types::pallet_balances::types::AccountData<
								::core::primitive::u128,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Account",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							248u8, 178u8, 160u8, 222u8, 45u8, 231u8, 115u8, 164u8, 98u8, 184u8,
							174u8, 206u8, 149u8, 190u8, 175u8, 34u8, 202u8, 230u8, 69u8, 218u8,
							83u8, 43u8, 170u8, 41u8, 106u8, 77u8, 233u8, 97u8, 114u8, 14u8, 155u8,
							131u8,
						],
					)
				}

				#[doc = " The full account information for a particular account ID."]
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::AccountInfo<
							::core::primitive::u32,
							runtime_types::pallet_balances::types::AccountData<
								::core::primitive::u128,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Account",
						Vec::new(),
						[
							248u8, 178u8, 160u8, 222u8, 45u8, 231u8, 115u8, 164u8, 98u8, 184u8,
							174u8, 206u8, 149u8, 190u8, 175u8, 34u8, 202u8, 230u8, 69u8, 218u8,
							83u8, 43u8, 170u8, 41u8, 106u8, 77u8, 233u8, 97u8, 114u8, 14u8, 155u8,
							131u8,
						],
					)
				}

				#[doc = " Total extrinsics count for the current block."]
				pub fn extrinsic_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ExtrinsicCount",
						vec![],
						[
							223u8, 60u8, 201u8, 120u8, 36u8, 44u8, 180u8, 210u8, 242u8, 53u8,
							222u8, 154u8, 123u8, 176u8, 249u8, 8u8, 225u8, 28u8, 232u8, 4u8, 136u8,
							41u8, 151u8, 82u8, 189u8, 149u8, 49u8, 166u8, 139u8, 9u8, 163u8, 231u8,
						],
					)
				}

				#[doc = " The current weight for the block."]
				pub fn block_weight(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_support::dispatch::PerDispatchClass<
							runtime_types::sp_weights::weight_v2::Weight,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"BlockWeight",
						vec![],
						[
							120u8, 67u8, 71u8, 163u8, 36u8, 202u8, 52u8, 106u8, 143u8, 155u8,
							144u8, 87u8, 142u8, 241u8, 232u8, 183u8, 56u8, 235u8, 27u8, 237u8,
							20u8, 202u8, 33u8, 85u8, 189u8, 0u8, 28u8, 52u8, 198u8, 40u8, 219u8,
							54u8,
						],
					)
				}

				#[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
				pub fn all_extrinsics_len(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::frame_system::ExtrinsicLen>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"AllExtrinsicsLen",
						vec![],
						[
							139u8, 123u8, 64u8, 5u8, 243u8, 234u8, 235u8, 6u8, 225u8, 27u8, 65u8,
							41u8, 104u8, 177u8, 170u8, 192u8, 102u8, 111u8, 87u8, 192u8, 111u8,
							233u8, 122u8, 148u8, 45u8, 172u8, 30u8, 173u8, 155u8, 45u8, 196u8,
							233u8,
						],
					)
				}

				#[doc = " Map of block numbers to block hashes."]
				pub fn block_hash(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"BlockHash",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8,
							21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8,
							80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8,
						],
					)
				}

				#[doc = " Map of block numbers to block hashes."]
				pub fn block_hash_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"BlockHash",
						Vec::new(),
						[
							50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8,
							21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8,
							80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8,
						],
					)
				}

				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ExtrinsicData",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
							254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
							59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
						],
					)
				}

				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ExtrinsicData",
						Vec::new(),
						[
							210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
							254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
							59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
						],
					)
				}

				#[doc = " The current block number being processed. Set by `execute_block`."]
				pub fn number(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Number",
						vec![],
						[
							228u8, 96u8, 102u8, 190u8, 252u8, 130u8, 239u8, 172u8, 126u8, 235u8,
							246u8, 139u8, 208u8, 15u8, 88u8, 245u8, 141u8, 232u8, 43u8, 204u8,
							36u8, 87u8, 211u8, 141u8, 187u8, 68u8, 236u8, 70u8, 193u8, 235u8,
							164u8, 191u8,
						],
					)
				}

				#[doc = " Hash of the previous block."]
				pub fn parent_hash(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ParentHash",
						vec![],
						[
							232u8, 206u8, 177u8, 119u8, 38u8, 57u8, 233u8, 50u8, 225u8, 49u8,
							169u8, 176u8, 210u8, 51u8, 231u8, 176u8, 234u8, 186u8, 188u8, 112u8,
							15u8, 152u8, 195u8, 232u8, 201u8, 97u8, 208u8, 249u8, 9u8, 163u8, 69u8,
							36u8,
						],
					)
				}

				#[doc = " Digest of the current block, also part of the block header."]
				pub fn digest(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::generic::digest::Digest,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Digest",
						vec![],
						[
							83u8, 141u8, 200u8, 132u8, 182u8, 55u8, 197u8, 122u8, 13u8, 159u8,
							31u8, 42u8, 60u8, 191u8, 89u8, 221u8, 242u8, 47u8, 199u8, 213u8, 48u8,
							216u8, 131u8, 168u8, 245u8, 82u8, 56u8, 190u8, 62u8, 69u8, 96u8, 37u8,
						],
					)
				}

				#[doc = " Events deposited for the current block."]
				#[doc = ""]
				#[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
				#[doc = " It could otherwise inflate the PoV size of a block."]
				#[doc = ""]
				#[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
				#[doc = " just in case someone still reads them from within the runtime."]
				pub fn events(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::frame_system::EventRecord<
								runtime_types::da_runtime::RuntimeEvent,
								::subxt::ext::sp_core::H256,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Events",
						vec![],
						[
							168u8, 99u8, 90u8, 139u8, 124u8, 22u8, 85u8, 32u8, 113u8, 115u8, 237u8,
							140u8, 219u8, 175u8, 39u8, 207u8, 51u8, 179u8, 117u8, 159u8, 144u8,
							206u8, 17u8, 214u8, 121u8, 151u8, 185u8, 214u8, 3u8, 26u8, 70u8, 189u8,
						],
					)
				}

				#[doc = " The number of events in the `Events<T>` list."]
				pub fn event_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"EventCount",
						vec![],
						[
							236u8, 93u8, 90u8, 177u8, 250u8, 211u8, 138u8, 187u8, 26u8, 208u8,
							203u8, 113u8, 221u8, 233u8, 227u8, 9u8, 249u8, 25u8, 202u8, 185u8,
							161u8, 144u8, 167u8, 104u8, 127u8, 187u8, 38u8, 18u8, 52u8, 61u8, 66u8,
							112u8,
						],
					)
				}

				#[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
				#[doc = " of events in the `<Events<T>>` list."]
				#[doc = ""]
				#[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
				#[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
				#[doc = " in case of changes fetch the list of events of interest."]
				#[doc = ""]
				#[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
				#[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
				#[doc = " no notification will be triggered thus the event might be lost."]
				pub fn event_topics(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"EventTopics",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8,
							63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8,
							111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8,
							38u8,
						],
					)
				}

				#[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
				#[doc = " of events in the `<Events<T>>` list."]
				#[doc = ""]
				#[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
				#[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
				#[doc = " in case of changes fetch the list of events of interest."]
				#[doc = ""]
				#[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
				#[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
				#[doc = " no notification will be triggered thus the event might be lost."]
				pub fn event_topics_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"EventTopics",
						Vec::new(),
						[
							205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8,
							63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8,
							111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8,
							38u8,
						],
					)
				}

				#[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
				pub fn last_runtime_upgrade(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::LastRuntimeUpgradeInfo,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"LastRuntimeUpgrade",
						vec![],
						[
							52u8, 37u8, 117u8, 111u8, 57u8, 130u8, 196u8, 14u8, 99u8, 77u8, 91u8,
							126u8, 178u8, 249u8, 78u8, 34u8, 9u8, 194u8, 92u8, 105u8, 113u8, 81u8,
							185u8, 127u8, 245u8, 184u8, 60u8, 29u8, 234u8, 182u8, 96u8, 196u8,
						],
					)
				}

				#[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
				pub fn upgraded_to_u32_ref_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"UpgradedToU32RefCount",
						vec![],
						[
							171u8, 88u8, 244u8, 92u8, 122u8, 67u8, 27u8, 18u8, 59u8, 175u8, 175u8,
							178u8, 20u8, 150u8, 213u8, 59u8, 222u8, 141u8, 32u8, 107u8, 3u8, 114u8,
							83u8, 250u8, 180u8, 233u8, 152u8, 54u8, 187u8, 99u8, 131u8, 204u8,
						],
					)
				}

				#[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
				#[doc = " (default) if not."]
				pub fn upgraded_to_triple_ref_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"UpgradedToTripleRefCount",
						vec![],
						[
							90u8, 33u8, 56u8, 86u8, 90u8, 101u8, 89u8, 133u8, 203u8, 56u8, 201u8,
							210u8, 244u8, 232u8, 150u8, 18u8, 51u8, 105u8, 14u8, 230u8, 103u8,
							155u8, 246u8, 99u8, 53u8, 207u8, 225u8, 128u8, 186u8, 76u8, 40u8,
							185u8,
						],
					)
				}

				#[doc = " The execution phase of the block."]
				pub fn execution_phase(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::frame_system::Phase>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ExecutionPhase",
						vec![],
						[
							230u8, 183u8, 221u8, 135u8, 226u8, 223u8, 55u8, 104u8, 138u8, 224u8,
							103u8, 156u8, 222u8, 99u8, 203u8, 199u8, 164u8, 168u8, 193u8, 133u8,
							201u8, 155u8, 63u8, 95u8, 17u8, 206u8, 165u8, 123u8, 161u8, 33u8,
							172u8, 93u8,
						],
					)
				}

				#[doc = " The dynamic block length"]
				pub fn dynamic_block_length(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::limits::BlockLength,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"DynamicBlockLength",
						vec![],
						[
							41u8, 235u8, 249u8, 191u8, 222u8, 101u8, 12u8, 241u8, 205u8, 177u8,
							72u8, 151u8, 167u8, 198u8, 91u8, 27u8, 202u8, 34u8, 24u8, 190u8, 208u8,
							220u8, 145u8, 91u8, 161u8, 179u8, 152u8, 118u8, 48u8, 74u8, 2u8, 28u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Block & extrinsics weights: base values and limits."]
				pub fn block_weights(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::limits::BlockWeights,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new("System", "BlockWeights", [
						118u8, 253u8, 239u8, 217u8, 145u8, 115u8, 85u8, 86u8, 172u8, 248u8, 139u8,
						32u8, 158u8, 126u8, 172u8, 188u8, 197u8, 105u8, 145u8, 235u8, 171u8, 50u8,
						31u8, 225u8, 167u8, 187u8, 241u8, 87u8, 6u8, 17u8, 234u8, 185u8,
					])
				}

				#[doc = " The maximum length of a block (in bytes)."]
				pub fn block_length(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::limits::BlockLength,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new("System", "BlockLength", [
						242u8, 210u8, 220u8, 76u8, 196u8, 181u8, 230u8, 121u8, 80u8, 216u8, 116u8,
						56u8, 16u8, 204u8, 254u8, 191u8, 53u8, 101u8, 115u8, 117u8, 163u8, 218u8,
						212u8, 187u8, 95u8, 233u8, 16u8, 179u8, 179u8, 130u8, 102u8, 158u8,
					])
				}

				#[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
				pub fn block_hash_count(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("System", "BlockHashCount", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " The weight of runtime database operations the runtime can invoke."]
				pub fn db_weight(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::sp_weights::RuntimeDbWeight>,
				> {
					::subxt::constants::StaticConstantAddress::new("System", "DbWeight", [
						124u8, 162u8, 190u8, 149u8, 49u8, 177u8, 162u8, 231u8, 62u8, 167u8, 199u8,
						181u8, 43u8, 232u8, 185u8, 116u8, 195u8, 51u8, 233u8, 223u8, 20u8, 129u8,
						246u8, 13u8, 65u8, 180u8, 64u8, 9u8, 157u8, 59u8, 245u8, 118u8,
					])
				}

				#[doc = " Get the chain's current version."]
				pub fn version(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::sp_version::RuntimeVersion>,
				> {
					::subxt::constants::StaticConstantAddress::new("System", "Version", [
						93u8, 98u8, 57u8, 243u8, 229u8, 8u8, 234u8, 231u8, 72u8, 230u8, 139u8,
						47u8, 63u8, 181u8, 17u8, 2u8, 220u8, 231u8, 104u8, 237u8, 185u8, 143u8,
						165u8, 253u8, 188u8, 76u8, 147u8, 12u8, 170u8, 26u8, 74u8, 200u8,
					])
				}

				#[doc = " The designated SS58 prefix of this chain."]
				#[doc = ""]
				#[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
				#[doc = " that the runtime should know about the prefix in order to make use of it as"]
				#[doc = " an identifier of the chain."]
				pub fn ss58_prefix(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u16>,
				> {
					::subxt::constants::StaticConstantAddress::new("System", "SS58Prefix", [
						116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8, 41u8,
						144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8, 90u8,
						208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
					])
				}
			}
		}
	}
	pub mod utility {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Batch {
				pub calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct AsDerivative {
				pub index: ::core::primitive::u16,
				pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct BatchAll {
				pub calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct DispatchAs {
				pub as_origin: ::std::boxed::Box<runtime_types::da_runtime::OriginCaller>,
				pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ForceBatch {
				pub calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct WithWeight {
				pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::batch`]."]
				pub fn batch(
					&self,
					calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
				) -> ::subxt::tx::StaticTxPayload<Batch> {
					::subxt::tx::StaticTxPayload::new("Utility", "batch", Batch { calls }, [
						116u8, 76u8, 253u8, 133u8, 50u8, 23u8, 192u8, 79u8, 111u8, 51u8, 191u8,
						51u8, 151u8, 214u8, 78u8, 67u8, 222u8, 25u8, 113u8, 100u8, 70u8, 102u8,
						160u8, 45u8, 39u8, 153u8, 159u8, 163u8, 203u8, 159u8, 120u8, 111u8,
					])
				}

				#[doc = "See [`Pallet::as_derivative`]."]
				pub fn as_derivative(
					&self,
					index: ::core::primitive::u16,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<AsDerivative> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"as_derivative",
						AsDerivative {
							index,
							call: ::std::boxed::Box::new(call),
						},
						[
							156u8, 144u8, 137u8, 44u8, 87u8, 74u8, 218u8, 216u8, 207u8, 30u8,
							117u8, 183u8, 87u8, 201u8, 108u8, 148u8, 136u8, 24u8, 212u8, 173u8,
							220u8, 8u8, 219u8, 116u8, 207u8, 90u8, 165u8, 38u8, 120u8, 180u8, 55u8,
							223u8,
						],
					)
				}

				#[doc = "See [`Pallet::batch_all`]."]
				pub fn batch_all(
					&self,
					calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
				) -> ::subxt::tx::StaticTxPayload<BatchAll> {
					::subxt::tx::StaticTxPayload::new("Utility", "batch_all", BatchAll { calls }, [
						234u8, 46u8, 233u8, 165u8, 178u8, 9u8, 248u8, 205u8, 74u8, 226u8, 91u8,
						183u8, 156u8, 213u8, 60u8, 2u8, 86u8, 134u8, 194u8, 165u8, 33u8, 186u8,
						33u8, 126u8, 217u8, 238u8, 150u8, 83u8, 132u8, 3u8, 120u8, 108u8,
					])
				}

				#[doc = "See [`Pallet::dispatch_as`]."]
				pub fn dispatch_as(
					&self,
					as_origin: runtime_types::da_runtime::OriginCaller,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<DispatchAs> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"dispatch_as",
						DispatchAs {
							as_origin: ::std::boxed::Box::new(as_origin),
							call: ::std::boxed::Box::new(call),
						},
						[
							213u8, 225u8, 229u8, 141u8, 184u8, 239u8, 209u8, 209u8, 142u8, 72u8,
							37u8, 134u8, 97u8, 72u8, 211u8, 76u8, 192u8, 56u8, 125u8, 171u8, 98u8,
							57u8, 67u8, 201u8, 148u8, 118u8, 228u8, 132u8, 46u8, 230u8, 247u8,
							13u8,
						],
					)
				}

				#[doc = "See [`Pallet::force_batch`]."]
				pub fn force_batch(
					&self,
					calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
				) -> ::subxt::tx::StaticTxPayload<ForceBatch> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"force_batch",
						ForceBatch { calls },
						[
							187u8, 100u8, 150u8, 117u8, 230u8, 57u8, 192u8, 61u8, 1u8, 124u8, 58u8,
							211u8, 215u8, 239u8, 229u8, 100u8, 72u8, 214u8, 62u8, 41u8, 36u8,
							139u8, 79u8, 43u8, 113u8, 27u8, 46u8, 163u8, 124u8, 228u8, 188u8,
							225u8,
						],
					)
				}

				#[doc = "See [`Pallet::with_weight`]."]
				pub fn with_weight(
					&self,
					call: runtime_types::da_runtime::RuntimeCall,
					weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::StaticTxPayload<WithWeight> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"with_weight",
						WithWeight {
							call: ::std::boxed::Box::new(call),
							weight,
						},
						[
							42u8, 111u8, 215u8, 255u8, 41u8, 146u8, 224u8, 66u8, 1u8, 69u8, 215u8,
							28u8, 162u8, 223u8, 100u8, 167u8, 30u8, 51u8, 25u8, 183u8, 253u8, 77u8,
							185u8, 74u8, 67u8, 34u8, 121u8, 218u8, 124u8, 217u8, 92u8, 114u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_utility::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Batch of dispatches did not complete fully. Index of first failing dispatch given, as"]
			#[doc = "well as the error."]
			pub struct BatchInterrupted {
				pub index: ::core::primitive::u32,
				pub error: runtime_types::sp_runtime::DispatchError,
			}
			impl ::subxt::events::StaticEvent for BatchInterrupted {
				const EVENT: &'static str = "BatchInterrupted";
				const PALLET: &'static str = "Utility";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Batch of dispatches completed fully with no error."]
			pub struct BatchCompleted;
			impl ::subxt::events::StaticEvent for BatchCompleted {
				const EVENT: &'static str = "BatchCompleted";
				const PALLET: &'static str = "Utility";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Batch of dispatches completed but has errors."]
			pub struct BatchCompletedWithErrors;
			impl ::subxt::events::StaticEvent for BatchCompletedWithErrors {
				const EVENT: &'static str = "BatchCompletedWithErrors";
				const PALLET: &'static str = "Utility";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A single item within a Batch of dispatches has completed with no error."]
			pub struct ItemCompleted;
			impl ::subxt::events::StaticEvent for ItemCompleted {
				const EVENT: &'static str = "ItemCompleted";
				const PALLET: &'static str = "Utility";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A single item within a Batch of dispatches has completed with error."]
			pub struct ItemFailed {
				pub error: runtime_types::sp_runtime::DispatchError,
			}
			impl ::subxt::events::StaticEvent for ItemFailed {
				const EVENT: &'static str = "ItemFailed";
				const PALLET: &'static str = "Utility";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A call was dispatched."]
			pub struct DispatchedAs {
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for DispatchedAs {
				const EVENT: &'static str = "DispatchedAs";
				const PALLET: &'static str = "Utility";
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The limit on the number of batched calls."]
				pub fn batched_calls_limit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Utility",
						"batched_calls_limit",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod babe {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ReportEquivocation {
				pub equivocation_proof: ::std::boxed::Box<
					runtime_types::sp_consensus_slots::EquivocationProof<
						runtime_types::avail_core::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
						runtime_types::sp_consensus_babe::app::Public,
					>,
				>,
				pub key_owner_proof: runtime_types::sp_session::MembershipProof,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ReportEquivocationUnsigned {
				pub equivocation_proof: ::std::boxed::Box<
					runtime_types::sp_consensus_slots::EquivocationProof<
						runtime_types::avail_core::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
						runtime_types::sp_consensus_babe::app::Public,
					>,
				>,
				pub key_owner_proof: runtime_types::sp_session::MembershipProof,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct PlanConfigChange {
				pub config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::report_equivocation`]."]
				pub fn report_equivocation(
					&self,
					equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
						runtime_types::avail_core::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
						runtime_types::sp_consensus_babe::app::Public,
					>,
					key_owner_proof: runtime_types::sp_session::MembershipProof,
				) -> ::subxt::tx::StaticTxPayload<ReportEquivocation> {
					::subxt::tx::StaticTxPayload::new(
						"Babe",
						"report_equivocation",
						ReportEquivocation {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							83u8, 29u8, 91u8, 18u8, 161u8, 50u8, 202u8, 65u8, 17u8, 151u8, 25u8,
							255u8, 78u8, 245u8, 144u8, 169u8, 113u8, 67u8, 9u8, 83u8, 20u8, 7u8,
							205u8, 244u8, 63u8, 67u8, 103u8, 33u8, 204u8, 128u8, 198u8, 99u8,
						],
					)
				}

				#[doc = "See [`Pallet::report_equivocation_unsigned`]."]
				pub fn report_equivocation_unsigned(
					&self,
					equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
						runtime_types::avail_core::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
						runtime_types::sp_consensus_babe::app::Public,
					>,
					key_owner_proof: runtime_types::sp_session::MembershipProof,
				) -> ::subxt::tx::StaticTxPayload<ReportEquivocationUnsigned> {
					::subxt::tx::StaticTxPayload::new(
						"Babe",
						"report_equivocation_unsigned",
						ReportEquivocationUnsigned {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							5u8, 78u8, 86u8, 186u8, 250u8, 62u8, 207u8, 60u8, 251u8, 253u8, 231u8,
							14u8, 185u8, 55u8, 220u8, 75u8, 247u8, 205u8, 237u8, 134u8, 176u8,
							121u8, 166u8, 247u8, 238u8, 43u8, 71u8, 122u8, 66u8, 159u8, 239u8,
							53u8,
						],
					)
				}

				#[doc = "See [`Pallet::plan_config_change`]."]
				pub fn plan_config_change(
					&self,
					config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
				) -> ::subxt::tx::StaticTxPayload<PlanConfigChange> {
					::subxt::tx::StaticTxPayload::new(
						"Babe",
						"plan_config_change",
						PlanConfigChange { config },
						[
							229u8, 157u8, 41u8, 58u8, 56u8, 4u8, 52u8, 107u8, 104u8, 20u8, 42u8,
							110u8, 1u8, 17u8, 45u8, 196u8, 30u8, 135u8, 63u8, 46u8, 40u8, 137u8,
							209u8, 37u8, 24u8, 108u8, 251u8, 189u8, 77u8, 208u8, 74u8, 32u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Current epoch index."]
				pub fn epoch_index(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"EpochIndex",
						vec![],
						[
							51u8, 27u8, 91u8, 156u8, 118u8, 99u8, 46u8, 219u8, 190u8, 147u8, 205u8,
							23u8, 106u8, 169u8, 121u8, 218u8, 208u8, 235u8, 135u8, 127u8, 243u8,
							41u8, 55u8, 243u8, 235u8, 122u8, 57u8, 86u8, 37u8, 90u8, 208u8, 71u8,
						],
					)
				}

				#[doc = " Current epoch authorities."]
				pub fn authorities(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
							runtime_types::sp_consensus_babe::app::Public,
							::core::primitive::u64,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"Authorities",
						vec![],
						[
							61u8, 8u8, 133u8, 111u8, 169u8, 120u8, 0u8, 213u8, 31u8, 159u8, 204u8,
							212u8, 18u8, 205u8, 93u8, 84u8, 140u8, 108u8, 136u8, 209u8, 234u8,
							107u8, 145u8, 9u8, 204u8, 224u8, 105u8, 9u8, 238u8, 241u8, 65u8, 30u8,
						],
					)
				}

				#[doc = " The slot at which the first epoch actually started. This is 0"]
				#[doc = " until the first block of the chain."]
				pub fn genesis_slot(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::sp_consensus_slots::Slot>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"GenesisSlot",
						vec![],
						[
							234u8, 127u8, 243u8, 100u8, 124u8, 160u8, 66u8, 248u8, 48u8, 218u8,
							61u8, 52u8, 54u8, 142u8, 158u8, 77u8, 32u8, 63u8, 156u8, 39u8, 94u8,
							255u8, 192u8, 238u8, 170u8, 118u8, 58u8, 42u8, 199u8, 61u8, 199u8,
							77u8,
						],
					)
				}

				#[doc = " Current slot number."]
				pub fn current_slot(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::sp_consensus_slots::Slot>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"CurrentSlot",
						vec![],
						[
							139u8, 237u8, 185u8, 137u8, 251u8, 179u8, 69u8, 167u8, 133u8, 168u8,
							204u8, 64u8, 178u8, 123u8, 92u8, 250u8, 119u8, 190u8, 208u8, 178u8,
							208u8, 176u8, 124u8, 187u8, 74u8, 165u8, 33u8, 78u8, 161u8, 206u8, 8u8,
							108u8,
						],
					)
				}

				#[doc = " The epoch randomness for the *current* epoch."]
				#[doc = ""]
				#[doc = " # Security"]
				#[doc = ""]
				#[doc = " This MUST NOT be used for gambling, as it can be influenced by a"]
				#[doc = " malicious validator in the short term. It MAY be used in many"]
				#[doc = " cryptographic protocols, however, so long as one remembers that this"]
				#[doc = " (like everything else on-chain) it is public. For example, it can be"]
				#[doc = " used where a number is needed that cannot have been chosen by an"]
				#[doc = " adversary, for purposes such as public-coin zero-knowledge proofs."]
				pub fn randomness(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<[::core::primitive::u8; 32usize]>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"Randomness",
						vec![],
						[
							191u8, 197u8, 25u8, 164u8, 104u8, 248u8, 247u8, 193u8, 244u8, 60u8,
							181u8, 195u8, 248u8, 90u8, 41u8, 199u8, 82u8, 123u8, 72u8, 126u8, 18u8,
							17u8, 128u8, 215u8, 34u8, 251u8, 227u8, 70u8, 166u8, 10u8, 104u8,
							140u8,
						],
					)
				}

				#[doc = " Pending epoch configuration change that will be applied when the next epoch is enacted."]
				pub fn pending_epoch_config_change(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"PendingEpochConfigChange",
						vec![],
						[
							4u8, 201u8, 0u8, 204u8, 47u8, 246u8, 4u8, 185u8, 163u8, 242u8, 242u8,
							152u8, 29u8, 222u8, 71u8, 127u8, 49u8, 203u8, 206u8, 180u8, 244u8,
							50u8, 80u8, 49u8, 199u8, 97u8, 3u8, 170u8, 156u8, 139u8, 106u8, 113u8,
						],
					)
				}

				#[doc = " Next epoch randomness."]
				pub fn next_randomness(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<[::core::primitive::u8; 32usize]>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"NextRandomness",
						vec![],
						[
							185u8, 98u8, 45u8, 109u8, 253u8, 38u8, 238u8, 221u8, 240u8, 29u8, 38u8,
							107u8, 118u8, 117u8, 131u8, 115u8, 21u8, 255u8, 203u8, 81u8, 243u8,
							251u8, 91u8, 60u8, 163u8, 202u8, 125u8, 193u8, 173u8, 234u8, 166u8,
							92u8,
						],
					)
				}

				#[doc = " Next epoch authorities."]
				pub fn next_authorities(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
							runtime_types::sp_consensus_babe::app::Public,
							::core::primitive::u64,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"NextAuthorities",
						vec![],
						[
							201u8, 193u8, 164u8, 18u8, 155u8, 253u8, 124u8, 163u8, 143u8, 73u8,
							212u8, 20u8, 241u8, 108u8, 110u8, 5u8, 171u8, 66u8, 224u8, 208u8, 10u8,
							65u8, 148u8, 164u8, 1u8, 12u8, 216u8, 83u8, 20u8, 226u8, 254u8, 183u8,
						],
					)
				}

				#[doc = " Randomness under construction."]
				#[doc = ""]
				#[doc = " We make a trade-off between storage accesses and list length."]
				#[doc = " We store the under-construction randomness in segments of up to"]
				#[doc = " `UNDER_CONSTRUCTION_SEGMENT_LENGTH`."]
				#[doc = ""]
				#[doc = " Once a segment reaches this length, we begin the next one."]
				#[doc = " We reset all segments and return to `0` at the beginning of every"]
				#[doc = " epoch."]
				pub fn segment_index(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"SegmentIndex",
						vec![],
						[
							128u8, 45u8, 87u8, 58u8, 174u8, 152u8, 241u8, 156u8, 56u8, 192u8, 19u8,
							45u8, 75u8, 160u8, 35u8, 253u8, 145u8, 11u8, 178u8, 81u8, 114u8, 117u8,
							112u8, 107u8, 163u8, 208u8, 240u8, 151u8, 102u8, 176u8, 246u8, 5u8,
						],
					)
				}

				#[doc = " TWOX-NOTE: `SegmentIndex` is an increasing integer, so this is okay."]
				pub fn under_construction(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							[::core::primitive::u8; 32usize],
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"UnderConstruction",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							180u8, 4u8, 149u8, 245u8, 231u8, 92u8, 99u8, 170u8, 254u8, 172u8,
							182u8, 3u8, 152u8, 156u8, 132u8, 196u8, 140u8, 97u8, 7u8, 84u8, 220u8,
							89u8, 195u8, 177u8, 235u8, 51u8, 98u8, 144u8, 73u8, 238u8, 59u8, 164u8,
						],
					)
				}

				#[doc = " TWOX-NOTE: `SegmentIndex` is an increasing integer, so this is okay."]
				pub fn under_construction_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							[::core::primitive::u8; 32usize],
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"UnderConstruction",
						Vec::new(),
						[
							180u8, 4u8, 149u8, 245u8, 231u8, 92u8, 99u8, 170u8, 254u8, 172u8,
							182u8, 3u8, 152u8, 156u8, 132u8, 196u8, 140u8, 97u8, 7u8, 84u8, 220u8,
							89u8, 195u8, 177u8, 235u8, 51u8, 98u8, 144u8, 73u8, 238u8, 59u8, 164u8,
						],
					)
				}

				#[doc = " Temporary value (cleared at block finalization) which is `Some`"]
				#[doc = " if per-block initialization has already been called for current block."]
				pub fn initialized(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::core::option::Option<
							runtime_types::sp_consensus_babe::digests::PreDigest,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"Initialized",
						vec![],
						[
							40u8, 135u8, 28u8, 144u8, 247u8, 208u8, 48u8, 220u8, 46u8, 60u8, 131u8,
							190u8, 196u8, 235u8, 126u8, 66u8, 34u8, 14u8, 32u8, 131u8, 71u8, 46u8,
							62u8, 207u8, 177u8, 213u8, 167u8, 34u8, 199u8, 29u8, 16u8, 236u8,
						],
					)
				}

				#[doc = " This field should always be populated during block processing unless"]
				#[doc = " secondary plain slots are enabled (which don't contain a VRF output)."]
				#[doc = ""]
				#[doc = " It is set in `on_finalize`, before it will contain the value from the last block."]
				pub fn author_vrf_randomness(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::core::option::Option<[::core::primitive::u8; 32usize]>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"AuthorVrfRandomness",
						vec![],
						[
							66u8, 235u8, 74u8, 252u8, 222u8, 135u8, 19u8, 28u8, 74u8, 191u8, 170u8,
							197u8, 207u8, 127u8, 77u8, 121u8, 138u8, 138u8, 110u8, 187u8, 34u8,
							14u8, 230u8, 43u8, 241u8, 241u8, 63u8, 163u8, 53u8, 179u8, 250u8,
							247u8,
						],
					)
				}

				#[doc = " The block numbers when the last and current epoch have started, respectively `N-1` and"]
				#[doc = " `N`."]
				#[doc = " NOTE: We track this is in order to annotate the block number when a given pool of"]
				#[doc = " entropy was fixed (i.e. it was known to chain observers). Since epochs are defined in"]
				#[doc = " slots, which may be skipped, the block numbers may not line up with the slot numbers."]
				pub fn epoch_start(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"EpochStart",
						vec![],
						[
							196u8, 39u8, 241u8, 20u8, 150u8, 180u8, 136u8, 4u8, 195u8, 205u8,
							218u8, 10u8, 130u8, 131u8, 168u8, 243u8, 207u8, 249u8, 58u8, 195u8,
							177u8, 119u8, 110u8, 243u8, 241u8, 3u8, 245u8, 56u8, 157u8, 5u8, 68u8,
							60u8,
						],
					)
				}

				#[doc = " How late the current block is compared to its parent."]
				#[doc = ""]
				#[doc = " This entry is populated as part of block execution and is cleaned up"]
				#[doc = " on block finalization. Querying this storage entry outside of block"]
				#[doc = " execution context should always yield zero."]
				pub fn lateness(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"Lateness",
						vec![],
						[
							229u8, 230u8, 224u8, 89u8, 49u8, 213u8, 198u8, 236u8, 144u8, 56u8,
							193u8, 234u8, 62u8, 242u8, 191u8, 199u8, 105u8, 131u8, 74u8, 63u8,
							75u8, 1u8, 210u8, 49u8, 3u8, 128u8, 18u8, 77u8, 219u8, 146u8, 60u8,
							88u8,
						],
					)
				}

				#[doc = " The configuration for the current epoch. Should never be `None` as it is initialized in"]
				#[doc = " genesis."]
				pub fn epoch_config(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_consensus_babe::BabeEpochConfiguration,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"EpochConfig",
						vec![],
						[
							41u8, 118u8, 141u8, 244u8, 72u8, 17u8, 125u8, 203u8, 43u8, 153u8,
							203u8, 119u8, 117u8, 223u8, 123u8, 133u8, 73u8, 235u8, 130u8, 21u8,
							160u8, 167u8, 16u8, 173u8, 177u8, 35u8, 117u8, 97u8, 149u8, 49u8,
							220u8, 24u8,
						],
					)
				}

				#[doc = " The configuration for the next epoch, `None` if the config will not change"]
				#[doc = " (you can fallback to `EpochConfig` instead in that case)."]
				pub fn next_epoch_config(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_consensus_babe::BabeEpochConfiguration,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"NextEpochConfig",
						vec![],
						[
							111u8, 182u8, 144u8, 180u8, 92u8, 146u8, 102u8, 249u8, 196u8, 229u8,
							226u8, 30u8, 25u8, 198u8, 133u8, 9u8, 136u8, 95u8, 11u8, 151u8, 139u8,
							156u8, 105u8, 228u8, 181u8, 12u8, 175u8, 148u8, 174u8, 33u8, 233u8,
							228u8,
						],
					)
				}

				#[doc = " A list of the last 100 skipped epochs and the corresponding session index"]
				#[doc = " when the epoch was skipped."]
				#[doc = ""]
				#[doc = " This is only used for validating equivocation proofs. An equivocation proof"]
				#[doc = " must contains a key-ownership proof for a given session, therefore we need a"]
				#[doc = " way to tie together sessions and epoch indices, i.e. we need to validate that"]
				#[doc = " a validator was the owner of a given key on a given session, and what the"]
				#[doc = " active epoch index was during that session."]
				pub fn skipped_epochs(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<(
							::core::primitive::u64,
							::core::primitive::u32,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"SkippedEpochs",
						vec![],
						[
							187u8, 66u8, 178u8, 110u8, 247u8, 41u8, 128u8, 194u8, 173u8, 197u8,
							28u8, 219u8, 112u8, 75u8, 9u8, 184u8, 51u8, 12u8, 121u8, 117u8, 176u8,
							213u8, 139u8, 144u8, 122u8, 72u8, 243u8, 105u8, 248u8, 63u8, 6u8, 87u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The amount of time, in slots, that each epoch should last."]
				#[doc = " NOTE: Currently it is not possible to change the epoch duration after"]
				#[doc = " the chain has started. Attempting to do so will brick block production."]
				pub fn epoch_duration(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new("Babe", "EpochDuration", [
						128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8, 59u8,
						226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8, 103u8, 119u8,
						53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8, 246u8,
					])
				}

				#[doc = " The expected average block time at which BABE should be creating"]
				#[doc = " blocks. Since BABE is probabilistic it is not trivial to figure out"]
				#[doc = " what the expected average block time should be based on the slot"]
				#[doc = " duration and the security parameter `c` (where `1 - c` represents"]
				#[doc = " the probability of a slot being empty)."]
				pub fn expected_block_time(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new("Babe", "ExpectedBlockTime", [
						128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8, 59u8,
						226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8, 103u8, 119u8,
						53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8, 246u8,
					])
				}

				#[doc = " Max number of authorities allowed"]
				pub fn max_authorities(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Babe", "MaxAuthorities", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}
			}
		}
	}
	pub mod timestamp {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Set {
				#[codec(compact)]
				pub now: ::core::primitive::u64,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::set`]."]
				pub fn set(
					&self,
					now: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<Set> {
					::subxt::tx::StaticTxPayload::new("Timestamp", "set", Set { now }, [
						6u8, 97u8, 172u8, 236u8, 118u8, 238u8, 228u8, 114u8, 15u8, 115u8, 102u8,
						85u8, 66u8, 151u8, 16u8, 33u8, 187u8, 17u8, 166u8, 88u8, 127u8, 214u8,
						182u8, 51u8, 168u8, 88u8, 43u8, 101u8, 185u8, 8u8, 1u8, 28u8,
					])
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Current time for the current block."]
				pub fn now(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Timestamp",
						"Now",
						vec![],
						[
							148u8, 53u8, 50u8, 54u8, 13u8, 161u8, 57u8, 150u8, 16u8, 83u8, 144u8,
							221u8, 59u8, 75u8, 158u8, 130u8, 39u8, 123u8, 106u8, 134u8, 202u8,
							185u8, 83u8, 85u8, 60u8, 41u8, 120u8, 96u8, 210u8, 34u8, 2u8, 250u8,
						],
					)
				}

				#[doc = " Did the timestamp get updated in this block?"]
				pub fn did_update(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Timestamp",
						"DidUpdate",
						vec![],
						[
							70u8, 13u8, 92u8, 186u8, 80u8, 151u8, 167u8, 90u8, 158u8, 232u8, 175u8,
							13u8, 103u8, 135u8, 2u8, 78u8, 16u8, 6u8, 39u8, 158u8, 167u8, 85u8,
							27u8, 47u8, 122u8, 73u8, 127u8, 26u8, 35u8, 168u8, 72u8, 204u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
				#[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
				#[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
				#[doc = " double this period on default settings."]
				pub fn minimum_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new("Timestamp", "MinimumPeriod", [
						128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8, 59u8,
						226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8, 103u8, 119u8,
						53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8, 246u8,
					])
				}
			}
		}
	}
	pub mod authorship {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Author of current block."]
				pub fn author(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Authorship",
						"Author",
						vec![],
						[
							149u8, 42u8, 33u8, 147u8, 190u8, 207u8, 174u8, 227u8, 190u8, 110u8,
							25u8, 131u8, 5u8, 167u8, 237u8, 188u8, 188u8, 33u8, 177u8, 126u8,
							181u8, 49u8, 126u8, 118u8, 46u8, 128u8, 154u8, 95u8, 15u8, 91u8, 103u8,
							113u8,
						],
					)
				}
			}
		}
	}
	pub mod indices {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Claim {
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Transfer {
				pub new: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Free {
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ForceTransfer {
				pub new: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub index: ::core::primitive::u32,
				pub freeze: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Freeze {
				pub index: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::claim`]."]
				pub fn claim(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Claim> {
					::subxt::tx::StaticTxPayload::new("Indices", "claim", Claim { index }, [
						5u8, 24u8, 11u8, 173u8, 226u8, 170u8, 0u8, 30u8, 193u8, 102u8, 214u8, 59u8,
						252u8, 32u8, 221u8, 88u8, 196u8, 189u8, 244u8, 18u8, 233u8, 37u8, 228u8,
						248u8, 76u8, 175u8, 212u8, 233u8, 238u8, 203u8, 162u8, 68u8,
					])
				}

				#[doc = "See [`Pallet::transfer`]."]
				pub fn transfer(
					&self,
					new: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Transfer> {
					::subxt::tx::StaticTxPayload::new(
						"Indices",
						"transfer",
						Transfer { new, index },
						[
							1u8, 83u8, 197u8, 184u8, 8u8, 96u8, 48u8, 146u8, 116u8, 76u8, 229u8,
							115u8, 226u8, 215u8, 41u8, 154u8, 27u8, 34u8, 205u8, 188u8, 10u8,
							169u8, 203u8, 39u8, 2u8, 236u8, 181u8, 162u8, 115u8, 254u8, 42u8, 28u8,
						],
					)
				}

				#[doc = "See [`Pallet::free`]."]
				pub fn free(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Free> {
					::subxt::tx::StaticTxPayload::new("Indices", "free", Free { index }, [
						133u8, 202u8, 225u8, 127u8, 69u8, 145u8, 43u8, 13u8, 160u8, 248u8, 215u8,
						243u8, 232u8, 166u8, 74u8, 203u8, 235u8, 138u8, 255u8, 27u8, 163u8, 71u8,
						254u8, 217u8, 6u8, 208u8, 202u8, 204u8, 238u8, 70u8, 126u8, 252u8,
					])
				}

				#[doc = "See [`Pallet::force_transfer`]."]
				pub fn force_transfer(
					&self,
					new: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					index: ::core::primitive::u32,
					freeze: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<ForceTransfer> {
					::subxt::tx::StaticTxPayload::new(
						"Indices",
						"force_transfer",
						ForceTransfer { new, index, freeze },
						[
							126u8, 8u8, 145u8, 175u8, 177u8, 153u8, 131u8, 123u8, 184u8, 53u8,
							72u8, 207u8, 21u8, 140u8, 87u8, 181u8, 172u8, 64u8, 37u8, 165u8, 121u8,
							111u8, 173u8, 224u8, 181u8, 79u8, 76u8, 134u8, 93u8, 169u8, 65u8,
							131u8,
						],
					)
				}

				#[doc = "See [`Pallet::freeze`]."]
				pub fn freeze(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Freeze> {
					::subxt::tx::StaticTxPayload::new("Indices", "freeze", Freeze { index }, [
						121u8, 45u8, 118u8, 2u8, 72u8, 48u8, 38u8, 7u8, 234u8, 204u8, 68u8, 20u8,
						76u8, 251u8, 205u8, 246u8, 149u8, 31u8, 168u8, 186u8, 208u8, 90u8, 40u8,
						47u8, 100u8, 228u8, 188u8, 33u8, 79u8, 220u8, 105u8, 209u8,
					])
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_indices::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A account index was assigned."]
			pub struct IndexAssigned {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for IndexAssigned {
				const EVENT: &'static str = "IndexAssigned";
				const PALLET: &'static str = "Indices";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A account index has been freed up (unassigned)."]
			pub struct IndexFreed {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for IndexFreed {
				const EVENT: &'static str = "IndexFreed";
				const PALLET: &'static str = "Indices";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A account index has been frozen to its current account ID."]
			pub struct IndexFrozen {
				pub index: ::core::primitive::u32,
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for IndexFrozen {
				const EVENT: &'static str = "IndexFrozen";
				const PALLET: &'static str = "Indices";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The lookup from index to account."]
				pub fn accounts(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						::core::primitive::bool,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Indices",
						"Accounts",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							211u8, 169u8, 54u8, 254u8, 88u8, 57u8, 22u8, 223u8, 108u8, 27u8, 38u8,
							9u8, 202u8, 209u8, 111u8, 209u8, 144u8, 13u8, 211u8, 114u8, 239u8,
							127u8, 75u8, 166u8, 234u8, 222u8, 225u8, 35u8, 160u8, 163u8, 112u8,
							242u8,
						],
					)
				}

				#[doc = " The lookup from index to account."]
				pub fn accounts_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						::core::primitive::bool,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Indices",
						"Accounts",
						Vec::new(),
						[
							211u8, 169u8, 54u8, 254u8, 88u8, 57u8, 22u8, 223u8, 108u8, 27u8, 38u8,
							9u8, 202u8, 209u8, 111u8, 209u8, 144u8, 13u8, 211u8, 114u8, 239u8,
							127u8, 75u8, 166u8, 234u8, 222u8, 225u8, 35u8, 160u8, 163u8, 112u8,
							242u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The deposit needed for reserving an index."]
				pub fn deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new("Indices", "Deposit", [
						84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
						27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
						71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
					])
				}
			}
		}
	}
	pub mod balances {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct TransferAllowDeath {
				pub dest: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetBalanceDeprecated {
				pub who: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub new_free: ::core::primitive::u128,
				#[codec(compact)]
				pub old_reserved: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ForceTransfer {
				pub source: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub dest: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct TransferKeepAlive {
				pub dest: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct TransferAll {
				pub dest: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ForceUnreserve {
				pub who: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct UpgradeAccounts {
				pub who: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Transfer {
				pub dest: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ForceSetBalance {
				pub who: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub new_free: ::core::primitive::u128,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::transfer_allow_death`]."]
				pub fn transfer_allow_death(
					&self,
					dest: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<TransferAllowDeath> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"transfer_allow_death",
						TransferAllowDeath { dest, value },
						[
							169u8, 58u8, 41u8, 215u8, 117u8, 65u8, 173u8, 196u8, 215u8, 171u8,
							179u8, 31u8, 136u8, 58u8, 247u8, 90u8, 178u8, 246u8, 26u8, 131u8, 94u8,
							245u8, 221u8, 236u8, 120u8, 121u8, 227u8, 91u8, 71u8, 140u8, 235u8,
							208u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_balance_deprecated`]."]
				pub fn set_balance_deprecated(
					&self,
					who: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					new_free: ::core::primitive::u128,
					old_reserved: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<SetBalanceDeprecated> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"set_balance_deprecated",
						SetBalanceDeprecated {
							who,
							new_free,
							old_reserved,
						},
						[
							201u8, 253u8, 138u8, 90u8, 20u8, 49u8, 250u8, 9u8, 150u8, 190u8, 107u8,
							42u8, 197u8, 147u8, 243u8, 253u8, 171u8, 41u8, 180u8, 73u8, 116u8,
							62u8, 54u8, 15u8, 107u8, 251u8, 188u8, 150u8, 144u8, 215u8, 16u8, 31u8,
						],
					)
				}

				#[doc = "See [`Pallet::force_transfer`]."]
				pub fn force_transfer(
					&self,
					source: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					dest: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ForceTransfer> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"force_transfer",
						ForceTransfer {
							source,
							dest,
							value,
						},
						[
							56u8, 80u8, 186u8, 45u8, 134u8, 147u8, 200u8, 19u8, 53u8, 221u8, 213u8,
							32u8, 13u8, 51u8, 130u8, 42u8, 244u8, 85u8, 50u8, 246u8, 189u8, 51u8,
							93u8, 1u8, 108u8, 142u8, 112u8, 245u8, 104u8, 255u8, 15u8, 62u8,
						],
					)
				}

				#[doc = "See [`Pallet::transfer_keep_alive`]."]
				pub fn transfer_keep_alive(
					&self,
					dest: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<TransferKeepAlive> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"transfer_keep_alive",
						TransferKeepAlive { dest, value },
						[
							202u8, 239u8, 204u8, 0u8, 52u8, 57u8, 158u8, 8u8, 252u8, 178u8, 91u8,
							197u8, 238u8, 186u8, 205u8, 56u8, 217u8, 250u8, 21u8, 44u8, 239u8,
							66u8, 79u8, 99u8, 25u8, 106u8, 70u8, 226u8, 50u8, 255u8, 176u8, 71u8,
						],
					)
				}

				#[doc = "See [`Pallet::transfer_all`]."]
				pub fn transfer_all(
					&self,
					dest: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<TransferAll> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"transfer_all",
						TransferAll { dest, keep_alive },
						[
							118u8, 215u8, 198u8, 243u8, 4u8, 173u8, 108u8, 224u8, 113u8, 203u8,
							149u8, 23u8, 130u8, 176u8, 53u8, 205u8, 112u8, 147u8, 88u8, 167u8,
							197u8, 32u8, 104u8, 117u8, 201u8, 168u8, 144u8, 230u8, 120u8, 29u8,
							122u8, 159u8,
						],
					)
				}

				#[doc = "See [`Pallet::force_unreserve`]."]
				pub fn force_unreserve(
					&self,
					who: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ForceUnreserve> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"force_unreserve",
						ForceUnreserve { who, amount },
						[
							39u8, 229u8, 111u8, 44u8, 147u8, 80u8, 7u8, 26u8, 185u8, 121u8, 149u8,
							25u8, 151u8, 37u8, 124u8, 46u8, 108u8, 136u8, 167u8, 145u8, 103u8,
							65u8, 33u8, 168u8, 36u8, 214u8, 126u8, 237u8, 180u8, 61u8, 108u8,
							110u8,
						],
					)
				}

				#[doc = "See [`Pallet::upgrade_accounts`]."]
				pub fn upgrade_accounts(
					&self,
					who: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::tx::StaticTxPayload<UpgradeAccounts> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"upgrade_accounts",
						UpgradeAccounts { who },
						[
							164u8, 61u8, 119u8, 24u8, 165u8, 46u8, 197u8, 59u8, 39u8, 198u8, 228u8,
							96u8, 228u8, 45u8, 85u8, 51u8, 37u8, 5u8, 75u8, 40u8, 241u8, 163u8,
							86u8, 228u8, 151u8, 217u8, 47u8, 105u8, 203u8, 103u8, 207u8, 4u8,
						],
					)
				}

				#[doc = "See [`Pallet::transfer`]."]
				pub fn transfer(
					&self,
					dest: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Transfer> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"transfer",
						Transfer { dest, value },
						[
							255u8, 181u8, 144u8, 248u8, 64u8, 167u8, 5u8, 134u8, 208u8, 20u8,
							223u8, 103u8, 235u8, 35u8, 66u8, 184u8, 27u8, 94u8, 176u8, 60u8, 233u8,
							236u8, 145u8, 218u8, 44u8, 138u8, 240u8, 224u8, 16u8, 193u8, 220u8,
							95u8,
						],
					)
				}

				#[doc = "See [`Pallet::force_set_balance`]."]
				pub fn force_set_balance(
					&self,
					who: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					new_free: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ForceSetBalance> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"force_set_balance",
						ForceSetBalance { who, new_free },
						[
							124u8, 74u8, 215u8, 44u8, 119u8, 59u8, 187u8, 206u8, 142u8, 203u8,
							51u8, 31u8, 211u8, 18u8, 241u8, 82u8, 172u8, 34u8, 20u8, 241u8, 28u8,
							21u8, 27u8, 151u8, 205u8, 48u8, 42u8, 65u8, 157u8, 122u8, 116u8, 80u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_balances::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An account was created with some free balance."]
			pub struct Endowed {
				pub account: ::subxt::ext::sp_core::crypto::AccountId32,
				pub free_balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Endowed {
				const EVENT: &'static str = "Endowed";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
			#[doc = "resulting in an outright loss."]
			pub struct DustLost {
				pub account: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for DustLost {
				const EVENT: &'static str = "DustLost";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Transfer succeeded."]
			pub struct Transfer {
				pub from: ::subxt::ext::sp_core::crypto::AccountId32,
				pub to: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Transfer {
				const EVENT: &'static str = "Transfer";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A balance was set by root."]
			pub struct BalanceSet {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub free: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for BalanceSet {
				const EVENT: &'static str = "BalanceSet";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some balance was reserved (moved from free to reserved)."]
			pub struct Reserved {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Reserved {
				const EVENT: &'static str = "Reserved";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some balance was unreserved (moved from reserved to free)."]
			pub struct Unreserved {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unreserved {
				const EVENT: &'static str = "Unreserved";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some balance was moved from the reserve of the first account to the second account."]
			#[doc = "Final argument indicates the destination balance type."]
			pub struct ReserveRepatriated {
				pub from: ::subxt::ext::sp_core::crypto::AccountId32,
				pub to: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
				pub destination_status:
					runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
			}
			impl ::subxt::events::StaticEvent for ReserveRepatriated {
				const EVENT: &'static str = "ReserveRepatriated";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some amount was deposited (e.g. for transaction fees)."]
			pub struct Deposit {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposit {
				const EVENT: &'static str = "Deposit";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
			pub struct Withdraw {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdraw {
				const EVENT: &'static str = "Withdraw";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
			pub struct Slashed {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Slashed {
				const EVENT: &'static str = "Slashed";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some amount was minted into an account."]
			pub struct Minted {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Minted {
				const EVENT: &'static str = "Minted";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some amount was burned from an account."]
			pub struct Burned {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Burned {
				const EVENT: &'static str = "Burned";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some amount was suspended from an account (it can be restored later)."]
			pub struct Suspended {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Suspended {
				const EVENT: &'static str = "Suspended";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some amount was restored into an account."]
			pub struct Restored {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Restored {
				const EVENT: &'static str = "Restored";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An account was upgraded."]
			pub struct Upgraded {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Upgraded {
				const EVENT: &'static str = "Upgraded";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
			pub struct Issued {
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Issued {
				const EVENT: &'static str = "Issued";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
			pub struct Rescinded {
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rescinded {
				const EVENT: &'static str = "Rescinded";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some balance was locked."]
			pub struct Locked {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Locked {
				const EVENT: &'static str = "Locked";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some balance was unlocked."]
			pub struct Unlocked {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unlocked {
				const EVENT: &'static str = "Unlocked";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some balance was frozen."]
			pub struct Frozen {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Frozen {
				const EVENT: &'static str = "Frozen";
				const PALLET: &'static str = "Balances";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some balance was thawed."]
			pub struct Thawed {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Thawed {
				const EVENT: &'static str = "Thawed";
				const PALLET: &'static str = "Balances";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The total units issued in the system."]
				pub fn total_issuance(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"TotalIssuance",
						vec![],
						[
							1u8, 206u8, 252u8, 237u8, 6u8, 30u8, 20u8, 232u8, 164u8, 115u8, 51u8,
							156u8, 156u8, 206u8, 241u8, 187u8, 44u8, 84u8, 25u8, 164u8, 235u8,
							20u8, 86u8, 242u8, 124u8, 23u8, 28u8, 140u8, 26u8, 73u8, 231u8, 51u8,
						],
					)
				}

				#[doc = " The total units of outstanding deactivated balance in the system."]
				pub fn inactive_issuance(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"InactiveIssuance",
						vec![],
						[
							74u8, 203u8, 111u8, 142u8, 225u8, 104u8, 173u8, 51u8, 226u8, 12u8,
							85u8, 135u8, 41u8, 206u8, 177u8, 238u8, 94u8, 246u8, 184u8, 250u8,
							140u8, 213u8, 91u8, 118u8, 163u8, 111u8, 211u8, 46u8, 204u8, 160u8,
							154u8, 21u8,
						],
					)
				}

				#[doc = " The Balances pallet example of storing the balance of an account."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " You can also store the balance of an account in the `System` pallet."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "   type AccountStore = System"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
				#[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
				#[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Account",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							109u8, 250u8, 18u8, 96u8, 139u8, 232u8, 4u8, 139u8, 133u8, 239u8, 30u8,
							237u8, 73u8, 209u8, 143u8, 160u8, 94u8, 248u8, 124u8, 43u8, 224u8,
							165u8, 11u8, 6u8, 176u8, 144u8, 189u8, 161u8, 174u8, 210u8, 56u8,
							225u8,
						],
					)
				}

				#[doc = " The Balances pallet example of storing the balance of an account."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " You can also store the balance of an account in the `System` pallet."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "   type AccountStore = System"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
				#[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
				#[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Account",
						Vec::new(),
						[
							109u8, 250u8, 18u8, 96u8, 139u8, 232u8, 4u8, 139u8, 133u8, 239u8, 30u8,
							237u8, 73u8, 209u8, 143u8, 160u8, 94u8, 248u8, 124u8, 43u8, 224u8,
							165u8, 11u8, 6u8, 176u8, 144u8, 189u8, 161u8, 174u8, 210u8, 56u8,
							225u8,
						],
					)
				}

				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				pub fn locks(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							runtime_types::pallet_balances::types::BalanceLock<
								::core::primitive::u128,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Locks",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8,
							58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
							136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8,
						],
					)
				}

				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				pub fn locks_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							runtime_types::pallet_balances::types::BalanceLock<
								::core::primitive::u128,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Locks",
						Vec::new(),
						[
							216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8,
							58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
							136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8,
						],
					)
				}

				#[doc = " Named reserves on some account balances."]
				pub fn reserves(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::pallet_balances::types::ReserveData<
								[::core::primitive::u8; 8usize],
								::core::primitive::u128,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Reserves",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8,
							167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8,
							183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8,
						],
					)
				}

				#[doc = " Named reserves on some account balances."]
				pub fn reserves_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::pallet_balances::types::ReserveData<
								[::core::primitive::u8; 8usize],
								::core::primitive::u128,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Reserves",
						Vec::new(),
						[
							17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8,
							167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8,
							183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8,
						],
					)
				}

				#[doc = " Holds on account balances."]
				pub fn holds(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::pallet_balances::types::IdAmount<
								runtime_types::da_runtime::RuntimeHoldReason,
								::core::primitive::u128,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Holds",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							209u8, 212u8, 29u8, 29u8, 197u8, 118u8, 65u8, 232u8, 89u8, 192u8,
							255u8, 71u8, 50u8, 146u8, 96u8, 255u8, 32u8, 91u8, 147u8, 161u8, 213u8,
							215u8, 63u8, 134u8, 233u8, 70u8, 227u8, 132u8, 224u8, 30u8, 70u8,
							187u8,
						],
					)
				}

				#[doc = " Holds on account balances."]
				pub fn holds_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::pallet_balances::types::IdAmount<
								runtime_types::da_runtime::RuntimeHoldReason,
								::core::primitive::u128,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Holds",
						Vec::new(),
						[
							209u8, 212u8, 29u8, 29u8, 197u8, 118u8, 65u8, 232u8, 89u8, 192u8,
							255u8, 71u8, 50u8, 146u8, 96u8, 255u8, 32u8, 91u8, 147u8, 161u8, 213u8,
							215u8, 63u8, 134u8, 233u8, 70u8, 227u8, 132u8, 224u8, 30u8, 70u8,
							187u8,
						],
					)
				}

				#[doc = " Freeze locks on account balances."]
				pub fn freezes(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::pallet_balances::types::IdAmount<
								(),
								::core::primitive::u128,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Freezes",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							211u8, 24u8, 237u8, 217u8, 47u8, 230u8, 147u8, 39u8, 112u8, 209u8,
							193u8, 47u8, 242u8, 13u8, 241u8, 0u8, 100u8, 45u8, 116u8, 130u8, 246u8,
							196u8, 50u8, 134u8, 135u8, 112u8, 206u8, 1u8, 12u8, 53u8, 106u8, 131u8,
						],
					)
				}

				#[doc = " Freeze locks on account balances."]
				pub fn freezes_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::pallet_balances::types::IdAmount<
								(),
								::core::primitive::u128,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Freezes",
						Vec::new(),
						[
							211u8, 24u8, 237u8, 217u8, 47u8, 230u8, 147u8, 39u8, 112u8, 209u8,
							193u8, 47u8, 242u8, 13u8, 241u8, 0u8, 100u8, 45u8, 116u8, 130u8, 246u8,
							196u8, 50u8, 134u8, 135u8, 112u8, 206u8, 1u8, 12u8, 53u8, 106u8, 131u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!"]
				#[doc = ""]
				#[doc = " If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for"]
				#[doc = " this pallet. However, you do so at your own risk: this will open up a major DoS vector."]
				#[doc = " In case you have multiple sources of provider references, you may also get unexpected"]
				#[doc = " behaviour if you set this to zero."]
				#[doc = ""]
				#[doc = " Bottom line: Do yourself a favour and make it at least one!"]
				pub fn existential_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Balances",
						"ExistentialDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}

				#[doc = " The maximum number of locks that should exist on an account."]
				#[doc = " Not strictly enforced, but used for weight estimation."]
				pub fn max_locks(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Balances", "MaxLocks", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " The maximum number of named reserves that can exist on an account."]
				pub fn max_reserves(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Balances", "MaxReserves", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " The maximum number of holds that can exist on an account at any time."]
				pub fn max_holds(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Balances", "MaxHolds", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " The maximum number of individual freeze locks that can exist on an account at any time."]
				pub fn max_freezes(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Balances", "MaxFreezes", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}
			}
		}
	}
	pub mod transaction_payment {
		use super::{root_mod, runtime_types};
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
			#[doc = "has been paid by `who`."]
			pub struct TransactionFeePaid {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub actual_fee: ::core::primitive::u128,
				pub tip: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TransactionFeePaid {
				const EVENT: &'static str = "TransactionFeePaid";
				const PALLET: &'static str = "TransactionPayment";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn next_fee_multiplier(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::fixed_point::FixedU128,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TransactionPayment",
						"NextFeeMultiplier",
						vec![],
						[
							210u8, 0u8, 206u8, 165u8, 183u8, 10u8, 206u8, 52u8, 14u8, 90u8, 218u8,
							197u8, 189u8, 125u8, 113u8, 216u8, 52u8, 161u8, 45u8, 24u8, 245u8,
							237u8, 121u8, 41u8, 106u8, 29u8, 45u8, 129u8, 250u8, 203u8, 206u8,
							180u8,
						],
					)
				}

				pub fn storage_version(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_transaction_payment::Releases,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TransactionPayment",
						"StorageVersion",
						vec![],
						[
							219u8, 243u8, 82u8, 176u8, 65u8, 5u8, 132u8, 114u8, 8u8, 82u8, 176u8,
							200u8, 97u8, 150u8, 177u8, 164u8, 166u8, 11u8, 34u8, 12u8, 12u8, 198u8,
							58u8, 191u8, 186u8, 221u8, 221u8, 119u8, 181u8, 253u8, 154u8, 228u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " A fee mulitplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
				#[doc = " `priority`"]
				#[doc = ""]
				#[doc = " This value is multipled by the `final_fee` to obtain a \"virtual tip\" that is later"]
				#[doc = " added to a tip component in regular `priority` calculations."]
				#[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
				#[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
				#[doc = ""]
				#[doc = " ```rust,ignore"]
				#[doc = " // For `Normal`"]
				#[doc = " let priority = priority_calc(tip);"]
				#[doc = ""]
				#[doc = " // For `Operational`"]
				#[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
				#[doc = " let priority = priority_calc(tip + virtual_tip);"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
				#[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
				#[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
				#[doc = " transactions."]
				pub fn operational_fee_multiplier(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u8>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"TransactionPayment",
						"OperationalFeeMultiplier",
						[
							141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
							28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
							114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
							165u8,
						],
					)
				}
			}
		}
	}
	pub mod election_provider_multi_phase {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SubmitUnsigned {
				pub raw_solution: ::std::boxed::Box<
					runtime_types::pallet_election_provider_multi_phase::RawSolution<
						runtime_types::da_runtime::constants::staking::NposSolution16,
					>,
				>,
				pub witness:
					runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetMinimumUntrustedScore {
				pub maybe_next_score:
					::core::option::Option<runtime_types::sp_npos_elections::ElectionScore>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetEmergencyElectionResult {
				pub supports: ::std::vec::Vec<(
					::subxt::ext::sp_core::crypto::AccountId32,
					runtime_types::sp_npos_elections::Support<
						::subxt::ext::sp_core::crypto::AccountId32,
					>,
				)>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Submit {
				pub raw_solution: ::std::boxed::Box<
					runtime_types::pallet_election_provider_multi_phase::RawSolution<
						runtime_types::da_runtime::constants::staking::NposSolution16,
					>,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct GovernanceFallback {
				pub maybe_max_voters: ::core::option::Option<::core::primitive::u32>,
				pub maybe_max_targets: ::core::option::Option<::core::primitive::u32>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::submit_unsigned`]."]
				pub fn submit_unsigned(
					&self,
					raw_solution: runtime_types::pallet_election_provider_multi_phase::RawSolution<
						runtime_types::da_runtime::constants::staking::NposSolution16,
					>,
					witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize,
				) -> ::subxt::tx::StaticTxPayload<SubmitUnsigned> {
					::subxt::tx::StaticTxPayload::new(
						"ElectionProviderMultiPhase",
						"submit_unsigned",
						SubmitUnsigned {
							raw_solution: ::std::boxed::Box::new(raw_solution),
							witness,
						},
						[
							100u8, 240u8, 31u8, 34u8, 93u8, 98u8, 93u8, 57u8, 41u8, 197u8, 97u8,
							58u8, 242u8, 10u8, 69u8, 250u8, 185u8, 169u8, 21u8, 8u8, 202u8, 61u8,
							36u8, 25u8, 4u8, 148u8, 82u8, 56u8, 242u8, 18u8, 27u8, 219u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_minimum_untrusted_score`]."]
				pub fn set_minimum_untrusted_score(
					&self,
					maybe_next_score: ::core::option::Option<
						runtime_types::sp_npos_elections::ElectionScore,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetMinimumUntrustedScore> {
					::subxt::tx::StaticTxPayload::new(
						"ElectionProviderMultiPhase",
						"set_minimum_untrusted_score",
						SetMinimumUntrustedScore { maybe_next_score },
						[
							63u8, 101u8, 105u8, 146u8, 133u8, 162u8, 149u8, 112u8, 150u8, 219u8,
							183u8, 213u8, 234u8, 211u8, 144u8, 74u8, 106u8, 15u8, 62u8, 196u8,
							247u8, 49u8, 20u8, 48u8, 3u8, 105u8, 85u8, 46u8, 76u8, 4u8, 67u8, 81u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_emergency_election_result`]."]
				pub fn set_emergency_election_result(
					&self,
					supports: ::std::vec::Vec<(
						::subxt::ext::sp_core::crypto::AccountId32,
						runtime_types::sp_npos_elections::Support<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					)>,
				) -> ::subxt::tx::StaticTxPayload<SetEmergencyElectionResult> {
					::subxt::tx::StaticTxPayload::new(
						"ElectionProviderMultiPhase",
						"set_emergency_election_result",
						SetEmergencyElectionResult { supports },
						[
							115u8, 255u8, 205u8, 58u8, 153u8, 1u8, 246u8, 8u8, 225u8, 36u8, 66u8,
							144u8, 250u8, 145u8, 70u8, 76u8, 54u8, 63u8, 251u8, 51u8, 214u8, 204u8,
							55u8, 112u8, 46u8, 228u8, 255u8, 250u8, 151u8, 5u8, 44u8, 133u8,
						],
					)
				}

				#[doc = "See [`Pallet::submit`]."]
				pub fn submit(
					&self,
					raw_solution: runtime_types::pallet_election_provider_multi_phase::RawSolution<
						runtime_types::da_runtime::constants::staking::NposSolution16,
					>,
				) -> ::subxt::tx::StaticTxPayload<Submit> {
					::subxt::tx::StaticTxPayload::new(
						"ElectionProviderMultiPhase",
						"submit",
						Submit {
							raw_solution: ::std::boxed::Box::new(raw_solution),
						},
						[
							220u8, 167u8, 40u8, 47u8, 253u8, 244u8, 72u8, 124u8, 30u8, 123u8,
							127u8, 227u8, 2u8, 66u8, 119u8, 64u8, 211u8, 200u8, 210u8, 98u8, 248u8,
							132u8, 68u8, 25u8, 34u8, 182u8, 230u8, 225u8, 241u8, 58u8, 193u8,
							134u8,
						],
					)
				}

				#[doc = "See [`Pallet::governance_fallback`]."]
				pub fn governance_fallback(
					&self,
					maybe_max_voters: ::core::option::Option<::core::primitive::u32>,
					maybe_max_targets: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::tx::StaticTxPayload<GovernanceFallback> {
					::subxt::tx::StaticTxPayload::new(
						"ElectionProviderMultiPhase",
						"governance_fallback",
						GovernanceFallback {
							maybe_max_voters,
							maybe_max_targets,
						},
						[
							206u8, 247u8, 76u8, 85u8, 7u8, 24u8, 231u8, 226u8, 192u8, 143u8, 43u8,
							67u8, 91u8, 202u8, 88u8, 176u8, 130u8, 1u8, 83u8, 229u8, 227u8, 200u8,
							179u8, 4u8, 113u8, 60u8, 99u8, 190u8, 53u8, 226u8, 142u8, 182u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_election_provider_multi_phase::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A solution was stored with the given compute."]
			#[doc = ""]
			#[doc = "The `origin` indicates the origin of the solution. If `origin` is `Some(AccountId)`,"]
			#[doc = "the stored solution was submited in the signed phase by a miner with the `AccountId`."]
			#[doc = "Otherwise, the solution was stored either during the unsigned phase or by"]
			#[doc = "`T::ForceOrigin`. The `bool` is `true` when a previous solution was ejected to make"]
			#[doc = "room for this one."]
			pub struct SolutionStored {
				pub compute: runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
				pub origin: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
				pub prev_ejected: ::core::primitive::bool,
			}
			impl ::subxt::events::StaticEvent for SolutionStored {
				const EVENT: &'static str = "SolutionStored";
				const PALLET: &'static str = "ElectionProviderMultiPhase";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The election has been finalized, with the given computation and score."]
			pub struct ElectionFinalized {
				pub compute: runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
				pub score: runtime_types::sp_npos_elections::ElectionScore,
			}
			impl ::subxt::events::StaticEvent for ElectionFinalized {
				const EVENT: &'static str = "ElectionFinalized";
				const PALLET: &'static str = "ElectionProviderMultiPhase";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An election failed."]
			#[doc = ""]
			#[doc = "Not much can be said about which computes failed in the process."]
			pub struct ElectionFailed;
			impl ::subxt::events::StaticEvent for ElectionFailed {
				const EVENT: &'static str = "ElectionFailed";
				const PALLET: &'static str = "ElectionProviderMultiPhase";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An account has been rewarded for their signed submission being finalized."]
			pub struct Rewarded {
				pub account: ::subxt::ext::sp_core::crypto::AccountId32,
				pub value: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rewarded {
				const EVENT: &'static str = "Rewarded";
				const PALLET: &'static str = "ElectionProviderMultiPhase";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An account has been slashed for submitting an invalid signed submission."]
			pub struct Slashed {
				pub account: ::subxt::ext::sp_core::crypto::AccountId32,
				pub value: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Slashed {
				const EVENT: &'static str = "Slashed";
				const PALLET: &'static str = "ElectionProviderMultiPhase";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "There was a phase transition in a given round."]
			pub struct PhaseTransitioned {
				pub from: runtime_types::pallet_election_provider_multi_phase::Phase<
					::core::primitive::u32,
				>,
				pub to: runtime_types::pallet_election_provider_multi_phase::Phase<
					::core::primitive::u32,
				>,
				pub round: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for PhaseTransitioned {
				const EVENT: &'static str = "PhaseTransitioned";
				const PALLET: &'static str = "ElectionProviderMultiPhase";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Internal counter for the number of rounds."]
				#[doc = ""]
				#[doc = " This is useful for de-duplication of transactions submitted to the pool, and general"]
				#[doc = " diagnostics of the pallet."]
				#[doc = ""]
				#[doc = " This is merely incremented once per every time that an upstream `elect` is called."]
				pub fn round(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ElectionProviderMultiPhase",
						"Round",
						vec![],
						[
							16u8, 49u8, 176u8, 52u8, 202u8, 111u8, 120u8, 8u8, 217u8, 96u8, 35u8,
							14u8, 233u8, 130u8, 47u8, 98u8, 34u8, 44u8, 166u8, 188u8, 199u8, 210u8,
							21u8, 19u8, 70u8, 96u8, 139u8, 8u8, 53u8, 82u8, 165u8, 239u8,
						],
					)
				}

				#[doc = " Current phase."]
				pub fn current_phase(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_election_provider_multi_phase::Phase<
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ElectionProviderMultiPhase",
						"CurrentPhase",
						vec![],
						[
							236u8, 101u8, 8u8, 52u8, 68u8, 240u8, 74u8, 159u8, 181u8, 53u8, 153u8,
							101u8, 228u8, 81u8, 96u8, 161u8, 34u8, 67u8, 35u8, 28u8, 121u8, 44u8,
							229u8, 45u8, 196u8, 87u8, 73u8, 125u8, 216u8, 245u8, 255u8, 15u8,
						],
					)
				}

				#[doc = " Current best solution, signed or unsigned, queued to be returned upon `elect`."]
				#[doc = ""]
				#[doc = " Always sorted by score."]
				pub fn queued_solution(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_election_provider_multi_phase::ReadySolution,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ElectionProviderMultiPhase",
						"QueuedSolution",
						vec![],
						[
							11u8, 152u8, 13u8, 167u8, 204u8, 209u8, 171u8, 249u8, 59u8, 250u8,
							58u8, 152u8, 164u8, 121u8, 146u8, 112u8, 241u8, 16u8, 159u8, 251u8,
							209u8, 251u8, 114u8, 29u8, 188u8, 30u8, 84u8, 71u8, 136u8, 173u8,
							145u8, 236u8,
						],
					)
				}

				#[doc = " Snapshot data of the round."]
				#[doc = ""]
				#[doc = " This is created at the beginning of the signed phase and cleared upon calling `elect`."]
				pub fn snapshot(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_election_provider_multi_phase::RoundSnapshot<
							::subxt::ext::sp_core::crypto::AccountId32,
							(
								::subxt::ext::sp_core::crypto::AccountId32,
								::core::primitive::u64,
								runtime_types::bounded_collections::bounded_vec::BoundedVec<
									::subxt::ext::sp_core::crypto::AccountId32,
								>,
							),
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ElectionProviderMultiPhase",
						"Snapshot",
						vec![],
						[
							239u8, 56u8, 191u8, 77u8, 150u8, 224u8, 248u8, 88u8, 132u8, 224u8,
							164u8, 83u8, 253u8, 36u8, 46u8, 156u8, 72u8, 152u8, 36u8, 206u8, 72u8,
							27u8, 226u8, 87u8, 146u8, 220u8, 93u8, 178u8, 26u8, 115u8, 232u8, 71u8,
						],
					)
				}

				#[doc = " Desired number of targets to elect for this round."]
				#[doc = ""]
				#[doc = " Only exists when [`Snapshot`] is present."]
				pub fn desired_targets(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ElectionProviderMultiPhase",
						"DesiredTargets",
						vec![],
						[
							16u8, 247u8, 4u8, 181u8, 93u8, 79u8, 12u8, 212u8, 146u8, 167u8, 80u8,
							58u8, 118u8, 52u8, 68u8, 87u8, 90u8, 140u8, 31u8, 210u8, 2u8, 116u8,
							220u8, 231u8, 115u8, 112u8, 118u8, 118u8, 68u8, 34u8, 151u8, 165u8,
						],
					)
				}

				#[doc = " The metadata of the [`RoundSnapshot`]"]
				#[doc = ""]
				#[doc = " Only exists when [`Snapshot`] is present."]
				pub fn snapshot_metadata(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ElectionProviderMultiPhase",
						"SnapshotMetadata",
						vec![],
						[
							135u8, 122u8, 60u8, 75u8, 194u8, 240u8, 187u8, 96u8, 240u8, 203u8,
							192u8, 22u8, 117u8, 148u8, 118u8, 24u8, 240u8, 213u8, 94u8, 22u8,
							194u8, 47u8, 181u8, 245u8, 77u8, 149u8, 11u8, 251u8, 117u8, 220u8,
							205u8, 78u8,
						],
					)
				}

				#[doc = " The next index to be assigned to an incoming signed submission."]
				#[doc = ""]
				#[doc = " Every accepted submission is assigned a unique index; that index is bound to that particular"]
				#[doc = " submission for the duration of the election. On election finalization, the next index is"]
				#[doc = " reset to 0."]
				#[doc = ""]
				#[doc = " We can't just use `SignedSubmissionIndices.len()`, because that's a bounded set; past its"]
				#[doc = " capacity, it will simply saturate. We can't just iterate over `SignedSubmissionsMap`,"]
				#[doc = " because iteration is slow. Instead, we store the value here."]
				pub fn signed_submission_next_index(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ElectionProviderMultiPhase",
						"SignedSubmissionNextIndex",
						vec![],
						[
							242u8, 11u8, 157u8, 105u8, 96u8, 7u8, 31u8, 20u8, 51u8, 141u8, 182u8,
							180u8, 13u8, 172u8, 155u8, 59u8, 42u8, 238u8, 115u8, 8u8, 6u8, 137u8,
							45u8, 2u8, 123u8, 187u8, 53u8, 215u8, 19u8, 129u8, 54u8, 22u8,
						],
					)
				}

				#[doc = " A sorted, bounded vector of `(score, block_number, index)`, where each `index` points to a"]
				#[doc = " value in `SignedSubmissions`."]
				#[doc = ""]
				#[doc = " We never need to process more than a single signed submission at a time. Signed submissions"]
				#[doc = " can be quite large, so we're willing to pay the cost of multiple database accesses to access"]
				#[doc = " them one at a time instead of reading and decoding all of them at once."]
				pub fn signed_submission_indices(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<(
							runtime_types::sp_npos_elections::ElectionScore,
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ElectionProviderMultiPhase",
						"SignedSubmissionIndices",
						vec![],
						[
							228u8, 166u8, 94u8, 248u8, 71u8, 26u8, 125u8, 81u8, 32u8, 22u8, 46u8,
							185u8, 209u8, 123u8, 46u8, 17u8, 152u8, 149u8, 222u8, 125u8, 112u8,
							230u8, 29u8, 177u8, 162u8, 214u8, 66u8, 38u8, 170u8, 121u8, 129u8,
							100u8,
						],
					)
				}

				#[doc = " Unchecked, signed solutions."]
				#[doc = ""]
				#[doc = " Together with `SubmissionIndices`, this stores a bounded set of `SignedSubmissions` while"]
				#[doc = " allowing us to keep only a single one in memory at a time."]
				#[doc = ""]
				#[doc = " Twox note: the key of the map is an auto-incrementing index which users cannot inspect or"]
				#[doc = " affect; we shouldn't need a cryptographically secure hasher."]				pub fn signed_submissions_map (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , runtime_types :: da_runtime :: constants :: staking :: NposSolution16 > > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::StaticStorageAddress::new(
						"ElectionProviderMultiPhase",
						"SignedSubmissionsMap",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							84u8, 65u8, 205u8, 191u8, 143u8, 246u8, 239u8, 27u8, 243u8, 54u8,
							250u8, 8u8, 125u8, 32u8, 241u8, 141u8, 210u8, 225u8, 56u8, 101u8,
							241u8, 52u8, 157u8, 29u8, 13u8, 155u8, 73u8, 132u8, 118u8, 53u8, 2u8,
							135u8,
						],
					)
				}

				#[doc = " Unchecked, signed solutions."]
				#[doc = ""]
				#[doc = " Together with `SubmissionIndices`, this stores a bounded set of `SignedSubmissions` while"]
				#[doc = " allowing us to keep only a single one in memory at a time."]
				#[doc = ""]
				#[doc = " Twox note: the key of the map is an auto-incrementing index which users cannot inspect or"]
				#[doc = " affect; we shouldn't need a cryptographically secure hasher."]				pub fn signed_submissions_map_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , runtime_types :: da_runtime :: constants :: staking :: NposSolution16 > > , () , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::StaticStorageAddress::new(
						"ElectionProviderMultiPhase",
						"SignedSubmissionsMap",
						Vec::new(),
						[
							84u8, 65u8, 205u8, 191u8, 143u8, 246u8, 239u8, 27u8, 243u8, 54u8,
							250u8, 8u8, 125u8, 32u8, 241u8, 141u8, 210u8, 225u8, 56u8, 101u8,
							241u8, 52u8, 157u8, 29u8, 13u8, 155u8, 73u8, 132u8, 118u8, 53u8, 2u8,
							135u8,
						],
					)
				}

				#[doc = " The minimum score that each 'untrusted' solution must attain in order to be considered"]
				#[doc = " feasible."]
				#[doc = ""]
				#[doc = " Can be set via `set_minimum_untrusted_score`."]
				pub fn minimum_untrusted_score(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_npos_elections::ElectionScore,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ElectionProviderMultiPhase",
						"MinimumUntrustedScore",
						vec![],
						[
							77u8, 235u8, 181u8, 45u8, 230u8, 12u8, 0u8, 179u8, 152u8, 38u8, 74u8,
							199u8, 47u8, 84u8, 85u8, 55u8, 171u8, 226u8, 217u8, 125u8, 17u8, 194u8,
							95u8, 157u8, 73u8, 245u8, 75u8, 130u8, 248u8, 7u8, 53u8, 226u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Duration of the unsigned phase."]
				pub fn unsigned_phase(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"UnsignedPhase",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " Duration of the signed phase."]
				pub fn signed_phase(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"SignedPhase",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " The minimum amount of improvement to the solution score that defines a solution as"]
				#[doc = " \"better\" in the Signed phase."]
				pub fn better_signed_threshold(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::per_things::Perbill,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"BetterSignedThreshold",
						[
							225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
							80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
							177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
						],
					)
				}

				#[doc = " The minimum amount of improvement to the solution score that defines a solution as"]
				#[doc = " \"better\" in the Unsigned phase."]
				pub fn better_unsigned_threshold(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::per_things::Perbill,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"BetterUnsignedThreshold",
						[
							225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
							80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
							177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
						],
					)
				}

				#[doc = " The repeat threshold of the offchain worker."]
				#[doc = ""]
				#[doc = " For example, if it is 5, that means that at least 5 blocks will elapse between attempts"]
				#[doc = " to submit the worker's solution."]
				pub fn offchain_repeat(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"OffchainRepeat",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " The priority of the unsigned transaction submitted in the unsigned-phase"]
				pub fn miner_tx_priority(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"MinerTxPriority",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}

				#[doc = " Maximum number of signed submissions that can be queued."]
				#[doc = ""]
				#[doc = " It is best to avoid adjusting this during an election, as it impacts downstream data"]
				#[doc = " structures. In particular, `SignedSubmissionIndices<T>` is bounded on this value. If you"]
				#[doc = " update this value during an election, you _must_ ensure that"]
				#[doc = " `SignedSubmissionIndices.len()` is less than or equal to the new value. Otherwise,"]
				#[doc = " attempts to submit new solutions may cause a runtime panic."]
				pub fn signed_max_submissions(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"SignedMaxSubmissions",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " Maximum weight of a signed solution."]
				#[doc = ""]
				#[doc = " If [`Config::MinerConfig`] is being implemented to submit signed solutions (outside of"]
				#[doc = " this pallet), then [`MinerConfig::solution_weight`] is used to compare against"]
				#[doc = " this value."]
				pub fn signed_max_weight(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_weights::weight_v2::Weight,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"SignedMaxWeight",
						[
							206u8, 61u8, 253u8, 247u8, 163u8, 40u8, 161u8, 52u8, 134u8, 140u8,
							206u8, 83u8, 44u8, 166u8, 226u8, 115u8, 181u8, 14u8, 227u8, 130u8,
							210u8, 32u8, 85u8, 29u8, 230u8, 97u8, 130u8, 165u8, 147u8, 134u8,
							106u8, 76u8,
						],
					)
				}

				#[doc = " The maximum amount of unchecked solutions to refund the call fee for."]
				pub fn signed_max_refunds(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"SignedMaxRefunds",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " Base reward for a signed solution"]
				pub fn signed_reward_base(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"SignedRewardBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}

				#[doc = " Base deposit for a signed solution."]
				pub fn signed_deposit_base(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"SignedDepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}

				#[doc = " Per-byte deposit for a signed solution."]
				pub fn signed_deposit_byte(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"SignedDepositByte",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}

				#[doc = " Per-weight deposit for a signed solution."]
				pub fn signed_deposit_weight(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"SignedDepositWeight",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}

				#[doc = " The maximum number of electing voters to put in the snapshot. At the moment, snapshots"]
				#[doc = " are only over a single block, but once multi-block elections are introduced they will"]
				#[doc = " take place over multiple blocks."]
				pub fn max_electing_voters(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"MaxElectingVoters",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " The maximum number of electable targets to put in the snapshot."]
				pub fn max_electable_targets(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u16>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"MaxElectableTargets",
						[
							116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
							41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
							90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
						],
					)
				}

				#[doc = " The maximum number of winners that can be elected by this `ElectionProvider`"]
				#[doc = " implementation."]
				#[doc = ""]
				#[doc = " Note: This must always be greater or equal to `T::DataProvider::desired_targets()`."]
				pub fn max_winners(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"MaxWinners",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				pub fn miner_max_length(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"MinerMaxLength",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				pub fn miner_max_weight(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_weights::weight_v2::Weight,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"MinerMaxWeight",
						[
							206u8, 61u8, 253u8, 247u8, 163u8, 40u8, 161u8, 52u8, 134u8, 140u8,
							206u8, 83u8, 44u8, 166u8, 226u8, 115u8, 181u8, 14u8, 227u8, 130u8,
							210u8, 32u8, 85u8, 29u8, 230u8, 97u8, 130u8, 165u8, 147u8, 134u8,
							106u8, 76u8,
						],
					)
				}

				pub fn miner_max_votes_per_voter(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"MinerMaxVotesPerVoter",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				pub fn miner_max_winners(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"MinerMaxWinners",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod staking {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Bond {
				#[codec(compact)]
				pub value: ::core::primitive::u128,
				pub payee: runtime_types::pallet_staking::RewardDestination<
					::subxt::ext::sp_core::crypto::AccountId32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct BondExtra {
				#[codec(compact)]
				pub max_additional: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Unbond {
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct WithdrawUnbonded {
				pub num_slashing_spans: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Validate {
				pub prefs: runtime_types::pallet_staking::ValidatorPrefs,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Nominate {
				pub targets: ::std::vec::Vec<
					::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Chill;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetPayee {
				pub payee: runtime_types::pallet_staking::RewardDestination<
					::subxt::ext::sp_core::crypto::AccountId32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetController;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetValidatorCount {
				#[codec(compact)]
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct IncreaseValidatorCount {
				#[codec(compact)]
				pub additional: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ScaleValidatorCount {
				pub factor: runtime_types::sp_arithmetic::per_things::Percent,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ForceNoEras;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ForceNewEra;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetInvulnerables {
				pub invulnerables: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ForceUnstake {
				pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
				pub num_slashing_spans: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ForceNewEraAlways;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct CancelDeferredSlash {
				pub era: ::core::primitive::u32,
				pub slash_indices: ::std::vec::Vec<::core::primitive::u32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct PayoutStakers {
				pub validator_stash: ::subxt::ext::sp_core::crypto::AccountId32,
				pub era: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Rebond {
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ReapStash {
				pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
				pub num_slashing_spans: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Kick {
				pub who: ::std::vec::Vec<
					::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetStakingConfigs {
				pub min_nominator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
					::core::primitive::u128,
				>,
				pub min_validator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
					::core::primitive::u128,
				>,
				pub max_nominator_count:
					runtime_types::pallet_staking::pallet::pallet::ConfigOp<::core::primitive::u32>,
				pub max_validator_count:
					runtime_types::pallet_staking::pallet::pallet::ConfigOp<::core::primitive::u32>,
				pub chill_threshold: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
					runtime_types::sp_arithmetic::per_things::Percent,
				>,
				pub min_commission: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
					runtime_types::sp_arithmetic::per_things::Perbill,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ChillOther {
				pub controller: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ForceApplyMinCommission {
				pub validator_stash: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetMinCommission {
				pub new: runtime_types::sp_arithmetic::per_things::Perbill,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::bond`]."]
				pub fn bond(
					&self,
					value: ::core::primitive::u128,
					payee: runtime_types::pallet_staking::RewardDestination<
						::subxt::ext::sp_core::crypto::AccountId32,
					>,
				) -> ::subxt::tx::StaticTxPayload<Bond> {
					::subxt::tx::StaticTxPayload::new("Staking", "bond", Bond { value, payee }, [
						65u8, 253u8, 138u8, 237u8, 195u8, 40u8, 110u8, 138u8, 5u8, 208u8, 1u8,
						33u8, 85u8, 51u8, 75u8, 224u8, 145u8, 220u8, 97u8, 60u8, 189u8, 60u8, 39u8,
						255u8, 1u8, 54u8, 124u8, 49u8, 183u8, 97u8, 120u8, 223u8,
					])
				}

				#[doc = "See [`Pallet::bond_extra`]."]
				pub fn bond_extra(
					&self,
					max_additional: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<BondExtra> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"bond_extra",
						BondExtra { max_additional },
						[
							60u8, 45u8, 82u8, 223u8, 113u8, 95u8, 0u8, 71u8, 59u8, 108u8, 228u8,
							9u8, 95u8, 210u8, 113u8, 106u8, 252u8, 15u8, 19u8, 128u8, 11u8, 187u8,
							4u8, 151u8, 103u8, 143u8, 24u8, 33u8, 149u8, 82u8, 35u8, 192u8,
						],
					)
				}

				#[doc = "See [`Pallet::unbond`]."]
				pub fn unbond(
					&self,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Unbond> {
					::subxt::tx::StaticTxPayload::new("Staking", "unbond", Unbond { value }, [
						85u8, 62u8, 34u8, 127u8, 60u8, 241u8, 134u8, 60u8, 125u8, 91u8, 31u8,
						193u8, 50u8, 230u8, 237u8, 42u8, 114u8, 230u8, 240u8, 146u8, 14u8, 109u8,
						185u8, 151u8, 148u8, 44u8, 147u8, 182u8, 192u8, 253u8, 51u8, 87u8,
					])
				}

				#[doc = "See [`Pallet::withdraw_unbonded`]."]
				pub fn withdraw_unbonded(
					&self,
					num_slashing_spans: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<WithdrawUnbonded> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"withdraw_unbonded",
						WithdrawUnbonded { num_slashing_spans },
						[
							95u8, 223u8, 122u8, 217u8, 76u8, 208u8, 86u8, 129u8, 31u8, 104u8, 70u8,
							154u8, 23u8, 250u8, 165u8, 192u8, 149u8, 249u8, 158u8, 159u8, 194u8,
							224u8, 118u8, 134u8, 204u8, 157u8, 72u8, 136u8, 19u8, 193u8, 183u8,
							84u8,
						],
					)
				}

				#[doc = "See [`Pallet::validate`]."]
				pub fn validate(
					&self,
					prefs: runtime_types::pallet_staking::ValidatorPrefs,
				) -> ::subxt::tx::StaticTxPayload<Validate> {
					::subxt::tx::StaticTxPayload::new("Staking", "validate", Validate { prefs }, [
						191u8, 116u8, 139u8, 35u8, 250u8, 211u8, 86u8, 240u8, 35u8, 9u8, 19u8,
						44u8, 148u8, 35u8, 91u8, 106u8, 200u8, 172u8, 108u8, 145u8, 194u8, 146u8,
						61u8, 145u8, 233u8, 168u8, 2u8, 26u8, 145u8, 101u8, 114u8, 157u8,
					])
				}

				#[doc = "See [`Pallet::nominate`]."]
				pub fn nominate(
					&self,
					targets: ::std::vec::Vec<
						::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					>,
				) -> ::subxt::tx::StaticTxPayload<Nominate> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"nominate",
						Nominate { targets },
						[
							249u8, 66u8, 140u8, 39u8, 26u8, 221u8, 135u8, 225u8, 98u8, 255u8, 13u8,
							54u8, 106u8, 215u8, 129u8, 156u8, 190u8, 83u8, 178u8, 170u8, 116u8,
							27u8, 8u8, 244u8, 56u8, 73u8, 164u8, 223u8, 199u8, 115u8, 168u8, 83u8,
						],
					)
				}

				#[doc = "See [`Pallet::chill`]."]
				pub fn chill(&self) -> ::subxt::tx::StaticTxPayload<Chill> {
					::subxt::tx::StaticTxPayload::new("Staking", "chill", Chill {}, [
						94u8, 20u8, 196u8, 31u8, 220u8, 125u8, 115u8, 167u8, 140u8, 3u8, 20u8,
						132u8, 81u8, 120u8, 215u8, 166u8, 230u8, 56u8, 16u8, 222u8, 31u8, 153u8,
						120u8, 62u8, 153u8, 67u8, 220u8, 239u8, 11u8, 234u8, 127u8, 122u8,
					])
				}

				#[doc = "See [`Pallet::set_payee`]."]
				pub fn set_payee(
					&self,
					payee: runtime_types::pallet_staking::RewardDestination<
						::subxt::ext::sp_core::crypto::AccountId32,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetPayee> {
					::subxt::tx::StaticTxPayload::new("Staking", "set_payee", SetPayee { payee }, [
						96u8, 8u8, 254u8, 164u8, 87u8, 46u8, 120u8, 11u8, 197u8, 63u8, 20u8, 178u8,
						167u8, 236u8, 149u8, 245u8, 14u8, 171u8, 108u8, 195u8, 250u8, 133u8, 0u8,
						75u8, 192u8, 159u8, 84u8, 220u8, 242u8, 133u8, 60u8, 62u8,
					])
				}

				#[doc = "See [`Pallet::set_controller`]."]
				pub fn set_controller(&self) -> ::subxt::tx::StaticTxPayload<SetController> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"set_controller",
						SetController {},
						[
							67u8, 49u8, 25u8, 140u8, 36u8, 142u8, 80u8, 124u8, 8u8, 122u8, 70u8,
							63u8, 196u8, 131u8, 182u8, 76u8, 208u8, 148u8, 205u8, 43u8, 108u8,
							41u8, 212u8, 250u8, 83u8, 104u8, 82u8, 163u8, 211u8, 82u8, 96u8, 170u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_validator_count`]."]
				pub fn set_validator_count(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetValidatorCount> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"set_validator_count",
						SetValidatorCount { new },
						[
							55u8, 232u8, 95u8, 66u8, 228u8, 217u8, 11u8, 27u8, 3u8, 202u8, 199u8,
							242u8, 70u8, 160u8, 250u8, 187u8, 194u8, 91u8, 15u8, 36u8, 215u8, 36u8,
							160u8, 108u8, 251u8, 60u8, 240u8, 202u8, 249u8, 235u8, 28u8, 94u8,
						],
					)
				}

				#[doc = "See [`Pallet::increase_validator_count`]."]
				pub fn increase_validator_count(
					&self,
					additional: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<IncreaseValidatorCount> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"increase_validator_count",
						IncreaseValidatorCount { additional },
						[
							239u8, 184u8, 155u8, 213u8, 25u8, 22u8, 193u8, 13u8, 102u8, 192u8,
							82u8, 153u8, 249u8, 192u8, 60u8, 158u8, 8u8, 78u8, 175u8, 219u8, 46u8,
							51u8, 222u8, 193u8, 193u8, 201u8, 78u8, 90u8, 58u8, 86u8, 196u8, 17u8,
						],
					)
				}

				#[doc = "See [`Pallet::scale_validator_count`]."]
				pub fn scale_validator_count(
					&self,
					factor: runtime_types::sp_arithmetic::per_things::Percent,
				) -> ::subxt::tx::StaticTxPayload<ScaleValidatorCount> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"scale_validator_count",
						ScaleValidatorCount { factor },
						[
							198u8, 68u8, 227u8, 94u8, 110u8, 157u8, 209u8, 217u8, 112u8, 37u8,
							78u8, 142u8, 12u8, 193u8, 219u8, 167u8, 149u8, 112u8, 49u8, 139u8,
							74u8, 81u8, 172u8, 72u8, 253u8, 224u8, 56u8, 194u8, 185u8, 90u8, 87u8,
							125u8,
						],
					)
				}

				#[doc = "See [`Pallet::force_no_eras`]."]
				pub fn force_no_eras(&self) -> ::subxt::tx::StaticTxPayload<ForceNoEras> {
					::subxt::tx::StaticTxPayload::new("Staking", "force_no_eras", ForceNoEras {}, [
						16u8, 81u8, 207u8, 168u8, 23u8, 236u8, 11u8, 75u8, 141u8, 107u8, 92u8, 2u8,
						53u8, 111u8, 252u8, 116u8, 91u8, 120u8, 75u8, 24u8, 125u8, 53u8, 9u8, 28u8,
						242u8, 87u8, 245u8, 55u8, 40u8, 103u8, 151u8, 178u8,
					])
				}

				#[doc = "See [`Pallet::force_new_era`]."]
				pub fn force_new_era(&self) -> ::subxt::tx::StaticTxPayload<ForceNewEra> {
					::subxt::tx::StaticTxPayload::new("Staking", "force_new_era", ForceNewEra {}, [
						230u8, 242u8, 169u8, 196u8, 78u8, 145u8, 24u8, 191u8, 113u8, 68u8, 5u8,
						138u8, 48u8, 51u8, 109u8, 126u8, 73u8, 136u8, 162u8, 158u8, 174u8, 201u8,
						213u8, 230u8, 215u8, 44u8, 200u8, 32u8, 75u8, 27u8, 23u8, 254u8,
					])
				}

				#[doc = "See [`Pallet::set_invulnerables`]."]
				pub fn set_invulnerables(
					&self,
					invulnerables: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::tx::StaticTxPayload<SetInvulnerables> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"set_invulnerables",
						SetInvulnerables { invulnerables },
						[
							2u8, 148u8, 221u8, 111u8, 153u8, 48u8, 222u8, 36u8, 228u8, 84u8, 18u8,
							35u8, 168u8, 239u8, 53u8, 245u8, 27u8, 76u8, 18u8, 203u8, 206u8, 9u8,
							8u8, 81u8, 35u8, 224u8, 22u8, 133u8, 58u8, 99u8, 103u8, 39u8,
						],
					)
				}

				#[doc = "See [`Pallet::force_unstake`]."]
				pub fn force_unstake(
					&self,
					stash: ::subxt::ext::sp_core::crypto::AccountId32,
					num_slashing_spans: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ForceUnstake> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"force_unstake",
						ForceUnstake {
							stash,
							num_slashing_spans,
						},
						[
							94u8, 247u8, 238u8, 47u8, 250u8, 6u8, 96u8, 175u8, 173u8, 123u8, 161u8,
							187u8, 162u8, 214u8, 176u8, 233u8, 33u8, 33u8, 167u8, 239u8, 40u8,
							223u8, 19u8, 131u8, 230u8, 39u8, 175u8, 200u8, 36u8, 182u8, 76u8,
							207u8,
						],
					)
				}

				#[doc = "See [`Pallet::force_new_era_always`]."]
				pub fn force_new_era_always(
					&self,
				) -> ::subxt::tx::StaticTxPayload<ForceNewEraAlways> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"force_new_era_always",
						ForceNewEraAlways {},
						[
							179u8, 118u8, 189u8, 54u8, 248u8, 141u8, 207u8, 142u8, 80u8, 37u8,
							241u8, 185u8, 138u8, 254u8, 117u8, 147u8, 225u8, 118u8, 34u8, 177u8,
							197u8, 158u8, 8u8, 82u8, 202u8, 108u8, 208u8, 26u8, 64u8, 33u8, 74u8,
							43u8,
						],
					)
				}

				#[doc = "See [`Pallet::cancel_deferred_slash`]."]
				pub fn cancel_deferred_slash(
					&self,
					era: ::core::primitive::u32,
					slash_indices: ::std::vec::Vec<::core::primitive::u32>,
				) -> ::subxt::tx::StaticTxPayload<CancelDeferredSlash> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"cancel_deferred_slash",
						CancelDeferredSlash { era, slash_indices },
						[
							120u8, 57u8, 162u8, 105u8, 91u8, 250u8, 129u8, 240u8, 110u8, 234u8,
							170u8, 98u8, 164u8, 65u8, 106u8, 101u8, 19u8, 88u8, 146u8, 210u8,
							171u8, 44u8, 37u8, 50u8, 65u8, 178u8, 37u8, 223u8, 239u8, 197u8, 116u8,
							168u8,
						],
					)
				}

				#[doc = "See [`Pallet::payout_stakers`]."]
				pub fn payout_stakers(
					&self,
					validator_stash: ::subxt::ext::sp_core::crypto::AccountId32,
					era: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<PayoutStakers> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"payout_stakers",
						PayoutStakers {
							validator_stash,
							era,
						},
						[
							184u8, 194u8, 33u8, 118u8, 7u8, 203u8, 89u8, 119u8, 214u8, 76u8, 178u8,
							20u8, 82u8, 111u8, 57u8, 132u8, 212u8, 43u8, 232u8, 91u8, 252u8, 49u8,
							42u8, 115u8, 1u8, 181u8, 154u8, 207u8, 144u8, 206u8, 205u8, 33u8,
						],
					)
				}

				#[doc = "See [`Pallet::rebond`]."]
				pub fn rebond(
					&self,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Rebond> {
					::subxt::tx::StaticTxPayload::new("Staking", "rebond", Rebond { value }, [
						25u8, 22u8, 191u8, 172u8, 133u8, 101u8, 139u8, 102u8, 134u8, 16u8, 136u8,
						56u8, 137u8, 162u8, 4u8, 253u8, 196u8, 30u8, 234u8, 49u8, 102u8, 68u8,
						145u8, 96u8, 148u8, 219u8, 162u8, 17u8, 177u8, 184u8, 34u8, 113u8,
					])
				}

				#[doc = "See [`Pallet::reap_stash`]."]
				pub fn reap_stash(
					&self,
					stash: ::subxt::ext::sp_core::crypto::AccountId32,
					num_slashing_spans: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ReapStash> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"reap_stash",
						ReapStash {
							stash,
							num_slashing_spans,
						},
						[
							34u8, 168u8, 120u8, 161u8, 95u8, 199u8, 106u8, 233u8, 61u8, 240u8,
							166u8, 31u8, 183u8, 165u8, 158u8, 179u8, 32u8, 130u8, 27u8, 164u8,
							112u8, 44u8, 14u8, 125u8, 227u8, 87u8, 70u8, 203u8, 194u8, 24u8, 212u8,
							177u8,
						],
					)
				}

				#[doc = "See [`Pallet::kick`]."]
				pub fn kick(
					&self,
					who: ::std::vec::Vec<
						::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					>,
				) -> ::subxt::tx::StaticTxPayload<Kick> {
					::subxt::tx::StaticTxPayload::new("Staking", "kick", Kick { who }, [
						94u8, 27u8, 18u8, 16u8, 126u8, 129u8, 47u8, 169u8, 114u8, 84u8, 48u8, 95u8,
						235u8, 237u8, 33u8, 118u8, 115u8, 243u8, 166u8, 120u8, 121u8, 70u8, 227u8,
						240u8, 205u8, 240u8, 211u8, 202u8, 251u8, 232u8, 209u8, 12u8,
					])
				}

				#[doc = "See [`Pallet::set_staking_configs`]."]
				pub fn set_staking_configs(
					&self,
					min_nominator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
						::core::primitive::u128,
					>,
					min_validator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
						::core::primitive::u128,
					>,
					max_nominator_count: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
						::core::primitive::u32,
					>,
					max_validator_count: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
						::core::primitive::u32,
					>,
					chill_threshold: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
						runtime_types::sp_arithmetic::per_things::Percent,
					>,
					min_commission: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
						runtime_types::sp_arithmetic::per_things::Perbill,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetStakingConfigs> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"set_staking_configs",
						SetStakingConfigs {
							min_nominator_bond,
							min_validator_bond,
							max_nominator_count,
							max_validator_count,
							chill_threshold,
							min_commission,
						},
						[
							176u8, 168u8, 155u8, 176u8, 27u8, 79u8, 223u8, 92u8, 88u8, 93u8, 223u8,
							69u8, 179u8, 250u8, 138u8, 138u8, 87u8, 220u8, 36u8, 3u8, 126u8, 213u8,
							16u8, 68u8, 3u8, 16u8, 218u8, 151u8, 98u8, 169u8, 217u8, 75u8,
						],
					)
				}

				#[doc = "See [`Pallet::chill_other`]."]
				pub fn chill_other(
					&self,
					controller: ::subxt::ext::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<ChillOther> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"chill_other",
						ChillOther { controller },
						[
							140u8, 98u8, 4u8, 203u8, 91u8, 131u8, 123u8, 119u8, 169u8, 47u8, 188u8,
							23u8, 205u8, 170u8, 82u8, 220u8, 166u8, 170u8, 135u8, 176u8, 68u8,
							228u8, 14u8, 67u8, 42u8, 52u8, 140u8, 231u8, 62u8, 167u8, 80u8, 173u8,
						],
					)
				}

				#[doc = "See [`Pallet::force_apply_min_commission`]."]
				pub fn force_apply_min_commission(
					&self,
					validator_stash: ::subxt::ext::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<ForceApplyMinCommission> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"force_apply_min_commission",
						ForceApplyMinCommission { validator_stash },
						[
							136u8, 163u8, 85u8, 134u8, 240u8, 247u8, 183u8, 227u8, 226u8, 202u8,
							102u8, 186u8, 138u8, 119u8, 78u8, 123u8, 229u8, 135u8, 129u8, 241u8,
							119u8, 106u8, 41u8, 182u8, 121u8, 181u8, 242u8, 175u8, 74u8, 207u8,
							64u8, 106u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_min_commission`]."]
				pub fn set_min_commission(
					&self,
					new: runtime_types::sp_arithmetic::per_things::Perbill,
				) -> ::subxt::tx::StaticTxPayload<SetMinCommission> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"set_min_commission",
						SetMinCommission { new },
						[
							62u8, 139u8, 175u8, 245u8, 212u8, 113u8, 117u8, 130u8, 191u8, 173u8,
							78u8, 97u8, 19u8, 104u8, 185u8, 207u8, 201u8, 14u8, 200u8, 208u8,
							184u8, 195u8, 242u8, 175u8, 158u8, 156u8, 51u8, 58u8, 118u8, 154u8,
							68u8, 221u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_staking::pallet::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The era payout has been set; the first balance is the validator-payout; the second is"]
			#[doc = "the remainder from the maximum amount of reward."]
			pub struct EraPaid {
				pub era_index: ::core::primitive::u32,
				pub validator_payout: ::core::primitive::u128,
				pub remainder: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for EraPaid {
				const EVENT: &'static str = "EraPaid";
				const PALLET: &'static str = "Staking";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The nominator has been rewarded by this amount."]
			pub struct Rewarded {
				pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rewarded {
				const EVENT: &'static str = "Rewarded";
				const PALLET: &'static str = "Staking";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A staker (validator or nominator) has been slashed by the given amount."]
			pub struct Slashed {
				pub staker: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Slashed {
				const EVENT: &'static str = "Slashed";
				const PALLET: &'static str = "Staking";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A slash for the given validator, for the given percentage of their stake, at the given"]
			#[doc = "era as been reported."]
			pub struct SlashReported {
				pub validator: ::subxt::ext::sp_core::crypto::AccountId32,
				pub fraction: runtime_types::sp_arithmetic::per_things::Perbill,
				pub slash_era: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for SlashReported {
				const EVENT: &'static str = "SlashReported";
				const PALLET: &'static str = "Staking";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An old slashing report from a prior era was discarded because it could"]
			#[doc = "not be processed."]
			pub struct OldSlashingReportDiscarded {
				pub session_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for OldSlashingReportDiscarded {
				const EVENT: &'static str = "OldSlashingReportDiscarded";
				const PALLET: &'static str = "Staking";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A new set of stakers was elected."]
			pub struct StakersElected;
			impl ::subxt::events::StaticEvent for StakersElected {
				const EVENT: &'static str = "StakersElected";
				const PALLET: &'static str = "Staking";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An account has bonded this amount. \\[stash, amount\\]"]
			#[doc = ""]
			#[doc = "NOTE: This event is only emitted when funds are bonded via a dispatchable. Notably,"]
			#[doc = "it will not be emitted for staking rewards when they are added to stake."]
			pub struct Bonded {
				pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Bonded {
				const EVENT: &'static str = "Bonded";
				const PALLET: &'static str = "Staking";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An account has unbonded this amount."]
			pub struct Unbonded {
				pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unbonded {
				const EVENT: &'static str = "Unbonded";
				const PALLET: &'static str = "Staking";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An account has called `withdraw_unbonded` and removed unbonding chunks worth `Balance`"]
			#[doc = "from the unlocking queue."]
			pub struct Withdrawn {
				pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdrawn {
				const EVENT: &'static str = "Withdrawn";
				const PALLET: &'static str = "Staking";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A nominator has been kicked from a validator."]
			pub struct Kicked {
				pub nominator: ::subxt::ext::sp_core::crypto::AccountId32,
				pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Kicked {
				const EVENT: &'static str = "Kicked";
				const PALLET: &'static str = "Staking";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The election failed. No new era is planned."]
			pub struct StakingElectionFailed;
			impl ::subxt::events::StaticEvent for StakingElectionFailed {
				const EVENT: &'static str = "StakingElectionFailed";
				const PALLET: &'static str = "Staking";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An account has stopped participating as either a validator or nominator."]
			pub struct Chilled {
				pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Chilled {
				const EVENT: &'static str = "Chilled";
				const PALLET: &'static str = "Staking";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The stakers' rewards are getting paid."]
			pub struct PayoutStarted {
				pub era_index: ::core::primitive::u32,
				pub validator_stash: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for PayoutStarted {
				const EVENT: &'static str = "PayoutStarted";
				const PALLET: &'static str = "Staking";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A validator has set their preferences."]
			pub struct ValidatorPrefsSet {
				pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
				pub prefs: runtime_types::pallet_staking::ValidatorPrefs,
			}
			impl ::subxt::events::StaticEvent for ValidatorPrefsSet {
				const EVENT: &'static str = "ValidatorPrefsSet";
				const PALLET: &'static str = "Staking";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A new force era mode was set."]
			pub struct ForceEra {
				pub mode: runtime_types::pallet_staking::Forcing,
			}
			impl ::subxt::events::StaticEvent for ForceEra {
				const EVENT: &'static str = "ForceEra";
				const PALLET: &'static str = "Staking";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The ideal number of active validators."]
				pub fn validator_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ValidatorCount",
						vec![],
						[
							245u8, 75u8, 214u8, 110u8, 66u8, 164u8, 86u8, 206u8, 69u8, 89u8, 12u8,
							111u8, 117u8, 16u8, 228u8, 184u8, 207u8, 6u8, 0u8, 126u8, 221u8, 67u8,
							125u8, 218u8, 188u8, 245u8, 156u8, 188u8, 34u8, 85u8, 208u8, 197u8,
						],
					)
				}

				#[doc = " Minimum number of staking participants before emergency conditions are imposed."]
				pub fn minimum_validator_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"MinimumValidatorCount",
						vec![],
						[
							82u8, 95u8, 128u8, 55u8, 136u8, 134u8, 71u8, 117u8, 135u8, 76u8, 44u8,
							46u8, 174u8, 34u8, 170u8, 228u8, 175u8, 1u8, 234u8, 162u8, 91u8, 252u8,
							127u8, 68u8, 243u8, 241u8, 13u8, 107u8, 214u8, 70u8, 87u8, 249u8,
						],
					)
				}

				#[doc = " Any validators that may never be slashed or forcibly kicked. It's a Vec since they're"]
				#[doc = " easy to initialize and the performance hit is minimal (we expect no more than four"]
				#[doc = " invulnerables) and restricted to testnets."]
				pub fn invulnerables(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"Invulnerables",
						vec![],
						[
							77u8, 78u8, 63u8, 199u8, 150u8, 167u8, 135u8, 130u8, 192u8, 51u8,
							202u8, 119u8, 68u8, 49u8, 241u8, 68u8, 82u8, 90u8, 226u8, 201u8, 96u8,
							170u8, 21u8, 173u8, 236u8, 116u8, 148u8, 8u8, 174u8, 92u8, 7u8, 11u8,
						],
					)
				}

				#[doc = " Map from all locked \"stash\" accounts to the controller account."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn bonded(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"Bonded",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							35u8, 197u8, 156u8, 60u8, 22u8, 59u8, 103u8, 83u8, 77u8, 15u8, 118u8,
							193u8, 155u8, 97u8, 229u8, 36u8, 119u8, 128u8, 224u8, 162u8, 21u8,
							46u8, 199u8, 221u8, 15u8, 74u8, 59u8, 70u8, 77u8, 218u8, 73u8, 165u8,
						],
					)
				}

				#[doc = " Map from all locked \"stash\" accounts to the controller account."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn bonded_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"Bonded",
						Vec::new(),
						[
							35u8, 197u8, 156u8, 60u8, 22u8, 59u8, 103u8, 83u8, 77u8, 15u8, 118u8,
							193u8, 155u8, 97u8, 229u8, 36u8, 119u8, 128u8, 224u8, 162u8, 21u8,
							46u8, 199u8, 221u8, 15u8, 74u8, 59u8, 70u8, 77u8, 218u8, 73u8, 165u8,
						],
					)
				}

				#[doc = " The minimum active bond to become and maintain the role of a nominator."]
				pub fn min_nominator_bond(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"MinNominatorBond",
						vec![],
						[
							187u8, 66u8, 149u8, 226u8, 72u8, 219u8, 57u8, 246u8, 102u8, 47u8, 71u8,
							12u8, 219u8, 204u8, 127u8, 223u8, 58u8, 134u8, 81u8, 165u8, 200u8,
							142u8, 196u8, 158u8, 26u8, 38u8, 165u8, 19u8, 91u8, 251u8, 119u8, 84u8,
						],
					)
				}

				#[doc = " The minimum active bond to become and maintain the role of a validator."]
				pub fn min_validator_bond(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"MinValidatorBond",
						vec![],
						[
							48u8, 105u8, 85u8, 178u8, 142u8, 208u8, 208u8, 19u8, 236u8, 130u8,
							129u8, 169u8, 35u8, 245u8, 66u8, 182u8, 92u8, 20u8, 22u8, 109u8, 155u8,
							174u8, 87u8, 118u8, 242u8, 216u8, 193u8, 154u8, 4u8, 5u8, 66u8, 56u8,
						],
					)
				}

				#[doc = " The minimum active nominator stake of the last successful election."]
				pub fn minimum_active_stake(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"MinimumActiveStake",
						vec![],
						[
							172u8, 190u8, 228u8, 47u8, 47u8, 192u8, 182u8, 59u8, 9u8, 18u8, 103u8,
							46u8, 175u8, 54u8, 17u8, 79u8, 89u8, 107u8, 255u8, 200u8, 182u8, 107u8,
							89u8, 157u8, 55u8, 16u8, 77u8, 46u8, 154u8, 169u8, 103u8, 151u8,
						],
					)
				}

				#[doc = " The minimum amount of commission that validators can set."]
				#[doc = ""]
				#[doc = " If set to `0`, no limit exists."]
				pub fn min_commission(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::per_things::Perbill,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"MinCommission",
						vec![],
						[
							61u8, 101u8, 69u8, 27u8, 220u8, 179u8, 5u8, 71u8, 66u8, 227u8, 84u8,
							98u8, 18u8, 141u8, 183u8, 49u8, 98u8, 46u8, 123u8, 114u8, 198u8, 85u8,
							15u8, 175u8, 243u8, 239u8, 133u8, 129u8, 146u8, 174u8, 254u8, 158u8,
						],
					)
				}

				#[doc = " Map from all (unlocked) \"controller\" accounts to the info regarding the staking."]
				pub fn ledger(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::StakingLedger,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"Ledger",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							31u8, 205u8, 3u8, 165u8, 22u8, 22u8, 62u8, 92u8, 33u8, 189u8, 124u8,
							120u8, 177u8, 70u8, 27u8, 242u8, 188u8, 184u8, 204u8, 188u8, 242u8,
							140u8, 128u8, 230u8, 85u8, 99u8, 181u8, 173u8, 67u8, 252u8, 37u8,
							236u8,
						],
					)
				}

				#[doc = " Map from all (unlocked) \"controller\" accounts to the info regarding the staking."]
				pub fn ledger_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::StakingLedger,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"Ledger",
						Vec::new(),
						[
							31u8, 205u8, 3u8, 165u8, 22u8, 22u8, 62u8, 92u8, 33u8, 189u8, 124u8,
							120u8, 177u8, 70u8, 27u8, 242u8, 188u8, 184u8, 204u8, 188u8, 242u8,
							140u8, 128u8, 230u8, 85u8, 99u8, 181u8, 173u8, 67u8, 252u8, 37u8,
							236u8,
						],
					)
				}

				#[doc = " Where the reward payment should be made. Keyed by stash."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn payee(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::RewardDestination<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"Payee",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							195u8, 125u8, 82u8, 213u8, 216u8, 64u8, 76u8, 63u8, 187u8, 163u8, 20u8,
							230u8, 153u8, 13u8, 189u8, 232u8, 119u8, 118u8, 107u8, 17u8, 102u8,
							245u8, 36u8, 42u8, 232u8, 137u8, 177u8, 165u8, 169u8, 246u8, 199u8,
							57u8,
						],
					)
				}

				#[doc = " Where the reward payment should be made. Keyed by stash."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn payee_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::RewardDestination<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"Payee",
						Vec::new(),
						[
							195u8, 125u8, 82u8, 213u8, 216u8, 64u8, 76u8, 63u8, 187u8, 163u8, 20u8,
							230u8, 153u8, 13u8, 189u8, 232u8, 119u8, 118u8, 107u8, 17u8, 102u8,
							245u8, 36u8, 42u8, 232u8, 137u8, 177u8, 165u8, 169u8, 246u8, 199u8,
							57u8,
						],
					)
				}

				#[doc = " The map from (wannabe) validator stash key to the preferences of that validator."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn validators(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::ValidatorPrefs,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"Validators",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							80u8, 77u8, 66u8, 18u8, 197u8, 250u8, 41u8, 185u8, 43u8, 24u8, 149u8,
							164u8, 208u8, 60u8, 144u8, 29u8, 251u8, 195u8, 236u8, 196u8, 108u8,
							58u8, 80u8, 115u8, 246u8, 66u8, 226u8, 241u8, 201u8, 172u8, 229u8,
							152u8,
						],
					)
				}

				#[doc = " The map from (wannabe) validator stash key to the preferences of that validator."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn validators_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::ValidatorPrefs,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"Validators",
						Vec::new(),
						[
							80u8, 77u8, 66u8, 18u8, 197u8, 250u8, 41u8, 185u8, 43u8, 24u8, 149u8,
							164u8, 208u8, 60u8, 144u8, 29u8, 251u8, 195u8, 236u8, 196u8, 108u8,
							58u8, 80u8, 115u8, 246u8, 66u8, 226u8, 241u8, 201u8, 172u8, 229u8,
							152u8,
						],
					)
				}

				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_validators(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"CounterForValidators",
						vec![],
						[
							139u8, 25u8, 223u8, 6u8, 160u8, 239u8, 212u8, 85u8, 36u8, 185u8, 69u8,
							63u8, 21u8, 156u8, 144u8, 241u8, 112u8, 85u8, 49u8, 78u8, 88u8, 11u8,
							8u8, 48u8, 118u8, 34u8, 62u8, 159u8, 239u8, 122u8, 90u8, 45u8,
						],
					)
				}

				#[doc = " The maximum validator count before we stop allowing new validators to join."]
				#[doc = ""]
				#[doc = " When this value is not set, no limits are enforced."]
				pub fn max_validators_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"MaxValidatorsCount",
						vec![],
						[
							250u8, 62u8, 16u8, 68u8, 192u8, 216u8, 236u8, 211u8, 217u8, 9u8, 213u8,
							49u8, 41u8, 37u8, 58u8, 62u8, 131u8, 112u8, 64u8, 26u8, 133u8, 7u8,
							130u8, 1u8, 71u8, 158u8, 14u8, 55u8, 169u8, 239u8, 223u8, 245u8,
						],
					)
				}

				#[doc = " The map from nominator stash key to their nomination preferences, namely the validators that"]
				#[doc = " they wish to support."]
				#[doc = ""]
				#[doc = " Note that the keys of this storage map might become non-decodable in case the"]
				#[doc = " [`Config::MaxNominations`] configuration is decreased. In this rare case, these nominators"]
				#[doc = " are still existent in storage, their key is correct and retrievable (i.e. `contains_key`"]
				#[doc = " indicates that they exist), but their value cannot be decoded. Therefore, the non-decodable"]
				#[doc = " nominators will effectively not-exist, until they re-submit their preferences such that it"]
				#[doc = " is within the bounds of the newly set `Config::MaxNominations`."]
				#[doc = ""]
				#[doc = " This implies that `::iter_keys().count()` and `::iter().count()` might return different"]
				#[doc = " values for this map. Moreover, the main `::count()` is aligned with the former, namely the"]
				#[doc = " number of keys that exist."]
				#[doc = ""]
				#[doc = " Lastly, if any of the nominators become non-decodable, they can be chilled immediately via"]
				#[doc = " [`Call::chill_other`] dispatchable by anyone."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn nominators(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::pallet_staking::Nominations>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"Nominators",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							1u8, 154u8, 55u8, 170u8, 215u8, 64u8, 56u8, 83u8, 254u8, 19u8, 152u8,
							85u8, 164u8, 171u8, 206u8, 129u8, 184u8, 45u8, 221u8, 181u8, 229u8,
							133u8, 200u8, 231u8, 16u8, 146u8, 247u8, 21u8, 77u8, 122u8, 165u8,
							134u8,
						],
					)
				}

				#[doc = " The map from nominator stash key to their nomination preferences, namely the validators that"]
				#[doc = " they wish to support."]
				#[doc = ""]
				#[doc = " Note that the keys of this storage map might become non-decodable in case the"]
				#[doc = " [`Config::MaxNominations`] configuration is decreased. In this rare case, these nominators"]
				#[doc = " are still existent in storage, their key is correct and retrievable (i.e. `contains_key`"]
				#[doc = " indicates that they exist), but their value cannot be decoded. Therefore, the non-decodable"]
				#[doc = " nominators will effectively not-exist, until they re-submit their preferences such that it"]
				#[doc = " is within the bounds of the newly set `Config::MaxNominations`."]
				#[doc = ""]
				#[doc = " This implies that `::iter_keys().count()` and `::iter().count()` might return different"]
				#[doc = " values for this map. Moreover, the main `::count()` is aligned with the former, namely the"]
				#[doc = " number of keys that exist."]
				#[doc = ""]
				#[doc = " Lastly, if any of the nominators become non-decodable, they can be chilled immediately via"]
				#[doc = " [`Call::chill_other`] dispatchable by anyone."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn nominators_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::pallet_staking::Nominations>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"Nominators",
						Vec::new(),
						[
							1u8, 154u8, 55u8, 170u8, 215u8, 64u8, 56u8, 83u8, 254u8, 19u8, 152u8,
							85u8, 164u8, 171u8, 206u8, 129u8, 184u8, 45u8, 221u8, 181u8, 229u8,
							133u8, 200u8, 231u8, 16u8, 146u8, 247u8, 21u8, 77u8, 122u8, 165u8,
							134u8,
						],
					)
				}

				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_nominators(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"CounterForNominators",
						vec![],
						[
							31u8, 94u8, 130u8, 138u8, 75u8, 8u8, 38u8, 162u8, 181u8, 5u8, 125u8,
							116u8, 9u8, 51u8, 22u8, 234u8, 40u8, 117u8, 215u8, 46u8, 82u8, 117u8,
							225u8, 1u8, 9u8, 208u8, 83u8, 63u8, 39u8, 187u8, 207u8, 191u8,
						],
					)
				}

				#[doc = " The maximum nominator count before we stop allowing new validators to join."]
				#[doc = ""]
				#[doc = " When this value is not set, no limits are enforced."]
				pub fn max_nominators_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"MaxNominatorsCount",
						vec![],
						[
							180u8, 190u8, 180u8, 66u8, 235u8, 173u8, 76u8, 160u8, 197u8, 92u8,
							96u8, 165u8, 220u8, 188u8, 32u8, 119u8, 3u8, 73u8, 86u8, 49u8, 104u8,
							17u8, 186u8, 98u8, 221u8, 175u8, 109u8, 254u8, 207u8, 245u8, 125u8,
							179u8,
						],
					)
				}

				#[doc = " The current era index."]
				#[doc = ""]
				#[doc = " This is the latest planned era, depending on how the Session pallet queues the validator"]
				#[doc = " set, it might be active or not."]
				pub fn current_era(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"CurrentEra",
						vec![],
						[
							105u8, 150u8, 49u8, 122u8, 4u8, 78u8, 8u8, 121u8, 34u8, 136u8, 157u8,
							227u8, 59u8, 139u8, 7u8, 253u8, 7u8, 10u8, 117u8, 71u8, 240u8, 74u8,
							86u8, 36u8, 198u8, 37u8, 153u8, 93u8, 196u8, 22u8, 192u8, 243u8,
						],
					)
				}

				#[doc = " The active era information, it holds index and start."]
				#[doc = ""]
				#[doc = " The active era is the era being currently rewarded. Validator set of this era must be"]
				#[doc = " equal to [`SessionInterface::validators`]."]
				pub fn active_era(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::ActiveEraInfo,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ActiveEra",
						vec![],
						[
							15u8, 112u8, 251u8, 183u8, 108u8, 61u8, 28u8, 71u8, 44u8, 150u8, 162u8,
							4u8, 143u8, 121u8, 11u8, 37u8, 83u8, 29u8, 193u8, 21u8, 210u8, 116u8,
							190u8, 236u8, 213u8, 235u8, 49u8, 97u8, 189u8, 142u8, 251u8, 124u8,
						],
					)
				}

				#[doc = " The session index at which the era start for the last `HISTORY_DEPTH` eras."]
				#[doc = ""]
				#[doc = " Note: This tracks the starting session (i.e. session index when era start being active)"]
				#[doc = " for the eras in `[CurrentEra - HISTORY_DEPTH, CurrentEra]`."]
				pub fn eras_start_session_index(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ErasStartSessionIndex",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							92u8, 157u8, 168u8, 144u8, 132u8, 3u8, 212u8, 80u8, 230u8, 229u8,
							251u8, 218u8, 97u8, 55u8, 79u8, 100u8, 163u8, 91u8, 32u8, 246u8, 122u8,
							78u8, 149u8, 214u8, 103u8, 249u8, 119u8, 20u8, 101u8, 116u8, 110u8,
							185u8,
						],
					)
				}

				#[doc = " The session index at which the era start for the last `HISTORY_DEPTH` eras."]
				#[doc = ""]
				#[doc = " Note: This tracks the starting session (i.e. session index when era start being active)"]
				#[doc = " for the eras in `[CurrentEra - HISTORY_DEPTH, CurrentEra]`."]
				pub fn eras_start_session_index_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ErasStartSessionIndex",
						Vec::new(),
						[
							92u8, 157u8, 168u8, 144u8, 132u8, 3u8, 212u8, 80u8, 230u8, 229u8,
							251u8, 218u8, 97u8, 55u8, 79u8, 100u8, 163u8, 91u8, 32u8, 246u8, 122u8,
							78u8, 149u8, 214u8, 103u8, 249u8, 119u8, 20u8, 101u8, 116u8, 110u8,
							185u8,
						],
					)
				}

				#[doc = " Exposure of validator at era."]
				#[doc = ""]
				#[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
				#[doc = ""]
				#[doc = " Is it removed after `HISTORY_DEPTH` eras."]
				#[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
				pub fn eras_stakers(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::Exposure<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ErasStakers",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							192u8, 50u8, 152u8, 151u8, 92u8, 180u8, 206u8, 15u8, 139u8, 210u8,
							128u8, 65u8, 92u8, 253u8, 43u8, 35u8, 139u8, 171u8, 73u8, 185u8, 32u8,
							78u8, 20u8, 197u8, 154u8, 90u8, 233u8, 231u8, 23u8, 22u8, 187u8, 156u8,
						],
					)
				}

				#[doc = " Exposure of validator at era."]
				#[doc = ""]
				#[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
				#[doc = ""]
				#[doc = " Is it removed after `HISTORY_DEPTH` eras."]
				#[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
				pub fn eras_stakers_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::Exposure<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ErasStakers",
						Vec::new(),
						[
							192u8, 50u8, 152u8, 151u8, 92u8, 180u8, 206u8, 15u8, 139u8, 210u8,
							128u8, 65u8, 92u8, 253u8, 43u8, 35u8, 139u8, 171u8, 73u8, 185u8, 32u8,
							78u8, 20u8, 197u8, 154u8, 90u8, 233u8, 231u8, 23u8, 22u8, 187u8, 156u8,
						],
					)
				}

				#[doc = " Clipped Exposure of validator at era."]
				#[doc = ""]
				#[doc = " This is similar to [`ErasStakers`] but number of nominators exposed is reduced to the"]
				#[doc = " `T::MaxNominatorRewardedPerValidator` biggest stakers."]
				#[doc = " (Note: the field `total` and `own` of the exposure remains unchanged)."]
				#[doc = " This is used to limit the i/o cost for the nominator payout."]
				#[doc = ""]
				#[doc = " This is keyed fist by the era index to allow bulk deletion and then the stash account."]
				#[doc = ""]
				#[doc = " Is it removed after `HISTORY_DEPTH` eras."]
				#[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
				pub fn eras_stakers_clipped(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::Exposure<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ErasStakersClipped",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							43u8, 159u8, 113u8, 223u8, 122u8, 169u8, 98u8, 153u8, 26u8, 55u8, 71u8,
							119u8, 174u8, 48u8, 158u8, 45u8, 214u8, 26u8, 136u8, 215u8, 46u8,
							161u8, 185u8, 17u8, 174u8, 204u8, 206u8, 246u8, 49u8, 87u8, 134u8,
							169u8,
						],
					)
				}

				#[doc = " Clipped Exposure of validator at era."]
				#[doc = ""]
				#[doc = " This is similar to [`ErasStakers`] but number of nominators exposed is reduced to the"]
				#[doc = " `T::MaxNominatorRewardedPerValidator` biggest stakers."]
				#[doc = " (Note: the field `total` and `own` of the exposure remains unchanged)."]
				#[doc = " This is used to limit the i/o cost for the nominator payout."]
				#[doc = ""]
				#[doc = " This is keyed fist by the era index to allow bulk deletion and then the stash account."]
				#[doc = ""]
				#[doc = " Is it removed after `HISTORY_DEPTH` eras."]
				#[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
				pub fn eras_stakers_clipped_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::Exposure<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ErasStakersClipped",
						Vec::new(),
						[
							43u8, 159u8, 113u8, 223u8, 122u8, 169u8, 98u8, 153u8, 26u8, 55u8, 71u8,
							119u8, 174u8, 48u8, 158u8, 45u8, 214u8, 26u8, 136u8, 215u8, 46u8,
							161u8, 185u8, 17u8, 174u8, 204u8, 206u8, 246u8, 49u8, 87u8, 134u8,
							169u8,
						],
					)
				}

				#[doc = " Similar to `ErasStakers`, this holds the preferences of validators."]
				#[doc = ""]
				#[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
				#[doc = ""]
				#[doc = " Is it removed after `HISTORY_DEPTH` eras."]
				pub fn eras_validator_prefs(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::ValidatorPrefs,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ErasValidatorPrefs",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							6u8, 196u8, 209u8, 138u8, 252u8, 18u8, 203u8, 86u8, 129u8, 62u8, 4u8,
							56u8, 234u8, 114u8, 141u8, 136u8, 127u8, 224u8, 142u8, 89u8, 150u8,
							33u8, 31u8, 50u8, 140u8, 108u8, 124u8, 77u8, 188u8, 102u8, 230u8,
							174u8,
						],
					)
				}

				#[doc = " Similar to `ErasStakers`, this holds the preferences of validators."]
				#[doc = ""]
				#[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
				#[doc = ""]
				#[doc = " Is it removed after `HISTORY_DEPTH` eras."]
				pub fn eras_validator_prefs_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::ValidatorPrefs,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ErasValidatorPrefs",
						Vec::new(),
						[
							6u8, 196u8, 209u8, 138u8, 252u8, 18u8, 203u8, 86u8, 129u8, 62u8, 4u8,
							56u8, 234u8, 114u8, 141u8, 136u8, 127u8, 224u8, 142u8, 89u8, 150u8,
							33u8, 31u8, 50u8, 140u8, 108u8, 124u8, 77u8, 188u8, 102u8, 230u8,
							174u8,
						],
					)
				}

				#[doc = " The total validator era payout for the last `HISTORY_DEPTH` eras."]
				#[doc = ""]
				#[doc = " Eras that haven't finished yet or has been removed doesn't have reward."]
				pub fn eras_validator_reward(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ErasValidatorReward",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							87u8, 80u8, 156u8, 123u8, 107u8, 77u8, 203u8, 37u8, 231u8, 84u8, 124u8,
							155u8, 227u8, 212u8, 212u8, 179u8, 84u8, 161u8, 223u8, 255u8, 254u8,
							107u8, 52u8, 89u8, 98u8, 169u8, 136u8, 241u8, 104u8, 3u8, 244u8, 161u8,
						],
					)
				}

				#[doc = " The total validator era payout for the last `HISTORY_DEPTH` eras."]
				#[doc = ""]
				#[doc = " Eras that haven't finished yet or has been removed doesn't have reward."]
				pub fn eras_validator_reward_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ErasValidatorReward",
						Vec::new(),
						[
							87u8, 80u8, 156u8, 123u8, 107u8, 77u8, 203u8, 37u8, 231u8, 84u8, 124u8,
							155u8, 227u8, 212u8, 212u8, 179u8, 84u8, 161u8, 223u8, 255u8, 254u8,
							107u8, 52u8, 89u8, 98u8, 169u8, 136u8, 241u8, 104u8, 3u8, 244u8, 161u8,
						],
					)
				}

				#[doc = " Rewards for the last `HISTORY_DEPTH` eras."]
				#[doc = " If reward hasn't been set or has been removed then 0 reward is returned."]
				pub fn eras_reward_points(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::EraRewardPoints<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ErasRewardPoints",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							194u8, 29u8, 20u8, 83u8, 200u8, 47u8, 158u8, 102u8, 88u8, 65u8, 24u8,
							255u8, 120u8, 178u8, 23u8, 232u8, 15u8, 64u8, 206u8, 0u8, 170u8, 40u8,
							18u8, 149u8, 45u8, 90u8, 179u8, 127u8, 52u8, 59u8, 37u8, 192u8,
						],
					)
				}

				#[doc = " Rewards for the last `HISTORY_DEPTH` eras."]
				#[doc = " If reward hasn't been set or has been removed then 0 reward is returned."]
				pub fn eras_reward_points_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::EraRewardPoints<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ErasRewardPoints",
						Vec::new(),
						[
							194u8, 29u8, 20u8, 83u8, 200u8, 47u8, 158u8, 102u8, 88u8, 65u8, 24u8,
							255u8, 120u8, 178u8, 23u8, 232u8, 15u8, 64u8, 206u8, 0u8, 170u8, 40u8,
							18u8, 149u8, 45u8, 90u8, 179u8, 127u8, 52u8, 59u8, 37u8, 192u8,
						],
					)
				}

				#[doc = " The total amount staked for the last `HISTORY_DEPTH` eras."]
				#[doc = " If total hasn't been set or has been removed then 0 stake is returned."]
				pub fn eras_total_stake(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ErasTotalStake",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							224u8, 240u8, 168u8, 69u8, 148u8, 140u8, 249u8, 240u8, 4u8, 46u8, 77u8,
							11u8, 224u8, 65u8, 26u8, 239u8, 1u8, 110u8, 53u8, 11u8, 247u8, 235u8,
							142u8, 234u8, 22u8, 43u8, 24u8, 36u8, 37u8, 43u8, 170u8, 40u8,
						],
					)
				}

				#[doc = " The total amount staked for the last `HISTORY_DEPTH` eras."]
				#[doc = " If total hasn't been set or has been removed then 0 stake is returned."]
				pub fn eras_total_stake_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ErasTotalStake",
						Vec::new(),
						[
							224u8, 240u8, 168u8, 69u8, 148u8, 140u8, 249u8, 240u8, 4u8, 46u8, 77u8,
							11u8, 224u8, 65u8, 26u8, 239u8, 1u8, 110u8, 53u8, 11u8, 247u8, 235u8,
							142u8, 234u8, 22u8, 43u8, 24u8, 36u8, 37u8, 43u8, 170u8, 40u8,
						],
					)
				}

				#[doc = " Mode of era forcing."]
				pub fn force_era(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::pallet_staking::Forcing>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ForceEra",
						vec![],
						[
							221u8, 41u8, 71u8, 21u8, 28u8, 193u8, 65u8, 97u8, 103u8, 37u8, 145u8,
							146u8, 183u8, 194u8, 57u8, 131u8, 214u8, 136u8, 68u8, 156u8, 140u8,
							194u8, 69u8, 151u8, 115u8, 177u8, 92u8, 147u8, 29u8, 40u8, 41u8, 31u8,
						],
					)
				}

				#[doc = " The percentage of the slash that is distributed to reporters."]
				#[doc = ""]
				#[doc = " The rest of the slashed value is handled by the `Slash`."]
				pub fn slash_reward_fraction(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::per_things::Perbill,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"SlashRewardFraction",
						vec![],
						[
							167u8, 79u8, 143u8, 202u8, 199u8, 100u8, 129u8, 162u8, 23u8, 165u8,
							106u8, 170u8, 244u8, 86u8, 144u8, 242u8, 65u8, 207u8, 115u8, 224u8,
							231u8, 155u8, 55u8, 139u8, 101u8, 129u8, 242u8, 196u8, 130u8, 50u8,
							3u8, 117u8,
						],
					)
				}

				#[doc = " The amount of currency given to reporters of a slash event which was"]
				#[doc = " canceled by extraordinary circumstances (e.g. governance)."]
				pub fn canceled_slash_payout(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"CanceledSlashPayout",
						vec![],
						[
							126u8, 218u8, 66u8, 92u8, 82u8, 124u8, 145u8, 161u8, 40u8, 176u8, 14u8,
							211u8, 178u8, 216u8, 8u8, 156u8, 83u8, 14u8, 91u8, 15u8, 200u8, 170u8,
							3u8, 127u8, 141u8, 139u8, 151u8, 98u8, 74u8, 96u8, 238u8, 29u8,
						],
					)
				}

				#[doc = " All unapplied slashes that are queued for later."]
				pub fn unapplied_slashes(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::pallet_staking::UnappliedSlash<
								::subxt::ext::sp_core::crypto::AccountId32,
								::core::primitive::u128,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"UnappliedSlashes",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							130u8, 4u8, 163u8, 163u8, 28u8, 85u8, 34u8, 156u8, 47u8, 125u8, 57u8,
							0u8, 133u8, 176u8, 130u8, 2u8, 175u8, 180u8, 167u8, 203u8, 230u8, 82u8,
							198u8, 183u8, 55u8, 82u8, 221u8, 248u8, 100u8, 173u8, 206u8, 151u8,
						],
					)
				}

				#[doc = " All unapplied slashes that are queued for later."]
				pub fn unapplied_slashes_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::pallet_staking::UnappliedSlash<
								::subxt::ext::sp_core::crypto::AccountId32,
								::core::primitive::u128,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"UnappliedSlashes",
						Vec::new(),
						[
							130u8, 4u8, 163u8, 163u8, 28u8, 85u8, 34u8, 156u8, 47u8, 125u8, 57u8,
							0u8, 133u8, 176u8, 130u8, 2u8, 175u8, 180u8, 167u8, 203u8, 230u8, 82u8,
							198u8, 183u8, 55u8, 82u8, 221u8, 248u8, 100u8, 173u8, 206u8, 151u8,
						],
					)
				}

				#[doc = " A mapping from still-bonded eras to the first session index of that era."]
				#[doc = ""]
				#[doc = " Must contains information for eras for the range:"]
				#[doc = " `[active_era - bounding_duration; active_era]`"]
				pub fn bonded_eras(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"BondedEras",
						vec![],
						[
							243u8, 162u8, 236u8, 198u8, 122u8, 182u8, 37u8, 55u8, 171u8, 156u8,
							235u8, 223u8, 226u8, 129u8, 89u8, 206u8, 2u8, 155u8, 222u8, 154u8,
							116u8, 124u8, 4u8, 119u8, 155u8, 94u8, 248u8, 30u8, 171u8, 51u8, 78u8,
							106u8,
						],
					)
				}

				#[doc = " All slashing events on validators, mapped by era to the highest slash proportion"]
				#[doc = " and slash value of the era."]
				pub fn validator_slash_in_era(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::sp_arithmetic::per_things::Perbill,
						::core::primitive::u128,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ValidatorSlashInEra",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							237u8, 80u8, 3u8, 237u8, 9u8, 40u8, 212u8, 15u8, 251u8, 196u8, 85u8,
							29u8, 27u8, 151u8, 98u8, 122u8, 189u8, 147u8, 205u8, 40u8, 202u8,
							194u8, 158u8, 96u8, 138u8, 16u8, 116u8, 71u8, 140u8, 163u8, 121u8,
							197u8,
						],
					)
				}

				#[doc = " All slashing events on validators, mapped by era to the highest slash proportion"]
				#[doc = " and slash value of the era."]
				pub fn validator_slash_in_era_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::sp_arithmetic::per_things::Perbill,
						::core::primitive::u128,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ValidatorSlashInEra",
						Vec::new(),
						[
							237u8, 80u8, 3u8, 237u8, 9u8, 40u8, 212u8, 15u8, 251u8, 196u8, 85u8,
							29u8, 27u8, 151u8, 98u8, 122u8, 189u8, 147u8, 205u8, 40u8, 202u8,
							194u8, 158u8, 96u8, 138u8, 16u8, 116u8, 71u8, 140u8, 163u8, 121u8,
							197u8,
						],
					)
				}

				#[doc = " All slashing events on nominators, mapped by era to the highest slash value of the era."]
				pub fn nominator_slash_in_era(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"NominatorSlashInEra",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							249u8, 85u8, 170u8, 41u8, 179u8, 194u8, 180u8, 12u8, 53u8, 101u8, 80u8,
							96u8, 166u8, 71u8, 239u8, 23u8, 153u8, 19u8, 152u8, 38u8, 138u8, 136u8,
							221u8, 200u8, 18u8, 165u8, 26u8, 228u8, 195u8, 199u8, 62u8, 4u8,
						],
					)
				}

				#[doc = " All slashing events on nominators, mapped by era to the highest slash value of the era."]
				pub fn nominator_slash_in_era_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"NominatorSlashInEra",
						Vec::new(),
						[
							249u8, 85u8, 170u8, 41u8, 179u8, 194u8, 180u8, 12u8, 53u8, 101u8, 80u8,
							96u8, 166u8, 71u8, 239u8, 23u8, 153u8, 19u8, 152u8, 38u8, 138u8, 136u8,
							221u8, 200u8, 18u8, 165u8, 26u8, 228u8, 195u8, 199u8, 62u8, 4u8,
						],
					)
				}

				#[doc = " Slashing spans for stash accounts."]
				pub fn slashing_spans(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::slashing::SlashingSpans,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"SlashingSpans",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							106u8, 115u8, 118u8, 52u8, 89u8, 77u8, 246u8, 5u8, 255u8, 204u8, 44u8,
							5u8, 66u8, 36u8, 227u8, 252u8, 86u8, 159u8, 186u8, 152u8, 196u8, 21u8,
							74u8, 201u8, 133u8, 93u8, 142u8, 191u8, 20u8, 27u8, 218u8, 157u8,
						],
					)
				}

				#[doc = " Slashing spans for stash accounts."]
				pub fn slashing_spans_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::slashing::SlashingSpans,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"SlashingSpans",
						Vec::new(),
						[
							106u8, 115u8, 118u8, 52u8, 89u8, 77u8, 246u8, 5u8, 255u8, 204u8, 44u8,
							5u8, 66u8, 36u8, 227u8, 252u8, 86u8, 159u8, 186u8, 152u8, 196u8, 21u8,
							74u8, 201u8, 133u8, 93u8, 142u8, 191u8, 20u8, 27u8, 218u8, 157u8,
						],
					)
				}

				#[doc = " Records information about the maximum slash of a stash within a slashing span,"]
				#[doc = " as well as how much reward has been paid out."]
				pub fn span_slash(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::slashing::SpanRecord<
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"SpanSlash",
						vec![::subxt::storage::address::StorageMapKey::new(
							&(_0.borrow(), _1.borrow()),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							160u8, 63u8, 115u8, 190u8, 233u8, 148u8, 75u8, 3u8, 11u8, 59u8, 184u8,
							220u8, 205u8, 64u8, 28u8, 190u8, 116u8, 210u8, 225u8, 230u8, 224u8,
							163u8, 103u8, 157u8, 100u8, 29u8, 86u8, 167u8, 84u8, 217u8, 109u8,
							200u8,
						],
					)
				}

				#[doc = " Records information about the maximum slash of a stash within a slashing span,"]
				#[doc = " as well as how much reward has been paid out."]
				pub fn span_slash_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::slashing::SpanRecord<
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"SpanSlash",
						Vec::new(),
						[
							160u8, 63u8, 115u8, 190u8, 233u8, 148u8, 75u8, 3u8, 11u8, 59u8, 184u8,
							220u8, 205u8, 64u8, 28u8, 190u8, 116u8, 210u8, 225u8, 230u8, 224u8,
							163u8, 103u8, 157u8, 100u8, 29u8, 86u8, 167u8, 84u8, 217u8, 109u8,
							200u8,
						],
					)
				}

				#[doc = " The last planned session scheduled by the session pallet."]
				#[doc = ""]
				#[doc = " This is basically in sync with the call to [`pallet_session::SessionManager::new_session`]."]
				pub fn current_planned_session(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"CurrentPlannedSession",
						vec![],
						[
							38u8, 22u8, 56u8, 250u8, 17u8, 154u8, 99u8, 37u8, 155u8, 253u8, 100u8,
							117u8, 5u8, 239u8, 31u8, 190u8, 53u8, 241u8, 11u8, 185u8, 163u8, 227u8,
							10u8, 77u8, 210u8, 64u8, 156u8, 218u8, 105u8, 16u8, 1u8, 57u8,
						],
					)
				}

				#[doc = " Indices of validators that have offended in the active era and whether they are currently"]
				#[doc = " disabled."]
				#[doc = ""]
				#[doc = " This value should be a superset of disabled validators since not all offences lead to the"]
				#[doc = " validator being disabled (if there was no slash). This is needed to track the percentage of"]
				#[doc = " validators that have offended in the current era, ensuring a new era is forced if"]
				#[doc = " `OffendingValidatorsThreshold` is reached. The vec is always kept sorted so that we can find"]
				#[doc = " whether a given validator has previously offended using binary search. It gets cleared when"]
				#[doc = " the era ends."]
				pub fn offending_validators(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(::core::primitive::u32, ::core::primitive::bool)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"OffendingValidators",
						vec![],
						[
							94u8, 254u8, 0u8, 50u8, 76u8, 232u8, 51u8, 153u8, 118u8, 14u8, 70u8,
							101u8, 112u8, 215u8, 173u8, 82u8, 182u8, 104u8, 167u8, 103u8, 187u8,
							168u8, 86u8, 16u8, 51u8, 235u8, 51u8, 119u8, 38u8, 154u8, 42u8, 113u8,
						],
					)
				}

				#[doc = " The threshold for when users can start calling `chill_other` for other validators /"]
				#[doc = " nominators. The threshold is compared to the actual number of validators / nominators"]
				#[doc = " (`CountFor*`) in the system compared to the configured max (`Max*Count`)."]
				pub fn chill_threshold(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::per_things::Percent,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"ChillThreshold",
						vec![],
						[
							174u8, 165u8, 249u8, 105u8, 24u8, 151u8, 115u8, 166u8, 199u8, 251u8,
							28u8, 5u8, 50u8, 95u8, 144u8, 110u8, 220u8, 76u8, 14u8, 23u8, 179u8,
							41u8, 11u8, 248u8, 28u8, 154u8, 159u8, 255u8, 156u8, 109u8, 98u8, 92u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Maximum number of nominations per nominator."]
				pub fn max_nominations(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Staking", "MaxNominations", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " Number of eras to keep in history."]
				#[doc = ""]
				#[doc = " Following information is kept for eras in `[current_era -"]
				#[doc = " HistoryDepth, current_era]`: `ErasStakers`, `ErasStakersClipped`,"]
				#[doc = " `ErasValidatorPrefs`, `ErasValidatorReward`, `ErasRewardPoints`,"]
				#[doc = " `ErasTotalStake`, `ErasStartSessionIndex`,"]
				#[doc = " `StakingLedger.claimed_rewards`."]
				#[doc = ""]
				#[doc = " Must be more than the number of eras delayed by session."]
				#[doc = " I.e. active era must always be in history. I.e. `active_era >"]
				#[doc = " current_era - history_depth` must be guaranteed."]
				#[doc = ""]
				#[doc = " If migrating an existing pallet from storage value to config value,"]
				#[doc = " this should be set to same value or greater as in storage."]
				#[doc = ""]
				#[doc = " Note: `HistoryDepth` is used as the upper bound for the `BoundedVec`"]
				#[doc = " item `StakingLedger.claimed_rewards`. Setting this value lower than"]
				#[doc = " the existing value can lead to inconsistencies in the"]
				#[doc = " `StakingLedger` and will need to be handled properly in a migration."]
				#[doc = " The test `reducing_history_depth_abrupt` shows this effect."]
				pub fn history_depth(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Staking", "HistoryDepth", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " Number of sessions per era."]
				pub fn sessions_per_era(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Staking", "SessionsPerEra", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " Number of eras that staked funds must remain bonded for."]
				pub fn bonding_duration(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Staking", "BondingDuration", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " Number of eras that slashes are deferred by, after computation."]
				#[doc = ""]
				#[doc = " This should be less than the bonding duration. Set to 0 if slashes"]
				#[doc = " should be applied immediately, without opportunity for intervention."]
				pub fn slash_defer_duration(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Staking",
						"SlashDeferDuration",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " The maximum number of nominators rewarded for each validator."]
				#[doc = ""]
				#[doc = " For each validator only the `$MaxNominatorRewardedPerValidator` biggest stakers can"]
				#[doc = " claim their reward. This used to limit the i/o cost for the nominator payout."]
				pub fn max_nominator_rewarded_per_validator(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Staking",
						"MaxNominatorRewardedPerValidator",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " The maximum number of `unlocking` chunks a [`StakingLedger`] can"]
				#[doc = " have. Effectively determines how many unique eras a staker may be"]
				#[doc = " unbonding in."]
				#[doc = ""]
				#[doc = " Note: `MaxUnlockingChunks` is used as the upper bound for the"]
				#[doc = " `BoundedVec` item `StakingLedger.unlocking`. Setting this value"]
				#[doc = " lower than the existing value can lead to inconsistencies in the"]
				#[doc = " `StakingLedger` and will need to be handled properly in a runtime"]
				#[doc = " migration. The test `reducing_max_unlocking_chunks_abrupt` shows"]
				#[doc = " this effect."]
				pub fn max_unlocking_chunks(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Staking",
						"MaxUnlockingChunks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod session {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetKeys {
				pub keys: runtime_types::da_runtime::primitives::SessionKeys,
				pub proof: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct PurgeKeys;
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::set_keys`]."]
				pub fn set_keys(
					&self,
					keys: runtime_types::da_runtime::primitives::SessionKeys,
					proof: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<SetKeys> {
					::subxt::tx::StaticTxPayload::new(
						"Session",
						"set_keys",
						SetKeys { keys, proof },
						[
							100u8, 34u8, 3u8, 243u8, 22u8, 163u8, 244u8, 57u8, 211u8, 164u8, 55u8,
							212u8, 151u8, 91u8, 13u8, 131u8, 89u8, 95u8, 173u8, 216u8, 149u8, 70u8,
							103u8, 199u8, 216u8, 87u8, 172u8, 149u8, 28u8, 184u8, 133u8, 8u8,
						],
					)
				}

				#[doc = "See [`Pallet::purge_keys`]."]
				pub fn purge_keys(&self) -> ::subxt::tx::StaticTxPayload<PurgeKeys> {
					::subxt::tx::StaticTxPayload::new("Session", "purge_keys", PurgeKeys {}, [
						200u8, 255u8, 4u8, 213u8, 188u8, 92u8, 99u8, 116u8, 163u8, 152u8, 29u8,
						35u8, 133u8, 119u8, 246u8, 44u8, 91u8, 31u8, 145u8, 23u8, 213u8, 64u8,
						71u8, 242u8, 207u8, 239u8, 231u8, 37u8, 61u8, 63u8, 190u8, 35u8,
					])
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_session::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "New session has happened. Note that the argument is the session index, not the"]
			#[doc = "block number as the type might suggest."]
			pub struct NewSession {
				pub session_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NewSession {
				const EVENT: &'static str = "NewSession";
				const PALLET: &'static str = "Session";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current set of validators."]
				pub fn validators(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"Validators",
						vec![],
						[
							144u8, 235u8, 200u8, 43u8, 151u8, 57u8, 147u8, 172u8, 201u8, 202u8,
							242u8, 96u8, 57u8, 76u8, 124u8, 77u8, 42u8, 113u8, 218u8, 220u8, 230u8,
							32u8, 151u8, 152u8, 172u8, 106u8, 60u8, 227u8, 122u8, 118u8, 137u8,
							68u8,
						],
					)
				}

				#[doc = " Current index of the session."]
				pub fn current_index(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"CurrentIndex",
						vec![],
						[
							148u8, 179u8, 159u8, 15u8, 197u8, 95u8, 214u8, 30u8, 209u8, 251u8,
							183u8, 231u8, 91u8, 25u8, 181u8, 191u8, 143u8, 252u8, 227u8, 80u8,
							159u8, 66u8, 194u8, 67u8, 113u8, 74u8, 111u8, 91u8, 218u8, 187u8,
							130u8, 40u8,
						],
					)
				}

				#[doc = " True if the underlying economic identities or weighting behind the validators"]
				#[doc = " has changed in the queued validator set."]
				pub fn queued_changed(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"QueuedChanged",
						vec![],
						[
							105u8, 140u8, 235u8, 218u8, 96u8, 100u8, 252u8, 10u8, 58u8, 221u8,
							244u8, 251u8, 67u8, 91u8, 80u8, 202u8, 152u8, 42u8, 50u8, 113u8, 200u8,
							247u8, 59u8, 213u8, 77u8, 195u8, 1u8, 150u8, 220u8, 18u8, 245u8, 46u8,
						],
					)
				}

				#[doc = " The queued keys for the next session. When the next session begins, these keys"]
				#[doc = " will be used to determine the validator's session keys."]
				pub fn queued_keys(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							::subxt::ext::sp_core::crypto::AccountId32,
							runtime_types::da_runtime::primitives::SessionKeys,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"QueuedKeys",
						vec![],
						[
							90u8, 48u8, 252u8, 89u8, 147u8, 67u8, 111u8, 201u8, 74u8, 181u8, 95u8,
							53u8, 95u8, 174u8, 167u8, 83u8, 223u8, 247u8, 221u8, 177u8, 172u8,
							148u8, 110u8, 21u8, 156u8, 34u8, 124u8, 81u8, 47u8, 26u8, 98u8, 210u8,
						],
					)
				}

				#[doc = " Indices of disabled validators."]
				#[doc = ""]
				#[doc = " The vec is always kept sorted so that we can find whether a given validator is"]
				#[doc = " disabled using binary search. It gets cleared when `on_session_ending` returns"]
				#[doc = " a new set of identities."]
				pub fn disabled_validators(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u32>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"DisabledValidators",
						vec![],
						[
							135u8, 22u8, 22u8, 97u8, 82u8, 217u8, 144u8, 141u8, 121u8, 240u8,
							189u8, 16u8, 176u8, 88u8, 177u8, 31u8, 20u8, 242u8, 73u8, 104u8, 11u8,
							110u8, 214u8, 34u8, 52u8, 217u8, 106u8, 33u8, 174u8, 174u8, 198u8,
							84u8,
						],
					)
				}

				#[doc = " The next session keys for a validator."]
				pub fn next_keys(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::da_runtime::primitives::SessionKeys,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"NextKeys",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							211u8, 170u8, 242u8, 9u8, 171u8, 174u8, 161u8, 66u8, 152u8, 134u8,
							69u8, 128u8, 194u8, 61u8, 225u8, 55u8, 230u8, 15u8, 77u8, 102u8, 95u8,
							114u8, 136u8, 204u8, 114u8, 52u8, 147u8, 252u8, 166u8, 48u8, 251u8,
							67u8,
						],
					)
				}

				#[doc = " The next session keys for a validator."]
				pub fn next_keys_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::da_runtime::primitives::SessionKeys,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"NextKeys",
						Vec::new(),
						[
							211u8, 170u8, 242u8, 9u8, 171u8, 174u8, 161u8, 66u8, 152u8, 134u8,
							69u8, 128u8, 194u8, 61u8, 225u8, 55u8, 230u8, 15u8, 77u8, 102u8, 95u8,
							114u8, 136u8, 204u8, 114u8, 52u8, 147u8, 252u8, 166u8, 48u8, 251u8,
							67u8,
						],
					)
				}

				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::KeyTypeId>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"KeyOwner",
						vec![::subxt::storage::address::StorageMapKey::new(
							&(_0.borrow(), _1.borrow()),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							4u8, 91u8, 25u8, 84u8, 250u8, 201u8, 174u8, 129u8, 201u8, 58u8, 197u8,
							199u8, 137u8, 240u8, 118u8, 33u8, 99u8, 2u8, 195u8, 57u8, 53u8, 172u8,
							0u8, 148u8, 203u8, 144u8, 149u8, 64u8, 135u8, 254u8, 242u8, 215u8,
						],
					)
				}

				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"KeyOwner",
						Vec::new(),
						[
							4u8, 91u8, 25u8, 84u8, 250u8, 201u8, 174u8, 129u8, 201u8, 58u8, 197u8,
							199u8, 137u8, 240u8, 118u8, 33u8, 99u8, 2u8, 195u8, 57u8, 53u8, 172u8,
							0u8, 148u8, 203u8, 144u8, 149u8, 64u8, 135u8, 254u8, 242u8, 215u8,
						],
					)
				}
			}
		}
	}
	pub mod technical_committee {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetMembers {
				pub new_members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				pub prime: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
				pub old_count: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Execute {
				pub proposal: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Propose {
				#[codec(compact)]
				pub threshold: ::core::primitive::u32,
				pub proposal: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Vote {
				pub proposal: ::subxt::ext::sp_core::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub approve: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct DisapproveProposal {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Close {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::set_members`]."]
				pub fn set_members(
					&self,
					new_members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					prime: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
					old_count: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetMembers> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommittee",
						"set_members",
						SetMembers {
							new_members,
							prime,
							old_count,
						},
						[
							196u8, 103u8, 123u8, 125u8, 226u8, 177u8, 126u8, 37u8, 160u8, 114u8,
							34u8, 136u8, 219u8, 84u8, 199u8, 94u8, 242u8, 20u8, 126u8, 126u8,
							166u8, 190u8, 198u8, 33u8, 162u8, 113u8, 237u8, 222u8, 90u8, 1u8, 2u8,
							234u8,
						],
					)
				}

				#[doc = "See [`Pallet::execute`]."]
				pub fn execute(
					&self,
					proposal: runtime_types::da_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Execute> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommittee",
						"execute",
						Execute {
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						},
						[
							152u8, 202u8, 82u8, 159u8, 47u8, 219u8, 43u8, 53u8, 123u8, 40u8, 171u8,
							71u8, 232u8, 167u8, 148u8, 180u8, 59u8, 46u8, 195u8, 54u8, 169u8,
							224u8, 114u8, 14u8, 169u8, 168u8, 0u8, 53u8, 20u8, 95u8, 118u8, 144u8,
						],
					)
				}

				#[doc = "See [`Pallet::propose`]."]
				pub fn propose(
					&self,
					threshold: ::core::primitive::u32,
					proposal: runtime_types::da_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Propose> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommittee",
						"propose",
						Propose {
							threshold,
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						},
						[
							56u8, 12u8, 148u8, 35u8, 158u8, 103u8, 212u8, 161u8, 113u8, 122u8,
							158u8, 235u8, 197u8, 17u8, 248u8, 67u8, 90u8, 15u8, 53u8, 64u8, 103u8,
							187u8, 16u8, 79u8, 145u8, 50u8, 161u8, 99u8, 63u8, 225u8, 109u8, 12u8,
						],
					)
				}

				#[doc = "See [`Pallet::vote`]."]
				pub fn vote(
					&self,
					proposal: ::subxt::ext::sp_core::H256,
					index: ::core::primitive::u32,
					approve: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<Vote> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommittee",
						"vote",
						Vote {
							proposal,
							index,
							approve,
						},
						[
							108u8, 46u8, 180u8, 148u8, 145u8, 24u8, 173u8, 56u8, 36u8, 100u8,
							216u8, 43u8, 178u8, 202u8, 26u8, 136u8, 93u8, 84u8, 80u8, 134u8, 14u8,
							42u8, 248u8, 205u8, 68u8, 92u8, 79u8, 11u8, 113u8, 115u8, 157u8, 100u8,
						],
					)
				}

				#[doc = "See [`Pallet::disapprove_proposal`]."]
				pub fn disapprove_proposal(
					&self,
					proposal_hash: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<DisapproveProposal> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommittee",
						"disapprove_proposal",
						DisapproveProposal { proposal_hash },
						[
							25u8, 123u8, 1u8, 8u8, 74u8, 37u8, 3u8, 40u8, 97u8, 37u8, 175u8, 224u8,
							72u8, 155u8, 123u8, 109u8, 104u8, 43u8, 91u8, 125u8, 199u8, 51u8, 17u8,
							225u8, 133u8, 38u8, 120u8, 76u8, 164u8, 5u8, 194u8, 201u8,
						],
					)
				}

				#[doc = "See [`Pallet::close`]."]
				pub fn close(
					&self,
					proposal_hash: ::subxt::ext::sp_core::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Close> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommittee",
						"close",
						Close {
							proposal_hash,
							index,
							proposal_weight_bound,
							length_bound,
						},
						[
							191u8, 138u8, 89u8, 247u8, 97u8, 51u8, 45u8, 193u8, 76u8, 16u8, 80u8,
							225u8, 197u8, 83u8, 204u8, 133u8, 169u8, 16u8, 86u8, 32u8, 125u8, 16u8,
							116u8, 185u8, 45u8, 20u8, 76u8, 148u8, 206u8, 163u8, 154u8, 30u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_collective::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
			#[doc = "`MemberCount`)."]
			pub struct Proposed {
				pub account: ::subxt::ext::sp_core::crypto::AccountId32,
				pub proposal_index: ::core::primitive::u32,
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub threshold: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const EVENT: &'static str = "Proposed";
				const PALLET: &'static str = "TechnicalCommittee";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A motion (given hash) has been voted on by given account, leaving"]
			#[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
			pub struct Voted {
				pub account: ::subxt::ext::sp_core::crypto::AccountId32,
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub voted: ::core::primitive::bool,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Voted {
				const EVENT: &'static str = "Voted";
				const PALLET: &'static str = "TechnicalCommittee";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A motion was approved by the required threshold."]
			pub struct Approved {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
			}
			impl ::subxt::events::StaticEvent for Approved {
				const EVENT: &'static str = "Approved";
				const PALLET: &'static str = "TechnicalCommittee";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A motion was not approved by the required threshold."]
			pub struct Disapproved {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
			}
			impl ::subxt::events::StaticEvent for Disapproved {
				const EVENT: &'static str = "Disapproved";
				const PALLET: &'static str = "TechnicalCommittee";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A motion was executed; result will be `Ok` if it returned without error."]
			pub struct Executed {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Executed {
				const EVENT: &'static str = "Executed";
				const PALLET: &'static str = "TechnicalCommittee";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A single member did some action; result will be `Ok` if it returned without error."]
			pub struct MemberExecuted {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MemberExecuted {
				const EVENT: &'static str = "MemberExecuted";
				const PALLET: &'static str = "TechnicalCommittee";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
			pub struct Closed {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Closed {
				const EVENT: &'static str = "Closed";
				const PALLET: &'static str = "TechnicalCommittee";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The hashes of the active proposals."]
				pub fn proposals(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::subxt::ext::sp_core::H256,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"Proposals",
						vec![],
						[
							10u8, 133u8, 82u8, 54u8, 193u8, 41u8, 253u8, 159u8, 56u8, 96u8, 249u8,
							148u8, 43u8, 57u8, 116u8, 43u8, 222u8, 243u8, 237u8, 231u8, 238u8,
							60u8, 26u8, 225u8, 19u8, 203u8, 213u8, 220u8, 114u8, 217u8, 100u8,
							27u8,
						],
					)
				}

				#[doc = " Actual proposal for a given hash, if it's current."]
				pub fn proposal_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::da_runtime::RuntimeCall>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"ProposalOf",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							234u8, 152u8, 190u8, 99u8, 237u8, 208u8, 206u8, 3u8, 211u8, 220u8,
							90u8, 166u8, 15u8, 161u8, 114u8, 169u8, 70u8, 214u8, 112u8, 111u8, 6u8,
							164u8, 250u8, 128u8, 137u8, 172u8, 96u8, 202u8, 218u8, 199u8, 205u8,
							188u8,
						],
					)
				}

				#[doc = " Actual proposal for a given hash, if it's current."]
				pub fn proposal_of_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::da_runtime::RuntimeCall>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"ProposalOf",
						Vec::new(),
						[
							234u8, 152u8, 190u8, 99u8, 237u8, 208u8, 206u8, 3u8, 211u8, 220u8,
							90u8, 166u8, 15u8, 161u8, 114u8, 169u8, 70u8, 214u8, 112u8, 111u8, 6u8,
							164u8, 250u8, 128u8, 137u8, 172u8, 96u8, 202u8, 218u8, 199u8, 205u8,
							188u8,
						],
					)
				}

				#[doc = " Votes on a given proposal, if it is ongoing."]
				pub fn voting(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_collective::Votes<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"Voting",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
							168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
							136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
							221u8,
						],
					)
				}

				#[doc = " Votes on a given proposal, if it is ongoing."]
				pub fn voting_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_collective::Votes<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"Voting",
						Vec::new(),
						[
							89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
							168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
							136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
							221u8,
						],
					)
				}

				#[doc = " Proposals so far."]
				pub fn proposal_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"ProposalCount",
						vec![],
						[
							132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
							140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
							24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
							70u8,
						],
					)
				}

				#[doc = " The current members of the collective. This is stored sorted (just by value)."]
				pub fn members(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"Members",
						vec![],
						[
							162u8, 72u8, 174u8, 204u8, 140u8, 105u8, 205u8, 176u8, 197u8, 117u8,
							206u8, 134u8, 157u8, 110u8, 139u8, 54u8, 43u8, 233u8, 25u8, 51u8, 36u8,
							238u8, 94u8, 124u8, 221u8, 52u8, 237u8, 71u8, 125u8, 56u8, 129u8,
							222u8,
						],
					)
				}

				#[doc = " The prime member that helps determine the default vote behavior in case of absentations."]
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"Prime",
						vec![],
						[
							108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
							157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
							209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
							158u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The maximum weight of a dispatch call that can be proposed and executed."]
				pub fn max_proposal_weight(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_weights::weight_v2::Weight,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"TechnicalCommittee",
						"MaxProposalWeight",
						[
							206u8, 61u8, 253u8, 247u8, 163u8, 40u8, 161u8, 52u8, 134u8, 140u8,
							206u8, 83u8, 44u8, 166u8, 226u8, 115u8, 181u8, 14u8, 227u8, 130u8,
							210u8, 32u8, 85u8, 29u8, 230u8, 97u8, 130u8, 165u8, 147u8, 134u8,
							106u8, 76u8,
						],
					)
				}
			}
		}
	}
	pub mod technical_membership {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct AddMember {
				pub who: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct RemoveMember {
				pub who: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SwapMember {
				pub remove: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub add: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ResetMembers {
				pub members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ChangeKey {
				pub new: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetPrime {
				pub who: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ClearPrime;
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::add_member`]."]
				pub fn add_member(
					&self,
					who: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<AddMember> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalMembership",
						"add_member",
						AddMember { who },
						[
							168u8, 166u8, 6u8, 167u8, 12u8, 109u8, 99u8, 96u8, 240u8, 57u8, 60u8,
							174u8, 57u8, 52u8, 131u8, 16u8, 230u8, 172u8, 23u8, 140u8, 48u8, 131u8,
							73u8, 131u8, 133u8, 217u8, 137u8, 50u8, 165u8, 149u8, 174u8, 188u8,
						],
					)
				}

				#[doc = "See [`Pallet::remove_member`]."]
				pub fn remove_member(
					&self,
					who: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<RemoveMember> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalMembership",
						"remove_member",
						RemoveMember { who },
						[
							33u8, 178u8, 96u8, 158u8, 126u8, 172u8, 0u8, 207u8, 143u8, 144u8,
							219u8, 28u8, 205u8, 197u8, 192u8, 195u8, 141u8, 26u8, 39u8, 101u8,
							140u8, 88u8, 212u8, 26u8, 221u8, 29u8, 187u8, 160u8, 119u8, 101u8,
							45u8, 162u8,
						],
					)
				}

				#[doc = "See [`Pallet::swap_member`]."]
				pub fn swap_member(
					&self,
					remove: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					add: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<SwapMember> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalMembership",
						"swap_member",
						SwapMember { remove, add },
						[
							52u8, 10u8, 13u8, 175u8, 35u8, 141u8, 159u8, 135u8, 34u8, 235u8, 117u8,
							146u8, 134u8, 49u8, 76u8, 116u8, 93u8, 209u8, 24u8, 242u8, 123u8, 82u8,
							34u8, 192u8, 147u8, 237u8, 163u8, 167u8, 18u8, 64u8, 196u8, 132u8,
						],
					)
				}

				#[doc = "See [`Pallet::reset_members`]."]
				pub fn reset_members(
					&self,
					members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::tx::StaticTxPayload<ResetMembers> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalMembership",
						"reset_members",
						ResetMembers { members },
						[
							9u8, 35u8, 28u8, 59u8, 158u8, 232u8, 89u8, 78u8, 101u8, 53u8, 240u8,
							98u8, 13u8, 104u8, 235u8, 161u8, 201u8, 150u8, 117u8, 32u8, 75u8,
							209u8, 166u8, 252u8, 57u8, 131u8, 96u8, 215u8, 51u8, 81u8, 42u8, 123u8,
						],
					)
				}

				#[doc = "See [`Pallet::change_key`]."]
				pub fn change_key(
					&self,
					new: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<ChangeKey> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalMembership",
						"change_key",
						ChangeKey { new },
						[
							202u8, 114u8, 208u8, 33u8, 254u8, 51u8, 31u8, 220u8, 229u8, 251u8,
							167u8, 149u8, 139u8, 131u8, 252u8, 100u8, 32u8, 20u8, 72u8, 97u8, 5u8,
							8u8, 25u8, 198u8, 95u8, 154u8, 73u8, 220u8, 46u8, 85u8, 162u8, 40u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_prime`]."]
				pub fn set_prime(
					&self,
					who: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetPrime> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalMembership",
						"set_prime",
						SetPrime { who },
						[
							109u8, 16u8, 35u8, 72u8, 169u8, 141u8, 101u8, 209u8, 241u8, 218u8,
							170u8, 180u8, 37u8, 223u8, 249u8, 37u8, 168u8, 20u8, 130u8, 30u8,
							191u8, 157u8, 230u8, 156u8, 135u8, 73u8, 96u8, 98u8, 193u8, 44u8, 38u8,
							247u8,
						],
					)
				}

				#[doc = "See [`Pallet::clear_prime`]."]
				pub fn clear_prime(&self) -> ::subxt::tx::StaticTxPayload<ClearPrime> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalMembership",
						"clear_prime",
						ClearPrime {},
						[
							186u8, 182u8, 225u8, 90u8, 71u8, 124u8, 69u8, 100u8, 234u8, 25u8, 53u8,
							23u8, 182u8, 32u8, 176u8, 81u8, 54u8, 140u8, 235u8, 126u8, 247u8, 7u8,
							155u8, 62u8, 35u8, 135u8, 48u8, 61u8, 88u8, 160u8, 183u8, 72u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_membership::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The given member was added; see the transaction for who."]
			pub struct MemberAdded;
			impl ::subxt::events::StaticEvent for MemberAdded {
				const EVENT: &'static str = "MemberAdded";
				const PALLET: &'static str = "TechnicalMembership";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The given member was removed; see the transaction for who."]
			pub struct MemberRemoved;
			impl ::subxt::events::StaticEvent for MemberRemoved {
				const EVENT: &'static str = "MemberRemoved";
				const PALLET: &'static str = "TechnicalMembership";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Two members were swapped; see the transaction for who."]
			pub struct MembersSwapped;
			impl ::subxt::events::StaticEvent for MembersSwapped {
				const EVENT: &'static str = "MembersSwapped";
				const PALLET: &'static str = "TechnicalMembership";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The membership was reset; see the transaction for who the new set is."]
			pub struct MembersReset;
			impl ::subxt::events::StaticEvent for MembersReset {
				const EVENT: &'static str = "MembersReset";
				const PALLET: &'static str = "TechnicalMembership";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "One of the members' keys changed."]
			pub struct KeyChanged;
			impl ::subxt::events::StaticEvent for KeyChanged {
				const EVENT: &'static str = "KeyChanged";
				const PALLET: &'static str = "TechnicalMembership";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Phantom member, never used."]
			pub struct Dummy;
			impl ::subxt::events::StaticEvent for Dummy {
				const EVENT: &'static str = "Dummy";
				const PALLET: &'static str = "TechnicalMembership";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current membership, stored as an ordered Vec."]
				pub fn members(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalMembership",
						"Members",
						vec![],
						[
							56u8, 56u8, 29u8, 90u8, 26u8, 115u8, 252u8, 185u8, 37u8, 108u8, 16u8,
							46u8, 136u8, 139u8, 30u8, 19u8, 235u8, 78u8, 176u8, 129u8, 180u8, 57u8,
							178u8, 239u8, 211u8, 6u8, 64u8, 129u8, 195u8, 46u8, 178u8, 157u8,
						],
					)
				}

				#[doc = " The current prime member, if one exists."]
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalMembership",
						"Prime",
						vec![],
						[
							108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
							157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
							209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
							158u8,
						],
					)
				}
			}
		}
	}
	pub mod grandpa {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ReportEquivocation {
				pub equivocation_proof: ::std::boxed::Box<
					runtime_types::sp_consensus_grandpa::EquivocationProof<
						::subxt::ext::sp_core::H256,
						::core::primitive::u32,
					>,
				>,
				pub key_owner_proof: runtime_types::sp_session::MembershipProof,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ReportEquivocationUnsigned {
				pub equivocation_proof: ::std::boxed::Box<
					runtime_types::sp_consensus_grandpa::EquivocationProof<
						::subxt::ext::sp_core::H256,
						::core::primitive::u32,
					>,
				>,
				pub key_owner_proof: runtime_types::sp_session::MembershipProof,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct NoteStalled {
				pub delay: ::core::primitive::u32,
				pub best_finalized_block_number: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::report_equivocation`]."]
				pub fn report_equivocation(
					&self,
					equivocation_proof: runtime_types::sp_consensus_grandpa::EquivocationProof<
						::subxt::ext::sp_core::H256,
						::core::primitive::u32,
					>,
					key_owner_proof: runtime_types::sp_session::MembershipProof,
				) -> ::subxt::tx::StaticTxPayload<ReportEquivocation> {
					::subxt::tx::StaticTxPayload::new(
						"Grandpa",
						"report_equivocation",
						ReportEquivocation {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							156u8, 162u8, 189u8, 89u8, 60u8, 156u8, 129u8, 176u8, 62u8, 35u8,
							214u8, 7u8, 68u8, 245u8, 130u8, 117u8, 30u8, 3u8, 73u8, 218u8, 142u8,
							82u8, 13u8, 141u8, 124u8, 19u8, 53u8, 138u8, 70u8, 4u8, 40u8, 32u8,
						],
					)
				}

				#[doc = "See [`Pallet::report_equivocation_unsigned`]."]
				pub fn report_equivocation_unsigned(
					&self,
					equivocation_proof: runtime_types::sp_consensus_grandpa::EquivocationProof<
						::subxt::ext::sp_core::H256,
						::core::primitive::u32,
					>,
					key_owner_proof: runtime_types::sp_session::MembershipProof,
				) -> ::subxt::tx::StaticTxPayload<ReportEquivocationUnsigned> {
					::subxt::tx::StaticTxPayload::new(
						"Grandpa",
						"report_equivocation_unsigned",
						ReportEquivocationUnsigned {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							166u8, 26u8, 217u8, 185u8, 215u8, 37u8, 174u8, 170u8, 137u8, 160u8,
							151u8, 43u8, 246u8, 86u8, 58u8, 18u8, 248u8, 73u8, 99u8, 161u8, 158u8,
							93u8, 212u8, 186u8, 224u8, 253u8, 234u8, 18u8, 151u8, 111u8, 227u8,
							249u8,
						],
					)
				}

				#[doc = "See [`Pallet::note_stalled`]."]
				pub fn note_stalled(
					&self,
					delay: ::core::primitive::u32,
					best_finalized_block_number: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<NoteStalled> {
					::subxt::tx::StaticTxPayload::new(
						"Grandpa",
						"note_stalled",
						NoteStalled {
							delay,
							best_finalized_block_number,
						},
						[
							197u8, 236u8, 137u8, 32u8, 46u8, 200u8, 144u8, 13u8, 89u8, 181u8,
							235u8, 73u8, 167u8, 131u8, 174u8, 93u8, 42u8, 136u8, 238u8, 59u8,
							129u8, 60u8, 83u8, 100u8, 5u8, 182u8, 99u8, 250u8, 145u8, 180u8, 1u8,
							199u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_grandpa::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "New authority set has been applied."]
			pub struct NewAuthorities {
				pub authority_set: ::std::vec::Vec<(
					runtime_types::sp_consensus_grandpa::app::Public,
					::core::primitive::u64,
				)>,
			}
			impl ::subxt::events::StaticEvent for NewAuthorities {
				const EVENT: &'static str = "NewAuthorities";
				const PALLET: &'static str = "Grandpa";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Current authority set has been paused."]
			pub struct Paused;
			impl ::subxt::events::StaticEvent for Paused {
				const EVENT: &'static str = "Paused";
				const PALLET: &'static str = "Grandpa";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Current authority set has been resumed."]
			pub struct Resumed;
			impl ::subxt::events::StaticEvent for Resumed {
				const EVENT: &'static str = "Resumed";
				const PALLET: &'static str = "Grandpa";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " State of the current authority set."]
				pub fn state(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Grandpa",
						"State",
						vec![],
						[
							211u8, 149u8, 114u8, 217u8, 206u8, 194u8, 115u8, 67u8, 12u8, 218u8,
							246u8, 213u8, 208u8, 29u8, 216u8, 104u8, 2u8, 39u8, 123u8, 172u8,
							252u8, 210u8, 52u8, 129u8, 147u8, 237u8, 244u8, 68u8, 252u8, 169u8,
							97u8, 148u8,
						],
					)
				}

				#[doc = " Pending change: (signaled at, scheduled change)."]
				pub fn pending_change(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Grandpa",
						"PendingChange",
						vec![],
						[
							178u8, 24u8, 140u8, 7u8, 8u8, 196u8, 18u8, 58u8, 3u8, 226u8, 181u8,
							47u8, 155u8, 160u8, 70u8, 12u8, 75u8, 189u8, 38u8, 255u8, 104u8, 141u8,
							64u8, 34u8, 134u8, 201u8, 102u8, 21u8, 75u8, 81u8, 218u8, 60u8,
						],
					)
				}

				#[doc = " next block number where we can force a change."]
				pub fn next_forced(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Grandpa",
						"NextForced",
						vec![],
						[
							99u8, 43u8, 245u8, 201u8, 60u8, 9u8, 122u8, 99u8, 188u8, 29u8, 67u8,
							6u8, 193u8, 133u8, 179u8, 67u8, 202u8, 208u8, 62u8, 179u8, 19u8, 169u8,
							196u8, 119u8, 107u8, 75u8, 100u8, 3u8, 121u8, 18u8, 80u8, 156u8,
						],
					)
				}

				#[doc = " `true` if we are currently stalled."]
				pub fn stalled(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Grandpa",
						"Stalled",
						vec![],
						[
							219u8, 8u8, 37u8, 78u8, 150u8, 55u8, 0u8, 57u8, 201u8, 170u8, 186u8,
							189u8, 56u8, 161u8, 44u8, 15u8, 53u8, 178u8, 224u8, 208u8, 231u8,
							109u8, 14u8, 209u8, 57u8, 205u8, 237u8, 153u8, 231u8, 156u8, 24u8,
							185u8,
						],
					)
				}

				#[doc = " The number of changes (both in terms of keys and underlying economic responsibilities)"]
				#[doc = " in the \"set\" of Grandpa validators from genesis."]
				pub fn current_set_id(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Grandpa",
						"CurrentSetId",
						vec![],
						[
							129u8, 7u8, 62u8, 101u8, 199u8, 60u8, 56u8, 33u8, 54u8, 158u8, 20u8,
							178u8, 244u8, 145u8, 189u8, 197u8, 157u8, 163u8, 116u8, 36u8, 105u8,
							52u8, 149u8, 244u8, 108u8, 94u8, 109u8, 111u8, 244u8, 137u8, 7u8,
							108u8,
						],
					)
				}

				#[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
				#[doc = " members were responsible."]
				#[doc = ""]
				#[doc = " This is only used for validating equivocation proofs. An equivocation proof must"]
				#[doc = " contains a key-ownership proof for a given session, therefore we need a way to tie"]
				#[doc = " together sessions and GRANDPA set ids, i.e. we need to validate that a validator"]
				#[doc = " was the owner of a given key on a given session, and what the active set ID was"]
				#[doc = " during that session."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: `SetId` is not under user control."]
				pub fn set_id_session(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Grandpa",
						"SetIdSession",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8, 231u8, 110u8, 11u8,
							166u8, 169u8, 103u8, 146u8, 123u8, 133u8, 157u8, 15u8, 33u8, 234u8,
							108u8, 13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8, 47u8, 53u8,
							134u8,
						],
					)
				}

				#[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
				#[doc = " members were responsible."]
				#[doc = ""]
				#[doc = " This is only used for validating equivocation proofs. An equivocation proof must"]
				#[doc = " contains a key-ownership proof for a given session, therefore we need a way to tie"]
				#[doc = " together sessions and GRANDPA set ids, i.e. we need to validate that a validator"]
				#[doc = " was the owner of a given key on a given session, and what the active set ID was"]
				#[doc = " during that session."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: `SetId` is not under user control."]
				pub fn set_id_session_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Grandpa",
						"SetIdSession",
						Vec::new(),
						[
							91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8, 231u8, 110u8, 11u8,
							166u8, 169u8, 103u8, 146u8, 123u8, 133u8, 157u8, 15u8, 33u8, 234u8,
							108u8, 13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8, 47u8, 53u8,
							134u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Max Authorities in use"]
				pub fn max_authorities(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Grandpa", "MaxAuthorities", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " The maximum number of entries to keep in the set id to session index mapping."]
				#[doc = ""]
				#[doc = " Since the `SetIdSession` map is only used for validating equivocations this"]
				#[doc = " value should relate to the bonding duration of whatever staking system is"]
				#[doc = " being used (if any). If equivocation handling is not enabled then this value"]
				#[doc = " can be zero."]
				pub fn max_set_id_session_entries(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Grandpa",
						"MaxSetIdSessionEntries",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod treasury {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ProposeSpend {
				#[codec(compact)]
				pub value: ::core::primitive::u128,
				pub beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct RejectProposal {
				#[codec(compact)]
				pub proposal_id: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ApproveProposal {
				#[codec(compact)]
				pub proposal_id: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Spend {
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
				pub beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct RemoveApproval {
				#[codec(compact)]
				pub proposal_id: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::propose_spend`]."]
				pub fn propose_spend(
					&self,
					value: ::core::primitive::u128,
					beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<ProposeSpend> {
					::subxt::tx::StaticTxPayload::new(
						"Treasury",
						"propose_spend",
						ProposeSpend { value, beneficiary },
						[
							124u8, 32u8, 83u8, 127u8, 240u8, 169u8, 3u8, 190u8, 235u8, 163u8, 23u8,
							29u8, 88u8, 242u8, 238u8, 187u8, 136u8, 75u8, 193u8, 192u8, 239u8, 2u8,
							54u8, 238u8, 147u8, 42u8, 91u8, 14u8, 244u8, 175u8, 41u8, 14u8,
						],
					)
				}

				#[doc = "See [`Pallet::reject_proposal`]."]
				pub fn reject_proposal(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<RejectProposal> {
					::subxt::tx::StaticTxPayload::new(
						"Treasury",
						"reject_proposal",
						RejectProposal { proposal_id },
						[
							106u8, 223u8, 97u8, 22u8, 111u8, 208u8, 128u8, 26u8, 198u8, 140u8,
							118u8, 126u8, 187u8, 51u8, 193u8, 50u8, 193u8, 68u8, 143u8, 144u8,
							34u8, 132u8, 44u8, 244u8, 105u8, 186u8, 223u8, 234u8, 17u8, 145u8,
							209u8, 145u8,
						],
					)
				}

				#[doc = "See [`Pallet::approve_proposal`]."]
				pub fn approve_proposal(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ApproveProposal> {
					::subxt::tx::StaticTxPayload::new(
						"Treasury",
						"approve_proposal",
						ApproveProposal { proposal_id },
						[
							164u8, 229u8, 172u8, 98u8, 129u8, 62u8, 84u8, 128u8, 47u8, 108u8, 33u8,
							120u8, 89u8, 79u8, 57u8, 121u8, 4u8, 197u8, 170u8, 153u8, 156u8, 17u8,
							59u8, 164u8, 123u8, 227u8, 175u8, 195u8, 220u8, 160u8, 60u8, 186u8,
						],
					)
				}

				#[doc = "See [`Pallet::spend`]."]
				pub fn spend(
					&self,
					amount: ::core::primitive::u128,
					beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<Spend> {
					::subxt::tx::StaticTxPayload::new(
						"Treasury",
						"spend",
						Spend {
							amount,
							beneficiary,
						},
						[
							208u8, 79u8, 96u8, 218u8, 205u8, 209u8, 165u8, 119u8, 92u8, 208u8,
							54u8, 168u8, 83u8, 190u8, 98u8, 97u8, 6u8, 2u8, 35u8, 249u8, 18u8,
							88u8, 193u8, 51u8, 130u8, 33u8, 28u8, 99u8, 49u8, 194u8, 34u8, 77u8,
						],
					)
				}

				#[doc = "See [`Pallet::remove_approval`]."]
				pub fn remove_approval(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<RemoveApproval> {
					::subxt::tx::StaticTxPayload::new(
						"Treasury",
						"remove_approval",
						RemoveApproval { proposal_id },
						[
							133u8, 126u8, 181u8, 47u8, 196u8, 243u8, 7u8, 46u8, 25u8, 251u8, 154u8,
							125u8, 217u8, 77u8, 54u8, 245u8, 240u8, 180u8, 97u8, 34u8, 186u8, 53u8,
							225u8, 144u8, 155u8, 107u8, 172u8, 54u8, 250u8, 184u8, 178u8, 86u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_treasury::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "New proposal."]
			pub struct Proposed {
				pub proposal_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const EVENT: &'static str = "Proposed";
				const PALLET: &'static str = "Treasury";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "We have ended a spend period and will now allocate funds."]
			pub struct Spending {
				pub budget_remaining: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Spending {
				const EVENT: &'static str = "Spending";
				const PALLET: &'static str = "Treasury";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some funds have been allocated."]
			pub struct Awarded {
				pub proposal_index: ::core::primitive::u32,
				pub award: ::core::primitive::u128,
				pub account: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Awarded {
				const EVENT: &'static str = "Awarded";
				const PALLET: &'static str = "Treasury";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A proposal was rejected; funds were slashed."]
			pub struct Rejected {
				pub proposal_index: ::core::primitive::u32,
				pub slashed: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rejected {
				const EVENT: &'static str = "Rejected";
				const PALLET: &'static str = "Treasury";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some of our funds have been burnt."]
			pub struct Burnt {
				pub burnt_funds: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Burnt {
				const EVENT: &'static str = "Burnt";
				const PALLET: &'static str = "Treasury";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Spending has finished; this is the amount that rolls over until next spend."]
			pub struct Rollover {
				pub rollover_balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rollover {
				const EVENT: &'static str = "Rollover";
				const PALLET: &'static str = "Treasury";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Some funds have been deposited."]
			pub struct Deposit {
				pub value: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposit {
				const EVENT: &'static str = "Deposit";
				const PALLET: &'static str = "Treasury";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A new spend proposal has been approved."]
			pub struct SpendApproved {
				pub proposal_index: ::core::primitive::u32,
				pub amount: ::core::primitive::u128,
				pub beneficiary: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for SpendApproved {
				const EVENT: &'static str = "SpendApproved";
				const PALLET: &'static str = "Treasury";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The inactive funds of the pallet have been updated."]
			pub struct UpdatedInactive {
				pub reactivated: ::core::primitive::u128,
				pub deactivated: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for UpdatedInactive {
				const EVENT: &'static str = "UpdatedInactive";
				const PALLET: &'static str = "Treasury";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Number of proposals that have been made."]
				pub fn proposal_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Treasury",
						"ProposalCount",
						vec![],
						[
							132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
							140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
							24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
							70u8,
						],
					)
				}

				#[doc = " Proposals that have been made."]
				pub fn proposals(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_treasury::Proposal<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Treasury",
						"Proposals",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							62u8, 223u8, 55u8, 209u8, 151u8, 134u8, 122u8, 65u8, 207u8, 38u8,
							113u8, 213u8, 237u8, 48u8, 129u8, 32u8, 91u8, 228u8, 108u8, 91u8, 37u8,
							49u8, 94u8, 4u8, 75u8, 122u8, 25u8, 34u8, 198u8, 224u8, 246u8, 160u8,
						],
					)
				}

				#[doc = " Proposals that have been made."]
				pub fn proposals_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_treasury::Proposal<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Treasury",
						"Proposals",
						Vec::new(),
						[
							62u8, 223u8, 55u8, 209u8, 151u8, 134u8, 122u8, 65u8, 207u8, 38u8,
							113u8, 213u8, 237u8, 48u8, 129u8, 32u8, 91u8, 228u8, 108u8, 91u8, 37u8,
							49u8, 94u8, 4u8, 75u8, 122u8, 25u8, 34u8, 198u8, 224u8, 246u8, 160u8,
						],
					)
				}

				#[doc = " The amount which has been reported as inactive to Currency."]
				pub fn deactivated(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Treasury",
						"Deactivated",
						vec![],
						[
							159u8, 57u8, 5u8, 85u8, 136u8, 128u8, 70u8, 43u8, 67u8, 76u8, 123u8,
							206u8, 48u8, 253u8, 51u8, 40u8, 14u8, 35u8, 162u8, 173u8, 127u8, 79u8,
							38u8, 235u8, 9u8, 141u8, 201u8, 37u8, 211u8, 176u8, 119u8, 106u8,
						],
					)
				}

				#[doc = " Proposal indices that have been approved but not yet awarded."]
				pub fn approvals(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Treasury",
						"Approvals",
						vec![],
						[
							202u8, 106u8, 189u8, 40u8, 127u8, 172u8, 108u8, 50u8, 193u8, 4u8,
							248u8, 226u8, 176u8, 101u8, 212u8, 222u8, 64u8, 206u8, 244u8, 175u8,
							111u8, 106u8, 86u8, 96u8, 19u8, 109u8, 218u8, 152u8, 30u8, 59u8, 96u8,
							1u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Fraction of a proposal's value that should be bonded in order to place the proposal."]
				#[doc = " An accepted proposal gets these back. A rejected proposal does not."]
				pub fn proposal_bond(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::per_things::Permill,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new("Treasury", "ProposalBond", [
						225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
						80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
						177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
					])
				}

				#[doc = " Minimum amount of funds that should be placed in a deposit for making a proposal."]
				pub fn proposal_bond_minimum(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Treasury",
						"ProposalBondMinimum",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}

				#[doc = " Maximum amount of funds that should be placed in a deposit for making a proposal."]
				pub fn proposal_bond_maximum(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						::core::option::Option<::core::primitive::u128>,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Treasury",
						"ProposalBondMaximum",
						[
							84u8, 154u8, 218u8, 83u8, 84u8, 189u8, 32u8, 20u8, 120u8, 194u8, 88u8,
							205u8, 109u8, 216u8, 114u8, 193u8, 120u8, 198u8, 154u8, 237u8, 134u8,
							204u8, 102u8, 247u8, 52u8, 103u8, 231u8, 43u8, 243u8, 122u8, 60u8,
							216u8,
						],
					)
				}

				#[doc = " Period between successive spends."]
				pub fn spend_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Treasury", "SpendPeriod", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " Percentage of spare funds (if any) that are burnt per spend period."]
				pub fn burn(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::per_things::Permill,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new("Treasury", "Burn", [
						225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
						80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
						177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
					])
				}

				#[doc = " The treasury's pallet id, used for deriving its sovereign account ID."]
				pub fn pallet_id(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::frame_support::PalletId>,
				> {
					::subxt::constants::StaticConstantAddress::new("Treasury", "PalletId", [
						139u8, 109u8, 228u8, 151u8, 252u8, 32u8, 130u8, 69u8, 112u8, 154u8, 174u8,
						45u8, 83u8, 245u8, 51u8, 132u8, 173u8, 5u8, 186u8, 24u8, 243u8, 9u8, 12u8,
						214u8, 80u8, 74u8, 69u8, 189u8, 30u8, 94u8, 22u8, 39u8,
					])
				}

				#[doc = " The maximum number of approvals that can wait in the spending queue."]
				#[doc = ""]
				#[doc = " NOTE: This parameter is also used within the Bounties Pallet extension if enabled."]
				pub fn max_approvals(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Treasury", "MaxApprovals", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}
			}
		}
	}
	pub mod sudo {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Sudo {
				pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SudoUncheckedWeight {
				pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetKey {
				pub new: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SudoAs {
				pub who: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::sudo`]."]
				pub fn sudo(
					&self,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<Sudo> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"sudo",
						Sudo {
							call: ::std::boxed::Box::new(call),
						},
						[
							111u8, 241u8, 59u8, 83u8, 203u8, 208u8, 185u8, 31u8, 198u8, 91u8, 93u8,
							169u8, 110u8, 122u8, 71u8, 194u8, 64u8, 219u8, 235u8, 8u8, 119u8,
							179u8, 166u8, 160u8, 142u8, 95u8, 236u8, 199u8, 225u8, 250u8, 57u8,
							40u8,
						],
					)
				}

				#[doc = "See [`Pallet::sudo_unchecked_weight`]."]
				pub fn sudo_unchecked_weight(
					&self,
					call: runtime_types::da_runtime::RuntimeCall,
					weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::StaticTxPayload<SudoUncheckedWeight> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"sudo_unchecked_weight",
						SudoUncheckedWeight {
							call: ::std::boxed::Box::new(call),
							weight,
						},
						[
							207u8, 234u8, 8u8, 77u8, 175u8, 23u8, 100u8, 58u8, 194u8, 213u8, 185u8,
							68u8, 51u8, 188u8, 231u8, 176u8, 189u8, 145u8, 252u8, 194u8, 6u8,
							219u8, 82u8, 161u8, 119u8, 87u8, 249u8, 155u8, 248u8, 181u8, 65u8,
							145u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_key`]."]
				pub fn set_key(
					&self,
					new: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetKey> {
					::subxt::tx::StaticTxPayload::new("Sudo", "set_key", SetKey { new }, [
						34u8, 116u8, 170u8, 18u8, 106u8, 17u8, 231u8, 159u8, 110u8, 246u8, 2u8,
						27u8, 161u8, 155u8, 163u8, 41u8, 138u8, 7u8, 81u8, 98u8, 230u8, 182u8,
						23u8, 222u8, 240u8, 117u8, 173u8, 232u8, 192u8, 55u8, 92u8, 208u8,
					])
				}

				#[doc = "See [`Pallet::sudo_as`]."]
				pub fn sudo_as(
					&self,
					who: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<SudoAs> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"sudo_as",
						SudoAs {
							who,
							call: ::std::boxed::Box::new(call),
						},
						[
							188u8, 2u8, 153u8, 100u8, 82u8, 82u8, 158u8, 99u8, 171u8, 111u8, 74u8,
							2u8, 104u8, 96u8, 7u8, 137u8, 253u8, 114u8, 209u8, 15u8, 0u8, 206u8,
							53u8, 77u8, 105u8, 63u8, 228u8, 95u8, 255u8, 102u8, 210u8, 168u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_sudo::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A sudo just took place. \\[result\\]"]
			pub struct Sudid {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Sudid {
				const EVENT: &'static str = "Sudid";
				const PALLET: &'static str = "Sudo";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
			pub struct KeyChanged {
				pub old_sudoer: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for KeyChanged {
				const EVENT: &'static str = "KeyChanged";
				const PALLET: &'static str = "Sudo";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A sudo just took place. \\[result\\]"]
			pub struct SudoAsDone {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for SudoAsDone {
				const EVENT: &'static str = "SudoAsDone";
				const PALLET: &'static str = "Sudo";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The `AccountId` of the sudo key."]
				pub fn key(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new("Sudo", "Key", vec![], [
						244u8, 73u8, 188u8, 136u8, 218u8, 163u8, 68u8, 179u8, 122u8, 173u8, 34u8,
						108u8, 137u8, 28u8, 182u8, 16u8, 196u8, 92u8, 138u8, 34u8, 102u8, 80u8,
						199u8, 88u8, 107u8, 207u8, 36u8, 22u8, 168u8, 167u8, 20u8, 142u8,
					])
				}
			}
		}
	}
	pub mod im_online {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Heartbeat {
				pub heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
				pub signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::heartbeat`]."]
				pub fn heartbeat(
					&self,
					heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
					signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
				) -> ::subxt::tx::StaticTxPayload<Heartbeat> {
					::subxt::tx::StaticTxPayload::new(
						"ImOnline",
						"heartbeat",
						Heartbeat {
							heartbeat,
							signature,
						},
						[
							79u8, 181u8, 65u8, 127u8, 102u8, 174u8, 149u8, 80u8, 249u8, 172u8,
							247u8, 69u8, 16u8, 252u8, 164u8, 143u8, 74u8, 196u8, 27u8, 133u8,
							216u8, 54u8, 66u8, 178u8, 170u8, 177u8, 16u8, 121u8, 180u8, 165u8,
							167u8, 107u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_im_online::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A new heartbeat was received from `AuthorityId`."]
			pub struct HeartbeatReceived {
				pub authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
			}
			impl ::subxt::events::StaticEvent for HeartbeatReceived {
				const EVENT: &'static str = "HeartbeatReceived";
				const PALLET: &'static str = "ImOnline";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "At the end of the session, no offence was committed."]
			pub struct AllGood;
			impl ::subxt::events::StaticEvent for AllGood {
				const EVENT: &'static str = "AllGood";
				const PALLET: &'static str = "ImOnline";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "At the end of the session, at least one validator was found to be offline."]
			pub struct SomeOffline {
				pub offline: ::std::vec::Vec<(
					::subxt::ext::sp_core::crypto::AccountId32,
					runtime_types::pallet_staking::Exposure<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					>,
				)>,
			}
			impl ::subxt::events::StaticEvent for SomeOffline {
				const EVENT: &'static str = "SomeOffline";
				const PALLET: &'static str = "ImOnline";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The block number after which it's ok to send heartbeats in the current"]
				#[doc = " session."]
				#[doc = ""]
				#[doc = " At the beginning of each session we set this to a value that should fall"]
				#[doc = " roughly in the middle of the session duration. The idea is to first wait for"]
				#[doc = " the validators to produce a block in the current session, so that the"]
				#[doc = " heartbeat later on will not be necessary."]
				#[doc = ""]
				#[doc = " This value will only be used as a fallback if we fail to get a proper session"]
				#[doc = " progress estimate from `NextSessionRotation`, as those estimates should be"]
				#[doc = " more accurate then the value we calculate for `HeartbeatAfter`."]
				pub fn heartbeat_after(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ImOnline",
						"HeartbeatAfter",
						vec![],
						[
							108u8, 100u8, 85u8, 198u8, 226u8, 122u8, 94u8, 225u8, 97u8, 154u8,
							135u8, 95u8, 106u8, 28u8, 185u8, 78u8, 192u8, 196u8, 35u8, 191u8, 12u8,
							19u8, 163u8, 46u8, 232u8, 235u8, 193u8, 81u8, 126u8, 204u8, 25u8,
							228u8,
						],
					)
				}

				#[doc = " The current set of keys that may issue a heartbeat."]
				pub fn keys(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ImOnline",
						"Keys",
						vec![],
						[
							6u8, 198u8, 221u8, 58u8, 14u8, 166u8, 245u8, 103u8, 191u8, 20u8, 69u8,
							233u8, 147u8, 245u8, 24u8, 64u8, 207u8, 180u8, 39u8, 208u8, 252u8,
							236u8, 247u8, 112u8, 187u8, 97u8, 70u8, 11u8, 248u8, 148u8, 208u8,
							106u8,
						],
					)
				}

				#[doc = " For each session index, we keep a mapping of `SessionIndex` and `AuthIndex`."]
				pub fn received_heartbeats(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ImOnline",
						"ReceivedHeartbeats",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							19u8, 177u8, 124u8, 81u8, 135u8, 39u8, 34u8, 67u8, 152u8, 235u8, 17u8,
							224u8, 238u8, 84u8, 179u8, 248u8, 250u8, 13u8, 17u8, 85u8, 24u8, 107u8,
							234u8, 110u8, 16u8, 237u8, 203u8, 21u8, 115u8, 124u8, 195u8, 229u8,
						],
					)
				}

				#[doc = " For each session index, we keep a mapping of `SessionIndex` and `AuthIndex`."]
				pub fn received_heartbeats_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ImOnline",
						"ReceivedHeartbeats",
						Vec::new(),
						[
							19u8, 177u8, 124u8, 81u8, 135u8, 39u8, 34u8, 67u8, 152u8, 235u8, 17u8,
							224u8, 238u8, 84u8, 179u8, 248u8, 250u8, 13u8, 17u8, 85u8, 24u8, 107u8,
							234u8, 110u8, 16u8, 237u8, 203u8, 21u8, 115u8, 124u8, 195u8, 229u8,
						],
					)
				}

				#[doc = " For each session index, we keep a mapping of `ValidatorId<T>` to the"]
				#[doc = " number of blocks authored by the given authority."]
				pub fn authored_blocks(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ImOnline",
						"AuthoredBlocks",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							50u8, 4u8, 242u8, 240u8, 247u8, 184u8, 114u8, 245u8, 233u8, 170u8,
							24u8, 197u8, 18u8, 245u8, 8u8, 28u8, 33u8, 115u8, 166u8, 245u8, 221u8,
							223u8, 56u8, 144u8, 33u8, 139u8, 10u8, 227u8, 228u8, 223u8, 103u8,
							151u8,
						],
					)
				}

				#[doc = " For each session index, we keep a mapping of `ValidatorId<T>` to the"]
				#[doc = " number of blocks authored by the given authority."]
				pub fn authored_blocks_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ImOnline",
						"AuthoredBlocks",
						Vec::new(),
						[
							50u8, 4u8, 242u8, 240u8, 247u8, 184u8, 114u8, 245u8, 233u8, 170u8,
							24u8, 197u8, 18u8, 245u8, 8u8, 28u8, 33u8, 115u8, 166u8, 245u8, 221u8,
							223u8, 56u8, 144u8, 33u8, 139u8, 10u8, 227u8, 228u8, 223u8, 103u8,
							151u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " A configuration for base priority of unsigned transactions."]
				#[doc = ""]
				#[doc = " This is exposed so that it can be tuned for particular runtime, when"]
				#[doc = " multiple pallets send unsigned transactions."]
				pub fn unsigned_priority(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ImOnline",
						"UnsignedPriority",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod authority_discovery {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Keys of the current authority set."]
				pub fn keys(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							runtime_types::sp_authority_discovery::app::Public,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"AuthorityDiscovery",
						"Keys",
						vec![],
						[
							6u8, 198u8, 221u8, 58u8, 14u8, 166u8, 245u8, 103u8, 191u8, 20u8, 69u8,
							233u8, 147u8, 245u8, 24u8, 64u8, 207u8, 180u8, 39u8, 208u8, 252u8,
							236u8, 247u8, 112u8, 187u8, 97u8, 70u8, 11u8, 248u8, 148u8, 208u8,
							106u8,
						],
					)
				}

				#[doc = " Keys of the next authority set."]
				pub fn next_keys(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							runtime_types::sp_authority_discovery::app::Public,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"AuthorityDiscovery",
						"NextKeys",
						vec![],
						[
							213u8, 94u8, 49u8, 159u8, 135u8, 1u8, 13u8, 150u8, 28u8, 15u8, 105u8,
							130u8, 90u8, 15u8, 130u8, 138u8, 186u8, 118u8, 10u8, 238u8, 173u8,
							229u8, 8u8, 144u8, 206u8, 121u8, 90u8, 203u8, 125u8, 106u8, 145u8,
							144u8,
						],
					)
				}
			}
		}
	}
	pub mod offences {
		use super::{root_mod, runtime_types};
		#[doc = "Events type."]
		pub type Event = runtime_types::pallet_offences::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "There is an offence reported of the given `kind` happened at the `session_index` and"]
			#[doc = "(kind-specific) time slot. This event is not deposited for duplicate slashes."]
			#[doc = "\\[kind, timeslot\\]."]
			pub struct Offence {
				pub kind: [::core::primitive::u8; 16usize],
				pub timeslot: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for Offence {
				const EVENT: &'static str = "Offence";
				const PALLET: &'static str = "Offences";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The primary structure that holds all offence records keyed by report identifiers."]
				pub fn reports(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_staking::offence::OffenceDetails<
							::subxt::ext::sp_core::crypto::AccountId32,
							(
								::subxt::ext::sp_core::crypto::AccountId32,
								runtime_types::pallet_staking::Exposure<
									::subxt::ext::sp_core::crypto::AccountId32,
									::core::primitive::u128,
								>,
							),
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Offences",
						"Reports",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							144u8, 30u8, 66u8, 199u8, 102u8, 236u8, 175u8, 201u8, 206u8, 170u8,
							55u8, 162u8, 137u8, 120u8, 220u8, 213u8, 57u8, 252u8, 0u8, 88u8, 210u8,
							68u8, 5u8, 25u8, 77u8, 114u8, 204u8, 23u8, 190u8, 32u8, 211u8, 30u8,
						],
					)
				}

				#[doc = " The primary structure that holds all offence records keyed by report identifiers."]
				pub fn reports_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_staking::offence::OffenceDetails<
							::subxt::ext::sp_core::crypto::AccountId32,
							(
								::subxt::ext::sp_core::crypto::AccountId32,
								runtime_types::pallet_staking::Exposure<
									::subxt::ext::sp_core::crypto::AccountId32,
									::core::primitive::u128,
								>,
							),
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Offences",
						"Reports",
						Vec::new(),
						[
							144u8, 30u8, 66u8, 199u8, 102u8, 236u8, 175u8, 201u8, 206u8, 170u8,
							55u8, 162u8, 137u8, 120u8, 220u8, 213u8, 57u8, 252u8, 0u8, 88u8, 210u8,
							68u8, 5u8, 25u8, 77u8, 114u8, 204u8, 23u8, 190u8, 32u8, 211u8, 30u8,
						],
					)
				}

				#[doc = " A vector of reports of the same kind that happened at the same time slot."]
				pub fn concurrent_reports_index(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8; 16usize]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::ext::sp_core::H256>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Offences",
						"ConcurrentReportsIndex",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							106u8, 21u8, 104u8, 5u8, 4u8, 66u8, 28u8, 70u8, 161u8, 195u8, 238u8,
							28u8, 69u8, 241u8, 221u8, 113u8, 140u8, 103u8, 181u8, 143u8, 60u8,
							177u8, 13u8, 129u8, 224u8, 149u8, 77u8, 32u8, 75u8, 74u8, 101u8, 65u8,
						],
					)
				}

				#[doc = " A vector of reports of the same kind that happened at the same time slot."]
				pub fn concurrent_reports_index_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::ext::sp_core::H256>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Offences",
						"ConcurrentReportsIndex",
						Vec::new(),
						[
							106u8, 21u8, 104u8, 5u8, 4u8, 66u8, 28u8, 70u8, 161u8, 195u8, 238u8,
							28u8, 69u8, 241u8, 221u8, 113u8, 140u8, 103u8, 181u8, 143u8, 60u8,
							177u8, 13u8, 129u8, 224u8, 149u8, 77u8, 32u8, 75u8, 74u8, 101u8, 65u8,
						],
					)
				}
			}
		}
	}
	pub mod historical {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Mapping from historical session indices to session-data root hash and validator count."]
				pub fn historical_sessions(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::subxt::ext::sp_core::H256,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Historical",
						"HistoricalSessions",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							94u8, 72u8, 245u8, 151u8, 214u8, 10u8, 12u8, 113u8, 13u8, 141u8, 176u8,
							178u8, 115u8, 238u8, 224u8, 181u8, 18u8, 5u8, 71u8, 65u8, 189u8, 148u8,
							161u8, 106u8, 24u8, 211u8, 72u8, 66u8, 221u8, 244u8, 117u8, 184u8,
						],
					)
				}

				#[doc = " Mapping from historical session indices to session-data root hash and validator count."]
				pub fn historical_sessions_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::subxt::ext::sp_core::H256,
						::core::primitive::u32,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Historical",
						"HistoricalSessions",
						Vec::new(),
						[
							94u8, 72u8, 245u8, 151u8, 214u8, 10u8, 12u8, 113u8, 13u8, 141u8, 176u8,
							178u8, 115u8, 238u8, 224u8, 181u8, 18u8, 5u8, 71u8, 65u8, 189u8, 148u8,
							161u8, 106u8, 24u8, 211u8, 72u8, 66u8, 221u8, 244u8, 117u8, 184u8,
						],
					)
				}

				#[doc = " The range of historical sessions we store. [first, last)"]
				pub fn stored_range(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Historical",
						"StoredRange",
						vec![],
						[
							89u8, 239u8, 197u8, 93u8, 135u8, 62u8, 142u8, 237u8, 64u8, 200u8,
							164u8, 4u8, 130u8, 233u8, 16u8, 238u8, 166u8, 206u8, 71u8, 42u8, 171u8,
							84u8, 8u8, 245u8, 183u8, 216u8, 212u8, 16u8, 190u8, 3u8, 167u8, 189u8,
						],
					)
				}
			}
		}
	}
	pub mod scheduler {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Schedule {
				pub when: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Cancel {
				pub when: ::core::primitive::u32,
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ScheduleNamed {
				pub id: [::core::primitive::u8; 32usize],
				pub when: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct CancelNamed {
				pub id: [::core::primitive::u8; 32usize],
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ScheduleAfter {
				pub after: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ScheduleNamedAfter {
				pub id: [::core::primitive::u8; 32usize],
				pub after: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::schedule`]."]
				pub fn schedule(
					&self,
					when: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<Schedule> {
					::subxt::tx::StaticTxPayload::new(
						"Scheduler",
						"schedule",
						Schedule {
							when,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							30u8, 131u8, 149u8, 166u8, 123u8, 241u8, 198u8, 14u8, 227u8, 216u8,
							76u8, 130u8, 28u8, 79u8, 28u8, 70u8, 0u8, 134u8, 150u8, 24u8, 128u8,
							56u8, 127u8, 228u8, 254u8, 183u8, 122u8, 181u8, 167u8, 111u8, 97u8,
							144u8,
						],
					)
				}

				#[doc = "See [`Pallet::cancel`]."]
				pub fn cancel(
					&self,
					when: ::core::primitive::u32,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Cancel> {
					::subxt::tx::StaticTxPayload::new(
						"Scheduler",
						"cancel",
						Cancel { when, index },
						[
							81u8, 251u8, 234u8, 17u8, 214u8, 75u8, 19u8, 59u8, 19u8, 30u8, 89u8,
							74u8, 6u8, 216u8, 238u8, 165u8, 7u8, 19u8, 153u8, 253u8, 161u8, 103u8,
							178u8, 227u8, 152u8, 180u8, 80u8, 156u8, 82u8, 126u8, 132u8, 120u8,
						],
					)
				}

				#[doc = "See [`Pallet::schedule_named`]."]
				pub fn schedule_named(
					&self,
					id: [::core::primitive::u8; 32usize],
					when: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<ScheduleNamed> {
					::subxt::tx::StaticTxPayload::new(
						"Scheduler",
						"schedule_named",
						ScheduleNamed {
							id,
							when,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							124u8, 138u8, 126u8, 172u8, 2u8, 233u8, 233u8, 113u8, 163u8, 44u8,
							239u8, 21u8, 24u8, 207u8, 138u8, 197u8, 25u8, 203u8, 50u8, 74u8, 241u8,
							4u8, 31u8, 22u8, 162u8, 184u8, 202u8, 202u8, 15u8, 217u8, 166u8, 245u8,
						],
					)
				}

				#[doc = "See [`Pallet::cancel_named`]."]
				pub fn cancel_named(
					&self,
					id: [::core::primitive::u8; 32usize],
				) -> ::subxt::tx::StaticTxPayload<CancelNamed> {
					::subxt::tx::StaticTxPayload::new(
						"Scheduler",
						"cancel_named",
						CancelNamed { id },
						[
							51u8, 3u8, 140u8, 50u8, 214u8, 211u8, 50u8, 4u8, 19u8, 43u8, 230u8,
							114u8, 18u8, 108u8, 138u8, 67u8, 99u8, 24u8, 255u8, 11u8, 246u8, 37u8,
							192u8, 207u8, 90u8, 157u8, 171u8, 93u8, 233u8, 189u8, 64u8, 180u8,
						],
					)
				}

				#[doc = "See [`Pallet::schedule_after`]."]
				pub fn schedule_after(
					&self,
					after: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<ScheduleAfter> {
					::subxt::tx::StaticTxPayload::new(
						"Scheduler",
						"schedule_after",
						ScheduleAfter {
							after,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							72u8, 44u8, 178u8, 23u8, 230u8, 126u8, 63u8, 187u8, 65u8, 215u8, 193u8,
							147u8, 72u8, 157u8, 252u8, 230u8, 164u8, 117u8, 248u8, 54u8, 161u8,
							232u8, 63u8, 61u8, 242u8, 90u8, 21u8, 233u8, 165u8, 235u8, 202u8,
							119u8,
						],
					)
				}

				#[doc = "See [`Pallet::schedule_named_after`]."]
				pub fn schedule_named_after(
					&self,
					id: [::core::primitive::u8; 32usize],
					after: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<ScheduleNamedAfter> {
					::subxt::tx::StaticTxPayload::new(
						"Scheduler",
						"schedule_named_after",
						ScheduleNamedAfter {
							id,
							after,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							135u8, 94u8, 32u8, 178u8, 242u8, 49u8, 160u8, 45u8, 156u8, 116u8,
							151u8, 20u8, 19u8, 236u8, 139u8, 76u8, 80u8, 120u8, 109u8, 235u8, 77u8,
							108u8, 82u8, 53u8, 182u8, 191u8, 12u8, 44u8, 68u8, 155u8, 162u8, 145u8,
						],
					)
				}
			}
		}
		#[doc = "Events type."]
		pub type Event = runtime_types::pallet_scheduler::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Scheduled some task."]
			pub struct Scheduled {
				pub when: ::core::primitive::u32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Scheduled {
				const EVENT: &'static str = "Scheduled";
				const PALLET: &'static str = "Scheduler";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Canceled some task."]
			pub struct Canceled {
				pub when: ::core::primitive::u32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Canceled {
				const EVENT: &'static str = "Canceled";
				const PALLET: &'static str = "Scheduler";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Dispatched some task."]
			pub struct Dispatched {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Dispatched {
				const EVENT: &'static str = "Dispatched";
				const PALLET: &'static str = "Scheduler";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The call for the provided hash was not found so the task has been aborted."]
			pub struct CallUnavailable {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for CallUnavailable {
				const EVENT: &'static str = "CallUnavailable";
				const PALLET: &'static str = "Scheduler";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The given task was unable to be renewed since the agenda is full at that block."]
			pub struct PeriodicFailed {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for PeriodicFailed {
				const EVENT: &'static str = "PeriodicFailed";
				const PALLET: &'static str = "Scheduler";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The given task can never be executed since it is overweight."]
			pub struct PermanentlyOverweight {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for PermanentlyOverweight {
				const EVENT: &'static str = "PermanentlyOverweight";
				const PALLET: &'static str = "Scheduler";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn incomplete_since(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Scheduler",
						"IncompleteSince",
						vec![],
						[
							149u8, 66u8, 239u8, 67u8, 235u8, 219u8, 101u8, 182u8, 145u8, 56u8,
							252u8, 150u8, 253u8, 221u8, 125u8, 57u8, 38u8, 152u8, 153u8, 31u8,
							92u8, 238u8, 66u8, 246u8, 104u8, 163u8, 94u8, 73u8, 222u8, 168u8,
							193u8, 227u8,
						],
					)
				}

				#[doc = " Items to be executed, indexed by the block number that they should be executed on."]
				pub fn agenda(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::option::Option<
								runtime_types::pallet_scheduler::Scheduled<
									[::core::primitive::u8; 32usize],
									runtime_types::frame_support::traits::preimages::Bounded<
										runtime_types::da_runtime::RuntimeCall,
									>,
									::core::primitive::u32,
									runtime_types::da_runtime::OriginCaller,
									::subxt::ext::sp_core::crypto::AccountId32,
								>,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Scheduler",
						"Agenda",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							105u8, 206u8, 94u8, 132u8, 203u8, 92u8, 167u8, 62u8, 230u8, 165u8,
							123u8, 162u8, 237u8, 14u8, 19u8, 53u8, 76u8, 109u8, 54u8, 39u8, 45u8,
							32u8, 170u8, 130u8, 112u8, 122u8, 29u8, 46u8, 69u8, 81u8, 59u8, 98u8,
						],
					)
				}

				#[doc = " Items to be executed, indexed by the block number that they should be executed on."]
				pub fn agenda_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::option::Option<
								runtime_types::pallet_scheduler::Scheduled<
									[::core::primitive::u8; 32usize],
									runtime_types::frame_support::traits::preimages::Bounded<
										runtime_types::da_runtime::RuntimeCall,
									>,
									::core::primitive::u32,
									runtime_types::da_runtime::OriginCaller,
									::subxt::ext::sp_core::crypto::AccountId32,
								>,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Scheduler",
						"Agenda",
						Vec::new(),
						[
							105u8, 206u8, 94u8, 132u8, 203u8, 92u8, 167u8, 62u8, 230u8, 165u8,
							123u8, 162u8, 237u8, 14u8, 19u8, 53u8, 76u8, 109u8, 54u8, 39u8, 45u8,
							32u8, 170u8, 130u8, 112u8, 122u8, 29u8, 46u8, 69u8, 81u8, 59u8, 98u8,
						],
					)
				}

				#[doc = " Lookup from a name to the block number and index of the task."]
				#[doc = ""]
				#[doc = " For v3 -> v4 the previously unbounded identities are Blake2-256 hashed to form the v4"]
				#[doc = " identities."]
				pub fn lookup(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8; 32usize]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Scheduler",
						"Lookup",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							82u8, 20u8, 178u8, 101u8, 108u8, 198u8, 71u8, 99u8, 16u8, 175u8, 15u8,
							187u8, 229u8, 243u8, 140u8, 200u8, 99u8, 77u8, 248u8, 178u8, 45u8,
							121u8, 193u8, 67u8, 165u8, 43u8, 234u8, 211u8, 158u8, 250u8, 103u8,
							243u8,
						],
					)
				}

				#[doc = " Lookup from a name to the block number and index of the task."]
				#[doc = ""]
				#[doc = " For v3 -> v4 the previously unbounded identities are Blake2-256 hashed to form the v4"]
				#[doc = " identities."]
				pub fn lookup_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Scheduler",
						"Lookup",
						Vec::new(),
						[
							82u8, 20u8, 178u8, 101u8, 108u8, 198u8, 71u8, 99u8, 16u8, 175u8, 15u8,
							187u8, 229u8, 243u8, 140u8, 200u8, 99u8, 77u8, 248u8, 178u8, 45u8,
							121u8, 193u8, 67u8, 165u8, 43u8, 234u8, 211u8, 158u8, 250u8, 103u8,
							243u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The maximum weight that may be scheduled per block for any dispatchables."]
				pub fn maximum_weight(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_weights::weight_v2::Weight,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new("Scheduler", "MaximumWeight", [
						206u8, 61u8, 253u8, 247u8, 163u8, 40u8, 161u8, 52u8, 134u8, 140u8, 206u8,
						83u8, 44u8, 166u8, 226u8, 115u8, 181u8, 14u8, 227u8, 130u8, 210u8, 32u8,
						85u8, 29u8, 230u8, 97u8, 130u8, 165u8, 147u8, 134u8, 106u8, 76u8,
					])
				}

				#[doc = " The maximum number of scheduled calls in the queue for a single block."]
				#[doc = ""]
				#[doc = " NOTE:"]
				#[doc = " + Dependent pallets' benchmarks might require a higher limit for the setting. Set a"]
				#[doc = " higher limit under `runtime-benchmarks` feature."]
				pub fn max_scheduled_per_block(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Scheduler",
						"MaxScheduledPerBlock",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod bounties {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ProposeBounty {
				#[codec(compact)]
				pub value: ::core::primitive::u128,
				pub description: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ApproveBounty {
				#[codec(compact)]
				pub bounty_id: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ProposeCurator {
				#[codec(compact)]
				pub bounty_id: ::core::primitive::u32,
				pub curator: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub fee: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct UnassignCurator {
				#[codec(compact)]
				pub bounty_id: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct AcceptCurator {
				#[codec(compact)]
				pub bounty_id: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct AwardBounty {
				#[codec(compact)]
				pub bounty_id: ::core::primitive::u32,
				pub beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ClaimBounty {
				#[codec(compact)]
				pub bounty_id: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct CloseBounty {
				#[codec(compact)]
				pub bounty_id: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ExtendBountyExpiry {
				#[codec(compact)]
				pub bounty_id: ::core::primitive::u32,
				pub remark: ::std::vec::Vec<::core::primitive::u8>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::propose_bounty`]."]
				pub fn propose_bounty(
					&self,
					value: ::core::primitive::u128,
					description: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<ProposeBounty> {
					::subxt::tx::StaticTxPayload::new(
						"Bounties",
						"propose_bounty",
						ProposeBounty { value, description },
						[
							99u8, 160u8, 94u8, 74u8, 105u8, 161u8, 123u8, 239u8, 241u8, 117u8,
							97u8, 99u8, 84u8, 101u8, 87u8, 3u8, 88u8, 175u8, 75u8, 59u8, 114u8,
							87u8, 18u8, 113u8, 126u8, 26u8, 42u8, 104u8, 201u8, 128u8, 102u8,
							219u8,
						],
					)
				}

				#[doc = "See [`Pallet::approve_bounty`]."]
				pub fn approve_bounty(
					&self,
					bounty_id: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ApproveBounty> {
					::subxt::tx::StaticTxPayload::new(
						"Bounties",
						"approve_bounty",
						ApproveBounty { bounty_id },
						[
							82u8, 228u8, 232u8, 103u8, 198u8, 173u8, 190u8, 148u8, 159u8, 86u8,
							48u8, 4u8, 32u8, 169u8, 1u8, 129u8, 96u8, 145u8, 235u8, 68u8, 48u8,
							34u8, 5u8, 1u8, 76u8, 26u8, 100u8, 228u8, 92u8, 198u8, 183u8, 173u8,
						],
					)
				}

				#[doc = "See [`Pallet::propose_curator`]."]
				pub fn propose_curator(
					&self,
					bounty_id: ::core::primitive::u32,
					curator: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					fee: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ProposeCurator> {
					::subxt::tx::StaticTxPayload::new(
						"Bounties",
						"propose_curator",
						ProposeCurator {
							bounty_id,
							curator,
							fee,
						},
						[
							85u8, 186u8, 206u8, 137u8, 98u8, 87u8, 202u8, 71u8, 89u8, 241u8, 56u8,
							212u8, 89u8, 215u8, 65u8, 97u8, 202u8, 139u8, 78u8, 66u8, 92u8, 177u8,
							163u8, 111u8, 212u8, 244u8, 41u8, 153u8, 104u8, 129u8, 112u8, 237u8,
						],
					)
				}

				#[doc = "See [`Pallet::unassign_curator`]."]
				pub fn unassign_curator(
					&self,
					bounty_id: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<UnassignCurator> {
					::subxt::tx::StaticTxPayload::new(
						"Bounties",
						"unassign_curator",
						UnassignCurator { bounty_id },
						[
							218u8, 241u8, 247u8, 89u8, 95u8, 120u8, 93u8, 18u8, 85u8, 114u8, 158u8,
							254u8, 68u8, 77u8, 230u8, 186u8, 230u8, 201u8, 63u8, 223u8, 28u8,
							173u8, 244u8, 82u8, 113u8, 177u8, 99u8, 27u8, 207u8, 247u8, 207u8,
							213u8,
						],
					)
				}

				#[doc = "See [`Pallet::accept_curator`]."]
				pub fn accept_curator(
					&self,
					bounty_id: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<AcceptCurator> {
					::subxt::tx::StaticTxPayload::new(
						"Bounties",
						"accept_curator",
						AcceptCurator { bounty_id },
						[
							106u8, 96u8, 22u8, 67u8, 52u8, 109u8, 180u8, 225u8, 122u8, 253u8,
							209u8, 214u8, 132u8, 131u8, 247u8, 131u8, 162u8, 51u8, 144u8, 30u8,
							12u8, 126u8, 50u8, 152u8, 229u8, 119u8, 54u8, 116u8, 112u8, 235u8,
							34u8, 166u8,
						],
					)
				}

				#[doc = "See [`Pallet::award_bounty`]."]
				pub fn award_bounty(
					&self,
					bounty_id: ::core::primitive::u32,
					beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<AwardBounty> {
					::subxt::tx::StaticTxPayload::new(
						"Bounties",
						"award_bounty",
						AwardBounty {
							bounty_id,
							beneficiary,
						},
						[
							7u8, 205u8, 73u8, 45u8, 57u8, 8u8, 24u8, 135u8, 89u8, 157u8, 35u8,
							176u8, 224u8, 106u8, 167u8, 232u8, 230u8, 153u8, 239u8, 45u8, 210u8,
							61u8, 17u8, 106u8, 220u8, 131u8, 105u8, 136u8, 232u8, 194u8, 243u8,
							48u8,
						],
					)
				}

				#[doc = "See [`Pallet::claim_bounty`]."]
				pub fn claim_bounty(
					&self,
					bounty_id: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ClaimBounty> {
					::subxt::tx::StaticTxPayload::new(
						"Bounties",
						"claim_bounty",
						ClaimBounty { bounty_id },
						[
							102u8, 95u8, 8u8, 89u8, 4u8, 126u8, 189u8, 28u8, 241u8, 16u8, 125u8,
							218u8, 42u8, 92u8, 177u8, 91u8, 8u8, 235u8, 33u8, 48u8, 64u8, 115u8,
							177u8, 95u8, 242u8, 97u8, 181u8, 50u8, 68u8, 37u8, 59u8, 85u8,
						],
					)
				}

				#[doc = "See [`Pallet::close_bounty`]."]
				pub fn close_bounty(
					&self,
					bounty_id: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<CloseBounty> {
					::subxt::tx::StaticTxPayload::new(
						"Bounties",
						"close_bounty",
						CloseBounty { bounty_id },
						[
							64u8, 113u8, 151u8, 228u8, 90u8, 55u8, 251u8, 63u8, 27u8, 211u8, 119u8,
							229u8, 137u8, 137u8, 183u8, 240u8, 241u8, 146u8, 69u8, 169u8, 124u8,
							220u8, 236u8, 111u8, 98u8, 188u8, 100u8, 52u8, 127u8, 245u8, 244u8,
							92u8,
						],
					)
				}

				#[doc = "See [`Pallet::extend_bounty_expiry`]."]
				pub fn extend_bounty_expiry(
					&self,
					bounty_id: ::core::primitive::u32,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<ExtendBountyExpiry> {
					::subxt::tx::StaticTxPayload::new(
						"Bounties",
						"extend_bounty_expiry",
						ExtendBountyExpiry { bounty_id, remark },
						[
							97u8, 69u8, 157u8, 39u8, 59u8, 72u8, 79u8, 88u8, 104u8, 119u8, 91u8,
							26u8, 73u8, 216u8, 174u8, 95u8, 254u8, 214u8, 63u8, 138u8, 100u8,
							112u8, 185u8, 81u8, 159u8, 247u8, 221u8, 60u8, 87u8, 40u8, 80u8, 202u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_bounties::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "New bounty proposal."]
			pub struct BountyProposed {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for BountyProposed {
				const EVENT: &'static str = "BountyProposed";
				const PALLET: &'static str = "Bounties";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A bounty proposal was rejected; funds were slashed."]
			pub struct BountyRejected {
				pub index: ::core::primitive::u32,
				pub bond: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for BountyRejected {
				const EVENT: &'static str = "BountyRejected";
				const PALLET: &'static str = "Bounties";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A bounty proposal is funded and became active."]
			pub struct BountyBecameActive {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for BountyBecameActive {
				const EVENT: &'static str = "BountyBecameActive";
				const PALLET: &'static str = "Bounties";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A bounty is awarded to a beneficiary."]
			pub struct BountyAwarded {
				pub index: ::core::primitive::u32,
				pub beneficiary: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for BountyAwarded {
				const EVENT: &'static str = "BountyAwarded";
				const PALLET: &'static str = "Bounties";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A bounty is claimed by beneficiary."]
			pub struct BountyClaimed {
				pub index: ::core::primitive::u32,
				pub payout: ::core::primitive::u128,
				pub beneficiary: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for BountyClaimed {
				const EVENT: &'static str = "BountyClaimed";
				const PALLET: &'static str = "Bounties";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A bounty is cancelled."]
			pub struct BountyCanceled {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for BountyCanceled {
				const EVENT: &'static str = "BountyCanceled";
				const PALLET: &'static str = "Bounties";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A bounty expiry is extended."]
			pub struct BountyExtended {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for BountyExtended {
				const EVENT: &'static str = "BountyExtended";
				const PALLET: &'static str = "Bounties";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Number of bounty proposals that have been made."]
				pub fn bounty_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Bounties",
						"BountyCount",
						vec![],
						[
							5u8, 188u8, 134u8, 220u8, 64u8, 49u8, 188u8, 98u8, 185u8, 186u8, 230u8,
							65u8, 247u8, 199u8, 28u8, 178u8, 202u8, 193u8, 41u8, 83u8, 115u8,
							253u8, 182u8, 123u8, 92u8, 138u8, 12u8, 31u8, 31u8, 213u8, 23u8, 118u8,
						],
					)
				}

				#[doc = " Bounties that have been made."]
				pub fn bounties(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_bounties::Bounty<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Bounties",
						"Bounties",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							111u8, 149u8, 33u8, 54u8, 172u8, 143u8, 41u8, 231u8, 184u8, 255u8,
							238u8, 206u8, 87u8, 142u8, 84u8, 10u8, 236u8, 141u8, 190u8, 193u8,
							72u8, 170u8, 19u8, 110u8, 135u8, 136u8, 220u8, 11u8, 99u8, 126u8,
							225u8, 208u8,
						],
					)
				}

				#[doc = " Bounties that have been made."]
				pub fn bounties_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_bounties::Bounty<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
							::core::primitive::u32,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Bounties",
						"Bounties",
						Vec::new(),
						[
							111u8, 149u8, 33u8, 54u8, 172u8, 143u8, 41u8, 231u8, 184u8, 255u8,
							238u8, 206u8, 87u8, 142u8, 84u8, 10u8, 236u8, 141u8, 190u8, 193u8,
							72u8, 170u8, 19u8, 110u8, 135u8, 136u8, 220u8, 11u8, 99u8, 126u8,
							225u8, 208u8,
						],
					)
				}

				#[doc = " The description of each bounty."]
				pub fn bounty_descriptions(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Bounties",
						"BountyDescriptions",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							252u8, 0u8, 9u8, 225u8, 13u8, 135u8, 7u8, 121u8, 154u8, 155u8, 116u8,
							83u8, 160u8, 37u8, 72u8, 11u8, 72u8, 0u8, 248u8, 73u8, 158u8, 84u8,
							125u8, 221u8, 176u8, 231u8, 100u8, 239u8, 111u8, 22u8, 29u8, 13u8,
						],
					)
				}

				#[doc = " The description of each bounty."]
				pub fn bounty_descriptions_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Bounties",
						"BountyDescriptions",
						Vec::new(),
						[
							252u8, 0u8, 9u8, 225u8, 13u8, 135u8, 7u8, 121u8, 154u8, 155u8, 116u8,
							83u8, 160u8, 37u8, 72u8, 11u8, 72u8, 0u8, 248u8, 73u8, 158u8, 84u8,
							125u8, 221u8, 176u8, 231u8, 100u8, 239u8, 111u8, 22u8, 29u8, 13u8,
						],
					)
				}

				#[doc = " Bounty indices that have been approved but not yet funded."]
				pub fn bounty_approvals(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Bounties",
						"BountyApprovals",
						vec![],
						[
							64u8, 93u8, 54u8, 94u8, 122u8, 9u8, 246u8, 86u8, 234u8, 30u8, 125u8,
							132u8, 49u8, 128u8, 1u8, 219u8, 241u8, 13u8, 217u8, 186u8, 48u8, 21u8,
							5u8, 227u8, 71u8, 157u8, 128u8, 226u8, 214u8, 49u8, 249u8, 183u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The amount held on deposit for placing a bounty proposal."]
				pub fn bounty_deposit_base(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Bounties",
						"BountyDepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}

				#[doc = " The delay period for which a bounty beneficiary need to wait before claim the payout."]
				pub fn bounty_deposit_payout_delay(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Bounties",
						"BountyDepositPayoutDelay",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " Bounty duration in blocks."]
				pub fn bounty_update_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Bounties",
						"BountyUpdatePeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " The curator deposit is calculated as a percentage of the curator fee."]
				#[doc = ""]
				#[doc = " This deposit has optional upper and lower bounds with `CuratorDepositMax` and"]
				#[doc = " `CuratorDepositMin`."]
				pub fn curator_deposit_multiplier(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::per_things::Permill,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Bounties",
						"CuratorDepositMultiplier",
						[
							225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
							80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
							177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
						],
					)
				}

				#[doc = " Maximum amount of funds that should be placed in a deposit for making a proposal."]
				pub fn curator_deposit_max(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						::core::option::Option<::core::primitive::u128>,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Bounties",
						"CuratorDepositMax",
						[
							84u8, 154u8, 218u8, 83u8, 84u8, 189u8, 32u8, 20u8, 120u8, 194u8, 88u8,
							205u8, 109u8, 216u8, 114u8, 193u8, 120u8, 198u8, 154u8, 237u8, 134u8,
							204u8, 102u8, 247u8, 52u8, 103u8, 231u8, 43u8, 243u8, 122u8, 60u8,
							216u8,
						],
					)
				}

				#[doc = " Minimum amount of funds that should be placed in a deposit for making a proposal."]
				pub fn curator_deposit_min(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						::core::option::Option<::core::primitive::u128>,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Bounties",
						"CuratorDepositMin",
						[
							84u8, 154u8, 218u8, 83u8, 84u8, 189u8, 32u8, 20u8, 120u8, 194u8, 88u8,
							205u8, 109u8, 216u8, 114u8, 193u8, 120u8, 198u8, 154u8, 237u8, 134u8,
							204u8, 102u8, 247u8, 52u8, 103u8, 231u8, 43u8, 243u8, 122u8, 60u8,
							216u8,
						],
					)
				}

				#[doc = " Minimum value for a bounty."]
				pub fn bounty_value_minimum(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Bounties",
						"BountyValueMinimum",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}

				#[doc = " The amount held on deposit per byte within the tip report reason or bounty description."]
				pub fn data_deposit_per_byte(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Bounties",
						"DataDepositPerByte",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}

				#[doc = " Maximum acceptable reason length."]
				#[doc = ""]
				#[doc = " Benchmarks depend on this value, be sure to update weights file when changing this value"]
				pub fn maximum_reason_length(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Bounties",
						"MaximumReasonLength",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod tips {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ReportAwesome {
				pub reason: ::std::vec::Vec<::core::primitive::u8>,
				pub who: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct RetractTip {
				pub hash: ::subxt::ext::sp_core::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct TipNew {
				pub reason: ::std::vec::Vec<::core::primitive::u8>,
				pub who: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub tip_value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Tip {
				pub hash: ::subxt::ext::sp_core::H256,
				#[codec(compact)]
				pub tip_value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct CloseTip {
				pub hash: ::subxt::ext::sp_core::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SlashTip {
				pub hash: ::subxt::ext::sp_core::H256,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::report_awesome`]."]
				pub fn report_awesome(
					&self,
					reason: ::std::vec::Vec<::core::primitive::u8>,
					who: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<ReportAwesome> {
					::subxt::tx::StaticTxPayload::new(
						"Tips",
						"report_awesome",
						ReportAwesome { reason, who },
						[
							234u8, 199u8, 253u8, 109u8, 105u8, 252u8, 249u8, 138u8, 197u8, 187u8,
							226u8, 67u8, 195u8, 51u8, 197u8, 85u8, 110u8, 173u8, 85u8, 176u8, 27u8,
							147u8, 221u8, 85u8, 177u8, 55u8, 2u8, 218u8, 4u8, 115u8, 67u8, 121u8,
						],
					)
				}

				#[doc = "See [`Pallet::retract_tip`]."]
				pub fn retract_tip(
					&self,
					hash: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<RetractTip> {
					::subxt::tx::StaticTxPayload::new("Tips", "retract_tip", RetractTip { hash }, [
						137u8, 42u8, 229u8, 188u8, 157u8, 195u8, 184u8, 176u8, 64u8, 142u8, 67u8,
						175u8, 185u8, 207u8, 214u8, 71u8, 165u8, 29u8, 137u8, 227u8, 132u8, 195u8,
						255u8, 66u8, 186u8, 57u8, 34u8, 184u8, 187u8, 65u8, 129u8, 131u8,
					])
				}

				#[doc = "See [`Pallet::tip_new`]."]
				pub fn tip_new(
					&self,
					reason: ::std::vec::Vec<::core::primitive::u8>,
					who: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					tip_value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<TipNew> {
					::subxt::tx::StaticTxPayload::new(
						"Tips",
						"tip_new",
						TipNew {
							reason,
							who,
							tip_value,
						},
						[
							166u8, 19u8, 234u8, 170u8, 180u8, 74u8, 235u8, 156u8, 58u8, 125u8,
							199u8, 215u8, 15u8, 96u8, 240u8, 71u8, 205u8, 61u8, 60u8, 204u8, 12u8,
							35u8, 252u8, 233u8, 31u8, 86u8, 204u8, 230u8, 40u8, 247u8, 47u8, 135u8,
						],
					)
				}

				#[doc = "See [`Pallet::tip`]."]
				pub fn tip(
					&self,
					hash: ::subxt::ext::sp_core::H256,
					tip_value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Tip> {
					::subxt::tx::StaticTxPayload::new("Tips", "tip", Tip { hash, tip_value }, [
						133u8, 52u8, 131u8, 14u8, 71u8, 232u8, 254u8, 31u8, 33u8, 206u8, 50u8,
						76u8, 56u8, 167u8, 228u8, 202u8, 195u8, 0u8, 164u8, 107u8, 170u8, 98u8,
						192u8, 37u8, 209u8, 199u8, 130u8, 15u8, 168u8, 63u8, 181u8, 134u8,
					])
				}

				#[doc = "See [`Pallet::close_tip`]."]
				pub fn close_tip(
					&self,
					hash: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<CloseTip> {
					::subxt::tx::StaticTxPayload::new("Tips", "close_tip", CloseTip { hash }, [
						32u8, 53u8, 0u8, 222u8, 45u8, 157u8, 107u8, 174u8, 203u8, 50u8, 81u8,
						230u8, 6u8, 111u8, 79u8, 55u8, 49u8, 151u8, 107u8, 114u8, 81u8, 200u8,
						144u8, 175u8, 29u8, 142u8, 115u8, 184u8, 102u8, 116u8, 156u8, 173u8,
					])
				}

				#[doc = "See [`Pallet::slash_tip`]."]
				pub fn slash_tip(
					&self,
					hash: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<SlashTip> {
					::subxt::tx::StaticTxPayload::new("Tips", "slash_tip", SlashTip { hash }, [
						222u8, 209u8, 22u8, 47u8, 114u8, 230u8, 81u8, 200u8, 131u8, 0u8, 209u8,
						54u8, 17u8, 200u8, 175u8, 125u8, 100u8, 254u8, 41u8, 178u8, 20u8, 27u8,
						9u8, 184u8, 79u8, 93u8, 208u8, 148u8, 27u8, 190u8, 176u8, 169u8,
					])
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_tips::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A new tip suggestion has been opened."]
			pub struct NewTip {
				pub tip_hash: ::subxt::ext::sp_core::H256,
			}
			impl ::subxt::events::StaticEvent for NewTip {
				const EVENT: &'static str = "NewTip";
				const PALLET: &'static str = "Tips";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A tip suggestion has reached threshold and is closing."]
			pub struct TipClosing {
				pub tip_hash: ::subxt::ext::sp_core::H256,
			}
			impl ::subxt::events::StaticEvent for TipClosing {
				const EVENT: &'static str = "TipClosing";
				const PALLET: &'static str = "Tips";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A tip suggestion has been closed."]
			pub struct TipClosed {
				pub tip_hash: ::subxt::ext::sp_core::H256,
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub payout: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TipClosed {
				const EVENT: &'static str = "TipClosed";
				const PALLET: &'static str = "Tips";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A tip suggestion has been retracted."]
			pub struct TipRetracted {
				pub tip_hash: ::subxt::ext::sp_core::H256,
			}
			impl ::subxt::events::StaticEvent for TipRetracted {
				const EVENT: &'static str = "TipRetracted";
				const PALLET: &'static str = "Tips";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A tip suggestion has been slashed."]
			pub struct TipSlashed {
				pub tip_hash: ::subxt::ext::sp_core::H256,
				pub finder: ::subxt::ext::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TipSlashed {
				const EVENT: &'static str = "TipSlashed";
				const PALLET: &'static str = "Tips";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " TipsMap that are not yet completed. Keyed by the hash of `(reason, who)` from the value."]
				#[doc = " This has the insecure enumerable hash function since the key itself is already"]
				#[doc = " guaranteed to be a secure hash."]
				pub fn tips(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_tips::OpenTip<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
							::core::primitive::u32,
							::subxt::ext::sp_core::H256,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Tips",
						"Tips",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							241u8, 196u8, 105u8, 248u8, 29u8, 66u8, 86u8, 98u8, 6u8, 159u8, 191u8,
							0u8, 227u8, 232u8, 147u8, 248u8, 173u8, 20u8, 225u8, 12u8, 232u8, 5u8,
							93u8, 78u8, 18u8, 154u8, 130u8, 38u8, 142u8, 36u8, 66u8, 0u8,
						],
					)
				}

				#[doc = " TipsMap that are not yet completed. Keyed by the hash of `(reason, who)` from the value."]
				#[doc = " This has the insecure enumerable hash function since the key itself is already"]
				#[doc = " guaranteed to be a secure hash."]
				pub fn tips_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_tips::OpenTip<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
							::core::primitive::u32,
							::subxt::ext::sp_core::H256,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Tips",
						"Tips",
						Vec::new(),
						[
							241u8, 196u8, 105u8, 248u8, 29u8, 66u8, 86u8, 98u8, 6u8, 159u8, 191u8,
							0u8, 227u8, 232u8, 147u8, 248u8, 173u8, 20u8, 225u8, 12u8, 232u8, 5u8,
							93u8, 78u8, 18u8, 154u8, 130u8, 38u8, 142u8, 36u8, 66u8, 0u8,
						],
					)
				}

				#[doc = " Simple preimage lookup from the reason's hash to the original data. Again, has an"]
				#[doc = " insecure enumerable hash since the key is guaranteed to be the result of a secure hash."]
				pub fn reasons(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Tips",
						"Reasons",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							202u8, 191u8, 36u8, 162u8, 156u8, 102u8, 115u8, 10u8, 203u8, 35u8,
							201u8, 70u8, 195u8, 151u8, 89u8, 82u8, 202u8, 35u8, 210u8, 176u8, 82u8,
							1u8, 77u8, 94u8, 31u8, 70u8, 252u8, 194u8, 166u8, 91u8, 189u8, 134u8,
						],
					)
				}

				#[doc = " Simple preimage lookup from the reason's hash to the original data. Again, has an"]
				#[doc = " insecure enumerable hash since the key is guaranteed to be the result of a secure hash."]
				pub fn reasons_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Tips",
						"Reasons",
						Vec::new(),
						[
							202u8, 191u8, 36u8, 162u8, 156u8, 102u8, 115u8, 10u8, 203u8, 35u8,
							201u8, 70u8, 195u8, 151u8, 89u8, 82u8, 202u8, 35u8, 210u8, 176u8, 82u8,
							1u8, 77u8, 94u8, 31u8, 70u8, 252u8, 194u8, 166u8, 91u8, 189u8, 134u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Maximum acceptable reason length."]
				#[doc = ""]
				#[doc = " Benchmarks depend on this value, be sure to update weights file when changing this value"]
				pub fn maximum_reason_length(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Tips", "MaximumReasonLength", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " The amount held on deposit per byte within the tip report reason or bounty description."]
				pub fn data_deposit_per_byte(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new("Tips", "DataDepositPerByte", [
						84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
						27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
						71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
					])
				}

				#[doc = " The period for which a tip remains open after is has achieved threshold tippers."]
				pub fn tip_countdown(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Tips", "TipCountdown", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " The percent of the final tip which goes to the original reporter of the tip."]
				pub fn tip_finders_fee(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::per_things::Percent,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new("Tips", "TipFindersFee", [
						99u8, 121u8, 176u8, 172u8, 235u8, 159u8, 116u8, 114u8, 179u8, 91u8, 129u8,
						117u8, 204u8, 135u8, 53u8, 7u8, 151u8, 26u8, 124u8, 151u8, 202u8, 171u8,
						171u8, 207u8, 183u8, 177u8, 24u8, 53u8, 109u8, 185u8, 71u8, 183u8,
					])
				}

				#[doc = " The amount held on deposit for placing a tip report."]
				pub fn tip_report_deposit_base(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Tips",
						"TipReportDepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
			}
		}
	}
	pub mod mmr {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Latest MMR Root hash."]
				pub fn root_hash(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Mmr",
						"RootHash",
						vec![],
						[
							182u8, 163u8, 37u8, 44u8, 2u8, 163u8, 57u8, 184u8, 97u8, 55u8, 1u8,
							116u8, 55u8, 169u8, 23u8, 221u8, 182u8, 5u8, 174u8, 217u8, 111u8, 55u8,
							180u8, 161u8, 69u8, 120u8, 212u8, 73u8, 2u8, 1u8, 39u8, 224u8,
						],
					)
				}

				#[doc = " Current size of the MMR (number of leaves)."]
				pub fn number_of_leaves(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Mmr",
						"NumberOfLeaves",
						vec![],
						[
							138u8, 124u8, 23u8, 186u8, 255u8, 231u8, 187u8, 122u8, 213u8, 160u8,
							29u8, 24u8, 88u8, 98u8, 171u8, 36u8, 195u8, 216u8, 27u8, 190u8, 192u8,
							152u8, 8u8, 13u8, 210u8, 232u8, 45u8, 184u8, 240u8, 255u8, 156u8,
							204u8,
						],
					)
				}

				#[doc = " Hashes of the nodes in the MMR."]
				#[doc = ""]
				#[doc = " Note this collection only contains MMR peaks, the inner nodes (and leaves)"]
				#[doc = " are pruned and only stored in the Offchain DB."]
				pub fn nodes(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Mmr",
						"Nodes",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							188u8, 148u8, 126u8, 226u8, 142u8, 91u8, 61u8, 52u8, 213u8, 36u8,
							120u8, 232u8, 20u8, 11u8, 61u8, 1u8, 130u8, 155u8, 81u8, 34u8, 153u8,
							149u8, 210u8, 232u8, 113u8, 242u8, 249u8, 8u8, 61u8, 51u8, 148u8, 98u8,
						],
					)
				}

				#[doc = " Hashes of the nodes in the MMR."]
				#[doc = ""]
				#[doc = " Note this collection only contains MMR peaks, the inner nodes (and leaves)"]
				#[doc = " are pruned and only stored in the Offchain DB."]
				pub fn nodes_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Mmr",
						"Nodes",
						Vec::new(),
						[
							188u8, 148u8, 126u8, 226u8, 142u8, 91u8, 61u8, 52u8, 213u8, 36u8,
							120u8, 232u8, 20u8, 11u8, 61u8, 1u8, 130u8, 155u8, 81u8, 34u8, 153u8,
							149u8, 210u8, 232u8, 113u8, 242u8, 249u8, 8u8, 61u8, 51u8, 148u8, 98u8,
						],
					)
				}
			}
		}
	}
	pub mod data_availability {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct CreateApplicationKey {
				pub key: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					::core::primitive::u8,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SubmitData {
				pub data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					::core::primitive::u8,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SubmitBlockLengthProposal {
				pub rows: ::core::primitive::u32,
				pub cols: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::create_application_key`]."]
				pub fn create_application_key(
					&self,
					key: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				) -> ::subxt::tx::StaticTxPayload<CreateApplicationKey> {
					::subxt::tx::StaticTxPayload::new(
						"DataAvailability",
						"create_application_key",
						CreateApplicationKey { key },
						[
							72u8, 173u8, 240u8, 89u8, 218u8, 104u8, 43u8, 227u8, 167u8, 173u8,
							58u8, 254u8, 243u8, 117u8, 221u8, 121u8, 163u8, 205u8, 31u8, 35u8,
							239u8, 53u8, 148u8, 250u8, 48u8, 81u8, 46u8, 121u8, 72u8, 23u8, 236u8,
							6u8,
						],
					)
				}

				#[doc = "See [`Pallet::submit_data`]."]
				pub fn submit_data(
					&self,
					data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				) -> ::subxt::tx::StaticTxPayload<SubmitData> {
					::subxt::tx::StaticTxPayload::new(
						"DataAvailability",
						"submit_data",
						SubmitData { data },
						[
							154u8, 109u8, 83u8, 128u8, 67u8, 33u8, 87u8, 165u8, 135u8, 119u8,
							223u8, 220u8, 107u8, 18u8, 81u8, 65u8, 241u8, 213u8, 198u8, 110u8,
							175u8, 25u8, 6u8, 188u8, 157u8, 85u8, 213u8, 133u8, 235u8, 240u8, 32u8,
							76u8,
						],
					)
				}

				#[doc = "See [`Pallet::submit_block_length_proposal`]."]
				pub fn submit_block_length_proposal(
					&self,
					rows: ::core::primitive::u32,
					cols: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SubmitBlockLengthProposal> {
					::subxt::tx::StaticTxPayload::new(
						"DataAvailability",
						"submit_block_length_proposal",
						SubmitBlockLengthProposal { rows, cols },
						[
							97u8, 1u8, 244u8, 28u8, 180u8, 49u8, 28u8, 9u8, 9u8, 215u8, 184u8,
							200u8, 14u8, 77u8, 21u8, 190u8, 176u8, 8u8, 12u8, 83u8, 176u8, 95u8,
							149u8, 84u8, 129u8, 111u8, 181u8, 236u8, 77u8, 62u8, 115u8, 153u8,
						],
					)
				}
			}
		}
		#[doc = "Event for the pallet."]
		pub type Event = runtime_types::da_control::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A new application key was created."]
			pub struct ApplicationKeyCreated {
				pub key: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					::core::primitive::u8,
				>,
				pub owner: ::subxt::ext::sp_core::crypto::AccountId32,
				pub id: runtime_types::avail_core::AppId,
			}
			impl ::subxt::events::StaticEvent for ApplicationKeyCreated {
				const EVENT: &'static str = "ApplicationKeyCreated";
				const PALLET: &'static str = "DataAvailability";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct DataSubmitted {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					::core::primitive::u8,
				>,
			}
			impl ::subxt::events::StaticEvent for DataSubmitted {
				const EVENT: &'static str = "DataSubmitted";
				const PALLET: &'static str = "DataAvailability";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct BlockLengthProposalSubmitted {
				pub rows: runtime_types::avail_core::BlockLengthRows,
				pub cols: runtime_types::avail_core::BlockLengthColumns,
			}
			impl ::subxt::events::StaticEvent for BlockLengthProposalSubmitted {
				const EVENT: &'static str = "BlockLengthProposalSubmitted";
				const PALLET: &'static str = "DataAvailability";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Last application ID"]
				pub fn next_app_id(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::avail_core::AppId>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"DataAvailability",
						"NextAppId",
						vec![],
						[
							21u8, 88u8, 250u8, 59u8, 123u8, 113u8, 217u8, 137u8, 130u8, 207u8,
							168u8, 182u8, 200u8, 84u8, 110u8, 250u8, 109u8, 163u8, 72u8, 199u8,
							88u8, 145u8, 217u8, 67u8, 49u8, 107u8, 26u8, 52u8, 76u8, 69u8, 4u8,
							63u8,
						],
					)
				}

				#[doc = " Store all application keys."]
				pub fn app_keys(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::da_control::pallet::AppKeyInfo<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"DataAvailability",
						"AppKeys",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							167u8, 153u8, 205u8, 126u8, 104u8, 82u8, 243u8, 1u8, 106u8, 10u8, 85u8,
							118u8, 236u8, 46u8, 81u8, 144u8, 229u8, 68u8, 89u8, 82u8, 98u8, 91u8,
							0u8, 66u8, 102u8, 96u8, 163u8, 244u8, 47u8, 169u8, 237u8, 250u8,
						],
					)
				}

				#[doc = " Store all application keys."]
				pub fn app_keys_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::da_control::pallet::AppKeyInfo<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"DataAvailability",
						"AppKeys",
						Vec::new(),
						[
							167u8, 153u8, 205u8, 126u8, 104u8, 82u8, 243u8, 1u8, 106u8, 10u8, 85u8,
							118u8, 236u8, 46u8, 81u8, 144u8, 229u8, 68u8, 89u8, 82u8, 98u8, 91u8,
							0u8, 66u8, 102u8, 96u8, 163u8, 244u8, 47u8, 169u8, 237u8, 250u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The max length of application key."]
				pub fn max_app_key_length(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"DataAvailability",
						"MaxAppKeyLength",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " The max length of app data."]
				pub fn max_app_data_length(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"DataAvailability",
						"MaxAppDataLength",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " Minimum number of rows in a block."]
				pub fn min_block_rows(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::avail_core::BlockLengthRows>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"DataAvailability",
						"MinBlockRows",
						[
							240u8, 185u8, 129u8, 113u8, 97u8, 167u8, 124u8, 171u8, 245u8, 13u8,
							246u8, 129u8, 19u8, 85u8, 91u8, 215u8, 57u8, 41u8, 249u8, 219u8, 130u8,
							113u8, 255u8, 83u8, 254u8, 106u8, 212u8, 187u8, 160u8, 46u8, 32u8,
							61u8,
						],
					)
				}

				#[doc = " Maximum number of rows in a block."]
				pub fn max_block_rows(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::avail_core::BlockLengthRows>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"DataAvailability",
						"MaxBlockRows",
						[
							240u8, 185u8, 129u8, 113u8, 97u8, 167u8, 124u8, 171u8, 245u8, 13u8,
							246u8, 129u8, 19u8, 85u8, 91u8, 215u8, 57u8, 41u8, 249u8, 219u8, 130u8,
							113u8, 255u8, 83u8, 254u8, 106u8, 212u8, 187u8, 160u8, 46u8, 32u8,
							61u8,
						],
					)
				}

				#[doc = " Minimum number of cols in a block."]
				pub fn min_block_cols(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::avail_core::BlockLengthColumns,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"DataAvailability",
						"MinBlockCols",
						[
							240u8, 185u8, 129u8, 113u8, 97u8, 167u8, 124u8, 171u8, 245u8, 13u8,
							246u8, 129u8, 19u8, 85u8, 91u8, 215u8, 57u8, 41u8, 249u8, 219u8, 130u8,
							113u8, 255u8, 83u8, 254u8, 106u8, 212u8, 187u8, 160u8, 46u8, 32u8,
							61u8,
						],
					)
				}

				#[doc = " Maximum number of cols in a block."]
				pub fn max_block_cols(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::avail_core::BlockLengthColumns,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"DataAvailability",
						"MaxBlockCols",
						[
							240u8, 185u8, 129u8, 113u8, 97u8, 167u8, 124u8, 171u8, 245u8, 13u8,
							246u8, 129u8, 19u8, 85u8, 91u8, 215u8, 57u8, 41u8, 249u8, 219u8, 130u8,
							113u8, 255u8, 83u8, 254u8, 106u8, 212u8, 187u8, 160u8, 46u8, 32u8,
							61u8,
						],
					)
				}
			}
		}
	}
	pub mod nomad_updater_manager {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::nomad_updater_manager::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct NewUpdater {
				pub old_updater: ::subxt::ext::sp_core::H160,
				pub new_updater: ::subxt::ext::sp_core::H160,
			}
			impl ::subxt::events::StaticEvent for NewUpdater {
				const EVENT: &'static str = "NewUpdater";
				const PALLET: &'static str = "NomadUpdaterManager";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct FakeSlashed {
				pub reporter: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for FakeSlashed {
				const EVENT: &'static str = "FakeSlashed";
				const PALLET: &'static str = "NomadUpdaterManager";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn updater(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H160>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NomadUpdaterManager",
						"Updater",
						vec![],
						[
							133u8, 141u8, 193u8, 158u8, 124u8, 101u8, 100u8, 156u8, 48u8, 193u8,
							106u8, 83u8, 240u8, 237u8, 242u8, 44u8, 26u8, 206u8, 213u8, 106u8,
							159u8, 239u8, 185u8, 213u8, 142u8, 26u8, 244u8, 234u8, 130u8, 187u8,
							104u8, 154u8,
						],
					)
				}
			}
		}
	}
	pub mod nomad_home {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Dispatch {
				#[codec(compact)]
				pub destination_domain: ::core::primitive::u32,
				pub recipient_address: ::subxt::ext::sp_core::H256,
				pub message_body: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					::core::primitive::u8,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Update {
				pub signed_update: runtime_types::nomad_core::update::SignedUpdate,
				#[codec(compact)]
				pub max_index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ImproperUpdate {
				pub signed_update: runtime_types::nomad_core::update::SignedUpdate,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetUpdater {
				pub new_updater: ::subxt::ext::sp_core::H160,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::dispatch`]."]
				pub fn dispatch(
					&self,
					destination_domain: ::core::primitive::u32,
					recipient_address: ::subxt::ext::sp_core::H256,
					message_body: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				) -> ::subxt::tx::StaticTxPayload<Dispatch> {
					::subxt::tx::StaticTxPayload::new(
						"NomadHome",
						"dispatch",
						Dispatch {
							destination_domain,
							recipient_address,
							message_body,
						},
						[
							254u8, 10u8, 145u8, 46u8, 242u8, 218u8, 11u8, 43u8, 209u8, 67u8, 34u8,
							219u8, 211u8, 119u8, 183u8, 61u8, 181u8, 163u8, 136u8, 92u8, 23u8,
							24u8, 247u8, 142u8, 146u8, 138u8, 115u8, 150u8, 55u8, 121u8, 55u8,
							75u8,
						],
					)
				}

				#[doc = "See [`Pallet::update`]."]
				pub fn update(
					&self,
					signed_update: runtime_types::nomad_core::update::SignedUpdate,
					max_index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Update> {
					::subxt::tx::StaticTxPayload::new(
						"NomadHome",
						"update",
						Update {
							signed_update,
							max_index,
						},
						[
							114u8, 14u8, 19u8, 186u8, 185u8, 155u8, 190u8, 199u8, 108u8, 207u8,
							138u8, 45u8, 149u8, 190u8, 241u8, 86u8, 73u8, 152u8, 118u8, 97u8, 45u8,
							68u8, 190u8, 141u8, 191u8, 24u8, 86u8, 147u8, 133u8, 160u8, 129u8,
							175u8,
						],
					)
				}

				#[doc = "See [`Pallet::improper_update`]."]
				pub fn improper_update(
					&self,
					signed_update: runtime_types::nomad_core::update::SignedUpdate,
				) -> ::subxt::tx::StaticTxPayload<ImproperUpdate> {
					::subxt::tx::StaticTxPayload::new(
						"NomadHome",
						"improper_update",
						ImproperUpdate { signed_update },
						[
							235u8, 105u8, 81u8, 68u8, 152u8, 117u8, 164u8, 91u8, 92u8, 240u8,
							134u8, 122u8, 79u8, 100u8, 163u8, 221u8, 80u8, 144u8, 193u8, 112u8,
							141u8, 202u8, 228u8, 227u8, 136u8, 4u8, 233u8, 100u8, 181u8, 22u8,
							154u8, 54u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_updater`]."]
				pub fn set_updater(
					&self,
					new_updater: ::subxt::ext::sp_core::H160,
				) -> ::subxt::tx::StaticTxPayload<SetUpdater> {
					::subxt::tx::StaticTxPayload::new(
						"NomadHome",
						"set_updater",
						SetUpdater { new_updater },
						[
							242u8, 210u8, 5u8, 118u8, 160u8, 203u8, 74u8, 59u8, 187u8, 198u8,
							240u8, 40u8, 91u8, 106u8, 103u8, 54u8, 223u8, 138u8, 29u8, 231u8, 91u8,
							179u8, 187u8, 183u8, 247u8, 249u8, 27u8, 15u8, 138u8, 245u8, 153u8,
							22u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::nomad_home::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Dispatch {
				pub message_hash: ::subxt::ext::sp_core::H256,
				pub leaf_index: ::core::primitive::u32,
				pub destination_and_nonce: ::core::primitive::u64,
				pub committed_root: ::subxt::ext::sp_core::H256,
				pub message: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for Dispatch {
				const EVENT: &'static str = "Dispatch";
				const PALLET: &'static str = "NomadHome";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Update {
				pub home_domain: ::core::primitive::u32,
				pub previous_root: ::subxt::ext::sp_core::H256,
				pub new_root: ::subxt::ext::sp_core::H256,
				pub signature: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for Update {
				const EVENT: &'static str = "Update";
				const PALLET: &'static str = "NomadHome";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ImproperUpdate {
				pub previous_root: ::subxt::ext::sp_core::H256,
				pub new_root: ::subxt::ext::sp_core::H256,
				pub signature: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for ImproperUpdate {
				const EVENT: &'static str = "ImproperUpdate";
				const PALLET: &'static str = "NomadHome";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct UpdaterSlashed {
				pub updater: ::subxt::ext::sp_core::H160,
				pub reporter: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for UpdaterSlashed {
				const EVENT: &'static str = "UpdaterSlashed";
				const PALLET: &'static str = "NomadHome";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn base(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::nomad_base::NomadBase>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NomadHome",
						"Base",
						vec![],
						[
							205u8, 101u8, 115u8, 25u8, 58u8, 208u8, 131u8, 248u8, 249u8, 168u8,
							248u8, 25u8, 217u8, 233u8, 102u8, 189u8, 222u8, 72u8, 63u8, 92u8,
							134u8, 136u8, 22u8, 12u8, 184u8, 242u8, 183u8, 52u8, 154u8, 87u8, 15u8,
							52u8,
						],
					)
				}

				pub fn tree(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::nomad_merkle::light::LightMerkle,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NomadHome",
						"Tree",
						vec![],
						[
							22u8, 207u8, 157u8, 183u8, 36u8, 47u8, 192u8, 247u8, 85u8, 143u8, 23u8,
							21u8, 189u8, 221u8, 230u8, 215u8, 141u8, 38u8, 233u8, 65u8, 165u8,
							57u8, 14u8, 252u8, 185u8, 140u8, 204u8, 186u8, 205u8, 195u8, 214u8,
							235u8,
						],
					)
				}

				pub fn nonces(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NomadHome",
						"Nonces",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							225u8, 196u8, 0u8, 213u8, 118u8, 3u8, 219u8, 130u8, 21u8, 64u8, 253u8,
							206u8, 79u8, 103u8, 199u8, 75u8, 182u8, 43u8, 13u8, 130u8, 169u8, 67u8,
							174u8, 208u8, 72u8, 111u8, 50u8, 68u8, 110u8, 138u8, 5u8, 67u8,
						],
					)
				}

				pub fn nonces_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NomadHome",
						"Nonces",
						Vec::new(),
						[
							225u8, 196u8, 0u8, 213u8, 118u8, 3u8, 219u8, 130u8, 21u8, 64u8, 253u8,
							206u8, 79u8, 103u8, 199u8, 75u8, 182u8, 43u8, 13u8, 130u8, 169u8, 67u8,
							174u8, 208u8, 72u8, 111u8, 50u8, 68u8, 110u8, 138u8, 5u8, 67u8,
						],
					)
				}

				pub fn index_to_root(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NomadHome",
						"IndexToRoot",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							55u8, 53u8, 87u8, 227u8, 177u8, 25u8, 61u8, 107u8, 134u8, 232u8, 131u8,
							188u8, 144u8, 138u8, 45u8, 113u8, 17u8, 164u8, 100u8, 233u8, 226u8,
							9u8, 186u8, 184u8, 147u8, 164u8, 34u8, 80u8, 28u8, 212u8, 253u8, 16u8,
						],
					)
				}

				pub fn index_to_root_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NomadHome",
						"IndexToRoot",
						Vec::new(),
						[
							55u8, 53u8, 87u8, 227u8, 177u8, 25u8, 61u8, 107u8, 134u8, 232u8, 131u8,
							188u8, 144u8, 138u8, 45u8, 113u8, 17u8, 164u8, 100u8, 233u8, 226u8,
							9u8, 186u8, 184u8, 147u8, 164u8, 34u8, 80u8, 28u8, 212u8, 253u8, 16u8,
						],
					)
				}

				pub fn root_to_index(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NomadHome",
						"RootToIndex",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							72u8, 228u8, 165u8, 136u8, 129u8, 151u8, 17u8, 28u8, 141u8, 139u8, 7u8,
							133u8, 237u8, 63u8, 219u8, 86u8, 163u8, 26u8, 194u8, 28u8, 112u8,
							111u8, 195u8, 50u8, 122u8, 33u8, 32u8, 193u8, 192u8, 210u8, 233u8,
							91u8,
						],
					)
				}

				pub fn root_to_index_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NomadHome",
						"RootToIndex",
						Vec::new(),
						[
							72u8, 228u8, 165u8, 136u8, 129u8, 151u8, 17u8, 28u8, 141u8, 139u8, 7u8,
							133u8, 237u8, 63u8, 219u8, 86u8, 163u8, 26u8, 194u8, 28u8, 112u8,
							111u8, 195u8, 50u8, 122u8, 33u8, 32u8, 193u8, 192u8, 210u8, 233u8,
							91u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Max allowed message body size"]
				pub fn max_message_body_bytes(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"NomadHome",
						"MaxMessageBodyBytes",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod nomad_da_bridge {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct TryDispatchDataRoot {
				#[codec(compact)]
				pub destination_domain: ::core::primitive::u32,
				pub recipient_address: ::subxt::ext::sp_core::H256,
				pub header: ::std::boxed::Box<
					runtime_types::avail_core::header::Header<
						::core::primitive::u32,
						runtime_types::sp_runtime::traits::BlakeTwo256,
					>,
				>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::try_dispatch_data_root`]."]
				pub fn try_dispatch_data_root(
					&self,
					destination_domain: ::core::primitive::u32,
					recipient_address: ::subxt::ext::sp_core::H256,
					header: runtime_types::avail_core::header::Header<
						::core::primitive::u32,
						runtime_types::sp_runtime::traits::BlakeTwo256,
					>,
				) -> ::subxt::tx::StaticTxPayload<TryDispatchDataRoot> {
					::subxt::tx::StaticTxPayload::new(
						"NomadDABridge",
						"try_dispatch_data_root",
						TryDispatchDataRoot {
							destination_domain,
							recipient_address,
							header: ::std::boxed::Box::new(header),
						},
						[
							190u8, 51u8, 167u8, 216u8, 67u8, 207u8, 45u8, 202u8, 198u8, 90u8,
							165u8, 128u8, 60u8, 206u8, 218u8, 51u8, 148u8, 10u8, 230u8, 102u8,
							168u8, 91u8, 178u8, 242u8, 66u8, 246u8, 214u8, 92u8, 91u8, 152u8,
							210u8, 67u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::nomad_da_bridge::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct DataRootDispatched {
				pub destination_domain: ::core::primitive::u32,
				pub recipient_address: ::subxt::ext::sp_core::H256,
				pub block_number: ::core::primitive::u32,
				pub data_root: ::subxt::ext::sp_core::H256,
			}
			impl ::subxt::events::StaticEvent for DataRootDispatched {
				const EVENT: &'static str = "DataRootDispatched";
				const PALLET: &'static str = "NomadDABridge";
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn da_bridge_pallet_id(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"NomadDABridge",
						"DABridgePalletId",
						[
							167u8, 71u8, 0u8, 47u8, 217u8, 107u8, 29u8, 163u8, 157u8, 187u8, 110u8,
							219u8, 88u8, 213u8, 82u8, 107u8, 46u8, 199u8, 41u8, 110u8, 102u8,
							187u8, 45u8, 201u8, 247u8, 66u8, 33u8, 228u8, 33u8, 99u8, 242u8, 80u8,
						],
					)
				}
			}
		}
	}
	pub mod preimage {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct NotePreimage {
				pub bytes: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct UnnotePreimage {
				pub hash: ::subxt::ext::sp_core::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct RequestPreimage {
				pub hash: ::subxt::ext::sp_core::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct UnrequestPreimage {
				pub hash: ::subxt::ext::sp_core::H256,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::note_preimage`]."]
				pub fn note_preimage(
					&self,
					bytes: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<NotePreimage> {
					::subxt::tx::StaticTxPayload::new(
						"Preimage",
						"note_preimage",
						NotePreimage { bytes },
						[
							77u8, 48u8, 104u8, 3u8, 254u8, 65u8, 106u8, 95u8, 204u8, 89u8, 149u8,
							29u8, 144u8, 188u8, 99u8, 23u8, 146u8, 142u8, 35u8, 17u8, 125u8, 130u8,
							31u8, 206u8, 106u8, 83u8, 163u8, 192u8, 81u8, 23u8, 232u8, 230u8,
						],
					)
				}

				#[doc = "See [`Pallet::unnote_preimage`]."]
				pub fn unnote_preimage(
					&self,
					hash: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<UnnotePreimage> {
					::subxt::tx::StaticTxPayload::new(
						"Preimage",
						"unnote_preimage",
						UnnotePreimage { hash },
						[
							211u8, 204u8, 205u8, 58u8, 33u8, 179u8, 68u8, 74u8, 149u8, 138u8,
							213u8, 45u8, 140u8, 27u8, 106u8, 81u8, 68u8, 212u8, 147u8, 116u8, 27u8,
							130u8, 84u8, 34u8, 231u8, 197u8, 135u8, 8u8, 19u8, 242u8, 207u8, 17u8,
						],
					)
				}

				#[doc = "See [`Pallet::request_preimage`]."]
				pub fn request_preimage(
					&self,
					hash: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<RequestPreimage> {
					::subxt::tx::StaticTxPayload::new(
						"Preimage",
						"request_preimage",
						RequestPreimage { hash },
						[
							195u8, 26u8, 146u8, 255u8, 79u8, 43u8, 73u8, 60u8, 115u8, 78u8, 99u8,
							197u8, 137u8, 95u8, 139u8, 141u8, 79u8, 213u8, 170u8, 169u8, 127u8,
							30u8, 236u8, 65u8, 38u8, 16u8, 118u8, 228u8, 141u8, 83u8, 162u8, 233u8,
						],
					)
				}

				#[doc = "See [`Pallet::unrequest_preimage`]."]
				pub fn unrequest_preimage(
					&self,
					hash: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<UnrequestPreimage> {
					::subxt::tx::StaticTxPayload::new(
						"Preimage",
						"unrequest_preimage",
						UnrequestPreimage { hash },
						[
							143u8, 225u8, 239u8, 44u8, 237u8, 83u8, 18u8, 105u8, 101u8, 68u8,
							111u8, 116u8, 66u8, 212u8, 63u8, 190u8, 38u8, 32u8, 105u8, 152u8, 69u8,
							177u8, 193u8, 15u8, 60u8, 26u8, 95u8, 130u8, 11u8, 113u8, 187u8, 108u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_preimage::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A preimage has been noted."]
			pub struct Noted {
				pub hash: ::subxt::ext::sp_core::H256,
			}
			impl ::subxt::events::StaticEvent for Noted {
				const EVENT: &'static str = "Noted";
				const PALLET: &'static str = "Preimage";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A preimage has been requested."]
			pub struct Requested {
				pub hash: ::subxt::ext::sp_core::H256,
			}
			impl ::subxt::events::StaticEvent for Requested {
				const EVENT: &'static str = "Requested";
				const PALLET: &'static str = "Preimage";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A preimage has ben cleared."]
			pub struct Cleared {
				pub hash: ::subxt::ext::sp_core::H256,
			}
			impl ::subxt::events::StaticEvent for Cleared {
				const EVENT: &'static str = "Cleared";
				const PALLET: &'static str = "Preimage";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The request status of a given hash."]
				pub fn status_for(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_preimage::RequestStatus<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Preimage",
						"StatusFor",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							103u8, 208u8, 88u8, 167u8, 244u8, 198u8, 129u8, 134u8, 182u8, 80u8,
							71u8, 192u8, 73u8, 92u8, 190u8, 15u8, 20u8, 132u8, 37u8, 108u8, 88u8,
							233u8, 18u8, 145u8, 9u8, 235u8, 5u8, 132u8, 42u8, 17u8, 227u8, 56u8,
						],
					)
				}

				#[doc = " The request status of a given hash."]
				pub fn status_for_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_preimage::RequestStatus<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Preimage",
						"StatusFor",
						Vec::new(),
						[
							103u8, 208u8, 88u8, 167u8, 244u8, 198u8, 129u8, 134u8, 182u8, 80u8,
							71u8, 192u8, 73u8, 92u8, 190u8, 15u8, 20u8, 132u8, 37u8, 108u8, 88u8,
							233u8, 18u8, 145u8, 9u8, 235u8, 5u8, 132u8, 42u8, 17u8, 227u8, 56u8,
						],
					)
				}

				pub fn preimage_for(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Preimage",
						"PreimageFor",
						vec![::subxt::storage::address::StorageMapKey::new(
							&(_0.borrow(), _1.borrow()),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							96u8, 74u8, 30u8, 112u8, 120u8, 41u8, 52u8, 187u8, 252u8, 68u8, 42u8,
							5u8, 61u8, 228u8, 250u8, 192u8, 224u8, 61u8, 53u8, 222u8, 95u8, 148u8,
							6u8, 53u8, 43u8, 152u8, 88u8, 58u8, 185u8, 234u8, 131u8, 124u8,
						],
					)
				}

				pub fn preimage_for_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Preimage",
						"PreimageFor",
						Vec::new(),
						[
							96u8, 74u8, 30u8, 112u8, 120u8, 41u8, 52u8, 187u8, 252u8, 68u8, 42u8,
							5u8, 61u8, 228u8, 250u8, 192u8, 224u8, 61u8, 53u8, 222u8, 95u8, 148u8,
							6u8, 53u8, 43u8, 152u8, 88u8, 58u8, 185u8, 234u8, 131u8, 124u8,
						],
					)
				}
			}
		}
	}
	pub mod multisig {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct AsMultiThreshold1 {
				pub other_signatories: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct AsMulti {
				pub threshold: ::core::primitive::u16,
				pub other_signatories: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				pub maybe_timepoint: ::core::option::Option<
					runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				>,
				pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
				pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ApproveAsMulti {
				pub threshold: ::core::primitive::u16,
				pub other_signatories: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				pub maybe_timepoint: ::core::option::Option<
					runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				>,
				pub call_hash: [::core::primitive::u8; 32usize],
				pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct CancelAsMulti {
				pub threshold: ::core::primitive::u16,
				pub other_signatories: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::as_multi_threshold_1`]."]
				pub fn as_multi_threshold_1(
					&self,
					other_signatories: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<AsMultiThreshold1> {
					::subxt::tx::StaticTxPayload::new(
						"Multisig",
						"as_multi_threshold_1",
						AsMultiThreshold1 {
							other_signatories,
							call: ::std::boxed::Box::new(call),
						},
						[
							27u8, 145u8, 63u8, 30u8, 160u8, 89u8, 45u8, 88u8, 87u8, 42u8, 253u8,
							55u8, 245u8, 172u8, 58u8, 163u8, 40u8, 18u8, 110u8, 35u8, 169u8, 240u8,
							2u8, 106u8, 81u8, 18u8, 152u8, 163u8, 232u8, 56u8, 85u8, 244u8,
						],
					)
				}

				#[doc = "See [`Pallet::as_multi`]."]
				pub fn as_multi(
					&self,
					threshold: ::core::primitive::u16,
					other_signatories: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					maybe_timepoint: ::core::option::Option<
						runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					>,
					call: runtime_types::da_runtime::RuntimeCall,
					max_weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::StaticTxPayload<AsMulti> {
					::subxt::tx::StaticTxPayload::new(
						"Multisig",
						"as_multi",
						AsMulti {
							threshold,
							other_signatories,
							maybe_timepoint,
							call: ::std::boxed::Box::new(call),
							max_weight,
						},
						[
							143u8, 168u8, 224u8, 129u8, 213u8, 76u8, 215u8, 48u8, 88u8, 167u8,
							217u8, 203u8, 25u8, 217u8, 61u8, 244u8, 117u8, 175u8, 32u8, 170u8,
							59u8, 35u8, 13u8, 168u8, 4u8, 109u8, 98u8, 203u8, 40u8, 69u8, 109u8,
							20u8,
						],
					)
				}

				#[doc = "See [`Pallet::approve_as_multi`]."]
				pub fn approve_as_multi(
					&self,
					threshold: ::core::primitive::u16,
					other_signatories: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					maybe_timepoint: ::core::option::Option<
						runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					>,
					call_hash: [::core::primitive::u8; 32usize],
					max_weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::StaticTxPayload<ApproveAsMulti> {
					::subxt::tx::StaticTxPayload::new(
						"Multisig",
						"approve_as_multi",
						ApproveAsMulti {
							threshold,
							other_signatories,
							maybe_timepoint,
							call_hash,
							max_weight,
						},
						[
							133u8, 113u8, 121u8, 66u8, 218u8, 219u8, 48u8, 64u8, 211u8, 114u8,
							163u8, 193u8, 164u8, 21u8, 140u8, 218u8, 253u8, 237u8, 240u8, 126u8,
							200u8, 213u8, 184u8, 50u8, 187u8, 182u8, 30u8, 52u8, 142u8, 72u8,
							210u8, 101u8,
						],
					)
				}

				#[doc = "See [`Pallet::cancel_as_multi`]."]
				pub fn cancel_as_multi(
					&self,
					threshold: ::core::primitive::u16,
					other_signatories: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					call_hash: [::core::primitive::u8; 32usize],
				) -> ::subxt::tx::StaticTxPayload<CancelAsMulti> {
					::subxt::tx::StaticTxPayload::new(
						"Multisig",
						"cancel_as_multi",
						CancelAsMulti {
							threshold,
							other_signatories,
							timepoint,
							call_hash,
						},
						[
							30u8, 25u8, 186u8, 142u8, 168u8, 81u8, 235u8, 164u8, 82u8, 209u8, 66u8,
							129u8, 209u8, 78u8, 172u8, 9u8, 163u8, 222u8, 125u8, 57u8, 2u8, 43u8,
							169u8, 174u8, 159u8, 167u8, 25u8, 226u8, 254u8, 110u8, 80u8, 216u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_multisig::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A new multisig operation has begun."]
			pub struct NewMultisig {
				pub approving: ::subxt::ext::sp_core::crypto::AccountId32,
				pub multisig: ::subxt::ext::sp_core::crypto::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for NewMultisig {
				const EVENT: &'static str = "NewMultisig";
				const PALLET: &'static str = "Multisig";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A multisig operation has been approved by someone."]
			pub struct MultisigApproval {
				pub approving: ::subxt::ext::sp_core::crypto::AccountId32,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub multisig: ::subxt::ext::sp_core::crypto::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for MultisigApproval {
				const EVENT: &'static str = "MultisigApproval";
				const PALLET: &'static str = "Multisig";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A multisig operation has been executed."]
			pub struct MultisigExecuted {
				pub approving: ::subxt::ext::sp_core::crypto::AccountId32,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub multisig: ::subxt::ext::sp_core::crypto::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MultisigExecuted {
				const EVENT: &'static str = "MultisigExecuted";
				const PALLET: &'static str = "Multisig";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A multisig operation has been cancelled."]
			pub struct MultisigCancelled {
				pub cancelling: ::subxt::ext::sp_core::crypto::AccountId32,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub multisig: ::subxt::ext::sp_core::crypto::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for MultisigCancelled {
				const EVENT: &'static str = "MultisigCancelled";
				const PALLET: &'static str = "Multisig";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The set of open multisig operations."]
				pub fn multisigs(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8; 32usize]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_multisig::Multisig<
							::core::primitive::u32,
							::core::primitive::u128,
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Multisig",
						"Multisigs",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							69u8, 153u8, 186u8, 204u8, 117u8, 95u8, 119u8, 182u8, 220u8, 87u8, 8u8,
							15u8, 123u8, 83u8, 5u8, 188u8, 115u8, 121u8, 163u8, 96u8, 218u8, 3u8,
							106u8, 44u8, 44u8, 187u8, 46u8, 238u8, 80u8, 203u8, 175u8, 155u8,
						],
					)
				}

				#[doc = " The set of open multisig operations."]
				pub fn multisigs_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_multisig::Multisig<
							::core::primitive::u32,
							::core::primitive::u128,
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Multisig",
						"Multisigs",
						Vec::new(),
						[
							69u8, 153u8, 186u8, 204u8, 117u8, 95u8, 119u8, 182u8, 220u8, 87u8, 8u8,
							15u8, 123u8, 83u8, 5u8, 188u8, 115u8, 121u8, 163u8, 96u8, 218u8, 3u8,
							106u8, 44u8, 44u8, 187u8, 46u8, 238u8, 80u8, 203u8, 175u8, 155u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The base amount of currency needed to reserve for creating a multisig execution or to"]
				#[doc = " store a dispatch call for later."]
				#[doc = ""]
				#[doc = " This is held for an additional storage item whose value size is"]
				#[doc = " `4 + sizeof((BlockNumber, Balance, AccountId))` bytes and whose key size is"]
				#[doc = " `32 + sizeof(AccountId)` bytes."]
				pub fn deposit_base(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new("Multisig", "DepositBase", [
						84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
						27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
						71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
					])
				}

				#[doc = " The amount of currency needed per unit threshold when creating a multisig execution."]
				#[doc = ""]
				#[doc = " This is held for adding 32 bytes more into a pre-existing storage value."]
				pub fn deposit_factor(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new("Multisig", "DepositFactor", [
						84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
						27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
						71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
					])
				}

				#[doc = " The maximum amount of signatories allowed in the multisig."]
				pub fn max_signatories(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Multisig", "MaxSignatories", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}
			}
		}
	}
	pub mod voter_list {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Rebag {
				pub dislocated: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct PutInFrontOf {
				pub lighter: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::rebag`]."]
				pub fn rebag(
					&self,
					dislocated: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<Rebag> {
					::subxt::tx::StaticTxPayload::new("VoterList", "rebag", Rebag { dislocated }, [
						0u8, 168u8, 218u8, 188u8, 236u8, 124u8, 250u8, 201u8, 237u8, 20u8, 97u8,
						150u8, 117u8, 232u8, 116u8, 237u8, 55u8, 151u8, 71u8, 119u8, 42u8, 48u8,
						10u8, 66u8, 167u8, 208u8, 184u8, 228u8, 146u8, 181u8, 84u8, 70u8,
					])
				}

				#[doc = "See [`Pallet::put_in_front_of`]."]
				pub fn put_in_front_of(
					&self,
					lighter: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<PutInFrontOf> {
					::subxt::tx::StaticTxPayload::new(
						"VoterList",
						"put_in_front_of",
						PutInFrontOf { lighter },
						[
							104u8, 36u8, 96u8, 80u8, 236u8, 75u8, 203u8, 232u8, 136u8, 167u8,
							205u8, 143u8, 200u8, 53u8, 124u8, 148u8, 76u8, 246u8, 71u8, 246u8,
							205u8, 82u8, 32u8, 186u8, 33u8, 5u8, 183u8, 127u8, 153u8, 232u8, 80u8,
							164u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_bags_list::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Moved an account from one bag to another."]
			pub struct Rebagged {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub from: ::core::primitive::u64,
				pub to: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for Rebagged {
				const EVENT: &'static str = "Rebagged";
				const PALLET: &'static str = "VoterList";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Updated the score of some account to the given amount."]
			pub struct ScoreUpdated {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub new_score: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for ScoreUpdated {
				const EVENT: &'static str = "ScoreUpdated";
				const PALLET: &'static str = "VoterList";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " A single node, within some bag."]
				#[doc = ""]
				#[doc = " Nodes store links forward and back within their respective bags."]
				pub fn list_nodes(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_bags_list::list::Node,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"VoterList",
						"ListNodes",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							176u8, 186u8, 93u8, 51u8, 100u8, 184u8, 240u8, 29u8, 70u8, 3u8, 117u8,
							47u8, 23u8, 66u8, 231u8, 234u8, 53u8, 8u8, 234u8, 175u8, 181u8, 8u8,
							161u8, 154u8, 48u8, 178u8, 147u8, 227u8, 122u8, 115u8, 57u8, 97u8,
						],
					)
				}

				#[doc = " A single node, within some bag."]
				#[doc = ""]
				#[doc = " Nodes store links forward and back within their respective bags."]
				pub fn list_nodes_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_bags_list::list::Node,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"VoterList",
						"ListNodes",
						Vec::new(),
						[
							176u8, 186u8, 93u8, 51u8, 100u8, 184u8, 240u8, 29u8, 70u8, 3u8, 117u8,
							47u8, 23u8, 66u8, 231u8, 234u8, 53u8, 8u8, 234u8, 175u8, 181u8, 8u8,
							161u8, 154u8, 48u8, 178u8, 147u8, 227u8, 122u8, 115u8, 57u8, 97u8,
						],
					)
				}

				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_list_nodes(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"VoterList",
						"CounterForListNodes",
						vec![],
						[
							156u8, 168u8, 97u8, 33u8, 84u8, 117u8, 220u8, 89u8, 62u8, 182u8, 24u8,
							88u8, 231u8, 244u8, 41u8, 19u8, 210u8, 131u8, 87u8, 0u8, 241u8, 230u8,
							160u8, 142u8, 128u8, 153u8, 83u8, 36u8, 88u8, 247u8, 70u8, 130u8,
						],
					)
				}

				#[doc = " A bag stored in storage."]
				#[doc = ""]
				#[doc = " Stores a `Bag` struct, which stores head and tail pointers to itself."]
				pub fn list_bags(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::pallet_bags_list::list::Bag>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"VoterList",
						"ListBags",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							38u8, 86u8, 63u8, 92u8, 85u8, 59u8, 225u8, 244u8, 14u8, 155u8, 76u8,
							249u8, 153u8, 140u8, 179u8, 7u8, 96u8, 170u8, 236u8, 179u8, 4u8, 18u8,
							232u8, 146u8, 216u8, 51u8, 135u8, 116u8, 196u8, 117u8, 143u8, 153u8,
						],
					)
				}

				#[doc = " A bag stored in storage."]
				#[doc = ""]
				#[doc = " Stores a `Bag` struct, which stores head and tail pointers to itself."]
				pub fn list_bags_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::pallet_bags_list::list::Bag>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"VoterList",
						"ListBags",
						Vec::new(),
						[
							38u8, 86u8, 63u8, 92u8, 85u8, 59u8, 225u8, 244u8, 14u8, 155u8, 76u8,
							249u8, 153u8, 140u8, 179u8, 7u8, 96u8, 170u8, 236u8, 179u8, 4u8, 18u8,
							232u8, 146u8, 216u8, 51u8, 135u8, 116u8, 196u8, 117u8, 143u8, 153u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The list of thresholds separating the various bags."]
				#[doc = ""]
				#[doc = " Ids are separated into unsorted bags according to their score. This specifies the"]
				#[doc = " thresholds separating the bags. An id's bag is the largest bag for which the id's score"]
				#[doc = " is less than or equal to its upper threshold."]
				#[doc = ""]
				#[doc = " When ids are iterated, higher bags are iterated completely before lower bags. This means"]
				#[doc = " that iteration is _semi-sorted_: ids of higher score tend to come before ids of lower"]
				#[doc = " score, but peer ids within a particular bag are sorted in insertion order."]
				#[doc = ""]
				#[doc = " # Expressing the constant"]
				#[doc = ""]
				#[doc = " This constant must be sorted in strictly increasing order. Duplicate items are not"]
				#[doc = " permitted."]
				#[doc = ""]
				#[doc = " There is an implied upper limit of `Score::MAX`; that value does not need to be"]
				#[doc = " specified within the bag. For any two threshold lists, if one ends with"]
				#[doc = " `Score::MAX`, the other one does not, and they are otherwise equal, the two"]
				#[doc = " lists will behave identically."]
				#[doc = ""]
				#[doc = " # Calculation"]
				#[doc = ""]
				#[doc = " It is recommended to generate the set of thresholds in a geometric series, such that"]
				#[doc = " there exists some constant ratio such that `threshold[k + 1] == (threshold[k] *"]
				#[doc = " constant_ratio).max(threshold[k] + 1)` for all `k`."]
				#[doc = ""]
				#[doc = " The helpers in the `/utils/frame/generate-bags` module can simplify this calculation."]
				#[doc = ""]
				#[doc = " # Examples"]
				#[doc = ""]
				#[doc = " - If `BagThresholds::get().is_empty()`, then all ids are put into the same bag, and"]
				#[doc = "   iteration is strictly in insertion order."]
				#[doc = " - If `BagThresholds::get().len() == 64`, and the thresholds are determined according to"]
				#[doc = "   the procedure given above, then the constant ratio is equal to 2."]
				#[doc = " - If `BagThresholds::get().len() == 200`, and the thresholds are determined according to"]
				#[doc = "   the procedure given above, then the constant ratio is approximately equal to 1.248."]
				#[doc = " - If the threshold list begins `[1, 2, 3, ...]`, then an id with score 0 or 1 will fall"]
				#[doc = "   into bag 0, an id with score 2 will fall into bag 1, etc."]
				#[doc = ""]
				#[doc = " # Migration"]
				#[doc = ""]
				#[doc = " In the event that this list ever changes, a copy of the old bags list must be retained."]
				#[doc = " With that `List::migrate` can be called, which will perform the appropriate migration."]
				pub fn bag_thresholds(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u64>>,
				> {
					::subxt::constants::StaticConstantAddress::new("VoterList", "BagThresholds", [
						103u8, 102u8, 255u8, 165u8, 124u8, 54u8, 5u8, 172u8, 112u8, 234u8, 25u8,
						175u8, 178u8, 19u8, 251u8, 73u8, 91u8, 192u8, 227u8, 81u8, 249u8, 45u8,
						126u8, 116u8, 7u8, 37u8, 9u8, 200u8, 167u8, 182u8, 12u8, 131u8,
					])
				}
			}
		}
	}
	pub mod nomination_pools {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Join {
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
				pub pool_id: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct BondExtra {
				pub extra:
					runtime_types::pallet_nomination_pools::BondExtra<::core::primitive::u128>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ClaimPayout;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Unbond {
				pub member_account: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub unbonding_points: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct PoolWithdrawUnbonded {
				pub pool_id: ::core::primitive::u32,
				pub num_slashing_spans: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct WithdrawUnbonded {
				pub member_account: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub num_slashing_spans: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Create {
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
				pub root: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub nominator: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub bouncer: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct CreateWithPoolId {
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
				pub root: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub nominator: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub bouncer: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub pool_id: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Nominate {
				pub pool_id: ::core::primitive::u32,
				pub validators: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetState {
				pub pool_id: ::core::primitive::u32,
				pub state: runtime_types::pallet_nomination_pools::PoolState,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetMetadata {
				pub pool_id: ::core::primitive::u32,
				pub metadata: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetConfigs {
				pub min_join_bond:
					runtime_types::pallet_nomination_pools::ConfigOp<::core::primitive::u128>,
				pub min_create_bond:
					runtime_types::pallet_nomination_pools::ConfigOp<::core::primitive::u128>,
				pub max_pools:
					runtime_types::pallet_nomination_pools::ConfigOp<::core::primitive::u32>,
				pub max_members:
					runtime_types::pallet_nomination_pools::ConfigOp<::core::primitive::u32>,
				pub max_members_per_pool:
					runtime_types::pallet_nomination_pools::ConfigOp<::core::primitive::u32>,
				pub global_max_commission: runtime_types::pallet_nomination_pools::ConfigOp<
					runtime_types::sp_arithmetic::per_things::Perbill,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct UpdateRoles {
				pub pool_id: ::core::primitive::u32,
				pub new_root: runtime_types::pallet_nomination_pools::ConfigOp<
					::subxt::ext::sp_core::crypto::AccountId32,
				>,
				pub new_nominator: runtime_types::pallet_nomination_pools::ConfigOp<
					::subxt::ext::sp_core::crypto::AccountId32,
				>,
				pub new_bouncer: runtime_types::pallet_nomination_pools::ConfigOp<
					::subxt::ext::sp_core::crypto::AccountId32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Chill {
				pub pool_id: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct BondExtraOther {
				pub member: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub extra:
					runtime_types::pallet_nomination_pools::BondExtra<::core::primitive::u128>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetClaimPermission {
				pub permission: runtime_types::pallet_nomination_pools::ClaimPermission,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ClaimPayoutOther {
				pub other: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetCommission {
				pub pool_id: ::core::primitive::u32,
				pub new_commission: ::core::option::Option<(
					runtime_types::sp_arithmetic::per_things::Perbill,
					::subxt::ext::sp_core::crypto::AccountId32,
				)>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetCommissionMax {
				pub pool_id: ::core::primitive::u32,
				pub max_commission: runtime_types::sp_arithmetic::per_things::Perbill,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetCommissionChangeRate {
				pub pool_id: ::core::primitive::u32,
				pub change_rate: runtime_types::pallet_nomination_pools::CommissionChangeRate<
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ClaimCommission {
				pub pool_id: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::join`]."]
				pub fn join(
					&self,
					amount: ::core::primitive::u128,
					pool_id: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Join> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"join",
						Join { amount, pool_id },
						[
							205u8, 66u8, 42u8, 72u8, 146u8, 148u8, 119u8, 162u8, 101u8, 183u8,
							46u8, 176u8, 221u8, 204u8, 197u8, 20u8, 75u8, 226u8, 29u8, 118u8,
							208u8, 60u8, 192u8, 247u8, 222u8, 100u8, 69u8, 80u8, 172u8, 13u8, 69u8,
							250u8,
						],
					)
				}

				#[doc = "See [`Pallet::bond_extra`]."]
				pub fn bond_extra(
					&self,
					extra: runtime_types::pallet_nomination_pools::BondExtra<
						::core::primitive::u128,
					>,
				) -> ::subxt::tx::StaticTxPayload<BondExtra> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"bond_extra",
						BondExtra { extra },
						[
							50u8, 72u8, 181u8, 216u8, 249u8, 27u8, 250u8, 177u8, 253u8, 22u8,
							240u8, 100u8, 184u8, 202u8, 197u8, 34u8, 21u8, 188u8, 248u8, 191u8,
							11u8, 10u8, 236u8, 161u8, 168u8, 37u8, 38u8, 238u8, 61u8, 183u8, 86u8,
							55u8,
						],
					)
				}

				#[doc = "See [`Pallet::claim_payout`]."]
				pub fn claim_payout(&self) -> ::subxt::tx::StaticTxPayload<ClaimPayout> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"claim_payout",
						ClaimPayout {},
						[
							128u8, 58u8, 138u8, 55u8, 64u8, 16u8, 129u8, 25u8, 211u8, 229u8, 193u8,
							115u8, 47u8, 45u8, 155u8, 221u8, 218u8, 1u8, 222u8, 5u8, 236u8, 32u8,
							88u8, 0u8, 198u8, 72u8, 196u8, 181u8, 104u8, 16u8, 212u8, 29u8,
						],
					)
				}

				#[doc = "See [`Pallet::unbond`]."]
				pub fn unbond(
					&self,
					member_account: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					unbonding_points: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Unbond> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"unbond",
						Unbond {
							member_account,
							unbonding_points,
						},
						[
							139u8, 71u8, 78u8, 184u8, 141u8, 89u8, 179u8, 123u8, 153u8, 30u8,
							116u8, 186u8, 148u8, 49u8, 48u8, 98u8, 33u8, 21u8, 29u8, 106u8, 180u8,
							212u8, 37u8, 251u8, 237u8, 21u8, 255u8, 13u8, 236u8, 73u8, 250u8, 57u8,
						],
					)
				}

				#[doc = "See [`Pallet::pool_withdraw_unbonded`]."]
				pub fn pool_withdraw_unbonded(
					&self,
					pool_id: ::core::primitive::u32,
					num_slashing_spans: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<PoolWithdrawUnbonded> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"pool_withdraw_unbonded",
						PoolWithdrawUnbonded {
							pool_id,
							num_slashing_spans,
						},
						[
							152u8, 245u8, 131u8, 247u8, 106u8, 214u8, 154u8, 8u8, 7u8, 210u8,
							149u8, 218u8, 118u8, 46u8, 242u8, 182u8, 191u8, 119u8, 28u8, 199u8,
							36u8, 49u8, 219u8, 123u8, 58u8, 203u8, 211u8, 226u8, 217u8, 36u8, 56u8,
							0u8,
						],
					)
				}

				#[doc = "See [`Pallet::withdraw_unbonded`]."]
				pub fn withdraw_unbonded(
					&self,
					member_account: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					num_slashing_spans: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<WithdrawUnbonded> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"withdraw_unbonded",
						WithdrawUnbonded {
							member_account,
							num_slashing_spans,
						},
						[
							192u8, 183u8, 121u8, 87u8, 176u8, 70u8, 91u8, 226u8, 156u8, 79u8, 87u8,
							34u8, 227u8, 84u8, 22u8, 235u8, 3u8, 181u8, 166u8, 194u8, 147u8, 72u8,
							27u8, 221u8, 57u8, 14u8, 44u8, 70u8, 253u8, 236u8, 44u8, 84u8,
						],
					)
				}

				#[doc = "See [`Pallet::create`]."]
				pub fn create(
					&self,
					amount: ::core::primitive::u128,
					root: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					nominator: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					bouncer: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<Create> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"create",
						Create {
							amount,
							root,
							nominator,
							bouncer,
						},
						[
							19u8, 83u8, 115u8, 108u8, 192u8, 96u8, 44u8, 77u8, 251u8, 152u8, 61u8,
							103u8, 209u8, 17u8, 84u8, 245u8, 153u8, 174u8, 124u8, 92u8, 236u8,
							72u8, 225u8, 73u8, 89u8, 236u8, 174u8, 180u8, 60u8, 140u8, 198u8, 59u8,
						],
					)
				}

				#[doc = "See [`Pallet::create_with_pool_id`]."]
				pub fn create_with_pool_id(
					&self,
					amount: ::core::primitive::u128,
					root: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					nominator: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					bouncer: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pool_id: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<CreateWithPoolId> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"create_with_pool_id",
						CreateWithPoolId {
							amount,
							root,
							nominator,
							bouncer,
							pool_id,
						},
						[
							81u8, 185u8, 201u8, 99u8, 26u8, 96u8, 149u8, 43u8, 181u8, 3u8, 149u8,
							110u8, 158u8, 178u8, 138u8, 205u8, 79u8, 251u8, 86u8, 18u8, 128u8,
							117u8, 66u8, 220u8, 222u8, 238u8, 122u8, 177u8, 63u8, 117u8, 104u8,
							54u8,
						],
					)
				}

				#[doc = "See [`Pallet::nominate`]."]
				pub fn nominate(
					&self,
					pool_id: ::core::primitive::u32,
					validators: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::tx::StaticTxPayload<Nominate> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"nominate",
						Nominate {
							pool_id,
							validators,
						},
						[
							10u8, 235u8, 64u8, 157u8, 36u8, 249u8, 186u8, 27u8, 79u8, 172u8, 25u8,
							3u8, 203u8, 19u8, 192u8, 182u8, 36u8, 103u8, 13u8, 20u8, 89u8, 140u8,
							159u8, 4u8, 132u8, 242u8, 192u8, 146u8, 55u8, 251u8, 216u8, 255u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_state`]."]
				pub fn set_state(
					&self,
					pool_id: ::core::primitive::u32,
					state: runtime_types::pallet_nomination_pools::PoolState,
				) -> ::subxt::tx::StaticTxPayload<SetState> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"set_state",
						SetState { pool_id, state },
						[
							104u8, 40u8, 213u8, 88u8, 159u8, 115u8, 35u8, 249u8, 78u8, 180u8, 99u8,
							1u8, 225u8, 218u8, 192u8, 151u8, 25u8, 194u8, 192u8, 187u8, 39u8,
							170u8, 212u8, 125u8, 75u8, 250u8, 248u8, 175u8, 159u8, 161u8, 151u8,
							162u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_metadata`]."]
				pub fn set_metadata(
					&self,
					pool_id: ::core::primitive::u32,
					metadata: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<SetMetadata> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"set_metadata",
						SetMetadata { pool_id, metadata },
						[
							156u8, 81u8, 170u8, 161u8, 34u8, 100u8, 183u8, 174u8, 5u8, 81u8, 31u8,
							76u8, 12u8, 42u8, 77u8, 1u8, 6u8, 26u8, 168u8, 7u8, 8u8, 115u8, 158u8,
							151u8, 30u8, 211u8, 52u8, 177u8, 234u8, 87u8, 125u8, 127u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_configs`]."]
				pub fn set_configs(
					&self,
					min_join_bond: runtime_types::pallet_nomination_pools::ConfigOp<
						::core::primitive::u128,
					>,
					min_create_bond: runtime_types::pallet_nomination_pools::ConfigOp<
						::core::primitive::u128,
					>,
					max_pools: runtime_types::pallet_nomination_pools::ConfigOp<
						::core::primitive::u32,
					>,
					max_members: runtime_types::pallet_nomination_pools::ConfigOp<
						::core::primitive::u32,
					>,
					max_members_per_pool: runtime_types::pallet_nomination_pools::ConfigOp<
						::core::primitive::u32,
					>,
					global_max_commission: runtime_types::pallet_nomination_pools::ConfigOp<
						runtime_types::sp_arithmetic::per_things::Perbill,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetConfigs> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"set_configs",
						SetConfigs {
							min_join_bond,
							min_create_bond,
							max_pools,
							max_members,
							max_members_per_pool,
							global_max_commission,
						},
						[
							20u8, 66u8, 112u8, 172u8, 143u8, 78u8, 60u8, 159u8, 240u8, 102u8,
							245u8, 10u8, 207u8, 27u8, 99u8, 138u8, 217u8, 239u8, 101u8, 190u8,
							222u8, 253u8, 53u8, 77u8, 230u8, 225u8, 101u8, 109u8, 50u8, 144u8,
							31u8, 121u8,
						],
					)
				}

				#[doc = "See [`Pallet::update_roles`]."]
				pub fn update_roles(
					&self,
					pool_id: ::core::primitive::u32,
					new_root: runtime_types::pallet_nomination_pools::ConfigOp<
						::subxt::ext::sp_core::crypto::AccountId32,
					>,
					new_nominator: runtime_types::pallet_nomination_pools::ConfigOp<
						::subxt::ext::sp_core::crypto::AccountId32,
					>,
					new_bouncer: runtime_types::pallet_nomination_pools::ConfigOp<
						::subxt::ext::sp_core::crypto::AccountId32,
					>,
				) -> ::subxt::tx::StaticTxPayload<UpdateRoles> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"update_roles",
						UpdateRoles {
							pool_id,
							new_root,
							new_nominator,
							new_bouncer,
						},
						[
							15u8, 154u8, 204u8, 28u8, 204u8, 120u8, 174u8, 203u8, 186u8, 33u8,
							123u8, 201u8, 143u8, 120u8, 193u8, 49u8, 164u8, 178u8, 55u8, 234u8,
							126u8, 247u8, 123u8, 73u8, 147u8, 107u8, 43u8, 72u8, 217u8, 4u8, 199u8,
							253u8,
						],
					)
				}

				#[doc = "See [`Pallet::chill`]."]
				pub fn chill(
					&self,
					pool_id: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Chill> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"chill",
						Chill { pool_id },
						[
							41u8, 114u8, 128u8, 121u8, 244u8, 15u8, 15u8, 52u8, 129u8, 88u8, 239u8,
							167u8, 216u8, 38u8, 123u8, 240u8, 172u8, 229u8, 132u8, 64u8, 175u8,
							87u8, 217u8, 27u8, 11u8, 124u8, 1u8, 140u8, 40u8, 191u8, 187u8, 36u8,
						],
					)
				}

				#[doc = "See [`Pallet::bond_extra_other`]."]
				pub fn bond_extra_other(
					&self,
					member: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					extra: runtime_types::pallet_nomination_pools::BondExtra<
						::core::primitive::u128,
					>,
				) -> ::subxt::tx::StaticTxPayload<BondExtraOther> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"bond_extra_other",
						BondExtraOther { member, extra },
						[
							1u8, 73u8, 186u8, 39u8, 247u8, 130u8, 155u8, 185u8, 212u8, 110u8,
							151u8, 12u8, 18u8, 195u8, 106u8, 28u8, 194u8, 130u8, 54u8, 59u8, 184u8,
							192u8, 141u8, 227u8, 102u8, 210u8, 76u8, 41u8, 196u8, 90u8, 215u8,
							84u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_claim_permission`]."]
				pub fn set_claim_permission(
					&self,
					permission: runtime_types::pallet_nomination_pools::ClaimPermission,
				) -> ::subxt::tx::StaticTxPayload<SetClaimPermission> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"set_claim_permission",
						SetClaimPermission { permission },
						[
							23u8, 253u8, 135u8, 53u8, 83u8, 71u8, 182u8, 223u8, 123u8, 57u8, 93u8,
							154u8, 110u8, 91u8, 63u8, 241u8, 144u8, 218u8, 129u8, 238u8, 169u8,
							9u8, 215u8, 76u8, 65u8, 168u8, 103u8, 44u8, 40u8, 39u8, 34u8, 16u8,
						],
					)
				}

				#[doc = "See [`Pallet::claim_payout_other`]."]
				pub fn claim_payout_other(
					&self,
					other: ::subxt::ext::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<ClaimPayoutOther> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"claim_payout_other",
						ClaimPayoutOther { other },
						[
							52u8, 165u8, 191u8, 125u8, 180u8, 54u8, 27u8, 235u8, 195u8, 22u8, 55u8,
							183u8, 209u8, 63u8, 116u8, 88u8, 154u8, 74u8, 100u8, 103u8, 88u8, 76u8,
							35u8, 14u8, 39u8, 156u8, 219u8, 253u8, 123u8, 104u8, 168u8, 76u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_commission`]."]
				pub fn set_commission(
					&self,
					pool_id: ::core::primitive::u32,
					new_commission: ::core::option::Option<(
						runtime_types::sp_arithmetic::per_things::Perbill,
						::subxt::ext::sp_core::crypto::AccountId32,
					)>,
				) -> ::subxt::tx::StaticTxPayload<SetCommission> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"set_commission",
						SetCommission {
							pool_id,
							new_commission,
						},
						[
							118u8, 240u8, 166u8, 40u8, 247u8, 44u8, 23u8, 92u8, 4u8, 78u8, 156u8,
							21u8, 178u8, 97u8, 197u8, 148u8, 61u8, 234u8, 15u8, 94u8, 248u8, 188u8,
							211u8, 13u8, 134u8, 10u8, 75u8, 59u8, 218u8, 13u8, 104u8, 115u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_commission_max`]."]
				pub fn set_commission_max(
					&self,
					pool_id: ::core::primitive::u32,
					max_commission: runtime_types::sp_arithmetic::per_things::Perbill,
				) -> ::subxt::tx::StaticTxPayload<SetCommissionMax> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"set_commission_max",
						SetCommissionMax {
							pool_id,
							max_commission,
						},
						[
							115u8, 90u8, 156u8, 35u8, 7u8, 125u8, 184u8, 123u8, 149u8, 232u8, 59u8,
							21u8, 42u8, 120u8, 14u8, 152u8, 184u8, 167u8, 18u8, 22u8, 148u8, 83u8,
							16u8, 81u8, 93u8, 182u8, 154u8, 182u8, 46u8, 40u8, 179u8, 187u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_commission_change_rate`]."]
				pub fn set_commission_change_rate(
					&self,
					pool_id: ::core::primitive::u32,
					change_rate: runtime_types::pallet_nomination_pools::CommissionChangeRate<
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetCommissionChangeRate> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"set_commission_change_rate",
						SetCommissionChangeRate {
							pool_id,
							change_rate,
						},
						[
							118u8, 194u8, 114u8, 197u8, 214u8, 246u8, 23u8, 237u8, 10u8, 90u8,
							230u8, 123u8, 172u8, 174u8, 98u8, 198u8, 160u8, 71u8, 113u8, 76u8,
							201u8, 201u8, 153u8, 92u8, 222u8, 252u8, 7u8, 184u8, 236u8, 235u8,
							126u8, 201u8,
						],
					)
				}

				#[doc = "See [`Pallet::claim_commission`]."]
				pub fn claim_commission(
					&self,
					pool_id: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ClaimCommission> {
					::subxt::tx::StaticTxPayload::new(
						"NominationPools",
						"claim_commission",
						ClaimCommission { pool_id },
						[
							139u8, 126u8, 219u8, 117u8, 140u8, 51u8, 163u8, 32u8, 83u8, 60u8,
							250u8, 44u8, 186u8, 194u8, 225u8, 84u8, 61u8, 181u8, 212u8, 160u8,
							156u8, 93u8, 16u8, 255u8, 165u8, 178u8, 25u8, 64u8, 187u8, 29u8, 169u8,
							174u8,
						],
					)
				}
			}
		}
		#[doc = "Events of this pallet."]
		pub type Event = runtime_types::pallet_nomination_pools::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A pool has been created."]
			pub struct Created {
				pub depositor: ::subxt::ext::sp_core::crypto::AccountId32,
				pub pool_id: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Created {
				const EVENT: &'static str = "Created";
				const PALLET: &'static str = "NominationPools";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A member has became bonded in a pool."]
			pub struct Bonded {
				pub member: ::subxt::ext::sp_core::crypto::AccountId32,
				pub pool_id: ::core::primitive::u32,
				pub bonded: ::core::primitive::u128,
				pub joined: ::core::primitive::bool,
			}
			impl ::subxt::events::StaticEvent for Bonded {
				const EVENT: &'static str = "Bonded";
				const PALLET: &'static str = "NominationPools";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A payout has been made to a member."]
			pub struct PaidOut {
				pub member: ::subxt::ext::sp_core::crypto::AccountId32,
				pub pool_id: ::core::primitive::u32,
				pub payout: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for PaidOut {
				const EVENT: &'static str = "PaidOut";
				const PALLET: &'static str = "NominationPools";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A member has unbonded from their pool."]
			#[doc = ""]
			#[doc = "- `balance` is the corresponding balance of the number of points that has been"]
			#[doc = "  requested to be unbonded (the argument of the `unbond` transaction) from the bonded"]
			#[doc = "  pool."]
			#[doc = "- `points` is the number of points that are issued as a result of `balance` being"]
			#[doc = "dissolved into the corresponding unbonding pool."]
			#[doc = "- `era` is the era in which the balance will be unbonded."]
			#[doc = "In the absence of slashing, these values will match. In the presence of slashing, the"]
			#[doc = "number of points that are issued in the unbonding pool will be less than the amount"]
			#[doc = "requested to be unbonded."]
			pub struct Unbonded {
				pub member: ::subxt::ext::sp_core::crypto::AccountId32,
				pub pool_id: ::core::primitive::u32,
				pub balance: ::core::primitive::u128,
				pub points: ::core::primitive::u128,
				pub era: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Unbonded {
				const EVENT: &'static str = "Unbonded";
				const PALLET: &'static str = "NominationPools";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A member has withdrawn from their pool."]
			#[doc = ""]
			#[doc = "The given number of `points` have been dissolved in return of `balance`."]
			#[doc = ""]
			#[doc = "Similar to `Unbonded` event, in the absence of slashing, the ratio of point to balance"]
			#[doc = "will be 1."]
			pub struct Withdrawn {
				pub member: ::subxt::ext::sp_core::crypto::AccountId32,
				pub pool_id: ::core::primitive::u32,
				pub balance: ::core::primitive::u128,
				pub points: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdrawn {
				const EVENT: &'static str = "Withdrawn";
				const PALLET: &'static str = "NominationPools";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A pool has been destroyed."]
			pub struct Destroyed {
				pub pool_id: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Destroyed {
				const EVENT: &'static str = "Destroyed";
				const PALLET: &'static str = "NominationPools";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The state of a pool has changed"]
			pub struct StateChanged {
				pub pool_id: ::core::primitive::u32,
				pub new_state: runtime_types::pallet_nomination_pools::PoolState,
			}
			impl ::subxt::events::StaticEvent for StateChanged {
				const EVENT: &'static str = "StateChanged";
				const PALLET: &'static str = "NominationPools";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A member has been removed from a pool."]
			#[doc = ""]
			#[doc = "The removal can be voluntary (withdrawn all unbonded funds) or involuntary (kicked)."]
			pub struct MemberRemoved {
				pub pool_id: ::core::primitive::u32,
				pub member: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for MemberRemoved {
				const EVENT: &'static str = "MemberRemoved";
				const PALLET: &'static str = "NominationPools";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The roles of a pool have been updated to the given new roles. Note that the depositor"]
			#[doc = "can never change."]
			pub struct RolesUpdated {
				pub root: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
				pub bouncer: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
				pub nominator: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for RolesUpdated {
				const EVENT: &'static str = "RolesUpdated";
				const PALLET: &'static str = "NominationPools";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The active balance of pool `pool_id` has been slashed to `balance`."]
			pub struct PoolSlashed {
				pub pool_id: ::core::primitive::u32,
				pub balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for PoolSlashed {
				const EVENT: &'static str = "PoolSlashed";
				const PALLET: &'static str = "NominationPools";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The unbond pool at `era` of pool `pool_id` has been slashed to `balance`."]
			pub struct UnbondingPoolSlashed {
				pub pool_id: ::core::primitive::u32,
				pub era: ::core::primitive::u32,
				pub balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for UnbondingPoolSlashed {
				const EVENT: &'static str = "UnbondingPoolSlashed";
				const PALLET: &'static str = "NominationPools";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A pool's commission setting has been changed."]
			pub struct PoolCommissionUpdated {
				pub pool_id: ::core::primitive::u32,
				pub current: ::core::option::Option<(
					runtime_types::sp_arithmetic::per_things::Perbill,
					::subxt::ext::sp_core::crypto::AccountId32,
				)>,
			}
			impl ::subxt::events::StaticEvent for PoolCommissionUpdated {
				const EVENT: &'static str = "PoolCommissionUpdated";
				const PALLET: &'static str = "NominationPools";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A pool's maximum commission setting has been changed."]
			pub struct PoolMaxCommissionUpdated {
				pub pool_id: ::core::primitive::u32,
				pub max_commission: runtime_types::sp_arithmetic::per_things::Perbill,
			}
			impl ::subxt::events::StaticEvent for PoolMaxCommissionUpdated {
				const EVENT: &'static str = "PoolMaxCommissionUpdated";
				const PALLET: &'static str = "NominationPools";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A pool's commission `change_rate` has been changed."]
			pub struct PoolCommissionChangeRateUpdated {
				pub pool_id: ::core::primitive::u32,
				pub change_rate: runtime_types::pallet_nomination_pools::CommissionChangeRate<
					::core::primitive::u32,
				>,
			}
			impl ::subxt::events::StaticEvent for PoolCommissionChangeRateUpdated {
				const EVENT: &'static str = "PoolCommissionChangeRateUpdated";
				const PALLET: &'static str = "NominationPools";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Pool commission has been claimed."]
			pub struct PoolCommissionClaimed {
				pub pool_id: ::core::primitive::u32,
				pub commission: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for PoolCommissionClaimed {
				const EVENT: &'static str = "PoolCommissionClaimed";
				const PALLET: &'static str = "NominationPools";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Minimum amount to bond to join a pool."]
				pub fn min_join_bond(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"MinJoinBond",
						vec![],
						[
							125u8, 239u8, 45u8, 225u8, 74u8, 129u8, 247u8, 184u8, 205u8, 58u8,
							45u8, 186u8, 126u8, 170u8, 112u8, 120u8, 23u8, 190u8, 247u8, 97u8,
							131u8, 126u8, 215u8, 44u8, 147u8, 122u8, 132u8, 212u8, 217u8, 84u8,
							240u8, 91u8,
						],
					)
				}

				#[doc = " Minimum bond required to create a pool."]
				#[doc = ""]
				#[doc = " This is the amount that the depositor must put as their initial stake in the pool, as an"]
				#[doc = " indication of \"skin in the game\"."]
				#[doc = ""]
				#[doc = " This is the value that will always exist in the staking ledger of the pool bonded account"]
				#[doc = " while all other accounts leave."]
				pub fn min_create_bond(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"MinCreateBond",
						vec![],
						[
							31u8, 208u8, 240u8, 158u8, 23u8, 218u8, 212u8, 138u8, 92u8, 210u8,
							207u8, 170u8, 32u8, 60u8, 5u8, 21u8, 84u8, 162u8, 1u8, 111u8, 181u8,
							243u8, 24u8, 148u8, 193u8, 253u8, 248u8, 190u8, 16u8, 222u8, 219u8,
							67u8,
						],
					)
				}

				#[doc = " Maximum number of nomination pools that can exist. If `None`, then an unbounded number of"]
				#[doc = " pools can exist."]
				pub fn max_pools(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"MaxPools",
						vec![],
						[
							216u8, 111u8, 68u8, 103u8, 33u8, 50u8, 109u8, 3u8, 176u8, 195u8, 23u8,
							73u8, 112u8, 138u8, 9u8, 194u8, 233u8, 73u8, 68u8, 215u8, 162u8, 255u8,
							217u8, 173u8, 141u8, 27u8, 72u8, 199u8, 7u8, 240u8, 25u8, 34u8,
						],
					)
				}

				#[doc = " Maximum number of members that can exist in the system. If `None`, then the count"]
				#[doc = " members are not bound on a system wide basis."]
				pub fn max_pool_members(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"MaxPoolMembers",
						vec![],
						[
							82u8, 217u8, 26u8, 234u8, 223u8, 241u8, 66u8, 182u8, 43u8, 233u8, 59u8,
							242u8, 202u8, 254u8, 69u8, 50u8, 254u8, 196u8, 166u8, 89u8, 120u8,
							87u8, 76u8, 148u8, 31u8, 197u8, 49u8, 88u8, 206u8, 41u8, 242u8, 62u8,
						],
					)
				}

				#[doc = " Maximum number of members that may belong to pool. If `None`, then the count of"]
				#[doc = " members is not bound on a per pool basis."]
				pub fn max_pool_members_per_pool(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"MaxPoolMembersPerPool",
						vec![],
						[
							93u8, 241u8, 16u8, 169u8, 138u8, 199u8, 128u8, 149u8, 65u8, 30u8, 55u8,
							11u8, 41u8, 252u8, 83u8, 250u8, 9u8, 33u8, 152u8, 239u8, 195u8, 147u8,
							16u8, 248u8, 180u8, 153u8, 88u8, 231u8, 248u8, 169u8, 186u8, 48u8,
						],
					)
				}

				#[doc = " The maximum commission that can be charged by a pool. Used on commission payouts to bound"]
				#[doc = " pool commissions that are > `GlobalMaxCommission`, necessary if a future"]
				#[doc = " `GlobalMaxCommission` is lower than some current pool commissions."]
				pub fn global_max_commission(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::per_things::Perbill,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"GlobalMaxCommission",
						vec![],
						[
							142u8, 252u8, 92u8, 128u8, 162u8, 4u8, 216u8, 39u8, 118u8, 201u8,
							138u8, 171u8, 76u8, 90u8, 133u8, 176u8, 161u8, 138u8, 214u8, 183u8,
							193u8, 115u8, 245u8, 151u8, 216u8, 84u8, 99u8, 175u8, 144u8, 196u8,
							103u8, 190u8,
						],
					)
				}

				#[doc = " Active members."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn pool_members(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_nomination_pools::PoolMember,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"PoolMembers",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							252u8, 236u8, 201u8, 127u8, 219u8, 1u8, 19u8, 144u8, 5u8, 108u8, 70u8,
							30u8, 177u8, 232u8, 253u8, 237u8, 211u8, 91u8, 63u8, 62u8, 155u8,
							151u8, 153u8, 165u8, 206u8, 53u8, 111u8, 31u8, 60u8, 120u8, 100u8,
							249u8,
						],
					)
				}

				#[doc = " Active members."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn pool_members_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_nomination_pools::PoolMember,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"PoolMembers",
						Vec::new(),
						[
							252u8, 236u8, 201u8, 127u8, 219u8, 1u8, 19u8, 144u8, 5u8, 108u8, 70u8,
							30u8, 177u8, 232u8, 253u8, 237u8, 211u8, 91u8, 63u8, 62u8, 155u8,
							151u8, 153u8, 165u8, 206u8, 53u8, 111u8, 31u8, 60u8, 120u8, 100u8,
							249u8,
						],
					)
				}

				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_pool_members(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"CounterForPoolMembers",
						vec![],
						[
							114u8, 126u8, 27u8, 138u8, 119u8, 44u8, 45u8, 129u8, 84u8, 107u8,
							171u8, 206u8, 117u8, 141u8, 20u8, 75u8, 229u8, 237u8, 31u8, 229u8,
							124u8, 190u8, 27u8, 124u8, 63u8, 59u8, 167u8, 42u8, 62u8, 212u8, 160u8,
							2u8,
						],
					)
				}

				#[doc = " Storage for bonded pools."]
				pub fn bonded_pools(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_nomination_pools::BondedPoolInner,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"BondedPools",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							3u8, 183u8, 140u8, 154u8, 74u8, 225u8, 69u8, 243u8, 150u8, 132u8,
							163u8, 26u8, 101u8, 45u8, 231u8, 178u8, 85u8, 144u8, 9u8, 112u8, 212u8,
							167u8, 131u8, 188u8, 203u8, 50u8, 177u8, 218u8, 154u8, 182u8, 80u8,
							232u8,
						],
					)
				}

				#[doc = " Storage for bonded pools."]
				pub fn bonded_pools_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_nomination_pools::BondedPoolInner,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"BondedPools",
						Vec::new(),
						[
							3u8, 183u8, 140u8, 154u8, 74u8, 225u8, 69u8, 243u8, 150u8, 132u8,
							163u8, 26u8, 101u8, 45u8, 231u8, 178u8, 85u8, 144u8, 9u8, 112u8, 212u8,
							167u8, 131u8, 188u8, 203u8, 50u8, 177u8, 218u8, 154u8, 182u8, 80u8,
							232u8,
						],
					)
				}

				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_bonded_pools(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"CounterForBondedPools",
						vec![],
						[
							134u8, 94u8, 199u8, 73u8, 174u8, 253u8, 66u8, 242u8, 233u8, 244u8,
							140u8, 170u8, 242u8, 40u8, 41u8, 185u8, 183u8, 151u8, 58u8, 111u8,
							221u8, 225u8, 81u8, 71u8, 169u8, 219u8, 223u8, 135u8, 8u8, 171u8,
							180u8, 236u8,
						],
					)
				}

				#[doc = " Reward pools. This is where there rewards for each pool accumulate. When a members payout is"]
				#[doc = " claimed, the balance comes out fo the reward pool. Keyed by the bonded pools account."]
				pub fn reward_pools(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_nomination_pools::RewardPool,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"RewardPools",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							235u8, 6u8, 2u8, 103u8, 137u8, 31u8, 109u8, 165u8, 129u8, 48u8, 154u8,
							219u8, 110u8, 198u8, 241u8, 31u8, 174u8, 10u8, 92u8, 233u8, 161u8,
							76u8, 53u8, 136u8, 172u8, 214u8, 192u8, 12u8, 239u8, 165u8, 195u8,
							96u8,
						],
					)
				}

				#[doc = " Reward pools. This is where there rewards for each pool accumulate. When a members payout is"]
				#[doc = " claimed, the balance comes out fo the reward pool. Keyed by the bonded pools account."]
				pub fn reward_pools_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_nomination_pools::RewardPool,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"RewardPools",
						Vec::new(),
						[
							235u8, 6u8, 2u8, 103u8, 137u8, 31u8, 109u8, 165u8, 129u8, 48u8, 154u8,
							219u8, 110u8, 198u8, 241u8, 31u8, 174u8, 10u8, 92u8, 233u8, 161u8,
							76u8, 53u8, 136u8, 172u8, 214u8, 192u8, 12u8, 239u8, 165u8, 195u8,
							96u8,
						],
					)
				}

				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_reward_pools(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"CounterForRewardPools",
						vec![],
						[
							209u8, 139u8, 212u8, 116u8, 210u8, 178u8, 213u8, 38u8, 75u8, 23u8,
							188u8, 57u8, 253u8, 213u8, 95u8, 118u8, 182u8, 250u8, 45u8, 205u8,
							17u8, 175u8, 17u8, 201u8, 234u8, 14u8, 98u8, 49u8, 143u8, 135u8, 201u8,
							81u8,
						],
					)
				}

				#[doc = " Groups of unbonding pools. Each group of unbonding pools belongs to a"]
				#[doc = " bonded pool, hence the name sub-pools. Keyed by the bonded pools account."]
				pub fn sub_pools_storage(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_nomination_pools::SubPools,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"SubPoolsStorage",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							231u8, 13u8, 111u8, 248u8, 1u8, 208u8, 179u8, 134u8, 224u8, 196u8,
							94u8, 201u8, 229u8, 29u8, 155u8, 211u8, 163u8, 150u8, 157u8, 34u8,
							68u8, 238u8, 55u8, 4u8, 222u8, 96u8, 186u8, 29u8, 205u8, 237u8, 80u8,
							42u8,
						],
					)
				}

				#[doc = " Groups of unbonding pools. Each group of unbonding pools belongs to a"]
				#[doc = " bonded pool, hence the name sub-pools. Keyed by the bonded pools account."]
				pub fn sub_pools_storage_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_nomination_pools::SubPools,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"SubPoolsStorage",
						Vec::new(),
						[
							231u8, 13u8, 111u8, 248u8, 1u8, 208u8, 179u8, 134u8, 224u8, 196u8,
							94u8, 201u8, 229u8, 29u8, 155u8, 211u8, 163u8, 150u8, 157u8, 34u8,
							68u8, 238u8, 55u8, 4u8, 222u8, 96u8, 186u8, 29u8, 205u8, 237u8, 80u8,
							42u8,
						],
					)
				}

				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_sub_pools_storage(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"CounterForSubPoolsStorage",
						vec![],
						[
							212u8, 145u8, 212u8, 226u8, 234u8, 31u8, 26u8, 240u8, 107u8, 91u8,
							171u8, 120u8, 41u8, 195u8, 16u8, 86u8, 55u8, 127u8, 103u8, 93u8, 128u8,
							48u8, 69u8, 104u8, 168u8, 236u8, 81u8, 54u8, 2u8, 184u8, 215u8, 51u8,
						],
					)
				}

				#[doc = " Metadata for the pool."]
				pub fn metadata(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"Metadata",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							108u8, 250u8, 163u8, 54u8, 192u8, 143u8, 239u8, 62u8, 97u8, 163u8,
							161u8, 215u8, 171u8, 225u8, 49u8, 18u8, 37u8, 200u8, 143u8, 254u8,
							136u8, 26u8, 54u8, 187u8, 39u8, 3u8, 216u8, 24u8, 188u8, 25u8, 243u8,
							251u8,
						],
					)
				}

				#[doc = " Metadata for the pool."]
				pub fn metadata_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"Metadata",
						Vec::new(),
						[
							108u8, 250u8, 163u8, 54u8, 192u8, 143u8, 239u8, 62u8, 97u8, 163u8,
							161u8, 215u8, 171u8, 225u8, 49u8, 18u8, 37u8, 200u8, 143u8, 254u8,
							136u8, 26u8, 54u8, 187u8, 39u8, 3u8, 216u8, 24u8, 188u8, 25u8, 243u8,
							251u8,
						],
					)
				}

				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_metadata(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"CounterForMetadata",
						vec![],
						[
							190u8, 232u8, 77u8, 134u8, 245u8, 89u8, 160u8, 187u8, 163u8, 68u8,
							188u8, 204u8, 31u8, 145u8, 219u8, 165u8, 213u8, 1u8, 167u8, 90u8,
							175u8, 218u8, 147u8, 144u8, 158u8, 226u8, 23u8, 233u8, 55u8, 168u8,
							161u8, 237u8,
						],
					)
				}

				#[doc = " Ever increasing number of all pools created so far."]
				pub fn last_pool_id(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"LastPoolId",
						vec![],
						[
							50u8, 254u8, 218u8, 41u8, 213u8, 184u8, 170u8, 166u8, 31u8, 29u8,
							196u8, 57u8, 215u8, 20u8, 40u8, 40u8, 19u8, 22u8, 9u8, 184u8, 11u8,
							21u8, 21u8, 125u8, 97u8, 38u8, 219u8, 209u8, 2u8, 238u8, 247u8, 51u8,
						],
					)
				}

				#[doc = " A reverse lookup from the pool's account id to its id."]
				#[doc = ""]
				#[doc = " This is only used for slashing. In all other instances, the pool id is used, and the"]
				#[doc = " accounts are deterministically derived from it."]
				pub fn reverse_pool_id_lookup(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"ReversePoolIdLookup",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							178u8, 161u8, 51u8, 220u8, 128u8, 1u8, 135u8, 83u8, 236u8, 159u8, 36u8,
							237u8, 120u8, 128u8, 6u8, 191u8, 41u8, 159u8, 94u8, 178u8, 174u8,
							235u8, 221u8, 173u8, 44u8, 81u8, 211u8, 255u8, 231u8, 81u8, 16u8, 87u8,
						],
					)
				}

				#[doc = " A reverse lookup from the pool's account id to its id."]
				#[doc = ""]
				#[doc = " This is only used for slashing. In all other instances, the pool id is used, and the"]
				#[doc = " accounts are deterministically derived from it."]
				pub fn reverse_pool_id_lookup_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"ReversePoolIdLookup",
						Vec::new(),
						[
							178u8, 161u8, 51u8, 220u8, 128u8, 1u8, 135u8, 83u8, 236u8, 159u8, 36u8,
							237u8, 120u8, 128u8, 6u8, 191u8, 41u8, 159u8, 94u8, 178u8, 174u8,
							235u8, 221u8, 173u8, 44u8, 81u8, 211u8, 255u8, 231u8, 81u8, 16u8, 87u8,
						],
					)
				}

				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_reverse_pool_id_lookup(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"CounterForReversePoolIdLookup",
						vec![],
						[
							148u8, 83u8, 81u8, 33u8, 188u8, 72u8, 148u8, 208u8, 245u8, 178u8, 52u8,
							245u8, 229u8, 140u8, 100u8, 152u8, 8u8, 217u8, 161u8, 80u8, 226u8,
							42u8, 15u8, 252u8, 90u8, 197u8, 120u8, 114u8, 144u8, 90u8, 199u8,
							123u8,
						],
					)
				}

				#[doc = " Map from a pool member account to their opted claim permission."]
				pub fn claim_permissions(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_nomination_pools::ClaimPermission,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"ClaimPermissions",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							23u8, 124u8, 83u8, 109u8, 174u8, 228u8, 170u8, 25u8, 124u8, 91u8,
							224u8, 66u8, 55u8, 127u8, 190u8, 226u8, 163u8, 16u8, 81u8, 231u8,
							241u8, 214u8, 209u8, 137u8, 101u8, 206u8, 104u8, 138u8, 49u8, 56u8,
							152u8, 228u8,
						],
					)
				}

				#[doc = " Map from a pool member account to their opted claim permission."]
				pub fn claim_permissions_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_nomination_pools::ClaimPermission,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"NominationPools",
						"ClaimPermissions",
						Vec::new(),
						[
							23u8, 124u8, 83u8, 109u8, 174u8, 228u8, 170u8, 25u8, 124u8, 91u8,
							224u8, 66u8, 55u8, 127u8, 190u8, 226u8, 163u8, 16u8, 81u8, 231u8,
							241u8, 214u8, 209u8, 137u8, 101u8, 206u8, 104u8, 138u8, 49u8, 56u8,
							152u8, 228u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The nomination pool's pallet id."]
				pub fn pallet_id(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::frame_support::PalletId>,
				> {
					::subxt::constants::StaticConstantAddress::new("NominationPools", "PalletId", [
						139u8, 109u8, 228u8, 151u8, 252u8, 32u8, 130u8, 69u8, 112u8, 154u8, 174u8,
						45u8, 83u8, 245u8, 51u8, 132u8, 173u8, 5u8, 186u8, 24u8, 243u8, 9u8, 12u8,
						214u8, 80u8, 74u8, 69u8, 189u8, 30u8, 94u8, 22u8, 39u8,
					])
				}

				#[doc = " The maximum pool points-to-balance ratio that an `open` pool can have."]
				#[doc = ""]
				#[doc = " This is important in the event slashing takes place and the pool's points-to-balance"]
				#[doc = " ratio becomes disproportional."]
				#[doc = ""]
				#[doc = " Moreover, this relates to the `RewardCounter` type as well, as the arithmetic operations"]
				#[doc = " are a function of number of points, and by setting this value to e.g. 10, you ensure"]
				#[doc = " that the total number of points in the system are at most 10 times the total_issuance of"]
				#[doc = " the chain, in the absolute worse case."]
				#[doc = ""]
				#[doc = " For a value of 10, the threshold would be a pool points-to-balance ratio of 10:1."]
				#[doc = " Such a scenario would also be the equivalent of the pool being 90% slashed."]
				pub fn max_points_to_balance(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u8>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"NominationPools",
						"MaxPointsToBalance",
						[
							141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
							28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
							114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
							165u8,
						],
					)
				}
			}
		}
	}
	pub mod identity {
		use super::{root_mod, runtime_types};
		#[doc = "Identity pallet declaration."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct AddRegistrar {
				pub account: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetIdentity {
				pub info: ::std::boxed::Box<runtime_types::pallet_identity::types::IdentityInfo>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetSubs {
				pub subs: ::std::vec::Vec<(
					::subxt::ext::sp_core::crypto::AccountId32,
					runtime_types::pallet_identity::types::Data,
				)>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ClearIdentity;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct RequestJudgement {
				#[codec(compact)]
				pub reg_index: ::core::primitive::u32,
				#[codec(compact)]
				pub max_fee: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct CancelRequest {
				pub reg_index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetFee {
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				#[codec(compact)]
				pub fee: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetAccountId {
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub new: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SetFields {
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub fields: runtime_types::pallet_identity::types::BitFlags<
					runtime_types::pallet_identity::types::IdentityField,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ProvideJudgement {
				#[codec(compact)]
				pub reg_index: ::core::primitive::u32,
				pub target: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub judgement:
					runtime_types::pallet_identity::types::Judgement<::core::primitive::u128>,
				pub identity: ::subxt::ext::sp_core::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct KillIdentity {
				pub target: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct AddSub {
				pub sub: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub data: runtime_types::pallet_identity::types::Data,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct RenameSub {
				pub sub: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub data: runtime_types::pallet_identity::types::Data,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct RemoveSub {
				pub sub: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct QuitSub;
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::add_registrar`]."]
				pub fn add_registrar(
					&self,
					account: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<AddRegistrar> {
					::subxt::tx::StaticTxPayload::new(
						"Identity",
						"add_registrar",
						AddRegistrar { account },
						[
							96u8, 200u8, 92u8, 23u8, 3u8, 144u8, 56u8, 53u8, 245u8, 210u8, 33u8,
							36u8, 183u8, 233u8, 41u8, 1u8, 127u8, 2u8, 25u8, 5u8, 15u8, 133u8, 4u8,
							107u8, 206u8, 155u8, 114u8, 39u8, 14u8, 235u8, 115u8, 172u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_identity`]."]
				pub fn set_identity(
					&self,
					info: runtime_types::pallet_identity::types::IdentityInfo,
				) -> ::subxt::tx::StaticTxPayload<SetIdentity> {
					::subxt::tx::StaticTxPayload::new(
						"Identity",
						"set_identity",
						SetIdentity {
							info: ::std::boxed::Box::new(info),
						},
						[
							130u8, 89u8, 118u8, 6u8, 134u8, 166u8, 35u8, 192u8, 73u8, 6u8, 171u8,
							20u8, 225u8, 255u8, 152u8, 142u8, 111u8, 8u8, 206u8, 200u8, 64u8, 52u8,
							110u8, 123u8, 42u8, 101u8, 191u8, 242u8, 133u8, 139u8, 154u8, 205u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_subs`]."]
				pub fn set_subs(
					&self,
					subs: ::std::vec::Vec<(
						::subxt::ext::sp_core::crypto::AccountId32,
						runtime_types::pallet_identity::types::Data,
					)>,
				) -> ::subxt::tx::StaticTxPayload<SetSubs> {
					::subxt::tx::StaticTxPayload::new("Identity", "set_subs", SetSubs { subs }, [
						177u8, 219u8, 84u8, 183u8, 5u8, 32u8, 192u8, 82u8, 174u8, 68u8, 198u8,
						224u8, 56u8, 85u8, 134u8, 171u8, 30u8, 132u8, 140u8, 236u8, 117u8, 24u8,
						150u8, 218u8, 146u8, 194u8, 144u8, 92u8, 103u8, 206u8, 46u8, 90u8,
					])
				}

				#[doc = "See [`Pallet::clear_identity`]."]
				pub fn clear_identity(&self) -> ::subxt::tx::StaticTxPayload<ClearIdentity> {
					::subxt::tx::StaticTxPayload::new(
						"Identity",
						"clear_identity",
						ClearIdentity {},
						[
							75u8, 44u8, 74u8, 122u8, 149u8, 202u8, 114u8, 230u8, 0u8, 255u8, 140u8,
							122u8, 14u8, 196u8, 205u8, 249u8, 220u8, 94u8, 216u8, 34u8, 63u8, 14u8,
							8u8, 205u8, 74u8, 23u8, 181u8, 129u8, 252u8, 110u8, 231u8, 114u8,
						],
					)
				}

				#[doc = "See [`Pallet::request_judgement`]."]
				pub fn request_judgement(
					&self,
					reg_index: ::core::primitive::u32,
					max_fee: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<RequestJudgement> {
					::subxt::tx::StaticTxPayload::new(
						"Identity",
						"request_judgement",
						RequestJudgement { reg_index, max_fee },
						[
							186u8, 149u8, 61u8, 54u8, 159u8, 194u8, 77u8, 161u8, 220u8, 157u8, 3u8,
							216u8, 23u8, 105u8, 119u8, 76u8, 144u8, 198u8, 157u8, 45u8, 235u8,
							139u8, 87u8, 82u8, 81u8, 12u8, 25u8, 134u8, 225u8, 92u8, 182u8, 101u8,
						],
					)
				}

				#[doc = "See [`Pallet::cancel_request`]."]
				pub fn cancel_request(
					&self,
					reg_index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<CancelRequest> {
					::subxt::tx::StaticTxPayload::new(
						"Identity",
						"cancel_request",
						CancelRequest { reg_index },
						[
							83u8, 180u8, 239u8, 126u8, 32u8, 51u8, 17u8, 20u8, 180u8, 3u8, 59u8,
							96u8, 24u8, 32u8, 136u8, 92u8, 58u8, 254u8, 68u8, 70u8, 50u8, 11u8,
							51u8, 91u8, 180u8, 79u8, 81u8, 84u8, 216u8, 138u8, 6u8, 215u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_fee`]."]
				pub fn set_fee(
					&self,
					index: ::core::primitive::u32,
					fee: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<SetFee> {
					::subxt::tx::StaticTxPayload::new(
						"Identity",
						"set_fee",
						SetFee { index, fee },
						[
							21u8, 157u8, 123u8, 182u8, 160u8, 190u8, 117u8, 37u8, 136u8, 133u8,
							104u8, 234u8, 31u8, 145u8, 115u8, 154u8, 125u8, 40u8, 2u8, 87u8, 118u8,
							56u8, 247u8, 73u8, 89u8, 0u8, 251u8, 3u8, 58u8, 105u8, 239u8, 211u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_account_id`]."]
				pub fn set_account_id(
					&self,
					index: ::core::primitive::u32,
					new: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetAccountId> {
					::subxt::tx::StaticTxPayload::new(
						"Identity",
						"set_account_id",
						SetAccountId { index, new },
						[
							14u8, 154u8, 84u8, 48u8, 59u8, 133u8, 45u8, 204u8, 255u8, 85u8, 157u8,
							88u8, 56u8, 207u8, 113u8, 184u8, 233u8, 139u8, 129u8, 118u8, 59u8, 9u8,
							211u8, 184u8, 32u8, 141u8, 126u8, 208u8, 179u8, 4u8, 2u8, 95u8,
						],
					)
				}

				#[doc = "See [`Pallet::set_fields`]."]
				pub fn set_fields(
					&self,
					index: ::core::primitive::u32,
					fields: runtime_types::pallet_identity::types::BitFlags<
						runtime_types::pallet_identity::types::IdentityField,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetFields> {
					::subxt::tx::StaticTxPayload::new(
						"Identity",
						"set_fields",
						SetFields { index, fields },
						[
							50u8, 196u8, 179u8, 71u8, 66u8, 65u8, 235u8, 7u8, 51u8, 14u8, 81u8,
							173u8, 201u8, 58u8, 6u8, 151u8, 174u8, 245u8, 102u8, 184u8, 28u8, 84u8,
							125u8, 93u8, 126u8, 134u8, 92u8, 203u8, 200u8, 129u8, 240u8, 252u8,
						],
					)
				}

				#[doc = "See [`Pallet::provide_judgement`]."]
				pub fn provide_judgement(
					&self,
					reg_index: ::core::primitive::u32,
					target: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					judgement: runtime_types::pallet_identity::types::Judgement<
						::core::primitive::u128,
					>,
					identity: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<ProvideJudgement> {
					::subxt::tx::StaticTxPayload::new(
						"Identity",
						"provide_judgement",
						ProvideJudgement {
							reg_index,
							target,
							judgement,
							identity,
						},
						[
							83u8, 253u8, 77u8, 208u8, 198u8, 25u8, 202u8, 213u8, 223u8, 184u8,
							231u8, 185u8, 186u8, 216u8, 54u8, 62u8, 3u8, 7u8, 107u8, 152u8, 126u8,
							195u8, 175u8, 221u8, 134u8, 169u8, 199u8, 124u8, 232u8, 157u8, 67u8,
							75u8,
						],
					)
				}

				#[doc = "See [`Pallet::kill_identity`]."]
				pub fn kill_identity(
					&self,
					target: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<KillIdentity> {
					::subxt::tx::StaticTxPayload::new(
						"Identity",
						"kill_identity",
						KillIdentity { target },
						[
							65u8, 106u8, 116u8, 209u8, 219u8, 181u8, 185u8, 75u8, 146u8, 194u8,
							187u8, 170u8, 7u8, 34u8, 140u8, 87u8, 107u8, 112u8, 229u8, 34u8, 65u8,
							71u8, 58u8, 152u8, 74u8, 253u8, 137u8, 69u8, 149u8, 214u8, 158u8, 19u8,
						],
					)
				}

				#[doc = "See [`Pallet::add_sub`]."]
				pub fn add_sub(
					&self,
					sub: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					data: runtime_types::pallet_identity::types::Data,
				) -> ::subxt::tx::StaticTxPayload<AddSub> {
					::subxt::tx::StaticTxPayload::new(
						"Identity",
						"add_sub",
						AddSub { sub, data },
						[
							206u8, 112u8, 143u8, 96u8, 152u8, 12u8, 174u8, 226u8, 23u8, 27u8,
							154u8, 188u8, 195u8, 233u8, 185u8, 180u8, 246u8, 218u8, 154u8, 129u8,
							138u8, 52u8, 212u8, 109u8, 54u8, 211u8, 219u8, 255u8, 39u8, 79u8,
							154u8, 123u8,
						],
					)
				}

				#[doc = "See [`Pallet::rename_sub`]."]
				pub fn rename_sub(
					&self,
					sub: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					data: runtime_types::pallet_identity::types::Data,
				) -> ::subxt::tx::StaticTxPayload<RenameSub> {
					::subxt::tx::StaticTxPayload::new(
						"Identity",
						"rename_sub",
						RenameSub { sub, data },
						[
							110u8, 28u8, 134u8, 225u8, 44u8, 242u8, 20u8, 22u8, 197u8, 49u8, 173u8,
							178u8, 106u8, 181u8, 103u8, 90u8, 27u8, 73u8, 102u8, 130u8, 2u8, 216u8,
							172u8, 186u8, 124u8, 244u8, 128u8, 6u8, 112u8, 128u8, 25u8, 245u8,
						],
					)
				}

				#[doc = "See [`Pallet::remove_sub`]."]
				pub fn remove_sub(
					&self,
					sub: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<RemoveSub> {
					::subxt::tx::StaticTxPayload::new(
						"Identity",
						"remove_sub",
						RemoveSub { sub },
						[
							92u8, 201u8, 70u8, 170u8, 248u8, 110u8, 179u8, 186u8, 213u8, 197u8,
							150u8, 156u8, 156u8, 50u8, 19u8, 158u8, 186u8, 61u8, 106u8, 64u8, 84u8,
							38u8, 73u8, 134u8, 132u8, 233u8, 50u8, 152u8, 40u8, 18u8, 212u8, 121u8,
						],
					)
				}

				#[doc = "See [`Pallet::quit_sub`]."]
				pub fn quit_sub(&self) -> ::subxt::tx::StaticTxPayload<QuitSub> {
					::subxt::tx::StaticTxPayload::new("Identity", "quit_sub", QuitSub {}, [
						62u8, 57u8, 73u8, 72u8, 119u8, 216u8, 250u8, 155u8, 57u8, 169u8, 157u8,
						44u8, 87u8, 51u8, 63u8, 231u8, 77u8, 7u8, 0u8, 119u8, 244u8, 42u8, 179u8,
						51u8, 254u8, 240u8, 55u8, 25u8, 142u8, 38u8, 87u8, 44u8,
					])
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_identity::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A name was set or reset (which will remove all judgements)."]
			pub struct IdentitySet {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for IdentitySet {
				const EVENT: &'static str = "IdentitySet";
				const PALLET: &'static str = "Identity";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A name was cleared, and the given balance returned."]
			pub struct IdentityCleared {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for IdentityCleared {
				const EVENT: &'static str = "IdentityCleared";
				const PALLET: &'static str = "Identity";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A name was removed and the given balance slashed."]
			pub struct IdentityKilled {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for IdentityKilled {
				const EVENT: &'static str = "IdentityKilled";
				const PALLET: &'static str = "Identity";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A judgement was asked from a registrar."]
			pub struct JudgementRequested {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub registrar_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for JudgementRequested {
				const EVENT: &'static str = "JudgementRequested";
				const PALLET: &'static str = "Identity";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A judgement request was retracted."]
			pub struct JudgementUnrequested {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub registrar_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for JudgementUnrequested {
				const EVENT: &'static str = "JudgementUnrequested";
				const PALLET: &'static str = "Identity";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A judgement was given by a registrar."]
			pub struct JudgementGiven {
				pub target: ::subxt::ext::sp_core::crypto::AccountId32,
				pub registrar_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for JudgementGiven {
				const EVENT: &'static str = "JudgementGiven";
				const PALLET: &'static str = "Identity";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A registrar was added."]
			pub struct RegistrarAdded {
				pub registrar_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for RegistrarAdded {
				const EVENT: &'static str = "RegistrarAdded";
				const PALLET: &'static str = "Identity";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A sub-identity was added to an identity and the deposit paid."]
			pub struct SubIdentityAdded {
				pub sub: ::subxt::ext::sp_core::crypto::AccountId32,
				pub main: ::subxt::ext::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for SubIdentityAdded {
				const EVENT: &'static str = "SubIdentityAdded";
				const PALLET: &'static str = "Identity";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A sub-identity was removed from an identity and the deposit freed."]
			pub struct SubIdentityRemoved {
				pub sub: ::subxt::ext::sp_core::crypto::AccountId32,
				pub main: ::subxt::ext::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for SubIdentityRemoved {
				const EVENT: &'static str = "SubIdentityRemoved";
				const PALLET: &'static str = "Identity";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A sub-identity was cleared, and the given deposit repatriated from the"]
			#[doc = "main identity account to the sub-identity account."]
			pub struct SubIdentityRevoked {
				pub sub: ::subxt::ext::sp_core::crypto::AccountId32,
				pub main: ::subxt::ext::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for SubIdentityRevoked {
				const EVENT: &'static str = "SubIdentityRevoked";
				const PALLET: &'static str = "Identity";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Information that is pertinent to identify the entity behind an account."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]
				pub fn identity_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_identity::types::Registration<
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Identity",
						"IdentityOf",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							193u8, 195u8, 180u8, 188u8, 129u8, 250u8, 180u8, 219u8, 22u8, 95u8,
							175u8, 170u8, 143u8, 188u8, 80u8, 124u8, 234u8, 228u8, 245u8, 39u8,
							72u8, 153u8, 107u8, 199u8, 23u8, 75u8, 47u8, 247u8, 104u8, 208u8,
							171u8, 82u8,
						],
					)
				}

				#[doc = " Information that is pertinent to identify the entity behind an account."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]
				pub fn identity_of_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_identity::types::Registration<
							::core::primitive::u128,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Identity",
						"IdentityOf",
						Vec::new(),
						[
							193u8, 195u8, 180u8, 188u8, 129u8, 250u8, 180u8, 219u8, 22u8, 95u8,
							175u8, 170u8, 143u8, 188u8, 80u8, 124u8, 234u8, 228u8, 245u8, 39u8,
							72u8, 153u8, 107u8, 199u8, 23u8, 75u8, 47u8, 247u8, 104u8, 208u8,
							171u8, 82u8,
						],
					)
				}

				#[doc = " The super-identity of an alternative \"sub\" identity together with its name, within that"]
				#[doc = " context. If the account is not some other account's sub-identity, then just `None`."]
				pub fn super_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::subxt::ext::sp_core::crypto::AccountId32,
						runtime_types::pallet_identity::types::Data,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Identity",
						"SuperOf",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							170u8, 249u8, 112u8, 249u8, 75u8, 176u8, 21u8, 29u8, 152u8, 149u8,
							69u8, 113u8, 20u8, 92u8, 113u8, 130u8, 135u8, 62u8, 18u8, 204u8, 166u8,
							193u8, 133u8, 167u8, 248u8, 117u8, 80u8, 137u8, 158u8, 111u8, 100u8,
							137u8,
						],
					)
				}

				#[doc = " The super-identity of an alternative \"sub\" identity together with its name, within that"]
				#[doc = " context. If the account is not some other account's sub-identity, then just `None`."]
				pub fn super_of_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::subxt::ext::sp_core::crypto::AccountId32,
						runtime_types::pallet_identity::types::Data,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Identity",
						"SuperOf",
						Vec::new(),
						[
							170u8, 249u8, 112u8, 249u8, 75u8, 176u8, 21u8, 29u8, 152u8, 149u8,
							69u8, 113u8, 20u8, 92u8, 113u8, 130u8, 135u8, 62u8, 18u8, 204u8, 166u8,
							193u8, 133u8, 167u8, 248u8, 117u8, 80u8, 137u8, 158u8, 111u8, 100u8,
							137u8,
						],
					)
				}

				#[doc = " Alternative \"sub\" identities of this account."]
				#[doc = ""]
				#[doc = " The first item is the deposit, the second is a vector of the accounts."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]
				pub fn subs_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u128,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Identity",
						"SubsOf",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							128u8, 15u8, 175u8, 155u8, 216u8, 225u8, 200u8, 169u8, 215u8, 206u8,
							110u8, 22u8, 204u8, 89u8, 212u8, 210u8, 159u8, 169u8, 53u8, 7u8, 44u8,
							164u8, 91u8, 151u8, 7u8, 227u8, 38u8, 230u8, 175u8, 84u8, 6u8, 4u8,
						],
					)
				}

				#[doc = " Alternative \"sub\" identities of this account."]
				#[doc = ""]
				#[doc = " The first item is the deposit, the second is a vector of the accounts."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]
				pub fn subs_of_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u128,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					)>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Identity",
						"SubsOf",
						Vec::new(),
						[
							128u8, 15u8, 175u8, 155u8, 216u8, 225u8, 200u8, 169u8, 215u8, 206u8,
							110u8, 22u8, 204u8, 89u8, 212u8, 210u8, 159u8, 169u8, 53u8, 7u8, 44u8,
							164u8, 91u8, 151u8, 7u8, 227u8, 38u8, 230u8, 175u8, 84u8, 6u8, 4u8,
						],
					)
				}

				#[doc = " The set of registrars. Not expected to get very big as can only be added through a"]
				#[doc = " special origin (likely a council motion)."]
				#[doc = ""]
				#[doc = " The index into this can be cast to `RegistrarIndex` to get a valid value."]
				pub fn registrars(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::option::Option<
								runtime_types::pallet_identity::types::RegistrarInfo<
									::core::primitive::u128,
									::subxt::ext::sp_core::crypto::AccountId32,
								>,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Identity",
						"Registrars",
						vec![],
						[
							157u8, 87u8, 39u8, 240u8, 154u8, 54u8, 241u8, 229u8, 76u8, 9u8, 62u8,
							252u8, 40u8, 143u8, 186u8, 182u8, 233u8, 187u8, 251u8, 61u8, 236u8,
							229u8, 19u8, 55u8, 42u8, 36u8, 82u8, 173u8, 215u8, 155u8, 229u8, 111u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The amount held on deposit for a registered identity"]
				pub fn basic_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new("Identity", "BasicDeposit", [
						84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
						27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
						71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
					])
				}

				#[doc = " The amount held on deposit per additional field for a registered identity."]
				pub fn field_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new("Identity", "FieldDeposit", [
						84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
						27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
						71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
					])
				}

				#[doc = " The amount held on deposit for a registered subaccount. This should account for the fact"]
				#[doc = " that one storage item's value will increase by the size of an account ID, and there will"]
				#[doc = " be another trie item whose value is the size of an account ID plus 32 bytes."]
				pub fn sub_account_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Identity",
						"SubAccountDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}

				#[doc = " The maximum number of sub-accounts allowed per identified account."]
				pub fn max_sub_accounts(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Identity", "MaxSubAccounts", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " Maximum number of additional fields that may be stored in an ID. Needed to bound the I/O"]
				#[doc = " required to access an identity, but can be pretty high."]
				pub fn max_additional_fields(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Identity",
						"MaxAdditionalFields",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " Maxmimum number of registrars allowed in the system. Needed to bound the complexity"]
				#[doc = " of, e.g., updating judgements."]
				pub fn max_registrars(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Identity", "MaxRegistrars", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}
			}
		}
	}
	pub mod mandate {
		use super::{root_mod, runtime_types};
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Mandate {
				pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::mandate`]."]
				pub fn mandate(
					&self,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<Mandate> {
					::subxt::tx::StaticTxPayload::new(
						"Mandate",
						"mandate",
						Mandate {
							call: ::std::boxed::Box::new(call),
						},
						[
							66u8, 33u8, 234u8, 189u8, 72u8, 1u8, 230u8, 92u8, 183u8, 229u8, 247u8,
							243u8, 54u8, 14u8, 54u8, 11u8, 125u8, 131u8, 28u8, 41u8, 222u8, 104u8,
							102u8, 13u8, 252u8, 23u8, 28u8, 248u8, 211u8, 154u8, 184u8, 180u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_mandate::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A root operation was executed, show result"]
			pub struct RootOp {
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for RootOp {
				const EVENT: &'static str = "RootOp";
				const PALLET: &'static str = "Mandate";
			}
		}
	}
	pub mod runtime_types {
		use super::runtime_types;
		pub mod avail_core {
			use super::runtime_types;
			pub mod asdr {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct AppUncheckedExtrinsic<_0, _1, _2, _3>(
					pub ::std::vec::Vec<::core::primitive::u8>,
					#[codec(skip)] pub ::core::marker::PhantomData<(_1, _0, _2, _3)>,
				);
			}
			pub mod data_lookup {
				use super::runtime_types;
				pub mod compact {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub struct CompactDataLookup {
						#[codec(compact)]
						pub size: ::core::primitive::u32,
						pub index: ::std::vec::Vec<
							runtime_types::avail_core::data_lookup::compact::DataLookupItem,
						>,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					#[serde(rename_all = "camelCase")]
					#[serde(rename_all = "camelCase")]
					pub struct DataLookupItem {
						pub app_id: runtime_types::avail_core::AppId,
						#[codec(compact)]
						pub start: ::core::primitive::u32,
					}
				}
			}
			pub mod header {
				use super::runtime_types;
				pub mod extension {
					use super::runtime_types;
					pub mod v1 {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							Clone,
							Debug,
							Eq,
							PartialEq,
						)]
						#[serde(rename_all = "camelCase")]
						pub struct HeaderExtension {
							pub commitment:
								runtime_types::avail_core::kate_commitment::v1::KateCommitment,
							pub app_lookup:
								runtime_types::avail_core::data_lookup::compact::CompactDataLookup,
						}
					}
					pub mod v2 {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							Clone,
							Debug,
							Eq,
							PartialEq,
						)]
						pub struct HeaderExtension {
							pub commitment:
								runtime_types::avail_core::kate_commitment::v2::KateCommitment,
							pub app_lookup:
								runtime_types::avail_core::data_lookup::compact::CompactDataLookup,
						}
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub enum HeaderExtension {
						#[codec(index = 0)]
						V1(runtime_types::avail_core::header::extension::v1::HeaderExtension),
						#[codec(index = 1)]
						V2(runtime_types::avail_core::header::extension::v2::HeaderExtension),
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Header<_0, _1> {
					pub parent_hash: ::subxt::ext::sp_core::H256,
					#[codec(compact)]
					pub number: _0,
					pub state_root: ::subxt::ext::sp_core::H256,
					pub extrinsics_root: ::subxt::ext::sp_core::H256,
					pub digest: runtime_types::sp_runtime::generic::digest::Digest,
					pub extension: runtime_types::avail_core::header::extension::HeaderExtension,
					#[codec(skip)]
					pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
				}
			}
			pub mod kate_commitment {
				use super::runtime_types;
				pub mod v1 {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					#[serde(rename_all = "camelCase")]
					pub struct KateCommitment {
						#[codec(compact)]
						pub rows: ::core::primitive::u16,
						#[codec(compact)]
						pub cols: ::core::primitive::u16,
						pub data_root: ::subxt::ext::sp_core::H256,
						pub commitment: ::std::vec::Vec<::core::primitive::u8>,
					}
				}
				pub mod v2 {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub struct KateCommitment {
						#[codec(compact)]
						pub rows: ::core::primitive::u16,
						#[codec(compact)]
						pub cols: ::core::primitive::u16,
						pub data_root: ::core::option::Option<::subxt::ext::sp_core::H256>,
						pub commitment: ::std::vec::Vec<::core::primitive::u8>,
					}
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct AppId(#[codec(compact)] pub ::core::primitive::u32);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct BlockLengthColumns(#[codec(compact)] pub ::core::primitive::u32);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct BlockLengthRows(#[codec(compact)] pub ::core::primitive::u32);
		}
		pub mod bounded_collections {
			use super::runtime_types;
			pub mod bounded_btree_map {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct BoundedBTreeMap<_0, _1>(pub ::subxt::utils::KeyedVec<_0, _1>);
			}
			pub mod bounded_vec {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
			}
			pub mod weak_bounded_vec {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
			}
		}
		pub mod da_control {
			use super::runtime_types;
			pub mod extensions {
				use super::runtime_types;
				pub mod check_app_id {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub struct CheckAppId(pub runtime_types::avail_core::AppId);
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct AppKeyInfo<_0> {
					pub owner: _0,
					pub id: runtime_types::avail_core::AppId,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::create_application_key`]."]
					create_application_key {
						key: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::submit_data`]."]
					submit_data {
						data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::submit_block_length_proposal`]."]
					submit_block_length_proposal {
						rows: ::core::primitive::u32,
						cols: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Error for the System pallet"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The application key already exists."]
					AppKeyAlreadyExists,
					#[codec(index = 1)]
					#[doc = "The application key is an empty string."]
					AppKeyCannotBeEmpty,
					#[codec(index = 2)]
					#[doc = "The last application ID overflowed."]
					LastAppIdOverflowed,
					#[codec(index = 3)]
					#[doc = "The submitted data is empty."]
					DataCannotBeEmpty,
					#[codec(index = 4)]
					#[doc = "The last block length proposal Id overflowed."]
					LastBlockLenProposalIdOverflowed,
					#[codec(index = 5)]
					#[doc = "The proposed block dimensions are out of bounds."]
					BlockDimensionsOutOfBounds,
					#[codec(index = 6)]
					#[doc = "The proposed block dimensions are too small."]
					BlockDimensionsTooSmall,
					#[codec(index = 7)]
					#[doc = "The request to reduce block dimensions was made in a non-empty block"]
					InvalidBlockWeightReduction,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Event for the pallet."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A new application key was created."]
					ApplicationKeyCreated {
						key: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
						owner: ::subxt::ext::sp_core::crypto::AccountId32,
						id: runtime_types::avail_core::AppId,
					},
					#[codec(index = 1)]
					DataSubmitted {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					},
					#[codec(index = 2)]
					BlockLengthProposalSubmitted {
						rows: runtime_types::avail_core::BlockLengthRows,
						cols: runtime_types::avail_core::BlockLengthColumns,
					},
				}
			}
		}
		pub mod da_runtime {
			use super::runtime_types;
			pub mod constants {
				use super::runtime_types;
				pub mod staking {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub struct NposSolution16 {
						pub votes1: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes2: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							),
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes3: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 2usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes4: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 3usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes5: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 4usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes6: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 5usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes7: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 6usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes8: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 7usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes9: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 8usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes10: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 9usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes11: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 10usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes12: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 11usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes13: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 12usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes14: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 13usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes15: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 14usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes16: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 15usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
					}
				}
			}
			pub mod primitives {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct SessionKeys {
					pub babe: runtime_types::sp_consensus_babe::app::Public,
					pub grandpa: runtime_types::sp_consensus_grandpa::app::Public,
					pub im_online: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
					pub authority_discovery: runtime_types::sp_authority_discovery::app::Public,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum OriginCaller {
				#[codec(index = 0)]
				system(
					runtime_types::frame_support::dispatch::RawOrigin<
						::subxt::ext::sp_core::crypto::AccountId32,
					>,
				),
				#[codec(index = 14)]
				TechnicalCommittee(
					runtime_types::pallet_collective::RawOrigin<
						::subxt::ext::sp_core::crypto::AccountId32,
					>,
				),
				#[codec(index = 2)]
				Void(runtime_types::sp_core::Void),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Runtime;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum RuntimeCall {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Call),
				#[codec(index = 1)]
				Utility(runtime_types::pallet_utility::pallet::Call),
				#[codec(index = 2)]
				Babe(runtime_types::pallet_babe::pallet::Call),
				#[codec(index = 3)]
				Timestamp(runtime_types::pallet_timestamp::pallet::Call),
				#[codec(index = 5)]
				Indices(runtime_types::pallet_indices::pallet::Call),
				#[codec(index = 6)]
				Balances(runtime_types::pallet_balances::pallet::Call),
				#[codec(index = 9)]
				ElectionProviderMultiPhase(
					runtime_types::pallet_election_provider_multi_phase::pallet::Call,
				),
				#[codec(index = 10)]
				Staking(runtime_types::pallet_staking::pallet::pallet::Call),
				#[codec(index = 11)]
				Session(runtime_types::pallet_session::pallet::Call),
				#[codec(index = 14)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Call),
				#[codec(index = 16)]
				TechnicalMembership(runtime_types::pallet_membership::pallet::Call),
				#[codec(index = 17)]
				Grandpa(runtime_types::pallet_grandpa::pallet::Call),
				#[codec(index = 18)]
				Treasury(runtime_types::pallet_treasury::pallet::Call),
				#[codec(index = 19)]
				Sudo(runtime_types::pallet_sudo::pallet::Call),
				#[codec(index = 20)]
				ImOnline(runtime_types::pallet_im_online::pallet::Call),
				#[codec(index = 24)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Call),
				#[codec(index = 25)]
				Bounties(runtime_types::pallet_bounties::pallet::Call),
				#[codec(index = 26)]
				Tips(runtime_types::pallet_tips::pallet::Call),
				#[codec(index = 29)]
				DataAvailability(runtime_types::da_control::pallet::Call),
				#[codec(index = 30)]
				NomadUpdaterManager(runtime_types::nomad_updater_manager::pallet::Call),
				#[codec(index = 31)]
				NomadHome(runtime_types::nomad_home::pallet::Call),
				#[codec(index = 32)]
				NomadDABridge(runtime_types::nomad_da_bridge::pallet::Call),
				#[codec(index = 33)]
				Preimage(runtime_types::pallet_preimage::pallet::Call),
				#[codec(index = 34)]
				Multisig(runtime_types::pallet_multisig::pallet::Call),
				#[codec(index = 35)]
				VoterList(runtime_types::pallet_bags_list::pallet::Call),
				#[codec(index = 36)]
				NominationPools(runtime_types::pallet_nomination_pools::pallet::Call),
				#[codec(index = 37)]
				Identity(runtime_types::pallet_identity::pallet::Call),
				#[codec(index = 38)]
				Mandate(runtime_types::pallet_mandate::pallet::Call),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum RuntimeEvent {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Event),
				#[codec(index = 1)]
				Utility(runtime_types::pallet_utility::pallet::Event),
				#[codec(index = 5)]
				Indices(runtime_types::pallet_indices::pallet::Event),
				#[codec(index = 6)]
				Balances(runtime_types::pallet_balances::pallet::Event),
				#[codec(index = 7)]
				TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
				#[codec(index = 9)]
				ElectionProviderMultiPhase(
					runtime_types::pallet_election_provider_multi_phase::pallet::Event,
				),
				#[codec(index = 10)]
				Staking(runtime_types::pallet_staking::pallet::pallet::Event),
				#[codec(index = 11)]
				Session(runtime_types::pallet_session::pallet::Event),
				#[codec(index = 14)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Event),
				#[codec(index = 16)]
				TechnicalMembership(runtime_types::pallet_membership::pallet::Event),
				#[codec(index = 17)]
				Grandpa(runtime_types::pallet_grandpa::pallet::Event),
				#[codec(index = 18)]
				Treasury(runtime_types::pallet_treasury::pallet::Event),
				#[codec(index = 19)]
				Sudo(runtime_types::pallet_sudo::pallet::Event),
				#[codec(index = 20)]
				ImOnline(runtime_types::pallet_im_online::pallet::Event),
				#[codec(index = 22)]
				Offences(runtime_types::pallet_offences::pallet::Event),
				#[codec(index = 24)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Event),
				#[codec(index = 25)]
				Bounties(runtime_types::pallet_bounties::pallet::Event),
				#[codec(index = 26)]
				Tips(runtime_types::pallet_tips::pallet::Event),
				#[codec(index = 29)]
				DataAvailability(runtime_types::da_control::pallet::Event),
				#[codec(index = 30)]
				NomadUpdaterManager(runtime_types::nomad_updater_manager::pallet::Event),
				#[codec(index = 31)]
				NomadHome(runtime_types::nomad_home::pallet::Event),
				#[codec(index = 32)]
				NomadDABridge(runtime_types::nomad_da_bridge::pallet::Event),
				#[codec(index = 33)]
				Preimage(runtime_types::pallet_preimage::pallet::Event),
				#[codec(index = 34)]
				Multisig(runtime_types::pallet_multisig::pallet::Event),
				#[codec(index = 35)]
				VoterList(runtime_types::pallet_bags_list::pallet::Event),
				#[codec(index = 36)]
				NominationPools(runtime_types::pallet_nomination_pools::pallet::Event),
				#[codec(index = 37)]
				Identity(runtime_types::pallet_identity::pallet::Event),
				#[codec(index = 38)]
				Mandate(runtime_types::pallet_mandate::pallet::Event),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum RuntimeHoldReason {}
		}
		pub mod finality_grandpa {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Equivocation<_0, _1, _2> {
				pub round_number: ::core::primitive::u64,
				pub identity: _0,
				pub first: (_1, _2),
				pub second: (_1, _2),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Precommit<_0, _1> {
				pub target_hash: _0,
				pub target_number: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Prevote<_0, _1> {
				pub target_hash: _0,
				pub target_number: _1,
			}
		}
		pub mod frame_support {
			use super::runtime_types;
			pub mod dispatch {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum DispatchClass {
					#[codec(index = 0)]
					Normal,
					#[codec(index = 1)]
					Operational,
					#[codec(index = 2)]
					Mandatory,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct DispatchInfo {
					pub weight: runtime_types::sp_weights::weight_v2::Weight,
					pub class: runtime_types::frame_support::dispatch::DispatchClass,
					pub pays_fee: runtime_types::frame_support::dispatch::Pays,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum Pays {
					#[codec(index = 0)]
					Yes,
					#[codec(index = 1)]
					No,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct PerDispatchClass<_0> {
					pub normal: _0,
					pub operational: _0,
					pub mandatory: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum RawOrigin<_0> {
					#[codec(index = 0)]
					Root,
					#[codec(index = 1)]
					Signed(_0),
					#[codec(index = 2)]
					None,
				}
			}
			pub mod traits {
				use super::runtime_types;
				pub mod preimages {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub enum Bounded<_0> {
						#[codec(index = 0)]
						Legacy {
							hash: ::subxt::ext::sp_core::H256,
						},
						#[codec(index = 1)]
						Inline(
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 2)]
						Lookup {
							hash: ::subxt::ext::sp_core::H256,
							len: ::core::primitive::u32,
						},
						__Ignore(::core::marker::PhantomData<_0>),
					}
				}
				pub mod tokens {
					use super::runtime_types;
					pub mod misc {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							Clone,
							Debug,
							Eq,
							PartialEq,
						)]
						pub enum BalanceStatus {
							#[codec(index = 0)]
							Free,
							#[codec(index = 1)]
							Reserved,
						}
					}
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct PalletId(pub [::core::primitive::u8; 8usize]);
		}
		pub mod frame_system {
			use super::runtime_types;
			pub mod extensions {
				use super::runtime_types;
				pub mod check_genesis {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub struct CheckGenesis;
				}
				pub mod check_mortality {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
				}
				pub mod check_non_zero_sender {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub struct CheckNonZeroSender;
				}
				pub mod check_nonce {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
				}
				pub mod check_spec_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub struct CheckSpecVersion;
				}
				pub mod check_tx_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub struct CheckTxVersion;
				}
				pub mod check_weight {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub struct CheckWeight;
				}
			}
			pub mod limits {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct BlockLength {
					pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
						::core::primitive::u32,
					>,
					pub cols: runtime_types::avail_core::BlockLengthColumns,
					pub rows: runtime_types::avail_core::BlockLengthRows,
					#[codec(compact)]
					pub chunk_size: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct BlockWeights {
					pub base_block: runtime_types::sp_weights::weight_v2::Weight,
					pub max_block: runtime_types::sp_weights::weight_v2::Weight,
					pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::frame_system::limits::WeightsPerClass,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct WeightsPerClass {
					pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
					pub max_extrinsic:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub max_total:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub reserved:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::remark`]."]
					remark {
						remark: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::set_heap_pages`]."]
					set_heap_pages { pages: ::core::primitive::u64 },
					#[codec(index = 2)]
					#[doc = "See [`Pallet::set_code`]."]
					set_code {
						code: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::set_code_without_checks`]."]
					set_code_without_checks {
						code: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::set_storage`]."]
					set_storage {
						items: ::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::kill_storage`]."]
					kill_storage {
						keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 6)]
					#[doc = "See [`Pallet::kill_prefix`]."]
					kill_prefix {
						prefix: ::std::vec::Vec<::core::primitive::u8>,
						subkeys: ::core::primitive::u32,
					},
					#[codec(index = 7)]
					#[doc = "See [`Pallet::remark_with_event`]."]
					remark_with_event {
						remark: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Error for the System pallet"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The name of specification does not match between the current runtime"]
					#[doc = "and the new runtime."]
					InvalidSpecName,
					#[codec(index = 1)]
					#[doc = "The specification version is not allowed to decrease between the current runtime"]
					#[doc = "and the new runtime."]
					SpecVersionNeedsToIncrease,
					#[codec(index = 2)]
					#[doc = "Failed to extract the runtime version from the new runtime."]
					#[doc = ""]
					#[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
					FailedToExtractRuntimeVersion,
					#[codec(index = 3)]
					#[doc = "Suicide called when the account has non-default composite data."]
					NonDefaultComposite,
					#[codec(index = 4)]
					#[doc = "There is a non-zero reference count preventing the account from being purged."]
					NonZeroRefCount,
					#[codec(index = 5)]
					#[doc = "The origin filter prevent the call to be dispatched."]
					CallFiltered,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Event for the System pallet."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An extrinsic completed successfully."]
					ExtrinsicSuccess {
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 1)]
					#[doc = "An extrinsic failed."]
					ExtrinsicFailed {
						dispatch_error: runtime_types::sp_runtime::DispatchError,
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 2)]
					#[doc = "`:code` was updated."]
					CodeUpdated,
					#[codec(index = 3)]
					#[doc = "A new account was created."]
					NewAccount {
						account: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 4)]
					#[doc = "An account was reaped."]
					KilledAccount {
						account: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 5)]
					#[doc = "On on-chain remark happened."]
					Remarked {
						sender: ::subxt::ext::sp_core::crypto::AccountId32,
						hash: ::subxt::ext::sp_core::H256,
					},
					#[codec(index = 6)]
					#[doc = "On on-chain remark happend called by Root."]
					RemarkedByRoot { hash: ::subxt::ext::sp_core::H256 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct AccountInfo<_0, _1> {
				pub nonce: _0,
				pub consumers: _0,
				pub providers: _0,
				pub sufficients: _0,
				pub data: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct EventRecord<_0, _1> {
				pub phase: runtime_types::frame_system::Phase,
				pub event: _0,
				pub topics: ::std::vec::Vec<_1>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ExtrinsicLen {
				pub raw: ::core::primitive::u32,
				pub padded: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct LastRuntimeUpgradeInfo {
				#[codec(compact)]
				pub spec_version: ::core::primitive::u32,
				pub spec_name: ::std::string::String,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum Phase {
				#[codec(index = 0)]
				ApplyExtrinsic(::core::primitive::u32),
				#[codec(index = 1)]
				Finalization,
				#[codec(index = 2)]
				Initialization,
			}
		}
		pub mod nomad_base {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct NomadBase {
				pub state: runtime_types::nomad_core::state::NomadState,
				pub local_domain: ::core::primitive::u32,
				pub committed_root: ::subxt::ext::sp_core::H256,
				pub updater: ::subxt::ext::sp_core::H160,
			}
		}
		pub mod nomad_core {
			use super::runtime_types;
			pub mod state {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum NomadState {
					#[codec(index = 0)]
					Active,
					#[codec(index = 1)]
					Failed,
				}
			}
			pub mod update {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct SignedUpdate {
					pub update: runtime_types::nomad_core::update::Update,
					pub signature: runtime_types::nomad_signature::signature::Signature,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Update {
					pub home_domain: ::core::primitive::u32,
					pub previous_root: ::subxt::ext::sp_core::H256,
					pub new_root: ::subxt::ext::sp_core::H256,
				}
			}
		}
		pub mod nomad_da_bridge {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::try_dispatch_data_root`]."]
					try_dispatch_data_root {
						#[codec(compact)]
						destination_domain: ::core::primitive::u32,
						recipient_address: ::subxt::ext::sp_core::H256,
						header: ::std::boxed::Box<
							runtime_types::avail_core::header::Header<
								::core::primitive::u32,
								runtime_types::sp_runtime::traits::BlakeTwo256,
							>,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					InitializationError,
					#[codec(index = 1)]
					HashOfBlockNotMatchBlockNumber,
					#[codec(index = 2)]
					DABridgeMessageExceedsMaxMessageSize,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					DataRootDispatched {
						destination_domain: ::core::primitive::u32,
						recipient_address: ::subxt::ext::sp_core::H256,
						block_number: ::core::primitive::u32,
						data_root: ::subxt::ext::sp_core::H256,
					},
				}
			}
		}
		pub mod nomad_home {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::dispatch`]."]
					dispatch {
						#[codec(compact)]
						destination_domain: ::core::primitive::u32,
						recipient_address: ::subxt::ext::sp_core::H256,
						message_body: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::update`]."]
					update {
						signed_update: runtime_types::nomad_core::update::SignedUpdate,
						#[codec(compact)]
						max_index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::improper_update`]."]
					improper_update {
						signed_update: runtime_types::nomad_core::update::SignedUpdate,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::set_updater`]."]
					set_updater {
						new_updater: ::subxt::ext::sp_core::H160,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					InitializationError,
					#[codec(index = 1)]
					IngestionError,
					#[codec(index = 2)]
					SignatureRecoveryError,
					#[codec(index = 3)]
					MessageTooLarge,
					#[codec(index = 4)]
					InvalidUpdaterSignature,
					#[codec(index = 5)]
					CommittedRootNotMatchUpdatePrevious,
					#[codec(index = 6)]
					RootForIndexNotFound,
					#[codec(index = 7)]
					IndexForRootNotFound,
					#[codec(index = 8)]
					FailedState,
					#[codec(index = 9)]
					MaxIndexWitnessExhausted,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					Dispatch {
						message_hash: ::subxt::ext::sp_core::H256,
						leaf_index: ::core::primitive::u32,
						destination_and_nonce: ::core::primitive::u64,
						committed_root: ::subxt::ext::sp_core::H256,
						message: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					Update {
						home_domain: ::core::primitive::u32,
						previous_root: ::subxt::ext::sp_core::H256,
						new_root: ::subxt::ext::sp_core::H256,
						signature: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 2)]
					ImproperUpdate {
						previous_root: ::subxt::ext::sp_core::H256,
						new_root: ::subxt::ext::sp_core::H256,
						signature: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 3)]
					UpdaterSlashed {
						updater: ::subxt::ext::sp_core::H160,
						reporter: ::subxt::ext::sp_core::crypto::AccountId32,
					},
				}
			}
		}
		pub mod nomad_merkle {
			use super::runtime_types;
			pub mod light {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct LightMerkle {
					pub branch: [::subxt::ext::sp_core::H256; 32usize],
					pub count: ::core::primitive::u32,
				}
			}
		}
		pub mod nomad_signature {
			use super::runtime_types;
			pub mod signature {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Signature {
					pub r: runtime_types::primitive_types::U256,
					pub s: runtime_types::primitive_types::U256,
					pub v: ::core::primitive::u64,
				}
			}
		}
		pub mod nomad_updater_manager {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					InitializationError,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					NewUpdater {
						old_updater: ::subxt::ext::sp_core::H160,
						new_updater: ::subxt::ext::sp_core::H160,
					},
					#[codec(index = 1)]
					FakeSlashed {
						reporter: ::subxt::ext::sp_core::crypto::AccountId32,
					},
				}
			}
		}
		pub mod pallet_babe {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::report_equivocation`]."]
					report_equivocation {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_consensus_slots::EquivocationProof<
								runtime_types::avail_core::header::Header<
									::core::primitive::u32,
									runtime_types::sp_runtime::traits::BlakeTwo256,
								>,
								runtime_types::sp_consensus_babe::app::Public,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::report_equivocation_unsigned`]."]
					report_equivocation_unsigned {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_consensus_slots::EquivocationProof<
								runtime_types::avail_core::header::Header<
									::core::primitive::u32,
									runtime_types::sp_runtime::traits::BlakeTwo256,
								>,
								runtime_types::sp_consensus_babe::app::Public,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::plan_config_change`]."]
					plan_config_change {
						config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
					InvalidEquivocationProof,
					#[codec(index = 1)]
					#[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
					InvalidKeyOwnershipProof,
					#[codec(index = 2)]
					#[doc = "A given equivocation report is valid but already previously reported."]
					DuplicateOffenceReport,
					#[codec(index = 3)]
					#[doc = "Submitted configuration is invalid."]
					InvalidConfiguration,
				}
			}
		}
		pub mod pallet_bags_list {
			use super::runtime_types;
			pub mod list {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Bag {
					pub head: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
					pub tail: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum ListError {
					#[codec(index = 0)]
					Duplicate,
					#[codec(index = 1)]
					NotHeavier,
					#[codec(index = 2)]
					NotInSameBag,
					#[codec(index = 3)]
					NodeNotFound,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Node {
					pub id: ::subxt::ext::sp_core::crypto::AccountId32,
					pub prev: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
					pub next: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
					pub bag_upper: ::core::primitive::u64,
					pub score: ::core::primitive::u64,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::rebag`]."]
					rebag {
						dislocated: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::put_in_front_of`]."]
					put_in_front_of {
						lighter: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "A error in the list interface implementation."]
					List(runtime_types::pallet_bags_list::list::ListError),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Moved an account from one bag to another."]
					Rebagged {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						from: ::core::primitive::u64,
						to: ::core::primitive::u64,
					},
					#[codec(index = 1)]
					#[doc = "Updated the score of some account to the given amount."]
					ScoreUpdated {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						new_score: ::core::primitive::u64,
					},
				}
			}
		}
		pub mod pallet_balances {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::transfer_allow_death`]."]
					transfer_allow_death {
						dest: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::set_balance_deprecated`]."]
					set_balance_deprecated {
						who: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
						#[codec(compact)]
						old_reserved: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::force_transfer`]."]
					force_transfer {
						source: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						dest: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::transfer_keep_alive`]."]
					transfer_keep_alive {
						dest: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::transfer_all`]."]
					transfer_all {
						dest: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::force_unreserve`]."]
					force_unreserve {
						who: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "See [`Pallet::upgrade_accounts`]."]
					upgrade_accounts {
						who: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					},
					#[codec(index = 7)]
					#[doc = "See [`Pallet::transfer`]."]
					transfer {
						dest: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					#[doc = "See [`Pallet::force_set_balance`]."]
					force_set_balance {
						who: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Vesting balance too high to send value."]
					VestingBalance,
					#[codec(index = 1)]
					#[doc = "Account liquidity restrictions prevent withdrawal."]
					LiquidityRestrictions,
					#[codec(index = 2)]
					#[doc = "Balance too low to send value."]
					InsufficientBalance,
					#[codec(index = 3)]
					#[doc = "Value too low to create account due to existential deposit."]
					ExistentialDeposit,
					#[codec(index = 4)]
					#[doc = "Transfer/payment would kill account."]
					Expendability,
					#[codec(index = 5)]
					#[doc = "A vesting schedule already exists for this account."]
					ExistingVestingSchedule,
					#[codec(index = 6)]
					#[doc = "Beneficiary account must pre-exist."]
					DeadAccount,
					#[codec(index = 7)]
					#[doc = "Number of named reserves exceed `MaxReserves`."]
					TooManyReserves,
					#[codec(index = 8)]
					#[doc = "Number of holds exceed `MaxHolds`."]
					TooManyHolds,
					#[codec(index = 9)]
					#[doc = "Number of freezes exceed `MaxFreezes`."]
					TooManyFreezes,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An account was created with some free balance."]
					Endowed {
						account: ::subxt::ext::sp_core::crypto::AccountId32,
						free_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
					#[doc = "resulting in an outright loss."]
					DustLost {
						account: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Transfer succeeded."]
					Transfer {
						from: ::subxt::ext::sp_core::crypto::AccountId32,
						to: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "A balance was set by root."]
					BalanceSet {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						free: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Some balance was reserved (moved from free to reserved)."]
					Reserved {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					#[doc = "Some balance was unreserved (moved from reserved to free)."]
					Unreserved {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "Some balance was moved from the reserve of the first account to the second account."]
					#[doc = "Final argument indicates the destination balance type."]
					ReserveRepatriated {
						from: ::subxt::ext::sp_core::crypto::AccountId32,
						to: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
						destination_status:
							runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 7)]
					#[doc = "Some amount was deposited (e.g. for transaction fees)."]
					Deposit {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
					Withdraw {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
					Slashed {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 10)]
					#[doc = "Some amount was minted into an account."]
					Minted {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 11)]
					#[doc = "Some amount was burned from an account."]
					Burned {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 12)]
					#[doc = "Some amount was suspended from an account (it can be restored later)."]
					Suspended {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 13)]
					#[doc = "Some amount was restored into an account."]
					Restored {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 14)]
					#[doc = "An account was upgraded."]
					Upgraded {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 15)]
					#[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
					Issued { amount: ::core::primitive::u128 },
					#[codec(index = 16)]
					#[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
					Rescinded { amount: ::core::primitive::u128 },
					#[codec(index = 17)]
					#[doc = "Some balance was locked."]
					Locked {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 18)]
					#[doc = "Some balance was unlocked."]
					Unlocked {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 19)]
					#[doc = "Some balance was frozen."]
					Frozen {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 20)]
					#[doc = "Some balance was thawed."]
					Thawed {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct AccountData<_0> {
					pub free: _0,
					pub reserved: _0,
					pub frozen: _0,
					pub flags: runtime_types::pallet_balances::types::ExtraFlags,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct BalanceLock<_0> {
					pub id: [::core::primitive::u8; 8usize],
					pub amount: _0,
					pub reasons: runtime_types::pallet_balances::types::Reasons,
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct ExtraFlags(pub ::core::primitive::u128);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct IdAmount<_0, _1> {
					pub id: _0,
					pub amount: _1,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum Reasons {
					#[codec(index = 0)]
					Fee,
					#[codec(index = 1)]
					Misc,
					#[codec(index = 2)]
					All,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct ReserveData<_0, _1> {
					pub id: _0,
					pub amount: _1,
				}
			}
		}
		pub mod pallet_bounties {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::propose_bounty`]."]
					propose_bounty {
						#[codec(compact)]
						value: ::core::primitive::u128,
						description: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::approve_bounty`]."]
					approve_bounty {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::propose_curator`]."]
					propose_curator {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
						curator: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						fee: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::unassign_curator`]."]
					unassign_curator {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::accept_curator`]."]
					accept_curator {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::award_bounty`]."]
					award_bounty {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
						beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 6)]
					#[doc = "See [`Pallet::claim_bounty`]."]
					claim_bounty {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 7)]
					#[doc = "See [`Pallet::close_bounty`]."]
					close_bounty {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					#[doc = "See [`Pallet::extend_bounty_expiry`]."]
					extend_bounty_expiry {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
						remark: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Proposer's balance is too low."]
					InsufficientProposersBalance,
					#[codec(index = 1)]
					#[doc = "No proposal or bounty at that index."]
					InvalidIndex,
					#[codec(index = 2)]
					#[doc = "The reason given is just too big."]
					ReasonTooBig,
					#[codec(index = 3)]
					#[doc = "The bounty status is unexpected."]
					UnexpectedStatus,
					#[codec(index = 4)]
					#[doc = "Require bounty curator."]
					RequireCurator,
					#[codec(index = 5)]
					#[doc = "Invalid bounty value."]
					InvalidValue,
					#[codec(index = 6)]
					#[doc = "Invalid bounty fee."]
					InvalidFee,
					#[codec(index = 7)]
					#[doc = "A bounty payout is pending."]
					#[doc = "To cancel the bounty, you must unassign and slash the curator."]
					PendingPayout,
					#[codec(index = 8)]
					#[doc = "The bounties cannot be claimed/closed because it's still in the countdown period."]
					Premature,
					#[codec(index = 9)]
					#[doc = "The bounty cannot be closed because it has active child bounties."]
					HasActiveChildBounty,
					#[codec(index = 10)]
					#[doc = "Too many approvals are already queued."]
					TooManyQueued,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New bounty proposal."]
					BountyProposed { index: ::core::primitive::u32 },
					#[codec(index = 1)]
					#[doc = "A bounty proposal was rejected; funds were slashed."]
					BountyRejected {
						index: ::core::primitive::u32,
						bond: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "A bounty proposal is funded and became active."]
					BountyBecameActive { index: ::core::primitive::u32 },
					#[codec(index = 3)]
					#[doc = "A bounty is awarded to a beneficiary."]
					BountyAwarded {
						index: ::core::primitive::u32,
						beneficiary: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 4)]
					#[doc = "A bounty is claimed by beneficiary."]
					BountyClaimed {
						index: ::core::primitive::u32,
						payout: ::core::primitive::u128,
						beneficiary: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 5)]
					#[doc = "A bounty is cancelled."]
					BountyCanceled { index: ::core::primitive::u32 },
					#[codec(index = 6)]
					#[doc = "A bounty expiry is extended."]
					BountyExtended { index: ::core::primitive::u32 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Bounty<_0, _1, _2> {
				pub proposer: _0,
				pub value: _1,
				pub fee: _1,
				pub curator_deposit: _1,
				pub bond: _1,
				pub status: runtime_types::pallet_bounties::BountyStatus<_0, _2>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum BountyStatus<_0, _1> {
				#[codec(index = 0)]
				Proposed,
				#[codec(index = 1)]
				Approved,
				#[codec(index = 2)]
				Funded,
				#[codec(index = 3)]
				CuratorProposed { curator: _0 },
				#[codec(index = 4)]
				Active { curator: _0, update_due: _1 },
				#[codec(index = 5)]
				PendingPayout {
					curator: _0,
					beneficiary: _0,
					unlock_at: _1,
				},
			}
		}
		pub mod pallet_collective {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::set_members`]."]
					set_members {
						new_members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						prime: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
						old_count: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::execute`]."]
					execute {
						proposal: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::propose`]."]
					propose {
						#[codec(compact)]
						threshold: ::core::primitive::u32,
						proposal: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::vote`]."]
					vote {
						proposal: ::subxt::ext::sp_core::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						approve: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::disapprove_proposal`]."]
					disapprove_proposal {
						proposal_hash: ::subxt::ext::sp_core::H256,
					},
					#[codec(index = 6)]
					#[doc = "See [`Pallet::close`]."]
					close {
						proposal_hash: ::subxt::ext::sp_core::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Account is not a member"]
					NotMember,
					#[codec(index = 1)]
					#[doc = "Duplicate proposals not allowed"]
					DuplicateProposal,
					#[codec(index = 2)]
					#[doc = "Proposal must exist"]
					ProposalMissing,
					#[codec(index = 3)]
					#[doc = "Mismatched index"]
					WrongIndex,
					#[codec(index = 4)]
					#[doc = "Duplicate vote ignored"]
					DuplicateVote,
					#[codec(index = 5)]
					#[doc = "Members are already initialized!"]
					AlreadyInitialized,
					#[codec(index = 6)]
					#[doc = "The close call was made too early, before the end of the voting."]
					TooEarly,
					#[codec(index = 7)]
					#[doc = "There can only be a maximum of `MaxProposals` active proposals."]
					TooManyProposals,
					#[codec(index = 8)]
					#[doc = "The given weight bound for the proposal was too low."]
					WrongProposalWeight,
					#[codec(index = 9)]
					#[doc = "The given length bound for the proposal was too low."]
					WrongProposalLength,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
					#[doc = "`MemberCount`)."]
					Proposed {
						account: ::subxt::ext::sp_core::crypto::AccountId32,
						proposal_index: ::core::primitive::u32,
						proposal_hash: ::subxt::ext::sp_core::H256,
						threshold: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "A motion (given hash) has been voted on by given account, leaving"]
					#[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
					Voted {
						account: ::subxt::ext::sp_core::crypto::AccountId32,
						proposal_hash: ::subxt::ext::sp_core::H256,
						voted: ::core::primitive::bool,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "A motion was approved by the required threshold."]
					Approved {
						proposal_hash: ::subxt::ext::sp_core::H256,
					},
					#[codec(index = 3)]
					#[doc = "A motion was not approved by the required threshold."]
					Disapproved {
						proposal_hash: ::subxt::ext::sp_core::H256,
					},
					#[codec(index = 4)]
					#[doc = "A motion was executed; result will be `Ok` if it returned without error."]
					Executed {
						proposal_hash: ::subxt::ext::sp_core::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 5)]
					#[doc = "A single member did some action; result will be `Ok` if it returned without error."]
					MemberExecuted {
						proposal_hash: ::subxt::ext::sp_core::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 6)]
					#[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
					Closed {
						proposal_hash: ::subxt::ext::sp_core::H256,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum RawOrigin<_0> {
				#[codec(index = 0)]
				Members(::core::primitive::u32, ::core::primitive::u32),
				#[codec(index = 1)]
				Member(_0),
				#[codec(index = 2)]
				_Phantom,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Votes<_0, _1> {
				pub index: _1,
				pub threshold: _1,
				pub ayes: ::std::vec::Vec<_0>,
				pub nays: ::std::vec::Vec<_0>,
				pub end: _1,
			}
		}
		pub mod pallet_election_provider_multi_phase {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					# [codec (index = 0)] # [doc = "See [`Pallet::submit_unsigned`]."] submit_unsigned { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: da_runtime :: constants :: staking :: NposSolution16 > > , witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , } , # [codec (index = 1)] # [doc = "See [`Pallet::set_minimum_untrusted_score`]."] set_minimum_untrusted_score { maybe_next_score : :: core :: option :: Option < runtime_types :: sp_npos_elections :: ElectionScore > , } , # [codec (index = 2)] # [doc = "See [`Pallet::set_emergency_election_result`]."] set_emergency_election_result { supports : :: std :: vec :: Vec < (:: subxt :: ext :: sp_core :: crypto :: AccountId32 , runtime_types :: sp_npos_elections :: Support < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) > , } , # [codec (index = 3)] # [doc = "See [`Pallet::submit`]."] submit { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: da_runtime :: constants :: staking :: NposSolution16 > > , } , # [codec (index = 4)] # [doc = "See [`Pallet::governance_fallback`]."] governance_fallback { maybe_max_voters : :: core :: option :: Option < :: core :: primitive :: u32 > , maybe_max_targets : :: core :: option :: Option < :: core :: primitive :: u32 > , } , }
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Error of the pallet that can be returned in response to dispatches."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Submission was too early."]
					PreDispatchEarlySubmission,
					#[codec(index = 1)]
					#[doc = "Wrong number of winners presented."]
					PreDispatchWrongWinnerCount,
					#[codec(index = 2)]
					#[doc = "Submission was too weak, score-wise."]
					PreDispatchWeakSubmission,
					#[codec(index = 3)]
					#[doc = "The queue was full, and the solution was not better than any of the existing ones."]
					SignedQueueFull,
					#[codec(index = 4)]
					#[doc = "The origin failed to pay the deposit."]
					SignedCannotPayDeposit,
					#[codec(index = 5)]
					#[doc = "Witness data to dispatchable is invalid."]
					SignedInvalidWitness,
					#[codec(index = 6)]
					#[doc = "The signed submission consumes too much weight"]
					SignedTooMuchWeight,
					#[codec(index = 7)]
					#[doc = "OCW submitted solution for wrong round"]
					OcwCallWrongEra,
					#[codec(index = 8)]
					#[doc = "Snapshot metadata should exist but didn't."]
					MissingSnapshotMetadata,
					#[codec(index = 9)]
					#[doc = "`Self::insert_submission` returned an invalid index."]
					InvalidSubmissionIndex,
					#[codec(index = 10)]
					#[doc = "The call is not allowed at this point."]
					CallNotAllowed,
					#[codec(index = 11)]
					#[doc = "The fallback failed"]
					FallbackFailed,
					#[codec(index = 12)]
					#[doc = "Some bound not met"]
					BoundNotMet,
					#[codec(index = 13)]
					#[doc = "Submitted solution has too many winners"]
					TooManyWinners,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A solution was stored with the given compute."]
					#[doc = ""]
					#[doc = "The `origin` indicates the origin of the solution. If `origin` is `Some(AccountId)`,"]
					#[doc = "the stored solution was submited in the signed phase by a miner with the `AccountId`."]
					#[doc = "Otherwise, the solution was stored either during the unsigned phase or by"]
					#[doc = "`T::ForceOrigin`. The `bool` is `true` when a previous solution was ejected to make"]
					#[doc = "room for this one."]
					SolutionStored {
						compute:
							runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
						origin: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
						prev_ejected: ::core::primitive::bool,
					},
					#[codec(index = 1)]
					#[doc = "The election has been finalized, with the given computation and score."]
					ElectionFinalized {
						compute:
							runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
						score: runtime_types::sp_npos_elections::ElectionScore,
					},
					#[codec(index = 2)]
					#[doc = "An election failed."]
					#[doc = ""]
					#[doc = "Not much can be said about which computes failed in the process."]
					ElectionFailed,
					#[codec(index = 3)]
					#[doc = "An account has been rewarded for their signed submission being finalized."]
					Rewarded {
						account: ::subxt::ext::sp_core::crypto::AccountId32,
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "An account has been slashed for submitting an invalid signed submission."]
					Slashed {
						account: ::subxt::ext::sp_core::crypto::AccountId32,
						value: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					#[doc = "There was a phase transition in a given round."]
					PhaseTransitioned {
						from: runtime_types::pallet_election_provider_multi_phase::Phase<
							::core::primitive::u32,
						>,
						to: runtime_types::pallet_election_provider_multi_phase::Phase<
							::core::primitive::u32,
						>,
						round: ::core::primitive::u32,
					},
				}
			}
			pub mod signed {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct SignedSubmission<_0, _1, _2> {
					pub who: _0,
					pub deposit: _1,
					pub raw_solution:
						runtime_types::pallet_election_provider_multi_phase::RawSolution<_2>,
					pub call_fee: _1,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum ElectionCompute {
				#[codec(index = 0)]
				OnChain,
				#[codec(index = 1)]
				Signed,
				#[codec(index = 2)]
				Unsigned,
				#[codec(index = 3)]
				Fallback,
				#[codec(index = 4)]
				Emergency,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum Phase<_0> {
				#[codec(index = 0)]
				Off,
				#[codec(index = 1)]
				Signed,
				#[codec(index = 2)]
				Unsigned((::core::primitive::bool, _0)),
				#[codec(index = 3)]
				Emergency,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct RawSolution<_0> {
				pub solution: _0,
				pub score: runtime_types::sp_npos_elections::ElectionScore,
				pub round: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ReadySolution {
				pub supports: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
					::subxt::ext::sp_core::crypto::AccountId32,
					runtime_types::sp_npos_elections::Support<
						::subxt::ext::sp_core::crypto::AccountId32,
					>,
				)>,
				pub score: runtime_types::sp_npos_elections::ElectionScore,
				pub compute: runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct RoundSnapshot<_0, _1> {
				pub voters: ::std::vec::Vec<_1>,
				pub targets: ::std::vec::Vec<_0>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SolutionOrSnapshotSize {
				#[codec(compact)]
				pub voters: ::core::primitive::u32,
				#[codec(compact)]
				pub targets: ::core::primitive::u32,
			}
		}
		pub mod pallet_grandpa {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::report_equivocation`]."]
					report_equivocation {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_consensus_grandpa::EquivocationProof<
								::subxt::ext::sp_core::H256,
								::core::primitive::u32,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::report_equivocation_unsigned`]."]
					report_equivocation_unsigned {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_consensus_grandpa::EquivocationProof<
								::subxt::ext::sp_core::H256,
								::core::primitive::u32,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::note_stalled`]."]
					note_stalled {
						delay: ::core::primitive::u32,
						best_finalized_block_number: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Attempt to signal GRANDPA pause when the authority set isn't live"]
					#[doc = "(either paused or already pending pause)."]
					PauseFailed,
					#[codec(index = 1)]
					#[doc = "Attempt to signal GRANDPA resume when the authority set isn't paused"]
					#[doc = "(either live or already pending resume)."]
					ResumeFailed,
					#[codec(index = 2)]
					#[doc = "Attempt to signal GRANDPA change with one already pending."]
					ChangePending,
					#[codec(index = 3)]
					#[doc = "Cannot signal forced change so soon after last."]
					TooSoon,
					#[codec(index = 4)]
					#[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
					InvalidKeyOwnershipProof,
					#[codec(index = 5)]
					#[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
					InvalidEquivocationProof,
					#[codec(index = 6)]
					#[doc = "A given equivocation report is valid but already previously reported."]
					DuplicateOffenceReport,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New authority set has been applied."]
					NewAuthorities {
						authority_set: ::std::vec::Vec<(
							runtime_types::sp_consensus_grandpa::app::Public,
							::core::primitive::u64,
						)>,
					},
					#[codec(index = 1)]
					#[doc = "Current authority set has been paused."]
					Paused,
					#[codec(index = 2)]
					#[doc = "Current authority set has been resumed."]
					Resumed,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct StoredPendingChange<_0> {
				pub scheduled_at: _0,
				pub delay: _0,
				pub next_authorities:
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
						runtime_types::sp_consensus_grandpa::app::Public,
						::core::primitive::u64,
					)>,
				pub forced: ::core::option::Option<_0>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum StoredState<_0> {
				#[codec(index = 0)]
				Live,
				#[codec(index = 1)]
				PendingPause { scheduled_at: _0, delay: _0 },
				#[codec(index = 2)]
				Paused,
				#[codec(index = 3)]
				PendingResume { scheduled_at: _0, delay: _0 },
			}
		}
		pub mod pallet_identity {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Identity pallet declaration."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::add_registrar`]."]
					add_registrar {
						account: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::set_identity`]."]
					set_identity {
						info:
							::std::boxed::Box<runtime_types::pallet_identity::types::IdentityInfo>,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::set_subs`]."]
					set_subs {
						subs: ::std::vec::Vec<(
							::subxt::ext::sp_core::crypto::AccountId32,
							runtime_types::pallet_identity::types::Data,
						)>,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::clear_identity`]."]
					clear_identity,
					#[codec(index = 4)]
					#[doc = "See [`Pallet::request_judgement`]."]
					request_judgement {
						#[codec(compact)]
						reg_index: ::core::primitive::u32,
						#[codec(compact)]
						max_fee: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::cancel_request`]."]
					cancel_request { reg_index: ::core::primitive::u32 },
					#[codec(index = 6)]
					#[doc = "See [`Pallet::set_fee`]."]
					set_fee {
						#[codec(compact)]
						index: ::core::primitive::u32,
						#[codec(compact)]
						fee: ::core::primitive::u128,
					},
					#[codec(index = 7)]
					#[doc = "See [`Pallet::set_account_id`]."]
					set_account_id {
						#[codec(compact)]
						index: ::core::primitive::u32,
						new: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 8)]
					#[doc = "See [`Pallet::set_fields`]."]
					set_fields {
						#[codec(compact)]
						index: ::core::primitive::u32,
						fields: runtime_types::pallet_identity::types::BitFlags<
							runtime_types::pallet_identity::types::IdentityField,
						>,
					},
					#[codec(index = 9)]
					#[doc = "See [`Pallet::provide_judgement`]."]
					provide_judgement {
						#[codec(compact)]
						reg_index: ::core::primitive::u32,
						target: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						judgement: runtime_types::pallet_identity::types::Judgement<
							::core::primitive::u128,
						>,
						identity: ::subxt::ext::sp_core::H256,
					},
					#[codec(index = 10)]
					#[doc = "See [`Pallet::kill_identity`]."]
					kill_identity {
						target: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 11)]
					#[doc = "See [`Pallet::add_sub`]."]
					add_sub {
						sub: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						data: runtime_types::pallet_identity::types::Data,
					},
					#[codec(index = 12)]
					#[doc = "See [`Pallet::rename_sub`]."]
					rename_sub {
						sub: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						data: runtime_types::pallet_identity::types::Data,
					},
					#[codec(index = 13)]
					#[doc = "See [`Pallet::remove_sub`]."]
					remove_sub {
						sub: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 14)]
					#[doc = "See [`Pallet::quit_sub`]."]
					quit_sub,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Too many subs-accounts."]
					TooManySubAccounts,
					#[codec(index = 1)]
					#[doc = "Account isn't found."]
					NotFound,
					#[codec(index = 2)]
					#[doc = "Account isn't named."]
					NotNamed,
					#[codec(index = 3)]
					#[doc = "Empty index."]
					EmptyIndex,
					#[codec(index = 4)]
					#[doc = "Fee is changed."]
					FeeChanged,
					#[codec(index = 5)]
					#[doc = "No identity found."]
					NoIdentity,
					#[codec(index = 6)]
					#[doc = "Sticky judgement."]
					StickyJudgement,
					#[codec(index = 7)]
					#[doc = "Judgement given."]
					JudgementGiven,
					#[codec(index = 8)]
					#[doc = "Invalid judgement."]
					InvalidJudgement,
					#[codec(index = 9)]
					#[doc = "The index is invalid."]
					InvalidIndex,
					#[codec(index = 10)]
					#[doc = "The target is invalid."]
					InvalidTarget,
					#[codec(index = 11)]
					#[doc = "Too many additional fields."]
					TooManyFields,
					#[codec(index = 12)]
					#[doc = "Maximum amount of registrars reached. Cannot add any more."]
					TooManyRegistrars,
					#[codec(index = 13)]
					#[doc = "Account ID is already named."]
					AlreadyClaimed,
					#[codec(index = 14)]
					#[doc = "Sender is not a sub-account."]
					NotSub,
					#[codec(index = 15)]
					#[doc = "Sub-account isn't owned by sender."]
					NotOwned,
					#[codec(index = 16)]
					#[doc = "The provided judgement was for a different identity."]
					JudgementForDifferentIdentity,
					#[codec(index = 17)]
					#[doc = "Error that occurs when there is an issue paying for judgement."]
					JudgementPaymentFailed,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A name was set or reset (which will remove all judgements)."]
					IdentitySet {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 1)]
					#[doc = "A name was cleared, and the given balance returned."]
					IdentityCleared {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "A name was removed and the given balance slashed."]
					IdentityKilled {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "A judgement was asked from a registrar."]
					JudgementRequested {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					#[doc = "A judgement request was retracted."]
					JudgementUnrequested {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					#[doc = "A judgement was given by a registrar."]
					JudgementGiven {
						target: ::subxt::ext::sp_core::crypto::AccountId32,
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 6)]
					#[doc = "A registrar was added."]
					RegistrarAdded {
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 7)]
					#[doc = "A sub-identity was added to an identity and the deposit paid."]
					SubIdentityAdded {
						sub: ::subxt::ext::sp_core::crypto::AccountId32,
						main: ::subxt::ext::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					#[doc = "A sub-identity was removed from an identity and the deposit freed."]
					SubIdentityRemoved {
						sub: ::subxt::ext::sp_core::crypto::AccountId32,
						main: ::subxt::ext::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					#[doc = "A sub-identity was cleared, and the given deposit repatriated from the"]
					#[doc = "main identity account to the sub-identity account."]
					SubIdentityRevoked {
						sub: ::subxt::ext::sp_core::crypto::AccountId32,
						main: ::subxt::ext::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct BitFlags<_0>(
					pub ::core::primitive::u64,
					#[codec(skip)] pub ::core::marker::PhantomData<_0>,
				);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum Data {
					#[codec(index = 0)]
					None,
					#[codec(index = 1)]
					Raw0([::core::primitive::u8; 0usize]),
					#[codec(index = 2)]
					Raw1([::core::primitive::u8; 1usize]),
					#[codec(index = 3)]
					Raw2([::core::primitive::u8; 2usize]),
					#[codec(index = 4)]
					Raw3([::core::primitive::u8; 3usize]),
					#[codec(index = 5)]
					Raw4([::core::primitive::u8; 4usize]),
					#[codec(index = 6)]
					Raw5([::core::primitive::u8; 5usize]),
					#[codec(index = 7)]
					Raw6([::core::primitive::u8; 6usize]),
					#[codec(index = 8)]
					Raw7([::core::primitive::u8; 7usize]),
					#[codec(index = 9)]
					Raw8([::core::primitive::u8; 8usize]),
					#[codec(index = 10)]
					Raw9([::core::primitive::u8; 9usize]),
					#[codec(index = 11)]
					Raw10([::core::primitive::u8; 10usize]),
					#[codec(index = 12)]
					Raw11([::core::primitive::u8; 11usize]),
					#[codec(index = 13)]
					Raw12([::core::primitive::u8; 12usize]),
					#[codec(index = 14)]
					Raw13([::core::primitive::u8; 13usize]),
					#[codec(index = 15)]
					Raw14([::core::primitive::u8; 14usize]),
					#[codec(index = 16)]
					Raw15([::core::primitive::u8; 15usize]),
					#[codec(index = 17)]
					Raw16([::core::primitive::u8; 16usize]),
					#[codec(index = 18)]
					Raw17([::core::primitive::u8; 17usize]),
					#[codec(index = 19)]
					Raw18([::core::primitive::u8; 18usize]),
					#[codec(index = 20)]
					Raw19([::core::primitive::u8; 19usize]),
					#[codec(index = 21)]
					Raw20([::core::primitive::u8; 20usize]),
					#[codec(index = 22)]
					Raw21([::core::primitive::u8; 21usize]),
					#[codec(index = 23)]
					Raw22([::core::primitive::u8; 22usize]),
					#[codec(index = 24)]
					Raw23([::core::primitive::u8; 23usize]),
					#[codec(index = 25)]
					Raw24([::core::primitive::u8; 24usize]),
					#[codec(index = 26)]
					Raw25([::core::primitive::u8; 25usize]),
					#[codec(index = 27)]
					Raw26([::core::primitive::u8; 26usize]),
					#[codec(index = 28)]
					Raw27([::core::primitive::u8; 27usize]),
					#[codec(index = 29)]
					Raw28([::core::primitive::u8; 28usize]),
					#[codec(index = 30)]
					Raw29([::core::primitive::u8; 29usize]),
					#[codec(index = 31)]
					Raw30([::core::primitive::u8; 30usize]),
					#[codec(index = 32)]
					Raw31([::core::primitive::u8; 31usize]),
					#[codec(index = 33)]
					Raw32([::core::primitive::u8; 32usize]),
					#[codec(index = 34)]
					BlakeTwo256([::core::primitive::u8; 32usize]),
					#[codec(index = 35)]
					Sha256([::core::primitive::u8; 32usize]),
					#[codec(index = 36)]
					Keccak256([::core::primitive::u8; 32usize]),
					#[codec(index = 37)]
					ShaThree256([::core::primitive::u8; 32usize]),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum IdentityField {
					#[codec(index = 1)]
					Display,
					#[codec(index = 2)]
					Legal,
					#[codec(index = 4)]
					Web,
					#[codec(index = 8)]
					Riot,
					#[codec(index = 16)]
					Email,
					#[codec(index = 32)]
					PgpFingerprint,
					#[codec(index = 64)]
					Image,
					#[codec(index = 128)]
					Twitter,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct IdentityInfo {
					pub additional: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						runtime_types::pallet_identity::types::Data,
						runtime_types::pallet_identity::types::Data,
					)>,
					pub display: runtime_types::pallet_identity::types::Data,
					pub legal: runtime_types::pallet_identity::types::Data,
					pub web: runtime_types::pallet_identity::types::Data,
					pub riot: runtime_types::pallet_identity::types::Data,
					pub email: runtime_types::pallet_identity::types::Data,
					pub pgp_fingerprint: ::core::option::Option<[::core::primitive::u8; 20usize]>,
					pub image: runtime_types::pallet_identity::types::Data,
					pub twitter: runtime_types::pallet_identity::types::Data,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum Judgement<_0> {
					#[codec(index = 0)]
					Unknown,
					#[codec(index = 1)]
					FeePaid(_0),
					#[codec(index = 2)]
					Reasonable,
					#[codec(index = 3)]
					KnownGood,
					#[codec(index = 4)]
					OutOfDate,
					#[codec(index = 5)]
					LowQuality,
					#[codec(index = 6)]
					Erroneous,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct RegistrarInfo<_0, _1> {
					pub account: _1,
					pub fee: _0,
					pub fields: runtime_types::pallet_identity::types::BitFlags<
						runtime_types::pallet_identity::types::IdentityField,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Registration<_0> {
					pub judgements: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						::core::primitive::u32,
						runtime_types::pallet_identity::types::Judgement<_0>,
					)>,
					pub deposit: _0,
					pub info: runtime_types::pallet_identity::types::IdentityInfo,
				}
			}
		}
		pub mod pallet_im_online {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::heartbeat`]."]
					heartbeat {
						heartbeat:
							runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
						signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Non existent public key."]
					InvalidKey,
					#[codec(index = 1)]
					#[doc = "Duplicated heartbeat."]
					DuplicatedHeartbeat,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A new heartbeat was received from `AuthorityId`."]
					HeartbeatReceived {
						authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
					},
					#[codec(index = 1)]
					#[doc = "At the end of the session, no offence was committed."]
					AllGood,
					#[codec(index = 2)]
					#[doc = "At the end of the session, at least one validator was found to be offline."]
					SomeOffline {
						offline: ::std::vec::Vec<(
							::subxt::ext::sp_core::crypto::AccountId32,
							runtime_types::pallet_staking::Exposure<
								::subxt::ext::sp_core::crypto::AccountId32,
								::core::primitive::u128,
							>,
						)>,
					},
				}
			}
			pub mod sr25519 {
				use super::runtime_types;
				pub mod app_sr25519 {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Heartbeat<_0> {
				pub block_number: _0,
				pub session_index: _0,
				pub authority_index: _0,
				pub validators_len: _0,
			}
		}
		pub mod pallet_indices {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::claim`]."]
					claim { index: ::core::primitive::u32 },
					#[codec(index = 1)]
					#[doc = "See [`Pallet::transfer`]."]
					transfer {
						new: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::free`]."]
					free { index: ::core::primitive::u32 },
					#[codec(index = 3)]
					#[doc = "See [`Pallet::force_transfer`]."]
					force_transfer {
						new: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						index: ::core::primitive::u32,
						freeze: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::freeze`]."]
					freeze { index: ::core::primitive::u32 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The index was not already assigned."]
					NotAssigned,
					#[codec(index = 1)]
					#[doc = "The index is assigned to another account."]
					NotOwner,
					#[codec(index = 2)]
					#[doc = "The index was not available."]
					InUse,
					#[codec(index = 3)]
					#[doc = "The source and destination accounts are identical."]
					NotTransfer,
					#[codec(index = 4)]
					#[doc = "The index is permanent and may not be freed/changed."]
					Permanent,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A account index was assigned."]
					IndexAssigned {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "A account index has been freed up (unassigned)."]
					IndexFreed { index: ::core::primitive::u32 },
					#[codec(index = 2)]
					#[doc = "A account index has been frozen to its current account ID."]
					IndexFrozen {
						index: ::core::primitive::u32,
						who: ::subxt::ext::sp_core::crypto::AccountId32,
					},
				}
			}
		}
		pub mod pallet_mandate {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::mandate`]."]
					mandate {
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A root operation was executed, show result"]
					RootOp {
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod pallet_membership {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::add_member`]."]
					add_member {
						who: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::remove_member`]."]
					remove_member {
						who: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::swap_member`]."]
					swap_member {
						remove: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						add: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::reset_members`]."]
					reset_members {
						members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::change_key`]."]
					change_key {
						new: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::set_prime`]."]
					set_prime {
						who: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 6)]
					#[doc = "See [`Pallet::clear_prime`]."]
					clear_prime,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Already a member."]
					AlreadyMember,
					#[codec(index = 1)]
					#[doc = "Not a member."]
					NotMember,
					#[codec(index = 2)]
					#[doc = "Too many members."]
					TooManyMembers,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "The given member was added; see the transaction for who."]
					MemberAdded,
					#[codec(index = 1)]
					#[doc = "The given member was removed; see the transaction for who."]
					MemberRemoved,
					#[codec(index = 2)]
					#[doc = "Two members were swapped; see the transaction for who."]
					MembersSwapped,
					#[codec(index = 3)]
					#[doc = "The membership was reset; see the transaction for who the new set is."]
					MembersReset,
					#[codec(index = 4)]
					#[doc = "One of the members' keys changed."]
					KeyChanged,
					#[codec(index = 5)]
					#[doc = "Phantom member, never used."]
					Dummy,
				}
			}
		}
		pub mod pallet_multisig {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::as_multi_threshold_1`]."]
					as_multi_threshold_1 {
						other_signatories:
							::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::as_multi`]."]
					as_multi {
						threshold: ::core::primitive::u16,
						other_signatories:
							::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						maybe_timepoint: ::core::option::Option<
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						>,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::approve_as_multi`]."]
					approve_as_multi {
						threshold: ::core::primitive::u16,
						other_signatories:
							::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						maybe_timepoint: ::core::option::Option<
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						>,
						call_hash: [::core::primitive::u8; 32usize],
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::cancel_as_multi`]."]
					cancel_as_multi {
						threshold: ::core::primitive::u16,
						other_signatories:
							::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						call_hash: [::core::primitive::u8; 32usize],
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Threshold must be 2 or greater."]
					MinimumThreshold,
					#[codec(index = 1)]
					#[doc = "Call is already approved by this signatory."]
					AlreadyApproved,
					#[codec(index = 2)]
					#[doc = "Call doesn't need any (more) approvals."]
					NoApprovalsNeeded,
					#[codec(index = 3)]
					#[doc = "There are too few signatories in the list."]
					TooFewSignatories,
					#[codec(index = 4)]
					#[doc = "There are too many signatories in the list."]
					TooManySignatories,
					#[codec(index = 5)]
					#[doc = "The signatories were provided out of order; they should be ordered."]
					SignatoriesOutOfOrder,
					#[codec(index = 6)]
					#[doc = "The sender was contained in the other signatories; it shouldn't be."]
					SenderInSignatories,
					#[codec(index = 7)]
					#[doc = "Multisig operation not found when attempting to cancel."]
					NotFound,
					#[codec(index = 8)]
					#[doc = "Only the account that originally created the multisig is able to cancel it."]
					NotOwner,
					#[codec(index = 9)]
					#[doc = "No timepoint was given, yet the multisig operation is already underway."]
					NoTimepoint,
					#[codec(index = 10)]
					#[doc = "A different timepoint was given to the multisig operation that is underway."]
					WrongTimepoint,
					#[codec(index = 11)]
					#[doc = "A timepoint was given, yet no multisig operation is underway."]
					UnexpectedTimepoint,
					#[codec(index = 12)]
					#[doc = "The maximum weight information provided was too low."]
					MaxWeightTooLow,
					#[codec(index = 13)]
					#[doc = "The data to be stored is already stored."]
					AlreadyStored,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A new multisig operation has begun."]
					NewMultisig {
						approving: ::subxt::ext::sp_core::crypto::AccountId32,
						multisig: ::subxt::ext::sp_core::crypto::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 1)]
					#[doc = "A multisig operation has been approved by someone."]
					MultisigApproval {
						approving: ::subxt::ext::sp_core::crypto::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: ::subxt::ext::sp_core::crypto::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 2)]
					#[doc = "A multisig operation has been executed."]
					MultisigExecuted {
						approving: ::subxt::ext::sp_core::crypto::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: ::subxt::ext::sp_core::crypto::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 3)]
					#[doc = "A multisig operation has been cancelled."]
					MultisigCancelled {
						cancelling: ::subxt::ext::sp_core::crypto::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: ::subxt::ext::sp_core::crypto::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Multisig<_0, _1, _2> {
				pub when: runtime_types::pallet_multisig::Timepoint<_0>,
				pub deposit: _1,
				pub depositor: _2,
				pub approvals: runtime_types::bounded_collections::bounded_vec::BoundedVec<_2>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Timepoint<_0> {
				pub height: _0,
				pub index: _0,
			}
		}
		pub mod pallet_nomination_pools {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::join`]."]
					join {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						pool_id: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::bond_extra`]."]
					bond_extra {
						extra: runtime_types::pallet_nomination_pools::BondExtra<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::claim_payout`]."]
					claim_payout,
					#[codec(index = 3)]
					#[doc = "See [`Pallet::unbond`]."]
					unbond {
						member_account: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						unbonding_points: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::pool_withdraw_unbonded`]."]
					pool_withdraw_unbonded {
						pool_id: ::core::primitive::u32,
						num_slashing_spans: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::withdraw_unbonded`]."]
					withdraw_unbonded {
						member_account: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						num_slashing_spans: ::core::primitive::u32,
					},
					#[codec(index = 6)]
					#[doc = "See [`Pallet::create`]."]
					create {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						root: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						nominator: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						bouncer: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 7)]
					#[doc = "See [`Pallet::create_with_pool_id`]."]
					create_with_pool_id {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						root: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						nominator: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						bouncer: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						pool_id: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					#[doc = "See [`Pallet::nominate`]."]
					nominate {
						pool_id: ::core::primitive::u32,
						validators: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					},
					#[codec(index = 9)]
					#[doc = "See [`Pallet::set_state`]."]
					set_state {
						pool_id: ::core::primitive::u32,
						state: runtime_types::pallet_nomination_pools::PoolState,
					},
					#[codec(index = 10)]
					#[doc = "See [`Pallet::set_metadata`]."]
					set_metadata {
						pool_id: ::core::primitive::u32,
						metadata: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 11)]
					#[doc = "See [`Pallet::set_configs`]."]
					set_configs {
						min_join_bond: runtime_types::pallet_nomination_pools::ConfigOp<
							::core::primitive::u128,
						>,
						min_create_bond: runtime_types::pallet_nomination_pools::ConfigOp<
							::core::primitive::u128,
						>,
						max_pools: runtime_types::pallet_nomination_pools::ConfigOp<
							::core::primitive::u32,
						>,
						max_members: runtime_types::pallet_nomination_pools::ConfigOp<
							::core::primitive::u32,
						>,
						max_members_per_pool: runtime_types::pallet_nomination_pools::ConfigOp<
							::core::primitive::u32,
						>,
						global_max_commission: runtime_types::pallet_nomination_pools::ConfigOp<
							runtime_types::sp_arithmetic::per_things::Perbill,
						>,
					},
					#[codec(index = 12)]
					#[doc = "See [`Pallet::update_roles`]."]
					update_roles {
						pool_id: ::core::primitive::u32,
						new_root: runtime_types::pallet_nomination_pools::ConfigOp<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
						new_nominator: runtime_types::pallet_nomination_pools::ConfigOp<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
						new_bouncer: runtime_types::pallet_nomination_pools::ConfigOp<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					},
					#[codec(index = 13)]
					#[doc = "See [`Pallet::chill`]."]
					chill { pool_id: ::core::primitive::u32 },
					#[codec(index = 14)]
					#[doc = "See [`Pallet::bond_extra_other`]."]
					bond_extra_other {
						member: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						extra: runtime_types::pallet_nomination_pools::BondExtra<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 15)]
					#[doc = "See [`Pallet::set_claim_permission`]."]
					set_claim_permission {
						permission: runtime_types::pallet_nomination_pools::ClaimPermission,
					},
					#[codec(index = 16)]
					#[doc = "See [`Pallet::claim_payout_other`]."]
					claim_payout_other {
						other: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 17)]
					#[doc = "See [`Pallet::set_commission`]."]
					set_commission {
						pool_id: ::core::primitive::u32,
						new_commission: ::core::option::Option<(
							runtime_types::sp_arithmetic::per_things::Perbill,
							::subxt::ext::sp_core::crypto::AccountId32,
						)>,
					},
					#[codec(index = 18)]
					#[doc = "See [`Pallet::set_commission_max`]."]
					set_commission_max {
						pool_id: ::core::primitive::u32,
						max_commission: runtime_types::sp_arithmetic::per_things::Perbill,
					},
					#[codec(index = 19)]
					#[doc = "See [`Pallet::set_commission_change_rate`]."]
					set_commission_change_rate {
						pool_id: ::core::primitive::u32,
						change_rate: runtime_types::pallet_nomination_pools::CommissionChangeRate<
							::core::primitive::u32,
						>,
					},
					#[codec(index = 20)]
					#[doc = "See [`Pallet::claim_commission`]."]
					claim_commission { pool_id: ::core::primitive::u32 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum DefensiveError {
					#[codec(index = 0)]
					NotEnoughSpaceInUnbondPool,
					#[codec(index = 1)]
					PoolNotFound,
					#[codec(index = 2)]
					RewardPoolNotFound,
					#[codec(index = 3)]
					SubPoolsNotFound,
					#[codec(index = 4)]
					BondedStashKilledPrematurely,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "A (bonded) pool id does not exist."]
					PoolNotFound,
					#[codec(index = 1)]
					#[doc = "An account is not a member."]
					PoolMemberNotFound,
					#[codec(index = 2)]
					#[doc = "A reward pool does not exist. In all cases this is a system logic error."]
					RewardPoolNotFound,
					#[codec(index = 3)]
					#[doc = "A sub pool does not exist."]
					SubPoolsNotFound,
					#[codec(index = 4)]
					#[doc = "An account is already delegating in another pool. An account may only belong to one"]
					#[doc = "pool at a time."]
					AccountBelongsToOtherPool,
					#[codec(index = 5)]
					#[doc = "The member is fully unbonded (and thus cannot access the bonded and reward pool"]
					#[doc = "anymore to, for example, collect rewards)."]
					FullyUnbonding,
					#[codec(index = 6)]
					#[doc = "The member cannot unbond further chunks due to reaching the limit."]
					MaxUnbondingLimit,
					#[codec(index = 7)]
					#[doc = "None of the funds can be withdrawn yet because the bonding duration has not passed."]
					CannotWithdrawAny,
					#[codec(index = 8)]
					#[doc = "The amount does not meet the minimum bond to either join or create a pool."]
					#[doc = ""]
					#[doc = "The depositor can never unbond to a value less than"]
					#[doc = "`Pallet::depositor_min_bond`. The caller does not have nominating"]
					#[doc = "permissions for the pool. Members can never unbond to a value below `MinJoinBond`."]
					MinimumBondNotMet,
					#[codec(index = 9)]
					#[doc = "The transaction could not be executed due to overflow risk for the pool."]
					OverflowRisk,
					#[codec(index = 10)]
					#[doc = "A pool must be in [`PoolState::Destroying`] in order for the depositor to unbond or for"]
					#[doc = "other members to be permissionlessly unbonded."]
					NotDestroying,
					#[codec(index = 11)]
					#[doc = "The caller does not have nominating permissions for the pool."]
					NotNominator,
					#[codec(index = 12)]
					#[doc = "Either a) the caller cannot make a valid kick or b) the pool is not destroying."]
					NotKickerOrDestroying,
					#[codec(index = 13)]
					#[doc = "The pool is not open to join"]
					NotOpen,
					#[codec(index = 14)]
					#[doc = "The system is maxed out on pools."]
					MaxPools,
					#[codec(index = 15)]
					#[doc = "Too many members in the pool or system."]
					MaxPoolMembers,
					#[codec(index = 16)]
					#[doc = "The pools state cannot be changed."]
					CanNotChangeState,
					#[codec(index = 17)]
					#[doc = "The caller does not have adequate permissions."]
					DoesNotHavePermission,
					#[codec(index = 18)]
					#[doc = "Metadata exceeds [`Config::MaxMetadataLen`]"]
					MetadataExceedsMaxLen,
					#[codec(index = 19)]
					#[doc = "Some error occurred that should never happen. This should be reported to the"]
					#[doc = "maintainers."]
					Defensive(runtime_types::pallet_nomination_pools::pallet::DefensiveError),
					#[codec(index = 20)]
					#[doc = "Partial unbonding now allowed permissionlessly."]
					PartialUnbondNotAllowedPermissionlessly,
					#[codec(index = 21)]
					#[doc = "The pool's max commission cannot be set higher than the existing value."]
					MaxCommissionRestricted,
					#[codec(index = 22)]
					#[doc = "The supplied commission exceeds the max allowed commission."]
					CommissionExceedsMaximum,
					#[codec(index = 23)]
					#[doc = "The supplied commission exceeds global maximum commission."]
					CommissionExceedsGlobalMaximum,
					#[codec(index = 24)]
					#[doc = "Not enough blocks have surpassed since the last commission update."]
					CommissionChangeThrottled,
					#[codec(index = 25)]
					#[doc = "The submitted changes to commission change rate are not allowed."]
					CommissionChangeRateNotAllowed,
					#[codec(index = 26)]
					#[doc = "There is no pending commission to claim."]
					NoPendingCommission,
					#[codec(index = 27)]
					#[doc = "No commission current has been set."]
					NoCommissionCurrentSet,
					#[codec(index = 28)]
					#[doc = "Pool id currently in use."]
					PoolIdInUse,
					#[codec(index = 29)]
					#[doc = "Pool id provided is not correct/usable."]
					InvalidPoolId,
					#[codec(index = 30)]
					#[doc = "Bonding extra is restricted to the exact pending reward amount."]
					BondExtraRestricted,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Events of this pallet."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A pool has been created."]
					Created {
						depositor: ::subxt::ext::sp_core::crypto::AccountId32,
						pool_id: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "A member has became bonded in a pool."]
					Bonded {
						member: ::subxt::ext::sp_core::crypto::AccountId32,
						pool_id: ::core::primitive::u32,
						bonded: ::core::primitive::u128,
						joined: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					#[doc = "A payout has been made to a member."]
					PaidOut {
						member: ::subxt::ext::sp_core::crypto::AccountId32,
						pool_id: ::core::primitive::u32,
						payout: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "A member has unbonded from their pool."]
					#[doc = ""]
					#[doc = "- `balance` is the corresponding balance of the number of points that has been"]
					#[doc = "  requested to be unbonded (the argument of the `unbond` transaction) from the bonded"]
					#[doc = "  pool."]
					#[doc = "- `points` is the number of points that are issued as a result of `balance` being"]
					#[doc = "dissolved into the corresponding unbonding pool."]
					#[doc = "- `era` is the era in which the balance will be unbonded."]
					#[doc = "In the absence of slashing, these values will match. In the presence of slashing, the"]
					#[doc = "number of points that are issued in the unbonding pool will be less than the amount"]
					#[doc = "requested to be unbonded."]
					Unbonded {
						member: ::subxt::ext::sp_core::crypto::AccountId32,
						pool_id: ::core::primitive::u32,
						balance: ::core::primitive::u128,
						points: ::core::primitive::u128,
						era: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					#[doc = "A member has withdrawn from their pool."]
					#[doc = ""]
					#[doc = "The given number of `points` have been dissolved in return of `balance`."]
					#[doc = ""]
					#[doc = "Similar to `Unbonded` event, in the absence of slashing, the ratio of point to balance"]
					#[doc = "will be 1."]
					Withdrawn {
						member: ::subxt::ext::sp_core::crypto::AccountId32,
						pool_id: ::core::primitive::u32,
						balance: ::core::primitive::u128,
						points: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					#[doc = "A pool has been destroyed."]
					Destroyed { pool_id: ::core::primitive::u32 },
					#[codec(index = 6)]
					#[doc = "The state of a pool has changed"]
					StateChanged {
						pool_id: ::core::primitive::u32,
						new_state: runtime_types::pallet_nomination_pools::PoolState,
					},
					#[codec(index = 7)]
					#[doc = "A member has been removed from a pool."]
					#[doc = ""]
					#[doc = "The removal can be voluntary (withdrawn all unbonded funds) or involuntary (kicked)."]
					MemberRemoved {
						pool_id: ::core::primitive::u32,
						member: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 8)]
					#[doc = "The roles of a pool have been updated to the given new roles. Note that the depositor"]
					#[doc = "can never change."]
					RolesUpdated {
						root: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
						bouncer: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
						nominator:
							::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
					},
					#[codec(index = 9)]
					#[doc = "The active balance of pool `pool_id` has been slashed to `balance`."]
					PoolSlashed {
						pool_id: ::core::primitive::u32,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 10)]
					#[doc = "The unbond pool at `era` of pool `pool_id` has been slashed to `balance`."]
					UnbondingPoolSlashed {
						pool_id: ::core::primitive::u32,
						era: ::core::primitive::u32,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 11)]
					#[doc = "A pool's commission setting has been changed."]
					PoolCommissionUpdated {
						pool_id: ::core::primitive::u32,
						current: ::core::option::Option<(
							runtime_types::sp_arithmetic::per_things::Perbill,
							::subxt::ext::sp_core::crypto::AccountId32,
						)>,
					},
					#[codec(index = 12)]
					#[doc = "A pool's maximum commission setting has been changed."]
					PoolMaxCommissionUpdated {
						pool_id: ::core::primitive::u32,
						max_commission: runtime_types::sp_arithmetic::per_things::Perbill,
					},
					#[codec(index = 13)]
					#[doc = "A pool's commission `change_rate` has been changed."]
					PoolCommissionChangeRateUpdated {
						pool_id: ::core::primitive::u32,
						change_rate: runtime_types::pallet_nomination_pools::CommissionChangeRate<
							::core::primitive::u32,
						>,
					},
					#[codec(index = 14)]
					#[doc = "Pool commission has been claimed."]
					PoolCommissionClaimed {
						pool_id: ::core::primitive::u32,
						commission: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum BondExtra<_0> {
				#[codec(index = 0)]
				FreeBalance(_0),
				#[codec(index = 1)]
				Rewards,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct BondedPoolInner {
				pub commission: runtime_types::pallet_nomination_pools::Commission,
				pub member_counter: ::core::primitive::u32,
				pub points: ::core::primitive::u128,
				pub roles: runtime_types::pallet_nomination_pools::PoolRoles<
					::subxt::ext::sp_core::crypto::AccountId32,
				>,
				pub state: runtime_types::pallet_nomination_pools::PoolState,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum ClaimPermission {
				#[codec(index = 0)]
				Permissioned,
				#[codec(index = 1)]
				PermissionlessCompound,
				#[codec(index = 2)]
				PermissionlessWithdraw,
				#[codec(index = 3)]
				PermissionlessAll,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Commission {
				pub current: ::core::option::Option<(
					runtime_types::sp_arithmetic::per_things::Perbill,
					::subxt::ext::sp_core::crypto::AccountId32,
				)>,
				pub max: ::core::option::Option<runtime_types::sp_arithmetic::per_things::Perbill>,
				pub change_rate: ::core::option::Option<
					runtime_types::pallet_nomination_pools::CommissionChangeRate<
						::core::primitive::u32,
					>,
				>,
				pub throttle_from: ::core::option::Option<::core::primitive::u32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct CommissionChangeRate<_0> {
				pub max_increase: runtime_types::sp_arithmetic::per_things::Perbill,
				pub min_delay: _0,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum ConfigOp<_0> {
				#[codec(index = 0)]
				Noop,
				#[codec(index = 1)]
				Set(_0),
				#[codec(index = 2)]
				Remove,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct PoolMember {
				pub pool_id: ::core::primitive::u32,
				pub points: ::core::primitive::u128,
				pub last_recorded_reward_counter:
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
				pub unbonding_eras:
					runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
						::core::primitive::u32,
						::core::primitive::u128,
					>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct PoolRoles<_0> {
				pub depositor: _0,
				pub root: ::core::option::Option<_0>,
				pub nominator: ::core::option::Option<_0>,
				pub bouncer: ::core::option::Option<_0>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum PoolState {
				#[codec(index = 0)]
				Open,
				#[codec(index = 1)]
				Blocked,
				#[codec(index = 2)]
				Destroying,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct RewardPool {
				pub last_recorded_reward_counter:
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
				pub last_recorded_total_payouts: ::core::primitive::u128,
				pub total_rewards_claimed: ::core::primitive::u128,
				pub total_commission_pending: ::core::primitive::u128,
				pub total_commission_claimed: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SubPools {
				pub no_era: runtime_types::pallet_nomination_pools::UnbondPool,
				pub with_era:
					runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
						::core::primitive::u32,
						runtime_types::pallet_nomination_pools::UnbondPool,
					>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct UnbondPool {
				pub points: ::core::primitive::u128,
				pub balance: ::core::primitive::u128,
			}
		}
		pub mod pallet_offences {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Events type."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "There is an offence reported of the given `kind` happened at the `session_index` and"]
					#[doc = "(kind-specific) time slot. This event is not deposited for duplicate slashes."]
					#[doc = "\\[kind, timeslot\\]."]
					Offence {
						kind: [::core::primitive::u8; 16usize],
						timeslot: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
			}
		}
		pub mod pallet_preimage {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::note_preimage`]."]
					note_preimage {
						bytes: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::unnote_preimage`]."]
					unnote_preimage { hash: ::subxt::ext::sp_core::H256 },
					#[codec(index = 2)]
					#[doc = "See [`Pallet::request_preimage`]."]
					request_preimage { hash: ::subxt::ext::sp_core::H256 },
					#[codec(index = 3)]
					#[doc = "See [`Pallet::unrequest_preimage`]."]
					unrequest_preimage { hash: ::subxt::ext::sp_core::H256 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Preimage is too large to store on-chain."]
					TooBig,
					#[codec(index = 1)]
					#[doc = "Preimage has already been noted on-chain."]
					AlreadyNoted,
					#[codec(index = 2)]
					#[doc = "The user is not authorized to perform this action."]
					NotAuthorized,
					#[codec(index = 3)]
					#[doc = "The preimage cannot be removed since it has not yet been noted."]
					NotNoted,
					#[codec(index = 4)]
					#[doc = "A preimage may not be removed when there are outstanding requests."]
					Requested,
					#[codec(index = 5)]
					#[doc = "The preimage request cannot be removed since no outstanding requests exist."]
					NotRequested,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A preimage has been noted."]
					Noted { hash: ::subxt::ext::sp_core::H256 },
					#[codec(index = 1)]
					#[doc = "A preimage has been requested."]
					Requested { hash: ::subxt::ext::sp_core::H256 },
					#[codec(index = 2)]
					#[doc = "A preimage has ben cleared."]
					Cleared { hash: ::subxt::ext::sp_core::H256 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum RequestStatus<_0, _1> {
				#[codec(index = 0)]
				Unrequested {
					deposit: (_0, _1),
					len: ::core::primitive::u32,
				},
				#[codec(index = 1)]
				Requested {
					deposit: ::core::option::Option<(_0, _1)>,
					count: ::core::primitive::u32,
					len: ::core::option::Option<::core::primitive::u32>,
				},
			}
		}
		pub mod pallet_scheduler {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::schedule`]."]
					schedule {
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::cancel`]."]
					cancel {
						when: ::core::primitive::u32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::schedule_named`]."]
					schedule_named {
						id: [::core::primitive::u8; 32usize],
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::cancel_named`]."]
					cancel_named {
						id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::schedule_after`]."]
					schedule_after {
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::schedule_named_after`]."]
					schedule_named_after {
						id: [::core::primitive::u8; 32usize],
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Failed to schedule a call"]
					FailedToSchedule,
					#[codec(index = 1)]
					#[doc = "Cannot find the scheduled call."]
					NotFound,
					#[codec(index = 2)]
					#[doc = "Given target block number is in the past."]
					TargetBlockNumberInPast,
					#[codec(index = 3)]
					#[doc = "Reschedule failed because it does not change scheduled time."]
					RescheduleNoChange,
					#[codec(index = 4)]
					#[doc = "Attempt to use a non-named function on a named task."]
					Named,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Events type."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Scheduled some task."]
					Scheduled {
						when: ::core::primitive::u32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "Canceled some task."]
					Canceled {
						when: ::core::primitive::u32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Dispatched some task."]
					Dispatched {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 3)]
					#[doc = "The call for the provided hash was not found so the task has been aborted."]
					CallUnavailable {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 4)]
					#[doc = "The given task was unable to be renewed since the agenda is full at that block."]
					PeriodicFailed {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 5)]
					#[doc = "The given task can never be executed since it is overweight."]
					PermanentlyOverweight {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Scheduled<_0, _1, _2, _3, _4> {
				pub maybe_id: ::core::option::Option<_0>,
				pub priority: ::core::primitive::u8,
				pub call: _1,
				pub maybe_periodic: ::core::option::Option<(_2, _2)>,
				pub origin: _3,
				#[codec(skip)]
				pub __subxt_unused_type_params: ::core::marker::PhantomData<_4>,
			}
		}
		pub mod pallet_session {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::set_keys`]."]
					set_keys {
						keys: runtime_types::da_runtime::primitives::SessionKeys,
						proof: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::purge_keys`]."]
					purge_keys,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Error for the session pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Invalid ownership proof."]
					InvalidProof,
					#[codec(index = 1)]
					#[doc = "No associated validator ID for account."]
					NoAssociatedValidatorId,
					#[codec(index = 2)]
					#[doc = "Registered duplicate key."]
					DuplicatedKey,
					#[codec(index = 3)]
					#[doc = "No keys are associated with this account."]
					NoKeys,
					#[codec(index = 4)]
					#[doc = "Key setting account is not live, so it's impossible to associate keys."]
					NoAccount,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New session has happened. Note that the argument is the session index, not the"]
					#[doc = "block number as the type might suggest."]
					NewSession {
						session_index: ::core::primitive::u32,
					},
				}
			}
		}
		pub mod pallet_staking {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
					pub enum Call {
						#[codec(index = 0)]
						#[doc = "See [`Pallet::bond`]."]
						bond {
							#[codec(compact)]
							value: ::core::primitive::u128,
							payee: runtime_types::pallet_staking::RewardDestination<
								::subxt::ext::sp_core::crypto::AccountId32,
							>,
						},
						#[codec(index = 1)]
						#[doc = "See [`Pallet::bond_extra`]."]
						bond_extra {
							#[codec(compact)]
							max_additional: ::core::primitive::u128,
						},
						#[codec(index = 2)]
						#[doc = "See [`Pallet::unbond`]."]
						unbond {
							#[codec(compact)]
							value: ::core::primitive::u128,
						},
						#[codec(index = 3)]
						#[doc = "See [`Pallet::withdraw_unbonded`]."]
						withdraw_unbonded {
							num_slashing_spans: ::core::primitive::u32,
						},
						#[codec(index = 4)]
						#[doc = "See [`Pallet::validate`]."]
						validate {
							prefs: runtime_types::pallet_staking::ValidatorPrefs,
						},
						#[codec(index = 5)]
						#[doc = "See [`Pallet::nominate`]."]
						nominate {
							targets: ::std::vec::Vec<
								::subxt::ext::sp_runtime::MultiAddress<
									::subxt::ext::sp_core::crypto::AccountId32,
									::core::primitive::u32,
								>,
							>,
						},
						#[codec(index = 6)]
						#[doc = "See [`Pallet::chill`]."]
						chill,
						#[codec(index = 7)]
						#[doc = "See [`Pallet::set_payee`]."]
						set_payee {
							payee: runtime_types::pallet_staking::RewardDestination<
								::subxt::ext::sp_core::crypto::AccountId32,
							>,
						},
						#[codec(index = 8)]
						#[doc = "See [`Pallet::set_controller`]."]
						set_controller,
						#[codec(index = 9)]
						#[doc = "See [`Pallet::set_validator_count`]."]
						set_validator_count {
							#[codec(compact)]
							new: ::core::primitive::u32,
						},
						#[codec(index = 10)]
						#[doc = "See [`Pallet::increase_validator_count`]."]
						increase_validator_count {
							#[codec(compact)]
							additional: ::core::primitive::u32,
						},
						#[codec(index = 11)]
						#[doc = "See [`Pallet::scale_validator_count`]."]
						scale_validator_count {
							factor: runtime_types::sp_arithmetic::per_things::Percent,
						},
						#[codec(index = 12)]
						#[doc = "See [`Pallet::force_no_eras`]."]
						force_no_eras,
						#[codec(index = 13)]
						#[doc = "See [`Pallet::force_new_era`]."]
						force_new_era,
						#[codec(index = 14)]
						#[doc = "See [`Pallet::set_invulnerables`]."]
						set_invulnerables {
							invulnerables:
								::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						},
						#[codec(index = 15)]
						#[doc = "See [`Pallet::force_unstake`]."]
						force_unstake {
							stash: ::subxt::ext::sp_core::crypto::AccountId32,
							num_slashing_spans: ::core::primitive::u32,
						},
						#[codec(index = 16)]
						#[doc = "See [`Pallet::force_new_era_always`]."]
						force_new_era_always,
						#[codec(index = 17)]
						#[doc = "See [`Pallet::cancel_deferred_slash`]."]
						cancel_deferred_slash {
							era: ::core::primitive::u32,
							slash_indices: ::std::vec::Vec<::core::primitive::u32>,
						},
						#[codec(index = 18)]
						#[doc = "See [`Pallet::payout_stakers`]."]
						payout_stakers {
							validator_stash: ::subxt::ext::sp_core::crypto::AccountId32,
							era: ::core::primitive::u32,
						},
						#[codec(index = 19)]
						#[doc = "See [`Pallet::rebond`]."]
						rebond {
							#[codec(compact)]
							value: ::core::primitive::u128,
						},
						#[codec(index = 20)]
						#[doc = "See [`Pallet::reap_stash`]."]
						reap_stash {
							stash: ::subxt::ext::sp_core::crypto::AccountId32,
							num_slashing_spans: ::core::primitive::u32,
						},
						#[codec(index = 21)]
						#[doc = "See [`Pallet::kick`]."]
						kick {
							who: ::std::vec::Vec<
								::subxt::ext::sp_runtime::MultiAddress<
									::subxt::ext::sp_core::crypto::AccountId32,
									::core::primitive::u32,
								>,
							>,
						},
						#[codec(index = 22)]
						#[doc = "See [`Pallet::set_staking_configs`]."]
						set_staking_configs {
							min_nominator_bond:
								runtime_types::pallet_staking::pallet::pallet::ConfigOp<
									::core::primitive::u128,
								>,
							min_validator_bond:
								runtime_types::pallet_staking::pallet::pallet::ConfigOp<
									::core::primitive::u128,
								>,
							max_nominator_count:
								runtime_types::pallet_staking::pallet::pallet::ConfigOp<
									::core::primitive::u32,
								>,
							max_validator_count:
								runtime_types::pallet_staking::pallet::pallet::ConfigOp<
									::core::primitive::u32,
								>,
							chill_threshold:
								runtime_types::pallet_staking::pallet::pallet::ConfigOp<
									runtime_types::sp_arithmetic::per_things::Percent,
								>,
							min_commission: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
								runtime_types::sp_arithmetic::per_things::Perbill,
							>,
						},
						#[codec(index = 23)]
						#[doc = "See [`Pallet::chill_other`]."]
						chill_other {
							controller: ::subxt::ext::sp_core::crypto::AccountId32,
						},
						#[codec(index = 24)]
						#[doc = "See [`Pallet::force_apply_min_commission`]."]
						force_apply_min_commission {
							validator_stash: ::subxt::ext::sp_core::crypto::AccountId32,
						},
						#[codec(index = 25)]
						#[doc = "See [`Pallet::set_min_commission`]."]
						set_min_commission {
							new: runtime_types::sp_arithmetic::per_things::Perbill,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub enum ConfigOp<_0> {
						#[codec(index = 0)]
						Noop,
						#[codec(index = 1)]
						Set(_0),
						#[codec(index = 2)]
						Remove,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					#[doc = "The `Error` enum of this pallet."]
					pub enum Error {
						#[codec(index = 0)]
						#[doc = "Not a controller account."]
						NotController,
						#[codec(index = 1)]
						#[doc = "Not a stash account."]
						NotStash,
						#[codec(index = 2)]
						#[doc = "Stash is already bonded."]
						AlreadyBonded,
						#[codec(index = 3)]
						#[doc = "Controller is already paired."]
						AlreadyPaired,
						#[codec(index = 4)]
						#[doc = "Targets cannot be empty."]
						EmptyTargets,
						#[codec(index = 5)]
						#[doc = "Duplicate index."]
						DuplicateIndex,
						#[codec(index = 6)]
						#[doc = "Slash record index out of bounds."]
						InvalidSlashIndex,
						#[codec(index = 7)]
						#[doc = "Cannot have a validator or nominator role, with value less than the minimum defined by"]
						#[doc = "governance (see `MinValidatorBond` and `MinNominatorBond`). If unbonding is the"]
						#[doc = "intention, `chill` first to remove one's role as validator/nominator."]
						InsufficientBond,
						#[codec(index = 8)]
						#[doc = "Can not schedule more unlock chunks."]
						NoMoreChunks,
						#[codec(index = 9)]
						#[doc = "Can not rebond without unlocking chunks."]
						NoUnlockChunk,
						#[codec(index = 10)]
						#[doc = "Attempting to target a stash that still has funds."]
						FundedTarget,
						#[codec(index = 11)]
						#[doc = "Invalid era to reward."]
						InvalidEraToReward,
						#[codec(index = 12)]
						#[doc = "Invalid number of nominations."]
						InvalidNumberOfNominations,
						#[codec(index = 13)]
						#[doc = "Items are not sorted and unique."]
						NotSortedAndUnique,
						#[codec(index = 14)]
						#[doc = "Rewards for this era have already been claimed for this validator."]
						AlreadyClaimed,
						#[codec(index = 15)]
						#[doc = "Incorrect previous history depth input provided."]
						IncorrectHistoryDepth,
						#[codec(index = 16)]
						#[doc = "Incorrect number of slashing spans provided."]
						IncorrectSlashingSpans,
						#[codec(index = 17)]
						#[doc = "Internal state has become somehow corrupted and the operation cannot continue."]
						BadState,
						#[codec(index = 18)]
						#[doc = "Too many nomination targets supplied."]
						TooManyTargets,
						#[codec(index = 19)]
						#[doc = "A nomination target was supplied that was blocked or otherwise not a validator."]
						BadTarget,
						#[codec(index = 20)]
						#[doc = "The user has enough bond and thus cannot be chilled forcefully by an external person."]
						CannotChillOther,
						#[codec(index = 21)]
						#[doc = "There are too many nominators in the system. Governance needs to adjust the staking"]
						#[doc = "settings to keep things safe for the runtime."]
						TooManyNominators,
						#[codec(index = 22)]
						#[doc = "There are too many validator candidates in the system. Governance needs to adjust the"]
						#[doc = "staking settings to keep things safe for the runtime."]
						TooManyValidators,
						#[codec(index = 23)]
						#[doc = "Commission is too low. Must be at least `MinCommission`."]
						CommissionTooLow,
						#[codec(index = 24)]
						#[doc = "Some bound is not met."]
						BoundNotMet,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					#[doc = "The `Event` enum of this pallet"]
					pub enum Event {
						#[codec(index = 0)]
						#[doc = "The era payout has been set; the first balance is the validator-payout; the second is"]
						#[doc = "the remainder from the maximum amount of reward."]
						EraPaid {
							era_index: ::core::primitive::u32,
							validator_payout: ::core::primitive::u128,
							remainder: ::core::primitive::u128,
						},
						#[codec(index = 1)]
						#[doc = "The nominator has been rewarded by this amount."]
						Rewarded {
							stash: ::subxt::ext::sp_core::crypto::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 2)]
						#[doc = "A staker (validator or nominator) has been slashed by the given amount."]
						Slashed {
							staker: ::subxt::ext::sp_core::crypto::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 3)]
						#[doc = "A slash for the given validator, for the given percentage of their stake, at the given"]
						#[doc = "era as been reported."]
						SlashReported {
							validator: ::subxt::ext::sp_core::crypto::AccountId32,
							fraction: runtime_types::sp_arithmetic::per_things::Perbill,
							slash_era: ::core::primitive::u32,
						},
						#[codec(index = 4)]
						#[doc = "An old slashing report from a prior era was discarded because it could"]
						#[doc = "not be processed."]
						OldSlashingReportDiscarded {
							session_index: ::core::primitive::u32,
						},
						#[codec(index = 5)]
						#[doc = "A new set of stakers was elected."]
						StakersElected,
						#[codec(index = 6)]
						#[doc = "An account has bonded this amount. \\[stash, amount\\]"]
						#[doc = ""]
						#[doc = "NOTE: This event is only emitted when funds are bonded via a dispatchable. Notably,"]
						#[doc = "it will not be emitted for staking rewards when they are added to stake."]
						Bonded {
							stash: ::subxt::ext::sp_core::crypto::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 7)]
						#[doc = "An account has unbonded this amount."]
						Unbonded {
							stash: ::subxt::ext::sp_core::crypto::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 8)]
						#[doc = "An account has called `withdraw_unbonded` and removed unbonding chunks worth `Balance`"]
						#[doc = "from the unlocking queue."]
						Withdrawn {
							stash: ::subxt::ext::sp_core::crypto::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 9)]
						#[doc = "A nominator has been kicked from a validator."]
						Kicked {
							nominator: ::subxt::ext::sp_core::crypto::AccountId32,
							stash: ::subxt::ext::sp_core::crypto::AccountId32,
						},
						#[codec(index = 10)]
						#[doc = "The election failed. No new era is planned."]
						StakingElectionFailed,
						#[codec(index = 11)]
						#[doc = "An account has stopped participating as either a validator or nominator."]
						Chilled {
							stash: ::subxt::ext::sp_core::crypto::AccountId32,
						},
						#[codec(index = 12)]
						#[doc = "The stakers' rewards are getting paid."]
						PayoutStarted {
							era_index: ::core::primitive::u32,
							validator_stash: ::subxt::ext::sp_core::crypto::AccountId32,
						},
						#[codec(index = 13)]
						#[doc = "A validator has set their preferences."]
						ValidatorPrefsSet {
							stash: ::subxt::ext::sp_core::crypto::AccountId32,
							prefs: runtime_types::pallet_staking::ValidatorPrefs,
						},
						#[codec(index = 14)]
						#[doc = "A new force era mode was set."]
						ForceEra {
							mode: runtime_types::pallet_staking::Forcing,
						},
					}
				}
			}
			pub mod slashing {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct SlashingSpans {
					pub span_index: ::core::primitive::u32,
					pub last_start: ::core::primitive::u32,
					pub last_nonzero_slash: ::core::primitive::u32,
					pub prior: ::std::vec::Vec<::core::primitive::u32>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct SpanRecord<_0> {
					pub slashed: _0,
					pub paid_out: _0,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ActiveEraInfo {
				pub index: ::core::primitive::u32,
				pub start: ::core::option::Option<::core::primitive::u64>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct EraRewardPoints<_0> {
				pub total: ::core::primitive::u32,
				pub individual: ::subxt::utils::KeyedVec<_0, ::core::primitive::u32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Exposure<_0, _1> {
				#[codec(compact)]
				pub total: _1,
				#[codec(compact)]
				pub own: _1,
				pub others:
					::std::vec::Vec<runtime_types::pallet_staking::IndividualExposure<_0, _1>>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum Forcing {
				#[codec(index = 0)]
				NotForcing,
				#[codec(index = 1)]
				ForceNew,
				#[codec(index = 2)]
				ForceNone,
				#[codec(index = 3)]
				ForceAlways,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct IndividualExposure<_0, _1> {
				pub who: _0,
				#[codec(compact)]
				pub value: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Nominations {
				pub targets: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					::subxt::ext::sp_core::crypto::AccountId32,
				>,
				pub submitted_in: ::core::primitive::u32,
				pub suppressed: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum RewardDestination<_0> {
				#[codec(index = 0)]
				Staked,
				#[codec(index = 1)]
				Stash,
				#[codec(index = 2)]
				Controller,
				#[codec(index = 3)]
				Account(_0),
				#[codec(index = 4)]
				None,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct StakingLedger {
				pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
				#[codec(compact)]
				pub total: ::core::primitive::u128,
				#[codec(compact)]
				pub active: ::core::primitive::u128,
				pub unlocking: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					runtime_types::pallet_staking::UnlockChunk<::core::primitive::u128>,
				>,
				pub claimed_rewards: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct UnappliedSlash<_0, _1> {
				pub validator: _0,
				pub own: _1,
				pub others: ::std::vec::Vec<(_0, _1)>,
				pub reporters: ::std::vec::Vec<_0>,
				pub payout: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct UnlockChunk<_0> {
				#[codec(compact)]
				pub value: _0,
				#[codec(compact)]
				pub era: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ValidatorPrefs {
				#[codec(compact)]
				pub commission: runtime_types::sp_arithmetic::per_things::Perbill,
				pub blocked: ::core::primitive::bool,
			}
		}
		pub mod pallet_sudo {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::sudo`]."]
					sudo {
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::sudo_unchecked_weight`]."]
					sudo_unchecked_weight {
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::set_key`]."]
					set_key {
						new: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::sudo_as`]."]
					sudo_as {
						who: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Error for the Sudo pallet"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Sender must be the Sudo account"]
					RequireSudo,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A sudo just took place. \\[result\\]"]
					Sudid {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					#[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
					KeyChanged {
						old_sudoer:
							::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
					},
					#[codec(index = 2)]
					#[doc = "A sudo just took place. \\[result\\]"]
					SudoAsDone {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod pallet_timestamp {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::set`]."]
					set {
						#[codec(compact)]
						now: ::core::primitive::u64,
					},
				}
			}
		}
		pub mod pallet_tips {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::report_awesome`]."]
					report_awesome {
						reason: ::std::vec::Vec<::core::primitive::u8>,
						who: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::retract_tip`]."]
					retract_tip { hash: ::subxt::ext::sp_core::H256 },
					#[codec(index = 2)]
					#[doc = "See [`Pallet::tip_new`]."]
					tip_new {
						reason: ::std::vec::Vec<::core::primitive::u8>,
						who: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						tip_value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::tip`]."]
					tip {
						hash: ::subxt::ext::sp_core::H256,
						#[codec(compact)]
						tip_value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::close_tip`]."]
					close_tip { hash: ::subxt::ext::sp_core::H256 },
					#[codec(index = 5)]
					#[doc = "See [`Pallet::slash_tip`]."]
					slash_tip { hash: ::subxt::ext::sp_core::H256 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The reason given is just too big."]
					ReasonTooBig,
					#[codec(index = 1)]
					#[doc = "The tip was already found/started."]
					AlreadyKnown,
					#[codec(index = 2)]
					#[doc = "The tip hash is unknown."]
					UnknownTip,
					#[codec(index = 3)]
					#[doc = "The account attempting to retract the tip is not the finder of the tip."]
					NotFinder,
					#[codec(index = 4)]
					#[doc = "The tip cannot be claimed/closed because there are not enough tippers yet."]
					StillOpen,
					#[codec(index = 5)]
					#[doc = "The tip cannot be claimed/closed because it's still in the countdown period."]
					Premature,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A new tip suggestion has been opened."]
					NewTip {
						tip_hash: ::subxt::ext::sp_core::H256,
					},
					#[codec(index = 1)]
					#[doc = "A tip suggestion has reached threshold and is closing."]
					TipClosing {
						tip_hash: ::subxt::ext::sp_core::H256,
					},
					#[codec(index = 2)]
					#[doc = "A tip suggestion has been closed."]
					TipClosed {
						tip_hash: ::subxt::ext::sp_core::H256,
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						payout: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "A tip suggestion has been retracted."]
					TipRetracted {
						tip_hash: ::subxt::ext::sp_core::H256,
					},
					#[codec(index = 4)]
					#[doc = "A tip suggestion has been slashed."]
					TipSlashed {
						tip_hash: ::subxt::ext::sp_core::H256,
						finder: ::subxt::ext::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct OpenTip<_0, _1, _2, _3> {
				pub reason: _3,
				pub who: _0,
				pub finder: _0,
				pub deposit: _1,
				pub closes: ::core::option::Option<_2>,
				pub tips: ::std::vec::Vec<(_0, _1)>,
				pub finders_fee: ::core::primitive::bool,
			}
		}
		pub mod pallet_transaction_payment {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
					#[doc = "has been paid by `who`."]
					TransactionFeePaid {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						actual_fee: ::core::primitive::u128,
						tip: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum Releases {
				#[codec(index = 0)]
				V1Ancient,
				#[codec(index = 1)]
				V2,
			}
		}
		pub mod pallet_treasury {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::propose_spend`]."]
					propose_spend {
						#[codec(compact)]
						value: ::core::primitive::u128,
						beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::reject_proposal`]."]
					reject_proposal {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::approve_proposal`]."]
					approve_proposal {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::spend`]."]
					spend {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::remove_approval`]."]
					remove_approval {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Error for the treasury pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Proposer's balance is too low."]
					InsufficientProposersBalance,
					#[codec(index = 1)]
					#[doc = "No proposal or bounty at that index."]
					InvalidIndex,
					#[codec(index = 2)]
					#[doc = "Too many approvals in the queue."]
					TooManyApprovals,
					#[codec(index = 3)]
					#[doc = "The spend origin is valid but the amount it is allowed to spend is lower than the"]
					#[doc = "amount to be spent."]
					InsufficientPermission,
					#[codec(index = 4)]
					#[doc = "Proposal has not been approved."]
					ProposalNotApproved,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New proposal."]
					Proposed {
						proposal_index: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "We have ended a spend period and will now allocate funds."]
					Spending {
						budget_remaining: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Some funds have been allocated."]
					Awarded {
						proposal_index: ::core::primitive::u32,
						award: ::core::primitive::u128,
						account: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 3)]
					#[doc = "A proposal was rejected; funds were slashed."]
					Rejected {
						proposal_index: ::core::primitive::u32,
						slashed: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Some of our funds have been burnt."]
					Burnt {
						burnt_funds: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					#[doc = "Spending has finished; this is the amount that rolls over until next spend."]
					Rollover {
						rollover_balance: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "Some funds have been deposited."]
					Deposit { value: ::core::primitive::u128 },
					#[codec(index = 7)]
					#[doc = "A new spend proposal has been approved."]
					SpendApproved {
						proposal_index: ::core::primitive::u32,
						amount: ::core::primitive::u128,
						beneficiary: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 8)]
					#[doc = "The inactive funds of the pallet have been updated."]
					UpdatedInactive {
						reactivated: ::core::primitive::u128,
						deactivated: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Proposal<_0, _1> {
				pub proposer: _0,
				pub value: _1,
				pub beneficiary: _0,
				pub bond: _1,
			}
		}
		pub mod pallet_utility {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::batch`]."]
					batch {
						calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::as_derivative`]."]
					as_derivative {
						index: ::core::primitive::u16,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::batch_all`]."]
					batch_all {
						calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::dispatch_as`]."]
					dispatch_as {
						as_origin: ::std::boxed::Box<runtime_types::da_runtime::OriginCaller>,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::force_batch`]."]
					force_batch {
						calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::with_weight`]."]
					with_weight {
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Too many calls batched."]
					TooManyCalls,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Batch of dispatches did not complete fully. Index of first failing dispatch given, as"]
					#[doc = "well as the error."]
					BatchInterrupted {
						index: ::core::primitive::u32,
						error: runtime_types::sp_runtime::DispatchError,
					},
					#[codec(index = 1)]
					#[doc = "Batch of dispatches completed fully with no error."]
					BatchCompleted,
					#[codec(index = 2)]
					#[doc = "Batch of dispatches completed but has errors."]
					BatchCompletedWithErrors,
					#[codec(index = 3)]
					#[doc = "A single item within a Batch of dispatches has completed with no error."]
					ItemCompleted,
					#[codec(index = 4)]
					#[doc = "A single item within a Batch of dispatches has completed with error."]
					ItemFailed {
						error: runtime_types::sp_runtime::DispatchError,
					},
					#[codec(index = 5)]
					#[doc = "A call was dispatched."]
					DispatchedAs {
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod primitive_types {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct H160(pub [::core::primitive::u8; 20usize]);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct H256(pub [::core::primitive::u8; 32usize]);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct U256(pub [::core::primitive::u64; 4usize]);
		}
		pub mod sp_arithmetic {
			use super::runtime_types;
			pub mod fixed_point {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct FixedU128(pub ::core::primitive::u128);
			}
			pub mod per_things {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct PerU16(pub ::core::primitive::u16);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Perbill(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Percent(pub ::core::primitive::u8);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Permill(pub ::core::primitive::u32);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum ArithmeticError {
				#[codec(index = 0)]
				Underflow,
				#[codec(index = 1)]
				Overflow,
				#[codec(index = 2)]
				DivisionByZero,
			}
		}
		pub mod sp_authority_discovery {
			use super::runtime_types;
			pub mod app {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Public(pub runtime_types::sp_core::sr25519::Public);
			}
		}
		pub mod sp_consensus_babe {
			use super::runtime_types;
			pub mod app {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Public(pub runtime_types::sp_core::sr25519::Public);
			}
			pub mod digests {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum NextConfigDescriptor {
					#[codec(index = 1)]
					V1 {
						c: (::core::primitive::u64, ::core::primitive::u64),
						allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum PreDigest {
					#[codec(index = 1)]
					Primary(runtime_types::sp_consensus_babe::digests::PrimaryPreDigest),
					#[codec(index = 2)]
					SecondaryPlain(
						runtime_types::sp_consensus_babe::digests::SecondaryPlainPreDigest,
					),
					#[codec(index = 3)]
					SecondaryVRF(runtime_types::sp_consensus_babe::digests::SecondaryVRFPreDigest),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct PrimaryPreDigest {
					pub authority_index: ::core::primitive::u32,
					pub slot: runtime_types::sp_consensus_slots::Slot,
					pub vrf_signature: runtime_types::sp_core::sr25519::vrf::VrfSignature,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct SecondaryPlainPreDigest {
					pub authority_index: ::core::primitive::u32,
					pub slot: runtime_types::sp_consensus_slots::Slot,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct SecondaryVRFPreDigest {
					pub authority_index: ::core::primitive::u32,
					pub slot: runtime_types::sp_consensus_slots::Slot,
					pub vrf_signature: runtime_types::sp_core::sr25519::vrf::VrfSignature,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum AllowedSlots {
				#[codec(index = 0)]
				PrimarySlots,
				#[codec(index = 1)]
				PrimaryAndSecondaryPlainSlots,
				#[codec(index = 2)]
				PrimaryAndSecondaryVRFSlots,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct BabeEpochConfiguration {
				pub c: (::core::primitive::u64, ::core::primitive::u64),
				pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
			}
		}
		pub mod sp_consensus_grandpa {
			use super::runtime_types;
			pub mod app {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Public(pub runtime_types::sp_core::ed25519::Public);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum Equivocation<_0, _1> {
				#[codec(index = 0)]
				Prevote(
					runtime_types::finality_grandpa::Equivocation<
						runtime_types::sp_consensus_grandpa::app::Public,
						runtime_types::finality_grandpa::Prevote<_0, _1>,
						runtime_types::sp_consensus_grandpa::app::Signature,
					>,
				),
				#[codec(index = 1)]
				Precommit(
					runtime_types::finality_grandpa::Equivocation<
						runtime_types::sp_consensus_grandpa::app::Public,
						runtime_types::finality_grandpa::Precommit<_0, _1>,
						runtime_types::sp_consensus_grandpa::app::Signature,
					>,
				),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct EquivocationProof<_0, _1> {
				pub set_id: ::core::primitive::u64,
				pub equivocation: runtime_types::sp_consensus_grandpa::Equivocation<_0, _1>,
			}
		}
		pub mod sp_consensus_slots {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct EquivocationProof<_0, _1> {
				pub offender: _1,
				pub slot: runtime_types::sp_consensus_slots::Slot,
				pub first_header: _0,
				pub second_header: _0,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Slot(pub ::core::primitive::u64);
		}
		pub mod sp_core {
			use super::runtime_types;
			pub mod crypto {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
			}
			pub mod ecdsa {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Signature(pub [::core::primitive::u8; 65usize]);
			}
			pub mod ed25519 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			pub mod sr25519 {
				use super::runtime_types;
				pub mod vrf {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub struct VrfSignature {
						pub output: [::core::primitive::u8; 32usize],
						pub proof: [::core::primitive::u8; 64usize],
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum Void {}
		}
		pub mod sp_npos_elections {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ElectionScore {
				pub minimal_stake: ::core::primitive::u128,
				pub sum_stake: ::core::primitive::u128,
				pub sum_stake_squared: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Support<_0> {
				pub total: ::core::primitive::u128,
				pub voters: ::std::vec::Vec<(_0, ::core::primitive::u128)>,
			}
		}
		pub mod sp_runtime {
			use super::runtime_types;
			pub mod generic {
				use super::runtime_types;
				pub mod digest {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub struct Digest {
						pub logs:
							::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub enum DigestItem {
						#[codec(index = 6)]
						PreRuntime(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 4)]
						Consensus(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 5)]
						Seal(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 0)]
						Other(::std::vec::Vec<::core::primitive::u8>),
						#[codec(index = 8)]
						RuntimeEnvironmentUpdated,
					}
				}
				pub mod era {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					pub enum Era {
						#[codec(index = 0)]
						Immortal,
						#[codec(index = 1)]
						Mortal1(::core::primitive::u8),
						#[codec(index = 2)]
						Mortal2(::core::primitive::u8),
						#[codec(index = 3)]
						Mortal3(::core::primitive::u8),
						#[codec(index = 4)]
						Mortal4(::core::primitive::u8),
						#[codec(index = 5)]
						Mortal5(::core::primitive::u8),
						#[codec(index = 6)]
						Mortal6(::core::primitive::u8),
						#[codec(index = 7)]
						Mortal7(::core::primitive::u8),
						#[codec(index = 8)]
						Mortal8(::core::primitive::u8),
						#[codec(index = 9)]
						Mortal9(::core::primitive::u8),
						#[codec(index = 10)]
						Mortal10(::core::primitive::u8),
						#[codec(index = 11)]
						Mortal11(::core::primitive::u8),
						#[codec(index = 12)]
						Mortal12(::core::primitive::u8),
						#[codec(index = 13)]
						Mortal13(::core::primitive::u8),
						#[codec(index = 14)]
						Mortal14(::core::primitive::u8),
						#[codec(index = 15)]
						Mortal15(::core::primitive::u8),
						#[codec(index = 16)]
						Mortal16(::core::primitive::u8),
						#[codec(index = 17)]
						Mortal17(::core::primitive::u8),
						#[codec(index = 18)]
						Mortal18(::core::primitive::u8),
						#[codec(index = 19)]
						Mortal19(::core::primitive::u8),
						#[codec(index = 20)]
						Mortal20(::core::primitive::u8),
						#[codec(index = 21)]
						Mortal21(::core::primitive::u8),
						#[codec(index = 22)]
						Mortal22(::core::primitive::u8),
						#[codec(index = 23)]
						Mortal23(::core::primitive::u8),
						#[codec(index = 24)]
						Mortal24(::core::primitive::u8),
						#[codec(index = 25)]
						Mortal25(::core::primitive::u8),
						#[codec(index = 26)]
						Mortal26(::core::primitive::u8),
						#[codec(index = 27)]
						Mortal27(::core::primitive::u8),
						#[codec(index = 28)]
						Mortal28(::core::primitive::u8),
						#[codec(index = 29)]
						Mortal29(::core::primitive::u8),
						#[codec(index = 30)]
						Mortal30(::core::primitive::u8),
						#[codec(index = 31)]
						Mortal31(::core::primitive::u8),
						#[codec(index = 32)]
						Mortal32(::core::primitive::u8),
						#[codec(index = 33)]
						Mortal33(::core::primitive::u8),
						#[codec(index = 34)]
						Mortal34(::core::primitive::u8),
						#[codec(index = 35)]
						Mortal35(::core::primitive::u8),
						#[codec(index = 36)]
						Mortal36(::core::primitive::u8),
						#[codec(index = 37)]
						Mortal37(::core::primitive::u8),
						#[codec(index = 38)]
						Mortal38(::core::primitive::u8),
						#[codec(index = 39)]
						Mortal39(::core::primitive::u8),
						#[codec(index = 40)]
						Mortal40(::core::primitive::u8),
						#[codec(index = 41)]
						Mortal41(::core::primitive::u8),
						#[codec(index = 42)]
						Mortal42(::core::primitive::u8),
						#[codec(index = 43)]
						Mortal43(::core::primitive::u8),
						#[codec(index = 44)]
						Mortal44(::core::primitive::u8),
						#[codec(index = 45)]
						Mortal45(::core::primitive::u8),
						#[codec(index = 46)]
						Mortal46(::core::primitive::u8),
						#[codec(index = 47)]
						Mortal47(::core::primitive::u8),
						#[codec(index = 48)]
						Mortal48(::core::primitive::u8),
						#[codec(index = 49)]
						Mortal49(::core::primitive::u8),
						#[codec(index = 50)]
						Mortal50(::core::primitive::u8),
						#[codec(index = 51)]
						Mortal51(::core::primitive::u8),
						#[codec(index = 52)]
						Mortal52(::core::primitive::u8),
						#[codec(index = 53)]
						Mortal53(::core::primitive::u8),
						#[codec(index = 54)]
						Mortal54(::core::primitive::u8),
						#[codec(index = 55)]
						Mortal55(::core::primitive::u8),
						#[codec(index = 56)]
						Mortal56(::core::primitive::u8),
						#[codec(index = 57)]
						Mortal57(::core::primitive::u8),
						#[codec(index = 58)]
						Mortal58(::core::primitive::u8),
						#[codec(index = 59)]
						Mortal59(::core::primitive::u8),
						#[codec(index = 60)]
						Mortal60(::core::primitive::u8),
						#[codec(index = 61)]
						Mortal61(::core::primitive::u8),
						#[codec(index = 62)]
						Mortal62(::core::primitive::u8),
						#[codec(index = 63)]
						Mortal63(::core::primitive::u8),
						#[codec(index = 64)]
						Mortal64(::core::primitive::u8),
						#[codec(index = 65)]
						Mortal65(::core::primitive::u8),
						#[codec(index = 66)]
						Mortal66(::core::primitive::u8),
						#[codec(index = 67)]
						Mortal67(::core::primitive::u8),
						#[codec(index = 68)]
						Mortal68(::core::primitive::u8),
						#[codec(index = 69)]
						Mortal69(::core::primitive::u8),
						#[codec(index = 70)]
						Mortal70(::core::primitive::u8),
						#[codec(index = 71)]
						Mortal71(::core::primitive::u8),
						#[codec(index = 72)]
						Mortal72(::core::primitive::u8),
						#[codec(index = 73)]
						Mortal73(::core::primitive::u8),
						#[codec(index = 74)]
						Mortal74(::core::primitive::u8),
						#[codec(index = 75)]
						Mortal75(::core::primitive::u8),
						#[codec(index = 76)]
						Mortal76(::core::primitive::u8),
						#[codec(index = 77)]
						Mortal77(::core::primitive::u8),
						#[codec(index = 78)]
						Mortal78(::core::primitive::u8),
						#[codec(index = 79)]
						Mortal79(::core::primitive::u8),
						#[codec(index = 80)]
						Mortal80(::core::primitive::u8),
						#[codec(index = 81)]
						Mortal81(::core::primitive::u8),
						#[codec(index = 82)]
						Mortal82(::core::primitive::u8),
						#[codec(index = 83)]
						Mortal83(::core::primitive::u8),
						#[codec(index = 84)]
						Mortal84(::core::primitive::u8),
						#[codec(index = 85)]
						Mortal85(::core::primitive::u8),
						#[codec(index = 86)]
						Mortal86(::core::primitive::u8),
						#[codec(index = 87)]
						Mortal87(::core::primitive::u8),
						#[codec(index = 88)]
						Mortal88(::core::primitive::u8),
						#[codec(index = 89)]
						Mortal89(::core::primitive::u8),
						#[codec(index = 90)]
						Mortal90(::core::primitive::u8),
						#[codec(index = 91)]
						Mortal91(::core::primitive::u8),
						#[codec(index = 92)]
						Mortal92(::core::primitive::u8),
						#[codec(index = 93)]
						Mortal93(::core::primitive::u8),
						#[codec(index = 94)]
						Mortal94(::core::primitive::u8),
						#[codec(index = 95)]
						Mortal95(::core::primitive::u8),
						#[codec(index = 96)]
						Mortal96(::core::primitive::u8),
						#[codec(index = 97)]
						Mortal97(::core::primitive::u8),
						#[codec(index = 98)]
						Mortal98(::core::primitive::u8),
						#[codec(index = 99)]
						Mortal99(::core::primitive::u8),
						#[codec(index = 100)]
						Mortal100(::core::primitive::u8),
						#[codec(index = 101)]
						Mortal101(::core::primitive::u8),
						#[codec(index = 102)]
						Mortal102(::core::primitive::u8),
						#[codec(index = 103)]
						Mortal103(::core::primitive::u8),
						#[codec(index = 104)]
						Mortal104(::core::primitive::u8),
						#[codec(index = 105)]
						Mortal105(::core::primitive::u8),
						#[codec(index = 106)]
						Mortal106(::core::primitive::u8),
						#[codec(index = 107)]
						Mortal107(::core::primitive::u8),
						#[codec(index = 108)]
						Mortal108(::core::primitive::u8),
						#[codec(index = 109)]
						Mortal109(::core::primitive::u8),
						#[codec(index = 110)]
						Mortal110(::core::primitive::u8),
						#[codec(index = 111)]
						Mortal111(::core::primitive::u8),
						#[codec(index = 112)]
						Mortal112(::core::primitive::u8),
						#[codec(index = 113)]
						Mortal113(::core::primitive::u8),
						#[codec(index = 114)]
						Mortal114(::core::primitive::u8),
						#[codec(index = 115)]
						Mortal115(::core::primitive::u8),
						#[codec(index = 116)]
						Mortal116(::core::primitive::u8),
						#[codec(index = 117)]
						Mortal117(::core::primitive::u8),
						#[codec(index = 118)]
						Mortal118(::core::primitive::u8),
						#[codec(index = 119)]
						Mortal119(::core::primitive::u8),
						#[codec(index = 120)]
						Mortal120(::core::primitive::u8),
						#[codec(index = 121)]
						Mortal121(::core::primitive::u8),
						#[codec(index = 122)]
						Mortal122(::core::primitive::u8),
						#[codec(index = 123)]
						Mortal123(::core::primitive::u8),
						#[codec(index = 124)]
						Mortal124(::core::primitive::u8),
						#[codec(index = 125)]
						Mortal125(::core::primitive::u8),
						#[codec(index = 126)]
						Mortal126(::core::primitive::u8),
						#[codec(index = 127)]
						Mortal127(::core::primitive::u8),
						#[codec(index = 128)]
						Mortal128(::core::primitive::u8),
						#[codec(index = 129)]
						Mortal129(::core::primitive::u8),
						#[codec(index = 130)]
						Mortal130(::core::primitive::u8),
						#[codec(index = 131)]
						Mortal131(::core::primitive::u8),
						#[codec(index = 132)]
						Mortal132(::core::primitive::u8),
						#[codec(index = 133)]
						Mortal133(::core::primitive::u8),
						#[codec(index = 134)]
						Mortal134(::core::primitive::u8),
						#[codec(index = 135)]
						Mortal135(::core::primitive::u8),
						#[codec(index = 136)]
						Mortal136(::core::primitive::u8),
						#[codec(index = 137)]
						Mortal137(::core::primitive::u8),
						#[codec(index = 138)]
						Mortal138(::core::primitive::u8),
						#[codec(index = 139)]
						Mortal139(::core::primitive::u8),
						#[codec(index = 140)]
						Mortal140(::core::primitive::u8),
						#[codec(index = 141)]
						Mortal141(::core::primitive::u8),
						#[codec(index = 142)]
						Mortal142(::core::primitive::u8),
						#[codec(index = 143)]
						Mortal143(::core::primitive::u8),
						#[codec(index = 144)]
						Mortal144(::core::primitive::u8),
						#[codec(index = 145)]
						Mortal145(::core::primitive::u8),
						#[codec(index = 146)]
						Mortal146(::core::primitive::u8),
						#[codec(index = 147)]
						Mortal147(::core::primitive::u8),
						#[codec(index = 148)]
						Mortal148(::core::primitive::u8),
						#[codec(index = 149)]
						Mortal149(::core::primitive::u8),
						#[codec(index = 150)]
						Mortal150(::core::primitive::u8),
						#[codec(index = 151)]
						Mortal151(::core::primitive::u8),
						#[codec(index = 152)]
						Mortal152(::core::primitive::u8),
						#[codec(index = 153)]
						Mortal153(::core::primitive::u8),
						#[codec(index = 154)]
						Mortal154(::core::primitive::u8),
						#[codec(index = 155)]
						Mortal155(::core::primitive::u8),
						#[codec(index = 156)]
						Mortal156(::core::primitive::u8),
						#[codec(index = 157)]
						Mortal157(::core::primitive::u8),
						#[codec(index = 158)]
						Mortal158(::core::primitive::u8),
						#[codec(index = 159)]
						Mortal159(::core::primitive::u8),
						#[codec(index = 160)]
						Mortal160(::core::primitive::u8),
						#[codec(index = 161)]
						Mortal161(::core::primitive::u8),
						#[codec(index = 162)]
						Mortal162(::core::primitive::u8),
						#[codec(index = 163)]
						Mortal163(::core::primitive::u8),
						#[codec(index = 164)]
						Mortal164(::core::primitive::u8),
						#[codec(index = 165)]
						Mortal165(::core::primitive::u8),
						#[codec(index = 166)]
						Mortal166(::core::primitive::u8),
						#[codec(index = 167)]
						Mortal167(::core::primitive::u8),
						#[codec(index = 168)]
						Mortal168(::core::primitive::u8),
						#[codec(index = 169)]
						Mortal169(::core::primitive::u8),
						#[codec(index = 170)]
						Mortal170(::core::primitive::u8),
						#[codec(index = 171)]
						Mortal171(::core::primitive::u8),
						#[codec(index = 172)]
						Mortal172(::core::primitive::u8),
						#[codec(index = 173)]
						Mortal173(::core::primitive::u8),
						#[codec(index = 174)]
						Mortal174(::core::primitive::u8),
						#[codec(index = 175)]
						Mortal175(::core::primitive::u8),
						#[codec(index = 176)]
						Mortal176(::core::primitive::u8),
						#[codec(index = 177)]
						Mortal177(::core::primitive::u8),
						#[codec(index = 178)]
						Mortal178(::core::primitive::u8),
						#[codec(index = 179)]
						Mortal179(::core::primitive::u8),
						#[codec(index = 180)]
						Mortal180(::core::primitive::u8),
						#[codec(index = 181)]
						Mortal181(::core::primitive::u8),
						#[codec(index = 182)]
						Mortal182(::core::primitive::u8),
						#[codec(index = 183)]
						Mortal183(::core::primitive::u8),
						#[codec(index = 184)]
						Mortal184(::core::primitive::u8),
						#[codec(index = 185)]
						Mortal185(::core::primitive::u8),
						#[codec(index = 186)]
						Mortal186(::core::primitive::u8),
						#[codec(index = 187)]
						Mortal187(::core::primitive::u8),
						#[codec(index = 188)]
						Mortal188(::core::primitive::u8),
						#[codec(index = 189)]
						Mortal189(::core::primitive::u8),
						#[codec(index = 190)]
						Mortal190(::core::primitive::u8),
						#[codec(index = 191)]
						Mortal191(::core::primitive::u8),
						#[codec(index = 192)]
						Mortal192(::core::primitive::u8),
						#[codec(index = 193)]
						Mortal193(::core::primitive::u8),
						#[codec(index = 194)]
						Mortal194(::core::primitive::u8),
						#[codec(index = 195)]
						Mortal195(::core::primitive::u8),
						#[codec(index = 196)]
						Mortal196(::core::primitive::u8),
						#[codec(index = 197)]
						Mortal197(::core::primitive::u8),
						#[codec(index = 198)]
						Mortal198(::core::primitive::u8),
						#[codec(index = 199)]
						Mortal199(::core::primitive::u8),
						#[codec(index = 200)]
						Mortal200(::core::primitive::u8),
						#[codec(index = 201)]
						Mortal201(::core::primitive::u8),
						#[codec(index = 202)]
						Mortal202(::core::primitive::u8),
						#[codec(index = 203)]
						Mortal203(::core::primitive::u8),
						#[codec(index = 204)]
						Mortal204(::core::primitive::u8),
						#[codec(index = 205)]
						Mortal205(::core::primitive::u8),
						#[codec(index = 206)]
						Mortal206(::core::primitive::u8),
						#[codec(index = 207)]
						Mortal207(::core::primitive::u8),
						#[codec(index = 208)]
						Mortal208(::core::primitive::u8),
						#[codec(index = 209)]
						Mortal209(::core::primitive::u8),
						#[codec(index = 210)]
						Mortal210(::core::primitive::u8),
						#[codec(index = 211)]
						Mortal211(::core::primitive::u8),
						#[codec(index = 212)]
						Mortal212(::core::primitive::u8),
						#[codec(index = 213)]
						Mortal213(::core::primitive::u8),
						#[codec(index = 214)]
						Mortal214(::core::primitive::u8),
						#[codec(index = 215)]
						Mortal215(::core::primitive::u8),
						#[codec(index = 216)]
						Mortal216(::core::primitive::u8),
						#[codec(index = 217)]
						Mortal217(::core::primitive::u8),
						#[codec(index = 218)]
						Mortal218(::core::primitive::u8),
						#[codec(index = 219)]
						Mortal219(::core::primitive::u8),
						#[codec(index = 220)]
						Mortal220(::core::primitive::u8),
						#[codec(index = 221)]
						Mortal221(::core::primitive::u8),
						#[codec(index = 222)]
						Mortal222(::core::primitive::u8),
						#[codec(index = 223)]
						Mortal223(::core::primitive::u8),
						#[codec(index = 224)]
						Mortal224(::core::primitive::u8),
						#[codec(index = 225)]
						Mortal225(::core::primitive::u8),
						#[codec(index = 226)]
						Mortal226(::core::primitive::u8),
						#[codec(index = 227)]
						Mortal227(::core::primitive::u8),
						#[codec(index = 228)]
						Mortal228(::core::primitive::u8),
						#[codec(index = 229)]
						Mortal229(::core::primitive::u8),
						#[codec(index = 230)]
						Mortal230(::core::primitive::u8),
						#[codec(index = 231)]
						Mortal231(::core::primitive::u8),
						#[codec(index = 232)]
						Mortal232(::core::primitive::u8),
						#[codec(index = 233)]
						Mortal233(::core::primitive::u8),
						#[codec(index = 234)]
						Mortal234(::core::primitive::u8),
						#[codec(index = 235)]
						Mortal235(::core::primitive::u8),
						#[codec(index = 236)]
						Mortal236(::core::primitive::u8),
						#[codec(index = 237)]
						Mortal237(::core::primitive::u8),
						#[codec(index = 238)]
						Mortal238(::core::primitive::u8),
						#[codec(index = 239)]
						Mortal239(::core::primitive::u8),
						#[codec(index = 240)]
						Mortal240(::core::primitive::u8),
						#[codec(index = 241)]
						Mortal241(::core::primitive::u8),
						#[codec(index = 242)]
						Mortal242(::core::primitive::u8),
						#[codec(index = 243)]
						Mortal243(::core::primitive::u8),
						#[codec(index = 244)]
						Mortal244(::core::primitive::u8),
						#[codec(index = 245)]
						Mortal245(::core::primitive::u8),
						#[codec(index = 246)]
						Mortal246(::core::primitive::u8),
						#[codec(index = 247)]
						Mortal247(::core::primitive::u8),
						#[codec(index = 248)]
						Mortal248(::core::primitive::u8),
						#[codec(index = 249)]
						Mortal249(::core::primitive::u8),
						#[codec(index = 250)]
						Mortal250(::core::primitive::u8),
						#[codec(index = 251)]
						Mortal251(::core::primitive::u8),
						#[codec(index = 252)]
						Mortal252(::core::primitive::u8),
						#[codec(index = 253)]
						Mortal253(::core::primitive::u8),
						#[codec(index = 254)]
						Mortal254(::core::primitive::u8),
						#[codec(index = 255)]
						Mortal255(::core::primitive::u8),
					}
				}
			}
			pub mod multiaddress {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum MultiAddress<_0, _1> {
					#[codec(index = 0)]
					Id(_0),
					#[codec(index = 1)]
					Index(#[codec(compact)] _1),
					#[codec(index = 2)]
					Raw(::std::vec::Vec<::core::primitive::u8>),
					#[codec(index = 3)]
					Address32([::core::primitive::u8; 32usize]),
					#[codec(index = 4)]
					Address20([::core::primitive::u8; 20usize]),
				}
			}
			pub mod traits {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct BlakeTwo256;
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum DispatchError {
				#[codec(index = 0)]
				Other,
				#[codec(index = 1)]
				CannotLookup,
				#[codec(index = 2)]
				BadOrigin,
				#[codec(index = 3)]
				Module(runtime_types::sp_runtime::ModuleError),
				#[codec(index = 4)]
				ConsumerRemaining,
				#[codec(index = 5)]
				NoProviders,
				#[codec(index = 6)]
				TooManyConsumers,
				#[codec(index = 7)]
				Token(runtime_types::sp_runtime::TokenError),
				#[codec(index = 8)]
				Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
				#[codec(index = 9)]
				Transactional(runtime_types::sp_runtime::TransactionalError),
				#[codec(index = 10)]
				Exhausted,
				#[codec(index = 11)]
				Corruption,
				#[codec(index = 12)]
				Unavailable,
				#[codec(index = 13)]
				RootNotAllowed,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ModuleError {
				pub index: ::core::primitive::u8,
				pub error: [::core::primitive::u8; 4usize],
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum MultiSignature {
				#[codec(index = 0)]
				Ed25519(runtime_types::sp_core::ed25519::Signature),
				#[codec(index = 1)]
				Sr25519(runtime_types::sp_core::sr25519::Signature),
				#[codec(index = 2)]
				Ecdsa(runtime_types::sp_core::ecdsa::Signature),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum TokenError {
				#[codec(index = 0)]
				FundsUnavailable,
				#[codec(index = 1)]
				OnlyProvider,
				#[codec(index = 2)]
				BelowMinimum,
				#[codec(index = 3)]
				CannotCreate,
				#[codec(index = 4)]
				UnknownAsset,
				#[codec(index = 5)]
				Frozen,
				#[codec(index = 6)]
				Unsupported,
				#[codec(index = 7)]
				CannotCreateHold,
				#[codec(index = 8)]
				NotExpendable,
				#[codec(index = 9)]
				Blocked,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum TransactionalError {
				#[codec(index = 0)]
				LimitReached,
				#[codec(index = 1)]
				NoLayer,
			}
		}
		pub mod sp_session {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct MembershipProof {
				pub session: ::core::primitive::u32,
				pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				pub validator_count: ::core::primitive::u32,
			}
		}
		pub mod sp_staking {
			use super::runtime_types;
			pub mod offence {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct OffenceDetails<_0, _1> {
					pub offender: _1,
					pub reporters: ::std::vec::Vec<_0>,
				}
			}
		}
		pub mod sp_version {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct RuntimeVersion {
				pub spec_name: ::std::string::String,
				pub impl_name: ::std::string::String,
				pub authoring_version: ::core::primitive::u32,
				pub spec_version: ::core::primitive::u32,
				pub impl_version: ::core::primitive::u32,
				pub apis:
					::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
				pub transaction_version: ::core::primitive::u32,
				pub state_version: ::core::primitive::u8,
			}
		}
		pub mod sp_weights {
			use super::runtime_types;
			pub mod weight_v2 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Weight {
					#[codec(compact)]
					pub ref_time: ::core::primitive::u64,
					#[codec(compact)]
					pub proof_size: ::core::primitive::u64,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct RuntimeDbWeight {
				pub read: ::core::primitive::u64,
				pub write: ::core::primitive::u64,
			}
		}
	}
	#[doc = r" The default error type returned when there is a runtime issue,"]
	#[doc = r" exposed here for ease of use."]
	pub type DispatchError = runtime_types::sp_runtime::DispatchError;
	pub fn constants() -> ConstantsApi { ConstantsApi }
	pub fn storage() -> StorageApi { StorageApi }
	pub fn tx() -> TransactionApi { TransactionApi }
	pub struct ConstantsApi;
	impl ConstantsApi {
		pub fn system(&self) -> system::constants::ConstantsApi { system::constants::ConstantsApi }

		pub fn utility(&self) -> utility::constants::ConstantsApi {
			utility::constants::ConstantsApi
		}

		pub fn babe(&self) -> babe::constants::ConstantsApi { babe::constants::ConstantsApi }

		pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
			timestamp::constants::ConstantsApi
		}

		pub fn indices(&self) -> indices::constants::ConstantsApi {
			indices::constants::ConstantsApi
		}

		pub fn balances(&self) -> balances::constants::ConstantsApi {
			balances::constants::ConstantsApi
		}

		pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
			transaction_payment::constants::ConstantsApi
		}

		pub fn election_provider_multi_phase(
			&self,
		) -> election_provider_multi_phase::constants::ConstantsApi {
			election_provider_multi_phase::constants::ConstantsApi
		}

		pub fn staking(&self) -> staking::constants::ConstantsApi {
			staking::constants::ConstantsApi
		}

		pub fn technical_committee(&self) -> technical_committee::constants::ConstantsApi {
			technical_committee::constants::ConstantsApi
		}

		pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
			grandpa::constants::ConstantsApi
		}

		pub fn treasury(&self) -> treasury::constants::ConstantsApi {
			treasury::constants::ConstantsApi
		}

		pub fn im_online(&self) -> im_online::constants::ConstantsApi {
			im_online::constants::ConstantsApi
		}

		pub fn scheduler(&self) -> scheduler::constants::ConstantsApi {
			scheduler::constants::ConstantsApi
		}

		pub fn bounties(&self) -> bounties::constants::ConstantsApi {
			bounties::constants::ConstantsApi
		}

		pub fn tips(&self) -> tips::constants::ConstantsApi { tips::constants::ConstantsApi }

		pub fn data_availability(&self) -> data_availability::constants::ConstantsApi {
			data_availability::constants::ConstantsApi
		}

		pub fn nomad_home(&self) -> nomad_home::constants::ConstantsApi {
			nomad_home::constants::ConstantsApi
		}

		pub fn nomad_da_bridge(&self) -> nomad_da_bridge::constants::ConstantsApi {
			nomad_da_bridge::constants::ConstantsApi
		}

		pub fn multisig(&self) -> multisig::constants::ConstantsApi {
			multisig::constants::ConstantsApi
		}

		pub fn voter_list(&self) -> voter_list::constants::ConstantsApi {
			voter_list::constants::ConstantsApi
		}

		pub fn nomination_pools(&self) -> nomination_pools::constants::ConstantsApi {
			nomination_pools::constants::ConstantsApi
		}

		pub fn identity(&self) -> identity::constants::ConstantsApi {
			identity::constants::ConstantsApi
		}
	}
	pub struct StorageApi;
	impl StorageApi {
		pub fn system(&self) -> system::storage::StorageApi { system::storage::StorageApi }

		pub fn babe(&self) -> babe::storage::StorageApi { babe::storage::StorageApi }

		pub fn timestamp(&self) -> timestamp::storage::StorageApi { timestamp::storage::StorageApi }

		pub fn authorship(&self) -> authorship::storage::StorageApi {
			authorship::storage::StorageApi
		}

		pub fn indices(&self) -> indices::storage::StorageApi { indices::storage::StorageApi }

		pub fn balances(&self) -> balances::storage::StorageApi { balances::storage::StorageApi }

		pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
			transaction_payment::storage::StorageApi
		}

		pub fn election_provider_multi_phase(
			&self,
		) -> election_provider_multi_phase::storage::StorageApi {
			election_provider_multi_phase::storage::StorageApi
		}

		pub fn staking(&self) -> staking::storage::StorageApi { staking::storage::StorageApi }

		pub fn session(&self) -> session::storage::StorageApi { session::storage::StorageApi }

		pub fn technical_committee(&self) -> technical_committee::storage::StorageApi {
			technical_committee::storage::StorageApi
		}

		pub fn technical_membership(&self) -> technical_membership::storage::StorageApi {
			technical_membership::storage::StorageApi
		}

		pub fn grandpa(&self) -> grandpa::storage::StorageApi { grandpa::storage::StorageApi }

		pub fn treasury(&self) -> treasury::storage::StorageApi { treasury::storage::StorageApi }

		pub fn sudo(&self) -> sudo::storage::StorageApi { sudo::storage::StorageApi }

		pub fn im_online(&self) -> im_online::storage::StorageApi { im_online::storage::StorageApi }

		pub fn authority_discovery(&self) -> authority_discovery::storage::StorageApi {
			authority_discovery::storage::StorageApi
		}

		pub fn offences(&self) -> offences::storage::StorageApi { offences::storage::StorageApi }

		pub fn historical(&self) -> historical::storage::StorageApi {
			historical::storage::StorageApi
		}

		pub fn scheduler(&self) -> scheduler::storage::StorageApi { scheduler::storage::StorageApi }

		pub fn bounties(&self) -> bounties::storage::StorageApi { bounties::storage::StorageApi }

		pub fn tips(&self) -> tips::storage::StorageApi { tips::storage::StorageApi }

		pub fn mmr(&self) -> mmr::storage::StorageApi { mmr::storage::StorageApi }

		pub fn data_availability(&self) -> data_availability::storage::StorageApi {
			data_availability::storage::StorageApi
		}

		pub fn nomad_updater_manager(&self) -> nomad_updater_manager::storage::StorageApi {
			nomad_updater_manager::storage::StorageApi
		}

		pub fn nomad_home(&self) -> nomad_home::storage::StorageApi {
			nomad_home::storage::StorageApi
		}

		pub fn preimage(&self) -> preimage::storage::StorageApi { preimage::storage::StorageApi }

		pub fn multisig(&self) -> multisig::storage::StorageApi { multisig::storage::StorageApi }

		pub fn voter_list(&self) -> voter_list::storage::StorageApi {
			voter_list::storage::StorageApi
		}

		pub fn nomination_pools(&self) -> nomination_pools::storage::StorageApi {
			nomination_pools::storage::StorageApi
		}

		pub fn identity(&self) -> identity::storage::StorageApi { identity::storage::StorageApi }
	}
	pub struct TransactionApi;
	impl TransactionApi {
		pub fn system(&self) -> system::calls::TransactionApi { system::calls::TransactionApi }

		pub fn utility(&self) -> utility::calls::TransactionApi { utility::calls::TransactionApi }

		pub fn babe(&self) -> babe::calls::TransactionApi { babe::calls::TransactionApi }

		pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
			timestamp::calls::TransactionApi
		}

		pub fn indices(&self) -> indices::calls::TransactionApi { indices::calls::TransactionApi }

		pub fn balances(&self) -> balances::calls::TransactionApi {
			balances::calls::TransactionApi
		}

		pub fn election_provider_multi_phase(
			&self,
		) -> election_provider_multi_phase::calls::TransactionApi {
			election_provider_multi_phase::calls::TransactionApi
		}

		pub fn staking(&self) -> staking::calls::TransactionApi { staking::calls::TransactionApi }

		pub fn session(&self) -> session::calls::TransactionApi { session::calls::TransactionApi }

		pub fn technical_committee(&self) -> technical_committee::calls::TransactionApi {
			technical_committee::calls::TransactionApi
		}

		pub fn technical_membership(&self) -> technical_membership::calls::TransactionApi {
			technical_membership::calls::TransactionApi
		}

		pub fn grandpa(&self) -> grandpa::calls::TransactionApi { grandpa::calls::TransactionApi }

		pub fn treasury(&self) -> treasury::calls::TransactionApi {
			treasury::calls::TransactionApi
		}

		pub fn sudo(&self) -> sudo::calls::TransactionApi { sudo::calls::TransactionApi }

		pub fn im_online(&self) -> im_online::calls::TransactionApi {
			im_online::calls::TransactionApi
		}

		pub fn scheduler(&self) -> scheduler::calls::TransactionApi {
			scheduler::calls::TransactionApi
		}

		pub fn bounties(&self) -> bounties::calls::TransactionApi {
			bounties::calls::TransactionApi
		}

		pub fn tips(&self) -> tips::calls::TransactionApi { tips::calls::TransactionApi }

		pub fn data_availability(&self) -> data_availability::calls::TransactionApi {
			data_availability::calls::TransactionApi
		}

		pub fn nomad_updater_manager(&self) -> nomad_updater_manager::calls::TransactionApi {
			nomad_updater_manager::calls::TransactionApi
		}

		pub fn nomad_home(&self) -> nomad_home::calls::TransactionApi {
			nomad_home::calls::TransactionApi
		}

		pub fn nomad_da_bridge(&self) -> nomad_da_bridge::calls::TransactionApi {
			nomad_da_bridge::calls::TransactionApi
		}

		pub fn preimage(&self) -> preimage::calls::TransactionApi {
			preimage::calls::TransactionApi
		}

		pub fn multisig(&self) -> multisig::calls::TransactionApi {
			multisig::calls::TransactionApi
		}

		pub fn voter_list(&self) -> voter_list::calls::TransactionApi {
			voter_list::calls::TransactionApi
		}

		pub fn nomination_pools(&self) -> nomination_pools::calls::TransactionApi {
			nomination_pools::calls::TransactionApi
		}

		pub fn identity(&self) -> identity::calls::TransactionApi {
			identity::calls::TransactionApi
		}

		pub fn mandate(&self) -> mandate::calls::TransactionApi { mandate::calls::TransactionApi }
	}
	#[doc = r" check whether the Client you are using is aligned with the statically generated codegen."]
	pub fn validate_codegen<T: ::subxt::Config, C: ::subxt::client::OfflineClientT<T>>(
		client: &C,
	) -> Result<(), ::subxt::error::MetadataError> {
		let runtime_metadata_hash = client.metadata().metadata_hash(&PALLETS);
		if runtime_metadata_hash
			!= [
				120u8, 135u8, 49u8, 159u8, 52u8, 242u8, 139u8, 237u8, 51u8, 115u8, 243u8, 81u8,
				86u8, 224u8, 234u8, 112u8, 182u8, 101u8, 55u8, 118u8, 8u8, 103u8, 163u8, 31u8,
				27u8, 164u8, 125u8, 113u8, 81u8, 172u8, 30u8, 109u8,
			] {
			Err(::subxt::error::MetadataError::IncompatibleMetadata)
		} else {
			Ok(())
		}
	}
}
