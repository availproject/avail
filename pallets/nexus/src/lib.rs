#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use codec::{Decode, Encode, MaxEncodedLen};
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::{StorageMap, StorageValue},
		Identity,
	};
	use frame_system::pallet_prelude::OriginFor;
	use risc0_zkvm::{InnerReceipt, Receipt};
	use scale_info::TypeInfo;
	use serde::{Deserialize, Serialize};
	use sp_core::{bytes::deserialize, RuntimeDebug, H256};
  use sp_std::vec::Vec;

	#[pallet::error]
	pub enum Error<T> {
		NotLatestState,
        IncorrectHeaderEncoding,
		WrongFork,
        MockError,
		IncorrectReceiptEncoding,
		ProofVerificationFailed,
		InvalidProof,
		ImageIDNotSet,
	}

	#[derive(
		Clone,
		Encode,
		Decode,
		PartialEq,
		Serialize,
		Deserialize,
		Copy,
		RuntimeDebug,
		TypeInfo,
		MaxEncodedLen,
	)]
	pub struct NexusHeader {
		pub parent_hash: H256,
		pub prev_state_root: H256,
		pub state_root: H256,
		pub tx_root: H256,
		pub avail_header_hash: H256,
		pub number: u32,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct NexusStateRoot {
		pub number: u32,
		pub state_root: H256,
	}

	/// Maps Avail Block hash to Nexus Header
	// #[pallet::storage]
	// pub(super) type NexusStateHeader<T: Config> = StorageMap<_, Identity, H256, NexusHeader>;

	/// Latest proven nexus state root
	#[pallet::storage]
	pub(super) type LatestStateRoot<T: Config> = StorageValue<_, NexusStateRoot>;

	/// Latest proven nexus state root
	#[pallet::storage]
	pub(super) type ImageID<T: Config> = StorageValue<_, [u32; 8]>;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// Pallet internal functions
	impl<T: Config> Pallet<T> {
		fn set_image_id(image_id: [u32; 8]) -> Result<(), Error<T>> {
			ImageID::<T>::put(image_id);

			Ok(())
		}
		// Generates and returns the unique_id and color
		fn update_state(nexus_header: NexusHeader, risc0_receipt: Vec<u8>) -> Result<(), Error<T>> {
			let image_id = match ImageID::<T>::get() {
				Some(id) => id,
				None => return Err(Error::<T>::ImageIDNotSet),
			};

			let decoded_receipt: Receipt = serde_json::from_slice(risc0_receipt.as_slice())
				.map_err(|_| Error::<T>::IncorrectReceiptEncoding)?;

			let proof_header: NexusHeader = decoded_receipt
				.journal
				.decode()
				.map_err(|_| Error::<T>::IncorrectHeaderEncoding)?;

			if proof_header != nexus_header {
				return Err(Error::<T>::WrongFork);
			}

			match decoded_receipt.inner {
				InnerReceipt::Fake(_) => (),
				InnerReceipt::Groth16(_) => decoded_receipt
					.verify(image_id)
					.map_err(|_| Error::<T>::ProofVerificationFailed)?,
				_ => return Err(Error::<T>::InvalidProof),
			}

			let previous_state_opt = LatestStateRoot::<T>::get();

			if let Some(previous_state) = previous_state_opt {
				if previous_state.number >= nexus_header.number {
					return Err(Error::<T>::NotLatestState);
				}
			}

			LatestStateRoot::<T>::put(NexusStateRoot {
				number: nexus_header.number,
				state_root: nexus_header.state_root,
			});

			Ok(())
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn update_state_root(
			origin: OriginFor<T>,
			nexus_header: NexusHeader,
			risc0_receipt: Vec<u8>,
		) -> DispatchResult {
			Self::update_state(nexus_header, risc0_receipt)?;

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn update_image_id(origin: OriginFor<T>, image_id: [u32; 8]) -> DispatchResult {
			Self::set_image_id(image_id)?;
			Ok(())
		}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {}
}
