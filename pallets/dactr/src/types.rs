use crate::Config;
use avail_core::AppId;
use codec::{DecodeWithMemTracking, Encode};
use frame_support::pallet_prelude::*;
use scale_info::prelude::string::String;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId;
use sp_core::crypto::KeyTypeId;
use sp_core::sr25519::Public;
use sp_core::{sr25519, H256};
use sp_runtime::traits::Verify;
use sp_runtime::AccountId32;
use sp_runtime::Perbill;
// use sp_staking::offence::{DisableStrategy, Offence};
use sp_staking::offence::Offence;
use sp_staking::SessionIndex;
use sp_std::cmp::Ordering;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec::Vec;

pub type AppKeyFor<T> = BoundedVec<u8, <T as Config>::MaxAppKeyLength>;
pub type AppDataFor<T> = BoundedVec<u8, <T as Config>::MaxAppDataLength>;

/// Maximum encoded FRI evaluation-proof payload carried by one blob summary.
///
/// Current measurements peak below 660 KiB for a 32 MiB blob. The additional 108 KiB provides
/// headroom for encoding and proof-parameter changes without leaving this consensus-critical field
/// unbounded.
pub const MAX_EVAL_PROOF_SIZE: u32 = 768 * 1024;
pub const MAX_BLOB_SIZE: u64 = 32 * 1024 * 1024;
pub type BoundedEvalProof = BoundedVec<u8, ConstU32<MAX_EVAL_PROOF_SIZE>>;
pub const MAX_BLOB_SUMMARY_REASON_SIZE: u32 = 256;
/// Maximum ownership entries accepted in one blob summary.
///
/// This is intentionally aligned with the network assumption that the active validator set,
/// and therefore the owner set for one blob, will not exceed 100 entries. Raising this limit
/// requires revisiting the summary weight budget and the block-capacity analysis.
pub const MAX_BLOB_OWNERS: u32 = 100;
pub const MAX_ENCODED_PEER_ID_SIZE: u32 = 128;
pub const MAX_OWNERSHIP_SIGNATURE_SIZE: u32 = 64;
pub const MAX_BLOB_TX_SUMMARIES: u32 = avail_base::MAX_BLOB_TXS_PER_BLOCK as u32;

pub type BoundedBlobSummaryReason = BoundedVec<u8, ConstU32<MAX_BLOB_SUMMARY_REASON_SIZE>>;
pub type BoundedEncodedPeerId = BoundedVec<u8, ConstU32<MAX_ENCODED_PEER_ID_SIZE>>;
pub type BoundedOwnershipSignature = BoundedVec<u8, ConstU32<MAX_OWNERSHIP_SIGNATURE_SIZE>>;
pub type BoundedBlobOwnership = (
	AccountId32,
	AuthorityId,
	BoundedEncodedPeerId,
	BoundedOwnershipSignature,
);
pub type BoundedBlobOwnerships = BoundedVec<BoundedBlobOwnership, ConstU32<MAX_BLOB_OWNERS>>;
pub type BoundedBlobTxSummaries = BoundedVec<BlobTxSummaryRuntime, ConstU32<MAX_BLOB_TX_SUMMARIES>>;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Encode, Decode, TypeInfo, PartialEq, Debug, MaxEncodedLen)]
pub struct AppKeyInfo<Acc: PartialEq> {
	/// Owner of the key
	pub owner: Acc,
	/// Application ID associated.
	pub id: AppId,
}
impl<Acc> AppKeyInfo<Acc>
where
	Acc: PartialEq,
{
	pub fn new(owner: Acc, id: AppId) -> Self {
		Self { owner, id }
	}
}

pub type AppKeyInfoFor<T> = AppKeyInfo<<T as frame_system::Config>::AccountId>;

pub trait SessionDataProvider<AccountId> {
	fn validators() -> Vec<AccountId>;
	fn get_validator_from_key(id: KeyTypeId, key_data: Vec<u8>) -> Option<AccountId>;
}
impl<AccountId> SessionDataProvider<AccountId> for () {
	fn validators() -> Vec<AccountId> {
		Vec::new()
	}
	fn get_validator_from_key(_id: KeyTypeId, _key_data: Vec<u8>) -> Option<AccountId> {
		None
	}
}

