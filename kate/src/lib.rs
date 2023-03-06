#![cfg_attr(not(feature = "std"), no_std)]

use da_primitives::{BlockLengthColumns, BlockLengthRows};
#[cfg(feature = "std")]
pub use dusk_plonk::{commitment_scheme::kzg10::PublicParameters, prelude::BlsScalar};
use frame_support::sp_runtime::SaturatedConversion;
#[cfg(feature = "std")]
use kate_recovery::matrix::Dimensions;
use static_assertions::const_assert_ne;

use crate::config::DATA_CHUNK_SIZE;

pub const LOG_TARGET: &str = "kate";
pub type Seed = [u8; 32];

pub mod config {
	use super::{BlockLengthColumns, BlockLengthRows};

    // TODO: Delete this? not used anywhere
	pub const SCALAR_SIZE_WIDE: usize = 64;

	pub const SCALAR_SIZE: usize = 32;
	pub const DATA_CHUNK_SIZE: usize = 31; // Actual chunk size is 32 after 0 padding is done
	pub const EXTENSION_FACTOR: u32 = 2;
	pub const PROVER_KEY_SIZE: u32 = 48;
	pub const PROOF_SIZE: usize = 48;
	// MINIMUM_BLOCK_SIZE, MAX_BLOCK_ROWS and MAX_BLOCK_COLUMNS have to be a power of 2 because of the FFT functions requirements
	pub const MINIMUM_BLOCK_SIZE: usize = 128;
	pub const MAX_BLOCK_ROWS: BlockLengthRows = if cfg!(feature = "extended-columns") {
		BlockLengthRows(128)
	} else {
		BlockLengthRows(256)
	};
	pub const MAX_BLOCK_COLUMNS: BlockLengthColumns = if cfg!(feature = "extended-columns") {
		BlockLengthColumns(512)
	} else {
		BlockLengthColumns(256)
	};
	pub const MAXIMUM_BLOCK_SIZE: bool = cfg!(feature = "maximum-block-size");
}

/// TODO
///  - Dedup this from `kate-recovery` once that library support `no-std`.
#[cfg(feature = "std")]
pub mod testnet {
	use super::{BlockLengthColumns, PublicParameters};
	use ark_bls12_381::{G1Projective, G2Projective, Fr};
use ark_ff::{Fp, BigInt};
use ark_serialize::CanonicalDeserialize;
use once_cell::sync::Lazy;
	use poly_multiproof::m1_blst;
use rand::SeedableRng;
	use rand_chacha::ChaChaRng;
	use std::{collections::HashMap, sync::Mutex};

	static SRS_DATA: Lazy<Mutex<HashMap<u32, PublicParameters>>> =
		Lazy::new(|| Mutex::new(HashMap::new()));

	pub fn public_params(max_degree: BlockLengthColumns) -> PublicParameters {
		let mut srs_data_locked = SRS_DATA.lock().unwrap();
		srs_data_locked
			.entry(max_degree.0)
			.or_insert_with(|| {
				let mut rng = ChaChaRng::seed_from_u64(42);
				PublicParameters::setup(max_degree.as_usize(), &mut rng).unwrap()
			})
			.clone()
	}

	const SEC_LIMBS: [u64; 4] = [
		16526363067508752668,
		17870878028964021343,
		15693365399533249662,
		1020900941429372507,
	];
	const G1_BYTES: [u8; 48] = [
		164, 95, 117, 74, 158, 148, 204, 203, 178, 203, 233, 215, 196, 65, 184, 181, 39, 2, 110,
		240, 94, 42, 58, 255, 74, 164, 187, 28, 87, 223, 55, 103, 251, 102, 156, 196, 199, 99, 155,
		211, 126, 104, 54, 83, 189, 197, 11, 90,
	];
	const G2_BYTES: [u8; 96] = [
		184, 69, 172, 94, 123, 78, 200, 84, 29, 1, 38, 96, 39, 103, 114, 224, 1, 193, 224, 71, 94,
		96, 151, 24, 132, 72, 29, 67, 252, 189, 68, 222, 42, 2, 233, 134, 45, 191, 159, 83, 108,
		33, 24, 20, 246, 204, 84, 72, 16, 11, 205, 165, 220, 112, 120, 84, 175, 142, 56, 41, 117,
		13, 31, 177, 139, 18, 114, 134, 170, 164, 252, 149, 158, 115, 46, 33, 40, 168, 163, 21,
		242, 248, 244, 25, 191, 87, 116, 254, 4, 58, 244, 111, 187, 235, 75, 39,
	];

	pub fn multiproof_params(max_degree: usize, max_pts: usize) -> m1_blst::M1NoPrecomp {
		let x: Fr = Fp(BigInt(SEC_LIMBS), core::marker::PhantomData);

		let g1 = G1Projective::deserialize_compressed(&G1_BYTES[..]).unwrap();
		let g2 = G2Projective::deserialize_compressed(&G2_BYTES[..]).unwrap();

		m1_blst::M1NoPrecomp::new_from_scalar(x, g1, g2, max_degree + 1, max_pts)
	}

	#[cfg(test)]
	mod tests {
		use core::marker::PhantomData;

		use super::*;
		use ark_bls12_381::Fr;
		use ark_ff::{BigInt, Fp};
		use ark_poly::EvaluationDomain;
		use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
		use dusk_bytes::Serializable;
		use dusk_plonk::{
			fft::{EvaluationDomain as PlonkED, Evaluations as PlonkEV},
			prelude::BlsScalar,
		};
		use poly_multiproof::traits::Committer;
		use rand::thread_rng;

