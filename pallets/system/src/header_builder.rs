use avail_core::{header::HeaderExtension, traits::ExtendedHeader, AppExtrinsic};
use frame_support::traits::Randomness;
pub use kate::{
	metrics::{IgnoreMetrics, Metrics},
	Seed,
};
use sp_core::H256;
#[cfg(feature = "std")]
use sp_runtime::SaturatedConversion;
use sp_runtime::{generic::Digest, traits::Hash};
use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;

use crate::{limits::BlockLength, Config, LOG_TARGET};

pub mod da {
	use core::marker::PhantomData;

	use avail_core::header::{Header as DaHeader, HeaderExtension};
	use sp_runtime::traits::BlakeTwo256;

	use super::*;

	pub type Hash = sp_core::H256;
	pub type BlockNumber = u32;

	/// Data-Avail Header builder.
	pub struct HeaderExtensionBuilder<T: Config>(PhantomData<T>);

	impl<T: Config> super::HeaderExtensionBuilder for HeaderExtensionBuilder<T> {
		type Header = DaHeader<BlockNumber, BlakeTwo256>;

		#[inline]
		fn build(
			app_extrinsics: Vec<AppExtrinsic>,
			data_root: H256,
			block_length: BlockLength,
			block_number: u32,
		) -> HeaderExtension {
			let seed = Self::random_seed::<T>();

			super::hosted_header_builder::build(
				app_extrinsics,
				data_root,
				block_length,
				block_number,
				seed,
			)
		}
	}
}

/// Trait for header builder.
pub trait HeaderExtensionBuilder {
	type Header: sp_runtime::traits::Header + ExtendedHeader<u32, H256, Digest, HeaderExtension>;

	/// Creates the header using the given parameters.
	fn build(
		app_extrinsics: Vec<AppExtrinsic>,
		data_root: H256,
		block_length: BlockLength,
		block_number: u32,
	) -> HeaderExtension;

	/// Generates a random seed using the _epoch seed_ and the _current block_ returned by
	/// `T::Randomness` type.
	fn random_seed<T: Config>() -> Seed {
		let seed = if cfg!(feature = "secure_padding_fill") {
			let (epoch_seed, block_number) = <T as Config>::Randomness::random_seed();
			let seed = <T as Config>::Hashing::hash_of(&(&epoch_seed, &block_number));
			log::trace!(
				target: LOG_TARGET,
				"Header builder seed {seed:?} from epoch seed {epoch_seed:?} and block {block_number:?}");
			seed
		} else {
			<T as Config>::Hash::default()
		};

		seed.into()
	}
}

#[cfg(feature = "header_commitment_corruption")]
fn corrupt_commitment(block_number: u32, commitment: &mut Vec<u8>) {
	if let Some(ref_byte) = commitment.get_mut(0) {
		log::trace!(
			target: LOG_TARGET,
			"Block {block_number}, corrupting 1st byte of commitment from {ref_byte:x} to {:x}",
			*ref_byte ^ 0xffu8
		);

		*ref_byte ^= 0xffu8;
	} else {
		log::trace!(
			target: LOG_TARGET,
			"Block {block_number}, corrupting commitment by adding one `0xFF` byte "
		);
		commitment.push(0xffu8)
	}
}

#[cfg(feature = "std")]
pub fn build_extension<M: Metrics>(
	app_extrinsics: &[AppExtrinsic],
	data_root: H256,
	block_length: BlockLength,
	_block_number: u32,
	seed: Seed,
	metrics: &M,
) -> HeaderExtension {
	use avail_core::header::extension::{v1, v2};
	use once_cell::sync::Lazy;
	static PMP: Lazy<kate::pmp::m1_blst::M1NoPrecomp> =
		once_cell::sync::Lazy::new(|| kate::testnet::multiproof_params(256, 256));

	const MIN_WIDTH: usize = 4;
	let timer = std::time::Instant::now();
	let grid = kate::gridgen::EvaluationGrid::from_extrinsics(
		app_extrinsics.to_vec(),
		MIN_WIDTH,
		block_length.cols.0.saturated_into(), // even if we run on a u16 target this is fine
		block_length.rows.0.saturated_into(),
		seed,
	)
	.expect("Grid construction cannot fail");
	metrics.preparation_block_time(timer.elapsed());

	let timer = std::time::Instant::now();
	use kate::gridgen::AsBytes;
	let commitment = grid
		.make_polynomial_grid()
		.expect("Make polynomials cannot fail")
		.extended_commitments(&*PMP, 2)
		.expect("Extended commitments cannot fail")
		.iter()
		.flat_map(|c| c.to_bytes().expect("Commitment serialization cannot fail"))
		.collect::<Vec<u8>>();
	metrics.commitment_build_time(timer.elapsed());

	// Note that this uses the original dims, _not the extended ones_
	let rows = grid.dims().rows().get();
	let cols = grid.dims().cols().get();

	let app_lookup = grid.lookup().clone();
	// **NOTE:** Header extension V2 is not yet enable by default.
	if cfg!(feature = "header_extension_v2") {
		use avail_core::kate_commitment::v2::KateCommitment;
		#[allow(unused_mut)]
		let mut kate = KateCommitment::new(rows, cols, data_root, commitment);

		#[cfg(feature = "header_commitment_corruption")]
		if _block_number > 20 {
			corrupt_commitment(_block_number, &mut kate.commitment);
		}

		v2::HeaderExtension {
			commitment: kate,
			app_lookup,
		}
		.into()
	} else {
		#[allow(unused_mut)]
		let mut kate = avail_core::kate_commitment::v1::KateCommitment {
			rows,
			cols,
			commitment,
			data_root,
		};

		#[cfg(feature = "header_commitment_corruption")]
		if _block_number > 20 {
			corrupt_commitment(_block_number, &mut kate.commitment);
		}

		v1::HeaderExtension {
			commitment: kate,
			app_lookup,
		}
		.into()
	}
}

/// Hosted function to build the header using `kate` commitments.
#[runtime_interface]
pub trait HostedHeaderBuilder {
	/// Creates the header using the given parameters.
	/// *NOTE:* Version 1 uses `dusk-plonk v0.8.2`
	#[version(1)]
	fn build(
		app_extrinsics: Vec<AppExtrinsic>,
		data_root: H256,
		block_length: BlockLength,
		block_number: u32,
		seed: Seed,
	) -> HeaderExtension {
		let metrics = avail_base::metrics::MetricAdapter {};
		build_extension(
			&app_extrinsics,
			data_root,
			block_length,
			block_number,
			seed,
			&metrics,
		)
	}
}