#[derive(
	Clone, Encode, Decode, DecodeWithMemTracking, TypeInfo, PartialEq, Debug, MaxEncodedLen,
)]
pub struct BlobTxSummaryRuntime {
	pub hash: H256,
	pub tx_index: u32,
	pub success: bool,
	pub reason: Option<BoundedBlobSummaryReason>,
	pub ownership: BoundedBlobOwnerships,
	pub eval_proof: Option<BoundedEvalProof>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SummaryConversionError {
	ReasonTooLong,
	PeerIdTooLong,
	SignatureTooLong,
	TooManyOwners,
	ProofTooLong,
	TooManySummaries,
}

impl BlobTxSummaryRuntime {
	pub fn convert_into(
		input: Vec<(
			H256,
			u32,
			bool,
			Option<String>,
			Vec<(AccountId32, AuthorityId, String, Vec<u8>)>,
			Option<Vec<u8>>,
		)>,
	) -> Result<BoundedBlobTxSummaries, SummaryConversionError> {
		let summaries = input.into_iter().map(
			|(hash, tx_index, success, reason, ownership, eval_proof)| -> Result<_, SummaryConversionError> {
				let ownership = ownership
					.into_iter()
					.map(|(account, authority, peer_id, signature)| {
						Ok((
							account,
							authority,
							BoundedEncodedPeerId::try_from(peer_id.into_bytes())
								.map_err(|_| SummaryConversionError::PeerIdTooLong)?,
							BoundedOwnershipSignature::try_from(signature)
								.map_err(|_| SummaryConversionError::SignatureTooLong)?,
						))
					})
					.collect::<Result<Vec<_>, SummaryConversionError>>()?;
				Ok(BlobTxSummaryRuntime {
					hash,
					tx_index,
					success,
					reason: reason
						.map(|reason| BoundedBlobSummaryReason::try_from(reason.into_bytes()))
						.transpose()
						.map_err(|_| SummaryConversionError::ReasonTooLong)?,
					ownership: BoundedBlobOwnerships::try_from(ownership)
						.map_err(|_| SummaryConversionError::TooManyOwners)?,
					eval_proof: eval_proof
						.map(BoundedEvalProof::try_from)
						.transpose()
						.map_err(|_| SummaryConversionError::ProofTooLong)?,
				})
			}
		);
		let summaries = summaries.collect::<Result<Vec<_>, _>>()?;
		BoundedBlobTxSummaries::try_from(summaries)
			.map_err(|_| SummaryConversionError::TooManySummaries)
	}
}

/// Structure for blob runtime parameters
#[derive(
	Debug, Clone, PartialEq, Eq, Encode, Decode, DecodeWithMemTracking, MaxEncodedLen, TypeInfo,
)]
pub struct BlobRuntimeParameters {
	/// Maximum size of a blob, need to change the rpc request and response size to handle this
	pub max_blob_size: u64,
	/// Minimum amount of validator that needs to store a blob, if we have less than minimum, everyone stores it.
	pub min_blob_holder_percentage: Perbill,
	/// We take the maximum between this and MIN_BLOB_HOLDER_PERCENTAGE
	pub min_blob_holder_count: u32,
	/// Amount of block for which we need to store the blob metadata and blob.
	pub blob_ttl: u64,
	/// Amount of block for which we need to store the blob metadata if the blob is not notified yet.
	pub temp_blob_ttl: u64,
	/// Min Amount of block for which the transaction submitted through RPC is valid.
	/// This value needs to handle the time to upload the blob to the store
	pub min_transaction_validity: u64,
	/// Max Amount of block for which the transaction submitted through RPC is valid.
	/// This value is used so a transaction won't be stuck in the mempool.
	pub max_transaction_validity: u64,
	/// The number of time we'll allow trying to fetch internal blob metadata or blob data before letting the transaction go through to get discarded
	pub max_blob_retry_before_discarding: u16,
	/// Reserved for a possible future aggregate blob-data limit.
	///
	/// This field is not currently used for block admission, runtime weight calculation, or fee calculation.
	pub max_block_size: u64,
	/// The threshold to consider a blob missing accusation valid
	pub vouch_threshold: u32,
}
impl Default for BlobRuntimeParameters {
	fn default() -> Self {
		Self {
			max_blob_size: MAX_BLOB_SIZE,
			min_blob_holder_percentage: Perbill::from_percent(10),
			min_blob_holder_count: 2,
			blob_ttl: 120_960,                    // 20sec block -> 28d - 6sec block -> 8.4d
			temp_blob_ttl: 180,                   // 20sec block -> 1h - 6sec block -> 18mn
			min_transaction_validity: 15,         // In blocks
			max_transaction_validity: 150,        // In blocks
			max_blob_retry_before_discarding: 10, // In blocks
			max_block_size: 3 * 1024 * 1024 * 1024, // 3gb
			vouch_threshold: 2,
		}
	}
}