		use crate::testnet;
		#[test]
		fn test_consistent_testnet_params() {
			let x: Fr = Fp(BigInt(SEC_LIMBS), core::marker::PhantomData);
			let mut out = [0u8; 32];
			x.serialize_compressed(&mut out[..]).unwrap();
			const SEC_BYTES: [u8; 32] = [
				120, 72, 181, 215, 17, 188, 152, 131, 153, 99, 23, 163, 249, 201, 2, 105, 213, 103,
				113, 0, 93, 84, 10, 25, 24, 73, 57, 201, 232, 208, 219, 42,
			];
			assert_eq!(SEC_BYTES, out);

			let g1 = ark_bls12_381::G1Projective::deserialize_compressed(&G1_BYTES[..]).unwrap();
			let g2 = ark_bls12_381::G2Projective::deserialize_compressed(&G2_BYTES[..]).unwrap();

			let pmp = poly_multiproof::m1_blst::M1NoPrecomp::new_from_scalar(x, g1, g2, 1024, 256);

			let dp_evals = (0..30)
				.map(|_| BlsScalar::random(&mut thread_rng()))
				.collect::<Vec<_>>();

			let pmp_evals = dp_evals
				.iter()
				.map(|i| Fp(BigInt(i.0), PhantomData))
				.collect::<Vec<Fr>>();

			let dp_poly =
				PlonkEV::from_vec_and_domain(dp_evals, PlonkED::new(1024).unwrap()).interpolate();
			let pmp_ev = ark_poly::GeneralEvaluationDomain::<Fr>::new(1024).unwrap();
			let pmp_poly = pmp_ev.ifft(&pmp_evals);

			let pubs = testnet::public_params(da_primitives::BlockLengthColumns(1024));

			let dp_commit = pubs.commit_key().commit(&dp_poly).unwrap().0.to_bytes();
			let mut pmp_commit = [0u8; 48];
			pmp.commit(&pmp_poly)
				.unwrap()
				.0
				.serialize_compressed(&mut pmp_commit[..])
				.unwrap();

			assert_eq!(dp_commit, pmp_commit);
		}
	}
}

#[cfg(feature = "std")]
pub mod com;

pub mod gridgen;
pub mod utils;
/// Precalculate the length of padding IEC 9797 1.
///
/// # NOTE
/// There is a unit test to ensure this formula match with the current
/// IEC 9797 1 algorithm we implemented. See `fn pad_iec_9797_1`
#[inline]
fn padded_len_of_pad_iec_9797_1(len: u32) -> u32 {
	let len_plus_one = len.saturating_add(1);
	let offset = (DATA_CHUNK_SIZE - (len_plus_one as usize % DATA_CHUNK_SIZE)) % DATA_CHUNK_SIZE;
	let offset: u32 = offset.saturated_into();

	len_plus_one.saturating_add(offset)
}

/// Calculates the padded len based of initial `len`.
pub fn padded_len(len: u32, chunk_size: u32) -> u32 {
	let iec_9797_1_len = padded_len_of_pad_iec_9797_1(len);

	const_assert_ne!(DATA_CHUNK_SIZE, 0);
	debug_assert!(
		chunk_size >= DATA_CHUNK_SIZE as u32,
		"`BlockLength.chunk_size` is valid by design .qed"
	);
	let diff_per_chunk = chunk_size - DATA_CHUNK_SIZE as u32;
	let pad_to_chunk_extra = if diff_per_chunk != 0 {
		let chunks_count = iec_9797_1_len / DATA_CHUNK_SIZE as u32;
		chunks_count * diff_per_chunk
	} else {
		0
	};

	iec_9797_1_len + pad_to_chunk_extra
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BlockDimensions {
	pub rows: BlockLengthRows,
	pub cols: BlockLengthColumns,
	pub chunk_size: u32,
}

impl BlockDimensions {
	pub fn size(&self) -> usize {
		self.rows
			.0
			.saturating_mul(self.cols.0)
			.saturating_mul(self.chunk_size) as usize
	}

	pub fn new<R, C>(rows: R, cols: C, chunk_size: u32) -> Self
	where
		R: Into<BlockLengthRows>,
		C: Into<BlockLengthColumns>,
	{
		Self {
			rows: rows.into(),
			cols: cols.into(),
			chunk_size,
		}
	}
}

#[derive(PartialEq, Eq, Debug)]
pub enum TryFromBlockDimensionsError {
	InvalidRowsOrColumns(sp_std::num::TryFromIntError),
	InvalidDimensions,
}

impl From<sp_std::num::TryFromIntError> for TryFromBlockDimensionsError {
	fn from(error: sp_std::num::TryFromIntError) -> Self {
		TryFromBlockDimensionsError::InvalidRowsOrColumns(error)
	}
}

#[cfg(feature = "std")]
impl sp_std::convert::TryInto<Dimensions> for BlockDimensions {
	type Error = TryFromBlockDimensionsError;

	fn try_into(self) -> Result<Dimensions, Self::Error> {
		let rows = self.rows.0.try_into()?;
		let cols = self.cols.0.try_into()?;

		Dimensions::new(rows, cols).ok_or(TryFromBlockDimensionsError::InvalidDimensions)
	}
}

// vim: set noet nowrap
