How to build the API

1. Run Node
2. Run `./build_api.sh` script
3. Manually do the following change:

```diff
 				#[codec(index = 42)]
-				TreasuryCommittee(runtime_types::pallet_collective::pallet::Call),
+				TreasuryCommittee(runtime_types::pallet_collective::pallet::Call2),
```

and

Inisde `pub mod api { pub mod runtime_types { pub mod pallet_collective { pub mod pallet }}}` duplicate the whole `pub enum Call` enum and
call it `pub enum Call2`. (Around line 30168)

Should look like this

```diff
				#[derive(
					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
				#[codec(dumb_trait_bound)]
				#[decode_as_type(
					crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
				)]
				#[encode_as_type(
					crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
				)]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::set_members`]."]
					set_members {
						new_members: ::subxt::ext::subxt_core::alloc::vec::Vec<
							::subxt::ext::subxt_core::utils::AccountId32,
						>,
						prime: ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>,
						old_count: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::execute`]."]
					execute {
						proposal: ::subxt::ext::subxt_core::alloc::boxed::Box<
							runtime_types::da_runtime::RuntimeCall,
						>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::propose`]."]
					propose {
						#[codec(compact)]
						threshold: ::core::primitive::u32,
						proposal: ::subxt::ext::subxt_core::alloc::boxed::Box<
							runtime_types::da_runtime::RuntimeCall,
						>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::vote`]."]
					vote {
						proposal: ::subxt::ext::subxt_core::utils::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						approve: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::disapprove_proposal`]."]
					disapprove_proposal {
						proposal_hash: ::subxt::ext::subxt_core::utils::H256,
					},
					#[codec(index = 6)]
					#[doc = "See [`Pallet::close`]."]
					close {
						proposal_hash: ::subxt::ext::subxt_core::utils::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
				}
+				#[derive(
+					:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
+					:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
+					:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
+					:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
+					Clone,
+					Debug,
+					Eq,
+					PartialEq,
+				)]
+				# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
+				#[codec(dumb_trait_bound)]
+				#[decode_as_type(
+					crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
+				)]
+				#[encode_as_type(
+					crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
+				)]
+				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
+				pub enum Call2 {
+					#[codec(index = 0)]
+					#[doc = "See [`Pallet::set_members`]."]
+					set_members {
+						new_members: ::subxt::ext::subxt_core::alloc::vec::Vec<
+							::subxt::ext::subxt_core::utils::AccountId32,
+						>,
+						prime: ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>,
+						old_count: ::core::primitive::u32,
+					},
+					#[codec(index = 1)]
+					#[doc = "See [`Pallet::execute`]."]
+					execute {
+						proposal: ::subxt::ext::subxt_core::alloc::boxed::Box<
+							runtime_types::da_runtime::RuntimeCall,
+						>,
+						#[codec(compact)]
+						length_bound: ::core::primitive::u32,
+					},
+					#[codec(index = 2)]
+					#[doc = "See [`Pallet::propose`]."]
+					propose {
+						#[codec(compact)]
+						threshold: ::core::primitive::u32,
+						proposal: ::subxt::ext::subxt_core::alloc::boxed::Box<
+							runtime_types::da_runtime::RuntimeCall,
+						>,
+						#[codec(compact)]
+						length_bound: ::core::primitive::u32,
+					},
+					#[codec(index = 3)]
+					#[doc = "See [`Pallet::vote`]."]
+					vote {
+						proposal: ::subxt::ext::subxt_core::utils::H256,
+						#[codec(compact)]
+						index: ::core::primitive::u32,
+						approve: ::core::primitive::bool,
+					},
+					#[codec(index = 5)]
+					#[doc = "See [`Pallet::disapprove_proposal`]."]
+					disapprove_proposal {
+						proposal_hash: ::subxt::ext::subxt_core::utils::H256,
+					},
+					#[codec(index = 6)]
+					#[doc = "See [`Pallet::close`]."]
+					close {
+						proposal_hash: ::subxt::ext::subxt_core::utils::H256,
+						#[codec(compact)]
+						index: ::core::primitive::u32,
+						proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
+						#[codec(compact)]
+						length_bound: ::core::primitive::u32,
+					},
+				}
```