/// Structure used when there is an offence reported by validators client side.
/// They all need some vouching from accusing validators and it needs to reach a threshold to be considered valid.
#[derive(
	Debug, Clone, PartialEq, Eq, Encode, Decode, DecodeWithMemTracking, MaxEncodedLen, TypeInfo,
)]
pub enum BlobOffenceKind {
	SummaryNbBlobMismatch,
	InvalidNbOfOwnershipForBlob,
	InvalidSignatureForBlob,
	DuplicateSignatureForBlob,
	InvalidValidatorForBlob,
	MissingValidatorForBlob,
	OmittedValidatorForBlob,
}

#[derive(
	Debug, Clone, PartialEq, Eq, Encode, Decode, DecodeWithMemTracking, MaxEncodedLen, TypeInfo,
)]
pub struct ValidatorVoucher {
	pub session_index: u32,
	pub validator: AccountId32,
	pub key: Public,
	pub signature: sr25519::Signature,
	/// Each node, from its client, will accused who they think is the block author.
	/// It's deterministic but since done in the client side,
	/// it can be played with meaning we need vote/thresholds.
	pub block_author: AccountId32,
}
impl ValidatorVoucher {
	pub fn verify_signature(&self, payload: Vec<u8>) -> bool {
		self.signature.verify(payload.as_slice(), &self.key)
	}
}

#[derive(
	Debug, Clone, PartialEq, Eq, Encode, Decode, DecodeWithMemTracking, MaxEncodedLen, TypeInfo,
)]
pub struct OffenceKey {
	pub block_hash: H256,
	pub kind: BlobOffenceKind,
	/// None for block-wide offences.
	pub blob_hash: Option<H256>,
	/// None if the offence has no missing_validator.
	pub missing_validator: Option<AccountId32>,
}
impl OffenceKey {
	pub fn is_valid(&self) -> bool {
		match self.kind {
			BlobOffenceKind::SummaryNbBlobMismatch => {
				self.blob_hash.is_none() && self.missing_validator.is_none()
			},
			BlobOffenceKind::InvalidNbOfOwnershipForBlob => {
				self.blob_hash.is_some() && self.missing_validator.is_none()
			},
			BlobOffenceKind::InvalidSignatureForBlob => {
				self.blob_hash.is_some() && self.missing_validator.is_none()
			},
			BlobOffenceKind::DuplicateSignatureForBlob => {
				self.blob_hash.is_some() && self.missing_validator.is_none()
			},
			BlobOffenceKind::InvalidValidatorForBlob => {
				self.blob_hash.is_some() && self.missing_validator.is_none()
			},
			BlobOffenceKind::MissingValidatorForBlob => {
				self.blob_hash.is_some() && self.missing_validator.is_some()
			},
			BlobOffenceKind::OmittedValidatorForBlob => {
				self.blob_hash.is_some() && self.missing_validator.is_some()
			},
		}
	}
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct OffenceRecord<T: Config> {
	/// What type of offence this record represents.
	pub kind: BlobOffenceKind,

	/// Always present: which block the offence is about.
	pub block_hash: H256,

	/// Optional: only set when the offence targets a blob.
	pub blob_hash: Option<H256>,

	/// Optional "missing_validator" account when the variant needs one:
	/// - MissingValidatorForBlob || OmittedValidatorForBlob -> Some(missing_validator)
	/// - others -> None
	pub missing_validator: Option<AccountId32>,

	/// Vouches gathered client-side.
	pub vouches: BoundedVec<ValidatorVoucher, T::MaxVouchesPerRecord>,
}

impl<T: Config> Clone for OffenceRecord<T> {
	fn clone(&self) -> Self {
		Self {
			kind: self.kind.clone(),
			block_hash: self.block_hash,
			blob_hash: self.blob_hash,
			missing_validator: self.missing_validator.clone(),
			vouches: self.vouches.clone(),
		}
	}
}

impl<T: Config> OffenceRecord<T> {
	pub fn new(
		kind: BlobOffenceKind,
		block_hash: H256,
		blob_hash: Option<H256>,
		missing_validator: Option<AccountId32>,
	) -> Self {
		Self {
			kind,
			block_hash,
			blob_hash,
			missing_validator,
			vouches: BoundedVec::default(),
		}
	}

