use codec::Decode;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_hex::{SerHex, StrictPfx};

use crate::api::runtime_types::{sp_consensus_babe::app::Public, sp_core::sr25519::Signature};

#[derive(Decode)]
pub struct AuthorityId(pub Public);

impl Serialize for AuthorityId {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let raw = hex::encode(self.0 .0 .0);
		serializer.serialize_str(&raw)
	}
}

#[derive(Decode)]
pub struct AuthoritySignature(pub Signature);

impl Serialize for AuthoritySignature {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let raw = hex::encode(self.0 .0);
		serializer.serialize_str(&raw)
	}
}

pub type AuthorityIndex = u32;
pub type BabeAuthorityWeight = u64;

/// Schnorrkel randomness value. Same size as `VRFOutput`.
pub const RANDOMNESS_LENGTH: usize = 32;
pub type Randomness = [u8; RANDOMNESS_LENGTH];

#[derive(Decode, Serialize)]
pub struct NextEpochDescriptor {
	/// The authorities.
	pub authorities: Vec<(AuthorityId, BabeAuthorityWeight)>,

	/// The value of randomness to use for the slot-assignment.
	#[serde(with = "SerHex::<StrictPfx>")]
	pub randomness: Randomness,
}

#[derive(Clone, Copy, Decode, Serialize)]
pub enum AllowedSlots {
	/// Only allow primary slots.
	PrimarySlots,
	/// Allow primary and secondary plain slots.
	PrimaryAndSecondaryPlainSlots,
	/// Allow primary and secondary VRF slots.
	PrimaryAndSecondaryVRFSlots,
}

#[derive(Decode, Serialize)]
pub enum NextConfigDescriptor {
	/// Version 1.
	#[codec(index = 1)]
	V1 {
		/// Value of `c` in `BabeEpochConfiguration`.
		c: (u64, u64),
		/// Value of `allowed_slots` in `BabeEpochConfiguration`.
		allowed_slots: AllowedSlots,
	},
}

/// An consensus log item for BABE.
#[derive(Decode, Serialize)]
pub enum ConsensusLog {
	/// The epoch has changed. This provides information about the _next_
	/// epoch - information about the _current_ epoch (i.e. the one we've just
	/// entered) should already be available earlier in the chain.
	#[codec(index = 1)]
	NextEpochData(NextEpochDescriptor),
	/// Disable the authority with given index.
	#[codec(index = 2)]
	OnDisabled(AuthorityIndex),
	/// The epoch has changed, and the epoch after the current one will
	/// enact different epoch configurations.
	#[codec(index = 3)]
	NextConfigData(NextConfigDescriptor),
}
#[derive(Decode, Serialize)]
pub struct Slot(u64);

/// The length of a Ristretto Schnorr `PublicKey`, in bytes.
pub const PUBLIC_KEY_LENGTH: usize = 32;

/// VRF output, possibly unverified.
///
/// Internally, we keep both `RistrettoPoint` and `CompressedRistretto`
/// forms using `RistrettoBoth`.
///
/// We'd actually love to statically distinguish here between inputs
/// and outputs, as well as whether outputs were verified, but doing
/// so would disrupt our general purpose DLEQ proof mechanism, so
/// users must be responsible for this themselves.  We do however
/// consume by value in actual output methods, and do not implement
/// `Copy`, as a reminder that VRF outputs should only be used once
/// and should be checked before usage.
#[derive(Decode, Serialize)]
pub struct VRFOutput(#[serde(with = "SerHex::<StrictPfx>")] pub [u8; PUBLIC_KEY_LENGTH]);

/// Short proof of correctness for associated VRF output,
/// for which no batched verification works.
pub struct VRFProof(pub schnorrkel::vrf::VRFProof);

/// Length of the short VRF proof which lacks support for batch verification.
pub const VRF_PROOF_LENGTH: usize = 64;

impl Decode for VRFProof {
	fn decode<R: codec::Input>(i: &mut R) -> Result<Self, codec::Error> {
		let decoded = <[u8; VRF_PROOF_LENGTH]>::decode(i)?;
		let proof = schnorrkel::vrf::VRFProof::from_bytes(&decoded)
			.map_err(|_| codec::Error::from("VRFProof invalid"))?;

		Ok(Self(proof))
	}
}

impl Serialize for VRFProof {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let raw: [u8; VRF_PROOF_LENGTH] = self.0.to_bytes();
		let challenge = hex::encode(&raw[..32]);
		let schnorr = hex::encode(&raw[32..]);

		let mut state = serializer.serialize_struct("VRFProof", 2)?;
		state.serialize_field("challenge", &challenge)?;
		state.serialize_field("schnorr", &schnorr)?;
		state.end()
	}
}

/// Raw BABE primary slot assignment pre-digest.
#[derive(Decode, Serialize)]
pub struct PrimaryPreDigest {
	/// Authority index
	pub authority_index: AuthorityIndex,
	/// Slot
	pub slot: Slot,
	/// VRF output
	pub vrf_output: VRFOutput,
	/// VRF proof
	pub vrf_proof: VRFProof,
}

/// BABE secondary slot assignment pre-digest.
#[derive(Decode, Serialize)]
pub struct SecondaryPlainPreDigest {
	/// Authority index
	///
	/// This is not strictly-speaking necessary, since the secondary slots
	/// are assigned based on slot number and epoch randomness. But including
	/// it makes things easier for higher-level users of the chain data to
	/// be aware of the author of a secondary-slot block.
	pub authority_index: AuthorityIndex,
	/// Slot
	pub slot: Slot,
}

/// BABE secondary deterministic slot assignment with VRF outputs.
#[derive(Decode, Serialize)]
pub struct SecondaryVRFPreDigest {
	/// Authority index
	pub authority_index: AuthorityIndex,
	/// Slot
	pub slot: Slot,
	/// VRF output
	pub vrf_output: VRFOutput,
	/// VRF proof
	pub vrf_proof: VRFProof,
}

/// A BABE pre-runtime digest. This contains all data required to validate a
/// block and for the BABE runtime module. Slots can be assigned to a primary
/// (VRF based) and to a secondary (slot number based).
#[derive(Decode, Serialize)]
pub enum PreDigest {
	/// A primary VRF-based slot assignment.
	#[codec(index = 1)]
	Primary(PrimaryPreDigest),
	/// A secondary deterministic slot assignment.
	#[codec(index = 2)]
	SecondaryPlain(SecondaryPlainPreDigest),
	/// A secondary deterministic slot assignment with VRF outputs.
	#[codec(index = 3)]
	SecondaryVRF(SecondaryVRFPreDigest),
}
