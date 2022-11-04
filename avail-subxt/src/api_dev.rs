#[allow(clippy::all)]
#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
	use super::api as root_mod;
	pub static PALLETS: [&str; 32usize] = [
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
		"Democracy",
		"Council",
		"TechnicalCommittee",
		"Elections",
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
		"BagsList",
		"DataAvailability",
		"UpdaterManager",
		"NomadHome",
		"DABridge",
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
		#[codec(index = 9)]
		ElectionProviderMultiPhase(election_provider_multi_phase::Event),
		#[codec(index = 10)]
		Staking(staking::Event),
		#[codec(index = 11)]
		Session(session::Event),
		#[codec(index = 12)]
		Democracy(democracy::Event),
		#[codec(index = 13)]
		Council(council::Event),
		#[codec(index = 14)]
		TechnicalCommittee(technical_committee::Event),
		#[codec(index = 15)]
		Elections(elections::Event),
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
		#[codec(index = 28)]
		BagsList(bags_list::Event),
		#[codec(index = 29)]
		DataAvailability(data_availability::Event),
		#[codec(index = 30)]
		UpdaterManager(updater_manager::Event),
		#[codec(index = 31)]
		NomadHome(nomad_home::Event),
		#[codec(index = 32)]
		DABridge(da_bridge::Event),
	}
	pub mod system {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
			pub struct FillBlock {
				pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
			}
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
				#[doc = "A dispatch that will fill the block weight up to the given ratio."]
				pub fn fill_block(
					&self,
					ratio: runtime_types::sp_arithmetic::per_things::Perbill,
				) -> ::subxt::tx::StaticTxPayload<FillBlock> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"fill_block",
						FillBlock { ratio },
						[
							48u8, 18u8, 205u8, 90u8, 222u8, 4u8, 20u8, 251u8, 173u8, 76u8, 167u8,
							4u8, 83u8, 203u8, 160u8, 89u8, 132u8, 218u8, 191u8, 145u8, 130u8,
							245u8, 177u8, 201u8, 169u8, 129u8, 173u8, 105u8, 88u8, 45u8, 136u8,
							191u8,
						],
					)
				}

				#[doc = "Make some on-chain remark."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`"]
				#[doc = "# </weight>"]
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

				#[doc = "Set the number of pages in the WebAssembly environment's heap."]
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

				#[doc = "Set the new runtime code."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
				#[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
				#[doc = "  expensive)."]
				#[doc = "- 1 storage write (codec `O(C)`)."]
				#[doc = "- 1 digest item."]
				#[doc = "- 1 event."]
				#[doc = "The weight of this function is dependent on the runtime, but generally this is very expensive."]
				#[doc = "We will treat this as a full block."]
				#[doc = "# </weight>"]
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

				#[doc = "Set the new runtime code without doing any checks of the given `code`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(C)` where `C` length of `code`"]
				#[doc = "- 1 storage write (codec `O(C)`)."]
				#[doc = "- 1 digest item."]
				#[doc = "- 1 event."]
				#[doc = "The weight of this function is dependent on the runtime. We will treat this as a full block."]
				#[doc = "# </weight>"]
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

				#[doc = "Set some items of storage."]
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

				#[doc = "Kill some items from storage."]
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

				#[doc = "Kill all storage items with a key that starts with the given prefix."]
				#[doc = ""]
				#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
				#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
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

				#[doc = "Make some on-chain remark and emit event."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(b)` where b is the length of the remark."]
				#[doc = "- 1 event."]
				#[doc = "# </weight>"]
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
			#[doc = "An extrinsic completed successfully. \\[info\\]"]
			pub struct ExtrinsicSuccess(pub runtime_types::frame_support::weights::DispatchInfo);
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
			#[doc = "An extrinsic failed. \\[error, info\\]"]
			pub struct ExtrinsicFailed(
				pub runtime_types::sp_runtime::DispatchError,
				pub runtime_types::frame_support::weights::DispatchInfo,
			);
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
			#[doc = "A new \\[account\\] was created."]
			pub struct NewAccount(pub ::subxt::ext::sp_core::crypto::AccountId32);
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
			#[doc = "An \\[account\\] was reaped."]
			pub struct KilledAccount(pub ::subxt::ext::sp_core::crypto::AccountId32);
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
			#[doc = "On on-chain remark happened. \\[origin, remark_hash\\]"]
			pub struct Remarked(
				pub ::subxt::ext::sp_core::crypto::AccountId32,
				pub ::subxt::ext::sp_core::H256,
			);
			impl ::subxt::events::StaticEvent for Remarked {
				const EVENT: &'static str = "Remarked";
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
							runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
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
							176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8,
							114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8,
							42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8,
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
							runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
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
							176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8,
							114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8,
							42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8,
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
						runtime_types::frame_support::weights::PerDispatchClass<
							::core::primitive::u64,
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
							91u8, 211u8, 177u8, 36u8, 147u8, 249u8, 55u8, 164u8, 48u8, 49u8, 55u8,
							11u8, 121u8, 193u8, 103u8, 69u8, 38u8, 142u8, 148u8, 36u8, 137u8, 41u8,
							115u8, 195u8, 31u8, 174u8, 163u8, 125u8, 69u8, 5u8, 94u8, 79u8,
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
					::subxt::metadata::DecodeStaticType<
						runtime_types::da_primitives::asdr::AppExtrinsic,
					>,
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
							119u8, 13u8, 145u8, 135u8, 63u8, 188u8, 118u8, 55u8, 255u8, 212u8,
							39u8, 232u8, 231u8, 180u8, 129u8, 247u8, 65u8, 190u8, 22u8, 81u8,
							225u8, 135u8, 104u8, 238u8, 208u8, 111u8, 68u8, 104u8, 35u8, 219u8,
							156u8, 155u8,
						],
					)
				}

				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::da_primitives::asdr::AppExtrinsic,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ExtrinsicData",
						Vec::new(),
						[
							119u8, 13u8, 145u8, 135u8, 63u8, 188u8, 118u8, 55u8, 255u8, 212u8,
							39u8, 232u8, 231u8, 180u8, 129u8, 247u8, 65u8, 190u8, 22u8, 81u8,
							225u8, 135u8, 104u8, 238u8, 208u8, 111u8, 68u8, 104u8, 35u8, 219u8,
							156u8, 155u8,
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
				#[doc = " NOTE: This storage item is explicitly unbounded since it is never intended to be read"]
				#[doc = " from within the runtime."]
				pub fn events(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::frame_system::EventRecord<
								runtime_types::da_runtime::Event,
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
							192u8, 110u8, 236u8, 62u8, 86u8, 15u8, 78u8, 228u8, 172u8, 116u8,
							121u8, 129u8, 109u8, 54u8, 16u8, 3u8, 181u8, 137u8, 42u8, 89u8, 170u8,
							177u8, 94u8, 2u8, 67u8, 99u8, 31u8, 95u8, 175u8, 50u8, 27u8, 2u8,
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
				#[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
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
				#[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
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
							61u8, 67u8, 254u8, 36u8, 130u8, 136u8, 2u8, 229u8, 41u8, 248u8, 72u8,
							49u8, 111u8, 31u8, 211u8, 59u8, 4u8, 82u8, 119u8, 131u8, 217u8, 158u8,
							138u8, 241u8, 103u8, 99u8, 129u8, 82u8, 173u8, 126u8, 196u8, 195u8,
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
						153u8, 164u8, 86u8, 79u8, 97u8, 114u8, 248u8, 181u8, 179u8, 186u8, 214u8,
						124u8, 215u8, 96u8, 116u8, 109u8, 215u8, 182u8, 61u8, 10u8, 77u8, 74u8,
						29u8, 125u8, 131u8, 111u8, 249u8, 208u8, 233u8, 170u8, 11u8, 14u8,
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
						230u8, 122u8, 219u8, 215u8, 152u8, 88u8, 232u8, 109u8, 180u8, 145u8, 116u8,
						158u8, 216u8, 21u8, 118u8, 159u8, 251u8, 21u8, 28u8, 72u8, 170u8, 152u8,
						207u8, 17u8, 153u8, 57u8, 9u8, 151u8, 46u8, 182u8, 160u8, 65u8,
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
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_support::weights::RuntimeDbWeight,
					>,
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
						131u8, 233u8, 3u8, 141u8, 149u8, 146u8, 0u8, 55u8, 80u8, 231u8, 9u8, 247u8,
						242u8, 126u8, 200u8, 190u8, 193u8, 77u8, 190u8, 153u8, 139u8, 76u8, 2u8,
						152u8, 160u8, 250u8, 183u8, 26u8, 6u8, 174u8, 130u8, 105u8,
					])
				}

				#[doc = " The designated SS85 prefix of this chain."]
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
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				pub calls: ::std::vec::Vec<runtime_types::da_runtime::Call>,
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
				pub call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
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
				pub calls: ::std::vec::Vec<runtime_types::da_runtime::Call>,
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
				pub call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Send a batch of dispatch calls."]
				#[doc = ""]
				#[doc = "May be called from any origin."]
				#[doc = ""]
				#[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
				#[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
				#[doc = ""]
				#[doc = "If origin is root then call are dispatch without checking origin filter. (This includes"]
				#[doc = "bypassing `frame_system::Config::BaseCallFilter`)."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
				#[doc = "# </weight>"]
				#[doc = ""]
				#[doc = "This will return `Ok` in all circumstances. To determine the success of the batch, an"]
				#[doc = "event is deposited. If a call failed and the batch was interrupted, then the"]
				#[doc = "`BatchInterrupted` event is deposited, along with the number of successful calls made"]
				#[doc = "and the error of the failed call. If all were successful, then the `BatchCompleted`"]
				#[doc = "event is deposited."]
				pub fn batch(
					&self,
					calls: ::std::vec::Vec<runtime_types::da_runtime::Call>,
				) -> ::subxt::tx::StaticTxPayload<Batch> {
					::subxt::tx::StaticTxPayload::new("Utility", "batch", Batch { calls }, [
						115u8, 117u8, 162u8, 24u8, 13u8, 112u8, 34u8, 183u8, 187u8, 162u8, 45u8,
						46u8, 164u8, 13u8, 50u8, 60u8, 151u8, 233u8, 204u8, 36u8, 211u8, 244u8,
						54u8, 185u8, 194u8, 210u8, 57u8, 102u8, 113u8, 230u8, 22u8, 96u8,
					])
				}

				#[doc = "Send a call through an indexed pseudonym of the sender."]
				#[doc = ""]
				#[doc = "Filter from origin are passed along. The call will be dispatched with an origin which"]
				#[doc = "use the same filter as the origin of this call."]
				#[doc = ""]
				#[doc = "NOTE: If you need to ensure that any account-based filtering is not honored (i.e."]
				#[doc = "because you expect `proxy` to have been used prior in the call stack and you do not want"]
				#[doc = "the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`"]
				#[doc = "in the Multisig pallet instead."]
				#[doc = ""]
				#[doc = "NOTE: Prior to version *12, this was called `as_limited_sub`."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				pub fn as_derivative(
					&self,
					index: ::core::primitive::u16,
					call: runtime_types::da_runtime::Call,
				) -> ::subxt::tx::StaticTxPayload<AsDerivative> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"as_derivative",
						AsDerivative {
							index,
							call: ::std::boxed::Box::new(call),
						},
						[
							183u8, 32u8, 43u8, 178u8, 59u8, 70u8, 107u8, 153u8, 84u8, 73u8, 184u8,
							132u8, 193u8, 89u8, 101u8, 141u8, 37u8, 238u8, 112u8, 36u8, 122u8,
							149u8, 19u8, 239u8, 13u8, 103u8, 123u8, 91u8, 115u8, 225u8, 98u8,
							117u8,
						],
					)
				}

				#[doc = "Send a batch of dispatch calls and atomically execute them."]
				#[doc = "The whole transaction will rollback and fail if any of the calls failed."]
				#[doc = ""]
				#[doc = "May be called from any origin."]
				#[doc = ""]
				#[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
				#[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
				#[doc = ""]
				#[doc = "If origin is root then call are dispatch without checking origin filter. (This includes"]
				#[doc = "bypassing `frame_system::Config::BaseCallFilter`)."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
				#[doc = "# </weight>"]
				pub fn batch_all(
					&self,
					calls: ::std::vec::Vec<runtime_types::da_runtime::Call>,
				) -> ::subxt::tx::StaticTxPayload<BatchAll> {
					::subxt::tx::StaticTxPayload::new("Utility", "batch_all", BatchAll { calls }, [
						11u8, 123u8, 115u8, 50u8, 151u8, 84u8, 240u8, 219u8, 54u8, 38u8, 47u8,
						65u8, 113u8, 131u8, 165u8, 0u8, 76u8, 233u8, 224u8, 6u8, 53u8, 202u8,
						248u8, 53u8, 181u8, 240u8, 21u8, 240u8, 207u8, 98u8, 79u8, 193u8,
					])
				}

				#[doc = "Dispatches a function call with a provided origin."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Root_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- Limited storage reads."]
				#[doc = "- One DB write (event)."]
				#[doc = "- Weight of derivative `call` execution + T::WeightInfo::dispatch_as()."]
				#[doc = "# </weight>"]
				pub fn dispatch_as(
					&self,
					as_origin: runtime_types::da_runtime::OriginCaller,
					call: runtime_types::da_runtime::Call,
				) -> ::subxt::tx::StaticTxPayload<DispatchAs> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"dispatch_as",
						DispatchAs {
							as_origin: ::std::boxed::Box::new(as_origin),
							call: ::std::boxed::Box::new(call),
						},
						[
							51u8, 162u8, 124u8, 243u8, 152u8, 118u8, 138u8, 1u8, 155u8, 77u8,
							191u8, 138u8, 146u8, 237u8, 97u8, 139u8, 42u8, 75u8, 190u8, 228u8,
							160u8, 114u8, 16u8, 29u8, 196u8, 134u8, 67u8, 139u8, 31u8, 36u8, 43u8,
							47u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
			#[doc = "A call was dispatched. \\[result\\]"]
			pub struct DispatchedAs(
				pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			);
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
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
						runtime_types::da_primitives::header::Header<
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
						runtime_types::da_primitives::header::Header<
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
				#[doc = "Report authority equivocation/misbehavior. This method will verify"]
				#[doc = "the equivocation proof and validate the given key ownership proof"]
				#[doc = "against the extracted offender. If both are valid, the offence will"]
				#[doc = "be reported."]
				pub fn report_equivocation(
					&self,
					equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
						runtime_types::da_primitives::header::Header<
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
							208u8, 111u8, 191u8, 81u8, 157u8, 234u8, 10u8, 180u8, 203u8, 148u8,
							110u8, 31u8, 10u8, 197u8, 220u8, 146u8, 223u8, 49u8, 196u8, 217u8,
							137u8, 8u8, 40u8, 76u8, 125u8, 15u8, 135u8, 205u8, 146u8, 60u8, 3u8,
							117u8,
						],
					)
				}

				#[doc = "Report authority equivocation/misbehavior. This method will verify"]
				#[doc = "the equivocation proof and validate the given key ownership proof"]
				#[doc = "against the extracted offender. If both are valid, the offence will"]
				#[doc = "be reported."]
				#[doc = "This extrinsic must be called unsigned and it is expected that only"]
				#[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
				#[doc = "if the block author is defined it will be defined as the equivocation"]
				#[doc = "reporter."]
				pub fn report_equivocation_unsigned(
					&self,
					equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
						runtime_types::da_primitives::header::Header<
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
							150u8, 91u8, 173u8, 165u8, 128u8, 39u8, 104u8, 129u8, 157u8, 194u8,
							26u8, 152u8, 85u8, 26u8, 78u8, 245u8, 8u8, 144u8, 211u8, 252u8, 216u8,
							230u8, 16u8, 206u8, 61u8, 163u8, 41u8, 62u8, 124u8, 220u8, 37u8, 143u8,
						],
					)
				}

				#[doc = "Plan an epoch config change. The epoch config change is recorded and will be enacted on"]
				#[doc = "the next call to `enact_epoch_change`. The config will be activated one epoch after."]
				#[doc = "Multiple calls to this method will replace any existing planned config change that had"]
				#[doc = "not been enacted yet."]
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
						runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
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
						runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
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
						runtime_types::frame_support::storage::bounded_vec::BoundedVec<
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
						runtime_types::frame_support::storage::bounded_vec::BoundedVec<
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
						::core::option::Option<[::core::primitive::u8; 32usize]>,
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
							48u8, 206u8, 111u8, 118u8, 149u8, 175u8, 148u8, 53u8, 233u8, 82u8,
							220u8, 57u8, 22u8, 164u8, 116u8, 228u8, 134u8, 237u8, 129u8, 195u8,
							60u8, 169u8, 1u8, 164u8, 74u8, 177u8, 145u8, 112u8, 66u8, 198u8, 53u8,
							157u8,
						],
					)
				}

				#[doc = " This field should always be populated during block processing unless"]
				#[doc = " secondary plain slots are enabled (which don't contain a VRF output)."]
				#[doc = ""]
				#[doc = " It is set in `on_initialize`, before it will contain the value from the last block."]
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
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				#[doc = "Set the current time."]
				#[doc = ""]
				#[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
				#[doc = "phase, if this call hasn't been invoked by that time."]
				#[doc = ""]
				#[doc = "The timestamp should be greater than the previous one by the amount specified by"]
				#[doc = "`MinimumPeriod`."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Inherent`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
				#[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
				#[doc = "  `on_finalize`)"]
				#[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
				#[doc = "# </weight>"]
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
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
			pub struct SetUncles {
				pub new_uncles: ::std::vec::Vec<
					runtime_types::da_primitives::header::Header<
						::core::primitive::u32,
						runtime_types::sp_runtime::traits::BlakeTwo256,
					>,
				>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Provide a set of uncles."]
				pub fn set_uncles(
					&self,
					new_uncles: ::std::vec::Vec<
						runtime_types::da_primitives::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetUncles> {
					::subxt::tx::StaticTxPayload::new(
						"Authorship",
						"set_uncles",
						SetUncles { new_uncles },
						[
							107u8, 16u8, 191u8, 152u8, 109u8, 147u8, 123u8, 8u8, 145u8, 25u8,
							180u8, 212u8, 83u8, 30u8, 223u8, 118u8, 20u8, 114u8, 232u8, 201u8, 1u8,
							147u8, 5u8, 138u8, 163u8, 55u8, 49u8, 36u8, 193u8, 176u8, 79u8, 255u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Uncles"]
				pub fn uncles(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::pallet_authorship::UncleEntryItem<
								::core::primitive::u32,
								::subxt::ext::sp_core::H256,
								::subxt::ext::sp_core::crypto::AccountId32,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Authorship",
						"Uncles",
						vec![],
						[
							43u8, 181u8, 75u8, 158u8, 153u8, 32u8, 210u8, 36u8, 194u8, 34u8, 146u8,
							179u8, 154u8, 141u8, 75u8, 29u8, 51u8, 116u8, 94u8, 82u8, 90u8, 74u8,
							103u8, 216u8, 86u8, 27u8, 30u8, 213u8, 174u8, 80u8, 193u8, 51u8,
						],
					)
				}

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

				#[doc = " Whether uncles were already set in this block."]
				pub fn did_set_uncles(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Authorship",
						"DidSetUncles",
						vec![],
						[
							64u8, 3u8, 208u8, 187u8, 50u8, 45u8, 37u8, 88u8, 163u8, 226u8, 37u8,
							126u8, 232u8, 107u8, 156u8, 187u8, 29u8, 15u8, 53u8, 46u8, 28u8, 73u8,
							83u8, 123u8, 14u8, 244u8, 243u8, 43u8, 245u8, 143u8, 15u8, 115u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The number of blocks back we should accept uncles."]
				#[doc = " This means that we will deal with uncle-parents that are"]
				#[doc = " `UncleGenerations + 1` before `now`."]
				pub fn uncle_generations(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Authorship",
						"UncleGenerations",
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
	pub mod indices {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				pub new: ::subxt::ext::sp_core::crypto::AccountId32,
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
				pub new: ::subxt::ext::sp_core::crypto::AccountId32,
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
				#[doc = "Assign an previously unassigned index."]
				#[doc = ""]
				#[doc = "Payment: `Deposit` is reserved from the sender account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `index`: the index to be claimed. This must not be in use."]
				#[doc = ""]
				#[doc = "Emits `IndexAssigned` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`."]
				#[doc = "- One storage mutation (codec `O(1)`)."]
				#[doc = "- One reserve operation."]
				#[doc = "- One event."]
				#[doc = "-------------------"]
				#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
				#[doc = "# </weight>"]
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

				#[doc = "Assign an index already owned by the sender to another account. The balance reservation"]
				#[doc = "is effectively transferred to the new account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `index`: the index to be re-assigned. This must be owned by the sender."]
				#[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
				#[doc = ""]
				#[doc = "Emits `IndexAssigned` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`."]
				#[doc = "- One storage mutation (codec `O(1)`)."]
				#[doc = "- One transfer operation."]
				#[doc = "- One event."]
				#[doc = "-------------------"]
				#[doc = "- DB Weight:"]
				#[doc = "   - Reads: Indices Accounts, System Account (recipient)"]
				#[doc = "   - Writes: Indices Accounts, System Account (recipient)"]
				#[doc = "# </weight>"]
				pub fn transfer(
					&self,
					new: ::subxt::ext::sp_core::crypto::AccountId32,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Transfer> {
					::subxt::tx::StaticTxPayload::new(
						"Indices",
						"transfer",
						Transfer { new, index },
						[
							229u8, 48u8, 45u8, 2u8, 206u8, 24u8, 60u8, 43u8, 202u8, 99u8, 80u8,
							172u8, 62u8, 134u8, 224u8, 128u8, 107u8, 219u8, 57u8, 87u8, 144u8,
							220u8, 207u8, 79u8, 7u8, 89u8, 208u8, 75u8, 158u8, 75u8, 10u8, 113u8,
						],
					)
				}

				#[doc = "Free up an index owned by the sender."]
				#[doc = ""]
				#[doc = "Payment: Any previous deposit placed for the index is unreserved in the sender account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ and the sender must own the index."]
				#[doc = ""]
				#[doc = "- `index`: the index to be freed. This must be owned by the sender."]
				#[doc = ""]
				#[doc = "Emits `IndexFreed` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`."]
				#[doc = "- One storage mutation (codec `O(1)`)."]
				#[doc = "- One reserve operation."]
				#[doc = "- One event."]
				#[doc = "-------------------"]
				#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
				#[doc = "# </weight>"]
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

				#[doc = "Force an index to an account. This doesn't require a deposit. If the index is already"]
				#[doc = "held, then any deposit is reimbursed to its current owner."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Root_."]
				#[doc = ""]
				#[doc = "- `index`: the index to be (re-)assigned."]
				#[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
				#[doc = "- `freeze`: if set to `true`, will freeze the index so it cannot be transferred."]
				#[doc = ""]
				#[doc = "Emits `IndexAssigned` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`."]
				#[doc = "- One storage mutation (codec `O(1)`)."]
				#[doc = "- Up to one reserve operation."]
				#[doc = "- One event."]
				#[doc = "-------------------"]
				#[doc = "- DB Weight:"]
				#[doc = "   - Reads: Indices Accounts, System Account (original owner)"]
				#[doc = "   - Writes: Indices Accounts, System Account (original owner)"]
				#[doc = "# </weight>"]
				pub fn force_transfer(
					&self,
					new: ::subxt::ext::sp_core::crypto::AccountId32,
					index: ::core::primitive::u32,
					freeze: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<ForceTransfer> {
					::subxt::tx::StaticTxPayload::new(
						"Indices",
						"force_transfer",
						ForceTransfer { new, index, freeze },
						[
							2u8, 134u8, 200u8, 233u8, 224u8, 80u8, 237u8, 130u8, 28u8, 159u8,
							130u8, 223u8, 124u8, 205u8, 248u8, 70u8, 246u8, 77u8, 73u8, 193u8,
							78u8, 85u8, 58u8, 29u8, 191u8, 217u8, 252u8, 178u8, 113u8, 255u8,
							151u8, 49u8,
						],
					)
				}

				#[doc = "Freeze an index so it will always point to the sender account. This consumes the"]
				#[doc = "deposit."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ and the signing account must have a"]
				#[doc = "non-frozen account `index`."]
				#[doc = ""]
				#[doc = "- `index`: the index to be frozen in place."]
				#[doc = ""]
				#[doc = "Emits `IndexFrozen` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`."]
				#[doc = "- One storage mutation (codec `O(1)`)."]
				#[doc = "- Up to one slash operation."]
				#[doc = "- One event."]
				#[doc = "-------------------"]
				#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
				#[doc = "# </weight>"]
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
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
			pub struct SetBalance {
				pub who: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub new_free: ::core::primitive::u128,
				#[codec(compact)]
				pub new_reserved: ::core::primitive::u128,
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
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Transfer some liquid free balance to another account."]
				#[doc = ""]
				#[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
				#[doc = "It will decrease the total issuance of the system by the `TransferFee`."]
				#[doc = "If the sender's account is below the existential deposit as a result"]
				#[doc = "of the transfer, the account will be reaped."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
				#[doc = "  types. See related functions below."]
				#[doc = "- It contains a limited number of reads and writes internally and no complex"]
				#[doc = "  computation."]
				#[doc = ""]
				#[doc = "Related functions:"]
				#[doc = ""]
				#[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
				#[doc = "  - Transferring balances to accounts that did not exist before will cause"]
				#[doc = "    `T::OnNewAccount::on_new_account` to be called."]
				#[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
				#[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
				#[doc = "    that the transfer will not kill the origin account."]
				#[doc = "---------------------------------"]
				#[doc = "- Origin account is already in memory, so no DB operations for them."]
				#[doc = "# </weight>"]
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

				#[doc = "Set the balances of a given account."]
				#[doc = ""]
				#[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
				#[doc = "also decrease the total issuance of the system (`TotalIssuance`)."]
				#[doc = "If the new free or reserved balance is below the existential deposit,"]
				#[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call is `root`."]
				pub fn set_balance(
					&self,
					who: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					new_free: ::core::primitive::u128,
					new_reserved: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<SetBalance> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"set_balance",
						SetBalance {
							who,
							new_free,
							new_reserved,
						},
						[
							174u8, 34u8, 80u8, 252u8, 193u8, 51u8, 228u8, 236u8, 234u8, 16u8,
							173u8, 214u8, 122u8, 21u8, 254u8, 7u8, 49u8, 176u8, 18u8, 128u8, 122u8,
							68u8, 72u8, 181u8, 119u8, 90u8, 167u8, 46u8, 203u8, 220u8, 109u8,
							110u8,
						],
					)
				}

				#[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
				#[doc = "specified."]
				#[doc = "# <weight>"]
				#[doc = "- Same as transfer, but additional read and write because the source account is not"]
				#[doc = "  assumed to be in the overlay."]
				#[doc = "# </weight>"]
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

				#[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
				#[doc = "origin account."]
				#[doc = ""]
				#[doc = "99% of the time you want [`transfer`] instead."]
				#[doc = ""]
				#[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
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

				#[doc = "Transfer the entire transferable balance from the caller account."]
				#[doc = ""]
				#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
				#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
				#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
				#[doc = "you might need to prepare the account by removing any reference counters, storage"]
				#[doc = "deposits, etc..."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be Signed."]
				#[doc = ""]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
				#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
				#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
				#[doc = "  keep the sender account alive (true). # <weight>"]
				#[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
				#[doc = "  #</weight>"]
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

				#[doc = "Unreserve some balance from a user by force."]
				#[doc = ""]
				#[doc = "Can only be called by ROOT."]
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
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
				pub reserved: ::core::primitive::u128,
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

				#[doc = " The balance of an account."]
				#[doc = ""]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
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
							246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8,
							40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8,
							217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8,
						],
					)
				}

				#[doc = " The balance of an account."]
				#[doc = ""]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
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
							246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8,
							40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8,
							217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8,
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
						runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
							runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
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
						runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
							runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
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
						runtime_types::frame_support::storage::bounded_vec::BoundedVec<
							runtime_types::pallet_balances::ReserveData<
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
						runtime_types::frame_support::storage::bounded_vec::BoundedVec<
							runtime_types::pallet_balances::ReserveData<
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

				#[doc = " Storage version of the pallet."]
				#[doc = ""]
				#[doc = " This is set to v2.0.0 for new networks."]
				pub fn storage_version(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::pallet_balances::Releases>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"StorageVersion",
						vec![],
						[
							135u8, 96u8, 28u8, 234u8, 124u8, 212u8, 56u8, 140u8, 40u8, 101u8,
							235u8, 128u8, 136u8, 221u8, 182u8, 81u8, 17u8, 9u8, 184u8, 228u8,
							174u8, 165u8, 200u8, 162u8, 214u8, 178u8, 227u8, 72u8, 34u8, 5u8,
							173u8, 96u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum amount required to keep an account open."]
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
			}
		}
	}
	pub mod transaction_payment {
		use super::{root_mod, runtime_types};
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
				#[doc = " The fee to be paid for making a transaction; the per-byte portion."]
				pub fn transaction_byte_fee(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"TransactionPayment",
						"TransactionByteFee",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}

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

				#[doc = " The polynomial that is applied in order to derive fee from weight."]
				pub fn weight_to_fee(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::frame_support::weights::WeightToFeeCoefficient<
								::core::primitive::u128,
							>,
						>,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"TransactionPayment",
						"WeightToFee",
						[
							15u8, 119u8, 137u8, 69u8, 18u8, 105u8, 232u8, 166u8, 200u8, 253u8,
							17u8, 71u8, 185u8, 155u8, 82u8, 215u8, 166u8, 74u8, 238u8, 28u8, 61u8,
							206u8, 89u8, 133u8, 136u8, 131u8, 67u8, 243u8, 19u8, 238u8, 147u8,
							226u8,
						],
					)
				}
			}
		}
	}
	pub mod election_provider_multi_phase {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
						runtime_types::da_runtime::NposSolution16,
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
				pub maybe_next_score: ::core::option::Option<[::core::primitive::u128; 3usize]>,
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
						runtime_types::da_runtime::NposSolution16,
					>,
				>,
				pub num_signed_submissions: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Submit a solution for the unsigned phase."]
				#[doc = ""]
				#[doc = "The dispatch origin fo this call must be __none__."]
				#[doc = ""]
				#[doc = "This submission is checked on the fly. Moreover, this unsigned solution is only"]
				#[doc = "validated when submitted to the pool from the **local** node. Effectively, this means"]
				#[doc = "that only active validators can submit this transaction when authoring a block (similar"]
				#[doc = "to an inherent)."]
				#[doc = ""]
				#[doc = "To prevent any incorrect solution (and thus wasted time/weight), this transaction will"]
				#[doc = "panic if the solution submitted by the validator is invalid in any way, effectively"]
				#[doc = "putting their authoring reward at risk."]
				#[doc = ""]
				#[doc = "No deposit or reward is associated with this submission."]
				pub fn submit_unsigned(
					&self,
					raw_solution: runtime_types::pallet_election_provider_multi_phase::RawSolution<
						runtime_types::da_runtime::NposSolution16,
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
							78u8, 88u8, 168u8, 223u8, 50u8, 163u8, 76u8, 60u8, 55u8, 60u8, 30u8,
							230u8, 169u8, 157u8, 221u8, 88u8, 178u8, 139u8, 103u8, 130u8, 219u8,
							183u8, 246u8, 213u8, 64u8, 196u8, 181u8, 151u8, 42u8, 189u8, 185u8,
							141u8,
						],
					)
				}

				#[doc = "Set a new value for `MinimumUntrustedScore`."]
				#[doc = ""]
				#[doc = "Dispatch origin must be aligned with `T::ForceOrigin`."]
				#[doc = ""]
				#[doc = "This check can be turned off by setting the value to `None`."]
				pub fn set_minimum_untrusted_score(
					&self,
					maybe_next_score: ::core::option::Option<[::core::primitive::u128; 3usize]>,
				) -> ::subxt::tx::StaticTxPayload<SetMinimumUntrustedScore> {
					::subxt::tx::StaticTxPayload::new(
						"ElectionProviderMultiPhase",
						"set_minimum_untrusted_score",
						SetMinimumUntrustedScore { maybe_next_score },
						[
							254u8, 242u8, 51u8, 160u8, 180u8, 220u8, 142u8, 142u8, 221u8, 165u8,
							167u8, 199u8, 83u8, 49u8, 255u8, 224u8, 24u8, 197u8, 94u8, 177u8, 57u8,
							223u8, 225u8, 231u8, 33u8, 87u8, 41u8, 137u8, 203u8, 110u8, 192u8,
							54u8,
						],
					)
				}

				#[doc = "Set a solution in the queue, to be handed out to the client of this pallet in the next"]
				#[doc = "call to `ElectionProvider::elect`."]
				#[doc = ""]
				#[doc = "This can only be set by `T::ForceOrigin`, and only when the phase is `Emergency`."]
				#[doc = ""]
				#[doc = "The solution is not checked for any feasibility and is assumed to be trustworthy, as any"]
				#[doc = "feasibility check itself can in principle cause the election process to fail (due to"]
				#[doc = "memory/weight constrains)."]
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

				#[doc = "Submit a solution for the signed phase."]
				#[doc = ""]
				#[doc = "The dispatch origin fo this call must be __signed__."]
				#[doc = ""]
				#[doc = "The solution is potentially queued, based on the claimed score and processed at the end"]
				#[doc = "of the signed phase."]
				#[doc = ""]
				#[doc = "A deposit is reserved and recorded for the solution. Based on the outcome, the solution"]
				#[doc = "might be rewarded, slashed, or get all or a part of the deposit back."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Queue size must be provided as witness data."]
				#[doc = "# </weight>"]
				pub fn submit(
					&self,
					raw_solution: runtime_types::pallet_election_provider_multi_phase::RawSolution<
						runtime_types::da_runtime::NposSolution16,
					>,
					num_signed_submissions: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Submit> {
					::subxt::tx::StaticTxPayload::new(
						"ElectionProviderMultiPhase",
						"submit",
						Submit {
							raw_solution: ::std::boxed::Box::new(raw_solution),
							num_signed_submissions,
						},
						[
							28u8, 241u8, 193u8, 108u8, 35u8, 165u8, 57u8, 74u8, 109u8, 80u8, 21u8,
							222u8, 189u8, 105u8, 159u8, 132u8, 223u8, 57u8, 37u8, 178u8, 7u8, 25u8,
							88u8, 234u8, 68u8, 101u8, 31u8, 243u8, 202u8, 58u8, 13u8, 27u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
			#[doc = "If the solution is signed, this means that it hasn't yet been processed. If the"]
			#[doc = "solution is unsigned, this means that it has also been processed."]
			#[doc = ""]
			#[doc = "The `bool` is `true` when a previous solution was ejected to make room for this one."]
			pub struct SolutionStored {
				pub election_compute:
					runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
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
			#[doc = "The election has been finalized, with `Some` of the given computation, or else if the"]
			#[doc = "election failed, `None`."]
			pub struct ElectionFinalized {
				pub election_compute: ::core::option::Option<
					runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
				>,
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
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "The signed phase of the given round has started."]
			pub struct SignedPhaseStarted {
				pub round: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for SignedPhaseStarted {
				const EVENT: &'static str = "SignedPhaseStarted";
				const PALLET: &'static str = "ElectionProviderMultiPhase";
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
			#[doc = "The unsigned phase of the given round has started."]
			pub struct UnsignedPhaseStarted {
				pub round: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for UnsignedPhaseStarted {
				const EVENT: &'static str = "UnsignedPhaseStarted";
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
				pub fn queued_solution(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_election_provider_multi_phase::ReadySolution<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
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
							8u8, 176u8, 111u8, 220u8, 33u8, 50u8, 158u8, 87u8, 176u8, 7u8, 102u8,
							136u8, 81u8, 56u8, 45u8, 150u8, 241u8, 33u8, 81u8, 116u8, 42u8, 50u8,
							153u8, 26u8, 30u8, 72u8, 13u8, 165u8, 181u8, 97u8, 110u8, 84u8,
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
							18u8, 60u8, 209u8, 69u8, 24u8, 23u8, 217u8, 125u8, 88u8, 104u8, 97u8,
							43u8, 56u8, 124u8, 137u8, 146u8, 53u8, 233u8, 193u8, 117u8, 14u8,
							200u8, 59u8, 125u8, 31u8, 145u8, 135u8, 59u8, 149u8, 39u8, 222u8, 18u8,
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

				#[doc = " A sorted, bounded set of `(score, index)`, where each `index` points to a value in"]
				#[doc = " `SignedSubmissions`."]
				#[doc = ""]
				#[doc = " We never need to process more than a single signed submission at a time. Signed submissions"]
				#[doc = " can be quite large, so we're willing to pay the cost of multiple database accesses to access"]
				#[doc = " them one at a time instead of reading and decoding all of them at once."]
				pub fn signed_submission_indices(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_support::storage::bounded_btree_map::BoundedBTreeMap<
							[::core::primitive::u128; 3usize],
							::core::primitive::u32,
						>,
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
							109u8, 26u8, 187u8, 45u8, 252u8, 101u8, 124u8, 47u8, 70u8, 41u8, 155u8,
							221u8, 20u8, 191u8, 227u8, 236u8, 161u8, 217u8, 234u8, 57u8, 145u8,
							53u8, 21u8, 165u8, 150u8, 48u8, 241u8, 39u8, 196u8, 171u8, 50u8, 96u8,
						],
					)
				}

				#[doc = " Unchecked, signed solutions."]
				#[doc = ""]
				#[doc = " Together with `SubmissionIndices`, this stores a bounded set of `SignedSubmissions` while"]
				#[doc = " allowing us to keep only a single one in memory at a time."]
				#[doc = ""]
				#[doc = " Twox note: the key of the map is an auto-incrementing index which users cannot inspect or"]
				#[doc = " affect; we shouldn't need a cryptographically secure hasher."]				pub fn signed_submissions_map (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , runtime_types :: da_runtime :: NposSolution16 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::StaticStorageAddress::new(
						"ElectionProviderMultiPhase",
						"SignedSubmissionsMap",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							175u8, 77u8, 57u8, 228u8, 214u8, 90u8, 192u8, 157u8, 159u8, 197u8,
							191u8, 154u8, 215u8, 141u8, 229u8, 170u8, 10u8, 249u8, 237u8, 158u8,
							215u8, 82u8, 222u8, 45u8, 23u8, 218u8, 58u8, 11u8, 11u8, 139u8, 52u8,
							233u8,
						],
					)
				}

				#[doc = " Unchecked, signed solutions."]
				#[doc = ""]
				#[doc = " Together with `SubmissionIndices`, this stores a bounded set of `SignedSubmissions` while"]
				#[doc = " allowing us to keep only a single one in memory at a time."]
				#[doc = ""]
				#[doc = " Twox note: the key of the map is an auto-incrementing index which users cannot inspect or"]
				#[doc = " affect; we shouldn't need a cryptographically secure hasher."]				pub fn signed_submissions_map_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , runtime_types :: da_runtime :: NposSolution16 > > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::StaticStorageAddress::new(
						"ElectionProviderMultiPhase",
						"SignedSubmissionsMap",
						Vec::new(),
						[
							175u8, 77u8, 57u8, 228u8, 214u8, 90u8, 192u8, 157u8, 159u8, 197u8,
							191u8, 154u8, 215u8, 141u8, 229u8, 170u8, 10u8, 249u8, 237u8, 158u8,
							215u8, 82u8, 222u8, 45u8, 23u8, 218u8, 58u8, 11u8, 11u8, 139u8, 52u8,
							233u8,
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
					::subxt::metadata::DecodeStaticType<[::core::primitive::u128; 3usize]>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ElectionProviderMultiPhase",
						"MinimumUntrustedScore",
						vec![],
						[
							16u8, 168u8, 27u8, 66u8, 246u8, 15u8, 213u8, 144u8, 136u8, 38u8, 180u8,
							231u8, 79u8, 4u8, 100u8, 39u8, 189u8, 224u8, 180u8, 25u8, 3u8, 51u8,
							35u8, 27u8, 112u8, 234u8, 221u8, 18u8, 100u8, 129u8, 59u8, 185u8,
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
				#[doc = " \"better\" (in any phase)."]
				pub fn solution_improvement_threshold(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::per_things::Perbill,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"SolutionImprovementThreshold",
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

				#[doc = " Maximum weight that the miner should consume."]
				#[doc = ""]
				#[doc = " The miner will ensure that the total weight of the unsigned solution will not exceed"]
				#[doc = " this value, based on [`WeightInfo::submit_unsigned`]."]
				pub fn miner_max_weight(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"MinerMaxWeight",
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
				#[doc = " This should probably be similar to [`Config::MinerMaxWeight`]."]
				pub fn signed_max_weight(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"SignedMaxWeight",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
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

				#[doc = " The maximum number of voters to put in the snapshot. At the moment, snapshots are only"]
				#[doc = " over a single block, but once multi-block elections are introduced they will take place"]
				#[doc = " over multiple blocks."]
				#[doc = ""]
				#[doc = " Also, note the data type: If the voters are represented by a `u32` in `type"]
				#[doc = " CompactSolution`, the same `u32` is used here to ensure bounds are respected."]
				pub fn voter_snapshot_per_block(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ElectionProviderMultiPhase",
						"VoterSnapshotPerBlock",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " Maximum length (bytes) that the mined solution should consume."]
				#[doc = ""]
				#[doc = " The miner will ensure that the total length of the unsigned solution will not exceed"]
				#[doc = " this value."]
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
			}
		}
	}
	pub mod staking {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				pub controller: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
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
			pub struct SetController {
				pub controller: ::subxt::ext::sp_runtime::MultiAddress<
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
			pub struct SetHistoryDepth {
				#[codec(compact)]
				pub new_history_depth: ::core::primitive::u32,
				#[codec(compact)]
				pub era_items_deleted: ::core::primitive::u32,
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
			pub struct SetStakingLimits {
				pub min_nominator_bond: ::core::primitive::u128,
				pub min_validator_bond: ::core::primitive::u128,
				pub max_nominator_count: ::core::option::Option<::core::primitive::u32>,
				pub max_validator_count: ::core::option::Option<::core::primitive::u32>,
				pub threshold:
					::core::option::Option<runtime_types::sp_arithmetic::per_things::Percent>,
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
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Take the origin account as a stash and lock up `value` of its balance. `controller` will"]
				#[doc = "be the account that controls it."]
				#[doc = ""]
				#[doc = "`value` must be more than the `minimum_balance` specified by `T::Currency`."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ by the stash account."]
				#[doc = ""]
				#[doc = "Emits `Bonded`."]
				#[doc = "# <weight>"]
				#[doc = "- Independent of the arguments. Moderate complexity."]
				#[doc = "- O(1)."]
				#[doc = "- Three extra DB entries."]
				#[doc = ""]
				#[doc = "NOTE: Two of the storage writes (`Self::bonded`, `Self::payee`) are _never_ cleaned"]
				#[doc = "unless the `origin` falls below _existential deposit_ and gets removed as dust."]
				#[doc = "------------------"]
				#[doc = "# </weight>"]
				pub fn bond(
					&self,
					controller: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
					payee: runtime_types::pallet_staking::RewardDestination<
						::subxt::ext::sp_core::crypto::AccountId32,
					>,
				) -> ::subxt::tx::StaticTxPayload<Bond> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"bond",
						Bond {
							controller,
							value,
							payee,
						},
						[
							140u8, 13u8, 108u8, 181u8, 212u8, 177u8, 190u8, 212u8, 163u8, 40u8,
							120u8, 232u8, 126u8, 213u8, 6u8, 181u8, 99u8, 252u8, 58u8, 54u8, 139u8,
							64u8, 67u8, 76u8, 53u8, 226u8, 11u8, 133u8, 235u8, 159u8, 103u8, 210u8,
						],
					)
				}

				#[doc = "Add some extra amount that have appeared in the stash `free_balance` into the balance up"]
				#[doc = "for staking."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ by the stash, not the controller."]
				#[doc = ""]
				#[doc = "Use this if there are additional funds in your stash account that you wish to bond."]
				#[doc = "Unlike [`bond`](Self::bond) or [`unbond`](Self::unbond) this function does not impose"]
				#[doc = "any limitation on the amount that can be added."]
				#[doc = ""]
				#[doc = "Emits `Bonded`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Independent of the arguments. Insignificant complexity."]
				#[doc = "- O(1)."]
				#[doc = "# </weight>"]
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

				#[doc = "Schedule a portion of the stash to be unlocked ready for transfer out after the bond"]
				#[doc = "period ends. If this leaves an amount actively bonded less than"]
				#[doc = "T::Currency::minimum_balance(), then it is increased to the full amount."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
				#[doc = ""]
				#[doc = "Once the unlock period is done, you can call `withdraw_unbonded` to actually move"]
				#[doc = "the funds out of management ready for transfer."]
				#[doc = ""]
				#[doc = "No more than a limited number of unlocking chunks (see `MAX_UNLOCKING_CHUNKS`)"]
				#[doc = "can co-exists at the same time. In that case, [`Call::withdraw_unbonded`] need"]
				#[doc = "to be called first to remove some of the chunks (if possible)."]
				#[doc = ""]
				#[doc = "If a user encounters the `InsufficientBond` error when calling this extrinsic,"]
				#[doc = "they should call `chill` first in order to free up their bonded funds."]
				#[doc = ""]
				#[doc = "Emits `Unbonded`."]
				#[doc = ""]
				#[doc = "See also [`Call::withdraw_unbonded`]."]
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

				#[doc = "Remove any unlocked chunks from the `unlocking` queue from our management."]
				#[doc = ""]
				#[doc = "This essentially frees up that balance to be used by the stash account to do"]
				#[doc = "whatever it wants."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ by the controller."]
				#[doc = ""]
				#[doc = "Emits `Withdrawn`."]
				#[doc = ""]
				#[doc = "See also [`Call::unbond`]."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Complexity O(S) where S is the number of slashing spans to remove"]
				#[doc = "NOTE: Weight annotation is the kill scenario, we refund otherwise."]
				#[doc = "# </weight>"]
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

				#[doc = "Declare the desire to validate for the origin controller."]
				#[doc = ""]
				#[doc = "Effects will be felt at the beginning of the next era."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
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

				#[doc = "Declare the desire to nominate `targets` for the origin controller."]
				#[doc = ""]
				#[doc = "Effects will be felt at the beginning of the next era."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- The transaction's complexity is proportional to the size of `targets` (N)"]
				#[doc = "which is capped at CompactAssignments::LIMIT (MAX_NOMINATIONS)."]
				#[doc = "- Both the reads and writes follow a similar pattern."]
				#[doc = "# </weight>"]
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

				#[doc = "Declare no desire to either validate or nominate."]
				#[doc = ""]
				#[doc = "Effects will be felt at the beginning of the next era."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Independent of the arguments. Insignificant complexity."]
				#[doc = "- Contains one read."]
				#[doc = "- Writes are limited to the `origin` account key."]
				#[doc = "# </weight>"]
				pub fn chill(&self) -> ::subxt::tx::StaticTxPayload<Chill> {
					::subxt::tx::StaticTxPayload::new("Staking", "chill", Chill {}, [
						94u8, 20u8, 196u8, 31u8, 220u8, 125u8, 115u8, 167u8, 140u8, 3u8, 20u8,
						132u8, 81u8, 120u8, 215u8, 166u8, 230u8, 56u8, 16u8, 222u8, 31u8, 153u8,
						120u8, 62u8, 153u8, 67u8, 220u8, 239u8, 11u8, 234u8, 127u8, 122u8,
					])
				}

				#[doc = "(Re-)set the payment target for a controller."]
				#[doc = ""]
				#[doc = "Effects will be felt at the beginning of the next era."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Independent of the arguments. Insignificant complexity."]
				#[doc = "- Contains a limited number of reads."]
				#[doc = "- Writes are limited to the `origin` account key."]
				#[doc = "---------"]
				#[doc = "- Weight: O(1)"]
				#[doc = "- DB Weight:"]
				#[doc = "    - Read: Ledger"]
				#[doc = "    - Write: Payee"]
				#[doc = "# </weight>"]
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

				#[doc = "(Re-)set the controller of a stash."]
				#[doc = ""]
				#[doc = "Effects will be felt at the beginning of the next era."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ by the stash, not the controller."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Independent of the arguments. Insignificant complexity."]
				#[doc = "- Contains a limited number of reads."]
				#[doc = "- Writes are limited to the `origin` account key."]
				#[doc = "----------"]
				#[doc = "Weight: O(1)"]
				#[doc = "DB Weight:"]
				#[doc = "- Read: Bonded, Ledger New Controller, Ledger Old Controller"]
				#[doc = "- Write: Bonded, Ledger New Controller, Ledger Old Controller"]
				#[doc = "# </weight>"]
				pub fn set_controller(
					&self,
					controller: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetController> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"set_controller",
						SetController { controller },
						[
							154u8, 80u8, 184u8, 176u8, 74u8, 106u8, 72u8, 242u8, 64u8, 81u8, 169u8,
							157u8, 200u8, 97u8, 117u8, 192u8, 143u8, 166u8, 38u8, 235u8, 75u8,
							161u8, 177u8, 229u8, 229u8, 82u8, 95u8, 39u8, 40u8, 116u8, 9u8, 204u8,
						],
					)
				}

				#[doc = "Sets the ideal number of validators."]
				#[doc = ""]
				#[doc = "The dispatch origin must be Root."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Weight: O(1)"]
				#[doc = "Write: Validator Count"]
				#[doc = "# </weight>"]
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

				#[doc = "Increments the ideal number of validators."]
				#[doc = ""]
				#[doc = "The dispatch origin must be Root."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Same as [`Self::set_validator_count`]."]
				#[doc = "# </weight>"]
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

				#[doc = "Scale up the ideal number of validators by a factor."]
				#[doc = ""]
				#[doc = "The dispatch origin must be Root."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Same as [`Self::set_validator_count`]."]
				#[doc = "# </weight>"]
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

				#[doc = "Force there to be no new eras indefinitely."]
				#[doc = ""]
				#[doc = "The dispatch origin must be Root."]
				#[doc = ""]
				#[doc = "# Warning"]
				#[doc = ""]
				#[doc = "The election process starts multiple blocks before the end of the era."]
				#[doc = "Thus the election process may be ongoing when this is called. In this case the"]
				#[doc = "election will continue until the next era is triggered."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- No arguments."]
				#[doc = "- Weight: O(1)"]
				#[doc = "- Write: ForceEra"]
				#[doc = "# </weight>"]
				pub fn force_no_eras(&self) -> ::subxt::tx::StaticTxPayload<ForceNoEras> {
					::subxt::tx::StaticTxPayload::new("Staking", "force_no_eras", ForceNoEras {}, [
						16u8, 81u8, 207u8, 168u8, 23u8, 236u8, 11u8, 75u8, 141u8, 107u8, 92u8, 2u8,
						53u8, 111u8, 252u8, 116u8, 91u8, 120u8, 75u8, 24u8, 125u8, 53u8, 9u8, 28u8,
						242u8, 87u8, 245u8, 55u8, 40u8, 103u8, 151u8, 178u8,
					])
				}

				#[doc = "Force there to be a new era at the end of the next session. After this, it will be"]
				#[doc = "reset to normal (non-forced) behaviour."]
				#[doc = ""]
				#[doc = "The dispatch origin must be Root."]
				#[doc = ""]
				#[doc = "# Warning"]
				#[doc = ""]
				#[doc = "The election process starts multiple blocks before the end of the era."]
				#[doc = "If this is called just before a new era is triggered, the election process may not"]
				#[doc = "have enough blocks to get a result."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- No arguments."]
				#[doc = "- Weight: O(1)"]
				#[doc = "- Write ForceEra"]
				#[doc = "# </weight>"]
				pub fn force_new_era(&self) -> ::subxt::tx::StaticTxPayload<ForceNewEra> {
					::subxt::tx::StaticTxPayload::new("Staking", "force_new_era", ForceNewEra {}, [
						230u8, 242u8, 169u8, 196u8, 78u8, 145u8, 24u8, 191u8, 113u8, 68u8, 5u8,
						138u8, 48u8, 51u8, 109u8, 126u8, 73u8, 136u8, 162u8, 158u8, 174u8, 201u8,
						213u8, 230u8, 215u8, 44u8, 200u8, 32u8, 75u8, 27u8, 23u8, 254u8,
					])
				}

				#[doc = "Set the validators who cannot be slashed (if any)."]
				#[doc = ""]
				#[doc = "The dispatch origin must be Root."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(V)"]
				#[doc = "- Write: Invulnerables"]
				#[doc = "# </weight>"]
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

				#[doc = "Force a current staker to become completely unstaked, immediately."]
				#[doc = ""]
				#[doc = "The dispatch origin must be Root."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "O(S) where S is the number of slashing spans to be removed"]
				#[doc = "Reads: Bonded, Slashing Spans, Account, Locks"]
				#[doc = "Writes: Bonded, Slashing Spans (if S > 0), Ledger, Payee, Validators, Nominators,"]
				#[doc = "Account, Locks Writes Each: SpanSlash * S"]
				#[doc = "# </weight>"]
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

				#[doc = "Force there to be a new era at the end of sessions indefinitely."]
				#[doc = ""]
				#[doc = "The dispatch origin must be Root."]
				#[doc = ""]
				#[doc = "# Warning"]
				#[doc = ""]
				#[doc = "The election process starts multiple blocks before the end of the era."]
				#[doc = "If this is called just before a new era is triggered, the election process may not"]
				#[doc = "have enough blocks to get a result."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Weight: O(1)"]
				#[doc = "- Write: ForceEra"]
				#[doc = "# </weight>"]
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

				#[doc = "Cancel enactment of a deferred slash."]
				#[doc = ""]
				#[doc = "Can be called by the `T::SlashCancelOrigin`."]
				#[doc = ""]
				#[doc = "Parameters: era and indices of the slashes for that era to kill."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Complexity: O(U + S)"]
				#[doc = "with U unapplied slashes weighted with U=1000"]
				#[doc = "and S is the number of slash indices to be canceled."]
				#[doc = "- Read: Unapplied Slashes"]
				#[doc = "- Write: Unapplied Slashes"]
				#[doc = "# </weight>"]
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

				#[doc = "Pay out all the stakers behind a single validator for a single era."]
				#[doc = ""]
				#[doc = "- `validator_stash` is the stash account of the validator. Their nominators, up to"]
				#[doc = "  `T::MaxNominatorRewardedPerValidator`, will also receive their rewards."]
				#[doc = "- `era` may be any era between `[current_era - history_depth; current_era]`."]
				#[doc = ""]
				#[doc = "The origin of this call must be _Signed_. Any account can call this function, even if"]
				#[doc = "it is not one of the stakers."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Time complexity: at most O(MaxNominatorRewardedPerValidator)."]
				#[doc = "- Contains a limited number of reads and writes."]
				#[doc = "-----------"]
				#[doc = "N is the Number of payouts for the validator (including the validator)"]
				#[doc = "Weight:"]
				#[doc = "- Reward Destination Staked: O(N)"]
				#[doc = "- Reward Destination Controller (Creating): O(N)"]
				#[doc = ""]
				#[doc = "  NOTE: weights are assuming that payouts are made to alive stash account (Staked)."]
				#[doc = "  Paying even a dead controller is cheaper weight-wise. We don't do any refunds here."]
				#[doc = "# </weight>"]
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

				#[doc = "Rebond a portion of the stash scheduled to be unlocked."]
				#[doc = ""]
				#[doc = "The dispatch origin must be signed by the controller."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Time complexity: O(L), where L is unlocking chunks"]
				#[doc = "- Bounded by `MAX_UNLOCKING_CHUNKS`."]
				#[doc = "- Storage changes: Can't increase storage, only decrease it."]
				#[doc = "# </weight>"]
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

				#[doc = "Set `HistoryDepth` value. This function will delete any history information"]
				#[doc = "when `HistoryDepth` is reduced."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "- `new_history_depth`: The new history depth you would like to set."]
				#[doc = "- `era_items_deleted`: The number of items that will be deleted by this dispatch. This"]
				#[doc = "  should report all the storage items that will be deleted by clearing old era history."]
				#[doc = "  Needed to report an accurate weight for the dispatch. Trusted by `Root` to report an"]
				#[doc = "  accurate number."]
				#[doc = ""]
				#[doc = "Origin must be root."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- E: Number of history depths removed, i.e. 10 -> 7 = 3"]
				#[doc = "- Weight: O(E)"]
				#[doc = "- DB Weight:"]
				#[doc = "    - Reads: Current Era, History Depth"]
				#[doc = "    - Writes: History Depth"]
				#[doc = "    - Clear Prefix Each: Era Stakers, EraStakersClipped, ErasValidatorPrefs"]
				#[doc = "    - Writes Each: ErasValidatorReward, ErasRewardPoints, ErasTotalStake,"]
				#[doc = "      ErasStartSessionIndex"]
				#[doc = "# </weight>"]
				pub fn set_history_depth(
					&self,
					new_history_depth: ::core::primitive::u32,
					era_items_deleted: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetHistoryDepth> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"set_history_depth",
						SetHistoryDepth {
							new_history_depth,
							era_items_deleted,
						},
						[
							174u8, 55u8, 231u8, 132u8, 219u8, 215u8, 118u8, 202u8, 13u8, 151u8,
							193u8, 248u8, 141u8, 180u8, 56u8, 103u8, 90u8, 182u8, 194u8, 198u8,
							120u8, 251u8, 143u8, 218u8, 81u8, 59u8, 13u8, 161u8, 247u8, 57u8,
							178u8, 122u8,
						],
					)
				}

				#[doc = "Remove all data structures concerning a staker/stash once it is at a state where it can"]
				#[doc = "be considered `dust` in the staking system. The requirements are:"]
				#[doc = ""]
				#[doc = "1. the `total_balance` of the stash is below existential deposit."]
				#[doc = "2. or, the `ledger.total` of the stash is below existential deposit."]
				#[doc = ""]
				#[doc = "The former can happen in cases like a slash; the latter when a fully unbonded account"]
				#[doc = "is still receiving staking rewards in `RewardDestination::Staked`."]
				#[doc = ""]
				#[doc = "It can be called by anyone, as long as `stash` meets the above requirements."]
				#[doc = ""]
				#[doc = "Refunds the transaction fees upon successful execution."]
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

				#[doc = "Remove the given nominations from the calling validator."]
				#[doc = ""]
				#[doc = "Effects will be felt at the beginning of the next era."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
				#[doc = ""]
				#[doc = "- `who`: A list of nominator stash accounts who are nominating this validator which"]
				#[doc = "  should no longer be nominating this validator."]
				#[doc = ""]
				#[doc = "Note: Making this call only makes sense if you first set the validator preferences to"]
				#[doc = "block any further nominations."]
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

				#[doc = "Update the various staking limits this pallet."]
				#[doc = ""]
				#[doc = "* `min_nominator_bond`: The minimum active bond needed to be a nominator."]
				#[doc = "* `min_validator_bond`: The minimum active bond needed to be a validator."]
				#[doc = "* `max_nominator_count`: The max number of users who can be a nominator at once. When"]
				#[doc = "  set to `None`, no limit is enforced."]
				#[doc = "* `max_validator_count`: The max number of users who can be a validator at once. When"]
				#[doc = "  set to `None`, no limit is enforced."]
				#[doc = ""]
				#[doc = "Origin must be Root to call this function."]
				#[doc = ""]
				#[doc = "NOTE: Existing nominators and validators will not be affected by this update."]
				#[doc = "to kick people under the new limits, `chill_other` should be called."]
				pub fn set_staking_limits(
					&self,
					min_nominator_bond: ::core::primitive::u128,
					min_validator_bond: ::core::primitive::u128,
					max_nominator_count: ::core::option::Option<::core::primitive::u32>,
					max_validator_count: ::core::option::Option<::core::primitive::u32>,
					threshold: ::core::option::Option<
						runtime_types::sp_arithmetic::per_things::Percent,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetStakingLimits> {
					::subxt::tx::StaticTxPayload::new(
						"Staking",
						"set_staking_limits",
						SetStakingLimits {
							min_nominator_bond,
							min_validator_bond,
							max_nominator_count,
							max_validator_count,
							threshold,
						},
						[
							14u8, 16u8, 156u8, 44u8, 119u8, 0u8, 238u8, 48u8, 46u8, 230u8, 47u8,
							9u8, 131u8, 237u8, 139u8, 28u8, 230u8, 110u8, 194u8, 35u8, 152u8, 35u8,
							132u8, 30u8, 146u8, 180u8, 164u8, 12u8, 94u8, 80u8, 188u8, 113u8,
						],
					)
				}

				#[doc = "Declare a `controller` to stop participating as either a validator or nominator."]
				#[doc = ""]
				#[doc = "Effects will be felt at the beginning of the next era."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_, but can be called by anyone."]
				#[doc = ""]
				#[doc = "If the caller is the same as the controller being targeted, then no further checks are"]
				#[doc = "enforced, and this function behaves just like `chill`."]
				#[doc = ""]
				#[doc = "If the caller is different than the controller being targeted, the following conditions"]
				#[doc = "must be met:"]
				#[doc = "* A `ChillThreshold` must be set and checked which defines how close to the max"]
				#[doc = "  nominators or validators we must reach before users can start chilling one-another."]
				#[doc = "* A `MaxNominatorCount` and `MaxValidatorCount` must be set which is used to determine"]
				#[doc = "  how close we are to the threshold."]
				#[doc = "* A `MinNominatorBond` and `MinValidatorBond` must be set and checked, which determines"]
				#[doc = "  if this is a person that should be chilled because they have not met the threshold"]
				#[doc = "  bond required."]
				#[doc = ""]
				#[doc = "This can be helpful if bond requirements are updated, and we need to remove old users"]
				#[doc = "who do not satisfy these requirements."]
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
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
			#[doc = "\\[era_index, validator_payout, remainder\\]"]
			pub struct EraPaid(
				pub ::core::primitive::u32,
				pub ::core::primitive::u128,
				pub ::core::primitive::u128,
			);
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
			#[doc = "The nominator has been rewarded by this amount. \\[stash, amount\\]"]
			pub struct Rewarded(
				pub ::subxt::ext::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
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
			#[doc = "One validator (and its nominators) has been slashed by the given amount."]
			#[doc = "\\[validator, amount\\]"]
			pub struct Slashed(
				pub ::subxt::ext::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::events::StaticEvent for Slashed {
				const EVENT: &'static str = "Slashed";
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
			#[doc = "not be processed. \\[session_index\\]"]
			pub struct OldSlashingReportDiscarded(pub ::core::primitive::u32);
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
			pub struct Bonded(
				pub ::subxt::ext::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
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
			#[doc = "An account has unbonded this amount. \\[stash, amount\\]"]
			pub struct Unbonded(
				pub ::subxt::ext::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
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
			#[doc = "from the unlocking queue. \\[stash, amount\\]"]
			pub struct Withdrawn(
				pub ::subxt::ext::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
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
			#[doc = "A nominator has been kicked from a validator. \\[nominator, stash\\]"]
			pub struct Kicked(
				pub ::subxt::ext::sp_core::crypto::AccountId32,
				pub ::subxt::ext::sp_core::crypto::AccountId32,
			);
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
			#[doc = "\\[stash\\]"]
			pub struct Chilled(pub ::subxt::ext::sp_core::crypto::AccountId32);
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
			#[doc = "The stakers' rewards are getting paid. \\[era_index, validator_stash\\]"]
			pub struct PayoutStarted(
				pub ::core::primitive::u32,
				pub ::subxt::ext::sp_core::crypto::AccountId32,
			);
			impl ::subxt::events::StaticEvent for PayoutStarted {
				const EVENT: &'static str = "PayoutStarted";
				const PALLET: &'static str = "Staking";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Number of eras to keep in history."]
				#[doc = ""]
				#[doc = " Information is kept for eras in `[current_era - history_depth; current_era]`."]
				#[doc = ""]
				#[doc = " Must be more than the number of eras delayed by session otherwise. I.e. active era must"]
				#[doc = " always be in history. I.e. `active_era > current_era - history_depth` must be"]
				#[doc = " guaranteed."]
				pub fn history_depth(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"HistoryDepth",
						vec![],
						[
							41u8, 54u8, 118u8, 245u8, 75u8, 136u8, 220u8, 25u8, 55u8, 255u8, 149u8,
							177u8, 49u8, 155u8, 167u8, 188u8, 170u8, 29u8, 251u8, 44u8, 240u8,
							250u8, 225u8, 205u8, 102u8, 74u8, 25u8, 47u8, 52u8, 235u8, 204u8,
							167u8,
						],
					)
				}

				#[doc = " The ideal number of staking participants."]
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

				#[doc = " Map from all (unlocked) \"controller\" accounts to the info regarding the staking."]
				pub fn ledger(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::StakingLedger<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
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
							217u8, 229u8, 36u8, 67u8, 180u8, 245u8, 222u8, 190u8, 109u8, 153u8,
							161u8, 5u8, 220u8, 160u8, 108u8, 6u8, 26u8, 140u8, 111u8, 83u8, 194u8,
							115u8, 4u8, 120u8, 209u8, 79u8, 91u8, 171u8, 201u8, 41u8, 103u8, 4u8,
						],
					)
				}

				#[doc = " Map from all (unlocked) \"controller\" accounts to the info regarding the staking."]
				pub fn ledger_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::StakingLedger<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
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
							217u8, 229u8, 36u8, 67u8, 180u8, 245u8, 222u8, 190u8, 109u8, 153u8,
							161u8, 5u8, 220u8, 160u8, 108u8, 6u8, 26u8, 140u8, 111u8, 83u8, 194u8,
							115u8, 4u8, 120u8, 209u8, 79u8, 91u8, 171u8, 201u8, 41u8, 103u8, 4u8,
						],
					)
				}

				#[doc = " Where the reward payment should be made. Keyed by stash."]
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
				#[doc = " When updating this storage item, you must also update the `CounterForValidators`."]
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
				#[doc = " When updating this storage item, you must also update the `CounterForValidators`."]
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

				#[doc = " A tracker to keep count of the number of items in the `Validators` map."]
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

				#[doc = " The map from nominator stash key to the set of stash keys of all validators to nominate."]
				#[doc = ""]
				#[doc = " When updating this storage item, you must also update the `CounterForNominators`."]
				pub fn nominators(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::Nominations<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					>,
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
							117u8, 69u8, 174u8, 73u8, 25u8, 140u8, 164u8, 237u8, 158u8, 0u8, 156u8,
							129u8, 243u8, 172u8, 225u8, 85u8, 47u8, 23u8, 162u8, 6u8, 2u8, 10u8,
							221u8, 229u8, 121u8, 132u8, 196u8, 251u8, 66u8, 134u8, 173u8, 72u8,
						],
					)
				}

				#[doc = " The map from nominator stash key to the set of stash keys of all validators to nominate."]
				#[doc = ""]
				#[doc = " When updating this storage item, you must also update the `CounterForNominators`."]
				pub fn nominators_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_staking::Nominations<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"Nominators",
						Vec::new(),
						[
							117u8, 69u8, 174u8, 73u8, 25u8, 140u8, 164u8, 237u8, 158u8, 0u8, 156u8,
							129u8, 243u8, 172u8, 225u8, 85u8, 47u8, 23u8, 162u8, 6u8, 2u8, 10u8,
							221u8, 229u8, 121u8, 132u8, 196u8, 251u8, 66u8, 134u8, 173u8, 72u8,
						],
					)
				}

				#[doc = " A tracker to keep count of the number of items in the `Nominators` map."]
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

				#[doc = " The earliest era for which we have a pending, unapplied slash."]
				pub fn earliest_unapplied_slash(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"EarliestUnappliedSlash",
						vec![],
						[
							2u8, 167u8, 88u8, 76u8, 113u8, 225u8, 232u8, 80u8, 183u8, 162u8, 104u8,
							28u8, 162u8, 13u8, 120u8, 45u8, 200u8, 130u8, 147u8, 124u8, 210u8,
							111u8, 30u8, 222u8, 70u8, 79u8, 125u8, 157u8, 56u8, 252u8, 237u8,
							216u8,
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

				#[doc = " True if network has been upgraded to this version."]
				#[doc = " Storage version of the pallet."]
				#[doc = ""]
				#[doc = " This is set to v7.0.0 for new networks."]
				pub fn storage_version(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::pallet_staking::Releases>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Staking",
						"StorageVersion",
						vec![],
						[
							79u8, 94u8, 168u8, 253u8, 84u8, 21u8, 48u8, 226u8, 9u8, 146u8, 92u8,
							76u8, 254u8, 101u8, 212u8, 177u8, 191u8, 127u8, 74u8, 8u8, 26u8, 204u8,
							174u8, 250u8, 62u8, 221u8, 88u8, 214u8, 16u8, 65u8, 34u8, 243u8,
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
			}
		}
	}
	pub mod session {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				pub keys: runtime_types::da_runtime::SessionKeys,
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
				#[doc = "Sets the session key(s) of the function caller to `keys`."]
				#[doc = "Allows an account to set its session key prior to becoming a validator."]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be signed."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: `O(1)`. Actual cost depends on the number of length of"]
				#[doc = "  `T::Keys::key_ids()` which is fixed."]
				#[doc = "- DbReads: `origin account`, `T::ValidatorIdOf`, `NextKeys`"]
				#[doc = "- DbWrites: `origin account`, `NextKeys`"]
				#[doc = "- DbReads per key id: `KeyOwner`"]
				#[doc = "- DbWrites per key id: `KeyOwner`"]
				#[doc = "# </weight>"]
				pub fn set_keys(
					&self,
					keys: runtime_types::da_runtime::SessionKeys,
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

				#[doc = "Removes any session key(s) of the function caller."]
				#[doc = ""]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
				#[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
				#[doc = "means being a controller account) or directly convertible into a validator ID (which"]
				#[doc = "usually means being a stash account)."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: `O(1)` in number of key types. Actual cost depends on the number of length"]
				#[doc = "  of `T::Keys::key_ids()` which is fixed."]
				#[doc = "- DbReads: `T::ValidatorIdOf`, `NextKeys`, `origin account`"]
				#[doc = "- DbWrites: `NextKeys`, `origin account`"]
				#[doc = "- DbWrites per key id: `KeyOwner`"]
				#[doc = "# </weight>"]
				pub fn purge_keys(&self) -> ::subxt::tx::StaticTxPayload<PurgeKeys> {
					::subxt::tx::StaticTxPayload::new("Session", "purge_keys", PurgeKeys {}, [
						200u8, 255u8, 4u8, 213u8, 188u8, 92u8, 99u8, 116u8, 163u8, 152u8, 29u8,
						35u8, 133u8, 119u8, 246u8, 44u8, 91u8, 31u8, 145u8, 23u8, 213u8, 64u8,
						71u8, 242u8, 207u8, 239u8, 231u8, 37u8, 61u8, 63u8, 190u8, 35u8,
					])
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
							runtime_types::da_runtime::SessionKeys,
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
					::subxt::metadata::DecodeStaticType<runtime_types::da_runtime::SessionKeys>,
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
					::subxt::metadata::DecodeStaticType<runtime_types::da_runtime::SessionKeys>,
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
	pub mod democracy {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
			pub struct Propose {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
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
			pub struct Second {
				#[codec(compact)]
				pub proposal: ::core::primitive::u32,
				#[codec(compact)]
				pub seconds_upper_bound: ::core::primitive::u32,
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
				#[codec(compact)]
				pub ref_index: ::core::primitive::u32,
				pub vote:
					runtime_types::pallet_democracy::vote::AccountVote<::core::primitive::u128>,
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
			pub struct EmergencyCancel {
				pub ref_index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ExternalPropose {
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
			pub struct ExternalProposeMajority {
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
			pub struct ExternalProposeDefault {
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
			pub struct FastTrack {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub voting_period: ::core::primitive::u32,
				pub delay: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct VetoExternal {
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
			pub struct CancelReferendum {
				#[codec(compact)]
				pub ref_index: ::core::primitive::u32,
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
			pub struct CancelQueued {
				pub which: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Delegate {
				pub to: ::subxt::ext::sp_core::crypto::AccountId32,
				pub conviction: runtime_types::pallet_democracy::conviction::Conviction,
				pub balance: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Undelegate;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ClearPublicProposals;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct NotePreimage {
				pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct NotePreimageOperational {
				pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct NoteImminentPreimage {
				pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct NoteImminentPreimageOperational {
				pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ReapPreimage {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				#[codec(compact)]
				pub proposal_len_upper_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Unlock {
				pub target: ::subxt::ext::sp_core::crypto::AccountId32,
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
			pub struct RemoveVote {
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
			pub struct RemoveOtherVote {
				pub target: ::subxt::ext::sp_core::crypto::AccountId32,
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
			pub struct EnactProposal {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
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
			pub struct Blacklist {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct CancelProposal {
				#[codec(compact)]
				pub prop_index: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Propose a sensitive action to be taken."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_ and the sender must"]
				#[doc = "have funds to cover the deposit."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The hash of the proposal preimage."]
				#[doc = "- `value`: The amount of deposit (must be at least `MinimumDeposit`)."]
				#[doc = ""]
				#[doc = "Emits `Proposed`."]
				#[doc = ""]
				#[doc = "Weight: `O(p)`"]
				pub fn propose(
					&self,
					proposal_hash: ::subxt::ext::sp_core::H256,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Propose> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"propose",
						Propose {
							proposal_hash,
							value,
						},
						[
							151u8, 2u8, 117u8, 57u8, 201u8, 246u8, 181u8, 198u8, 83u8, 74u8, 99u8,
							211u8, 237u8, 53u8, 90u8, 173u8, 161u8, 250u8, 139u8, 253u8, 223u8,
							251u8, 39u8, 108u8, 254u8, 192u8, 233u8, 23u8, 9u8, 99u8, 169u8, 195u8,
						],
					)
				}

				#[doc = "Signals agreement with a particular proposal."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_ and the sender"]
				#[doc = "must have funds to cover the deposit, equal to the original deposit."]
				#[doc = ""]
				#[doc = "- `proposal`: The index of the proposal to second."]
				#[doc = "- `seconds_upper_bound`: an upper bound on the current number of seconds on this"]
				#[doc = "  proposal. Extrinsic is weighted according to this value with no refund."]
				#[doc = ""]
				#[doc = "Weight: `O(S)` where S is the number of seconds a proposal already has."]
				pub fn second(
					&self,
					proposal: ::core::primitive::u32,
					seconds_upper_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Second> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"second",
						Second {
							proposal,
							seconds_upper_bound,
						},
						[
							152u8, 56u8, 134u8, 181u8, 88u8, 224u8, 68u8, 238u8, 231u8, 78u8,
							237u8, 142u8, 133u8, 16u8, 93u8, 63u8, 253u8, 81u8, 96u8, 200u8, 43u8,
							21u8, 249u8, 92u8, 78u8, 24u8, 101u8, 217u8, 143u8, 16u8, 213u8, 244u8,
						],
					)
				}

				#[doc = "Vote in a referendum. If `vote.is_aye()`, the vote is to enact the proposal;"]
				#[doc = "otherwise it is a vote to keep the status quo."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `ref_index`: The index of the referendum to vote for."]
				#[doc = "- `vote`: The vote configuration."]
				#[doc = ""]
				#[doc = "Weight: `O(R)` where R is the number of referendums the voter has voted on."]
				pub fn vote(
					&self,
					ref_index: ::core::primitive::u32,
					vote: runtime_types::pallet_democracy::vote::AccountVote<
						::core::primitive::u128,
					>,
				) -> ::subxt::tx::StaticTxPayload<Vote> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"vote",
						Vote { ref_index, vote },
						[
							138u8, 213u8, 229u8, 111u8, 1u8, 191u8, 73u8, 3u8, 145u8, 28u8, 44u8,
							88u8, 163u8, 188u8, 129u8, 188u8, 64u8, 15u8, 64u8, 103u8, 250u8, 97u8,
							234u8, 188u8, 29u8, 205u8, 51u8, 6u8, 116u8, 58u8, 156u8, 201u8,
						],
					)
				}

				#[doc = "Schedule an emergency cancellation of a referendum. Cannot happen twice to the same"]
				#[doc = "referendum."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be `CancellationOrigin`."]
				#[doc = ""]
				#[doc = "-`ref_index`: The index of the referendum to cancel."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`."]
				pub fn emergency_cancel(
					&self,
					ref_index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<EmergencyCancel> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"emergency_cancel",
						EmergencyCancel { ref_index },
						[
							139u8, 213u8, 133u8, 75u8, 34u8, 206u8, 124u8, 245u8, 35u8, 237u8,
							132u8, 92u8, 49u8, 167u8, 117u8, 80u8, 188u8, 93u8, 198u8, 237u8,
							132u8, 77u8, 195u8, 65u8, 29u8, 37u8, 86u8, 74u8, 214u8, 119u8, 71u8,
							204u8,
						],
					)
				}

				#[doc = "Schedule a referendum to be tabled once it is legal to schedule an external"]
				#[doc = "referendum."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be `ExternalOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
				#[doc = ""]
				#[doc = "Weight: `O(V)` with V number of vetoers in the blacklist of proposal."]
				#[doc = "  Decoding vec of length V. Charged as maximum"]
				pub fn external_propose(
					&self,
					proposal_hash: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<ExternalPropose> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"external_propose",
						ExternalPropose { proposal_hash },
						[
							8u8, 206u8, 229u8, 218u8, 203u8, 208u8, 253u8, 113u8, 43u8, 62u8,
							110u8, 155u8, 123u8, 35u8, 187u8, 211u8, 180u8, 225u8, 41u8, 30u8,
							204u8, 110u8, 202u8, 210u8, 143u8, 84u8, 117u8, 20u8, 215u8, 110u8,
							211u8, 89u8,
						],
					)
				}

				#[doc = "Schedule a majority-carries referendum to be tabled next once it is legal to schedule"]
				#[doc = "an external referendum."]
				#[doc = ""]
				#[doc = "The dispatch of this call must be `ExternalMajorityOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
				#[doc = ""]
				#[doc = "Unlike `external_propose`, blacklisting has no effect on this and it may replace a"]
				#[doc = "pre-scheduled `external_propose` call."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn external_propose_majority(
					&self,
					proposal_hash: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<ExternalProposeMajority> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"external_propose_majority",
						ExternalProposeMajority { proposal_hash },
						[
							36u8, 47u8, 192u8, 177u8, 164u8, 82u8, 109u8, 215u8, 98u8, 28u8, 47u8,
							237u8, 159u8, 233u8, 53u8, 9u8, 158u8, 134u8, 232u8, 249u8, 55u8,
							189u8, 48u8, 133u8, 201u8, 46u8, 237u8, 158u8, 181u8, 163u8, 166u8,
							213u8,
						],
					)
				}

				#[doc = "Schedule a negative-turnout-bias referendum to be tabled next once it is legal to"]
				#[doc = "schedule an external referendum."]
				#[doc = ""]
				#[doc = "The dispatch of this call must be `ExternalDefaultOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
				#[doc = ""]
				#[doc = "Unlike `external_propose`, blacklisting has no effect on this and it may replace a"]
				#[doc = "pre-scheduled `external_propose` call."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn external_propose_default(
					&self,
					proposal_hash: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<ExternalProposeDefault> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"external_propose_default",
						ExternalProposeDefault { proposal_hash },
						[
							32u8, 100u8, 249u8, 175u8, 187u8, 77u8, 30u8, 65u8, 90u8, 103u8, 251u8,
							21u8, 21u8, 220u8, 8u8, 118u8, 97u8, 160u8, 152u8, 122u8, 71u8, 140u8,
							96u8, 8u8, 245u8, 74u8, 112u8, 164u8, 55u8, 130u8, 38u8, 14u8,
						],
					)
				}

				#[doc = "Schedule the currently externally-proposed majority-carries referendum to be tabled"]
				#[doc = "immediately. If there is no externally-proposed referendum currently, or if there is one"]
				#[doc = "but it is not a majority-carries referendum then it fails."]
				#[doc = ""]
				#[doc = "The dispatch of this call must be `FastTrackOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The hash of the current external proposal."]
				#[doc = "- `voting_period`: The period that is allowed for voting on this proposal. Increased to"]
				#[doc = "  `FastTrackVotingPeriod` if too low."]
				#[doc = "- `delay`: The number of block after voting has ended in approval and this should be"]
				#[doc = "  enacted. This doesn't have a minimum amount."]
				#[doc = ""]
				#[doc = "Emits `Started`."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn fast_track(
					&self,
					proposal_hash: ::subxt::ext::sp_core::H256,
					voting_period: ::core::primitive::u32,
					delay: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<FastTrack> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"fast_track",
						FastTrack {
							proposal_hash,
							voting_period,
							delay,
						},
						[
							125u8, 209u8, 107u8, 120u8, 93u8, 205u8, 129u8, 147u8, 254u8, 126u8,
							45u8, 126u8, 39u8, 0u8, 56u8, 14u8, 233u8, 49u8, 245u8, 220u8, 156u8,
							10u8, 252u8, 31u8, 102u8, 90u8, 163u8, 236u8, 178u8, 85u8, 13u8, 24u8,
						],
					)
				}

				#[doc = "Veto and blacklist the external proposal hash."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be `VetoOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The preimage hash of the proposal to veto and blacklist."]
				#[doc = ""]
				#[doc = "Emits `Vetoed`."]
				#[doc = ""]
				#[doc = "Weight: `O(V + log(V))` where V is number of `existing vetoers`"]
				pub fn veto_external(
					&self,
					proposal_hash: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<VetoExternal> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"veto_external",
						VetoExternal { proposal_hash },
						[
							209u8, 18u8, 18u8, 103u8, 186u8, 160u8, 214u8, 124u8, 150u8, 207u8,
							112u8, 90u8, 84u8, 197u8, 95u8, 157u8, 165u8, 65u8, 109u8, 101u8, 75u8,
							201u8, 41u8, 149u8, 75u8, 154u8, 37u8, 178u8, 239u8, 121u8, 124u8,
							23u8,
						],
					)
				}

				#[doc = "Remove a referendum."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Root_."]
				#[doc = ""]
				#[doc = "- `ref_index`: The index of the referendum to cancel."]
				#[doc = ""]
				#[doc = "# Weight: `O(1)`."]
				pub fn cancel_referendum(
					&self,
					ref_index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<CancelReferendum> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"cancel_referendum",
						CancelReferendum { ref_index },
						[
							51u8, 25u8, 25u8, 251u8, 236u8, 115u8, 130u8, 230u8, 72u8, 186u8,
							119u8, 71u8, 165u8, 137u8, 55u8, 167u8, 187u8, 128u8, 55u8, 8u8, 212u8,
							139u8, 245u8, 232u8, 103u8, 136u8, 229u8, 113u8, 125u8, 36u8, 1u8,
							149u8,
						],
					)
				}

				#[doc = "Cancel a proposal queued for enactment."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Root_."]
				#[doc = ""]
				#[doc = "- `which`: The index of the referendum to cancel."]
				#[doc = ""]
				#[doc = "Weight: `O(D)` where `D` is the items in the dispatch queue. Weighted as `D = 10`."]
				pub fn cancel_queued(
					&self,
					which: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<CancelQueued> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"cancel_queued",
						CancelQueued { which },
						[
							6u8, 97u8, 182u8, 142u8, 165u8, 206u8, 218u8, 245u8, 206u8, 224u8,
							143u8, 164u8, 232u8, 129u8, 202u8, 141u8, 78u8, 65u8, 79u8, 206u8, 3u8,
							195u8, 151u8, 36u8, 8u8, 220u8, 184u8, 239u8, 28u8, 187u8, 208u8,
							174u8,
						],
					)
				}

				#[doc = "Delegate the voting power (with some given conviction) of the sending account."]
				#[doc = ""]
				#[doc = "The balance delegated is locked for as long as it's delegated, and thereafter for the"]
				#[doc = "time appropriate for the conviction's lock period."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_, and the signing account must either:"]
				#[doc = "  - be delegating already; or"]
				#[doc = "  - have no voting activity (if there is, then it will need to be removed/consolidated"]
				#[doc = "    through `reap_vote` or `unvote`)."]
				#[doc = ""]
				#[doc = "- `to`: The account whose voting the `target` account's voting power will follow."]
				#[doc = "- `conviction`: The conviction that will be attached to the delegated votes. When the"]
				#[doc = "  account is undelegated, the funds will be locked for the corresponding period."]
				#[doc = "- `balance`: The amount of the account's balance to be used in delegating. This must not"]
				#[doc = "  be more than the account's current balance."]
				#[doc = ""]
				#[doc = "Emits `Delegated`."]
				#[doc = ""]
				#[doc = "Weight: `O(R)` where R is the number of referendums the voter delegating to has"]
				#[doc = "  voted on. Weight is charged as if maximum votes."]
				pub fn delegate(
					&self,
					to: ::subxt::ext::sp_core::crypto::AccountId32,
					conviction: runtime_types::pallet_democracy::conviction::Conviction,
					balance: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Delegate> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"delegate",
						Delegate {
							to,
							conviction,
							balance,
						},
						[
							190u8, 241u8, 243u8, 105u8, 114u8, 112u8, 169u8, 52u8, 119u8, 174u8,
							61u8, 72u8, 165u8, 161u8, 192u8, 234u8, 32u8, 144u8, 89u8, 214u8,
							178u8, 227u8, 251u8, 198u8, 129u8, 21u8, 244u8, 183u8, 135u8, 33u8,
							1u8, 224u8,
						],
					)
				}

				#[doc = "Undelegate the voting power of the sending account."]
				#[doc = ""]
				#[doc = "Tokens may be unlocked following once an amount of time consistent with the lock period"]
				#[doc = "of the conviction with which the delegation was issued."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_ and the signing account must be"]
				#[doc = "currently delegating."]
				#[doc = ""]
				#[doc = "Emits `Undelegated`."]
				#[doc = ""]
				#[doc = "Weight: `O(R)` where R is the number of referendums the voter delegating to has"]
				#[doc = "  voted on. Weight is charged as if maximum votes."]
				pub fn undelegate(&self) -> ::subxt::tx::StaticTxPayload<Undelegate> {
					::subxt::tx::StaticTxPayload::new("Democracy", "undelegate", Undelegate {}, [
						165u8, 40u8, 183u8, 209u8, 57u8, 153u8, 111u8, 29u8, 114u8, 109u8, 107u8,
						235u8, 97u8, 61u8, 53u8, 155u8, 44u8, 245u8, 28u8, 220u8, 56u8, 134u8,
						43u8, 122u8, 248u8, 156u8, 191u8, 154u8, 4u8, 121u8, 152u8, 153u8,
					])
				}

				#[doc = "Clears all public proposals."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Root_."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`."]
				pub fn clear_public_proposals(
					&self,
				) -> ::subxt::tx::StaticTxPayload<ClearPublicProposals> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"clear_public_proposals",
						ClearPublicProposals {},
						[
							59u8, 126u8, 254u8, 223u8, 252u8, 225u8, 75u8, 185u8, 188u8, 181u8,
							42u8, 179u8, 211u8, 73u8, 12u8, 141u8, 243u8, 197u8, 46u8, 130u8,
							215u8, 196u8, 225u8, 88u8, 48u8, 199u8, 231u8, 249u8, 195u8, 53u8,
							184u8, 204u8,
						],
					)
				}

				#[doc = "Register the preimage for an upcoming proposal. This doesn't require the proposal to be"]
				#[doc = "in the dispatch queue but does require a deposit, returned once enacted."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `encoded_proposal`: The preimage of a proposal."]
				#[doc = ""]
				#[doc = "Emits `PreimageNoted`."]
				#[doc = ""]
				#[doc = "Weight: `O(E)` with E size of `encoded_proposal` (protected by a required deposit)."]
				pub fn note_preimage(
					&self,
					encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<NotePreimage> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"note_preimage",
						NotePreimage { encoded_proposal },
						[
							31u8, 252u8, 248u8, 238u8, 103u8, 1u8, 82u8, 84u8, 135u8, 152u8, 246u8,
							234u8, 251u8, 124u8, 193u8, 73u8, 52u8, 255u8, 88u8, 31u8, 112u8, 99u8,
							191u8, 245u8, 251u8, 202u8, 51u8, 130u8, 136u8, 114u8, 177u8, 241u8,
						],
					)
				}

				#[doc = "Same as `note_preimage` but origin is `OperationalPreimageOrigin`."]
				pub fn note_preimage_operational(
					&self,
					encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<NotePreimageOperational> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"note_preimage_operational",
						NotePreimageOperational { encoded_proposal },
						[
							184u8, 81u8, 31u8, 172u8, 81u8, 113u8, 84u8, 246u8, 189u8, 219u8,
							167u8, 32u8, 191u8, 126u8, 165u8, 250u8, 147u8, 199u8, 241u8, 196u8,
							253u8, 34u8, 51u8, 158u8, 2u8, 157u8, 16u8, 122u8, 210u8, 66u8, 110u8,
							234u8,
						],
					)
				}

				#[doc = "Register the preimage for an upcoming proposal. This requires the proposal to be"]
				#[doc = "in the dispatch queue. No deposit is needed. When this call is successful, i.e."]
				#[doc = "the preimage has not been uploaded before and matches some imminent proposal,"]
				#[doc = "no fee is paid."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `encoded_proposal`: The preimage of a proposal."]
				#[doc = ""]
				#[doc = "Emits `PreimageNoted`."]
				#[doc = ""]
				#[doc = "Weight: `O(E)` with E size of `encoded_proposal` (protected by a required deposit)."]
				pub fn note_imminent_preimage(
					&self,
					encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<NoteImminentPreimage> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"note_imminent_preimage",
						NoteImminentPreimage { encoded_proposal },
						[
							32u8, 188u8, 10u8, 215u8, 245u8, 132u8, 234u8, 124u8, 19u8, 90u8,
							225u8, 216u8, 169u8, 105u8, 95u8, 231u8, 12u8, 109u8, 16u8, 91u8,
							153u8, 134u8, 240u8, 82u8, 80u8, 254u8, 117u8, 230u8, 88u8, 203u8,
							68u8, 42u8,
						],
					)
				}

				#[doc = "Same as `note_imminent_preimage` but origin is `OperationalPreimageOrigin`."]
				pub fn note_imminent_preimage_operational(
					&self,
					encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<NoteImminentPreimageOperational> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"note_imminent_preimage_operational",
						NoteImminentPreimageOperational { encoded_proposal },
						[
							7u8, 31u8, 49u8, 238u8, 155u8, 234u8, 187u8, 147u8, 123u8, 84u8, 50u8,
							98u8, 221u8, 39u8, 218u8, 204u8, 175u8, 136u8, 44u8, 93u8, 140u8,
							172u8, 73u8, 98u8, 168u8, 110u8, 31u8, 82u8, 22u8, 1u8, 205u8, 84u8,
						],
					)
				}

				#[doc = "Remove an expired proposal preimage and collect the deposit."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The preimage hash of a proposal."]
				#[doc = "- `proposal_length_upper_bound`: an upper bound on length of the proposal. Extrinsic is"]
				#[doc = "  weighted according to this value with no refund."]
				#[doc = ""]
				#[doc = "This will only work after `VotingPeriod` blocks from the time that the preimage was"]
				#[doc = "noted, if it's the same account doing it. If it's a different account, then it'll only"]
				#[doc = "work an additional `EnactmentPeriod` later."]
				#[doc = ""]
				#[doc = "Emits `PreimageReaped`."]
				#[doc = ""]
				#[doc = "Weight: `O(D)` where D is length of proposal."]
				pub fn reap_preimage(
					&self,
					proposal_hash: ::subxt::ext::sp_core::H256,
					proposal_len_upper_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ReapPreimage> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"reap_preimage",
						ReapPreimage {
							proposal_hash,
							proposal_len_upper_bound,
						},
						[
							135u8, 43u8, 115u8, 154u8, 93u8, 121u8, 112u8, 65u8, 145u8, 141u8,
							236u8, 252u8, 203u8, 155u8, 63u8, 130u8, 120u8, 221u8, 13u8, 105u8,
							81u8, 179u8, 167u8, 254u8, 213u8, 117u8, 146u8, 232u8, 18u8, 104u8,
							196u8, 112u8,
						],
					)
				}

				#[doc = "Unlock tokens that have an expired lock."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `target`: The account to remove the lock on."]
				#[doc = ""]
				#[doc = "Weight: `O(R)` with R number of vote of target."]
				pub fn unlock(
					&self,
					target: ::subxt::ext::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<Unlock> {
					::subxt::tx::StaticTxPayload::new("Democracy", "unlock", Unlock { target }, [
						137u8, 93u8, 240u8, 75u8, 142u8, 148u8, 51u8, 55u8, 88u8, 159u8, 2u8, 57u8,
						24u8, 169u8, 120u8, 121u8, 115u8, 53u8, 225u8, 176u8, 67u8, 156u8, 20u8,
						132u8, 39u8, 54u8, 125u8, 203u8, 199u8, 85u8, 60u8, 211u8,
					])
				}

				#[doc = "Remove a vote for a referendum."]
				#[doc = ""]
				#[doc = "If:"]
				#[doc = "- the referendum was cancelled, or"]
				#[doc = "- the referendum is ongoing, or"]
				#[doc = "- the referendum has ended such that"]
				#[doc = "  - the vote of the account was in opposition to the result; or"]
				#[doc = "  - there was no conviction to the account's vote; or"]
				#[doc = "  - the account made a split vote"]
				#[doc = "...then the vote is removed cleanly and a following call to `unlock` may result in more"]
				#[doc = "funds being available."]
				#[doc = ""]
				#[doc = "If, however, the referendum has ended and:"]
				#[doc = "- it finished corresponding to the vote of the account, and"]
				#[doc = "- the account made a standard vote with conviction, and"]
				#[doc = "- the lock period of the conviction is not over"]
				#[doc = "...then the lock will be aggregated into the overall account's lock, which may involve"]
				#[doc = "*overlocking* (where the two locks are combined into a single lock that is the maximum"]
				#[doc = "of both the amount locked and the time is it locked for)."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_, and the signer must have a vote"]
				#[doc = "registered for referendum `index`."]
				#[doc = ""]
				#[doc = "- `index`: The index of referendum of the vote to be removed."]
				#[doc = ""]
				#[doc = "Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on."]
				#[doc = "  Weight is calculated for the maximum number of vote."]
				pub fn remove_vote(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<RemoveVote> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"remove_vote",
						RemoveVote { index },
						[
							148u8, 120u8, 14u8, 172u8, 81u8, 152u8, 159u8, 178u8, 106u8, 244u8,
							36u8, 98u8, 120u8, 189u8, 213u8, 93u8, 119u8, 156u8, 112u8, 34u8,
							241u8, 72u8, 206u8, 113u8, 212u8, 161u8, 164u8, 126u8, 122u8, 82u8,
							160u8, 74u8,
						],
					)
				}

				#[doc = "Remove a vote for a referendum."]
				#[doc = ""]
				#[doc = "If the `target` is equal to the signer, then this function is exactly equivalent to"]
				#[doc = "`remove_vote`. If not equal to the signer, then the vote must have expired,"]
				#[doc = "either because the referendum was cancelled, because the voter lost the referendum or"]
				#[doc = "because the conviction period is over."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `target`: The account of the vote to be removed; this account must have voted for"]
				#[doc = "  referendum `index`."]
				#[doc = "- `index`: The index of referendum of the vote to be removed."]
				#[doc = ""]
				#[doc = "Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on."]
				#[doc = "  Weight is calculated for the maximum number of vote."]
				pub fn remove_other_vote(
					&self,
					target: ::subxt::ext::sp_core::crypto::AccountId32,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<RemoveOtherVote> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"remove_other_vote",
						RemoveOtherVote { target, index },
						[
							137u8, 59u8, 51u8, 72u8, 97u8, 181u8, 74u8, 123u8, 65u8, 147u8, 63u8,
							23u8, 14u8, 6u8, 66u8, 186u8, 105u8, 72u8, 112u8, 120u8, 51u8, 229u8,
							247u8, 96u8, 218u8, 137u8, 220u8, 65u8, 95u8, 109u8, 253u8, 45u8,
						],
					)
				}

				#[doc = "Enact a proposal from a referendum. For now we just make the weight be the maximum."]
				pub fn enact_proposal(
					&self,
					proposal_hash: ::subxt::ext::sp_core::H256,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<EnactProposal> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"enact_proposal",
						EnactProposal {
							proposal_hash,
							index,
						},
						[
							191u8, 244u8, 244u8, 174u8, 95u8, 86u8, 132u8, 63u8, 2u8, 94u8, 3u8,
							117u8, 96u8, 54u8, 100u8, 89u8, 124u8, 117u8, 205u8, 142u8, 214u8,
							192u8, 137u8, 141u8, 178u8, 145u8, 241u8, 167u8, 163u8, 76u8, 61u8,
							31u8,
						],
					)
				}

				#[doc = "Permanently place a proposal into the blacklist. This prevents it from ever being"]
				#[doc = "proposed again."]
				#[doc = ""]
				#[doc = "If called on a queued public or external proposal, then this will result in it being"]
				#[doc = "removed. If the `ref_index` supplied is an active referendum with the proposal hash,"]
				#[doc = "then it will be cancelled."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be `BlacklistOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The proposal hash to blacklist permanently."]
				#[doc = "- `ref_index`: An ongoing referendum whose hash is `proposal_hash`, which will be"]
				#[doc = "cancelled."]
				#[doc = ""]
				#[doc = "Weight: `O(p)` (though as this is an high-privilege dispatch, we assume it has a"]
				#[doc = "  reasonable value)."]
				pub fn blacklist(
					&self,
					proposal_hash: ::subxt::ext::sp_core::H256,
					maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::tx::StaticTxPayload<Blacklist> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"blacklist",
						Blacklist {
							proposal_hash,
							maybe_ref_index,
						},
						[
							48u8, 144u8, 81u8, 164u8, 54u8, 111u8, 197u8, 134u8, 6u8, 98u8, 121u8,
							179u8, 254u8, 191u8, 204u8, 212u8, 84u8, 255u8, 86u8, 110u8, 225u8,
							130u8, 26u8, 65u8, 133u8, 56u8, 231u8, 15u8, 245u8, 137u8, 146u8,
							242u8,
						],
					)
				}

				#[doc = "Remove a proposal."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be `CancelProposalOrigin`."]
				#[doc = ""]
				#[doc = "- `prop_index`: The index of the proposal to cancel."]
				#[doc = ""]
				#[doc = "Weight: `O(p)` where `p = PublicProps::<T>::decode_len()`"]
				pub fn cancel_proposal(
					&self,
					prop_index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<CancelProposal> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"cancel_proposal",
						CancelProposal { prop_index },
						[
							179u8, 3u8, 198u8, 244u8, 241u8, 124u8, 205u8, 58u8, 100u8, 80u8,
							177u8, 254u8, 98u8, 220u8, 189u8, 63u8, 229u8, 60u8, 157u8, 83u8,
							142u8, 6u8, 236u8, 183u8, 193u8, 235u8, 253u8, 126u8, 153u8, 185u8,
							74u8, 117u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_democracy::pallet::Event;
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
			#[doc = "A motion has been proposed by a public account."]
			pub struct Proposed {
				pub proposal_index: ::core::primitive::u32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const EVENT: &'static str = "Proposed";
				const PALLET: &'static str = "Democracy";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A public proposal has been tabled for referendum vote."]
			pub struct Tabled {
				pub proposal_index: ::core::primitive::u32,
				pub deposit: ::core::primitive::u128,
				pub depositors: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for Tabled {
				const EVENT: &'static str = "Tabled";
				const PALLET: &'static str = "Democracy";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An external proposal has been tabled."]
			pub struct ExternalTabled;
			impl ::subxt::events::StaticEvent for ExternalTabled {
				const EVENT: &'static str = "ExternalTabled";
				const PALLET: &'static str = "Democracy";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A referendum has begun."]
			pub struct Started {
				pub ref_index: ::core::primitive::u32,
				pub threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
			}
			impl ::subxt::events::StaticEvent for Started {
				const EVENT: &'static str = "Started";
				const PALLET: &'static str = "Democracy";
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
			#[doc = "A proposal has been approved by referendum."]
			pub struct Passed {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Passed {
				const EVENT: &'static str = "Passed";
				const PALLET: &'static str = "Democracy";
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
			#[doc = "A proposal has been rejected by referendum."]
			pub struct NotPassed {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NotPassed {
				const EVENT: &'static str = "NotPassed";
				const PALLET: &'static str = "Democracy";
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
			#[doc = "A referendum has been cancelled."]
			pub struct Cancelled {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Cancelled {
				const EVENT: &'static str = "Cancelled";
				const PALLET: &'static str = "Democracy";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A proposal has been enacted."]
			pub struct Executed {
				pub ref_index: ::core::primitive::u32,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Executed {
				const EVENT: &'static str = "Executed";
				const PALLET: &'static str = "Democracy";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An account has delegated their vote to another account."]
			pub struct Delegated {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub target: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Delegated {
				const EVENT: &'static str = "Delegated";
				const PALLET: &'static str = "Democracy";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An account has cancelled a previous delegation operation."]
			pub struct Undelegated {
				pub account: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Undelegated {
				const EVENT: &'static str = "Undelegated";
				const PALLET: &'static str = "Democracy";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "An external proposal has been vetoed."]
			pub struct Vetoed {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub until: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Vetoed {
				const EVENT: &'static str = "Vetoed";
				const PALLET: &'static str = "Democracy";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A proposal's preimage was noted, and the deposit taken."]
			pub struct PreimageNoted {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for PreimageNoted {
				const EVENT: &'static str = "PreimageNoted";
				const PALLET: &'static str = "Democracy";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A proposal preimage was removed and used (the deposit was returned)."]
			pub struct PreimageUsed {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub provider: ::subxt::ext::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for PreimageUsed {
				const EVENT: &'static str = "PreimageUsed";
				const PALLET: &'static str = "Democracy";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A proposal could not be executed because its preimage was invalid."]
			pub struct PreimageInvalid {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for PreimageInvalid {
				const EVENT: &'static str = "PreimageInvalid";
				const PALLET: &'static str = "Democracy";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A proposal could not be executed because its preimage was missing."]
			pub struct PreimageMissing {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for PreimageMissing {
				const EVENT: &'static str = "PreimageMissing";
				const PALLET: &'static str = "Democracy";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A registered preimage was removed and the deposit collected by the reaper."]
			pub struct PreimageReaped {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub provider: ::subxt::ext::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
				pub reaper: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for PreimageReaped {
				const EVENT: &'static str = "PreimageReaped";
				const PALLET: &'static str = "Democracy";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A proposal_hash has been blacklisted permanently."]
			pub struct Blacklisted {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
			}
			impl ::subxt::events::StaticEvent for Blacklisted {
				const EVENT: &'static str = "Blacklisted";
				const PALLET: &'static str = "Democracy";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The number of (public) proposals that have been made so far."]
				pub fn public_prop_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"PublicPropCount",
						vec![],
						[
							91u8, 14u8, 171u8, 94u8, 37u8, 157u8, 46u8, 157u8, 254u8, 13u8, 68u8,
							144u8, 23u8, 146u8, 128u8, 159u8, 9u8, 174u8, 74u8, 174u8, 218u8,
							197u8, 23u8, 235u8, 152u8, 226u8, 216u8, 4u8, 120u8, 121u8, 27u8,
							138u8,
						],
					)
				}

				#[doc = " The public proposals. Unsorted. The second item is the proposal's hash."]
				pub fn public_props(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							::core::primitive::u32,
							::subxt::ext::sp_core::H256,
							::subxt::ext::sp_core::crypto::AccountId32,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"PublicProps",
						vec![],
						[
							151u8, 247u8, 196u8, 97u8, 171u8, 230u8, 55u8, 45u8, 220u8, 16u8, 12u8,
							28u8, 22u8, 58u8, 127u8, 179u8, 130u8, 192u8, 115u8, 165u8, 5u8, 173u8,
							87u8, 104u8, 7u8, 186u8, 114u8, 47u8, 162u8, 182u8, 252u8, 154u8,
						],
					)
				}

				#[doc = " Those who have locked a deposit."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: Safe, as increasing integer keys are safe."]
				pub fn deposit_of(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						::core::primitive::u128,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"DepositOf",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							153u8, 236u8, 152u8, 224u8, 221u8, 90u8, 204u8, 183u8, 222u8, 160u8,
							227u8, 26u8, 8u8, 110u8, 230u8, 102u8, 133u8, 186u8, 66u8, 2u8, 84u8,
							31u8, 236u8, 228u8, 202u8, 75u8, 17u8, 97u8, 133u8, 232u8, 64u8, 7u8,
						],
					)
				}

				#[doc = " Those who have locked a deposit."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: Safe, as increasing integer keys are safe."]
				pub fn deposit_of_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						::core::primitive::u128,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"DepositOf",
						Vec::new(),
						[
							153u8, 236u8, 152u8, 224u8, 221u8, 90u8, 204u8, 183u8, 222u8, 160u8,
							227u8, 26u8, 8u8, 110u8, 230u8, 102u8, 133u8, 186u8, 66u8, 2u8, 84u8,
							31u8, 236u8, 228u8, 202u8, 75u8, 17u8, 97u8, 133u8, 232u8, 64u8, 7u8,
						],
					)
				}

				#[doc = " Map of hashes to the proposal preimage, along with who registered it and their deposit."]
				#[doc = " The block number is the block at which it was deposited."]
				pub fn preimages(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_democracy::PreimageStatus<
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
						"Democracy",
						"Preimages",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							206u8, 131u8, 7u8, 129u8, 172u8, 231u8, 164u8, 220u8, 129u8, 0u8,
							204u8, 227u8, 231u8, 244u8, 61u8, 145u8, 144u8, 146u8, 173u8, 215u8,
							174u8, 218u8, 192u8, 83u8, 174u8, 99u8, 87u8, 102u8, 98u8, 235u8,
							138u8, 127u8,
						],
					)
				}

				#[doc = " Map of hashes to the proposal preimage, along with who registered it and their deposit."]
				#[doc = " The block number is the block at which it was deposited."]
				pub fn preimages_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_democracy::PreimageStatus<
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
						"Democracy",
						"Preimages",
						Vec::new(),
						[
							206u8, 131u8, 7u8, 129u8, 172u8, 231u8, 164u8, 220u8, 129u8, 0u8,
							204u8, 227u8, 231u8, 244u8, 61u8, 145u8, 144u8, 146u8, 173u8, 215u8,
							174u8, 218u8, 192u8, 83u8, 174u8, 99u8, 87u8, 102u8, 98u8, 235u8,
							138u8, 127u8,
						],
					)
				}

				#[doc = " The next free referendum index, aka the number of referenda started so far."]
				pub fn referendum_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"ReferendumCount",
						vec![],
						[
							153u8, 210u8, 106u8, 244u8, 156u8, 70u8, 124u8, 251u8, 123u8, 75u8,
							7u8, 189u8, 199u8, 145u8, 95u8, 119u8, 137u8, 11u8, 240u8, 160u8,
							151u8, 248u8, 229u8, 231u8, 89u8, 222u8, 18u8, 237u8, 144u8, 78u8,
							99u8, 58u8,
						],
					)
				}

				#[doc = " The lowest referendum index representing an unbaked referendum. Equal to"]
				#[doc = " `ReferendumCount` if there isn't a unbaked referendum."]
				pub fn lowest_unbaked(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"LowestUnbaked",
						vec![],
						[
							4u8, 51u8, 108u8, 11u8, 48u8, 165u8, 19u8, 251u8, 182u8, 76u8, 163u8,
							73u8, 227u8, 2u8, 212u8, 74u8, 128u8, 27u8, 165u8, 164u8, 111u8, 22u8,
							209u8, 190u8, 103u8, 7u8, 116u8, 16u8, 160u8, 144u8, 123u8, 64u8,
						],
					)
				}

				#[doc = " Information concerning any given referendum."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE as indexes are not under an attacker’s control."]
				pub fn referendum_info_of(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_democracy::types::ReferendumInfo<
							::core::primitive::u32,
							::subxt::ext::sp_core::H256,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"ReferendumInfoOf",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							132u8, 4u8, 108u8, 126u8, 91u8, 168u8, 18u8, 17u8, 86u8, 79u8, 219u8,
							222u8, 195u8, 137u8, 149u8, 177u8, 101u8, 134u8, 130u8, 41u8, 217u8,
							109u8, 18u8, 18u8, 33u8, 206u8, 117u8, 131u8, 98u8, 26u8, 51u8, 8u8,
						],
					)
				}

				#[doc = " Information concerning any given referendum."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE as indexes are not under an attacker’s control."]
				pub fn referendum_info_of_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_democracy::types::ReferendumInfo<
							::core::primitive::u32,
							::subxt::ext::sp_core::H256,
							::core::primitive::u128,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"ReferendumInfoOf",
						Vec::new(),
						[
							132u8, 4u8, 108u8, 126u8, 91u8, 168u8, 18u8, 17u8, 86u8, 79u8, 219u8,
							222u8, 195u8, 137u8, 149u8, 177u8, 101u8, 134u8, 130u8, 41u8, 217u8,
							109u8, 18u8, 18u8, 33u8, 206u8, 117u8, 131u8, 98u8, 26u8, 51u8, 8u8,
						],
					)
				}

				#[doc = " All votes for a particular voter. We store the balance for the number of votes that we"]
				#[doc = " have recorded. The second item is the total amount of delegations, that will be added."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE as `AccountId`s are crypto hashes anyway."]
				pub fn voting_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_democracy::vote::Voting<
							::core::primitive::u128,
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"VotingOf",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							211u8, 38u8, 232u8, 65u8, 215u8, 97u8, 157u8, 208u8, 177u8, 150u8,
							250u8, 226u8, 72u8, 185u8, 187u8, 162u8, 80u8, 67u8, 195u8, 87u8,
							190u8, 180u8, 167u8, 137u8, 253u8, 142u8, 34u8, 158u8, 249u8, 168u8,
							209u8, 18u8,
						],
					)
				}

				#[doc = " All votes for a particular voter. We store the balance for the number of votes that we"]
				#[doc = " have recorded. The second item is the total amount of delegations, that will be added."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE as `AccountId`s are crypto hashes anyway."]
				pub fn voting_of_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_democracy::vote::Voting<
							::core::primitive::u128,
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"VotingOf",
						Vec::new(),
						[
							211u8, 38u8, 232u8, 65u8, 215u8, 97u8, 157u8, 208u8, 177u8, 150u8,
							250u8, 226u8, 72u8, 185u8, 187u8, 162u8, 80u8, 67u8, 195u8, 87u8,
							190u8, 180u8, 167u8, 137u8, 253u8, 142u8, 34u8, 158u8, 249u8, 168u8,
							209u8, 18u8,
						],
					)
				}

				#[doc = " Accounts for which there are locks in action which may be removed at some point in the"]
				#[doc = " future. The value is the block number at which the lock expires and may be removed."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]
				pub fn locks(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"Locks",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							115u8, 224u8, 3u8, 18u8, 7u8, 229u8, 114u8, 252u8, 184u8, 233u8, 95u8,
							3u8, 237u8, 104u8, 174u8, 21u8, 35u8, 26u8, 208u8, 8u8, 71u8, 56u8,
							1u8, 33u8, 200u8, 159u8, 187u8, 204u8, 225u8, 254u8, 227u8, 4u8,
						],
					)
				}

				#[doc = " Accounts for which there are locks in action which may be removed at some point in the"]
				#[doc = " future. The value is the block number at which the lock expires and may be removed."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]
				pub fn locks_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"Locks",
						Vec::new(),
						[
							115u8, 224u8, 3u8, 18u8, 7u8, 229u8, 114u8, 252u8, 184u8, 233u8, 95u8,
							3u8, 237u8, 104u8, 174u8, 21u8, 35u8, 26u8, 208u8, 8u8, 71u8, 56u8,
							1u8, 33u8, 200u8, 159u8, 187u8, 204u8, 225u8, 254u8, 227u8, 4u8,
						],
					)
				}

				#[doc = " True if the last referendum tabled was submitted externally. False if it was a public"]
				#[doc = " proposal."]
				pub fn last_tabled_was_external(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"LastTabledWasExternal",
						vec![],
						[
							3u8, 67u8, 106u8, 1u8, 89u8, 204u8, 4u8, 145u8, 121u8, 44u8, 34u8,
							76u8, 18u8, 206u8, 65u8, 214u8, 222u8, 82u8, 31u8, 223u8, 144u8, 169u8,
							17u8, 6u8, 138u8, 36u8, 113u8, 155u8, 241u8, 106u8, 189u8, 218u8,
						],
					)
				}

				#[doc = " The referendum to be tabled whenever it would be valid to table an external proposal."]
				#[doc = " This happens when a referendum needs to be tabled and one of two conditions are met:"]
				#[doc = " - `LastTabledWasExternal` is `false`; or"]
				#[doc = " - `PublicProps` is empty."]
				pub fn next_external(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::subxt::ext::sp_core::H256,
						runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					)>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"NextExternal",
						vec![],
						[
							123u8, 49u8, 252u8, 184u8, 75u8, 204u8, 16u8, 130u8, 43u8, 109u8, 62u8,
							113u8, 95u8, 0u8, 20u8, 163u8, 186u8, 210u8, 253u8, 33u8, 58u8, 121u8,
							36u8, 80u8, 9u8, 242u8, 180u8, 230u8, 167u8, 250u8, 32u8, 180u8,
						],
					)
				}

				#[doc = " A record of who vetoed what. Maps proposal hash to a possible existent block number"]
				#[doc = " (until when it may not be resubmitted) and who vetoed it."]
				pub fn blacklist(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"Blacklist",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							93u8, 165u8, 219u8, 135u8, 41u8, 114u8, 144u8, 133u8, 171u8, 83u8,
							153u8, 157u8, 79u8, 14u8, 170u8, 29u8, 179u8, 23u8, 222u8, 124u8,
							237u8, 253u8, 122u8, 21u8, 186u8, 209u8, 184u8, 89u8, 197u8, 5u8,
							178u8, 255u8,
						],
					)
				}

				#[doc = " A record of who vetoed what. Maps proposal hash to a possible existent block number"]
				#[doc = " (until when it may not be resubmitted) and who vetoed it."]
				pub fn blacklist_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"Blacklist",
						Vec::new(),
						[
							93u8, 165u8, 219u8, 135u8, 41u8, 114u8, 144u8, 133u8, 171u8, 83u8,
							153u8, 157u8, 79u8, 14u8, 170u8, 29u8, 179u8, 23u8, 222u8, 124u8,
							237u8, 253u8, 122u8, 21u8, 186u8, 209u8, 184u8, 89u8, 197u8, 5u8,
							178u8, 255u8,
						],
					)
				}

				#[doc = " Record of all proposals that have been subject to emergency cancellation."]
				pub fn cancellations(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"Cancellations",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							154u8, 36u8, 172u8, 46u8, 65u8, 218u8, 30u8, 151u8, 173u8, 186u8,
							166u8, 79u8, 35u8, 226u8, 94u8, 200u8, 67u8, 44u8, 47u8, 7u8, 17u8,
							89u8, 169u8, 166u8, 236u8, 101u8, 68u8, 54u8, 114u8, 141u8, 177u8,
							135u8,
						],
					)
				}

				#[doc = " Record of all proposals that have been subject to emergency cancellation."]
				pub fn cancellations_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"Cancellations",
						Vec::new(),
						[
							154u8, 36u8, 172u8, 46u8, 65u8, 218u8, 30u8, 151u8, 173u8, 186u8,
							166u8, 79u8, 35u8, 226u8, 94u8, 200u8, 67u8, 44u8, 47u8, 7u8, 17u8,
							89u8, 169u8, 166u8, 236u8, 101u8, 68u8, 54u8, 114u8, 141u8, 177u8,
							135u8,
						],
					)
				}

				#[doc = " Storage version of the pallet."]
				#[doc = ""]
				#[doc = " New networks start with last version."]
				pub fn storage_version(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::pallet_democracy::Releases>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"StorageVersion",
						vec![],
						[
							39u8, 219u8, 134u8, 64u8, 250u8, 96u8, 95u8, 156u8, 100u8, 236u8, 18u8,
							78u8, 59u8, 146u8, 5u8, 245u8, 113u8, 125u8, 220u8, 140u8, 125u8, 5u8,
							194u8, 134u8, 248u8, 95u8, 250u8, 108u8, 142u8, 230u8, 21u8, 120u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The period between a proposal being approved and enacted."]
				#[doc = ""]
				#[doc = " It should generally be a little more than the unstake period to ensure that"]
				#[doc = " voting stakers have an opportunity to remove themselves from the system in the case"]
				#[doc = " where they are on the losing side of a vote."]
				pub fn enactment_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Democracy",
						"EnactmentPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " How often (in blocks) new public referenda are launched."]
				pub fn launch_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Democracy", "LaunchPeriod", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " How often (in blocks) to check for new votes."]
				pub fn voting_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Democracy", "VotingPeriod", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " The minimum period of vote locking."]
				#[doc = ""]
				#[doc = " It should be no shorter than enactment period to ensure that in the case of an approval,"]
				#[doc = " those successful voters are locked into the consequences that their votes entail."]
				pub fn vote_locking_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Democracy",
						"VoteLockingPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " The minimum amount to be used as a deposit for a public referendum proposal."]
				pub fn minimum_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new("Democracy", "MinimumDeposit", [
						84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
						27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
						71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
					])
				}

				#[doc = " Indicator for whether an emergency origin is even allowed to happen. Some chains may"]
				#[doc = " want to set this permanently to `false`, others may want to condition it on things such"]
				#[doc = " as an upgrade having happened recently."]
				pub fn instant_allowed(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
				> {
					::subxt::constants::StaticConstantAddress::new("Democracy", "InstantAllowed", [
						165u8, 28u8, 112u8, 190u8, 18u8, 129u8, 182u8, 206u8, 237u8, 1u8, 68u8,
						252u8, 125u8, 234u8, 185u8, 50u8, 149u8, 164u8, 47u8, 126u8, 134u8, 100u8,
						14u8, 86u8, 209u8, 39u8, 20u8, 4u8, 233u8, 115u8, 102u8, 131u8,
					])
				}

				#[doc = " Minimum voting period allowed for a fast-track referendum."]
				pub fn fast_track_voting_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Democracy",
						"FastTrackVotingPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " Period in blocks where an external proposal may not be re-submitted after being vetoed."]
				pub fn cooloff_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Democracy", "CooloffPeriod", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " The amount of balance that must be deposited per byte of preimage stored."]
				pub fn preimage_byte_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Democracy",
						"PreimageByteDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}

				#[doc = " The maximum number of votes for an account."]
				#[doc = ""]
				#[doc = " Also used to compute weight, an overly big value can"]
				#[doc = " lead to extrinsic with very big weight: see `delegate` for instance."]
				pub fn max_votes(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Democracy", "MaxVotes", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " The maximum number of public proposals that can exist at any time."]
				pub fn max_proposals(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Democracy", "MaxProposals", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}
			}
		}
	}
	pub mod council {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				pub proposal: ::std::boxed::Box<runtime_types::da_runtime::Call>,
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
				pub proposal: ::std::boxed::Box<runtime_types::da_runtime::Call>,
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
			pub struct Close {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				#[codec(compact)]
				pub proposal_weight_bound: ::core::primitive::u64,
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
			pub struct DisapproveProposal {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the collective's membership."]
				#[doc = ""]
				#[doc = "- `new_members`: The new member list. Be nice to the chain and provide it sorted."]
				#[doc = "- `prime`: The prime member whose vote sets the default."]
				#[doc = "- `old_count`: The upper bound for the previous number of members in storage. Used for"]
				#[doc = "  weight estimation."]
				#[doc = ""]
				#[doc = "Requires root origin."]
				#[doc = ""]
				#[doc = "NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but"]
				#[doc = "      the weight estimations rely on it to estimate dispatchable weight."]
				#[doc = ""]
				#[doc = "# WARNING:"]
				#[doc = ""]
				#[doc = "The `pallet-collective` can also be managed by logic outside of the pallet through the"]
				#[doc = "implementation of the trait [`ChangeMembers`]."]
				#[doc = "Any call to `set_members` must be careful that the member set doesn't get out of sync"]
				#[doc = "with other logic managing the member set."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(MP + N)` where:"]
				#[doc = "  - `M` old-members-count (code- and governance-bounded)"]
				#[doc = "  - `N` new-members-count (code- and governance-bounded)"]
				#[doc = "  - `P` proposals-count (code-bounded)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage mutation (codec `O(M)` read, `O(N)` write) for reading and writing the"]
				#[doc = "    members"]
				#[doc = "  - 1 storage read (codec `O(P)`) for reading the proposals"]
				#[doc = "  - `P` storage mutations (codec `O(M)`) for updating the votes for each proposal"]
				#[doc = "  - 1 storage write (codec `O(1)`) for deleting the old `prime` and setting the new one"]
				#[doc = "# </weight>"]
				pub fn set_members(
					&self,
					new_members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					prime: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
					old_count: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetMembers> {
					::subxt::tx::StaticTxPayload::new(
						"Council",
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

				#[doc = "Dispatch a proposal from a member using the `Member` origin."]
				#[doc = ""]
				#[doc = "Origin must be a member of the collective."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(M + P)` where `M` members-count (code-bounded) and `P` complexity of dispatching"]
				#[doc = "  `proposal`"]
				#[doc = "- DB: 1 read (codec `O(M)`) + DB access of `proposal`"]
				#[doc = "- 1 event"]
				#[doc = "# </weight>"]
				pub fn execute(
					&self,
					proposal: runtime_types::da_runtime::Call,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Execute> {
					::subxt::tx::StaticTxPayload::new(
						"Council",
						"execute",
						Execute {
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						},
						[
							194u8, 57u8, 78u8, 249u8, 175u8, 56u8, 214u8, 240u8, 21u8, 83u8, 15u8,
							70u8, 78u8, 196u8, 228u8, 31u8, 2u8, 0u8, 184u8, 22u8, 44u8, 40u8,
							161u8, 188u8, 74u8, 216u8, 31u8, 29u8, 59u8, 125u8, 168u8, 129u8,
						],
					)
				}

				#[doc = "Add a new proposal to either be voted on or executed directly."]
				#[doc = ""]
				#[doc = "Requires the sender to be member."]
				#[doc = ""]
				#[doc = "`threshold` determines whether `proposal` is executed directly (`threshold < 2`)"]
				#[doc = "or put up for voting."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(B + M + P1)` or `O(B + M + P2)` where:"]
				#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
				#[doc = "  - `M` is members-count (code- and governance-bounded)"]
				#[doc = "  - branching is influenced by `threshold` where:"]
				#[doc = "    - `P1` is proposal execution complexity (`threshold < 2`)"]
				#[doc = "    - `P2` is proposals-count (code-bounded) (`threshold >= 2`)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage read `is_member` (codec `O(M)`)"]
				#[doc = "  - 1 storage read `ProposalOf::contains_key` (codec `O(1)`)"]
				#[doc = "  - DB accesses influenced by `threshold`:"]
				#[doc = "    - EITHER storage accesses done by `proposal` (`threshold < 2`)"]
				#[doc = "    - OR proposal insertion (`threshold <= 2`)"]
				#[doc = "      - 1 storage mutation `Proposals` (codec `O(P2)`)"]
				#[doc = "      - 1 storage mutation `ProposalCount` (codec `O(1)`)"]
				#[doc = "      - 1 storage write `ProposalOf` (codec `O(B)`)"]
				#[doc = "      - 1 storage write `Voting` (codec `O(M)`)"]
				#[doc = "  - 1 event"]
				#[doc = "# </weight>"]
				pub fn propose(
					&self,
					threshold: ::core::primitive::u32,
					proposal: runtime_types::da_runtime::Call,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Propose> {
					::subxt::tx::StaticTxPayload::new(
						"Council",
						"propose",
						Propose {
							threshold,
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						},
						[
							56u8, 153u8, 239u8, 134u8, 65u8, 98u8, 107u8, 29u8, 237u8, 143u8,
							248u8, 49u8, 115u8, 128u8, 205u8, 105u8, 199u8, 235u8, 214u8, 150u8,
							123u8, 72u8, 120u8, 104u8, 217u8, 27u8, 182u8, 210u8, 195u8, 248u8,
							68u8, 118u8,
						],
					)
				}

				#[doc = "Add an aye or nay vote for the sender to the given proposal."]
				#[doc = ""]
				#[doc = "Requires the sender to be a member."]
				#[doc = ""]
				#[doc = "Transaction fees will be waived if the member is voting on any particular proposal"]
				#[doc = "for the first time and the call is successful. Subsequent vote changes will charge a"]
				#[doc = "fee."]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(M)` where `M` is members-count (code- and governance-bounded)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage read `Members` (codec `O(M)`)"]
				#[doc = "  - 1 storage mutation `Voting` (codec `O(M)`)"]
				#[doc = "- 1 event"]
				#[doc = "# </weight>"]
				pub fn vote(
					&self,
					proposal: ::subxt::ext::sp_core::H256,
					index: ::core::primitive::u32,
					approve: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<Vote> {
					::subxt::tx::StaticTxPayload::new(
						"Council",
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

				#[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
				#[doc = ""]
				#[doc = "May be called by any signed account in order to finish voting and close the proposal."]
				#[doc = ""]
				#[doc = "If called before the end of the voting period it will only close the vote if it is"]
				#[doc = "has enough votes to be approved or disapproved."]
				#[doc = ""]
				#[doc = "If called after the end of the voting period abstentions are counted as rejections"]
				#[doc = "unless there is a prime member set and the prime member cast an approval."]
				#[doc = ""]
				#[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
				#[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
				#[doc = ""]
				#[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
				#[doc = "proposal."]
				#[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
				#[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(B + M + P1 + P2)` where:"]
				#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
				#[doc = "  - `M` is members-count (code- and governance-bounded)"]
				#[doc = "  - `P1` is the complexity of `proposal` preimage."]
				#[doc = "  - `P2` is proposal-count (code-bounded)"]
				#[doc = "- DB:"]
				#[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
				#[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
				#[doc = "   `O(P2)`)"]
				#[doc = " - any mutations done while executing `proposal` (`P1`)"]
				#[doc = "- up to 3 events"]
				#[doc = "# </weight>"]
				pub fn close(
					&self,
					proposal_hash: ::subxt::ext::sp_core::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: ::core::primitive::u64,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Close> {
					::subxt::tx::StaticTxPayload::new(
						"Council",
						"close",
						Close {
							proposal_hash,
							index,
							proposal_weight_bound,
							length_bound,
						},
						[
							88u8, 8u8, 33u8, 184u8, 4u8, 97u8, 120u8, 237u8, 43u8, 183u8, 130u8,
							139u8, 65u8, 74u8, 166u8, 119u8, 246u8, 65u8, 132u8, 219u8, 118u8,
							69u8, 182u8, 195u8, 111u8, 204u8, 107u8, 78u8, 152u8, 218u8, 181u8,
							208u8,
						],
					)
				}

				#[doc = "Disapprove a proposal, close, and remove it from the system, regardless of its current"]
				#[doc = "state."]
				#[doc = ""]
				#[doc = "Must be called by the Root origin."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "* `proposal_hash`: The hash of the proposal that should be disapproved."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Complexity: O(P) where P is the number of max proposals"]
				#[doc = "DB Weight:"]
				#[doc = "* Reads: Proposals"]
				#[doc = "* Writes: Voting, Proposals, ProposalOf"]
				#[doc = "# </weight>"]
				pub fn disapprove_proposal(
					&self,
					proposal_hash: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<DisapproveProposal> {
					::subxt::tx::StaticTxPayload::new(
						"Council",
						"disapprove_proposal",
						DisapproveProposal { proposal_hash },
						[
							25u8, 123u8, 1u8, 8u8, 74u8, 37u8, 3u8, 40u8, 97u8, 37u8, 175u8, 224u8,
							72u8, 155u8, 123u8, 109u8, 104u8, 43u8, 91u8, 125u8, 199u8, 51u8, 17u8,
							225u8, 133u8, 38u8, 120u8, 76u8, 164u8, 5u8, 194u8, 201u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
				const PALLET: &'static str = "Council";
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
				const PALLET: &'static str = "Council";
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
				const PALLET: &'static str = "Council";
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
				const PALLET: &'static str = "Council";
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
				const PALLET: &'static str = "Council";
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
				const PALLET: &'static str = "Council";
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
				const PALLET: &'static str = "Council";
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
						runtime_types::frame_support::storage::bounded_vec::BoundedVec<
							::subxt::ext::sp_core::H256,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Council",
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
					::subxt::metadata::DecodeStaticType<runtime_types::da_runtime::Call>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Council",
						"ProposalOf",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							196u8, 81u8, 145u8, 33u8, 105u8, 200u8, 14u8, 212u8, 127u8, 142u8,
							201u8, 45u8, 147u8, 150u8, 136u8, 251u8, 36u8, 67u8, 24u8, 42u8, 26u8,
							8u8, 81u8, 146u8, 88u8, 110u8, 104u8, 49u8, 156u8, 41u8, 223u8, 11u8,
						],
					)
				}

				#[doc = " Actual proposal for a given hash, if it's current."]
				pub fn proposal_of_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::da_runtime::Call>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Council",
						"ProposalOf",
						Vec::new(),
						[
							196u8, 81u8, 145u8, 33u8, 105u8, 200u8, 14u8, 212u8, 127u8, 142u8,
							201u8, 45u8, 147u8, 150u8, 136u8, 251u8, 36u8, 67u8, 24u8, 42u8, 26u8,
							8u8, 81u8, 146u8, 88u8, 110u8, 104u8, 49u8, 156u8, 41u8, 223u8, 11u8,
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
						"Council",
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
						"Council",
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
						"Council",
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
						"Council",
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
						"Council",
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
	pub mod technical_committee {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				pub proposal: ::std::boxed::Box<runtime_types::da_runtime::Call>,
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
				pub proposal: ::std::boxed::Box<runtime_types::da_runtime::Call>,
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
			pub struct Close {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				#[codec(compact)]
				pub proposal_weight_bound: ::core::primitive::u64,
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
			pub struct DisapproveProposal {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the collective's membership."]
				#[doc = ""]
				#[doc = "- `new_members`: The new member list. Be nice to the chain and provide it sorted."]
				#[doc = "- `prime`: The prime member whose vote sets the default."]
				#[doc = "- `old_count`: The upper bound for the previous number of members in storage. Used for"]
				#[doc = "  weight estimation."]
				#[doc = ""]
				#[doc = "Requires root origin."]
				#[doc = ""]
				#[doc = "NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but"]
				#[doc = "      the weight estimations rely on it to estimate dispatchable weight."]
				#[doc = ""]
				#[doc = "# WARNING:"]
				#[doc = ""]
				#[doc = "The `pallet-collective` can also be managed by logic outside of the pallet through the"]
				#[doc = "implementation of the trait [`ChangeMembers`]."]
				#[doc = "Any call to `set_members` must be careful that the member set doesn't get out of sync"]
				#[doc = "with other logic managing the member set."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(MP + N)` where:"]
				#[doc = "  - `M` old-members-count (code- and governance-bounded)"]
				#[doc = "  - `N` new-members-count (code- and governance-bounded)"]
				#[doc = "  - `P` proposals-count (code-bounded)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage mutation (codec `O(M)` read, `O(N)` write) for reading and writing the"]
				#[doc = "    members"]
				#[doc = "  - 1 storage read (codec `O(P)`) for reading the proposals"]
				#[doc = "  - `P` storage mutations (codec `O(M)`) for updating the votes for each proposal"]
				#[doc = "  - 1 storage write (codec `O(1)`) for deleting the old `prime` and setting the new one"]
				#[doc = "# </weight>"]
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

				#[doc = "Dispatch a proposal from a member using the `Member` origin."]
				#[doc = ""]
				#[doc = "Origin must be a member of the collective."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(M + P)` where `M` members-count (code-bounded) and `P` complexity of dispatching"]
				#[doc = "  `proposal`"]
				#[doc = "- DB: 1 read (codec `O(M)`) + DB access of `proposal`"]
				#[doc = "- 1 event"]
				#[doc = "# </weight>"]
				pub fn execute(
					&self,
					proposal: runtime_types::da_runtime::Call,
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
							194u8, 57u8, 78u8, 249u8, 175u8, 56u8, 214u8, 240u8, 21u8, 83u8, 15u8,
							70u8, 78u8, 196u8, 228u8, 31u8, 2u8, 0u8, 184u8, 22u8, 44u8, 40u8,
							161u8, 188u8, 74u8, 216u8, 31u8, 29u8, 59u8, 125u8, 168u8, 129u8,
						],
					)
				}

				#[doc = "Add a new proposal to either be voted on or executed directly."]
				#[doc = ""]
				#[doc = "Requires the sender to be member."]
				#[doc = ""]
				#[doc = "`threshold` determines whether `proposal` is executed directly (`threshold < 2`)"]
				#[doc = "or put up for voting."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(B + M + P1)` or `O(B + M + P2)` where:"]
				#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
				#[doc = "  - `M` is members-count (code- and governance-bounded)"]
				#[doc = "  - branching is influenced by `threshold` where:"]
				#[doc = "    - `P1` is proposal execution complexity (`threshold < 2`)"]
				#[doc = "    - `P2` is proposals-count (code-bounded) (`threshold >= 2`)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage read `is_member` (codec `O(M)`)"]
				#[doc = "  - 1 storage read `ProposalOf::contains_key` (codec `O(1)`)"]
				#[doc = "  - DB accesses influenced by `threshold`:"]
				#[doc = "    - EITHER storage accesses done by `proposal` (`threshold < 2`)"]
				#[doc = "    - OR proposal insertion (`threshold <= 2`)"]
				#[doc = "      - 1 storage mutation `Proposals` (codec `O(P2)`)"]
				#[doc = "      - 1 storage mutation `ProposalCount` (codec `O(1)`)"]
				#[doc = "      - 1 storage write `ProposalOf` (codec `O(B)`)"]
				#[doc = "      - 1 storage write `Voting` (codec `O(M)`)"]
				#[doc = "  - 1 event"]
				#[doc = "# </weight>"]
				pub fn propose(
					&self,
					threshold: ::core::primitive::u32,
					proposal: runtime_types::da_runtime::Call,
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
							56u8, 153u8, 239u8, 134u8, 65u8, 98u8, 107u8, 29u8, 237u8, 143u8,
							248u8, 49u8, 115u8, 128u8, 205u8, 105u8, 199u8, 235u8, 214u8, 150u8,
							123u8, 72u8, 120u8, 104u8, 217u8, 27u8, 182u8, 210u8, 195u8, 248u8,
							68u8, 118u8,
						],
					)
				}

				#[doc = "Add an aye or nay vote for the sender to the given proposal."]
				#[doc = ""]
				#[doc = "Requires the sender to be a member."]
				#[doc = ""]
				#[doc = "Transaction fees will be waived if the member is voting on any particular proposal"]
				#[doc = "for the first time and the call is successful. Subsequent vote changes will charge a"]
				#[doc = "fee."]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(M)` where `M` is members-count (code- and governance-bounded)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage read `Members` (codec `O(M)`)"]
				#[doc = "  - 1 storage mutation `Voting` (codec `O(M)`)"]
				#[doc = "- 1 event"]
				#[doc = "# </weight>"]
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

				#[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
				#[doc = ""]
				#[doc = "May be called by any signed account in order to finish voting and close the proposal."]
				#[doc = ""]
				#[doc = "If called before the end of the voting period it will only close the vote if it is"]
				#[doc = "has enough votes to be approved or disapproved."]
				#[doc = ""]
				#[doc = "If called after the end of the voting period abstentions are counted as rejections"]
				#[doc = "unless there is a prime member set and the prime member cast an approval."]
				#[doc = ""]
				#[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
				#[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
				#[doc = ""]
				#[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
				#[doc = "proposal."]
				#[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
				#[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(B + M + P1 + P2)` where:"]
				#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
				#[doc = "  - `M` is members-count (code- and governance-bounded)"]
				#[doc = "  - `P1` is the complexity of `proposal` preimage."]
				#[doc = "  - `P2` is proposal-count (code-bounded)"]
				#[doc = "- DB:"]
				#[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
				#[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
				#[doc = "   `O(P2)`)"]
				#[doc = " - any mutations done while executing `proposal` (`P1`)"]
				#[doc = "- up to 3 events"]
				#[doc = "# </weight>"]
				pub fn close(
					&self,
					proposal_hash: ::subxt::ext::sp_core::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: ::core::primitive::u64,
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
							88u8, 8u8, 33u8, 184u8, 4u8, 97u8, 120u8, 237u8, 43u8, 183u8, 130u8,
							139u8, 65u8, 74u8, 166u8, 119u8, 246u8, 65u8, 132u8, 219u8, 118u8,
							69u8, 182u8, 195u8, 111u8, 204u8, 107u8, 78u8, 152u8, 218u8, 181u8,
							208u8,
						],
					)
				}

				#[doc = "Disapprove a proposal, close, and remove it from the system, regardless of its current"]
				#[doc = "state."]
				#[doc = ""]
				#[doc = "Must be called by the Root origin."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "* `proposal_hash`: The hash of the proposal that should be disapproved."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Complexity: O(P) where P is the number of max proposals"]
				#[doc = "DB Weight:"]
				#[doc = "* Reads: Proposals"]
				#[doc = "* Writes: Voting, Proposals, ProposalOf"]
				#[doc = "# </weight>"]
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
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
						runtime_types::frame_support::storage::bounded_vec::BoundedVec<
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
					::subxt::metadata::DecodeStaticType<runtime_types::da_runtime::Call>,
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
							196u8, 81u8, 145u8, 33u8, 105u8, 200u8, 14u8, 212u8, 127u8, 142u8,
							201u8, 45u8, 147u8, 150u8, 136u8, 251u8, 36u8, 67u8, 24u8, 42u8, 26u8,
							8u8, 81u8, 146u8, 88u8, 110u8, 104u8, 49u8, 156u8, 41u8, 223u8, 11u8,
						],
					)
				}

				#[doc = " Actual proposal for a given hash, if it's current."]
				pub fn proposal_of_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::da_runtime::Call>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"ProposalOf",
						Vec::new(),
						[
							196u8, 81u8, 145u8, 33u8, 105u8, 200u8, 14u8, 212u8, 127u8, 142u8,
							201u8, 45u8, 147u8, 150u8, 136u8, 251u8, 36u8, 67u8, 24u8, 42u8, 26u8,
							8u8, 81u8, 146u8, 88u8, 110u8, 104u8, 49u8, 156u8, 41u8, 223u8, 11u8,
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
	}
	pub mod elections {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
			pub struct Vote {
				pub votes: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
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
			pub struct RemoveVoter;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SubmitCandidacy {
				#[codec(compact)]
				pub candidate_count: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct RenounceCandidacy {
				pub renouncing: runtime_types::pallet_elections_phragmen::Renouncing,
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
				pub has_replacement: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct CleanDefunctVoters {
				pub num_voters: ::core::primitive::u32,
				pub num_defunct: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Vote for a set of candidates for the upcoming round of election. This can be called to"]
				#[doc = "set the initial votes, or update already existing votes."]
				#[doc = ""]
				#[doc = "Upon initial voting, `value` units of `who`'s balance is locked and a deposit amount is"]
				#[doc = "reserved. The deposit is based on the number of votes and can be updated over time."]
				#[doc = ""]
				#[doc = "The `votes` should:"]
				#[doc = "  - not be empty."]
				#[doc = "  - be less than the number of possible candidates. Note that all current members and"]
				#[doc = "    runners-up are also automatically candidates for the next round."]
				#[doc = ""]
				#[doc = "If `value` is more than `who`'s total balance, then the maximum of the two is used."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be signed."]
				#[doc = ""]
				#[doc = "### Warning"]
				#[doc = ""]
				#[doc = "It is the responsibility of the caller to **NOT** place all of their balance into the"]
				#[doc = "lock and keep some for further operations."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "We assume the maximum weight among all 3 cases: vote_equal, vote_more and vote_less."]
				#[doc = "# </weight>"]
				pub fn vote(
					&self,
					votes: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Vote> {
					::subxt::tx::StaticTxPayload::new("Elections", "vote", Vote { votes, value }, [
						71u8, 90u8, 175u8, 225u8, 51u8, 202u8, 197u8, 252u8, 183u8, 92u8, 239u8,
						83u8, 112u8, 144u8, 128u8, 211u8, 109u8, 33u8, 252u8, 6u8, 156u8, 15u8,
						91u8, 88u8, 70u8, 19u8, 32u8, 29u8, 224u8, 255u8, 26u8, 145u8,
					])
				}

				#[doc = "Remove `origin` as a voter."]
				#[doc = ""]
				#[doc = "This removes the lock and returns the deposit."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be signed and be a voter."]
				pub fn remove_voter(&self) -> ::subxt::tx::StaticTxPayload<RemoveVoter> {
					::subxt::tx::StaticTxPayload::new(
						"Elections",
						"remove_voter",
						RemoveVoter {},
						[
							254u8, 46u8, 140u8, 4u8, 218u8, 45u8, 150u8, 72u8, 67u8, 131u8, 108u8,
							201u8, 46u8, 157u8, 104u8, 161u8, 53u8, 155u8, 130u8, 50u8, 88u8,
							149u8, 255u8, 12u8, 17u8, 85u8, 95u8, 69u8, 153u8, 130u8, 221u8, 1u8,
						],
					)
				}

				#[doc = "Submit oneself for candidacy. A fixed amount of deposit is recorded."]
				#[doc = ""]
				#[doc = "All candidates are wiped at the end of the term. They either become a member/runner-up,"]
				#[doc = "or leave the system while their deposit is slashed."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be signed."]
				#[doc = ""]
				#[doc = "### Warning"]
				#[doc = ""]
				#[doc = "Even if a candidate ends up being a member, they must call [`Call::renounce_candidacy`]"]
				#[doc = "to get their deposit back. Losing the spot in an election will always lead to a slash."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "The number of current candidates must be provided as witness data."]
				#[doc = "# </weight>"]
				pub fn submit_candidacy(
					&self,
					candidate_count: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SubmitCandidacy> {
					::subxt::tx::StaticTxPayload::new(
						"Elections",
						"submit_candidacy",
						SubmitCandidacy { candidate_count },
						[
							228u8, 63u8, 217u8, 99u8, 128u8, 104u8, 175u8, 10u8, 30u8, 35u8, 47u8,
							14u8, 254u8, 122u8, 146u8, 239u8, 61u8, 145u8, 82u8, 7u8, 181u8, 98u8,
							238u8, 208u8, 23u8, 84u8, 48u8, 255u8, 177u8, 255u8, 84u8, 83u8,
						],
					)
				}

				#[doc = "Renounce one's intention to be a candidate for the next election round. 3 potential"]
				#[doc = "outcomes exist:"]
				#[doc = ""]
				#[doc = "- `origin` is a candidate and not elected in any set. In this case, the deposit is"]
				#[doc = "  unreserved, returned and origin is removed as a candidate."]
				#[doc = "- `origin` is a current runner-up. In this case, the deposit is unreserved, returned and"]
				#[doc = "  origin is removed as a runner-up."]
				#[doc = "- `origin` is a current member. In this case, the deposit is unreserved and origin is"]
				#[doc = "  removed as a member, consequently not being a candidate for the next round anymore."]
				#[doc = "  Similar to [`remove_member`](Self::remove_member), if replacement runners exists, they"]
				#[doc = "  are immediately used. If the prime is renouncing, then no prime will exist until the"]
				#[doc = "  next round."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be signed, and have one of the above roles."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "The type of renouncing must be provided as witness data."]
				#[doc = "# </weight>"]
				pub fn renounce_candidacy(
					&self,
					renouncing: runtime_types::pallet_elections_phragmen::Renouncing,
				) -> ::subxt::tx::StaticTxPayload<RenounceCandidacy> {
					::subxt::tx::StaticTxPayload::new(
						"Elections",
						"renounce_candidacy",
						RenounceCandidacy { renouncing },
						[
							70u8, 72u8, 208u8, 36u8, 80u8, 245u8, 224u8, 75u8, 60u8, 142u8, 19u8,
							49u8, 142u8, 90u8, 14u8, 69u8, 15u8, 61u8, 170u8, 235u8, 16u8, 252u8,
							86u8, 200u8, 120u8, 127u8, 36u8, 42u8, 143u8, 130u8, 217u8, 128u8,
						],
					)
				}

				#[doc = "Remove a particular member from the set. This is effective immediately and the bond of"]
				#[doc = "the outgoing member is slashed."]
				#[doc = ""]
				#[doc = "If a runner-up is available, then the best runner-up will be removed and replaces the"]
				#[doc = "outgoing member. Otherwise, a new phragmen election is started."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be root."]
				#[doc = ""]
				#[doc = "Note that this does not affect the designated block number of the next election."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "If we have a replacement, we use a small weight. Else, since this is a root call and"]
				#[doc = "will go into phragmen, we assume full block for now."]
				#[doc = "# </weight>"]
				pub fn remove_member(
					&self,
					who: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					has_replacement: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<RemoveMember> {
					::subxt::tx::StaticTxPayload::new(
						"Elections",
						"remove_member",
						RemoveMember {
							who,
							has_replacement,
						},
						[
							74u8, 200u8, 25u8, 213u8, 209u8, 62u8, 229u8, 203u8, 33u8, 108u8,
							202u8, 134u8, 168u8, 49u8, 135u8, 227u8, 193u8, 201u8, 238u8, 11u8,
							197u8, 213u8, 58u8, 168u8, 204u8, 125u8, 36u8, 196u8, 34u8, 12u8, 1u8,
							148u8,
						],
					)
				}

				#[doc = "Clean all voters who are defunct (i.e. they do not serve any purpose at all). The"]
				#[doc = "deposit of the removed voters are returned."]
				#[doc = ""]
				#[doc = "This is an root function to be used only for cleaning the state."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be root."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "The total number of voters and those that are defunct must be provided as witness data."]
				#[doc = "# </weight>"]
				pub fn clean_defunct_voters(
					&self,
					num_voters: ::core::primitive::u32,
					num_defunct: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<CleanDefunctVoters> {
					::subxt::tx::StaticTxPayload::new(
						"Elections",
						"clean_defunct_voters",
						CleanDefunctVoters {
							num_voters,
							num_defunct,
						},
						[
							198u8, 162u8, 30u8, 249u8, 191u8, 38u8, 141u8, 123u8, 230u8, 90u8,
							213u8, 103u8, 168u8, 28u8, 5u8, 215u8, 213u8, 152u8, 46u8, 189u8,
							238u8, 209u8, 209u8, 142u8, 159u8, 222u8, 161u8, 26u8, 161u8, 250u8,
							9u8, 100u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_elections_phragmen::pallet::Event;
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
			#[doc = "A new term with new_members. This indicates that enough candidates existed to run"]
			#[doc = "the election, not that enough have has been elected. The inner value must be examined"]
			#[doc = "for this purpose. A `NewTerm(\\[\\])` indicates that some candidates got their bond"]
			#[doc = "slashed and none were elected, whilst `EmptyTerm` means that no candidates existed to"]
			#[doc = "begin with."]
			pub struct NewTerm {
				pub new_members: ::std::vec::Vec<(
					::subxt::ext::sp_core::crypto::AccountId32,
					::core::primitive::u128,
				)>,
			}
			impl ::subxt::events::StaticEvent for NewTerm {
				const EVENT: &'static str = "NewTerm";
				const PALLET: &'static str = "Elections";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "No (or not enough) candidates existed for this round. This is different from"]
			#[doc = "`NewTerm(\\[\\])`. See the description of `NewTerm`."]
			pub struct EmptyTerm;
			impl ::subxt::events::StaticEvent for EmptyTerm {
				const EVENT: &'static str = "EmptyTerm";
				const PALLET: &'static str = "Elections";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Internal error happened while trying to perform election."]
			pub struct ElectionError;
			impl ::subxt::events::StaticEvent for ElectionError {
				const EVENT: &'static str = "ElectionError";
				const PALLET: &'static str = "Elections";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A member has been removed. This should always be followed by either `NewTerm` or"]
			#[doc = "`EmptyTerm`."]
			pub struct MemberKicked {
				pub member: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for MemberKicked {
				const EVENT: &'static str = "MemberKicked";
				const PALLET: &'static str = "Elections";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "Someone has renounced their candidacy."]
			pub struct Renounced {
				pub candidate: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Renounced {
				const EVENT: &'static str = "Renounced";
				const PALLET: &'static str = "Elections";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A candidate was slashed by amount due to failing to obtain a seat as member or"]
			#[doc = "runner-up."]
			#[doc = ""]
			#[doc = "Note that old members and runners-up are also candidates."]
			pub struct CandidateSlashed {
				pub candidate: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for CandidateSlashed {
				const EVENT: &'static str = "CandidateSlashed";
				const PALLET: &'static str = "Elections";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			#[doc = "A seat holder was slashed by amount by being forcefully removed from the set."]
			pub struct SeatHolderSlashed {
				pub seat_holder: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for SeatHolderSlashed {
				const EVENT: &'static str = "SeatHolderSlashed";
				const PALLET: &'static str = "Elections";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current elected members."]
				#[doc = ""]
				#[doc = " Invariant: Always sorted based on account id."]
				pub fn members(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::pallet_elections_phragmen::SeatHolder<
								::subxt::ext::sp_core::crypto::AccountId32,
								::core::primitive::u128,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Elections",
						"Members",
						vec![],
						[
							2u8, 182u8, 43u8, 180u8, 87u8, 185u8, 26u8, 79u8, 196u8, 55u8, 28u8,
							26u8, 174u8, 133u8, 158u8, 221u8, 101u8, 161u8, 83u8, 9u8, 221u8,
							175u8, 221u8, 220u8, 81u8, 80u8, 1u8, 236u8, 74u8, 121u8, 10u8, 82u8,
						],
					)
				}

				#[doc = " The current reserved runners-up."]
				#[doc = ""]
				#[doc = " Invariant: Always sorted based on rank (worse to best). Upon removal of a member, the"]
				#[doc = " last (i.e. _best_) runner-up will be replaced."]
				pub fn runners_up(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::pallet_elections_phragmen::SeatHolder<
								::subxt::ext::sp_core::crypto::AccountId32,
								::core::primitive::u128,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Elections",
						"RunnersUp",
						vec![],
						[
							248u8, 81u8, 190u8, 53u8, 121u8, 49u8, 55u8, 69u8, 116u8, 177u8, 46u8,
							30u8, 131u8, 14u8, 32u8, 198u8, 10u8, 132u8, 73u8, 117u8, 2u8, 146u8,
							188u8, 146u8, 214u8, 227u8, 97u8, 77u8, 7u8, 131u8, 208u8, 209u8,
						],
					)
				}

				#[doc = " The present candidate list. A current member or runner-up can never enter this vector"]
				#[doc = " and is always implicitly assumed to be a candidate."]
				#[doc = ""]
				#[doc = " Second element is the deposit."]
				#[doc = ""]
				#[doc = " Invariant: Always sorted based on account id."]
				pub fn candidates(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Elections",
						"Candidates",
						vec![],
						[
							224u8, 107u8, 141u8, 11u8, 54u8, 86u8, 117u8, 45u8, 195u8, 252u8,
							152u8, 21u8, 165u8, 23u8, 198u8, 117u8, 5u8, 216u8, 183u8, 163u8,
							243u8, 56u8, 11u8, 102u8, 85u8, 107u8, 219u8, 250u8, 45u8, 80u8, 108u8,
							127u8,
						],
					)
				}

				#[doc = " The total number of vote rounds that have happened, excluding the upcoming one."]
				pub fn election_rounds(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Elections",
						"ElectionRounds",
						vec![],
						[
							144u8, 146u8, 10u8, 32u8, 149u8, 147u8, 59u8, 205u8, 61u8, 246u8, 28u8,
							169u8, 130u8, 136u8, 143u8, 104u8, 253u8, 86u8, 228u8, 68u8, 19u8,
							184u8, 166u8, 214u8, 58u8, 103u8, 176u8, 160u8, 240u8, 249u8, 117u8,
							115u8,
						],
					)
				}

				#[doc = " Votes and locked stake of a particular voter."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE as `AccountId` is a crypto hash."]
				pub fn voting(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_elections_phragmen::Voter<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Elections",
						"Voting",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							9u8, 135u8, 76u8, 194u8, 240u8, 182u8, 111u8, 207u8, 102u8, 37u8,
							126u8, 36u8, 84u8, 112u8, 26u8, 216u8, 175u8, 5u8, 14u8, 189u8, 83u8,
							185u8, 136u8, 39u8, 171u8, 221u8, 147u8, 20u8, 168u8, 126u8, 111u8,
							137u8,
						],
					)
				}

				#[doc = " Votes and locked stake of a particular voter."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE as `AccountId` is a crypto hash."]
				pub fn voting_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_elections_phragmen::Voter<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Elections",
						"Voting",
						Vec::new(),
						[
							9u8, 135u8, 76u8, 194u8, 240u8, 182u8, 111u8, 207u8, 102u8, 37u8,
							126u8, 36u8, 84u8, 112u8, 26u8, 216u8, 175u8, 5u8, 14u8, 189u8, 83u8,
							185u8, 136u8, 39u8, 171u8, 221u8, 147u8, 20u8, 168u8, 126u8, 111u8,
							137u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Identifier for the elections-phragmen pallet's lock"]
				pub fn pallet_id(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<[::core::primitive::u8; 8usize]>,
				> {
					::subxt::constants::StaticConstantAddress::new("Elections", "PalletId", [
						224u8, 197u8, 247u8, 125u8, 62u8, 180u8, 69u8, 91u8, 226u8, 36u8, 82u8,
						148u8, 70u8, 147u8, 209u8, 40u8, 210u8, 229u8, 181u8, 191u8, 170u8, 205u8,
						138u8, 97u8, 127u8, 59u8, 124u8, 244u8, 252u8, 30u8, 213u8, 179u8,
					])
				}

				#[doc = " How much should be locked up in order to submit one's candidacy."]
				pub fn candidacy_bond(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new("Elections", "CandidacyBond", [
						84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
						27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
						71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
					])
				}

				#[doc = " Base deposit associated with voting."]
				#[doc = ""]
				#[doc = " This should be sensibly high to economically ensure the pallet cannot be attacked by"]
				#[doc = " creating a gigantic number of votes."]
				pub fn voting_bond_base(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new("Elections", "VotingBondBase", [
						84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
						27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
						71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
					])
				}

				#[doc = " The amount of bond that need to be locked for each vote (32 bytes)."]
				pub fn voting_bond_factor(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Elections",
						"VotingBondFactor",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}

				#[doc = " Number of members to elect."]
				pub fn desired_members(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Elections", "DesiredMembers", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}

				#[doc = " Number of runners_up to keep."]
				pub fn desired_runners_up(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Elections",
						"DesiredRunnersUp",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " How long each seat is kept. This defines the next block number at which an election"]
				#[doc = " round will happen. If set to zero, no elections are ever triggered and the module will"]
				#[doc = " be in passive mode."]
				pub fn term_duration(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new("Elections", "TermDuration", [
						98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8,
						151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
						113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8,
					])
				}
			}
		}
	}
	pub mod technical_membership {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
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
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
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
				pub remove: ::subxt::ext::sp_core::crypto::AccountId32,
				pub add: ::subxt::ext::sp_core::crypto::AccountId32,
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
				pub new: ::subxt::ext::sp_core::crypto::AccountId32,
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
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
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
				#[doc = "Add a member `who` to the set."]
				#[doc = ""]
				#[doc = "May only be called from `T::AddOrigin`."]
				pub fn add_member(
					&self,
					who: ::subxt::ext::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<AddMember> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalMembership",
						"add_member",
						AddMember { who },
						[
							106u8, 33u8, 171u8, 114u8, 223u8, 105u8, 71u8, 15u8, 77u8, 253u8, 40u8,
							204u8, 244u8, 142u8, 103u8, 177u8, 200u8, 243u8, 114u8, 241u8, 36u8,
							135u8, 175u8, 255u8, 124u8, 193u8, 30u8, 46u8, 186u8, 172u8, 176u8,
							98u8,
						],
					)
				}

				#[doc = "Remove a member `who` from the set."]
				#[doc = ""]
				#[doc = "May only be called from `T::RemoveOrigin`."]
				pub fn remove_member(
					&self,
					who: ::subxt::ext::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<RemoveMember> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalMembership",
						"remove_member",
						RemoveMember { who },
						[
							100u8, 17u8, 75u8, 92u8, 58u8, 100u8, 34u8, 187u8, 41u8, 160u8, 137u8,
							58u8, 78u8, 166u8, 161u8, 116u8, 1u8, 67u8, 201u8, 144u8, 103u8, 84u8,
							55u8, 246u8, 133u8, 180u8, 148u8, 86u8, 175u8, 175u8, 70u8, 73u8,
						],
					)
				}

				#[doc = "Swap out one member `remove` for another `add`."]
				#[doc = ""]
				#[doc = "May only be called from `T::SwapOrigin`."]
				#[doc = ""]
				#[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
				pub fn swap_member(
					&self,
					remove: ::subxt::ext::sp_core::crypto::AccountId32,
					add: ::subxt::ext::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<SwapMember> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalMembership",
						"swap_member",
						SwapMember { remove, add },
						[
							66u8, 84u8, 183u8, 29u8, 104u8, 163u8, 220u8, 217u8, 103u8, 234u8,
							233u8, 138u8, 191u8, 147u8, 51u8, 98u8, 46u8, 51u8, 179u8, 200u8, 23u8,
							59u8, 112u8, 53u8, 8u8, 75u8, 135u8, 232u8, 116u8, 201u8, 60u8, 249u8,
						],
					)
				}

				#[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
				#[doc = "pass `members` pre-sorted."]
				#[doc = ""]
				#[doc = "May only be called from `T::ResetOrigin`."]
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

				#[doc = "Swap out the sending member for some other key `new`."]
				#[doc = ""]
				#[doc = "May only be called from `Signed` origin of a current member."]
				#[doc = ""]
				#[doc = "Prime membership is passed from the origin account to `new`, if extant."]
				pub fn change_key(
					&self,
					new: ::subxt::ext::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<ChangeKey> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalMembership",
						"change_key",
						ChangeKey { new },
						[
							53u8, 60u8, 54u8, 231u8, 151u8, 0u8, 27u8, 175u8, 250u8, 80u8, 74u8,
							184u8, 184u8, 63u8, 90u8, 216u8, 186u8, 136u8, 74u8, 214u8, 111u8,
							186u8, 137u8, 140u8, 108u8, 194u8, 128u8, 97u8, 168u8, 184u8, 112u8,
							60u8,
						],
					)
				}

				#[doc = "Set the prime member. Must be a current member."]
				#[doc = ""]
				#[doc = "May only be called from `T::PrimeOrigin`."]
				pub fn set_prime(
					&self,
					who: ::subxt::ext::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<SetPrime> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalMembership",
						"set_prime",
						SetPrime { who },
						[
							123u8, 95u8, 75u8, 129u8, 19u8, 34u8, 192u8, 65u8, 169u8, 47u8, 184u8,
							246u8, 55u8, 250u8, 31u8, 158u8, 57u8, 197u8, 22u8, 112u8, 167u8,
							198u8, 136u8, 17u8, 15u8, 203u8, 101u8, 149u8, 15u8, 39u8, 16u8, 232u8,
						],
					)
				}

				#[doc = "Remove the prime member if it exists."]
				#[doc = ""]
				#[doc = "May only be called from `T::PrimeOrigin`."]
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
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
						::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
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
							162u8, 72u8, 174u8, 204u8, 140u8, 105u8, 205u8, 176u8, 197u8, 117u8,
							206u8, 134u8, 157u8, 110u8, 139u8, 54u8, 43u8, 233u8, 25u8, 51u8, 36u8,
							238u8, 94u8, 124u8, 221u8, 52u8, 237u8, 71u8, 125u8, 56u8, 129u8,
							222u8,
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
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
					runtime_types::sp_finality_grandpa::EquivocationProof<
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
					runtime_types::sp_finality_grandpa::EquivocationProof<
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
				#[doc = "Report voter equivocation/misbehavior. This method will verify the"]
				#[doc = "equivocation proof and validate the given key ownership proof"]
				#[doc = "against the extracted offender. If both are valid, the offence"]
				#[doc = "will be reported."]
				pub fn report_equivocation(
					&self,
					equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
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

				#[doc = "Report voter equivocation/misbehavior. This method will verify the"]
				#[doc = "equivocation proof and validate the given key ownership proof"]
				#[doc = "against the extracted offender. If both are valid, the offence"]
				#[doc = "will be reported."]
				#[doc = ""]
				#[doc = "This extrinsic must be called unsigned and it is expected that only"]
				#[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
				#[doc = "if the block author is defined it will be defined as the equivocation"]
				#[doc = "reporter."]
				pub fn report_equivocation_unsigned(
					&self,
					equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
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

				#[doc = "Note that the current authority set of the GRANDPA finality gadget has"]
				#[doc = "stalled. This will trigger a forced authority set change at the beginning"]
				#[doc = "of the next session, to be enacted `delay` blocks after that. The delay"]
				#[doc = "should be high enough to safely assume that the block signalling the"]
				#[doc = "forced change will not be re-orged (e.g. 1000 blocks). The GRANDPA voters"]
				#[doc = "will start the new authority set using the given finalized block as base."]
				#[doc = "Only callable by root."]
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
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
					runtime_types::sp_finality_grandpa::app::Public,
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
			}
		}
	}
	pub mod treasury {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Put forward a suggestion for spending. A deposit proportional to the value"]
				#[doc = "is reserved and slashed if the proposal is rejected. It is returned once the"]
				#[doc = "proposal is awarded."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(1)"]
				#[doc = "- DbReads: `ProposalCount`, `origin account`"]
				#[doc = "- DbWrites: `ProposalCount`, `Proposals`, `origin account`"]
				#[doc = "# </weight>"]
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

				#[doc = "Reject a proposed spend. The original deposit will be slashed."]
				#[doc = ""]
				#[doc = "May only be called from `T::RejectOrigin`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(1)"]
				#[doc = "- DbReads: `Proposals`, `rejected proposer account`"]
				#[doc = "- DbWrites: `Proposals`, `rejected proposer account`"]
				#[doc = "# </weight>"]
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

				#[doc = "Approve a proposal. At a later time, the proposal will be allocated to the beneficiary"]
				#[doc = "and the original deposit will be returned."]
				#[doc = ""]
				#[doc = "May only be called from `T::ApproveOrigin`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(1)."]
				#[doc = "- DbReads: `Proposals`, `Approvals`"]
				#[doc = "- DbWrite: `Approvals`"]
				#[doc = "# </weight>"]
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
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
			#[doc = "New proposal. \\[proposal_index\\]"]
			pub struct Proposed(pub ::core::primitive::u32);
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
			#[doc = "We have ended a spend period and will now allocate funds. \\[budget_remaining\\]"]
			pub struct Spending(pub ::core::primitive::u128);
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
			#[doc = "Some funds have been allocated. \\[proposal_index, award, beneficiary\\]"]
			pub struct Awarded(
				pub ::core::primitive::u32,
				pub ::core::primitive::u128,
				pub ::subxt::ext::sp_core::crypto::AccountId32,
			);
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
			#[doc = "A proposal was rejected; funds were slashed. \\[proposal_index, slashed\\]"]
			pub struct Rejected(pub ::core::primitive::u32, pub ::core::primitive::u128);
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
			#[doc = "Some of our funds have been burnt. \\[burn\\]"]
			pub struct Burnt(pub ::core::primitive::u128);
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
			#[doc = "\\[budget_remaining\\]"]
			pub struct Rollover(pub ::core::primitive::u128);
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
			#[doc = "Some funds have been deposited. \\[deposit\\]"]
			pub struct Deposit(pub ::core::primitive::u128);
			impl ::subxt::events::StaticEvent for Deposit {
				const EVENT: &'static str = "Deposit";
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

				#[doc = " Proposal indices that have been approved but not yet awarded."]
				pub fn approvals(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_support::storage::bounded_vec::BoundedVec<
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
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				pub call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
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
				pub call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
				pub weight: ::core::primitive::u64,
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
				pub call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- Limited storage reads."]
				#[doc = "- One DB write (event)."]
				#[doc = "- Weight of derivative `call` execution + 10,000."]
				#[doc = "# </weight>"]
				pub fn sudo(
					&self,
					call: runtime_types::da_runtime::Call,
				) -> ::subxt::tx::StaticTxPayload<Sudo> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"sudo",
						Sudo {
							call: ::std::boxed::Box::new(call),
						},
						[
							171u8, 68u8, 181u8, 132u8, 70u8, 14u8, 217u8, 20u8, 26u8, 65u8, 185u8,
							120u8, 82u8, 128u8, 237u8, 234u8, 208u8, 225u8, 99u8, 66u8, 165u8,
							55u8, 45u8, 250u8, 158u8, 215u8, 65u8, 143u8, 47u8, 205u8, 56u8, 136u8,
						],
					)
				}

				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				#[doc = "This function does not check the weight of the call, and instead allows the"]
				#[doc = "Sudo user to specify the weight of the call."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- The weight of this call is defined by the caller."]
				#[doc = "# </weight>"]
				pub fn sudo_unchecked_weight(
					&self,
					call: runtime_types::da_runtime::Call,
					weight: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<SudoUncheckedWeight> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"sudo_unchecked_weight",
						SudoUncheckedWeight {
							call: ::std::boxed::Box::new(call),
							weight,
						},
						[
							183u8, 125u8, 101u8, 173u8, 153u8, 151u8, 94u8, 250u8, 7u8, 38u8,
							135u8, 115u8, 246u8, 255u8, 210u8, 51u8, 247u8, 170u8, 87u8, 139u8,
							138u8, 143u8, 7u8, 216u8, 120u8, 254u8, 171u8, 239u8, 119u8, 76u8,
							215u8, 251u8,
						],
					)
				}

				#[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
				#[doc = "key."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- Limited storage reads."]
				#[doc = "- One DB change."]
				#[doc = "# </weight>"]
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

				#[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
				#[doc = "a given account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- Limited storage reads."]
				#[doc = "- One DB write (event)."]
				#[doc = "- Weight of derivative `call` execution + 10,000."]
				#[doc = "# </weight>"]
				pub fn sudo_as(
					&self,
					who: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					call: runtime_types::da_runtime::Call,
				) -> ::subxt::tx::StaticTxPayload<SudoAs> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"sudo_as",
						SudoAs {
							who,
							call: ::std::boxed::Box::new(call),
						},
						[
							97u8, 169u8, 79u8, 164u8, 2u8, 113u8, 142u8, 119u8, 166u8, 223u8, 32u8,
							194u8, 181u8, 121u8, 90u8, 38u8, 102u8, 25u8, 186u8, 119u8, 254u8,
							175u8, 3u8, 26u8, 238u8, 152u8, 206u8, 10u8, 30u8, 12u8, 1u8, 70u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
			#[doc = "The \\[sudoer\\] just switched identity; the old key is supplied."]
			pub struct KeyChanged {
				pub new_sudoer: ::subxt::ext::sp_core::crypto::AccountId32,
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
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new("Sudo", "Key", vec![], [
						49u8, 130u8, 254u8, 227u8, 6u8, 58u8, 236u8, 207u8, 61u8, 99u8, 56u8,
						133u8, 92u8, 32u8, 156u8, 7u8, 153u8, 47u8, 57u8, 8u8, 92u8, 108u8, 86u8,
						196u8, 135u8, 191u8, 67u8, 102u8, 187u8, 191u8, 182u8, 153u8,
					])
				}
			}
		}
	}
	pub mod im_online {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				#[doc = "# <weight>"]
				#[doc = "- Complexity: `O(K + E)` where K is length of `Keys` (heartbeat.validators_len) and E is"]
				#[doc = "  length of `heartbeat.network_state.external_address`"]
				#[doc = "  - `O(K)`: decoding of length `K`"]
				#[doc = "  - `O(E)`: decoding/encoding of length `E`"]
				#[doc = "- DbReads: pallet_session `Validators`, pallet_session `CurrentIndex`, `Keys`,"]
				#[doc = "  `ReceivedHeartbeats`"]
				#[doc = "- DbWrites: `ReceivedHeartbeats`"]
				#[doc = "# </weight>"]
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
							212u8, 23u8, 174u8, 246u8, 60u8, 220u8, 178u8, 137u8, 53u8, 146u8,
							165u8, 225u8, 179u8, 209u8, 233u8, 152u8, 129u8, 210u8, 126u8, 32u8,
							216u8, 22u8, 76u8, 196u8, 255u8, 128u8, 246u8, 161u8, 30u8, 186u8,
							249u8, 34u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
						runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
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

				#[doc = " For each session index, we keep a mapping of `SessionIndex` and `AuthIndex` to"]
				#[doc = " `WrapperOpaque<BoundedOpaqueNetworkState>`."]
				pub fn received_heartbeats(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_support::traits::misc::WrapperOpaque<
							runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
						>,
					>,
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
							233u8, 128u8, 140u8, 233u8, 55u8, 146u8, 172u8, 54u8, 54u8, 57u8,
							141u8, 106u8, 168u8, 59u8, 147u8, 253u8, 119u8, 48u8, 50u8, 251u8,
							242u8, 109u8, 251u8, 2u8, 136u8, 80u8, 146u8, 121u8, 180u8, 219u8,
							245u8, 37u8,
						],
					)
				}

				#[doc = " For each session index, we keep a mapping of `SessionIndex` and `AuthIndex` to"]
				#[doc = " `WrapperOpaque<BoundedOpaqueNetworkState>`."]
				pub fn received_heartbeats_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_support::traits::misc::WrapperOpaque<
							runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ImOnline",
						"ReceivedHeartbeats",
						Vec::new(),
						[
							233u8, 128u8, 140u8, 233u8, 55u8, 146u8, 172u8, 54u8, 54u8, 57u8,
							141u8, 106u8, 168u8, 59u8, 147u8, 253u8, 119u8, 48u8, 50u8, 251u8,
							242u8, 109u8, 251u8, 2u8, 136u8, 80u8, 146u8, 121u8, 180u8, 219u8,
							245u8, 37u8,
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
						runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
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
						runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
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

				#[doc = " Enumerates all reports of a kind along with the time they happened."]
				#[doc = ""]
				#[doc = " All reports are sorted by the time of offence."]
				#[doc = ""]
				#[doc = " Note that the actual type of this mapping is `Vec<u8>`, this is because values of"]
				#[doc = " different types are not supported at the moment so we are doing the manual serialization."]
				pub fn reports_by_kind_index(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8; 16usize]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Offences",
						"ReportsByKindIndex",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							162u8, 66u8, 131u8, 48u8, 250u8, 237u8, 179u8, 214u8, 36u8, 137u8,
							226u8, 136u8, 120u8, 61u8, 215u8, 43u8, 164u8, 50u8, 91u8, 164u8, 20u8,
							96u8, 189u8, 100u8, 242u8, 106u8, 21u8, 136u8, 98u8, 215u8, 180u8,
							145u8,
						],
					)
				}

				#[doc = " Enumerates all reports of a kind along with the time they happened."]
				#[doc = ""]
				#[doc = " All reports are sorted by the time of offence."]
				#[doc = ""]
				#[doc = " Note that the actual type of this mapping is `Vec<u8>`, this is because values of"]
				#[doc = " different types are not supported at the moment so we are doing the manual serialization."]
				pub fn reports_by_kind_index_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Offences",
						"ReportsByKindIndex",
						Vec::new(),
						[
							162u8, 66u8, 131u8, 48u8, 250u8, 237u8, 179u8, 214u8, 36u8, 137u8,
							226u8, 136u8, 120u8, 61u8, 215u8, 43u8, 164u8, 50u8, 91u8, 164u8, 20u8,
							96u8, 189u8, 100u8, 242u8, 106u8, 21u8, 136u8, 98u8, 215u8, 180u8,
							145u8,
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
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				pub call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
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
				pub id: ::std::vec::Vec<::core::primitive::u8>,
				pub when: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
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
				pub id: ::std::vec::Vec<::core::primitive::u8>,
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
				pub call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
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
				pub id: ::std::vec::Vec<::core::primitive::u8>,
				pub after: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Anonymously schedule a task."]
				pub fn schedule(
					&self,
					when: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::da_runtime::Call,
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
							113u8, 150u8, 229u8, 7u8, 128u8, 203u8, 213u8, 88u8, 164u8, 19u8, 51u8,
							11u8, 97u8, 181u8, 165u8, 251u8, 232u8, 32u8, 15u8, 28u8, 220u8, 126u8,
							28u8, 49u8, 146u8, 89u8, 63u8, 79u8, 86u8, 101u8, 49u8, 218u8,
						],
					)
				}

				#[doc = "Cancel an anonymously scheduled task."]
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

				#[doc = "Schedule a named task."]
				pub fn schedule_named(
					&self,
					id: ::std::vec::Vec<::core::primitive::u8>,
					when: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::da_runtime::Call,
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
							63u8, 175u8, 81u8, 116u8, 29u8, 49u8, 99u8, 92u8, 85u8, 2u8, 157u8,
							178u8, 17u8, 161u8, 208u8, 9u8, 55u8, 241u8, 234u8, 31u8, 15u8, 188u8,
							103u8, 57u8, 172u8, 212u8, 200u8, 220u8, 82u8, 132u8, 177u8, 88u8,
						],
					)
				}

				#[doc = "Cancel a named scheduled task."]
				pub fn cancel_named(
					&self,
					id: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<CancelNamed> {
					::subxt::tx::StaticTxPayload::new(
						"Scheduler",
						"cancel_named",
						CancelNamed { id },
						[
							42u8, 232u8, 92u8, 167u8, 113u8, 136u8, 7u8, 215u8, 88u8, 117u8, 74u8,
							26u8, 225u8, 230u8, 244u8, 106u8, 150u8, 112u8, 46u8, 228u8, 96u8,
							252u8, 78u8, 126u8, 39u8, 207u8, 36u8, 110u8, 83u8, 62u8, 84u8, 241u8,
						],
					)
				}

				#[doc = "Anonymously schedule a task after a delay."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Same as [`schedule`]."]
				#[doc = "# </weight>"]
				pub fn schedule_after(
					&self,
					after: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::da_runtime::Call,
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
							169u8, 95u8, 7u8, 84u8, 167u8, 126u8, 80u8, 87u8, 150u8, 12u8, 48u8,
							204u8, 132u8, 40u8, 106u8, 225u8, 130u8, 53u8, 191u8, 230u8, 1u8,
							244u8, 240u8, 159u8, 117u8, 6u8, 101u8, 110u8, 93u8, 62u8, 105u8, 16u8,
						],
					)
				}

				#[doc = "Schedule a named task after a delay."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Same as [`schedule_named`](Self::schedule_named)."]
				#[doc = "# </weight>"]
				pub fn schedule_named_after(
					&self,
					id: ::std::vec::Vec<::core::primitive::u8>,
					after: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::da_runtime::Call,
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
							148u8, 77u8, 1u8, 187u8, 47u8, 64u8, 166u8, 15u8, 199u8, 133u8, 64u8,
							1u8, 187u8, 63u8, 242u8, 251u8, 4u8, 169u8, 100u8, 169u8, 230u8, 114u8,
							225u8, 200u8, 3u8, 164u8, 28u8, 99u8, 123u8, 246u8, 236u8, 197u8,
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
			#[doc = "Scheduled some task. \\[when, index\\]"]
			pub struct Scheduled(pub ::core::primitive::u32, pub ::core::primitive::u32);
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
			#[doc = "Canceled some task. \\[when, index\\]"]
			pub struct Canceled(pub ::core::primitive::u32, pub ::core::primitive::u32);
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
			#[doc = "Dispatched some task. \\[task, id, result\\]"]
			pub struct Dispatched(
				pub (::core::primitive::u32, ::core::primitive::u32),
				pub ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
				pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			);
			impl ::subxt::events::StaticEvent for Dispatched {
				const EVENT: &'static str = "Dispatched";
				const PALLET: &'static str = "Scheduler";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Items to be executed, indexed by the block number that they should be executed on."]
				pub fn agenda(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							::core::option::Option<
								runtime_types::pallet_scheduler::ScheduledV2<
									runtime_types::da_runtime::Call,
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
							58u8, 200u8, 120u8, 255u8, 24u8, 226u8, 20u8, 181u8, 8u8, 227u8, 145u8,
							86u8, 232u8, 8u8, 208u8, 205u8, 170u8, 131u8, 237u8, 238u8, 168u8,
							163u8, 69u8, 174u8, 215u8, 22u8, 9u8, 123u8, 59u8, 120u8, 99u8, 33u8,
						],
					)
				}

				#[doc = " Items to be executed, indexed by the block number that they should be executed on."]
				pub fn agenda_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							::core::option::Option<
								runtime_types::pallet_scheduler::ScheduledV2<
									runtime_types::da_runtime::Call,
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
							58u8, 200u8, 120u8, 255u8, 24u8, 226u8, 20u8, 181u8, 8u8, 227u8, 145u8,
							86u8, 232u8, 8u8, 208u8, 205u8, 170u8, 131u8, 237u8, 238u8, 168u8,
							163u8, 69u8, 174u8, 215u8, 22u8, 9u8, 123u8, 59u8, 120u8, 99u8, 33u8,
						],
					)
				}

				#[doc = " Lookup from identity to the block number and index of the task."]
				pub fn lookup(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
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
							56u8, 105u8, 156u8, 110u8, 251u8, 141u8, 219u8, 56u8, 131u8, 57u8,
							180u8, 33u8, 48u8, 30u8, 193u8, 194u8, 169u8, 182u8, 168u8, 43u8, 36u8,
							202u8, 222u8, 182u8, 41u8, 216u8, 222u8, 1u8, 72u8, 165u8, 62u8, 166u8,
						],
					)
				}

				#[doc = " Lookup from identity to the block number and index of the task."]
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
							56u8, 105u8, 156u8, 110u8, 251u8, 141u8, 219u8, 56u8, 131u8, 57u8,
							180u8, 33u8, 48u8, 30u8, 193u8, 194u8, 169u8, 182u8, 168u8, 43u8, 36u8,
							202u8, 222u8, 182u8, 41u8, 216u8, 222u8, 1u8, 72u8, 165u8, 62u8, 166u8,
						],
					)
				}

				#[doc = " Storage version of the pallet."]
				#[doc = ""]
				#[doc = " New networks start with last version."]
				pub fn storage_version(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::pallet_scheduler::Releases>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Scheduler",
						"StorageVersion",
						vec![],
						[
							181u8, 185u8, 153u8, 214u8, 175u8, 17u8, 111u8, 241u8, 124u8, 242u8,
							171u8, 99u8, 193u8, 23u8, 251u8, 248u8, 150u8, 241u8, 249u8, 142u8,
							234u8, 209u8, 246u8, 39u8, 232u8, 192u8, 44u8, 121u8, 63u8, 14u8,
							245u8, 110u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The maximum weight that may be scheduled per block for any dispatchables of less"]
				#[doc = " priority than `schedule::HARD_DEADLINE`."]
				pub fn maximum_weight(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new("Scheduler", "MaximumWeight", [
						128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8, 59u8,
						226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8, 103u8, 119u8,
						53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8, 246u8,
					])
				}

				#[doc = " The maximum number of scheduled calls in the queue for a single block."]
				#[doc = " Not strictly enforced, but used for weight estimation."]
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
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				#[doc = "Propose a new bounty."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "Payment: `TipReportDepositBase` will be reserved from the origin account, as well as"]
				#[doc = "`DataDepositPerByte` for each byte in `reason`. It will be unreserved upon approval,"]
				#[doc = "or slashed when rejected."]
				#[doc = ""]
				#[doc = "- `curator`: The curator account whom will manage this bounty."]
				#[doc = "- `fee`: The curator fee."]
				#[doc = "- `value`: The total payment amount of this bounty, curator fee included."]
				#[doc = "- `description`: The description of this bounty."]
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

				#[doc = "Approve a bounty proposal. At a later time, the bounty will be funded and become active"]
				#[doc = "and the original deposit will be returned."]
				#[doc = ""]
				#[doc = "May only be called from `T::ApproveOrigin`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "# </weight>"]
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

				#[doc = "Assign a curator to a funded bounty."]
				#[doc = ""]
				#[doc = "May only be called from `T::ApproveOrigin`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "# </weight>"]
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

				#[doc = "Unassign curator from a bounty."]
				#[doc = ""]
				#[doc = "This function can only be called by the `RejectOrigin` a signed origin."]
				#[doc = ""]
				#[doc = "If this function is called by the `RejectOrigin`, we assume that the curator is"]
				#[doc = "malicious or inactive. As a result, we will slash the curator when possible."]
				#[doc = ""]
				#[doc = "If the origin is the curator, we take this as a sign they are unable to do their job and"]
				#[doc = "they willingly give up. We could slash them, but for now we allow them to recover their"]
				#[doc = "deposit and exit without issue. (We may want to change this if it is abused.)"]
				#[doc = ""]
				#[doc = "Finally, the origin can be anyone if and only if the curator is \"inactive\". This allows"]
				#[doc = "anyone in the community to call out that a curator is not doing their due diligence, and"]
				#[doc = "we should pick a new curator. In this case the curator should also be slashed."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "# </weight>"]
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

				#[doc = "Accept the curator role for a bounty."]
				#[doc = "A deposit will be reserved from curator and refund upon successful payout."]
				#[doc = ""]
				#[doc = "May only be called from the curator."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "# </weight>"]
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

				#[doc = "Award bounty to a beneficiary account. The beneficiary will be able to claim the funds"]
				#[doc = "after a delay."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be the curator of this bounty."]
				#[doc = ""]
				#[doc = "- `bounty_id`: Bounty ID to award."]
				#[doc = "- `beneficiary`: The beneficiary account whom will receive the payout."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "# </weight>"]
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

				#[doc = "Claim the payout from an awarded bounty after payout delay."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be the beneficiary of this bounty."]
				#[doc = ""]
				#[doc = "- `bounty_id`: Bounty ID to claim."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "# </weight>"]
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

				#[doc = "Cancel a proposed or active bounty. All the funds will be sent to treasury and"]
				#[doc = "the curator deposit will be unreserved if possible."]
				#[doc = ""]
				#[doc = "Only `T::RejectOrigin` is able to cancel a bounty."]
				#[doc = ""]
				#[doc = "- `bounty_id`: Bounty ID to cancel."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "# </weight>"]
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

				#[doc = "Extend the expiry time of an active bounty."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be the curator of this bounty."]
				#[doc = ""]
				#[doc = "- `bounty_id`: Bounty ID to extend."]
				#[doc = "- `remark`: additional information."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "# </weight>"]
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
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
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
							123u8, 119u8, 5u8, 212u8, 136u8, 112u8, 152u8, 50u8, 214u8, 158u8,
							239u8, 234u8, 41u8, 16u8, 175u8, 63u8, 0u8, 131u8, 155u8, 33u8, 161u8,
							83u8, 59u8, 132u8, 125u8, 29u8, 254u8, 25u8, 249u8, 43u8, 213u8, 118u8,
						],
					)
				}

				#[doc = " The description of each bounty."]
				pub fn bounty_descriptions_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Bounties",
						"BountyDescriptions",
						Vec::new(),
						[
							123u8, 119u8, 5u8, 212u8, 136u8, 112u8, 152u8, 50u8, 214u8, 158u8,
							239u8, 234u8, 41u8, 16u8, 175u8, 63u8, 0u8, 131u8, 155u8, 33u8, 161u8,
							83u8, 59u8, 132u8, 125u8, 29u8, 254u8, 25u8, 249u8, 43u8, 213u8, 118u8,
						],
					)
				}

				#[doc = " Bounty indices that have been approved but not yet funded."]
				pub fn bounty_approvals(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u32>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Bounties",
						"BountyApprovals",
						vec![],
						[
							107u8, 39u8, 251u8, 45u8, 14u8, 33u8, 113u8, 234u8, 247u8, 75u8, 86u8,
							5u8, 215u8, 20u8, 119u8, 179u8, 221u8, 175u8, 110u8, 123u8, 240u8,
							212u8, 57u8, 238u8, 173u8, 172u8, 234u8, 88u8, 80u8, 209u8, 23u8,
							141u8,
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

				#[doc = " Percentage of the curator fee that will be reserved upfront as deposit for bounty"]
				#[doc = " curator."]
				pub fn bounty_curator_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::per_things::Permill,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Bounties",
						"BountyCuratorDeposit",
						[
							225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
							80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
							177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
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
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
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
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
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
				#[doc = "Report something `reason` that deserves a tip and claim any eventual the finder's fee."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "Payment: `TipReportDepositBase` will be reserved from the origin account, as well as"]
				#[doc = "`DataDepositPerByte` for each byte in `reason`."]
				#[doc = ""]
				#[doc = "- `reason`: The reason for, or the thing that deserves, the tip; generally this will be"]
				#[doc = "  a UTF-8-encoded URL."]
				#[doc = "- `who`: The account which should be credited for the tip."]
				#[doc = ""]
				#[doc = "Emits `NewTip` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: `O(R)` where `R` length of `reason`."]
				#[doc = "  - encoding and hashing of 'reason'"]
				#[doc = "- DbReads: `Reasons`, `Tips`"]
				#[doc = "- DbWrites: `Reasons`, `Tips`"]
				#[doc = "# </weight>"]
				pub fn report_awesome(
					&self,
					reason: ::std::vec::Vec<::core::primitive::u8>,
					who: ::subxt::ext::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<ReportAwesome> {
					::subxt::tx::StaticTxPayload::new(
						"Tips",
						"report_awesome",
						ReportAwesome { reason, who },
						[
							43u8, 6u8, 185u8, 209u8, 110u8, 99u8, 94u8, 100u8, 33u8, 5u8, 27u8,
							199u8, 67u8, 255u8, 252u8, 26u8, 104u8, 192u8, 55u8, 122u8, 106u8,
							129u8, 249u8, 181u8, 246u8, 205u8, 213u8, 175u8, 241u8, 59u8, 151u8,
							197u8,
						],
					)
				}

				#[doc = "Retract a prior tip-report from `report_awesome`, and cancel the process of tipping."]
				#[doc = ""]
				#[doc = "If successful, the original deposit will be unreserved."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ and the tip identified by `hash`"]
				#[doc = "must have been reported by the signing account through `report_awesome` (and not"]
				#[doc = "through `tip_new`)."]
				#[doc = ""]
				#[doc = "- `hash`: The identity of the open tip for which a tip value is declared. This is formed"]
				#[doc = "  as the hash of the tuple of the original tip `reason` and the beneficiary account ID."]
				#[doc = ""]
				#[doc = "Emits `TipRetracted` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: `O(1)`"]
				#[doc = "  - Depends on the length of `T::Hash` which is fixed."]
				#[doc = "- DbReads: `Tips`, `origin account`"]
				#[doc = "- DbWrites: `Reasons`, `Tips`, `origin account`"]
				#[doc = "# </weight>"]
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

				#[doc = "Give a tip for something new; no finder's fee will be taken."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ and the signing account must be a"]
				#[doc = "member of the `Tippers` set."]
				#[doc = ""]
				#[doc = "- `reason`: The reason for, or the thing that deserves, the tip; generally this will be"]
				#[doc = "  a UTF-8-encoded URL."]
				#[doc = "- `who`: The account which should be credited for the tip."]
				#[doc = "- `tip_value`: The amount of tip that the sender would like to give. The median tip"]
				#[doc = "  value of active tippers will be given to the `who`."]
				#[doc = ""]
				#[doc = "Emits `NewTip` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: `O(R + T)` where `R` length of `reason`, `T` is the number of tippers."]
				#[doc = "  - `O(T)`: decoding `Tipper` vec of length `T`. `T` is charged as upper bound given by"]
				#[doc = "    `ContainsLengthBound`. The actual cost depends on the implementation of"]
				#[doc = "    `T::Tippers`."]
				#[doc = "  - `O(R)`: hashing and encoding of reason of length `R`"]
				#[doc = "- DbReads: `Tippers`, `Reasons`"]
				#[doc = "- DbWrites: `Reasons`, `Tips`"]
				#[doc = "# </weight>"]
				pub fn tip_new(
					&self,
					reason: ::std::vec::Vec<::core::primitive::u8>,
					who: ::subxt::ext::sp_core::crypto::AccountId32,
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
							146u8, 216u8, 159u8, 132u8, 163u8, 180u8, 42u8, 203u8, 181u8, 76u8,
							217u8, 120u8, 75u8, 32u8, 165u8, 41u8, 250u8, 222u8, 204u8, 63u8, 61u8,
							218u8, 161u8, 37u8, 172u8, 10u8, 66u8, 218u8, 14u8, 130u8, 160u8,
							126u8,
						],
					)
				}

				#[doc = "Declare a tip value for an already-open tip."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ and the signing account must be a"]
				#[doc = "member of the `Tippers` set."]
				#[doc = ""]
				#[doc = "- `hash`: The identity of the open tip for which a tip value is declared. This is formed"]
				#[doc = "  as the hash of the tuple of the hash of the original tip `reason` and the beneficiary"]
				#[doc = "  account ID."]
				#[doc = "- `tip_value`: The amount of tip that the sender would like to give. The median tip"]
				#[doc = "  value of active tippers will be given to the `who`."]
				#[doc = ""]
				#[doc = "Emits `TipClosing` if the threshold of tippers has been reached and the countdown period"]
				#[doc = "has started."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: `O(T)` where `T` is the number of tippers. decoding `Tipper` vec of length"]
				#[doc = "  `T`, insert tip and check closing, `T` is charged as upper bound given by"]
				#[doc = "  `ContainsLengthBound`. The actual cost depends on the implementation of `T::Tippers`."]
				#[doc = ""]
				#[doc = "  Actually weight could be lower as it depends on how many tips are in `OpenTip` but it"]
				#[doc = "  is weighted as if almost full i.e of length `T-1`."]
				#[doc = "- DbReads: `Tippers`, `Tips`"]
				#[doc = "- DbWrites: `Tips`"]
				#[doc = "# </weight>"]
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

				#[doc = "Close and payout a tip."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "The tip identified by `hash` must have finished its countdown period."]
				#[doc = ""]
				#[doc = "- `hash`: The identity of the open tip for which a tip value is declared. This is formed"]
				#[doc = "  as the hash of the tuple of the original tip `reason` and the beneficiary account ID."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: `O(T)` where `T` is the number of tippers. decoding `Tipper` vec of length"]
				#[doc = "  `T`. `T` is charged as upper bound given by `ContainsLengthBound`. The actual cost"]
				#[doc = "  depends on the implementation of `T::Tippers`."]
				#[doc = "- DbReads: `Tips`, `Tippers`, `tip finder`"]
				#[doc = "- DbWrites: `Reasons`, `Tips`, `Tippers`, `tip finder`"]
				#[doc = "# </weight>"]
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

				#[doc = "Remove and slash an already-open tip."]
				#[doc = ""]
				#[doc = "May only be called from `T::RejectOrigin`."]
				#[doc = ""]
				#[doc = "As a result, the finder is slashed and the deposits are lost."]
				#[doc = ""]
				#[doc = "Emits `TipSlashed` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "  `T` is charged as upper bound given by `ContainsLengthBound`."]
				#[doc = "  The actual cost depends on the implementation of `T::Tippers`."]
				#[doc = "# </weight>"]
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
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
	pub mod bags_list {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				pub dislocated: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Declare that some `dislocated` account has, through rewards or penalties, sufficiently"]
				#[doc = "changed its weight that it should properly fall into a different bag than its current"]
				#[doc = "one."]
				#[doc = ""]
				#[doc = "Anyone can call this function about any potentially dislocated account."]
				#[doc = ""]
				#[doc = "Will never return an error; if `dislocated` does not exist or doesn't need a rebag, then"]
				#[doc = "it is a noop and fees are still collected from `origin`."]
				pub fn rebag(
					&self,
					dislocated: ::subxt::ext::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<Rebag> {
					::subxt::tx::StaticTxPayload::new("BagsList", "rebag", Rebag { dislocated }, [
						8u8, 182u8, 221u8, 221u8, 242u8, 48u8, 178u8, 182u8, 236u8, 54u8, 188u8,
						107u8, 32u8, 24u8, 90u8, 76u8, 28u8, 67u8, 8u8, 231u8, 6u8, 162u8, 169u8,
						77u8, 246u8, 88u8, 156u8, 189u8, 248u8, 19u8, 235u8, 236u8,
					])
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
				const PALLET: &'static str = "BagsList";
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
						"BagsList",
						"ListNodes",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							88u8, 95u8, 226u8, 81u8, 86u8, 125u8, 195u8, 19u8, 147u8, 16u8, 211u8,
							169u8, 178u8, 91u8, 135u8, 35u8, 14u8, 201u8, 48u8, 198u8, 233u8, 57u8,
							33u8, 155u8, 127u8, 223u8, 35u8, 128u8, 167u8, 102u8, 105u8, 68u8,
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
						"BagsList",
						"ListNodes",
						Vec::new(),
						[
							88u8, 95u8, 226u8, 81u8, 86u8, 125u8, 195u8, 19u8, 147u8, 16u8, 211u8,
							169u8, 178u8, 91u8, 135u8, 35u8, 14u8, 201u8, 48u8, 198u8, 233u8, 57u8,
							33u8, 155u8, 127u8, 223u8, 35u8, 128u8, 167u8, 102u8, 105u8, 68u8,
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
						"BagsList",
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
						"BagsList",
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
						"BagsList",
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
				#[doc = " Ids are separated into unsorted bags according to their vote weight. This specifies the"]
				#[doc = " thresholds separating the bags. An id's bag is the largest bag for which the id's weight"]
				#[doc = " is less than or equal to its upper threshold."]
				#[doc = ""]
				#[doc = " When ids are iterated, higher bags are iterated completely before lower bags. This means"]
				#[doc = " that iteration is _semi-sorted_: ids of higher weight tend to come before ids of lower"]
				#[doc = " weight, but peer ids within a particular bag are sorted in insertion order."]
				#[doc = ""]
				#[doc = " # Expressing the constant"]
				#[doc = ""]
				#[doc = " This constant must be sorted in strictly increasing order. Duplicate items are not"]
				#[doc = " permitted."]
				#[doc = ""]
				#[doc = " There is an implied upper limit of `VoteWeight::MAX`; that value does not need to be"]
				#[doc = " specified within the bag. For any two threshold lists, if one ends with"]
				#[doc = " `VoteWeight::MAX`, the other one does not, and they are otherwise equal, the two lists"]
				#[doc = " will behave identically."]
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
				#[doc = " - If the threshold list begins `[1, 2, 3, ...]`, then an id with weight 0 or 1 will fall"]
				#[doc = "   into bag 0, an id with weight 2 will fall into bag 1, etc."]
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
					::subxt::constants::StaticConstantAddress::new("BagsList", "BagThresholds", [
						103u8, 102u8, 255u8, 165u8, 124u8, 54u8, 5u8, 172u8, 112u8, 234u8, 25u8,
						175u8, 178u8, 19u8, 251u8, 73u8, 91u8, 192u8, 227u8, 81u8, 249u8, 45u8,
						126u8, 116u8, 7u8, 37u8, 9u8, 200u8, 167u8, 182u8, 12u8, 131u8,
					])
				}
			}
		}
	}
	pub mod data_availability {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
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
				pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
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
				#[doc = "Creates an application key if `key` does not exist yet."]
				pub fn create_application_key(
					&self,
					key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
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

				pub fn submit_data(
					&self,
					data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
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
				pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
					::core::primitive::u8,
				>,
				pub owner: ::subxt::ext::sp_core::crypto::AccountId32,
				pub id: runtime_types::da_primitives::asdr::AppId,
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
				pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
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
				pub rows: ::core::primitive::u32,
				pub cols: ::core::primitive::u32,
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
					::subxt::metadata::DecodeStaticType<runtime_types::da_primitives::asdr::AppId>,
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

				#[doc = " Last block length proposal."]
				#[doc = " # TODO"]
				#[doc = " - It is not used, could we removed it?"]
				pub fn last_block_len_id(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"DataAvailability",
						"LastBlockLenId",
						vec![],
						[
							51u8, 88u8, 3u8, 49u8, 185u8, 211u8, 12u8, 141u8, 53u8, 34u8, 47u8,
							67u8, 163u8, 215u8, 249u8, 135u8, 80u8, 9u8, 236u8, 177u8, 219u8, 94u8,
							56u8, 168u8, 176u8, 149u8, 104u8, 19u8, 20u8, 189u8, 124u8, 213u8,
						],
					)
				}

				#[doc = " Store all application keys."]
				pub fn app_keys(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::frame_support::storage::bounded_vec::BoundedVec<
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
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"DataAvailability",
						"MinBlockRows",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " Maximum number of rows in a block."]
				pub fn max_block_rows(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"DataAvailability",
						"MaxBlockRows",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " Minimum number of cols in a block."]
				pub fn min_block_cols(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"DataAvailability",
						"MinBlockCols",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}

				#[doc = " Maximum number of cols in a block."]
				pub fn max_block_cols(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"DataAvailability",
						"MaxBlockCols",
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
	pub mod updater_manager {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::updater_manager::pallet::Event;
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
				const PALLET: &'static str = "UpdaterManager";
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
				const PALLET: &'static str = "UpdaterManager";
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
						"UpdaterManager",
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
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				pub message_body: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
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
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Dispatch a message to the destination domain and recipient address."]
				pub fn dispatch(
					&self,
					destination_domain: ::core::primitive::u32,
					recipient_address: ::subxt::ext::sp_core::H256,
					message_body: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
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

				#[doc = "Verify/submit signed update."]
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

				#[doc = "Verify/slash updater for improper update."]
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
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
					::subxt::metadata::DecodeStaticType<runtime_types::merkle::light::LightMerkle>,
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
	pub mod da_bridge {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				pub header: runtime_types::da_primitives::header::Header<
					::core::primitive::u32,
					runtime_types::sp_runtime::traits::BlakeTwo256,
				>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Dispatch a data root message to the home if the header is valid."]
				pub fn try_dispatch_data_root(
					&self,
					destination_domain: ::core::primitive::u32,
					recipient_address: ::subxt::ext::sp_core::H256,
					header: runtime_types::da_primitives::header::Header<
						::core::primitive::u32,
						runtime_types::sp_runtime::traits::BlakeTwo256,
					>,
				) -> ::subxt::tx::StaticTxPayload<TryDispatchDataRoot> {
					::subxt::tx::StaticTxPayload::new(
						"DABridge",
						"try_dispatch_data_root",
						TryDispatchDataRoot {
							destination_domain,
							recipient_address,
							header,
						},
						[
							132u8, 48u8, 32u8, 77u8, 255u8, 172u8, 85u8, 111u8, 151u8, 3u8, 234u8,
							150u8, 17u8, 172u8, 88u8, 252u8, 112u8, 62u8, 63u8, 77u8, 1u8, 100u8,
							221u8, 161u8, 44u8, 172u8, 130u8, 194u8, 73u8, 222u8, 215u8, 242u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::da_bridge::pallet::Event;
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
				const PALLET: &'static str = "DABridge";
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
						"DABridge",
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
	pub mod runtime_types {
		use super::runtime_types;
		pub mod da_bridge {
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Dispatch a data root message to the home if the header is valid."]
					try_dispatch_data_root {
						#[codec(compact)]
						destination_domain: ::core::primitive::u32,
						recipient_address: ::subxt::ext::sp_core::H256,
						header: runtime_types::da_primitives::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
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
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
					pub struct CheckAppId(pub runtime_types::da_primitives::asdr::AppId);
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
					pub id: runtime_types::da_primitives::asdr::AppId,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Creates an application key if `key` does not exist yet."]
					create_application_key {
						key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					},
					#[codec(index = 1)]
					submit_data {
						data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					},
					#[codec(index = 2)]
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
					#[doc = "The last application ID overflowed."]
					LastAppIdOverflowed,
					#[codec(index = 2)]
					#[doc = "The last block length proposal Id overflowed."]
					LastBlockLenProposalIdOverflowed,
					#[codec(index = 3)]
					BlockDimensionsOutOfBounds,
					#[codec(index = 4)]
					BlockDimensionsTooSmall,
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
						key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
						owner: ::subxt::ext::sp_core::crypto::AccountId32,
						id: runtime_types::da_primitives::asdr::AppId,
					},
					#[codec(index = 1)]
					DataSubmitted {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					},
					#[codec(index = 2)]
					BlockLengthProposalSubmitted {
						rows: ::core::primitive::u32,
						cols: ::core::primitive::u32,
					},
				}
			}
		}
		pub mod da_primitives {
			use super::runtime_types;
			pub mod asdr {
				use super::runtime_types;
				pub mod app_unchecked_extrinsic {
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Default,
						Eq,
						PartialEq,
						serde :: Deserialize,
						serde :: Serialize,
					)]
					pub struct DataLookup {
						#[codec(compact)]
						pub size: ::core::primitive::u32,
						pub index: ::std::vec::Vec<
							runtime_types::da_primitives::asdr::data_lookup::DataLookupIndexItem,
						>,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
						serde :: Deserialize,
						serde :: Serialize,
					)]
					pub struct DataLookupIndexItem {
						pub app_id: runtime_types::da_primitives::asdr::AppId,
						#[codec(compact)]
						pub start: ::core::primitive::u32,
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
				pub struct AppExtrinsic {
					pub app_id: runtime_types::da_primitives::asdr::AppId,
					pub data: ::std::vec::Vec<::core::primitive::u8>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Copy,
					Debug,
					Default,
					Eq,
					PartialEq,
					derive_more :: From,
					serde :: Deserialize,
					serde :: Serialize,
				)]
				pub struct AppId(#[codec(compact)] pub ::core::primitive::u32);
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
							Default,
							Eq,
							PartialEq,
							serde :: Deserialize,
							serde :: Serialize,
						)]
						pub struct HeaderExtension {
							pub commitment:
								runtime_types::da_primitives::kate_commitment::KateCommitment,
							pub app_lookup:
								runtime_types::da_primitives::asdr::data_lookup::DataLookup,
						}
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
						serde :: Deserialize,
						serde :: Serialize,
					)]
					pub enum HeaderExtension {
						#[codec(index = 0)]
						V1(runtime_types::da_primitives::header::extension::v1::HeaderExtension),
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
					pub extension: runtime_types::da_primitives::header::extension::HeaderExtension,
					#[codec(skip)]
					pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
				}
			}
			pub mod kate_commitment {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Default,
					Eq,
					PartialEq,
					serde :: Deserialize,
					serde :: Serialize,
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
		}
		pub mod da_runtime {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub enum Call {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Call),
				#[codec(index = 1)]
				Utility(runtime_types::pallet_utility::pallet::Call),
				#[codec(index = 2)]
				Babe(runtime_types::pallet_babe::pallet::Call),
				#[codec(index = 3)]
				Timestamp(runtime_types::pallet_timestamp::pallet::Call),
				#[codec(index = 4)]
				Authorship(runtime_types::pallet_authorship::pallet::Call),
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
				#[codec(index = 12)]
				Democracy(runtime_types::pallet_democracy::pallet::Call),
				#[codec(index = 13)]
				Council(runtime_types::pallet_collective::pallet::Call),
				#[codec(index = 14)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Call),
				#[codec(index = 15)]
				Elections(runtime_types::pallet_elections_phragmen::pallet::Call),
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
				#[codec(index = 28)]
				BagsList(runtime_types::pallet_bags_list::pallet::Call),
				#[codec(index = 29)]
				DataAvailability(runtime_types::da_control::pallet::Call),
				#[codec(index = 30)]
				UpdaterManager(runtime_types::updater_manager::pallet::Call),
				#[codec(index = 31)]
				NomadHome(runtime_types::nomad_home::pallet::Call),
				#[codec(index = 32)]
				DABridge(runtime_types::da_bridge::pallet::Call),
			}
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
				System(runtime_types::frame_system::pallet::Event),
				#[codec(index = 1)]
				Utility(runtime_types::pallet_utility::pallet::Event),
				#[codec(index = 5)]
				Indices(runtime_types::pallet_indices::pallet::Event),
				#[codec(index = 6)]
				Balances(runtime_types::pallet_balances::pallet::Event),
				#[codec(index = 9)]
				ElectionProviderMultiPhase(
					runtime_types::pallet_election_provider_multi_phase::pallet::Event,
				),
				#[codec(index = 10)]
				Staking(runtime_types::pallet_staking::pallet::pallet::Event),
				#[codec(index = 11)]
				Session(runtime_types::pallet_session::pallet::Event),
				#[codec(index = 12)]
				Democracy(runtime_types::pallet_democracy::pallet::Event),
				#[codec(index = 13)]
				Council(runtime_types::pallet_collective::pallet::Event),
				#[codec(index = 14)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Event),
				#[codec(index = 15)]
				Elections(runtime_types::pallet_elections_phragmen::pallet::Event),
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
				#[codec(index = 28)]
				BagsList(runtime_types::pallet_bags_list::pallet::Event),
				#[codec(index = 29)]
				DataAvailability(runtime_types::da_control::pallet::Event),
				#[codec(index = 30)]
				UpdaterManager(runtime_types::updater_manager::pallet::Event),
				#[codec(index = 31)]
				NomadHome(runtime_types::nomad_home::pallet::Event),
				#[codec(index = 32)]
				DABridge(runtime_types::da_bridge::pallet::Event),
			}
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
					runtime_types::frame_system::RawOrigin<
						::subxt::ext::sp_core::crypto::AccountId32,
					>,
				),
				#[codec(index = 13)]
				Council(
					runtime_types::pallet_collective::RawOrigin<
						::subxt::ext::sp_core::crypto::AccountId32,
					>,
				),
				#[codec(index = 14)]
				TechnicalCommittee(
					runtime_types::pallet_collective::RawOrigin<
						::subxt::ext::sp_core::crypto::AccountId32,
					>,
				),
				#[codec(index = 3)]
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
			pub struct SessionKeys {
				pub babe: runtime_types::sp_consensus_babe::app::Public,
				pub grandpa: runtime_types::sp_finality_grandpa::app::Public,
				pub im_online: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
				pub authority_discovery: runtime_types::sp_authority_discovery::app::Public,
			}
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
			pub mod storage {
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
			pub mod traits {
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
					pub struct WrapperOpaque<_0>(
						#[codec(compact)] pub ::core::primitive::u32,
						pub _0,
					);
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
			pub mod weights {
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
					pub weight: ::core::primitive::u64,
					pub class: runtime_types::frame_support::weights::DispatchClass,
					pub pays_fee: runtime_types::frame_support::weights::Pays,
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
				pub struct RuntimeDbWeight {
					pub read: ::core::primitive::u64,
					pub write: ::core::primitive::u64,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct WeightToFeeCoefficient<_0> {
					pub coeff_integer: _0,
					pub coeff_frac: runtime_types::sp_arithmetic::per_things::Perbill,
					pub negative: ::core::primitive::bool,
					pub degree: ::core::primitive::u8,
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
					pub max: runtime_types::frame_support::weights::PerDispatchClass<
						::core::primitive::u32,
					>,
					#[codec(compact)]
					pub cols: ::core::primitive::u32,
					#[codec(compact)]
					pub rows: ::core::primitive::u32,
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
					pub base_block: ::core::primitive::u64,
					pub max_block: ::core::primitive::u64,
					pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
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
					pub base_extrinsic: ::core::primitive::u64,
					pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
					pub max_total: ::core::option::Option<::core::primitive::u64>,
					pub reserved: ::core::option::Option<::core::primitive::u64>,
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "A dispatch that will fill the block weight up to the given ratio."]
					fill_block {
						ratio: runtime_types::sp_arithmetic::per_things::Perbill,
					},
					#[codec(index = 1)]
					#[doc = "Make some on-chain remark."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`"]
					#[doc = "# </weight>"]
					remark {
						remark: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 2)]
					#[doc = "Set the number of pages in the WebAssembly environment's heap."]
					set_heap_pages { pages: ::core::primitive::u64 },
					#[codec(index = 3)]
					#[doc = "Set the new runtime code."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
					#[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
					#[doc = "  expensive)."]
					#[doc = "- 1 storage write (codec `O(C)`)."]
					#[doc = "- 1 digest item."]
					#[doc = "- 1 event."]
					#[doc = "The weight of this function is dependent on the runtime, but generally this is very expensive."]
					#[doc = "We will treat this as a full block."]
					#[doc = "# </weight>"]
					set_code {
						code: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 4)]
					#[doc = "Set the new runtime code without doing any checks of the given `code`."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(C)` where `C` length of `code`"]
					#[doc = "- 1 storage write (codec `O(C)`)."]
					#[doc = "- 1 digest item."]
					#[doc = "- 1 event."]
					#[doc = "The weight of this function is dependent on the runtime. We will treat this as a full block."]
					#[doc = "# </weight>"]
					set_code_without_checks {
						code: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 5)]
					#[doc = "Set some items of storage."]
					set_storage {
						items: ::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					},
					#[codec(index = 6)]
					#[doc = "Kill some items from storage."]
					kill_storage {
						keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 7)]
					#[doc = "Kill all storage items with a key that starts with the given prefix."]
					#[doc = ""]
					#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
					#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
					kill_prefix {
						prefix: ::std::vec::Vec<::core::primitive::u8>,
						subkeys: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					#[doc = "Make some on-chain remark and emit event."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(b)` where b is the length of the remark."]
					#[doc = "- 1 event."]
					#[doc = "# </weight>"]
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
					#[doc = "An extrinsic completed successfully. \\[info\\]"]
					ExtrinsicSuccess(runtime_types::frame_support::weights::DispatchInfo),
					#[codec(index = 1)]
					#[doc = "An extrinsic failed. \\[error, info\\]"]
					ExtrinsicFailed(
						runtime_types::sp_runtime::DispatchError,
						runtime_types::frame_support::weights::DispatchInfo,
					),
					#[codec(index = 2)]
					#[doc = "`:code` was updated."]
					CodeUpdated,
					#[codec(index = 3)]
					#[doc = "A new \\[account\\] was created."]
					NewAccount(::subxt::ext::sp_core::crypto::AccountId32),
					#[codec(index = 4)]
					#[doc = "An \\[account\\] was reaped."]
					KilledAccount(::subxt::ext::sp_core::crypto::AccountId32),
					#[codec(index = 5)]
					#[doc = "On on-chain remark happened. \\[origin, remark_hash\\]"]
					Remarked(
						::subxt::ext::sp_core::crypto::AccountId32,
						::subxt::ext::sp_core::H256,
					),
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
		pub mod merkle {
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
					pub signature: runtime_types::signature::signature::Signature,
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Dispatch a message to the destination domain and recipient address."]
					dispatch {
						#[codec(compact)]
						destination_domain: ::core::primitive::u32,
						recipient_address: ::subxt::ext::sp_core::H256,
						message_body:
							runtime_types::frame_support::storage::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
					},
					#[codec(index = 1)]
					#[doc = "Verify/submit signed update."]
					update {
						signed_update: runtime_types::nomad_core::update::SignedUpdate,
						#[codec(compact)]
						max_index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Verify/slash updater for improper update."]
					improper_update {
						signed_update: runtime_types::nomad_core::update::SignedUpdate,
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
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
		pub mod pallet_authorship {
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Provide a set of uncles."]
					set_uncles {
						new_uncles: ::std::vec::Vec<
							runtime_types::da_primitives::header::Header<
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
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The uncle parent not in the chain."]
					InvalidUncleParent,
					#[codec(index = 1)]
					#[doc = "Uncles already set in the block."]
					UnclesAlreadySet,
					#[codec(index = 2)]
					#[doc = "Too many uncles."]
					TooManyUncles,
					#[codec(index = 3)]
					#[doc = "The uncle is genesis."]
					GenesisUncle,
					#[codec(index = 4)]
					#[doc = "The uncle is too high in chain."]
					TooHighUncle,
					#[codec(index = 5)]
					#[doc = "The uncle is already included."]
					UncleAlreadyIncluded,
					#[codec(index = 6)]
					#[doc = "The uncle isn't recent enough to be included."]
					OldUncle,
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
			pub enum UncleEntryItem<_0, _1, _2> {
				#[codec(index = 0)]
				InclusionHeight(_0),
				#[codec(index = 1)]
				Uncle(_1, ::core::option::Option<_2>),
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Report authority equivocation/misbehavior. This method will verify"]
					#[doc = "the equivocation proof and validate the given key ownership proof"]
					#[doc = "against the extracted offender. If both are valid, the offence will"]
					#[doc = "be reported."]
					report_equivocation {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_consensus_slots::EquivocationProof<
								runtime_types::da_primitives::header::Header<
									::core::primitive::u32,
									runtime_types::sp_runtime::traits::BlakeTwo256,
								>,
								runtime_types::sp_consensus_babe::app::Public,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 1)]
					#[doc = "Report authority equivocation/misbehavior. This method will verify"]
					#[doc = "the equivocation proof and validate the given key ownership proof"]
					#[doc = "against the extracted offender. If both are valid, the offence will"]
					#[doc = "be reported."]
					#[doc = "This extrinsic must be called unsigned and it is expected that only"]
					#[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
					#[doc = "if the block author is defined it will be defined as the equivocation"]
					#[doc = "reporter."]
					report_equivocation_unsigned {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_consensus_slots::EquivocationProof<
								runtime_types::da_primitives::header::Header<
									::core::primitive::u32,
									runtime_types::sp_runtime::traits::BlakeTwo256,
								>,
								runtime_types::sp_consensus_babe::app::Public,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 2)]
					#[doc = "Plan an epoch config change. The epoch config change is recorded and will be enacted on"]
					#[doc = "the next call to `enact_epoch_change`. The config will be activated one epoch after."]
					#[doc = "Multiple calls to this method will replace any existing planned config change that had"]
					#[doc = "not been enacted yet."]
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
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
				pub struct Node {
					pub id: ::subxt::ext::sp_core::crypto::AccountId32,
					pub prev: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
					pub next: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
					pub bag_upper: ::core::primitive::u64,
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Declare that some `dislocated` account has, through rewards or penalties, sufficiently"]
					#[doc = "changed its weight that it should properly fall into a different bag than its current"]
					#[doc = "one."]
					#[doc = ""]
					#[doc = "Anyone can call this function about any potentially dislocated account."]
					#[doc = ""]
					#[doc = "Will never return an error; if `dislocated` does not exist or doesn't need a rebag, then"]
					#[doc = "it is a noop and fees are still collected from `origin`."]
					rebag {
						dislocated: ::subxt::ext::sp_core::crypto::AccountId32,
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
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Moved an account from one bag to another."]
					Rebagged {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						from: ::core::primitive::u64,
						to: ::core::primitive::u64,
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Transfer some liquid free balance to another account."]
					#[doc = ""]
					#[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
					#[doc = "It will decrease the total issuance of the system by the `TransferFee`."]
					#[doc = "If the sender's account is below the existential deposit as a result"]
					#[doc = "of the transfer, the account will be reaped."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
					#[doc = "  types. See related functions below."]
					#[doc = "- It contains a limited number of reads and writes internally and no complex"]
					#[doc = "  computation."]
					#[doc = ""]
					#[doc = "Related functions:"]
					#[doc = ""]
					#[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
					#[doc = "  - Transferring balances to accounts that did not exist before will cause"]
					#[doc = "    `T::OnNewAccount::on_new_account` to be called."]
					#[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
					#[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
					#[doc = "    that the transfer will not kill the origin account."]
					#[doc = "---------------------------------"]
					#[doc = "- Origin account is already in memory, so no DB operations for them."]
					#[doc = "# </weight>"]
					transfer {
						dest: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "Set the balances of a given account."]
					#[doc = ""]
					#[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
					#[doc = "also decrease the total issuance of the system (`TotalIssuance`)."]
					#[doc = "If the new free or reserved balance is below the existential deposit,"]
					#[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call is `root`."]
					set_balance {
						who: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
						#[codec(compact)]
						new_reserved: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
					#[doc = "specified."]
					#[doc = "# <weight>"]
					#[doc = "- Same as transfer, but additional read and write because the source account is not"]
					#[doc = "  assumed to be in the overlay."]
					#[doc = "# </weight>"]
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
					#[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
					#[doc = "origin account."]
					#[doc = ""]
					#[doc = "99% of the time you want [`transfer`] instead."]
					#[doc = ""]
					#[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
					transfer_keep_alive {
						dest: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Transfer the entire transferable balance from the caller account."]
					#[doc = ""]
					#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
					#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
					#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
					#[doc = "you might need to prepare the account by removing any reference counters, storage"]
					#[doc = "deposits, etc..."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be Signed."]
					#[doc = ""]
					#[doc = "- `dest`: The recipient of the transfer."]
					#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
					#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
					#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
					#[doc = "  keep the sender account alive (true). # <weight>"]
					#[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
					#[doc = "  #</weight>"]
					transfer_all {
						dest: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					#[doc = "Unreserve some balance from a user by force."]
					#[doc = ""]
					#[doc = "Can only be called by ROOT."]
					force_unreserve {
						who: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						amount: ::core::primitive::u128,
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
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Vesting balance too high to send value"]
					VestingBalance,
					#[codec(index = 1)]
					#[doc = "Account liquidity restrictions prevent withdrawal"]
					LiquidityRestrictions,
					#[codec(index = 2)]
					#[doc = "Balance too low to send value"]
					InsufficientBalance,
					#[codec(index = 3)]
					#[doc = "Value too low to create account due to existential deposit"]
					ExistentialDeposit,
					#[codec(index = 4)]
					#[doc = "Transfer/payment would kill account"]
					KeepAlive,
					#[codec(index = 5)]
					#[doc = "A vesting schedule already exists for this account"]
					ExistingVestingSchedule,
					#[codec(index = 6)]
					#[doc = "Beneficiary account must pre-exist"]
					DeadAccount,
					#[codec(index = 7)]
					#[doc = "Number of named reserves exceed MaxReserves"]
					TooManyReserves,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
						reserved: ::core::primitive::u128,
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
			pub struct AccountData<_0> {
				pub free: _0,
				pub reserved: _0,
				pub misc_frozen: _0,
				pub fee_frozen: _0,
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
				pub reasons: runtime_types::pallet_balances::Reasons,
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
			pub enum Releases {
				#[codec(index = 0)]
				V1_0_0,
				#[codec(index = 1)]
				V2_0_0,
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Propose a new bounty."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "Payment: `TipReportDepositBase` will be reserved from the origin account, as well as"]
					#[doc = "`DataDepositPerByte` for each byte in `reason`. It will be unreserved upon approval,"]
					#[doc = "or slashed when rejected."]
					#[doc = ""]
					#[doc = "- `curator`: The curator account whom will manage this bounty."]
					#[doc = "- `fee`: The curator fee."]
					#[doc = "- `value`: The total payment amount of this bounty, curator fee included."]
					#[doc = "- `description`: The description of this bounty."]
					propose_bounty {
						#[codec(compact)]
						value: ::core::primitive::u128,
						description: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "Approve a bounty proposal. At a later time, the bounty will be funded and become active"]
					#[doc = "and the original deposit will be returned."]
					#[doc = ""]
					#[doc = "May only be called from `T::ApproveOrigin`."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "# </weight>"]
					approve_bounty {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Assign a curator to a funded bounty."]
					#[doc = ""]
					#[doc = "May only be called from `T::ApproveOrigin`."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "# </weight>"]
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
					#[doc = "Unassign curator from a bounty."]
					#[doc = ""]
					#[doc = "This function can only be called by the `RejectOrigin` a signed origin."]
					#[doc = ""]
					#[doc = "If this function is called by the `RejectOrigin`, we assume that the curator is"]
					#[doc = "malicious or inactive. As a result, we will slash the curator when possible."]
					#[doc = ""]
					#[doc = "If the origin is the curator, we take this as a sign they are unable to do their job and"]
					#[doc = "they willingly give up. We could slash them, but for now we allow them to recover their"]
					#[doc = "deposit and exit without issue. (We may want to change this if it is abused.)"]
					#[doc = ""]
					#[doc = "Finally, the origin can be anyone if and only if the curator is \"inactive\". This allows"]
					#[doc = "anyone in the community to call out that a curator is not doing their due diligence, and"]
					#[doc = "we should pick a new curator. In this case the curator should also be slashed."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "# </weight>"]
					unassign_curator {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					#[doc = "Accept the curator role for a bounty."]
					#[doc = "A deposit will be reserved from curator and refund upon successful payout."]
					#[doc = ""]
					#[doc = "May only be called from the curator."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "# </weight>"]
					accept_curator {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					#[doc = "Award bounty to a beneficiary account. The beneficiary will be able to claim the funds"]
					#[doc = "after a delay."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be the curator of this bounty."]
					#[doc = ""]
					#[doc = "- `bounty_id`: Bounty ID to award."]
					#[doc = "- `beneficiary`: The beneficiary account whom will receive the payout."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "# </weight>"]
					award_bounty {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
						beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 6)]
					#[doc = "Claim the payout from an awarded bounty after payout delay."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be the beneficiary of this bounty."]
					#[doc = ""]
					#[doc = "- `bounty_id`: Bounty ID to claim."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "# </weight>"]
					claim_bounty {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 7)]
					#[doc = "Cancel a proposed or active bounty. All the funds will be sent to treasury and"]
					#[doc = "the curator deposit will be unreserved if possible."]
					#[doc = ""]
					#[doc = "Only `T::RejectOrigin` is able to cancel a bounty."]
					#[doc = ""]
					#[doc = "- `bounty_id`: Bounty ID to cancel."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "# </weight>"]
					close_bounty {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					#[doc = "Extend the expiry time of an active bounty."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be the curator of this bounty."]
					#[doc = ""]
					#[doc = "- `bounty_id`: Bounty ID to extend."]
					#[doc = "- `remark`: additional information."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "# </weight>"]
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
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Set the collective's membership."]
					#[doc = ""]
					#[doc = "- `new_members`: The new member list. Be nice to the chain and provide it sorted."]
					#[doc = "- `prime`: The prime member whose vote sets the default."]
					#[doc = "- `old_count`: The upper bound for the previous number of members in storage. Used for"]
					#[doc = "  weight estimation."]
					#[doc = ""]
					#[doc = "Requires root origin."]
					#[doc = ""]
					#[doc = "NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but"]
					#[doc = "      the weight estimations rely on it to estimate dispatchable weight."]
					#[doc = ""]
					#[doc = "# WARNING:"]
					#[doc = ""]
					#[doc = "The `pallet-collective` can also be managed by logic outside of the pallet through the"]
					#[doc = "implementation of the trait [`ChangeMembers`]."]
					#[doc = "Any call to `set_members` must be careful that the member set doesn't get out of sync"]
					#[doc = "with other logic managing the member set."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "## Weight"]
					#[doc = "- `O(MP + N)` where:"]
					#[doc = "  - `M` old-members-count (code- and governance-bounded)"]
					#[doc = "  - `N` new-members-count (code- and governance-bounded)"]
					#[doc = "  - `P` proposals-count (code-bounded)"]
					#[doc = "- DB:"]
					#[doc = "  - 1 storage mutation (codec `O(M)` read, `O(N)` write) for reading and writing the"]
					#[doc = "    members"]
					#[doc = "  - 1 storage read (codec `O(P)`) for reading the proposals"]
					#[doc = "  - `P` storage mutations (codec `O(M)`) for updating the votes for each proposal"]
					#[doc = "  - 1 storage write (codec `O(1)`) for deleting the old `prime` and setting the new one"]
					#[doc = "# </weight>"]
					set_members {
						new_members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						prime: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
						old_count: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "Dispatch a proposal from a member using the `Member` origin."]
					#[doc = ""]
					#[doc = "Origin must be a member of the collective."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "## Weight"]
					#[doc = "- `O(M + P)` where `M` members-count (code-bounded) and `P` complexity of dispatching"]
					#[doc = "  `proposal`"]
					#[doc = "- DB: 1 read (codec `O(M)`) + DB access of `proposal`"]
					#[doc = "- 1 event"]
					#[doc = "# </weight>"]
					execute {
						proposal: ::std::boxed::Box<runtime_types::da_runtime::Call>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Add a new proposal to either be voted on or executed directly."]
					#[doc = ""]
					#[doc = "Requires the sender to be member."]
					#[doc = ""]
					#[doc = "`threshold` determines whether `proposal` is executed directly (`threshold < 2`)"]
					#[doc = "or put up for voting."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "## Weight"]
					#[doc = "- `O(B + M + P1)` or `O(B + M + P2)` where:"]
					#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
					#[doc = "  - `M` is members-count (code- and governance-bounded)"]
					#[doc = "  - branching is influenced by `threshold` where:"]
					#[doc = "    - `P1` is proposal execution complexity (`threshold < 2`)"]
					#[doc = "    - `P2` is proposals-count (code-bounded) (`threshold >= 2`)"]
					#[doc = "- DB:"]
					#[doc = "  - 1 storage read `is_member` (codec `O(M)`)"]
					#[doc = "  - 1 storage read `ProposalOf::contains_key` (codec `O(1)`)"]
					#[doc = "  - DB accesses influenced by `threshold`:"]
					#[doc = "    - EITHER storage accesses done by `proposal` (`threshold < 2`)"]
					#[doc = "    - OR proposal insertion (`threshold <= 2`)"]
					#[doc = "      - 1 storage mutation `Proposals` (codec `O(P2)`)"]
					#[doc = "      - 1 storage mutation `ProposalCount` (codec `O(1)`)"]
					#[doc = "      - 1 storage write `ProposalOf` (codec `O(B)`)"]
					#[doc = "      - 1 storage write `Voting` (codec `O(M)`)"]
					#[doc = "  - 1 event"]
					#[doc = "# </weight>"]
					propose {
						#[codec(compact)]
						threshold: ::core::primitive::u32,
						proposal: ::std::boxed::Box<runtime_types::da_runtime::Call>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "Add an aye or nay vote for the sender to the given proposal."]
					#[doc = ""]
					#[doc = "Requires the sender to be a member."]
					#[doc = ""]
					#[doc = "Transaction fees will be waived if the member is voting on any particular proposal"]
					#[doc = "for the first time and the call is successful. Subsequent vote changes will charge a"]
					#[doc = "fee."]
					#[doc = "# <weight>"]
					#[doc = "## Weight"]
					#[doc = "- `O(M)` where `M` is members-count (code- and governance-bounded)"]
					#[doc = "- DB:"]
					#[doc = "  - 1 storage read `Members` (codec `O(M)`)"]
					#[doc = "  - 1 storage mutation `Voting` (codec `O(M)`)"]
					#[doc = "- 1 event"]
					#[doc = "# </weight>"]
					vote {
						proposal: ::subxt::ext::sp_core::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						approve: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					#[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
					#[doc = ""]
					#[doc = "May be called by any signed account in order to finish voting and close the proposal."]
					#[doc = ""]
					#[doc = "If called before the end of the voting period it will only close the vote if it is"]
					#[doc = "has enough votes to be approved or disapproved."]
					#[doc = ""]
					#[doc = "If called after the end of the voting period abstentions are counted as rejections"]
					#[doc = "unless there is a prime member set and the prime member cast an approval."]
					#[doc = ""]
					#[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
					#[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
					#[doc = ""]
					#[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
					#[doc = "proposal."]
					#[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
					#[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "## Weight"]
					#[doc = "- `O(B + M + P1 + P2)` where:"]
					#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
					#[doc = "  - `M` is members-count (code- and governance-bounded)"]
					#[doc = "  - `P1` is the complexity of `proposal` preimage."]
					#[doc = "  - `P2` is proposal-count (code-bounded)"]
					#[doc = "- DB:"]
					#[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
					#[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
					#[doc = "   `O(P2)`)"]
					#[doc = " - any mutations done while executing `proposal` (`P1`)"]
					#[doc = "- up to 3 events"]
					#[doc = "# </weight>"]
					close {
						proposal_hash: ::subxt::ext::sp_core::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						#[codec(compact)]
						proposal_weight_bound: ::core::primitive::u64,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					#[doc = "Disapprove a proposal, close, and remove it from the system, regardless of its current"]
					#[doc = "state."]
					#[doc = ""]
					#[doc = "Must be called by the Root origin."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "* `proposal_hash`: The hash of the proposal that should be disapproved."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Complexity: O(P) where P is the number of max proposals"]
					#[doc = "DB Weight:"]
					#[doc = "* Reads: Proposals"]
					#[doc = "* Writes: Voting, Proposals, ProposalOf"]
					#[doc = "# </weight>"]
					disapprove_proposal {
						proposal_hash: ::subxt::ext::sp_core::H256,
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
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
		pub mod pallet_democracy {
			use super::runtime_types;
			pub mod conviction {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum Conviction {
					#[codec(index = 0)]
					None,
					#[codec(index = 1)]
					Locked1x,
					#[codec(index = 2)]
					Locked2x,
					#[codec(index = 3)]
					Locked3x,
					#[codec(index = 4)]
					Locked4x,
					#[codec(index = 5)]
					Locked5x,
					#[codec(index = 6)]
					Locked6x,
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Propose a sensitive action to be taken."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_ and the sender must"]
					#[doc = "have funds to cover the deposit."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The hash of the proposal preimage."]
					#[doc = "- `value`: The amount of deposit (must be at least `MinimumDeposit`)."]
					#[doc = ""]
					#[doc = "Emits `Proposed`."]
					#[doc = ""]
					#[doc = "Weight: `O(p)`"]
					propose {
						proposal_hash: ::subxt::ext::sp_core::H256,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "Signals agreement with a particular proposal."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_ and the sender"]
					#[doc = "must have funds to cover the deposit, equal to the original deposit."]
					#[doc = ""]
					#[doc = "- `proposal`: The index of the proposal to second."]
					#[doc = "- `seconds_upper_bound`: an upper bound on the current number of seconds on this"]
					#[doc = "  proposal. Extrinsic is weighted according to this value with no refund."]
					#[doc = ""]
					#[doc = "Weight: `O(S)` where S is the number of seconds a proposal already has."]
					second {
						#[codec(compact)]
						proposal: ::core::primitive::u32,
						#[codec(compact)]
						seconds_upper_bound: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Vote in a referendum. If `vote.is_aye()`, the vote is to enact the proposal;"]
					#[doc = "otherwise it is a vote to keep the status quo."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `ref_index`: The index of the referendum to vote for."]
					#[doc = "- `vote`: The vote configuration."]
					#[doc = ""]
					#[doc = "Weight: `O(R)` where R is the number of referendums the voter has voted on."]
					vote {
						#[codec(compact)]
						ref_index: ::core::primitive::u32,
						vote: runtime_types::pallet_democracy::vote::AccountVote<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 3)]
					#[doc = "Schedule an emergency cancellation of a referendum. Cannot happen twice to the same"]
					#[doc = "referendum."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be `CancellationOrigin`."]
					#[doc = ""]
					#[doc = "-`ref_index`: The index of the referendum to cancel."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`."]
					emergency_cancel { ref_index: ::core::primitive::u32 },
					#[codec(index = 4)]
					#[doc = "Schedule a referendum to be tabled once it is legal to schedule an external"]
					#[doc = "referendum."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be `ExternalOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
					#[doc = ""]
					#[doc = "Weight: `O(V)` with V number of vetoers in the blacklist of proposal."]
					#[doc = "  Decoding vec of length V. Charged as maximum"]
					external_propose {
						proposal_hash: ::subxt::ext::sp_core::H256,
					},
					#[codec(index = 5)]
					#[doc = "Schedule a majority-carries referendum to be tabled next once it is legal to schedule"]
					#[doc = "an external referendum."]
					#[doc = ""]
					#[doc = "The dispatch of this call must be `ExternalMajorityOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
					#[doc = ""]
					#[doc = "Unlike `external_propose`, blacklisting has no effect on this and it may replace a"]
					#[doc = "pre-scheduled `external_propose` call."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					external_propose_majority {
						proposal_hash: ::subxt::ext::sp_core::H256,
					},
					#[codec(index = 6)]
					#[doc = "Schedule a negative-turnout-bias referendum to be tabled next once it is legal to"]
					#[doc = "schedule an external referendum."]
					#[doc = ""]
					#[doc = "The dispatch of this call must be `ExternalDefaultOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
					#[doc = ""]
					#[doc = "Unlike `external_propose`, blacklisting has no effect on this and it may replace a"]
					#[doc = "pre-scheduled `external_propose` call."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					external_propose_default {
						proposal_hash: ::subxt::ext::sp_core::H256,
					},
					#[codec(index = 7)]
					#[doc = "Schedule the currently externally-proposed majority-carries referendum to be tabled"]
					#[doc = "immediately. If there is no externally-proposed referendum currently, or if there is one"]
					#[doc = "but it is not a majority-carries referendum then it fails."]
					#[doc = ""]
					#[doc = "The dispatch of this call must be `FastTrackOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The hash of the current external proposal."]
					#[doc = "- `voting_period`: The period that is allowed for voting on this proposal. Increased to"]
					#[doc = "  `FastTrackVotingPeriod` if too low."]
					#[doc = "- `delay`: The number of block after voting has ended in approval and this should be"]
					#[doc = "  enacted. This doesn't have a minimum amount."]
					#[doc = ""]
					#[doc = "Emits `Started`."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					fast_track {
						proposal_hash: ::subxt::ext::sp_core::H256,
						voting_period: ::core::primitive::u32,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					#[doc = "Veto and blacklist the external proposal hash."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be `VetoOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The preimage hash of the proposal to veto and blacklist."]
					#[doc = ""]
					#[doc = "Emits `Vetoed`."]
					#[doc = ""]
					#[doc = "Weight: `O(V + log(V))` where V is number of `existing vetoers`"]
					veto_external {
						proposal_hash: ::subxt::ext::sp_core::H256,
					},
					#[codec(index = 9)]
					#[doc = "Remove a referendum."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Root_."]
					#[doc = ""]
					#[doc = "- `ref_index`: The index of the referendum to cancel."]
					#[doc = ""]
					#[doc = "# Weight: `O(1)`."]
					cancel_referendum {
						#[codec(compact)]
						ref_index: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					#[doc = "Cancel a proposal queued for enactment."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Root_."]
					#[doc = ""]
					#[doc = "- `which`: The index of the referendum to cancel."]
					#[doc = ""]
					#[doc = "Weight: `O(D)` where `D` is the items in the dispatch queue. Weighted as `D = 10`."]
					cancel_queued { which: ::core::primitive::u32 },
					#[codec(index = 11)]
					#[doc = "Delegate the voting power (with some given conviction) of the sending account."]
					#[doc = ""]
					#[doc = "The balance delegated is locked for as long as it's delegated, and thereafter for the"]
					#[doc = "time appropriate for the conviction's lock period."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_, and the signing account must either:"]
					#[doc = "  - be delegating already; or"]
					#[doc = "  - have no voting activity (if there is, then it will need to be removed/consolidated"]
					#[doc = "    through `reap_vote` or `unvote`)."]
					#[doc = ""]
					#[doc = "- `to`: The account whose voting the `target` account's voting power will follow."]
					#[doc = "- `conviction`: The conviction that will be attached to the delegated votes. When the"]
					#[doc = "  account is undelegated, the funds will be locked for the corresponding period."]
					#[doc = "- `balance`: The amount of the account's balance to be used in delegating. This must not"]
					#[doc = "  be more than the account's current balance."]
					#[doc = ""]
					#[doc = "Emits `Delegated`."]
					#[doc = ""]
					#[doc = "Weight: `O(R)` where R is the number of referendums the voter delegating to has"]
					#[doc = "  voted on. Weight is charged as if maximum votes."]
					delegate {
						to: ::subxt::ext::sp_core::crypto::AccountId32,
						conviction: runtime_types::pallet_democracy::conviction::Conviction,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 12)]
					#[doc = "Undelegate the voting power of the sending account."]
					#[doc = ""]
					#[doc = "Tokens may be unlocked following once an amount of time consistent with the lock period"]
					#[doc = "of the conviction with which the delegation was issued."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_ and the signing account must be"]
					#[doc = "currently delegating."]
					#[doc = ""]
					#[doc = "Emits `Undelegated`."]
					#[doc = ""]
					#[doc = "Weight: `O(R)` where R is the number of referendums the voter delegating to has"]
					#[doc = "  voted on. Weight is charged as if maximum votes."]
					undelegate,
					#[codec(index = 13)]
					#[doc = "Clears all public proposals."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Root_."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`."]
					clear_public_proposals,
					#[codec(index = 14)]
					#[doc = "Register the preimage for an upcoming proposal. This doesn't require the proposal to be"]
					#[doc = "in the dispatch queue but does require a deposit, returned once enacted."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `encoded_proposal`: The preimage of a proposal."]
					#[doc = ""]
					#[doc = "Emits `PreimageNoted`."]
					#[doc = ""]
					#[doc = "Weight: `O(E)` with E size of `encoded_proposal` (protected by a required deposit)."]
					note_preimage {
						encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 15)]
					#[doc = "Same as `note_preimage` but origin is `OperationalPreimageOrigin`."]
					note_preimage_operational {
						encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 16)]
					#[doc = "Register the preimage for an upcoming proposal. This requires the proposal to be"]
					#[doc = "in the dispatch queue. No deposit is needed. When this call is successful, i.e."]
					#[doc = "the preimage has not been uploaded before and matches some imminent proposal,"]
					#[doc = "no fee is paid."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `encoded_proposal`: The preimage of a proposal."]
					#[doc = ""]
					#[doc = "Emits `PreimageNoted`."]
					#[doc = ""]
					#[doc = "Weight: `O(E)` with E size of `encoded_proposal` (protected by a required deposit)."]
					note_imminent_preimage {
						encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 17)]
					#[doc = "Same as `note_imminent_preimage` but origin is `OperationalPreimageOrigin`."]
					note_imminent_preimage_operational {
						encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 18)]
					#[doc = "Remove an expired proposal preimage and collect the deposit."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The preimage hash of a proposal."]
					#[doc = "- `proposal_length_upper_bound`: an upper bound on length of the proposal. Extrinsic is"]
					#[doc = "  weighted according to this value with no refund."]
					#[doc = ""]
					#[doc = "This will only work after `VotingPeriod` blocks from the time that the preimage was"]
					#[doc = "noted, if it's the same account doing it. If it's a different account, then it'll only"]
					#[doc = "work an additional `EnactmentPeriod` later."]
					#[doc = ""]
					#[doc = "Emits `PreimageReaped`."]
					#[doc = ""]
					#[doc = "Weight: `O(D)` where D is length of proposal."]
					reap_preimage {
						proposal_hash: ::subxt::ext::sp_core::H256,
						#[codec(compact)]
						proposal_len_upper_bound: ::core::primitive::u32,
					},
					#[codec(index = 19)]
					#[doc = "Unlock tokens that have an expired lock."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `target`: The account to remove the lock on."]
					#[doc = ""]
					#[doc = "Weight: `O(R)` with R number of vote of target."]
					unlock {
						target: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 20)]
					#[doc = "Remove a vote for a referendum."]
					#[doc = ""]
					#[doc = "If:"]
					#[doc = "- the referendum was cancelled, or"]
					#[doc = "- the referendum is ongoing, or"]
					#[doc = "- the referendum has ended such that"]
					#[doc = "  - the vote of the account was in opposition to the result; or"]
					#[doc = "  - there was no conviction to the account's vote; or"]
					#[doc = "  - the account made a split vote"]
					#[doc = "...then the vote is removed cleanly and a following call to `unlock` may result in more"]
					#[doc = "funds being available."]
					#[doc = ""]
					#[doc = "If, however, the referendum has ended and:"]
					#[doc = "- it finished corresponding to the vote of the account, and"]
					#[doc = "- the account made a standard vote with conviction, and"]
					#[doc = "- the lock period of the conviction is not over"]
					#[doc = "...then the lock will be aggregated into the overall account's lock, which may involve"]
					#[doc = "*overlocking* (where the two locks are combined into a single lock that is the maximum"]
					#[doc = "of both the amount locked and the time is it locked for)."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_, and the signer must have a vote"]
					#[doc = "registered for referendum `index`."]
					#[doc = ""]
					#[doc = "- `index`: The index of referendum of the vote to be removed."]
					#[doc = ""]
					#[doc = "Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on."]
					#[doc = "  Weight is calculated for the maximum number of vote."]
					remove_vote { index: ::core::primitive::u32 },
					#[codec(index = 21)]
					#[doc = "Remove a vote for a referendum."]
					#[doc = ""]
					#[doc = "If the `target` is equal to the signer, then this function is exactly equivalent to"]
					#[doc = "`remove_vote`. If not equal to the signer, then the vote must have expired,"]
					#[doc = "either because the referendum was cancelled, because the voter lost the referendum or"]
					#[doc = "because the conviction period is over."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `target`: The account of the vote to be removed; this account must have voted for"]
					#[doc = "  referendum `index`."]
					#[doc = "- `index`: The index of referendum of the vote to be removed."]
					#[doc = ""]
					#[doc = "Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on."]
					#[doc = "  Weight is calculated for the maximum number of vote."]
					remove_other_vote {
						target: ::subxt::ext::sp_core::crypto::AccountId32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 22)]
					#[doc = "Enact a proposal from a referendum. For now we just make the weight be the maximum."]
					enact_proposal {
						proposal_hash: ::subxt::ext::sp_core::H256,
						index: ::core::primitive::u32,
					},
					#[codec(index = 23)]
					#[doc = "Permanently place a proposal into the blacklist. This prevents it from ever being"]
					#[doc = "proposed again."]
					#[doc = ""]
					#[doc = "If called on a queued public or external proposal, then this will result in it being"]
					#[doc = "removed. If the `ref_index` supplied is an active referendum with the proposal hash,"]
					#[doc = "then it will be cancelled."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be `BlacklistOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The proposal hash to blacklist permanently."]
					#[doc = "- `ref_index`: An ongoing referendum whose hash is `proposal_hash`, which will be"]
					#[doc = "cancelled."]
					#[doc = ""]
					#[doc = "Weight: `O(p)` (though as this is an high-privilege dispatch, we assume it has a"]
					#[doc = "  reasonable value)."]
					blacklist {
						proposal_hash: ::subxt::ext::sp_core::H256,
						maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec(index = 24)]
					#[doc = "Remove a proposal."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be `CancelProposalOrigin`."]
					#[doc = ""]
					#[doc = "- `prop_index`: The index of the proposal to cancel."]
					#[doc = ""]
					#[doc = "Weight: `O(p)` where `p = PublicProps::<T>::decode_len()`"]
					cancel_proposal {
						#[codec(compact)]
						prop_index: ::core::primitive::u32,
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
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Value too low"]
					ValueLow,
					#[codec(index = 1)]
					#[doc = "Proposal does not exist"]
					ProposalMissing,
					#[codec(index = 2)]
					#[doc = "Cannot cancel the same proposal twice"]
					AlreadyCanceled,
					#[codec(index = 3)]
					#[doc = "Proposal already made"]
					DuplicateProposal,
					#[codec(index = 4)]
					#[doc = "Proposal still blacklisted"]
					ProposalBlacklisted,
					#[codec(index = 5)]
					#[doc = "Next external proposal not simple majority"]
					NotSimpleMajority,
					#[codec(index = 6)]
					#[doc = "Invalid hash"]
					InvalidHash,
					#[codec(index = 7)]
					#[doc = "No external proposal"]
					NoProposal,
					#[codec(index = 8)]
					#[doc = "Identity may not veto a proposal twice"]
					AlreadyVetoed,
					#[codec(index = 9)]
					#[doc = "Preimage already noted"]
					DuplicatePreimage,
					#[codec(index = 10)]
					#[doc = "Not imminent"]
					NotImminent,
					#[codec(index = 11)]
					#[doc = "Too early"]
					TooEarly,
					#[codec(index = 12)]
					#[doc = "Imminent"]
					Imminent,
					#[codec(index = 13)]
					#[doc = "Preimage not found"]
					PreimageMissing,
					#[codec(index = 14)]
					#[doc = "Vote given for invalid referendum"]
					ReferendumInvalid,
					#[codec(index = 15)]
					#[doc = "Invalid preimage"]
					PreimageInvalid,
					#[codec(index = 16)]
					#[doc = "No proposals waiting"]
					NoneWaiting,
					#[codec(index = 17)]
					#[doc = "The given account did not vote on the referendum."]
					NotVoter,
					#[codec(index = 18)]
					#[doc = "The actor has no permission to conduct the action."]
					NoPermission,
					#[codec(index = 19)]
					#[doc = "The account is already delegating."]
					AlreadyDelegating,
					#[codec(index = 20)]
					#[doc = "Too high a balance was provided that the account cannot afford."]
					InsufficientFunds,
					#[codec(index = 21)]
					#[doc = "The account is not currently delegating."]
					NotDelegating,
					#[codec(index = 22)]
					#[doc = "The account currently has votes attached to it and the operation cannot succeed until"]
					#[doc = "these are removed, either through `unvote` or `reap_vote`."]
					VotesExist,
					#[codec(index = 23)]
					#[doc = "The instant referendum origin is currently disallowed."]
					InstantNotAllowed,
					#[codec(index = 24)]
					#[doc = "Delegation to oneself makes no sense."]
					Nonsense,
					#[codec(index = 25)]
					#[doc = "Invalid upper bound."]
					WrongUpperBound,
					#[codec(index = 26)]
					#[doc = "Maximum number of votes reached."]
					MaxVotesReached,
					#[codec(index = 27)]
					#[doc = "Maximum number of proposals reached."]
					TooManyProposals,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A motion has been proposed by a public account."]
					Proposed {
						proposal_index: ::core::primitive::u32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "A public proposal has been tabled for referendum vote."]
					Tabled {
						proposal_index: ::core::primitive::u32,
						deposit: ::core::primitive::u128,
						depositors: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					},
					#[codec(index = 2)]
					#[doc = "An external proposal has been tabled."]
					ExternalTabled,
					#[codec(index = 3)]
					#[doc = "A referendum has begun."]
					Started {
						ref_index: ::core::primitive::u32,
						threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					},
					#[codec(index = 4)]
					#[doc = "A proposal has been approved by referendum."]
					Passed { ref_index: ::core::primitive::u32 },
					#[codec(index = 5)]
					#[doc = "A proposal has been rejected by referendum."]
					NotPassed { ref_index: ::core::primitive::u32 },
					#[codec(index = 6)]
					#[doc = "A referendum has been cancelled."]
					Cancelled { ref_index: ::core::primitive::u32 },
					#[codec(index = 7)]
					#[doc = "A proposal has been enacted."]
					Executed {
						ref_index: ::core::primitive::u32,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 8)]
					#[doc = "An account has delegated their vote to another account."]
					Delegated {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						target: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 9)]
					#[doc = "An account has cancelled a previous delegation operation."]
					Undelegated {
						account: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 10)]
					#[doc = "An external proposal has been vetoed."]
					Vetoed {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						proposal_hash: ::subxt::ext::sp_core::H256,
						until: ::core::primitive::u32,
					},
					#[codec(index = 11)]
					#[doc = "A proposal's preimage was noted, and the deposit taken."]
					PreimageNoted {
						proposal_hash: ::subxt::ext::sp_core::H256,
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 12)]
					#[doc = "A proposal preimage was removed and used (the deposit was returned)."]
					PreimageUsed {
						proposal_hash: ::subxt::ext::sp_core::H256,
						provider: ::subxt::ext::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 13)]
					#[doc = "A proposal could not be executed because its preimage was invalid."]
					PreimageInvalid {
						proposal_hash: ::subxt::ext::sp_core::H256,
						ref_index: ::core::primitive::u32,
					},
					#[codec(index = 14)]
					#[doc = "A proposal could not be executed because its preimage was missing."]
					PreimageMissing {
						proposal_hash: ::subxt::ext::sp_core::H256,
						ref_index: ::core::primitive::u32,
					},
					#[codec(index = 15)]
					#[doc = "A registered preimage was removed and the deposit collected by the reaper."]
					PreimageReaped {
						proposal_hash: ::subxt::ext::sp_core::H256,
						provider: ::subxt::ext::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
						reaper: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 16)]
					#[doc = "A proposal_hash has been blacklisted permanently."]
					Blacklisted {
						proposal_hash: ::subxt::ext::sp_core::H256,
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
				pub struct Delegations<_0> {
					pub votes: _0,
					pub capital: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum ReferendumInfo<_0, _1, _2> {
					#[codec(index = 0)]
					Ongoing(runtime_types::pallet_democracy::types::ReferendumStatus<_0, _1, _2>),
					#[codec(index = 1)]
					Finished {
						approved: ::core::primitive::bool,
						end: _0,
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
				pub struct ReferendumStatus<_0, _1, _2> {
					pub end: _0,
					pub proposal_hash: _1,
					pub threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					pub delay: _0,
					pub tally: runtime_types::pallet_democracy::types::Tally<_2>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Tally<_0> {
					pub ayes: _0,
					pub nays: _0,
					pub turnout: _0,
				}
			}
			pub mod vote {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum AccountVote<_0> {
					#[codec(index = 0)]
					Standard {
						vote: runtime_types::pallet_democracy::vote::Vote,
						balance: _0,
					},
					#[codec(index = 1)]
					Split { aye: _0, nay: _0 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct PriorLock<_0, _1>(pub _0, pub _1);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct Vote(pub ::core::primitive::u8);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum Voting<_0, _1, _2> {
					#[codec(index = 0)]
					Direct {
						votes: ::std::vec::Vec<(
							_2,
							runtime_types::pallet_democracy::vote::AccountVote<_0>,
						)>,
						delegations: runtime_types::pallet_democracy::types::Delegations<_0>,
						prior: runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
					},
					#[codec(index = 1)]
					Delegating {
						balance: _0,
						target: _1,
						conviction: runtime_types::pallet_democracy::conviction::Conviction,
						delegations: runtime_types::pallet_democracy::types::Delegations<_0>,
						prior: runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
					},
				}
			}
			pub mod vote_threshold {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub enum VoteThreshold {
					#[codec(index = 0)]
					SuperMajorityApprove,
					#[codec(index = 1)]
					SuperMajorityAgainst,
					#[codec(index = 2)]
					SimpleMajority,
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
			pub enum PreimageStatus<_0, _1, _2> {
				#[codec(index = 0)]
				Missing(_2),
				#[codec(index = 1)]
				Available {
					data: ::std::vec::Vec<::core::primitive::u8>,
					provider: _0,
					deposit: _1,
					since: _2,
					expiry: ::core::option::Option<_2>,
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
			pub enum Releases {
				#[codec(index = 0)]
				V1,
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					# [codec (index = 0)] # [doc = "Submit a solution for the unsigned phase."] # [doc = ""] # [doc = "The dispatch origin fo this call must be __none__."] # [doc = ""] # [doc = "This submission is checked on the fly. Moreover, this unsigned solution is only"] # [doc = "validated when submitted to the pool from the **local** node. Effectively, this means"] # [doc = "that only active validators can submit this transaction when authoring a block (similar"] # [doc = "to an inherent)."] # [doc = ""] # [doc = "To prevent any incorrect solution (and thus wasted time/weight), this transaction will"] # [doc = "panic if the solution submitted by the validator is invalid in any way, effectively"] # [doc = "putting their authoring reward at risk."] # [doc = ""] # [doc = "No deposit or reward is associated with this submission."] submit_unsigned { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: da_runtime :: NposSolution16 > > , witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , } , # [codec (index = 1)] # [doc = "Set a new value for `MinimumUntrustedScore`."] # [doc = ""] # [doc = "Dispatch origin must be aligned with `T::ForceOrigin`."] # [doc = ""] # [doc = "This check can be turned off by setting the value to `None`."] set_minimum_untrusted_score { maybe_next_score : :: core :: option :: Option < [:: core :: primitive :: u128 ; 3usize] > , } , # [codec (index = 2)] # [doc = "Set a solution in the queue, to be handed out to the client of this pallet in the next"] # [doc = "call to `ElectionProvider::elect`."] # [doc = ""] # [doc = "This can only be set by `T::ForceOrigin`, and only when the phase is `Emergency`."] # [doc = ""] # [doc = "The solution is not checked for any feasibility and is assumed to be trustworthy, as any"] # [doc = "feasibility check itself can in principle cause the election process to fail (due to"] # [doc = "memory/weight constrains)."] set_emergency_election_result { supports : :: std :: vec :: Vec < (:: subxt :: ext :: sp_core :: crypto :: AccountId32 , runtime_types :: sp_npos_elections :: Support < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) > , } , # [codec (index = 3)] # [doc = "Submit a solution for the signed phase."] # [doc = ""] # [doc = "The dispatch origin fo this call must be __signed__."] # [doc = ""] # [doc = "The solution is potentially queued, based on the claimed score and processed at the end"] # [doc = "of the signed phase."] # [doc = ""] # [doc = "A deposit is reserved and recorded for the solution. Based on the outcome, the solution"] # [doc = "might be rewarded, slashed, or get all or a part of the deposit back."] # [doc = ""] # [doc = "# <weight>"] # [doc = "Queue size must be provided as witness data."] # [doc = "# </weight>"] submit { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: da_runtime :: NposSolution16 > > , num_signed_submissions : :: core :: primitive :: u32 , } , }
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
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A solution was stored with the given compute."]
					#[doc = ""]
					#[doc = "If the solution is signed, this means that it hasn't yet been processed. If the"]
					#[doc = "solution is unsigned, this means that it has also been processed."]
					#[doc = ""]
					#[doc = "The `bool` is `true` when a previous solution was ejected to make room for this one."]
					SolutionStored {
						election_compute:
							runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
						prev_ejected: ::core::primitive::bool,
					},
					#[codec(index = 1)]
					#[doc = "The election has been finalized, with `Some` of the given computation, or else if the"]
					#[doc = "election failed, `None`."]
					ElectionFinalized {
						election_compute: ::core::option::Option<
							runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
						>,
					},
					#[codec(index = 2)]
					#[doc = "An account has been rewarded for their signed submission being finalized."]
					Rewarded {
						account: ::subxt::ext::sp_core::crypto::AccountId32,
						value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "An account has been slashed for submitting an invalid signed submission."]
					Slashed {
						account: ::subxt::ext::sp_core::crypto::AccountId32,
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "The signed phase of the given round has started."]
					SignedPhaseStarted { round: ::core::primitive::u32 },
					#[codec(index = 5)]
					#[doc = "The unsigned phase of the given round has started."]
					UnsignedPhaseStarted { round: ::core::primitive::u32 },
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
					pub reward: _1,
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
				pub score: [::core::primitive::u128; 3usize],
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
			pub struct ReadySolution<_0> {
				pub supports: ::std::vec::Vec<(_0, runtime_types::sp_npos_elections::Support<_0>)>,
				pub score: [::core::primitive::u128; 3usize],
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
			pub struct RoundSnapshot<_0> {
				pub voters: ::std::vec::Vec<(_0, ::core::primitive::u64, ::std::vec::Vec<_0>)>,
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
		pub mod pallet_elections_phragmen {
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Vote for a set of candidates for the upcoming round of election. This can be called to"]
					#[doc = "set the initial votes, or update already existing votes."]
					#[doc = ""]
					#[doc = "Upon initial voting, `value` units of `who`'s balance is locked and a deposit amount is"]
					#[doc = "reserved. The deposit is based on the number of votes and can be updated over time."]
					#[doc = ""]
					#[doc = "The `votes` should:"]
					#[doc = "  - not be empty."]
					#[doc = "  - be less than the number of possible candidates. Note that all current members and"]
					#[doc = "    runners-up are also automatically candidates for the next round."]
					#[doc = ""]
					#[doc = "If `value` is more than `who`'s total balance, then the maximum of the two is used."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be signed."]
					#[doc = ""]
					#[doc = "### Warning"]
					#[doc = ""]
					#[doc = "It is the responsibility of the caller to **NOT** place all of their balance into the"]
					#[doc = "lock and keep some for further operations."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "We assume the maximum weight among all 3 cases: vote_equal, vote_more and vote_less."]
					#[doc = "# </weight>"]
					vote {
						votes: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "Remove `origin` as a voter."]
					#[doc = ""]
					#[doc = "This removes the lock and returns the deposit."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be signed and be a voter."]
					remove_voter,
					#[codec(index = 2)]
					#[doc = "Submit oneself for candidacy. A fixed amount of deposit is recorded."]
					#[doc = ""]
					#[doc = "All candidates are wiped at the end of the term. They either become a member/runner-up,"]
					#[doc = "or leave the system while their deposit is slashed."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be signed."]
					#[doc = ""]
					#[doc = "### Warning"]
					#[doc = ""]
					#[doc = "Even if a candidate ends up being a member, they must call [`Call::renounce_candidacy`]"]
					#[doc = "to get their deposit back. Losing the spot in an election will always lead to a slash."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "The number of current candidates must be provided as witness data."]
					#[doc = "# </weight>"]
					submit_candidacy {
						#[codec(compact)]
						candidate_count: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "Renounce one's intention to be a candidate for the next election round. 3 potential"]
					#[doc = "outcomes exist:"]
					#[doc = ""]
					#[doc = "- `origin` is a candidate and not elected in any set. In this case, the deposit is"]
					#[doc = "  unreserved, returned and origin is removed as a candidate."]
					#[doc = "- `origin` is a current runner-up. In this case, the deposit is unreserved, returned and"]
					#[doc = "  origin is removed as a runner-up."]
					#[doc = "- `origin` is a current member. In this case, the deposit is unreserved and origin is"]
					#[doc = "  removed as a member, consequently not being a candidate for the next round anymore."]
					#[doc = "  Similar to [`remove_member`](Self::remove_member), if replacement runners exists, they"]
					#[doc = "  are immediately used. If the prime is renouncing, then no prime will exist until the"]
					#[doc = "  next round."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be signed, and have one of the above roles."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "The type of renouncing must be provided as witness data."]
					#[doc = "# </weight>"]
					renounce_candidacy {
						renouncing: runtime_types::pallet_elections_phragmen::Renouncing,
					},
					#[codec(index = 4)]
					#[doc = "Remove a particular member from the set. This is effective immediately and the bond of"]
					#[doc = "the outgoing member is slashed."]
					#[doc = ""]
					#[doc = "If a runner-up is available, then the best runner-up will be removed and replaces the"]
					#[doc = "outgoing member. Otherwise, a new phragmen election is started."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be root."]
					#[doc = ""]
					#[doc = "Note that this does not affect the designated block number of the next election."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "If we have a replacement, we use a small weight. Else, since this is a root call and"]
					#[doc = "will go into phragmen, we assume full block for now."]
					#[doc = "# </weight>"]
					remove_member {
						who: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						has_replacement: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					#[doc = "Clean all voters who are defunct (i.e. they do not serve any purpose at all). The"]
					#[doc = "deposit of the removed voters are returned."]
					#[doc = ""]
					#[doc = "This is an root function to be used only for cleaning the state."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be root."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "The total number of voters and those that are defunct must be provided as witness data."]
					#[doc = "# </weight>"]
					clean_defunct_voters {
						num_voters: ::core::primitive::u32,
						num_defunct: ::core::primitive::u32,
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
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Cannot vote when no candidates or members exist."]
					UnableToVote,
					#[codec(index = 1)]
					#[doc = "Must vote for at least one candidate."]
					NoVotes,
					#[codec(index = 2)]
					#[doc = "Cannot vote more than candidates."]
					TooManyVotes,
					#[codec(index = 3)]
					#[doc = "Cannot vote more than maximum allowed."]
					MaximumVotesExceeded,
					#[codec(index = 4)]
					#[doc = "Cannot vote with stake less than minimum balance."]
					LowBalance,
					#[codec(index = 5)]
					#[doc = "Voter can not pay voting bond."]
					UnableToPayBond,
					#[codec(index = 6)]
					#[doc = "Must be a voter."]
					MustBeVoter,
					#[codec(index = 7)]
					#[doc = "Cannot report self."]
					ReportSelf,
					#[codec(index = 8)]
					#[doc = "Duplicated candidate submission."]
					DuplicatedCandidate,
					#[codec(index = 9)]
					#[doc = "Member cannot re-submit candidacy."]
					MemberSubmit,
					#[codec(index = 10)]
					#[doc = "Runner cannot re-submit candidacy."]
					RunnerUpSubmit,
					#[codec(index = 11)]
					#[doc = "Candidate does not have enough funds."]
					InsufficientCandidateFunds,
					#[codec(index = 12)]
					#[doc = "Not a member."]
					NotMember,
					#[codec(index = 13)]
					#[doc = "The provided count of number of candidates is incorrect."]
					InvalidWitnessData,
					#[codec(index = 14)]
					#[doc = "The provided count of number of votes is incorrect."]
					InvalidVoteCount,
					#[codec(index = 15)]
					#[doc = "The renouncing origin presented a wrong `Renouncing` parameter."]
					InvalidRenouncing,
					#[codec(index = 16)]
					#[doc = "Prediction regarding replacement after member removal is wrong."]
					InvalidReplacement,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A new term with new_members. This indicates that enough candidates existed to run"]
					#[doc = "the election, not that enough have has been elected. The inner value must be examined"]
					#[doc = "for this purpose. A `NewTerm(\\[\\])` indicates that some candidates got their bond"]
					#[doc = "slashed and none were elected, whilst `EmptyTerm` means that no candidates existed to"]
					#[doc = "begin with."]
					NewTerm {
						new_members: ::std::vec::Vec<(
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						)>,
					},
					#[codec(index = 1)]
					#[doc = "No (or not enough) candidates existed for this round. This is different from"]
					#[doc = "`NewTerm(\\[\\])`. See the description of `NewTerm`."]
					EmptyTerm,
					#[codec(index = 2)]
					#[doc = "Internal error happened while trying to perform election."]
					ElectionError,
					#[codec(index = 3)]
					#[doc = "A member has been removed. This should always be followed by either `NewTerm` or"]
					#[doc = "`EmptyTerm`."]
					MemberKicked {
						member: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 4)]
					#[doc = "Someone has renounced their candidacy."]
					Renounced {
						candidate: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 5)]
					#[doc = "A candidate was slashed by amount due to failing to obtain a seat as member or"]
					#[doc = "runner-up."]
					#[doc = ""]
					#[doc = "Note that old members and runners-up are also candidates."]
					CandidateSlashed {
						candidate: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "A seat holder was slashed by amount by being forcefully removed from the set."]
					SeatHolderSlashed {
						seat_holder: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
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
			pub enum Renouncing {
				#[codec(index = 0)]
				Member,
				#[codec(index = 1)]
				RunnerUp,
				#[codec(index = 2)]
				Candidate(#[codec(compact)] ::core::primitive::u32),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct SeatHolder<_0, _1> {
				pub who: _0,
				pub stake: _1,
				pub deposit: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct Voter<_0, _1> {
				pub votes: ::std::vec::Vec<_0>,
				pub stake: _1,
				pub deposit: _1,
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Report voter equivocation/misbehavior. This method will verify the"]
					#[doc = "equivocation proof and validate the given key ownership proof"]
					#[doc = "against the extracted offender. If both are valid, the offence"]
					#[doc = "will be reported."]
					report_equivocation {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_finality_grandpa::EquivocationProof<
								::subxt::ext::sp_core::H256,
								::core::primitive::u32,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 1)]
					#[doc = "Report voter equivocation/misbehavior. This method will verify the"]
					#[doc = "equivocation proof and validate the given key ownership proof"]
					#[doc = "against the extracted offender. If both are valid, the offence"]
					#[doc = "will be reported."]
					#[doc = ""]
					#[doc = "This extrinsic must be called unsigned and it is expected that only"]
					#[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
					#[doc = "if the block author is defined it will be defined as the equivocation"]
					#[doc = "reporter."]
					report_equivocation_unsigned {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_finality_grandpa::EquivocationProof<
								::subxt::ext::sp_core::H256,
								::core::primitive::u32,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 2)]
					#[doc = "Note that the current authority set of the GRANDPA finality gadget has"]
					#[doc = "stalled. This will trigger a forced authority set change at the beginning"]
					#[doc = "of the next session, to be enacted `delay` blocks after that. The delay"]
					#[doc = "should be high enough to safely assume that the block signalling the"]
					#[doc = "forced change will not be re-orged (e.g. 1000 blocks). The GRANDPA voters"]
					#[doc = "will start the new authority set using the given finalized block as base."]
					#[doc = "Only callable by root."]
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
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New authority set has been applied."]
					NewAuthorities {
						authority_set: ::std::vec::Vec<(
							runtime_types::sp_finality_grandpa::app::Public,
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
					runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
						runtime_types::sp_finality_grandpa::app::Public,
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: `O(K + E)` where K is length of `Keys` (heartbeat.validators_len) and E is"]
					#[doc = "  length of `heartbeat.network_state.external_address`"]
					#[doc = "  - `O(K)`: decoding of length `K`"]
					#[doc = "  - `O(E)`: decoding/encoding of length `E`"]
					#[doc = "- DbReads: pallet_session `Validators`, pallet_session `CurrentIndex`, `Keys`,"]
					#[doc = "  `ReceivedHeartbeats`"]
					#[doc = "- DbWrites: `ReceivedHeartbeats`"]
					#[doc = "# </weight>"]
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
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
			pub struct BoundedOpaqueNetworkState {
				pub peer_id:
					runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
						::core::primitive::u8,
					>,
				pub external_addresses:
					runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
						runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
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
			pub struct Heartbeat<_0> {
				pub block_number: _0,
				pub network_state: runtime_types::sp_core::offchain::OpaqueNetworkState,
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Assign an previously unassigned index."]
					#[doc = ""]
					#[doc = "Payment: `Deposit` is reserved from the sender account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `index`: the index to be claimed. This must not be in use."]
					#[doc = ""]
					#[doc = "Emits `IndexAssigned` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`."]
					#[doc = "- One storage mutation (codec `O(1)`)."]
					#[doc = "- One reserve operation."]
					#[doc = "- One event."]
					#[doc = "-------------------"]
					#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
					#[doc = "# </weight>"]
					claim { index: ::core::primitive::u32 },
					#[codec(index = 1)]
					#[doc = "Assign an index already owned by the sender to another account. The balance reservation"]
					#[doc = "is effectively transferred to the new account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `index`: the index to be re-assigned. This must be owned by the sender."]
					#[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
					#[doc = ""]
					#[doc = "Emits `IndexAssigned` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`."]
					#[doc = "- One storage mutation (codec `O(1)`)."]
					#[doc = "- One transfer operation."]
					#[doc = "- One event."]
					#[doc = "-------------------"]
					#[doc = "- DB Weight:"]
					#[doc = "   - Reads: Indices Accounts, System Account (recipient)"]
					#[doc = "   - Writes: Indices Accounts, System Account (recipient)"]
					#[doc = "# </weight>"]
					transfer {
						new: ::subxt::ext::sp_core::crypto::AccountId32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Free up an index owned by the sender."]
					#[doc = ""]
					#[doc = "Payment: Any previous deposit placed for the index is unreserved in the sender account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_ and the sender must own the index."]
					#[doc = ""]
					#[doc = "- `index`: the index to be freed. This must be owned by the sender."]
					#[doc = ""]
					#[doc = "Emits `IndexFreed` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`."]
					#[doc = "- One storage mutation (codec `O(1)`)."]
					#[doc = "- One reserve operation."]
					#[doc = "- One event."]
					#[doc = "-------------------"]
					#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
					#[doc = "# </weight>"]
					free { index: ::core::primitive::u32 },
					#[codec(index = 3)]
					#[doc = "Force an index to an account. This doesn't require a deposit. If the index is already"]
					#[doc = "held, then any deposit is reimbursed to its current owner."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Root_."]
					#[doc = ""]
					#[doc = "- `index`: the index to be (re-)assigned."]
					#[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
					#[doc = "- `freeze`: if set to `true`, will freeze the index so it cannot be transferred."]
					#[doc = ""]
					#[doc = "Emits `IndexAssigned` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`."]
					#[doc = "- One storage mutation (codec `O(1)`)."]
					#[doc = "- Up to one reserve operation."]
					#[doc = "- One event."]
					#[doc = "-------------------"]
					#[doc = "- DB Weight:"]
					#[doc = "   - Reads: Indices Accounts, System Account (original owner)"]
					#[doc = "   - Writes: Indices Accounts, System Account (original owner)"]
					#[doc = "# </weight>"]
					force_transfer {
						new: ::subxt::ext::sp_core::crypto::AccountId32,
						index: ::core::primitive::u32,
						freeze: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					#[doc = "Freeze an index so it will always point to the sender account. This consumes the"]
					#[doc = "deposit."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_ and the signing account must have a"]
					#[doc = "non-frozen account `index`."]
					#[doc = ""]
					#[doc = "- `index`: the index to be frozen in place."]
					#[doc = ""]
					#[doc = "Emits `IndexFrozen` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`."]
					#[doc = "- One storage mutation (codec `O(1)`)."]
					#[doc = "- Up to one slash operation."]
					#[doc = "- One event."]
					#[doc = "-------------------"]
					#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
					#[doc = "# </weight>"]
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
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Add a member `who` to the set."]
					#[doc = ""]
					#[doc = "May only be called from `T::AddOrigin`."]
					add_member {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 1)]
					#[doc = "Remove a member `who` from the set."]
					#[doc = ""]
					#[doc = "May only be called from `T::RemoveOrigin`."]
					remove_member {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 2)]
					#[doc = "Swap out one member `remove` for another `add`."]
					#[doc = ""]
					#[doc = "May only be called from `T::SwapOrigin`."]
					#[doc = ""]
					#[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
					swap_member {
						remove: ::subxt::ext::sp_core::crypto::AccountId32,
						add: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 3)]
					#[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
					#[doc = "pass `members` pre-sorted."]
					#[doc = ""]
					#[doc = "May only be called from `T::ResetOrigin`."]
					reset_members {
						members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					},
					#[codec(index = 4)]
					#[doc = "Swap out the sending member for some other key `new`."]
					#[doc = ""]
					#[doc = "May only be called from `Signed` origin of a current member."]
					#[doc = ""]
					#[doc = "Prime membership is passed from the origin account to `new`, if extant."]
					change_key {
						new: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 5)]
					#[doc = "Set the prime member. Must be a current member."]
					#[doc = ""]
					#[doc = "May only be called from `T::PrimeOrigin`."]
					set_prime {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 6)]
					#[doc = "Remove the prime member if it exists."]
					#[doc = ""]
					#[doc = "May only be called from `T::PrimeOrigin`."]
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
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Already a member."]
					AlreadyMember,
					#[codec(index = 1)]
					#[doc = "Not a member."]
					NotMember,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Anonymously schedule a task."]
					schedule {
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
					},
					#[codec(index = 1)]
					#[doc = "Cancel an anonymously scheduled task."]
					cancel {
						when: ::core::primitive::u32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Schedule a named task."]
					schedule_named {
						id: ::std::vec::Vec<::core::primitive::u8>,
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
					},
					#[codec(index = 3)]
					#[doc = "Cancel a named scheduled task."]
					cancel_named {
						id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 4)]
					#[doc = "Anonymously schedule a task after a delay."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Same as [`schedule`]."]
					#[doc = "# </weight>"]
					schedule_after {
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
					},
					#[codec(index = 5)]
					#[doc = "Schedule a named task after a delay."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Same as [`schedule_named`](Self::schedule_named)."]
					#[doc = "# </weight>"]
					schedule_named_after {
						id: ::std::vec::Vec<::core::primitive::u8>,
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
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
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
					#[doc = "Scheduled some task. \\[when, index\\]"]
					Scheduled(::core::primitive::u32, ::core::primitive::u32),
					#[codec(index = 1)]
					#[doc = "Canceled some task. \\[when, index\\]"]
					Canceled(::core::primitive::u32, ::core::primitive::u32),
					#[codec(index = 2)]
					#[doc = "Dispatched some task. \\[task, id, result\\]"]
					Dispatched(
						(::core::primitive::u32, ::core::primitive::u32),
						::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					),
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
			pub enum Releases {
				#[codec(index = 0)]
				V1,
				#[codec(index = 1)]
				V2,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct ScheduledV2<_0, _1, _2, _3> {
				pub maybe_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
				pub priority: ::core::primitive::u8,
				pub call: _0,
				pub maybe_periodic: ::core::option::Option<(_1, _1)>,
				pub origin: _2,
				#[codec(skip)]
				pub __subxt_unused_type_params: ::core::marker::PhantomData<_3>,
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Sets the session key(s) of the function caller to `keys`."]
					#[doc = "Allows an account to set its session key prior to becoming a validator."]
					#[doc = "This doesn't take effect until the next session."]
					#[doc = ""]
					#[doc = "The dispatch origin of this function must be signed."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: `O(1)`. Actual cost depends on the number of length of"]
					#[doc = "  `T::Keys::key_ids()` which is fixed."]
					#[doc = "- DbReads: `origin account`, `T::ValidatorIdOf`, `NextKeys`"]
					#[doc = "- DbWrites: `origin account`, `NextKeys`"]
					#[doc = "- DbReads per key id: `KeyOwner`"]
					#[doc = "- DbWrites per key id: `KeyOwner`"]
					#[doc = "# </weight>"]
					set_keys {
						keys: runtime_types::da_runtime::SessionKeys,
						proof: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "Removes any session key(s) of the function caller."]
					#[doc = ""]
					#[doc = "This doesn't take effect until the next session."]
					#[doc = ""]
					#[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
					#[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
					#[doc = "means being a controller account) or directly convertible into a validator ID (which"]
					#[doc = "usually means being a stash account)."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: `O(1)` in number of key types. Actual cost depends on the number of length"]
					#[doc = "  of `T::Keys::key_ids()` which is fixed."]
					#[doc = "- DbReads: `T::ValidatorIdOf`, `NextKeys`, `origin account`"]
					#[doc = "- DbWrites: `NextKeys`, `origin account`"]
					#[doc = "- DbWrites per key id: `KeyOwner`"]
					#[doc = "# </weight>"]
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
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {
						#[codec(index = 0)]
						#[doc = "Take the origin account as a stash and lock up `value` of its balance. `controller` will"]
						#[doc = "be the account that controls it."]
						#[doc = ""]
						#[doc = "`value` must be more than the `minimum_balance` specified by `T::Currency`."]
						#[doc = ""]
						#[doc = "The dispatch origin for this call must be _Signed_ by the stash account."]
						#[doc = ""]
						#[doc = "Emits `Bonded`."]
						#[doc = "# <weight>"]
						#[doc = "- Independent of the arguments. Moderate complexity."]
						#[doc = "- O(1)."]
						#[doc = "- Three extra DB entries."]
						#[doc = ""]
						#[doc = "NOTE: Two of the storage writes (`Self::bonded`, `Self::payee`) are _never_ cleaned"]
						#[doc = "unless the `origin` falls below _existential deposit_ and gets removed as dust."]
						#[doc = "------------------"]
						#[doc = "# </weight>"]
						bond {
							controller: ::subxt::ext::sp_runtime::MultiAddress<
								::subxt::ext::sp_core::crypto::AccountId32,
								::core::primitive::u32,
							>,
							#[codec(compact)]
							value: ::core::primitive::u128,
							payee: runtime_types::pallet_staking::RewardDestination<
								::subxt::ext::sp_core::crypto::AccountId32,
							>,
						},
						#[codec(index = 1)]
						#[doc = "Add some extra amount that have appeared in the stash `free_balance` into the balance up"]
						#[doc = "for staking."]
						#[doc = ""]
						#[doc = "The dispatch origin for this call must be _Signed_ by the stash, not the controller."]
						#[doc = ""]
						#[doc = "Use this if there are additional funds in your stash account that you wish to bond."]
						#[doc = "Unlike [`bond`](Self::bond) or [`unbond`](Self::unbond) this function does not impose"]
						#[doc = "any limitation on the amount that can be added."]
						#[doc = ""]
						#[doc = "Emits `Bonded`."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "- Independent of the arguments. Insignificant complexity."]
						#[doc = "- O(1)."]
						#[doc = "# </weight>"]
						bond_extra {
							#[codec(compact)]
							max_additional: ::core::primitive::u128,
						},
						#[codec(index = 2)]
						#[doc = "Schedule a portion of the stash to be unlocked ready for transfer out after the bond"]
						#[doc = "period ends. If this leaves an amount actively bonded less than"]
						#[doc = "T::Currency::minimum_balance(), then it is increased to the full amount."]
						#[doc = ""]
						#[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
						#[doc = ""]
						#[doc = "Once the unlock period is done, you can call `withdraw_unbonded` to actually move"]
						#[doc = "the funds out of management ready for transfer."]
						#[doc = ""]
						#[doc = "No more than a limited number of unlocking chunks (see `MAX_UNLOCKING_CHUNKS`)"]
						#[doc = "can co-exists at the same time. In that case, [`Call::withdraw_unbonded`] need"]
						#[doc = "to be called first to remove some of the chunks (if possible)."]
						#[doc = ""]
						#[doc = "If a user encounters the `InsufficientBond` error when calling this extrinsic,"]
						#[doc = "they should call `chill` first in order to free up their bonded funds."]
						#[doc = ""]
						#[doc = "Emits `Unbonded`."]
						#[doc = ""]
						#[doc = "See also [`Call::withdraw_unbonded`]."]
						unbond {
							#[codec(compact)]
							value: ::core::primitive::u128,
						},
						#[codec(index = 3)]
						#[doc = "Remove any unlocked chunks from the `unlocking` queue from our management."]
						#[doc = ""]
						#[doc = "This essentially frees up that balance to be used by the stash account to do"]
						#[doc = "whatever it wants."]
						#[doc = ""]
						#[doc = "The dispatch origin for this call must be _Signed_ by the controller."]
						#[doc = ""]
						#[doc = "Emits `Withdrawn`."]
						#[doc = ""]
						#[doc = "See also [`Call::unbond`]."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "Complexity O(S) where S is the number of slashing spans to remove"]
						#[doc = "NOTE: Weight annotation is the kill scenario, we refund otherwise."]
						#[doc = "# </weight>"]
						withdraw_unbonded {
							num_slashing_spans: ::core::primitive::u32,
						},
						#[codec(index = 4)]
						#[doc = "Declare the desire to validate for the origin controller."]
						#[doc = ""]
						#[doc = "Effects will be felt at the beginning of the next era."]
						#[doc = ""]
						#[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
						validate {
							prefs: runtime_types::pallet_staking::ValidatorPrefs,
						},
						#[codec(index = 5)]
						#[doc = "Declare the desire to nominate `targets` for the origin controller."]
						#[doc = ""]
						#[doc = "Effects will be felt at the beginning of the next era."]
						#[doc = ""]
						#[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "- The transaction's complexity is proportional to the size of `targets` (N)"]
						#[doc = "which is capped at CompactAssignments::LIMIT (MAX_NOMINATIONS)."]
						#[doc = "- Both the reads and writes follow a similar pattern."]
						#[doc = "# </weight>"]
						nominate {
							targets: ::std::vec::Vec<
								::subxt::ext::sp_runtime::MultiAddress<
									::subxt::ext::sp_core::crypto::AccountId32,
									::core::primitive::u32,
								>,
							>,
						},
						#[codec(index = 6)]
						#[doc = "Declare no desire to either validate or nominate."]
						#[doc = ""]
						#[doc = "Effects will be felt at the beginning of the next era."]
						#[doc = ""]
						#[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "- Independent of the arguments. Insignificant complexity."]
						#[doc = "- Contains one read."]
						#[doc = "- Writes are limited to the `origin` account key."]
						#[doc = "# </weight>"]
						chill,
						#[codec(index = 7)]
						#[doc = "(Re-)set the payment target for a controller."]
						#[doc = ""]
						#[doc = "Effects will be felt at the beginning of the next era."]
						#[doc = ""]
						#[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "- Independent of the arguments. Insignificant complexity."]
						#[doc = "- Contains a limited number of reads."]
						#[doc = "- Writes are limited to the `origin` account key."]
						#[doc = "---------"]
						#[doc = "- Weight: O(1)"]
						#[doc = "- DB Weight:"]
						#[doc = "    - Read: Ledger"]
						#[doc = "    - Write: Payee"]
						#[doc = "# </weight>"]
						set_payee {
							payee: runtime_types::pallet_staking::RewardDestination<
								::subxt::ext::sp_core::crypto::AccountId32,
							>,
						},
						#[codec(index = 8)]
						#[doc = "(Re-)set the controller of a stash."]
						#[doc = ""]
						#[doc = "Effects will be felt at the beginning of the next era."]
						#[doc = ""]
						#[doc = "The dispatch origin for this call must be _Signed_ by the stash, not the controller."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "- Independent of the arguments. Insignificant complexity."]
						#[doc = "- Contains a limited number of reads."]
						#[doc = "- Writes are limited to the `origin` account key."]
						#[doc = "----------"]
						#[doc = "Weight: O(1)"]
						#[doc = "DB Weight:"]
						#[doc = "- Read: Bonded, Ledger New Controller, Ledger Old Controller"]
						#[doc = "- Write: Bonded, Ledger New Controller, Ledger Old Controller"]
						#[doc = "# </weight>"]
						set_controller {
							controller: ::subxt::ext::sp_runtime::MultiAddress<
								::subxt::ext::sp_core::crypto::AccountId32,
								::core::primitive::u32,
							>,
						},
						#[codec(index = 9)]
						#[doc = "Sets the ideal number of validators."]
						#[doc = ""]
						#[doc = "The dispatch origin must be Root."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "Weight: O(1)"]
						#[doc = "Write: Validator Count"]
						#[doc = "# </weight>"]
						set_validator_count {
							#[codec(compact)]
							new: ::core::primitive::u32,
						},
						#[codec(index = 10)]
						#[doc = "Increments the ideal number of validators."]
						#[doc = ""]
						#[doc = "The dispatch origin must be Root."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "Same as [`Self::set_validator_count`]."]
						#[doc = "# </weight>"]
						increase_validator_count {
							#[codec(compact)]
							additional: ::core::primitive::u32,
						},
						#[codec(index = 11)]
						#[doc = "Scale up the ideal number of validators by a factor."]
						#[doc = ""]
						#[doc = "The dispatch origin must be Root."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "Same as [`Self::set_validator_count`]."]
						#[doc = "# </weight>"]
						scale_validator_count {
							factor: runtime_types::sp_arithmetic::per_things::Percent,
						},
						#[codec(index = 12)]
						#[doc = "Force there to be no new eras indefinitely."]
						#[doc = ""]
						#[doc = "The dispatch origin must be Root."]
						#[doc = ""]
						#[doc = "# Warning"]
						#[doc = ""]
						#[doc = "The election process starts multiple blocks before the end of the era."]
						#[doc = "Thus the election process may be ongoing when this is called. In this case the"]
						#[doc = "election will continue until the next era is triggered."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "- No arguments."]
						#[doc = "- Weight: O(1)"]
						#[doc = "- Write: ForceEra"]
						#[doc = "# </weight>"]
						force_no_eras,
						#[codec(index = 13)]
						#[doc = "Force there to be a new era at the end of the next session. After this, it will be"]
						#[doc = "reset to normal (non-forced) behaviour."]
						#[doc = ""]
						#[doc = "The dispatch origin must be Root."]
						#[doc = ""]
						#[doc = "# Warning"]
						#[doc = ""]
						#[doc = "The election process starts multiple blocks before the end of the era."]
						#[doc = "If this is called just before a new era is triggered, the election process may not"]
						#[doc = "have enough blocks to get a result."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "- No arguments."]
						#[doc = "- Weight: O(1)"]
						#[doc = "- Write ForceEra"]
						#[doc = "# </weight>"]
						force_new_era,
						#[codec(index = 14)]
						#[doc = "Set the validators who cannot be slashed (if any)."]
						#[doc = ""]
						#[doc = "The dispatch origin must be Root."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "- O(V)"]
						#[doc = "- Write: Invulnerables"]
						#[doc = "# </weight>"]
						set_invulnerables {
							invulnerables:
								::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						},
						#[codec(index = 15)]
						#[doc = "Force a current staker to become completely unstaked, immediately."]
						#[doc = ""]
						#[doc = "The dispatch origin must be Root."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "O(S) where S is the number of slashing spans to be removed"]
						#[doc = "Reads: Bonded, Slashing Spans, Account, Locks"]
						#[doc = "Writes: Bonded, Slashing Spans (if S > 0), Ledger, Payee, Validators, Nominators,"]
						#[doc = "Account, Locks Writes Each: SpanSlash * S"]
						#[doc = "# </weight>"]
						force_unstake {
							stash: ::subxt::ext::sp_core::crypto::AccountId32,
							num_slashing_spans: ::core::primitive::u32,
						},
						#[codec(index = 16)]
						#[doc = "Force there to be a new era at the end of sessions indefinitely."]
						#[doc = ""]
						#[doc = "The dispatch origin must be Root."]
						#[doc = ""]
						#[doc = "# Warning"]
						#[doc = ""]
						#[doc = "The election process starts multiple blocks before the end of the era."]
						#[doc = "If this is called just before a new era is triggered, the election process may not"]
						#[doc = "have enough blocks to get a result."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "- Weight: O(1)"]
						#[doc = "- Write: ForceEra"]
						#[doc = "# </weight>"]
						force_new_era_always,
						#[codec(index = 17)]
						#[doc = "Cancel enactment of a deferred slash."]
						#[doc = ""]
						#[doc = "Can be called by the `T::SlashCancelOrigin`."]
						#[doc = ""]
						#[doc = "Parameters: era and indices of the slashes for that era to kill."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "Complexity: O(U + S)"]
						#[doc = "with U unapplied slashes weighted with U=1000"]
						#[doc = "and S is the number of slash indices to be canceled."]
						#[doc = "- Read: Unapplied Slashes"]
						#[doc = "- Write: Unapplied Slashes"]
						#[doc = "# </weight>"]
						cancel_deferred_slash {
							era: ::core::primitive::u32,
							slash_indices: ::std::vec::Vec<::core::primitive::u32>,
						},
						#[codec(index = 18)]
						#[doc = "Pay out all the stakers behind a single validator for a single era."]
						#[doc = ""]
						#[doc = "- `validator_stash` is the stash account of the validator. Their nominators, up to"]
						#[doc = "  `T::MaxNominatorRewardedPerValidator`, will also receive their rewards."]
						#[doc = "- `era` may be any era between `[current_era - history_depth; current_era]`."]
						#[doc = ""]
						#[doc = "The origin of this call must be _Signed_. Any account can call this function, even if"]
						#[doc = "it is not one of the stakers."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "- Time complexity: at most O(MaxNominatorRewardedPerValidator)."]
						#[doc = "- Contains a limited number of reads and writes."]
						#[doc = "-----------"]
						#[doc = "N is the Number of payouts for the validator (including the validator)"]
						#[doc = "Weight:"]
						#[doc = "- Reward Destination Staked: O(N)"]
						#[doc = "- Reward Destination Controller (Creating): O(N)"]
						#[doc = ""]
						#[doc = "  NOTE: weights are assuming that payouts are made to alive stash account (Staked)."]
						#[doc = "  Paying even a dead controller is cheaper weight-wise. We don't do any refunds here."]
						#[doc = "# </weight>"]
						payout_stakers {
							validator_stash: ::subxt::ext::sp_core::crypto::AccountId32,
							era: ::core::primitive::u32,
						},
						#[codec(index = 19)]
						#[doc = "Rebond a portion of the stash scheduled to be unlocked."]
						#[doc = ""]
						#[doc = "The dispatch origin must be signed by the controller."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "- Time complexity: O(L), where L is unlocking chunks"]
						#[doc = "- Bounded by `MAX_UNLOCKING_CHUNKS`."]
						#[doc = "- Storage changes: Can't increase storage, only decrease it."]
						#[doc = "# </weight>"]
						rebond {
							#[codec(compact)]
							value: ::core::primitive::u128,
						},
						#[codec(index = 20)]
						#[doc = "Set `HistoryDepth` value. This function will delete any history information"]
						#[doc = "when `HistoryDepth` is reduced."]
						#[doc = ""]
						#[doc = "Parameters:"]
						#[doc = "- `new_history_depth`: The new history depth you would like to set."]
						#[doc = "- `era_items_deleted`: The number of items that will be deleted by this dispatch. This"]
						#[doc = "  should report all the storage items that will be deleted by clearing old era history."]
						#[doc = "  Needed to report an accurate weight for the dispatch. Trusted by `Root` to report an"]
						#[doc = "  accurate number."]
						#[doc = ""]
						#[doc = "Origin must be root."]
						#[doc = ""]
						#[doc = "# <weight>"]
						#[doc = "- E: Number of history depths removed, i.e. 10 -> 7 = 3"]
						#[doc = "- Weight: O(E)"]
						#[doc = "- DB Weight:"]
						#[doc = "    - Reads: Current Era, History Depth"]
						#[doc = "    - Writes: History Depth"]
						#[doc = "    - Clear Prefix Each: Era Stakers, EraStakersClipped, ErasValidatorPrefs"]
						#[doc = "    - Writes Each: ErasValidatorReward, ErasRewardPoints, ErasTotalStake,"]
						#[doc = "      ErasStartSessionIndex"]
						#[doc = "# </weight>"]
						set_history_depth {
							#[codec(compact)]
							new_history_depth: ::core::primitive::u32,
							#[codec(compact)]
							era_items_deleted: ::core::primitive::u32,
						},
						#[codec(index = 21)]
						#[doc = "Remove all data structures concerning a staker/stash once it is at a state where it can"]
						#[doc = "be considered `dust` in the staking system. The requirements are:"]
						#[doc = ""]
						#[doc = "1. the `total_balance` of the stash is below existential deposit."]
						#[doc = "2. or, the `ledger.total` of the stash is below existential deposit."]
						#[doc = ""]
						#[doc = "The former can happen in cases like a slash; the latter when a fully unbonded account"]
						#[doc = "is still receiving staking rewards in `RewardDestination::Staked`."]
						#[doc = ""]
						#[doc = "It can be called by anyone, as long as `stash` meets the above requirements."]
						#[doc = ""]
						#[doc = "Refunds the transaction fees upon successful execution."]
						reap_stash {
							stash: ::subxt::ext::sp_core::crypto::AccountId32,
							num_slashing_spans: ::core::primitive::u32,
						},
						#[codec(index = 22)]
						#[doc = "Remove the given nominations from the calling validator."]
						#[doc = ""]
						#[doc = "Effects will be felt at the beginning of the next era."]
						#[doc = ""]
						#[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
						#[doc = ""]
						#[doc = "- `who`: A list of nominator stash accounts who are nominating this validator which"]
						#[doc = "  should no longer be nominating this validator."]
						#[doc = ""]
						#[doc = "Note: Making this call only makes sense if you first set the validator preferences to"]
						#[doc = "block any further nominations."]
						kick {
							who: ::std::vec::Vec<
								::subxt::ext::sp_runtime::MultiAddress<
									::subxt::ext::sp_core::crypto::AccountId32,
									::core::primitive::u32,
								>,
							>,
						},
						#[codec(index = 23)]
						#[doc = "Update the various staking limits this pallet."]
						#[doc = ""]
						#[doc = "* `min_nominator_bond`: The minimum active bond needed to be a nominator."]
						#[doc = "* `min_validator_bond`: The minimum active bond needed to be a validator."]
						#[doc = "* `max_nominator_count`: The max number of users who can be a nominator at once. When"]
						#[doc = "  set to `None`, no limit is enforced."]
						#[doc = "* `max_validator_count`: The max number of users who can be a validator at once. When"]
						#[doc = "  set to `None`, no limit is enforced."]
						#[doc = ""]
						#[doc = "Origin must be Root to call this function."]
						#[doc = ""]
						#[doc = "NOTE: Existing nominators and validators will not be affected by this update."]
						#[doc = "to kick people under the new limits, `chill_other` should be called."]
						set_staking_limits {
							min_nominator_bond: ::core::primitive::u128,
							min_validator_bond: ::core::primitive::u128,
							max_nominator_count: ::core::option::Option<::core::primitive::u32>,
							max_validator_count: ::core::option::Option<::core::primitive::u32>,
							threshold: ::core::option::Option<
								runtime_types::sp_arithmetic::per_things::Percent,
							>,
						},
						#[codec(index = 24)]
						#[doc = "Declare a `controller` to stop participating as either a validator or nominator."]
						#[doc = ""]
						#[doc = "Effects will be felt at the beginning of the next era."]
						#[doc = ""]
						#[doc = "The dispatch origin for this call must be _Signed_, but can be called by anyone."]
						#[doc = ""]
						#[doc = "If the caller is the same as the controller being targeted, then no further checks are"]
						#[doc = "enforced, and this function behaves just like `chill`."]
						#[doc = ""]
						#[doc = "If the caller is different than the controller being targeted, the following conditions"]
						#[doc = "must be met:"]
						#[doc = "* A `ChillThreshold` must be set and checked which defines how close to the max"]
						#[doc = "  nominators or validators we must reach before users can start chilling one-another."]
						#[doc = "* A `MaxNominatorCount` and `MaxValidatorCount` must be set which is used to determine"]
						#[doc = "  how close we are to the threshold."]
						#[doc = "* A `MinNominatorBond` and `MinValidatorBond` must be set and checked, which determines"]
						#[doc = "  if this is a person that should be chilled because they have not met the threshold"]
						#[doc = "  bond required."]
						#[doc = ""]
						#[doc = "This can be helpful if bond requirements are updated, and we need to remove old users"]
						#[doc = "who do not satisfy these requirements."]
						chill_other {
							controller: ::subxt::ext::sp_core::crypto::AccountId32,
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
					#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
						#[doc = "There are too many validators in the system. Governance needs to adjust the staking"]
						#[doc = "settings to keep things safe for the runtime."]
						TooManyValidators,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
					pub enum Event {
						#[codec(index = 0)]
						#[doc = "The era payout has been set; the first balance is the validator-payout; the second is"]
						#[doc = "the remainder from the maximum amount of reward."]
						#[doc = "\\[era_index, validator_payout, remainder\\]"]
						EraPaid(
							::core::primitive::u32,
							::core::primitive::u128,
							::core::primitive::u128,
						),
						#[codec(index = 1)]
						#[doc = "The nominator has been rewarded by this amount. \\[stash, amount\\]"]
						Rewarded(
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						),
						#[codec(index = 2)]
						#[doc = "One validator (and its nominators) has been slashed by the given amount."]
						#[doc = "\\[validator, amount\\]"]
						Slashed(
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						),
						#[codec(index = 3)]
						#[doc = "An old slashing report from a prior era was discarded because it could"]
						#[doc = "not be processed. \\[session_index\\]"]
						OldSlashingReportDiscarded(::core::primitive::u32),
						#[codec(index = 4)]
						#[doc = "A new set of stakers was elected."]
						StakersElected,
						#[codec(index = 5)]
						#[doc = "An account has bonded this amount. \\[stash, amount\\]"]
						#[doc = ""]
						#[doc = "NOTE: This event is only emitted when funds are bonded via a dispatchable. Notably,"]
						#[doc = "it will not be emitted for staking rewards when they are added to stake."]
						Bonded(
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						),
						#[codec(index = 6)]
						#[doc = "An account has unbonded this amount. \\[stash, amount\\]"]
						Unbonded(
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						),
						#[codec(index = 7)]
						#[doc = "An account has called `withdraw_unbonded` and removed unbonding chunks worth `Balance`"]
						#[doc = "from the unlocking queue. \\[stash, amount\\]"]
						Withdrawn(
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						),
						#[codec(index = 8)]
						#[doc = "A nominator has been kicked from a validator. \\[nominator, stash\\]"]
						Kicked(
							::subxt::ext::sp_core::crypto::AccountId32,
							::subxt::ext::sp_core::crypto::AccountId32,
						),
						#[codec(index = 9)]
						#[doc = "The election failed. No new era is planned."]
						StakingElectionFailed,
						#[codec(index = 10)]
						#[doc = "An account has stopped participating as either a validator or nominator."]
						#[doc = "\\[stash\\]"]
						Chilled(::subxt::ext::sp_core::crypto::AccountId32),
						#[codec(index = 11)]
						#[doc = "The stakers' rewards are getting paid. \\[era_index, validator_stash\\]"]
						PayoutStarted(
							::core::primitive::u32,
							::subxt::ext::sp_core::crypto::AccountId32,
						),
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
			pub struct Nominations<_0> {
				pub targets: ::std::vec::Vec<_0>,
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
			pub enum Releases {
				#[codec(index = 0)]
				V1_0_0Ancient,
				#[codec(index = 1)]
				V2_0_0,
				#[codec(index = 2)]
				V3_0_0,
				#[codec(index = 3)]
				V4_0_0,
				#[codec(index = 4)]
				V5_0_0,
				#[codec(index = 5)]
				V6_0_0,
				#[codec(index = 6)]
				V7_0_0,
				#[codec(index = 7)]
				V8_0_0,
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
			pub struct StakingLedger<_0, _1> {
				pub stash: _0,
				#[codec(compact)]
				pub total: _1,
				#[codec(compact)]
				pub active: _1,
				pub unlocking: ::std::vec::Vec<runtime_types::pallet_staking::UnlockChunk<_1>>,
				pub claimed_rewards: ::std::vec::Vec<::core::primitive::u32>,
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- Limited storage reads."]
					#[doc = "- One DB write (event)."]
					#[doc = "- Weight of derivative `call` execution + 10,000."]
					#[doc = "# </weight>"]
					sudo {
						call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
					},
					#[codec(index = 1)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
					#[doc = "This function does not check the weight of the call, and instead allows the"]
					#[doc = "Sudo user to specify the weight of the call."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- The weight of this call is defined by the caller."]
					#[doc = "# </weight>"]
					sudo_unchecked_weight {
						call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
						weight: ::core::primitive::u64,
					},
					#[codec(index = 2)]
					#[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
					#[doc = "key."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- Limited storage reads."]
					#[doc = "- One DB change."]
					#[doc = "# </weight>"]
					set_key {
						new: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 3)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
					#[doc = "a given account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- Limited storage reads."]
					#[doc = "- One DB write (event)."]
					#[doc = "- Weight of derivative `call` execution + 10,000."]
					#[doc = "# </weight>"]
					sudo_as {
						who: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
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
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A sudo just took place. \\[result\\]"]
					Sudid {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					#[doc = "The \\[sudoer\\] just switched identity; the old key is supplied."]
					KeyChanged {
						new_sudoer: ::subxt::ext::sp_core::crypto::AccountId32,
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Set the current time."]
					#[doc = ""]
					#[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
					#[doc = "phase, if this call hasn't been invoked by that time."]
					#[doc = ""]
					#[doc = "The timestamp should be greater than the previous one by the amount specified by"]
					#[doc = "`MinimumPeriod`."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Inherent`."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
					#[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
					#[doc = "  `on_finalize`)"]
					#[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
					#[doc = "# </weight>"]
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Report something `reason` that deserves a tip and claim any eventual the finder's fee."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "Payment: `TipReportDepositBase` will be reserved from the origin account, as well as"]
					#[doc = "`DataDepositPerByte` for each byte in `reason`."]
					#[doc = ""]
					#[doc = "- `reason`: The reason for, or the thing that deserves, the tip; generally this will be"]
					#[doc = "  a UTF-8-encoded URL."]
					#[doc = "- `who`: The account which should be credited for the tip."]
					#[doc = ""]
					#[doc = "Emits `NewTip` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: `O(R)` where `R` length of `reason`."]
					#[doc = "  - encoding and hashing of 'reason'"]
					#[doc = "- DbReads: `Reasons`, `Tips`"]
					#[doc = "- DbWrites: `Reasons`, `Tips`"]
					#[doc = "# </weight>"]
					report_awesome {
						reason: ::std::vec::Vec<::core::primitive::u8>,
						who: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 1)]
					#[doc = "Retract a prior tip-report from `report_awesome`, and cancel the process of tipping."]
					#[doc = ""]
					#[doc = "If successful, the original deposit will be unreserved."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_ and the tip identified by `hash`"]
					#[doc = "must have been reported by the signing account through `report_awesome` (and not"]
					#[doc = "through `tip_new`)."]
					#[doc = ""]
					#[doc = "- `hash`: The identity of the open tip for which a tip value is declared. This is formed"]
					#[doc = "  as the hash of the tuple of the original tip `reason` and the beneficiary account ID."]
					#[doc = ""]
					#[doc = "Emits `TipRetracted` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: `O(1)`"]
					#[doc = "  - Depends on the length of `T::Hash` which is fixed."]
					#[doc = "- DbReads: `Tips`, `origin account`"]
					#[doc = "- DbWrites: `Reasons`, `Tips`, `origin account`"]
					#[doc = "# </weight>"]
					retract_tip { hash: ::subxt::ext::sp_core::H256 },
					#[codec(index = 2)]
					#[doc = "Give a tip for something new; no finder's fee will be taken."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_ and the signing account must be a"]
					#[doc = "member of the `Tippers` set."]
					#[doc = ""]
					#[doc = "- `reason`: The reason for, or the thing that deserves, the tip; generally this will be"]
					#[doc = "  a UTF-8-encoded URL."]
					#[doc = "- `who`: The account which should be credited for the tip."]
					#[doc = "- `tip_value`: The amount of tip that the sender would like to give. The median tip"]
					#[doc = "  value of active tippers will be given to the `who`."]
					#[doc = ""]
					#[doc = "Emits `NewTip` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: `O(R + T)` where `R` length of `reason`, `T` is the number of tippers."]
					#[doc = "  - `O(T)`: decoding `Tipper` vec of length `T`. `T` is charged as upper bound given by"]
					#[doc = "    `ContainsLengthBound`. The actual cost depends on the implementation of"]
					#[doc = "    `T::Tippers`."]
					#[doc = "  - `O(R)`: hashing and encoding of reason of length `R`"]
					#[doc = "- DbReads: `Tippers`, `Reasons`"]
					#[doc = "- DbWrites: `Reasons`, `Tips`"]
					#[doc = "# </weight>"]
					tip_new {
						reason: ::std::vec::Vec<::core::primitive::u8>,
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						#[codec(compact)]
						tip_value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "Declare a tip value for an already-open tip."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_ and the signing account must be a"]
					#[doc = "member of the `Tippers` set."]
					#[doc = ""]
					#[doc = "- `hash`: The identity of the open tip for which a tip value is declared. This is formed"]
					#[doc = "  as the hash of the tuple of the hash of the original tip `reason` and the beneficiary"]
					#[doc = "  account ID."]
					#[doc = "- `tip_value`: The amount of tip that the sender would like to give. The median tip"]
					#[doc = "  value of active tippers will be given to the `who`."]
					#[doc = ""]
					#[doc = "Emits `TipClosing` if the threshold of tippers has been reached and the countdown period"]
					#[doc = "has started."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: `O(T)` where `T` is the number of tippers. decoding `Tipper` vec of length"]
					#[doc = "  `T`, insert tip and check closing, `T` is charged as upper bound given by"]
					#[doc = "  `ContainsLengthBound`. The actual cost depends on the implementation of `T::Tippers`."]
					#[doc = ""]
					#[doc = "  Actually weight could be lower as it depends on how many tips are in `OpenTip` but it"]
					#[doc = "  is weighted as if almost full i.e of length `T-1`."]
					#[doc = "- DbReads: `Tippers`, `Tips`"]
					#[doc = "- DbWrites: `Tips`"]
					#[doc = "# </weight>"]
					tip {
						hash: ::subxt::ext::sp_core::H256,
						#[codec(compact)]
						tip_value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Close and payout a tip."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "The tip identified by `hash` must have finished its countdown period."]
					#[doc = ""]
					#[doc = "- `hash`: The identity of the open tip for which a tip value is declared. This is formed"]
					#[doc = "  as the hash of the tuple of the original tip `reason` and the beneficiary account ID."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: `O(T)` where `T` is the number of tippers. decoding `Tipper` vec of length"]
					#[doc = "  `T`. `T` is charged as upper bound given by `ContainsLengthBound`. The actual cost"]
					#[doc = "  depends on the implementation of `T::Tippers`."]
					#[doc = "- DbReads: `Tips`, `Tippers`, `tip finder`"]
					#[doc = "- DbWrites: `Reasons`, `Tips`, `Tippers`, `tip finder`"]
					#[doc = "# </weight>"]
					close_tip { hash: ::subxt::ext::sp_core::H256 },
					#[codec(index = 5)]
					#[doc = "Remove and slash an already-open tip."]
					#[doc = ""]
					#[doc = "May only be called from `T::RejectOrigin`."]
					#[doc = ""]
					#[doc = "As a result, the finder is slashed and the deposits are lost."]
					#[doc = ""]
					#[doc = "Emits `TipSlashed` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "  `T` is charged as upper bound given by `ContainsLengthBound`."]
					#[doc = "  The actual cost depends on the implementation of `T::Tippers`."]
					#[doc = "# </weight>"]
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
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Put forward a suggestion for spending. A deposit proportional to the value"]
					#[doc = "is reserved and slashed if the proposal is rejected. It is returned once the"]
					#[doc = "proposal is awarded."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(1)"]
					#[doc = "- DbReads: `ProposalCount`, `origin account`"]
					#[doc = "- DbWrites: `ProposalCount`, `Proposals`, `origin account`"]
					#[doc = "# </weight>"]
					propose_spend {
						#[codec(compact)]
						value: ::core::primitive::u128,
						beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "Reject a proposed spend. The original deposit will be slashed."]
					#[doc = ""]
					#[doc = "May only be called from `T::RejectOrigin`."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(1)"]
					#[doc = "- DbReads: `Proposals`, `rejected proposer account`"]
					#[doc = "- DbWrites: `Proposals`, `rejected proposer account`"]
					#[doc = "# </weight>"]
					reject_proposal {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Approve a proposal. At a later time, the proposal will be allocated to the beneficiary"]
					#[doc = "and the original deposit will be returned."]
					#[doc = ""]
					#[doc = "May only be called from `T::ApproveOrigin`."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(1)."]
					#[doc = "- DbReads: `Proposals`, `Approvals`"]
					#[doc = "- DbWrite: `Approvals`"]
					#[doc = "# </weight>"]
					approve_proposal {
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
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New proposal. \\[proposal_index\\]"]
					Proposed(::core::primitive::u32),
					#[codec(index = 1)]
					#[doc = "We have ended a spend period and will now allocate funds. \\[budget_remaining\\]"]
					Spending(::core::primitive::u128),
					#[codec(index = 2)]
					#[doc = "Some funds have been allocated. \\[proposal_index, award, beneficiary\\]"]
					Awarded(
						::core::primitive::u32,
						::core::primitive::u128,
						::subxt::ext::sp_core::crypto::AccountId32,
					),
					#[codec(index = 3)]
					#[doc = "A proposal was rejected; funds were slashed. \\[proposal_index, slashed\\]"]
					Rejected(::core::primitive::u32, ::core::primitive::u128),
					#[codec(index = 4)]
					#[doc = "Some of our funds have been burnt. \\[burn\\]"]
					Burnt(::core::primitive::u128),
					#[codec(index = 5)]
					#[doc = "Spending has finished; this is the amount that rolls over until next spend."]
					#[doc = "\\[budget_remaining\\]"]
					Rollover(::core::primitive::u128),
					#[codec(index = 6)]
					#[doc = "Some funds have been deposited. \\[deposit\\]"]
					Deposit(::core::primitive::u128),
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Send a batch of dispatch calls."]
					#[doc = ""]
					#[doc = "May be called from any origin."]
					#[doc = ""]
					#[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
					#[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
					#[doc = ""]
					#[doc = "If origin is root then call are dispatch without checking origin filter. (This includes"]
					#[doc = "bypassing `frame_system::Config::BaseCallFilter`)."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
					#[doc = "# </weight>"]
					#[doc = ""]
					#[doc = "This will return `Ok` in all circumstances. To determine the success of the batch, an"]
					#[doc = "event is deposited. If a call failed and the batch was interrupted, then the"]
					#[doc = "`BatchInterrupted` event is deposited, along with the number of successful calls made"]
					#[doc = "and the error of the failed call. If all were successful, then the `BatchCompleted`"]
					#[doc = "event is deposited."]
					batch {
						calls: ::std::vec::Vec<runtime_types::da_runtime::Call>,
					},
					#[codec(index = 1)]
					#[doc = "Send a call through an indexed pseudonym of the sender."]
					#[doc = ""]
					#[doc = "Filter from origin are passed along. The call will be dispatched with an origin which"]
					#[doc = "use the same filter as the origin of this call."]
					#[doc = ""]
					#[doc = "NOTE: If you need to ensure that any account-based filtering is not honored (i.e."]
					#[doc = "because you expect `proxy` to have been used prior in the call stack and you do not want"]
					#[doc = "the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`"]
					#[doc = "in the Multisig pallet instead."]
					#[doc = ""]
					#[doc = "NOTE: Prior to version *12, this was called `as_limited_sub`."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					as_derivative {
						index: ::core::primitive::u16,
						call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
					},
					#[codec(index = 2)]
					#[doc = "Send a batch of dispatch calls and atomically execute them."]
					#[doc = "The whole transaction will rollback and fail if any of the calls failed."]
					#[doc = ""]
					#[doc = "May be called from any origin."]
					#[doc = ""]
					#[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
					#[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
					#[doc = ""]
					#[doc = "If origin is root then call are dispatch without checking origin filter. (This includes"]
					#[doc = "bypassing `frame_system::Config::BaseCallFilter`)."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
					#[doc = "# </weight>"]
					batch_all {
						calls: ::std::vec::Vec<runtime_types::da_runtime::Call>,
					},
					#[codec(index = 3)]
					#[doc = "Dispatches a function call with a provided origin."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Root_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- Limited storage reads."]
					#[doc = "- One DB write (event)."]
					#[doc = "- Weight of derivative `call` execution + T::WeightInfo::dispatch_as()."]
					#[doc = "# </weight>"]
					dispatch_as {
						as_origin: ::std::boxed::Box<runtime_types::da_runtime::OriginCaller>,
						call: ::std::boxed::Box<runtime_types::da_runtime::Call>,
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
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
					#[doc = "A single item within a Batch of dispatches has completed with no error."]
					ItemCompleted,
					#[codec(index = 3)]
					#[doc = "A call was dispatched. \\[result\\]"]
					DispatchedAs(
						::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					),
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
		pub mod signature {
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
			pub mod offchain {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct OpaqueMultiaddr(pub ::std::vec::Vec<::core::primitive::u8>);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				pub struct OpaqueNetworkState {
					pub peer_id: runtime_types::sp_core::OpaquePeerId,
					pub external_addresses:
						::std::vec::Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>,
				}
			}
			pub mod sr25519 {
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
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
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
		pub mod sp_finality_grandpa {
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
						runtime_types::sp_finality_grandpa::app::Public,
						runtime_types::finality_grandpa::Prevote<_0, _1>,
						runtime_types::sp_finality_grandpa::app::Signature,
					>,
				),
				#[codec(index = 1)]
				Precommit(
					runtime_types::finality_grandpa::Equivocation<
						runtime_types::sp_finality_grandpa::app::Public,
						runtime_types::finality_grandpa::Precommit<_0, _1>,
						runtime_types::sp_finality_grandpa::app::Signature,
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
				pub equivocation: runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
			}
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
			pub enum ArithmeticError {
				#[codec(index = 0)]
				Underflow,
				#[codec(index = 1)]
				Overflow,
				#[codec(index = 2)]
				DivisionByZero,
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
				Module {
					index: ::core::primitive::u8,
					error: ::core::primitive::u8,
				},
				#[codec(index = 4)]
				ConsumerRemaining,
				#[codec(index = 5)]
				NoProviders,
				#[codec(index = 6)]
				Token(runtime_types::sp_runtime::TokenError),
				#[codec(index = 7)]
				Arithmetic(runtime_types::sp_runtime::ArithmeticError),
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
				NoFunds,
				#[codec(index = 1)]
				WouldDie,
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
			}
		}
		pub mod updater_manager {
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
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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

		pub fn authorship(&self) -> authorship::constants::ConstantsApi {
			authorship::constants::ConstantsApi
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

		pub fn democracy(&self) -> democracy::constants::ConstantsApi {
			democracy::constants::ConstantsApi
		}

		pub fn elections(&self) -> elections::constants::ConstantsApi {
			elections::constants::ConstantsApi
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

		pub fn bags_list(&self) -> bags_list::constants::ConstantsApi {
			bags_list::constants::ConstantsApi
		}

		pub fn data_availability(&self) -> data_availability::constants::ConstantsApi {
			data_availability::constants::ConstantsApi
		}

		pub fn nomad_home(&self) -> nomad_home::constants::ConstantsApi {
			nomad_home::constants::ConstantsApi
		}

		pub fn da_bridge(&self) -> da_bridge::constants::ConstantsApi {
			da_bridge::constants::ConstantsApi
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

		pub fn democracy(&self) -> democracy::storage::StorageApi { democracy::storage::StorageApi }

		pub fn council(&self) -> council::storage::StorageApi { council::storage::StorageApi }

		pub fn technical_committee(&self) -> technical_committee::storage::StorageApi {
			technical_committee::storage::StorageApi
		}

		pub fn elections(&self) -> elections::storage::StorageApi { elections::storage::StorageApi }

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

		pub fn bags_list(&self) -> bags_list::storage::StorageApi { bags_list::storage::StorageApi }

		pub fn data_availability(&self) -> data_availability::storage::StorageApi {
			data_availability::storage::StorageApi
		}

		pub fn updater_manager(&self) -> updater_manager::storage::StorageApi {
			updater_manager::storage::StorageApi
		}

		pub fn nomad_home(&self) -> nomad_home::storage::StorageApi {
			nomad_home::storage::StorageApi
		}
	}
	pub struct TransactionApi;
	impl TransactionApi {
		pub fn system(&self) -> system::calls::TransactionApi { system::calls::TransactionApi }

		pub fn utility(&self) -> utility::calls::TransactionApi { utility::calls::TransactionApi }

		pub fn babe(&self) -> babe::calls::TransactionApi { babe::calls::TransactionApi }

		pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
			timestamp::calls::TransactionApi
		}

		pub fn authorship(&self) -> authorship::calls::TransactionApi {
			authorship::calls::TransactionApi
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

		pub fn democracy(&self) -> democracy::calls::TransactionApi {
			democracy::calls::TransactionApi
		}

		pub fn council(&self) -> council::calls::TransactionApi { council::calls::TransactionApi }

		pub fn technical_committee(&self) -> technical_committee::calls::TransactionApi {
			technical_committee::calls::TransactionApi
		}

		pub fn elections(&self) -> elections::calls::TransactionApi {
			elections::calls::TransactionApi
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

		pub fn bags_list(&self) -> bags_list::calls::TransactionApi {
			bags_list::calls::TransactionApi
		}

		pub fn data_availability(&self) -> data_availability::calls::TransactionApi {
			data_availability::calls::TransactionApi
		}

		pub fn updater_manager(&self) -> updater_manager::calls::TransactionApi {
			updater_manager::calls::TransactionApi
		}

		pub fn nomad_home(&self) -> nomad_home::calls::TransactionApi {
			nomad_home::calls::TransactionApi
		}

		pub fn da_bridge(&self) -> da_bridge::calls::TransactionApi {
			da_bridge::calls::TransactionApi
		}
	}
	#[doc = r" check whether the Client you are using is aligned with the statically generated codegen."]
	pub fn validate_codegen<T: ::subxt::Config, C: ::subxt::client::OfflineClientT<T>>(
		client: &C,
	) -> Result<(), ::subxt::error::MetadataError> {
		let runtime_metadata_hash = client.metadata().metadata_hash(&PALLETS);
		if runtime_metadata_hash
			!= [
				127u8, 27u8, 160u8, 192u8, 160u8, 243u8, 224u8, 64u8, 42u8, 161u8, 227u8, 35u8,
				148u8, 233u8, 171u8, 124u8, 33u8, 107u8, 66u8, 128u8, 225u8, 6u8, 146u8, 177u8,
				106u8, 174u8, 22u8, 58u8, 202u8, 41u8, 84u8, 103u8,
			] {
			Err(::subxt::error::MetadataError::IncompatibleMetadata)
		} else {
			Ok(())
		}
	}
}