	pub fn offence_key(&self) -> OffenceKey {
		OffenceKey {
			block_hash: self.block_hash,
			kind: self.kind.clone(),
			blob_hash: self.blob_hash,
			missing_validator: self.missing_validator.clone(),
		}
	}

	pub fn has_reached_threshold(&self, nb_validators: u32, max_vouches_per_record: u32) -> bool {
		let nb_validators = if cfg!(test) { 1 } else { nb_validators };
		let threshold = nb_validators.min(max_vouches_per_record);
		(self.vouches.len() as u32) >= threshold
	}

	pub fn get_block_author(&self) -> Option<AccountId32> {
		if self.vouches.is_empty() {
			return None;
		}

		// Count occurrences per block_author.
		let mut counts: BTreeMap<&AccountId32, u32> = BTreeMap::new();
		for v in &self.vouches {
			*counts.entry(&v.block_author).or_insert(0) += 1;
		}

		// Find the author with the highest count.
		let mut best_author: Option<&AccountId32> = None;
		let mut best_count: u32 = 0;
		let mut tie = false;

		for (author, count) in counts.iter() {
			match count.cmp(&best_count) {
				Ordering::Greater => {
					best_author = Some(author);
					best_count = *count;
					tie = false;
				},
				Ordering::Equal => {
					tie = true;
				},
				_ => {},
			}
		}

		if tie {
			log::warn!(
				"Tie detected while determining block author for block {:?}, offence {:?}",
				self.block_hash,
				self.kind
			);
			None
		} else {
			best_author.cloned()
		}
	}
}

#[derive(Clone, Encode, Decode, Debug, PartialEq, Eq, TypeInfo)]
pub struct BlobOffence<Offender> {
	pub kind: BlobOffenceKind,
	pub key: OffenceKey,
	pub session_index: SessionIndex,
	pub validator_set_count: u32,
	pub offenders: Vec<Offender>,
}
impl<Offender: Clone + Eq + Encode> Offence<Offender> for BlobOffence<Offender> {
	const ID: sp_staking::offence::Kind = *b"da/blob-offences";

	type TimeSlot = Vec<u8>;

	fn offenders(&self) -> Vec<Offender> {
		self.offenders.clone()
	}

	fn session_index(&self) -> SessionIndex {
		self.session_index
	}

	fn time_slot(&self) -> Self::TimeSlot {
		(self.session_index, self.key.clone()).encode()
	}

	fn slash_fraction(&self, _offenders_count: u32) -> Perbill {
		match self.kind {
			BlobOffenceKind::SummaryNbBlobMismatch => Perbill::from_percent(1),
			BlobOffenceKind::InvalidNbOfOwnershipForBlob => Perbill::from_percent(5),
			BlobOffenceKind::InvalidSignatureForBlob => Perbill::from_percent(10),
			BlobOffenceKind::DuplicateSignatureForBlob => Perbill::from_percent(10),
			BlobOffenceKind::InvalidValidatorForBlob => Perbill::from_percent(10),
			BlobOffenceKind::MissingValidatorForBlob => Perbill::from_percent(10),
			BlobOffenceKind::OmittedValidatorForBlob => Perbill::from_percent(10),
		}
	}

	fn validator_set_count(&self) -> u32 {
		self.validator_set_count
	}

	// fn disable_strategy(&self) -> DisableStrategy {
	// 	DisableStrategy::Never
	// }
}
